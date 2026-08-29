use std::{
    env, fs,
    net::IpAddr,
    path::{Path, PathBuf},
    process::{Child, Command, ExitCode},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

const VERSION: &str = "0.1.0";
const DEFAULT_MAX_STATEMENT_MS: u128 = 30_000;
const DEFAULT_MAX_LOCK_WAIT_MS: u128 = 1_000;
const DEFAULT_MAX_TABLE_GROWTH_BYTES: u64 = 104_857_600;
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
    max_statement_ms: u128,
    max_lock_wait_ms: u128,
    max_table_growth_bytes: u64,
}

impl Default for Opt {
    fn default() -> Self {
        Self {
            engine: String::new(),
            fixture: String::new(),
            migration: String::new(),
            rollback: String::new(),
            workload: String::new(),
            output: String::new(),
            dry: false,
            json: false,
            reset: false,
            output_specified: false,
            migration_label: String::new(),
            max_statement_ms: DEFAULT_MAX_STATEMENT_MS,
            max_lock_wait_ms: DEFAULT_MAX_LOCK_WAIT_MS,
            max_table_growth_bytes: DEFAULT_MAX_TABLE_GROWTH_BYTES,
        }
    }
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
            "--max-statement-ms" => {
                o.max_statement_ms = parse_limit(&get(i)?, "--max-statement-ms")?;
                i += 1
            }
            "--max-lock-wait-ms" => {
                o.max_lock_wait_ms = parse_limit(&get(i)?, "--max-lock-wait-ms")?;
                i += 1
            }
            "--max-table-growth-bytes" => {
                o.max_table_growth_bytes = parse_limit(&get(i)?, "--max-table-growth-bytes")?;
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

fn parse_limit<T>(value: &str, option: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    value
        .parse()
        .map_err(|_| format!("{option} needs a non-negative whole number"))
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
        return Err("Docker is required. Install Docker, or use `mlr demo --dry-run` to inspect the bundled report".into());
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
    docker(&["exec", &name, "mkdir", "-p", "/work"])?;
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
    psql(&name, &["-v", "ON_ERROR_STOP=1", "-f", "/work/fixture.sql"])?;
    let mut measurements = Measurements::default();
    let before = match table_bytes(&name) {
        Ok(value) => value,
        Err(error) => {
            return write_failure_report(o, "postgres", "measurement", &error, measurements)
        }
    };
    measurements.table_bytes_before = Some(before);
    let mut workload = if !o.workload.is_empty() {
        match Command::new("docker")
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
                "/work/workload.sql",
            ])
            .spawn()
        {
            Ok(child) => Some(child),
            Err(error) => {
                return write_failure_report(
                    o,
                    "postgres",
                    "workload",
                    &format!("could not start supplied workload: {error}"),
                    measurements,
                )
            }
        }
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
            cancel_child(&mut workload);
            return write_failure_report(
                o,
                "postgres",
                "migration",
                &format!("could not start migration: {error}"),
                measurements,
            );
        }
    };
    let mut observed_wait = 0;
    loop {
        if let Err(error) = check_workload(&mut workload, "Postgres") {
            let _ = migration.kill();
            let _ = migration.wait();
            measurements.duration_ms = Some(start.elapsed().as_millis());
            measurements.max_lock_wait_ms = Some(observed_wait);
            return write_failure_report(o, "postgres", "workload", &error, measurements);
        }
        match migration.try_wait() {
            Ok(Some(status)) if status.success() => break,
            Ok(Some(_)) => {
                cancel_child(&mut workload);
                measurements.duration_ms = Some(start.elapsed().as_millis());
                measurements.max_lock_wait_ms = Some(observed_wait);
                return write_failure_report(
                    o,
                    "postgres",
                    "migration",
                    "migration command failed",
                    measurements,
                );
            }
            Ok(None) => {
                observed_wait = match pg_lock_wait_ms(&name) {
                    Ok(wait) => observed_wait.max(wait),
                    Err(error) => {
                        let _ = migration.kill();
                        let _ = migration.wait();
                        cancel_child(&mut workload);
                        measurements.duration_ms = Some(start.elapsed().as_millis());
                        return write_failure_report(
                            o,
                            "postgres",
                            "measurement",
                            &error,
                            measurements,
                        );
                    }
                };
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(error) => {
                cancel_child(&mut workload);
                measurements.duration_ms = Some(start.elapsed().as_millis());
                measurements.max_lock_wait_ms = Some(observed_wait);
                return write_failure_report(
                    o,
                    "postgres",
                    "migration",
                    &format!("could not monitor migration: {error}"),
                    measurements,
                );
            }
        }
    }
    let duration = start.elapsed().as_millis();
    measurements.duration_ms = Some(duration);
    measurements.max_lock_wait_ms = Some(observed_wait);
    if let Err(error) = finish_workload(&mut workload, "Postgres") {
        return write_failure_report(o, "postgres", "workload", &error, measurements);
    }
    let after = match table_bytes(&name) {
        Ok(value) => value,
        Err(error) => {
            return write_failure_report(o, "postgres", "measurement", &error, measurements)
        }
    };
    measurements.table_bytes_after = Some(after);
    let rolled = if o.rollback.is_empty() {
        false
    } else {
        psql(
            &name,
            &["-v", "ON_ERROR_STOP=1", "-f", "/work/rollback.sql"],
        )
        .is_ok()
    };
    drop(cleanup);
    write_report(
        o,
        completed_report(
            "postgres",
            o,
            measurements,
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
        return Err("Docker is required. Install Docker, or use `mlr demo --dry-run` to inspect the bundled report".into());
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
    docker(&["exec", &name, "mkdir", "-p", "/work"])?;
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
    let mut measurements = Measurements::default();
    let before = match clickhouse_bytes(&name) {
        Ok(value) => value,
        Err(error) => {
            return write_failure_report(o, "clickhouse", "measurement", &error, measurements)
        }
    };
    measurements.table_bytes_before = Some(before);
    let mut workload = if !o.workload.is_empty() {
        docker(&["cp", &o.workload, &format!("{name}:/work/workload.sql")])?;
        match Command::new("docker")
            .args([
                "exec",
                &name,
                "sh",
                "-lc",
                "clickhouse-client --multiquery < /work/workload.sql",
            ])
            .spawn()
        {
            Ok(child) => Some(child),
            Err(error) => {
                return write_failure_report(
                    o,
                    "clickhouse",
                    "workload",
                    &format!("could not start supplied workload: {error}"),
                    measurements,
                )
            }
        }
    } else {
        None
    };
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
            cancel_child(&mut workload);
            return write_failure_report(
                o,
                "clickhouse",
                "migration",
                &format!("could not start migration: {error}"),
                measurements,
            );
        }
    };
    let mut observed_wait = 0;
    loop {
        if let Err(error) = check_workload(&mut workload, "ClickHouse") {
            let _ = migration.kill();
            let _ = migration.wait();
            measurements.duration_ms = Some(start.elapsed().as_millis());
            measurements.max_lock_wait_ms = Some(observed_wait);
            return write_failure_report(o, "clickhouse", "workload", &error, measurements);
        }
        match migration.try_wait() {
            Ok(Some(status)) if status.success() => break,
            Ok(Some(_)) => {
                cancel_child(&mut workload);
                measurements.duration_ms = Some(start.elapsed().as_millis());
                measurements.max_lock_wait_ms = Some(observed_wait);
                return write_failure_report(
                    o,
                    "clickhouse",
                    "migration",
                    "migration command failed",
                    measurements,
                );
            }
            Ok(None) => {
                observed_wait = match clickhouse_lock_wait_ms(&name) {
                    Ok(wait) => observed_wait.max(wait),
                    Err(error) => {
                        let _ = migration.kill();
                        let _ = migration.wait();
                        cancel_child(&mut workload);
                        measurements.duration_ms = Some(start.elapsed().as_millis());
                        return write_failure_report(
                            o,
                            "clickhouse",
                            "measurement",
                            &error,
                            measurements,
                        );
                    }
                };
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(error) => {
                cancel_child(&mut workload);
                measurements.duration_ms = Some(start.elapsed().as_millis());
                return write_failure_report(
                    o,
                    "clickhouse",
                    "migration",
                    &format!("could not monitor migration: {error}"),
                    measurements,
                );
            }
        }
    }
    let duration = start.elapsed().as_millis();
    measurements.duration_ms = Some(duration);
    observed_wait = match clickhouse_lock_wait_ms(&name) {
        Ok(wait) => observed_wait.max(wait),
        Err(error) => {
            cancel_child(&mut workload);
            return write_failure_report(o, "clickhouse", "measurement", &error, measurements);
        }
    };
    measurements.max_lock_wait_ms = Some(observed_wait);
    if let Err(error) = finish_workload(&mut workload, "ClickHouse") {
        return write_failure_report(o, "clickhouse", "workload", &error, measurements);
    }
    let after = match clickhouse_bytes(&name) {
        Ok(value) => value,
        Err(error) => {
            return write_failure_report(o, "clickhouse", "measurement", &error, measurements)
        }
    };
    measurements.table_bytes_after = Some(after);
    let rolled = !o.rollback.is_empty() && clickhouse_file(&name, "/work/rollback.sql").is_ok();
    drop(cleanup);
    write_report(
        o,
        completed_report(
            "clickhouse",
            o,
            measurements,
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

fn cancel_child(child: &mut Option<Child>) {
    if let Some(child) = child.as_mut() {
        let _ = child.kill();
        let _ = child.wait();
    }
    *child = None;
}

fn check_workload(child: &mut Option<Child>, engine: &str) -> Result<(), String> {
    let status = match child.as_mut() {
        Some(child) => child
            .try_wait()
            .map_err(|error| format!("could not monitor {engine} workload: {error}"))?,
        None => None,
    };
    if let Some(status) = status {
        *child = None;
        if !status.success() {
            return Err(format!(
                "supplied {engine} workload command failed with {}",
                status
                    .code()
                    .map_or_else(|| "a signal".into(), |code| format!("exit {code}"))
            ));
        }
    }
    Ok(())
}

fn finish_workload(child: &mut Option<Child>, engine: &str) -> Result<(), String> {
    check_workload(child, engine)?;
    let status = match child.as_mut() {
        Some(child) => child
            .wait()
            .map_err(|error| format!("could not wait for {engine} workload: {error}"))?,
        None => return Ok(()),
    };
    *child = None;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "supplied {engine} workload command failed with {}",
            status
                .code()
                .map_or_else(|| "a signal".into(), |code| format!("exit {code}"))
        ))
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
fn table_bytes(name: &str) -> Result<u64, String> {
    let sql="SELECT COALESCE(sum(pg_total_relation_size(oid)),0) FROM pg_class WHERE relkind IN ('r','m')";
    let output = Command::new("docker")
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
        .output()
        .map_err(|error| format!("measure Postgres table bytes: {error}"))?;
    parse_measurement(output, "Postgres table bytes")
}
fn pg_lock_wait_ms(name: &str) -> Result<u128, String> {
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
        .output()
        .map_err(|error| format!("measure Postgres lock waits: {error}"))?;
    parse_measurement(out, "Postgres lock wait")
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
fn clickhouse_bytes(name: &str) -> Result<u64, String> {
    let sql = "SELECT coalesce(sum(bytes_on_disk),0) FROM system.parts WHERE active";
    let output = Command::new("docker")
        .args(["exec", name, "clickhouse-client", "--query", sql])
        .output()
        .map_err(|error| format!("measure ClickHouse table bytes: {error}"))?;
    parse_measurement(output, "ClickHouse table bytes")
}

fn parse_measurement<T>(output: std::process::Output, label: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    if !output.status.success() {
        return Err(format!("{label} command failed; no value was recorded"));
    }
    String::from_utf8(output.stdout)
        .map_err(|_| format!("{label} was not UTF-8"))?
        .trim()
        .parse()
        .map_err(|_| format!("{label} was not a non-negative whole number"))
}
fn clickhouse_lock_wait_ms(name: &str) -> Result<u128, String> {
    let sql = "SELECT coalesce(max(ProfileEvents['RWLockReadersWaitMilliseconds'] + ProfileEvents['RWLockWritersWaitMilliseconds'] + intDiv(ProfileEvents['ContextLockWaitMicroseconds'] + ProfileEvents['PartsLockWaitMicroseconds'], 1000)), 0) FROM system.processes WHERE query NOT LIKE '%system.processes%'";
    let output = Command::new("docker")
        .args(["exec", name, "clickhouse-client", "--query", sql])
        .output()
        .map_err(|e| format!("measure ClickHouse lock waits: {e}"))?;
    parse_measurement(output, "ClickHouse lock wait")
}

#[derive(Clone, Copy, Serialize)]
struct Thresholds {
    max_statement_ms: u128,
    max_lock_wait_ms: u128,
    max_table_growth_bytes: u64,
}

#[derive(Serialize)]
struct Report {
    engine: String,
    migration: String,
    duration_ms: Option<u128>,
    max_lock_wait_ms: Option<u128>,
    table_bytes_before: Option<u64>,
    table_bytes_after: Option<u64>,
    table_growth_bytes: Option<u64>,
    rollback_checked: bool,
    thresholds: Thresholds,
    verdict: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    failure: Option<String>,
    decision_reasons: Vec<String>,
    notes: Vec<String>,
}

#[derive(Default)]
struct Measurements {
    duration_ms: Option<u128>,
    max_lock_wait_ms: Option<u128>,
    table_bytes_before: Option<u64>,
    table_bytes_after: Option<u64>,
}

fn thresholds(o: &Opt) -> Thresholds {
    Thresholds {
        max_statement_ms: o.max_statement_ms,
        max_lock_wait_ms: o.max_lock_wait_ms,
        max_table_growth_bytes: o.max_table_growth_bytes,
    }
}

fn migration_label(o: &Opt) -> String {
    if o.migration_label.is_empty() {
        o.migration.clone()
    } else {
        o.migration_label.clone()
    }
}

fn completed_report(
    engine: &str,
    o: &Opt,
    measurements: Measurements,
    rollback: bool,
    mut notes: Vec<String>,
) -> Report {
    let table_growth_bytes = match (
        measurements.table_bytes_before,
        measurements.table_bytes_after,
    ) {
        (Some(before), Some(after)) => Some(after.saturating_sub(before)),
        _ => None,
    };
    let mut decision_reasons = Vec::new();
    if !rollback {
        decision_reasons.push("Rollback SQL was missing or failed.".into());
    }
    if measurements.duration_ms > Some(o.max_statement_ms) {
        decision_reasons.push(format!(
            "Statement time exceeded the {} ms limit.",
            o.max_statement_ms
        ));
    }
    if measurements.max_lock_wait_ms > Some(o.max_lock_wait_ms) {
        decision_reasons.push(format!(
            "Lock wait exceeded the {} ms limit.",
            o.max_lock_wait_ms
        ));
    }
    if table_growth_bytes > Some(o.max_table_growth_bytes) {
        decision_reasons.push(format!(
            "Table growth exceeded the {} byte limit.",
            o.max_table_growth_bytes
        ));
    }
    let verdict = if decision_reasons.is_empty() {
        decision_reasons
            .push("All required commands completed within the configured limits.".into());
        "GO"
    } else {
        "NO-GO"
    };
    notes.push(
        "Change the limits only after your team documents an approved release budget.".into(),
    );
    Report {
        engine: engine.into(),
        migration: migration_label(o),
        duration_ms: measurements.duration_ms,
        max_lock_wait_ms: measurements.max_lock_wait_ms,
        table_bytes_before: measurements.table_bytes_before,
        table_bytes_after: measurements.table_bytes_after,
        table_growth_bytes,
        rollback_checked: rollback,
        thresholds: thresholds(o),
        verdict: verdict.into(),
        failure_stage: if rollback {
            None
        } else {
            Some("rollback".into())
        },
        failure: if rollback {
            None
        } else {
            Some("rollback failed or was not supplied".into())
        },
        decision_reasons,
        notes,
    }
}

fn write_failure_report(
    o: &Opt,
    engine: &str,
    stage: &str,
    failure: &str,
    measurements: Measurements,
) -> Result<(), String> {
    let table_growth_bytes = match (
        measurements.table_bytes_before,
        measurements.table_bytes_after,
    ) {
        (Some(before), Some(after)) => Some(after.saturating_sub(before)),
        _ => None,
    };
    let report = Report {
        engine: engine.into(),
        migration: migration_label(o),
        duration_ms: measurements.duration_ms,
        max_lock_wait_ms: measurements.max_lock_wait_ms,
        table_bytes_before: measurements.table_bytes_before,
        table_bytes_after: measurements.table_bytes_after,
        table_growth_bytes,
        rollback_checked: false,
        thresholds: thresholds(o),
        verdict: "NO-GO".into(),
        failure_stage: Some(stage.into()),
        failure: Some(failure.into()),
        decision_reasons: vec![format!("The {stage} stage did not complete: {failure}.")],
        notes: vec![
            "No missing measurement was replaced with zero.".into(),
            format!("Fix the {stage} command, then run the full rehearsal again."),
        ],
    };
    write_report(o, report)
}

fn sample(o: &Opt) -> Result<(), String> {
    validate_engine(&o.engine)?;
    write_report(o, completed_report(&o.engine, o, Measurements { duration_ms: Some(184), max_lock_wait_ms: Some(0), table_bytes_before: Some(32768), table_bytes_after: Some(40960) }, true, vec!["Preview from the bundled sanitized fixture; it is an estimate, not a production measurement.".into(),"The migration adds a defaulted column. Rehearse against a production-shaped fixture before deployment.".into(),"Rollback SQL completed in the same disposable environment.".into()]))
}

fn markdown_inline(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| {
            if character.is_control() {
                character.escape_default().collect::<Vec<_>>()
            } else if character == '`' {
                vec!['\'']
            } else {
                vec![character]
            }
        })
        .collect()
}

fn write_report(o: &Opt, r: Report) -> Result<(), String> {
    validate_output_text(&o.output)?;
    fs::create_dir_all(&o.output).map_err(|e| e.to_string())?;
    let dir = PathBuf::from(&o.output);
    let json = serde_json::to_string_pretty(&r)
        .map_err(|error| format!("serialize report: {error}"))?
        + "\n";
    fs::write(dir.join("report.json"), &json).map_err(|e| e.to_string())?;
    let show = |value: Option<u128>| value.map_or_else(|| "not measured".into(), |v| v.to_string());
    let show_bytes =
        |value: Option<u64>| value.map_or_else(|| "not measured".into(), |v| v.to_string());
    let mut md=format!("# Migration runbook\n\n**Verdict: {}**\n\n- Engine: {}\n- Migration: `{}`\n- Statement time: {} ms\n- Maximum observed lock wait: {} ms\n- Table bytes: {} → {}\n- Table growth: {} bytes\n- Rollback checked: {}\n\n## Decision limits\n\n- Statement time: at most {} ms\n- Lock wait: at most {} ms\n- Table growth: at most {} bytes\n\n## Decision reasons\n\n",r.verdict,markdown_inline(&r.engine),markdown_inline(&r.migration),show(r.duration_ms),show(r.max_lock_wait_ms),show_bytes(r.table_bytes_before),show_bytes(r.table_bytes_after),show_bytes(r.table_growth_bytes),r.rollback_checked,r.thresholds.max_statement_ms,r.thresholds.max_lock_wait_ms,r.thresholds.max_table_growth_bytes);
    for reason in &r.decision_reasons {
        md.push_str(&format!("- {}\n", markdown_inline(reason)));
    }
    if let (Some(stage), Some(failure)) = (&r.failure_stage, &r.failure) {
        md.push_str(&format!(
            "\n## Failed stage\n\n- Stage: {}\n- Failure: {}\n- Recovery: Fix this stage, then run the full rehearsal again.\n",
            markdown_inline(stage),
            markdown_inline(failure)
        ));
    }
    md.push_str("\n## Operator notes\n\n");
    for note in &r.notes {
        md.push_str(&format!("- {}\n", markdown_inline(note)));
    }
    fs::write(dir.join("runbook.md"), md).map_err(|e| e.to_string())?;
    if o.json {
        println!("{}", json.trim());
    } else {
        println!("wrote {}/report.json", o.output);
        println!("wrote {}/runbook.md", o.output);
    }
    if r.verdict == "NO-GO" {
        let reason = r.failure.as_deref().unwrap_or_else(|| {
            r.decision_reasons
                .first()
                .map_or("a release limit was exceeded", String::as_str)
        });
        return Err(format!(
            "{reason}; wrote a NO-GO report. Fix the cause before proceeding"
        ));
    }
    Ok(())
}
fn usage() {
    println!("Migration Lock Rehearsal {VERSION}\n\nRehearse supplied Postgres or ClickHouse migration SQL in a fresh Docker container.\n\nUsage:\n  mlr demo [--engine postgres|clickhouse] [--output DIR] [--dry-run] [--json] [LIMITS]\n  mlr demo --output DIR --reset\n  mlr rehearse --engine postgres|clickhouse --fixture FIXTURE.sql --migration CHANGE.sql [--rollback DOWN.sql] [--workload READ.sql] [--output DIR] [--json] [LIMITS]\n  mlr guard DATABASE_URL\n\nLimits:\n  --max-statement-ms N          Default: {DEFAULT_MAX_STATEMENT_MS}\n  --max-lock-wait-ms N          Default: {DEFAULT_MAX_LOCK_WAIT_MS}\n  --max-table-growth-bytes N    Default: {DEFAULT_MAX_TABLE_GROWTH_BYTES}\n\nA failed command, rollback, or exceeded limit writes NO-GO and exits non-zero. The URL guard accepts only exact loopback hosts. A rehearsal creates and removes its own disposable container.")
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
        let report = completed_report(
            engine,
            &opt,
            Measurements {
                duration_ms: Some(1),
                max_lock_wait_ms: Some(0),
                table_bytes_before: Some(1),
                table_bytes_after: Some(1),
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
