# Migration Lock Rehearsal

Rehearse a database migration before production.

Migration Lock Rehearsal is for Postgres or ClickHouse maintainers who need a concrete go/no-go card before a schema change. It starts a fresh Docker database, loads a sanitized fixture, applies supplied SQL, checks optional rollback SQL, and writes a report. A failed rollback is always **NO-GO**. It never accepts a remote database URL.

The static documentation site lives at `https://migration-lock-rehearsal.sociobot.in`.

## Quick demo

The bundled demo gives a usable result with no setup beyond Rust:

```sh
cargo run -- demo --dry-run --output ./mlr-demo
cat ./mlr-demo/runbook.md
```

For the Docker-backed rehearsal, run:

```sh
cargo run -- demo --output ./mlr-demo
```

The demo uses invented customer data in `examples/postgres/`. It writes only to the output folder you name. The Docker-backed command creates a disposable Postgres 16 container and removes it when the run ends.

## Use your migration

Docker must be running. Provide a sanitized fixture, the migration SQL, and optionally its rollback SQL:

```sh
cargo run -- rehearse \
  --fixture ./fixture.sql \
  --migration ./2026-flag.sql \
  --rollback ./2026-flag-down.sql \
  --output ./rehearsal-card
```

Read `./rehearsal-card/report.json` in automation and `./rehearsal-card/runbook.md` during the change review.
Without a rollback file, or when that file fails, the card is **NO-GO** and the CLI exits non-zero after writing both files.

Use `--engine clickhouse` with a ClickHouse fixture and migration. Results are estimates from a new, disposable environment. Use a production-shaped sanitized fixture before relying on timing or size movement. This release does not connect to, copy from, or run against production databases.

## Commands

```text
mlr demo [--engine postgres|clickhouse] [--output DIR] [--dry-run] [--json]
mlr demo --output DIR --reset
mlr rehearse --engine postgres|clickhouse --fixture FIXTURE.sql --migration CHANGE.sql [--rollback DOWN.sql] [--output DIR] [--json]
mlr guard DATABASE_URL
```

`mlr guard` is a safety check for automation. It accepts loopback or explicitly disposable targets and rejects production-looking URLs. The rehearsal command itself creates its own Docker container instead of taking a database URL.

## Develop and verify

Requirements: Rust stable, Node 22+, npm, and Docker for a real rehearsal.

```sh
npm install
npm test
npm run build:site  # static deploy output: dist/site/
cargo build --release
```

The exact static deploy command is `npm run build:site`; it places `index.html` at `dist/site/index.html`. `npm test` runs Rust tests and the claim tests. `cargo package` prepares the CLI package for registry review; do not publish it from this repository.

## Privacy

The site has no analytics and loads no third-party scripts or fonts. The CLI stays local to your chosen output folder and its disposable Docker database. See the site’s `/privacy` and `/terms` pages for full details. There is no checkout or account.

## License

MIT. See [LICENSE](LICENSE).
