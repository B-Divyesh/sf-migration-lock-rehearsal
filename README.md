# Migration Lock Rehearsal

Rehearse a database migration before production.

Migration Lock Rehearsal is for Postgres or ClickHouse maintainers who need a concrete go/no-go card before a schema change. It starts a fresh Docker database, loads the fixture you provide, runs the migration under an optional workload, checks rollback SQL, and writes a measured report. A failed rollback is always **NO-GO**. Its URL guard accepts exact loopback hosts only.

The static documentation site lives at `https://migration-lock-rehearsal.sociobot.in`.

## Quick demo

The bundled dry-run demo gives a usable sample card:

```sh
cargo run -- demo --dry-run --output ./mlr-demo
cat ./mlr-demo/runbook.md
```

For the Docker-backed rehearsal, run:

```sh
cargo run -- demo --output ./mlr-demo
```

The demo uses invented customer data in `examples/postgres/`. It writes only to the non-blank output folder you name. The Docker-backed command creates a disposable Postgres 16 container and removes it when the run ends.

## Use your migration

Docker must be running. Provide a sanitized fixture, the migration SQL, and optionally its rollback SQL:

```sh
cargo run -- rehearse \
  --fixture ./fixture.sql \
  --migration ./2026-flag.sql \
  --rollback ./2026-flag-down.sql \
  --workload ./read.sql \
  --output ./rehearsal-card
```

Read `./rehearsal-card/report.json` in automation and `./rehearsal-card/runbook.md` during the change review.
Without a rollback file, or when that file fails, the card is **NO-GO** and the CLI exits non-zero after writing both files.

Use `--engine clickhouse` with a ClickHouse fixture and migration. Both engines run the workload while the migration executes and record statement time, observed lock waits, table bytes, and rollback status. Results are estimates from a new container. Use a production-shaped sanitized fixture before relying on them. The rehearsal command has no database URL option.

## Commands

```text
mlr demo [--engine postgres|clickhouse] [--output DIR] [--dry-run] [--json]
mlr demo --output DIR --reset
mlr rehearse --engine postgres|clickhouse --fixture FIXTURE.sql --migration CHANGE.sql [--rollback DOWN.sql] [--workload READ.sql] [--output DIR] [--json]
mlr guard DATABASE_URL
```

`mlr guard` is a safety check for automation. It parses the URL host, accepts only exact localhost or loopback addresses, and rejects substring decoys. The rehearsal command creates its own Docker container instead of taking a database URL.

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

The site makes only same-origin requests and stores no visitor data. The CLI writes reports to your chosen output folder and runs SQL in its new Docker container. See the site’s `/privacy` and `/terms` pages.

## License

MIT. See [LICENSE](LICENSE).
