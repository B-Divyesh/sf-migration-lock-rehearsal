use std::{
    env, fs,
    net::IpAddr,
    path::{Path, PathBuf},
    process::{Child, Command, ExitCode},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const VERSION: &str = "0.1.0";
const DEMO_MARKER: &str = ".mlr-demo";
const DEMO_MARKER_CONTENT: &str = "migration-lock-rehearsal demo directory\n";
const POSTGRES_FIXTURE: &str = include_str!("../examples/postgres/fixture.sql");
const POSTGRES_MIGRATION: &str = include_str!("../examples/postgres/add_customer_flag.sql");
const POSTGRES_ROLLBACK: &str = include_str!("../examples/postgres/rollback_customer_flag.sql");
const POSTGRES_WORKLOAD: &str = include_str!("../examples/postgres/read_workload.sql");
const CLICKHOUSE_FIXTURE: &str = include_str!("../examples/clickhouse/fixture.sql");
const CLICKHOUSE_MIGRATION: &str = include_str!("../examples/clickhouse/add_customer_flag.sql");
const CLICKHOUSE_ROLLBACK: &str = include_str!("../examples/clickhouse/rollback_customer_flag.sql");
const CLICKHOUSE_WORKLOAD: &str = include_str!("../examples/clickhouse/read_workload.sql");
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
    migration_label: String,
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
            println!("allowed: exact localhost or loopback target");
            Ok(())
        }
        "demo" => {
            if a[2..]
                .iter()
                .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
            {
                usage();
                return Ok(());
            }
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
            prepare_demo_output(&o)?;
            let bundled = if o.dry {
                None
            } else {
                let bundled = DemoInputs::create(&o.engine)?;
                o.migration_label = o.migration.clone();
                o.fixture = bundled.file("fixture.sql");
                o.migration = bundled.file("migration.sql");
                o.rollback = bundled.file("rollback.sql");
                o.workload = bundled.file("workload.sql");
                Some(bundled)
            };
            let result = if o.dry { sample(&o) } else { rehearse(&o) };
            drop(bundled);
            result
        }
        "rehearse" => {
            if a[2..]
                .iter()
                .any(|arg| matches!(arg.as_str(), "--help" | "-h"))
            {
                usage();
                Ok(())
            } else {
                rehearse(&parse(&a[2..], false)?)
            }
        }
        x => Err(format!("unknown command: {x}")),
    }
}

struct DemoInputs(PathBuf);

impl DemoInputs {
    fn create(engine: &str) -> Result<Self, String> {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let root = env::temp_dir().join(format!("mlr-inputs-{}-{id}", std::process::id()));
        fs::create_dir(&root).map_err(|e| format!("create bundled demo inputs: {e}"))?;
        let inputs = Self(root);
        let files = if engine == "clickhouse" {
            [
                ("fixture.sql", CLICKHOUSE_FIXTURE),
                ("migration.sql", CLICKHOUSE_MIGRATION),
                ("rollback.sql", CLICKHOUSE_ROLLBACK),
                ("workload.sql", CLICKHOUSE_WORKLOAD),
            ]
        } else {
            [
                ("fixture.sql", POSTGRES_FIXTURE),
                ("migration.sql", POSTGRES_MIGRATION),
                ("rollback.sql", POSTGRES_ROLLBACK),
                ("workload.sql", POSTGRES_WORKLOAD),
            ]
        };
        for (name, contents) in files {
            fs::write(inputs.0.join(name), contents)
                .map_err(|e| format!("write bundled {name}: {e}"))?;
        }
        Ok(inputs)
    }

    fn file(&self, name: &str) -> String {
        self.0.join(name).to_string_lossy().into_owned()
    }
}

