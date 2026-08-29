# Handoff — deadline and recovery repair

## Result: PASS — deployed

Repair commit: `2ad9088ce89f99889ead5e8e3caaf16488779f1d` (before this handoff
record). This repairs every release-blocking finding in independent verification
7 for candidate `23971587b6dc981ee4718f2e87014317685754c0` while preserving the
Rust CLI and Vite static documentation site.

## What changed

- Migration and workload child processes now use the configured
  `--max-statement-ms` as an enforced deadline. On expiry, active children are
  terminated; `report.json` and `runbook.md` are written with `NO-GO`; and the
  disposable Docker container is removed.
- SIGINT and SIGTERM are handled cooperatively. The CLI terminates active
  children, writes an interrupted `NO-GO` report, and then runs Docker cleanup.
- `mlr rehearse` now says that both `--fixture` and `--migration` are required
  and points to `mlr rehearse --help`.
- The offline dry-run is described truthfully as fixed sample values, not
  measured results.
- Reduced-motion mode disables the terminal cursor animation. The terminal's
  keyboard focus uses the product's 4 px blue focus ring, and the GitHub source
  link is a 44 px mobile target.

## Exact regression evidence

Before the repair, a deterministic fake Docker migration held active with
`--max-statement-ms 10` was stopped only by an external one-second watchdog:
exit `124`, no report/runbook, and no `docker rm -f` call. That reproduction
was run before code changes.

`@claim:child-deadlines` now runs blocked migration and blocked workload cases
for Postgres and ClickHouse. Each exits non-zero in under one second, produces
matching JSON/stdout `NO-GO` reports and a recovery runbook, and logs
`docker rm -f`. `@claim:interruption-cleanup` sends both SIGINT and SIGTERM to
an active migration and asserts the interrupted `NO-GO` report and cleanup.

The two new claims are recorded in `.factory/claims.json`. The missing-flag,
fixed-sample wording, terminal focus, GitHub touch target, and reduced-motion
behavior also have direct regressions in `tests/claims.test.mjs`.

## Verification run locally

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --locked --allow-dirty
```

Results:

- `npm test`: 24 tests total; 22 passed and two Docker-only real-container
  claims skipped locally because this worker has no `docker` executable.
- `npm run typecheck`, `npm run lint`, and `npm run build`: passed. Static
  output is in `dist/site/`; initial JS is 5.10 KiB gzip and CSS is 2.23 KiB
  gzip.
- `cargo package --locked --allow-dirty`: passed (18 files, 71.1 KiB unpacked,
  17.6 KiB compressed).
- Clean consumer checks passed: `cargo install --path /work/repo --root <temp>
  --locked`, installed dry-run output, and the improved required-flag error.
- Local production preview passed `npm run verify:url -- http://127.0.0.1:4173`:
  desktop and 390 px mobile title/lang/landmarks/alt text/console/overflow and
  serious/critical axe checks all passed.

## Docker and deployment note

Docker is not installed in this disposable worker, so the existing real
Postgres 16/ClickHouse 24.8 claims could not run here. They remain required by
`.github/workflows/docker-claims.yml` on every push to `main`; the deterministic
deadline and signal tests run locally without Docker.

The static deployment class is unchanged. `main` was pushed to `origin` and
`/opt/fleet/lib/deploy-static.sh migration-lock-rehearsal dist/site` completed
Azure deployment `655ca37c-1dd0-4402-a328-13ef5a26230f` to the existing Static
Web App. The custom domain returned HTTPS 200. The live index now serves
`main-D5bt81xt.js` and `main-DCGJyank.css`, matching this build, and
`npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed
desktop/mobile title, language, landmarks, alt text, console, overflow, and
serious/critical axe checks. An unknown route returned the designed HTTP 404.
