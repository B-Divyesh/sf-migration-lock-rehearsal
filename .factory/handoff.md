# Handoff — Migration Lock Rehearsal v0.1.0

## Independent verification status — **FAIL**

Candidate `9de38a35115afeedc61a59e98443f496e9c6f6e6` was independently verified
against https://migration-lock-rehearsal.sociobot.in on 2026-08-28 UTC.
The live assets exactly match this candidate, but it is **not releaseable**.

- **P0:** A rollback failure writes a `GO` verdict and exits 0. This is unsafe
  for a go/no-go migration tool.
- **P1:** The advertised live Sociobot checkout is HTTP 404.
- **P1:** `mlr demo --engine mysql --dry-run` manufactures a MySQL-labelled
  report despite MySQL being unsupported.
- **P1:** Documented `mlr demo --reset` is unimplemented.
- **P1:** The live styled unknown-route page is delivered as HTTP 200, not
  HTTP 404.
- **P1:** Header/footer mobile link targets are below the required 44 px.

See `.factory/verification.md` for commands, observable evidence, passing
checks, rate-limit result (30 requests allowed; request 31 was 429 with
`Retry-After: 4`), and non-blocking follow-ups. Real Docker-backed database
execution could not be run in this verifier container because Docker is not
installed; all other listed checks were run.

## What shipped

- A Rust `mlr` CLI for Docker-isolated Postgres and ClickHouse rehearsals.
- `mlr demo` runs bundled sanitized fixtures. `--dry-run` produces an immediate sample go/no-go card without Docker.
- Postgres runs the supplied workload alongside migration SQL, samples `pg_stat_activity` lock waits, records statement time and table bytes, and validates optional rollback SQL.
- ClickHouse runs its fixture, workload, migration, optional rollback, and records statement time and active-part byte movement. Its report labels merge timing as an estimate.
- The CLI refuses remote-looking targets through `mlr guard`; rehearsals create their own disposable Docker containers and remove them after a run.
- A Vite static documentation site in `dist/site/`, with `/demo`, `/privacy`, `/terms`, and a styled `/404` state.
- One-click terminal demo, bundled fixtures, a $29 Sociobot one-time license checkout/restore/verify flow, and no analytics or third-party runtime fonts/scripts.

## Verification

Run from a clean checkout:

```sh
npm install
npm test
npm run build:site
cargo build --release
cargo package
```

Completed in this work order:

- `npm test` passed: 2 Rust tests and 4 claim tests.
- `npm run build:site` passed; deploy root is `dist/site/index.html`.
- `cargo build --release` and `cargo package --allow-dirty` passed. The publishable crate is 51.4 KB compressed.
- `mlr demo --dry-run` wrote both `report.json` and `runbook.md`.
- `mlr demo --engine clickhouse --dry-run` wrote a ClickHouse-labelled report.
- Local site verifier: 547 ms load, no console errors, title/lang/main present, one h1, no missing image alt text, and no unlabeled buttons.
- Axe Playwright scan: zero serious or critical issues. Checked `/`, `/demo`, `/privacy`, and `/terms` at a 390px viewport; every route had one h1 and one main landmark.
- Production static assets: 3.68 KB gzip JS, 1.99 KB gzip CSS, 107.87 KB WebP hero. This is below the static budgets.

## Notes and next steps

- Results are deliberately estimates. Use a production-shaped, sanitized fixture before a deployment. ClickHouse merges may outlive the statement timing.
- The one-time license uses the mandated Sociobot endpoint and has no registered product configuration in this repository; factory registration supplies that later.
- The environment has Playwright Chromium but no system Chrome. The standard Lighthouse CLI could not attach to that browser, so this handoff records the equivalent static-size, local-load, verifier, and Axe results rather than a Lighthouse score.
- The stack is Rust rather than Go because this worker image does not include a Go toolchain. This is within the assigned Rust-or-Go stack decision.