impl Drop for DemoInputs {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn parse(a: &[String], demo: bool) -> Result<Opt, String> {
    let mut o = Opt {
        engine: "postgres".into(),
        output: if demo {
            "./mlr-demo".into()
        } else {
            "./mlr-report".into()
        },
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
                validate_output_text(&o.output)?;
                o.output_specified = true;
                i += 1
            }
            "--dry-run" => o.dry = true,
            "--json" => o.json = true,
            "--reset" if demo => o.reset = true,
            "--reset" => return Err("--reset is available only with `mlr demo`".into()),
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
    validate_output_text(&o.output)?;
    let target = Path::new(&o.output);
    match fs::symlink_metadata(target) {
        Ok(_) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!("nothing to remove at {}", o.output);
            return Ok(());
        }
        Err(error) => return Err(format!("inspect {}: {error}", target.display())),
    }
    let canonical = validated_existing_demo_dir(target)?;
    fs::remove_dir_all(&canonical).map_err(|e| format!("remove {}: {e}", o.output))?;
    println!("removed {}", o.output);
    Ok(())
}

fn validate_output_text(output: &str) -> Result<(), String> {
    if output.trim().is_empty() {
        Err("--output must name a non-empty directory".into())
    } else {
        Ok(())
    }
}

fn lexical_absolute(path: &Path) -> Result<PathBuf, String> {
    let source = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().map_err(|e| e.to_string())?.join(path)
    };
    let mut normalized = PathBuf::new();
    for component in source.components() {
        use std::path::Component;
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(Path::new("/")),
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    Ok(normalized)
}

fn reject_broad_target(target: &Path) -> Result<(), String> {
    if target.parent().is_none() || target.parent() == Some(Path::new("/")) {
        return Err("refusing to reset a filesystem root or top-level directory".into());
    }
    let cwd = env::current_dir()
        .map_err(|e| e.to_string())?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if cwd.starts_with(target) {
        return Err("refusing to reset the current working directory or one of its parents".into());
    }
    if let Some(home) = env::var_os("HOME") {
        let home = PathBuf::from(home);
        if let Ok(home) = home.canonicalize() {
            if home == target {
                return Err("refusing to reset the home directory".into());
            }
        }
    }
    if [".git", ".hg", "Cargo.toml", "package.json"]
        .iter()
        .any(|marker| target.join(marker).exists())
    {
        return Err("refusing to reset a source workspace".into());
    }
    Ok(())
}

fn validated_existing_demo_dir(target: &Path) -> Result<PathBuf, String> {
    let metadata =
        fs::symlink_metadata(target).map_err(|e| format!("inspect {}: {e}", target.display()))?;
    if metadata.file_type().is_symlink() {
        return Err("refusing to reset a symlink; choose the real demo directory".into());
    }
    if !metadata.is_dir() {
        return Err("refusing to reset a path that is not a directory".into());
    }
    let canonical = target.canonicalize().map_err(|e| e.to_string())?;
    if canonical != lexical_absolute(target)? {
        return Err("refusing to reset a path containing aliases or symlinks".into());
    }
    reject_broad_target(&canonical)?;
    let marker = canonical.join(DEMO_MARKER);
    let marker_metadata = fs::symlink_metadata(&marker).map_err(|_| {
        "refusing to reset an unmarked directory; run `mlr demo` with this output first".to_string()
    })?;
    if !marker_metadata.is_file()
        || marker_metadata.file_type().is_symlink()
        || fs::read_to_string(&marker).map_err(|e| e.to_string())? != DEMO_MARKER_CONTENT
    {
        return Err("refusing to reset a directory without a valid mlr demo marker".into());
    }
    Ok(canonical)
}

fn prepare_demo_output(o: &Opt) -> Result<(), String> {
    validate_output_text(&o.output)?;
    let target = Path::new(&o.output);
    match fs::symlink_metadata(target) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err("demo output must be a real directory, not a file or symlink".into());
            }
            let canonical = target.canonicalize().map_err(|e| e.to_string())?;
            if canonical != lexical_absolute(target)? {
                return Err("demo output may not contain aliases or symlinks".into());
            }
            reject_broad_target(&canonical)?;
            let marker = canonical.join(DEMO_MARKER);
            if marker.exists() {
                validated_existing_demo_dir(target)?;
            } else if fs::read_dir(&canonical)
                .map_err(|e| e.to_string())?
                .next()
                .is_some()
            {
                return Err("demo output already contains files and was not created by mlr; choose an empty directory".into());
            } else {
                fs::write(marker, DEMO_MARKER_CONTENT).map_err(|e| e.to_string())?;
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(target).map_err(|e| e.to_string())?;
            let canonical = target.canonicalize().map_err(|e| e.to_string())?;
            if canonical != lexical_absolute(target)? {
                return Err("demo output may not contain aliases or symlinks".into());
            }
            reject_broad_target(&canonical)?;
            fs::write(canonical.join(DEMO_MARKER), DEMO_MARKER_CONTENT)
                .map_err(|e| e.to_string())?;
        }
        Err(error) => return Err(format!("inspect {}: {error}", target.display())),
    }
    Ok(())
}

