# Migration Lock Rehearsal

Rehearse a database migration before production.

Migration Lock Rehearsal is for Postgres or ClickHouse maintainers who need a go/no-go report before a migration. It starts a fresh Docker database and loads your fixture. It runs the migration with an optional workload. It checks rollback SQL and writes a go/no-go report. A failed Docker command, failed rollback, or exceeded limit is always **NO-GO**. Its URL guard accepts exact loopback hosts only.

The static documentation site lives at `https://migration-lock-rehearsal.sociobot.in`.

## Quick demo

The bundled dry-run demo works locally without Docker or network access. It writes a sample go/no-go report with fixed sample values:

```sh
cargo run -- demo --dry-run --output ./mlr-demo
cat ./mlr-demo/runbook.md
```

For the Docker-backed sample rehearsal, run:

```sh
cargo run -- demo --output ./mlr-demo
```

The demo uses invented customer data in `examples/postgres/`. It writes only to the non-blank output folder you name. The Docker-backed command creates a disposable Postgres 16 container and removes it when the run ends.

## Install and use your migration

Install the CLI from this repository:

```sh
cargo install --git https://github.com/B-Divyesh/sf-migration-lock-rehearsal --locked
```

Then run your migration:

Docker must be running. Provide a sanitized fixture, the migration SQL, and optionally its rollback SQL:

```sh
cargo run -- rehearse \
  --fixture ./fixture.sql \
  --migration ./2026-flag.sql \
  --rollback ./2026-flag-down.sql \
  --workload ./read.sql \
  --output ./rehearsal-report
```

Read `./rehearsal-report/report.json` in automation and `./rehearsal-report/runbook.md` during the change review.
When any Docker command in a rehearsal fails, the report is **NO-GO**. The CLI writes both files with the failed stage and recovery step, then exits non-zero. Missing measurements are `null`, never zero.

Each migration, workload, and rollback command must finish within `--max-statement-ms`. On expiry, the CLI terminates the active command, writes **NO-GO**, and removes the disposable container. SIGINT and SIGTERM follow the same recovery path.

Use `--engine clickhouse` with a ClickHouse fixture and migration. Both engines run the workload while the migration executes. They record statement time, lock waits, table bytes, table growth, and rollback status. Results are estimates from a new container. Use a production-shaped sanitized fixture before relying on them. The rehearsal command has no database URL option.

The default release limits are 30,000 ms statement time, 1,000 ms lock wait, and 104,857,600 bytes table growth. Override them with `--max-statement-ms`, `--max-lock-wait-ms`, and `--max-table-growth-bytes`. Every configured limit appears in the JSON report and runbook. An exceeded limit writes **NO-GO** and exits non-zero.

## Commands

```text
mlr demo [--engine postgres|clickhouse] [--output DIR] [--dry-run] [--json] [LIMITS]
mlr demo --output DIR --reset
mlr rehearse --engine postgres|clickhouse --fixture FIXTURE.sql --migration CHANGE.sql [--rollback DOWN.sql] [--workload READ.sql] [--output DIR] [--json] [LIMITS]
mlr guard DATABASE_URL
```

`mlr guard` is a safety check for automation. It parses the URL host, accepts only exact localhost or loopback addresses, and rejects substring decoys. The rehearsal command creates its own Docker container instead of taking a database URL.

`mlr rehearse` requires `--fixture` and `--migration`. Run `mlr rehearse --help` to see a complete command.

Demo reset is deliberately narrow. `mlr demo --output ./mlr-demo --reset` removes only a real directory marked by a prior `mlr demo` run. It refuses roots, workspaces, home/current directories, aliases, symlinks, and unmarked folders.

## Develop and verify

Requirements: Rust stable, Node 22+, npm, and Docker for a real rehearsal.

```sh
npm ci
npm test
npm run build:site  # static deploy output: dist/site/
cargo build --release
```

The exact static deploy command is `npm run build:site`; it places `index.html` at `dist/site/index.html`. `npm test` runs Rust tests and the claim tests. `cargo package` prepares the CLI package for registry review; do not publish it from this repository.

## Privacy

Without a license action, the site makes only same-origin requests and stores no visitor data. The CLI writes reports to your chosen output folder and runs SQL in its new Docker container. See the site’s `/privacy` and `/terms` pages.

## Operator license

The optional operator license costs $29 once. It adds the browser-based operator review checklist. CLI reports and safety checks do not require a license.

Purchase uses Sociobot’s hosted checkout. Sociobot/Dodo is the merchant of record, and refunds are handled there. A returned or pasted token is stored under `sb_license:migration-lock-rehearsal`, sent only to `api.sociobot.in`, and verified at most once daily. Use **Remove saved license** to delete it.

## License

MIT. See [LICENSE](LICENSE).
