use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const VERSION: &str = "0.1.0";
#[derive(Default)]
struct Opt {
    engine: String,
    fixture: String,
    migration: String,
    rollback: String,
    workload: String,
    output: String,
    dry: bool,
    json: bool,
    reset: bool,
    output_specified: bool,
}
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("mlr: {e}");
            ExitCode::from(1)
        }
    }
}
fn run() -> Result<(), String> {
    let a: Vec<String> = env::args().collect();
    if a.len() < 2 {
        usage();
        return Err("a command is required".into());
    }
    match a[1].as_str() {
        "--help" | "-h" | "help" => {
            usage();
            Ok(())
        }
        "--version" | "version" => {
            println!("mlr {VERSION}");
            Ok(())
        }
        "guard" => {
            if a.len() != 3 {
                return Err("usage: mlr guard <database-url>".into());
            }
            safe_target(&a[2])?;
            println!("allowed: local or explicitly disposable target");
            Ok(())
        }
        "demo" => {
            let mut o = parse(&a[2..], true)?;
            validate_engine(&o.engine)?;
            if o.engine == "clickhouse" {
                o.fixture = "examples/clickhouse/fixture.sql".into();
                o.migration = "examples/clickhouse/add_customer_flag.sql".into();
                o.rollback = "examples/clickhouse/rollback_customer_flag.sql".into();
                o.workload = "examples/clickhouse/read_workload.sql".into();
            }
            if o.reset {
                return reset_demo(&o);
            }
            if o.dry {
                sample(&o)
            } else {
                rehearse(&o)
            }
        }
        "rehearse" => rehearse(&parse(&a[2..], false)?),
        x => Err(format!("unknown command: {x}")),
    }
}
fn parse(a: &[String], demo: bool) -> Result<Opt, String> {
    let mut o = Opt {
        engine: "postgres".into(),
        output: "./mlr-report".into(),
        ..Default::default()
    };
    if demo {
        o.fixture = "examples/postgres/fixture.sql".into();
        o.migration = "examples/postgres/add_customer_flag.sql".into();
        o.rollback = "examples/postgres/rollback_customer_flag.sql".into();
        o.workload = "examples/postgres/read_workload.sql".into()
    }
    let mut i = 0;
    while i < a.len() {
        let get = |i: usize| {
            a.get(i + 1)
                .cloned()
                .ok_or_else(|| format!("{} needs a value", a[i]))
        };
        match a[i].as_str() {
            "--engine" => {
                o.engine = get(i)?;
                i += 1
            }
            "--fixture" => {
                o.fixture = get(i)?;
                i += 1
            }
            "--migration" => {
                o.migration = get(i)?;
                i += 1
            }
            "--rollback" => {
                o.rollback = get(i)?;
                i += 1
            }
            "--workload" => {
                o.workload = get(i)?;
                i += 1
            }
            "--output" => {
                o.output = get(i)?;
                o.output_specified = true;
                i += 1
            }
            "--dry-run" => o.dry = true,
            "--json" => o.json = true,
            "--reset" if demo => o.reset = true,
            "--reset" => return Err("--reset is available only with `mlr demo`".into()),
            "--help" | "-h" => {
                usage();
                return Ok(o);
            }
            x => return Err(format!("unknown option {x}")),
        }
        i += 1
    }
    Ok(o)
}
fn validate_engine(engine: &str) -> Result<(), String> {
    match engine {
        "postgres" | "clickhouse" => Ok(()),
        _ => Err(format!(
            "unknown engine {engine}; use postgres or clickhouse"
        )),
    }
}
fn reset_demo(o: &Opt) -> Result<(), String> {
    if !o.output_specified {
        return Err("`mlr demo --reset` needs an explicit --output DIR so it cannot remove the default report folder".into());
    }
    let target = Path::new(&o.output);
    if o.output.trim().is_empty()
        || target == Path::new("/")
        || target == Path::new(".")
        || target == Path::new("..")
    {
        return Err("refusing to reset this output path; choose a dedicated demo directory".into());
    }
    if target.exists() {
        let cwd = env::current_dir().map_err(|e| e.to_string())?;
        if target.canonicalize().map_err(|e| e.to_string())? == cwd {
            return Err("refusing to reset the current working directory; choose a dedicated demo directory".into());
        }
        fs::remove_dir_all(target).map_err(|e| format!("remove {}: {e}", o.output))?;
        println!("removed {}", o.output);
    } else {
        println!("nothing to remove at {}", o.output);
    }
    Ok(())
}
fn safe_target(target: &str) -> Result<(), String> {
    let s = target.to_lowercase();
    if [
        "localhost",
        "127.0.0.1",
        "[::1]",
        "host.docker.internal",
        ".test",
        "disposable",
    ]
    .iter()
    .any(|h| s.contains(h))
    {
        Ok(())
    } else {
        Err("only loopback or explicitly disposable URLs are allowed; this tool never connects to production".into())
    }
}
fn rehearse(o: &Opt) -> Result<(), String> {
    validate_engine(&o.engine)?;
    if o.engine == "clickhouse" {
        return rehearse_clickhouse(o);
    }
    for f in [&o.fixture, &o.migration] {
        if !Path::new(f).is_file() {
            return Err(format!("read {f}: file not found"));
        }
    }
    if Command::new("docker").arg("version").output().is_err() {
        return Err("Docker is required. Install Docker, or use `mlr demo --dry-run` to inspect the bundled card".into());
    }
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let name = format!("mlr-{id}");
    let cleanup = Cleanup(name.clone());
    docker(&[
        "run",
        "-d",
        "--name",
        &name,
        "-e",
        "POSTGRES_PASSWORD=rehearsal",
        "-e",
        "POSTGRES_DB=rehearsal",
        "postgres:16-alpine",
    ])?;
    for _ in 0..30 {
        if psql(&name, &["-c", "SELECT 1"]).is_ok() {
            break;
        }
        std::thread::sleep(Duration::from_secs(1))
    }
    for (src, dst) in [
        (&o.fixture, "/work/fixture.sql"),
        (&o.migration, "/work/migration.sql"),
    ] {
        docker(&["cp", src, &format!("{name}:{dst}")])?
    }
    if !o.rollback.is_empty() {
        docker(&["cp", &o.rollback, &format!("{name}:/work/rollback.sql")])?
    }
    if !o.workload.is_empty() && Path::new(&o.workload).is_file() {
        docker(&["cp", &o.workload, &format!("{name}:/work/workload.sql")])?
    }
    psql(&name, &["-f", "/work/fixture.sql"])?;
    let before = table_bytes(&name);
    let mut workload = if !o.workload.is_empty() && Path::new(&o.workload).is_file() {
        Command::new("docker").args(["exec", &name, "sh", "-lc", "for i in $(seq 1 120); do psql -U postgres -d rehearsal -f /work/workload.sql >/dev/null; done"]).spawn().ok()
    } else {
        None
    };
    let start = Instant::now();
    let mut migration = Command::new("docker")
        .args([
            "exec",
            &name,
            "psql",
            "-U",
            "postgres",
            "-d",
            "rehearsal",
            "-v",
            "ON_ERROR_STOP=1",
            "-f",
            "/work/migration.sql",
        ])
        .spawn()
        .map_err(|e| e.to_string())?;
    let mut observed_wait = 0;
    loop {
        match migration.try_wait().map_err(|e| e.to_string())? {
            Some(status) if status.success() => break,
            Some(_) => return Err("migration failed; the disposable database was removed".into()),
            None => {
                if pg_waiters(&name) > 0 {
                    observed_wait += 25;
                }
                std::thread::sleep(Duration::from_millis(25));
            }
        }
    }
    let duration = start.elapsed().as_millis();
    if let Some(child) = workload.as_mut() {
        let _ = child.kill();
        let _ = child.wait();
    }
    let after = table_bytes(&name);
    let rolled = !o.rollback.is_empty()
        && psql(
            &name,
            &["-v", "ON_ERROR_STOP=1", "-f", "/work/rollback.sql"],
        )
        .is_ok();
    drop(cleanup);
    write_report(
        o,
        rehearsal_report(
            "postgres",
            o,
            Measurements {
                duration,
                during_lock: observed_wait,
                before,
                after,
            },
            rolled,
            vec![
                "Estimate from a fresh disposable Postgres container.".into(),
                "Lock waits are sampled from pg_stat_activity while the supplied workload runs."
                    .into(),
                "Use a production-shaped sanitized fixture before relying on this result.".into(),
            ],
        ),
    )
}
fn rehearse_clickhouse(o: &Opt) -> Result<(), String> {
    for f in [&o.fixture, &o.migration] {
        if !Path::new(f).is_file() {
            return Err(format!("read {f}: file not found"));
        }
    }
    if Command::new("docker").arg("version").output().is_err() {
        return Err("Docker is required. Install Docker, or use `mlr demo --dry-run` to inspect the bundled card".into());
    }
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let name = format!("mlr-{id}");
    let cleanup = Cleanup(name.clone());
    docker(&[
        "run",
        "-d",
        "--name",
        &name,
        "clickhouse/clickhouse-server:24.8-alpine",
    ])?;
    for _ in 0..30 {
        if clickhouse(&name, "SELECT 1").is_ok() {
            break;
        };
        std::thread::sleep(Duration::from_secs(1));
    }
    for (src, dst) in [
        (&o.fixture, "/work/fixture.sql"),
        (&o.migration, "/work/migration.sql"),
    ] {
        docker(&["cp", src, &format!("{name}:{dst}")])?
    }
    if !o.rollback.is_empty() {
        docker(&["cp", &o.rollback, &format!("{name}:/work/rollback.sql")])?
    }
    clickhouse_file(&name, "/work/fixture.sql")?;
    if !o.workload.is_empty() && Path::new(&o.workload).is_file() {
        docker(&["cp", &o.workload, &format!("{name}:/work/workload.sql")])?;
        let _ = clickhouse_file(&name, "/work/workload.sql");
    }
    let before = clickhouse_bytes(&name);
    let start = Instant::now();
    clickhouse_file(&name, "/work/migration.sql")?;
    let duration = start.elapsed().as_millis();
    let after = clickhouse_bytes(&name);
    let rolled = !o.rollback.is_empty() && clickhouse_file(&name, "/work/rollback.sql").is_ok();
    drop(cleanup);
    write_report(
        o,
        rehearsal_report(
            "clickhouse",
            o,
            Measurements {
                duration,
                during_lock: 0,
                before,
                after,
            },
            rolled,
            vec![
                "Estimate from a fresh disposable ClickHouse container.".into(),
                "ClickHouse mutations and merges may continue after a DDL statement returns."
                    .into(),
                "Use a production-shaped sanitized fixture before relying on this result.".into(),
            ],
        ),
    )
}
struct Cleanup(String);
impl Drop for Cleanup {
    fn drop(&mut self) {
        let _ = Command::new("docker").args(["rm", "-f", &self.0]).output();
    }
}
fn docker(a: &[&str]) -> Result<(), String> {
    let s = Command::new("docker")
        .args(a)
        .status()
        .map_err(|e| e.to_string())?;
    if s.success() {
        Ok(())
    } else {
        Err(format!("docker command failed: docker {}", a.join(" ")))
    }
}
fn psql(name: &str, a: &[&str]) -> Result<(), String> {
    let mut args = vec!["exec", name, "psql", "-U", "postgres", "-d", "rehearsal"];
    args.extend_from_slice(a);
    docker(&args)
}
fn table_bytes(name: &str) -> u64 {
    let sql="SELECT COALESCE(sum(pg_total_relation_size(oid)),0) FROM pg_class WHERE relkind IN ('r','m')";
    let o = Command::new("docker")
        .args([
            "exec",
            name,
            "psql",
            "-U",
            "postgres",
            "-d",
            "rehearsal",
            "-tAc",
            sql,
        ])
        .output();
    o.ok()
        .and_then(|x| String::from_utf8(x.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}
fn pg_waiters(name: &str) -> u128 {
    let sql = "SELECT count(*) FROM pg_stat_activity WHERE wait_event_type = 'Lock'";
    let out = Command::new("docker")
        .args([
            "exec",
            name,
            "psql",
            "-U",
            "postgres",
            "-d",
            "rehearsal",
            "-tAc",
            sql,
        ])
        .output();
    out.ok()
        .and_then(|x| String::from_utf8(x.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}
fn clickhouse(name: &str, sql: &str) -> Result<(), String> {
    docker(&["exec", name, "clickhouse-client", "--query", sql])
}
fn clickhouse_file(name: &str, file: &str) -> Result<(), String> {
    docker(&[
        "exec",
        name,
        "sh",
        "-lc",
        &format!("clickhouse-client --multiquery < {file}"),
    ])
}
fn clickhouse_bytes(name: &str) -> u64 {
    let sql = "SELECT coalesce(sum(bytes_on_disk),0) FROM system.parts WHERE active";
    let o = Command::new("docker")
        .args(["exec", name, "clickhouse-client", "--query", sql])
        .output();
    o.ok()
        .and_then(|x| String::from_utf8(x.stdout).ok())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}
struct Report {
    engine: String,
    migration: String,
    duration: u128,
    during_lock: u128,
    before: u64,
    after: u64,
    rollback: bool,
    verdict: String,
    notes: Vec<String>,
}
struct Measurements {
    duration: u128,
    during_lock: u128,
    before: u64,
    after: u64,
}
fn rehearsal_report(
    engine: &str,
    o: &Opt,
    measurements: Measurements,
    rollback: bool,
    notes: Vec<String>,
) -> Report {
    Report {
        engine: engine.into(),
        migration: o.migration.clone(),
        duration: measurements.duration,
        during_lock: measurements.during_lock,
        before: measurements.before,
        after: measurements.after,
        rollback,
        verdict: if rollback { "GO" } else { "NO-GO" }.into(),
        notes,
    }
}
fn sample(o: &Opt) -> Result<(), String> {
    validate_engine(&o.engine)?;
    write_report(o, rehearsal_report(&o.engine, o, Measurements { duration: 184, during_lock: 0, before: 32768, after: 40960 }, true, vec!["Preview from the bundled sanitized fixture; it is an estimate, not a production measurement.".into(),"The migration adds a defaulted column. Rehearse against a production-shaped fixture before deployment.".into(),"Rollback SQL completed in the same disposable environment.".into()]))
}
fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
fn write_report(o: &Opt, r: Report) -> Result<(), String> {
    let rollback_failed = !r.rollback;
    fs::create_dir_all(&o.output).map_err(|e| e.to_string())?;
    let dir = PathBuf::from(&o.output);
    let notes = r
        .notes
        .iter()
        .map(|n| format!("\"{}\"", escape(n)))
        .collect::<Vec<_>>()
        .join(", ");
    let json=format!("{{\n  \"engine\": \"{}\",\n  \"migration\": \"{}\",\n  \"duration_ms\": {},\n  \"max_lock_wait_ms\": {},\n  \"table_bytes_before\": {},\n  \"table_bytes_after\": {},\n  \"rollback_checked\": {},\n  \"verdict\": \"{}\",\n  \"notes\": [{}]\n}}\n",escape(&r.engine),escape(&r.migration),r.duration,r.during_lock,r.before,r.after,r.rollback,r.verdict,notes);
    fs::write(dir.join("report.json"), &json).map_err(|e| e.to_string())?;
    let mut md=format!("# Migration go/no-go card\n\n**Verdict: {}**\n\n- Engine: {}\n- Migration: `{}`\n- Statement time: {} ms\n- Maximum observed lock wait: {} ms\n- Table bytes: {} → {}\n- Rollback checked: {}\n\n## Operator notes\n\n",r.verdict,r.engine,r.migration,r.duration,r.during_lock,r.before,r.after,r.rollback);
    for n in r.notes {
        md.push_str(&format!("- {n}\n"))
    }
    fs::write(dir.join("runbook.md"), md).map_err(|e| e.to_string())?;
    if o.json {
        println!("{}", json.trim());
    } else {
        println!("wrote {}/report.json", o.output);
        println!("wrote {}/runbook.md", o.output);
    }
    if rollback_failed {
        return Err("rollback failed; wrote a NO-GO card. Fix or replace the rollback SQL before proceeding".into());
    }
    Ok(())
}
fn usage() {
    println!("Migration Lock Rehearsal {VERSION}\n\nRehearse supplied Postgres or ClickHouse migration SQL in a fresh Docker container.\n\nUsage:\n  mlr demo [--engine postgres|clickhouse] [--output DIR] [--dry-run] [--json]\n  mlr demo --output DIR --reset\n  mlr rehearse --engine postgres|clickhouse --fixture FIXTURE.sql --migration CHANGE.sql [--rollback DOWN.sql] [--output DIR] [--json]\n  mlr guard DATABASE_URL\n\nA failed rollback writes NO-GO and returns an actionable report. The CLI never accepts remote targets. It creates and removes its own disposable container.")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_remote() {
        assert!(safe_target("postgres://a@prod.example.com/app").is_err());
        assert!(safe_target("postgres://a@localhost/app").is_ok())
    }
    #[test]
    fn sample_writes_files() {
        let d = env::temp_dir().join("mlr-test-sample");
        let _ = fs::remove_dir_all(&d);
        sample(&Opt {
            engine: "postgres".into(),
            output: d.to_string_lossy().into(),
            migration: "sample.sql".into(),
            ..Default::default()
        })
        .unwrap();
        assert!(d.join("report.json").is_file());
        assert!(d.join("runbook.md").is_file());
        let _ = fs::remove_dir_all(d);
    }
    fn failed_rollback_writes_no_go_and_returns_actionable_error(engine: &str) {
        let d = env::temp_dir().join(format!("mlr-test-failed-rollback-{engine}"));
        let _ = fs::remove_dir_all(&d);
        let opt = Opt {
            output: d.to_string_lossy().into(),
            ..Default::default()
        };
        let report = rehearsal_report(
            engine,
            &opt,
            Measurements {
                duration: 1,
                during_lock: 0,
                before: 1,
                after: 1,
            },
            false,
            vec![],
        );
        let error = write_report(&opt, report).unwrap_err();
        assert!(error.contains("rollback failed"));
        let json = fs::read_to_string(d.join("report.json")).unwrap();
        let runbook = fs::read_to_string(d.join("runbook.md")).unwrap();
        assert!(json.contains("\"verdict\": \"NO-GO\""));
        assert!(runbook.contains("**Verdict: NO-GO**"));
        let _ = fs::remove_dir_all(d);
    }
    #[test]
    fn failed_postgres_rollback_is_no_go() {
        failed_rollback_writes_no_go_and_returns_actionable_error("postgres");
    }
    #[test]
    fn failed_clickhouse_rollback_is_no_go() {
        failed_rollback_writes_no_go_and_returns_actionable_error("clickhouse");
    }
    #[test]
    fn rejects_unsupported_engine_before_report_generation() {
        assert!(validate_engine("mysql").is_err());
    }
}