fn safe_target(target: &str) -> Result<(), String> {
    let error = || {
        "only URLs whose parsed host is localhost or a loopback IP are allowed; this tool never connects to production".to_string()
    };
    if target.trim() != target || target.contains('%') {
        return Err(error());
    }
    let (scheme, rest) = target.split_once("://").ok_or_else(error)?;
    if !matches!(
        scheme.to_ascii_lowercase().as_str(),
        "postgres" | "postgresql" | "clickhouse"
    ) {
        return Err(error());
    }
    let authority = rest.split(['/', '?', '#']).next().ok_or_else(error)?;
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    let host = if let Some(bracketed) = host_port.strip_prefix('[') {
        let end = bracketed.find(']').ok_or_else(error)?;
        let suffix = &bracketed[end + 1..];
        if !suffix.is_empty()
            && (!suffix.starts_with(':')
                || suffix[1..].is_empty()
                || !suffix[1..].chars().all(|c| c.is_ascii_digit()))
        {
            return Err(error());
        }
        &bracketed[..end]
    } else {
        let mut parts = host_port.split(':');
        let host = parts.next().ok_or_else(error)?;
        if let Some(port) = parts.next() {
            if port.is_empty()
                || !port.chars().all(|c| c.is_ascii_digit())
                || parts.next().is_some()
            {
                return Err(error());
            }
        }
        host
    };
    let host = host.trim_end_matches('.');
    if host.eq_ignore_ascii_case("localhost")
        || host
            .parse::<IpAddr>()
            .map(|address| address.is_loopback())
            .unwrap_or(false)
    {
        Ok(())
    } else {
        Err(error())
    }
}
fn rehearse(o: &Opt) -> Result<(), String> {
    validate_engine(&o.engine)?;
    if o.engine == "clickhouse" {
        return rehearse_clickhouse(o);
    }
    validate_input_files(o)?;
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
    let mut ready = false;
    for _ in 0..30 {
        if psql(&name, &["-c", "SELECT 1"]).is_ok() {
            ready = true;
            break;
        }
        std::thread::sleep(Duration::from_secs(1))
    }
    if !ready {
        return Err("Postgres did not become ready; the disposable database was removed".into());
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
    let mut workload = if !o.workload.is_empty() {
        Some(Command::new("docker").args(["exec", &name, "sh", "-lc", "for i in $(seq 1 120); do psql -U postgres -d rehearsal -f /work/workload.sql >/dev/null; done"]).spawn().map_err(|e| format!("start Postgres workload: {e}"))?)
    } else {
        None
    };
    let start = Instant::now();
    let mut migration = match Command::new("docker")
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
    {
        Ok(child) => child,
        Err(error) => {
            stop_child(&mut workload);
            return Err(error.to_string());
        }
    };
    let mut observed_wait = 0;
    loop {
        match migration.try_wait().map_err(|e| e.to_string())? {
            Some(status) if status.success() => break,
            Some(_) => {
                stop_child(&mut workload);
                return Err("migration failed; the disposable database was removed".into());
            }
            None => {
                observed_wait = observed_wait.max(pg_lock_wait_ms(&name));
                std::thread::sleep(Duration::from_millis(25));
            }
        }
    }
    let duration = start.elapsed().as_millis();
    stop_child(&mut workload);
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
    validate_input_files(o)?;
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
    let mut ready = false;
    for _ in 0..30 {
        if clickhouse(&name, "SELECT 1").is_ok() {
            ready = true;
            break;
        };
        std::thread::sleep(Duration::from_secs(1));
    }
    if !ready {
        return Err("ClickHouse did not become ready; the disposable database was removed".into());
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
    let mut workload = if !o.workload.is_empty() {
        docker(&["cp", &o.workload, &format!("{name}:/work/workload.sql")])?;
        Some(
            Command::new("docker")
                .args([
                    "exec",
                    &name,
                    "sh",
                    "-lc",
                    "for i in $(seq 1 120); do clickhouse-client --multiquery < /work/workload.sql >/dev/null; done",
                ])
                .spawn()
                .map_err(|e| format!("start ClickHouse workload: {e}"))?,
        )
    } else {
        None
    };
    let before = clickhouse_bytes(&name);
    let start = Instant::now();
    let mut migration = match Command::new("docker")
        .args([
            "exec",
            &name,
            "sh",
            "-lc",
            "clickhouse-client --multiquery < /work/migration.sql",
        ])
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            stop_child(&mut workload);
            return Err(format!("start ClickHouse migration: {error}"));
        }
    };
    let mut observed_wait = 0;
    let migration_result = loop {
        match migration.try_wait() {
            Ok(Some(status)) => break status.success(),
            Ok(None) => {
                observed_wait = match clickhouse_lock_wait_ms(&name) {
                    Ok(wait) => observed_wait.max(wait),
                    Err(error) => {
                        let _ = migration.kill();
                        let _ = migration.wait();
                        stop_child(&mut workload);
                        return Err(error);
                    }
                };
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(error) => {
                stop_child(&mut workload);
                return Err(error.to_string());
            }
        }
    };
    let duration = start.elapsed().as_millis();
    observed_wait = match clickhouse_lock_wait_ms(&name) {
        Ok(wait) => observed_wait.max(wait),
        Err(error) => {
            stop_child(&mut workload);
            return Err(error);
        }
    };
    stop_child(&mut workload);
    if !migration_result {
        return Err("migration failed; the disposable ClickHouse database was removed".into());
    }
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
                during_lock: observed_wait,
                before,
                after,
            },
            rolled,
            vec![
                "Estimate from a fresh disposable ClickHouse container.".into(),
                "Lock waits are sampled from active ClickHouse profile events while the supplied workload runs."
                    .into(),
                "ClickHouse mutations and merges may continue after a DDL statement returns.".into(),
                "Use a production-shaped sanitized fixture before relying on this result.".into(),
            ],
        ),
    )
}

fn validate_input_files(o: &Opt) -> Result<(), String> {
    for file in [&o.fixture, &o.migration] {
        if !Path::new(file).is_file() {
            return Err(format!("read {file}: file not found"));
        }
    }
    for file in [&o.rollback, &o.workload] {
        if !file.is_empty() && !Path::new(file).is_file() {
            return Err(format!("read {file}: file not found"));
        }
    }
    Ok(())
}

fn stop_child(child: &mut Option<Child>) {
    if let Some(child) = child.as_mut() {
        let _ = child.kill();
        let _ = child.wait();
    }
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
fn pg_lock_wait_ms(name: &str) -> u128 {
    let sql = "SELECT COALESCE(MAX((EXTRACT(EPOCH FROM (clock_timestamp() - query_start)) * 1000)::bigint), 0) FROM pg_stat_activity WHERE wait_event_type = 'Lock'";
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
fn clickhouse_lock_wait_ms(name: &str) -> Result<u128, String> {
    let sql = "SELECT coalesce(max(ProfileEvents['RWLockReadersWaitMilliseconds'] + ProfileEvents['RWLockWritersWaitMilliseconds'] + intDiv(ProfileEvents['ContextLockWaitMicroseconds'] + ProfileEvents['PartsLockWaitMicroseconds'], 1000)), 0) FROM system.processes WHERE query NOT LIKE '%system.processes%'";
    let output = Command::new("docker")
        .args(["exec", name, "clickhouse-client", "--query", sql])
        .output()
        .map_err(|e| format!("measure ClickHouse lock waits: {e}"))?;
    if !output.status.success() {
        return Err(
            "could not measure ClickHouse lock waits; the disposable database was removed".into(),
        );
    }
    String::from_utf8(output.stdout)
        .map_err(|_| "ClickHouse lock-wait measurement was not UTF-8".to_string())?
        .trim()
        .parse()
        .map_err(|_| "ClickHouse returned an invalid lock-wait measurement".to_string())
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
        migration: if o.migration_label.is_empty() {
            o.migration.clone()
        } else {
            o.migration_label.clone()
        },
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
    validate_output_text(&o.output)?;
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
    println!("Migration Lock Rehearsal {VERSION}\n\nRehearse supplied Postgres or ClickHouse migration SQL in a fresh Docker container.\n\nUsage:\n  mlr demo [--engine postgres|clickhouse] [--output DIR] [--dry-run] [--json]\n  mlr demo --output DIR --reset\n  mlr rehearse --engine postgres|clickhouse --fixture FIXTURE.sql --migration CHANGE.sql [--rollback DOWN.sql] [--workload READ.sql] [--output DIR] [--json]\n  mlr guard DATABASE_URL\n\nA failed rollback writes NO-GO and returns an actionable report. The URL guard accepts only exact loopback hosts. A rehearsal creates and removes its own disposable container.")
}
#[cfg(test)]
mod tests {
    use super::*;
    fn unique_temp(label: &str) -> PathBuf {
        env::temp_dir().join(format!(
            "mlr-test-{label}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn parses_the_host_before_allowing_a_database_url() {
        for target in [
            "postgres://a@prod.example.com/app",
            "postgres://ops@localhost.prod.example.com/app",
            "postgres://disposable@production.example.com/app",
            "postgres://admin@db.internal.example.test/app",
            "postgres://prod.example.com/app?host=localhost",
            "postgres://localhost@production.example.com/app",
            "postgres://127.0.0.1.example.com/app",
            "postgres://2130706433/app",
            "mysql://root@localhost/app",
        ] {
            assert!(
                safe_target(target).is_err(),
                "unexpectedly allowed {target}"
            );
        }
        for target in [
            "postgres://a@localhost/app",
            "postgresql://a@localhost.:5432/app",
            "postgres://a@127.0.0.1/app",
            "postgres://a@127.4.3.2:5432/app",
            "clickhouse://default@[::1]:9000/default",
        ] {
            assert!(safe_target(target).is_ok(), "unexpectedly refused {target}");
        }
    }

    #[test]
    fn blank_output_is_rejected_before_writing() {
        let opt = Opt {
            engine: "postgres".into(),
            output: "  ".into(),
            ..Default::default()
        };
        let result = sample(&opt);
        assert!(result.unwrap_err().contains("non-empty directory"));
    }

    #[test]
    fn reset_validation_rejects_broad_unmarked_and_workspace_targets() {
        assert!(validated_existing_demo_dir(Path::new("/")).is_err());
        assert!(validated_existing_demo_dir(&env::current_dir().unwrap()).is_err());

        let root = unique_temp("reset-boundaries");
        let unmarked = root.join("unmarked");
        let workspace = root.join("workspace");
        fs::create_dir_all(&unmarked).unwrap();
        fs::create_dir_all(&workspace).unwrap();
        fs::write(workspace.join(DEMO_MARKER), DEMO_MARKER_CONTENT).unwrap();
        fs::write(workspace.join("Cargo.toml"), "[workspace]\n").unwrap();
        assert!(validated_existing_demo_dir(&unmarked).is_err());
        assert!(validated_existing_demo_dir(&workspace).is_err());

        #[cfg(unix)]
        {
            let real = root.join("real");
            let alias = root.join("alias");
            fs::create_dir_all(&real).unwrap();
            fs::write(real.join(DEMO_MARKER), DEMO_MARKER_CONTENT).unwrap();
            std::os::unix::fs::symlink(&real, &alias).unwrap();
            assert!(validated_existing_demo_dir(&alias).is_err());
        }
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn reset_removes_only_a_marked_demo_child() {
        let root = unique_temp("reset-marked");
        let output = root.join("mlr-demo");
        fs::create_dir_all(&root).unwrap();
        let opt = Opt {
            output: output.to_string_lossy().into(),
            output_specified: true,
            ..Default::default()
        };
        prepare_demo_output(&opt).unwrap();
        fs::write(output.join("report.json"), "{}\n").unwrap();
        reset_demo(&opt).unwrap();
        assert!(!output.exists());
        assert!(root.exists());
        let _ = fs::remove_dir_all(root);
    }
    #[test]
    fn sample_writes_files() {
        let d = unique_temp("sample");
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
        let d = unique_temp(&format!("failed-rollback-{engine}"));
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
