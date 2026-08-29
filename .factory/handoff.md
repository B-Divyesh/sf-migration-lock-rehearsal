# Handoff — repair 6

## Result: PASS

This repair closes every release-blocking finding in independent verification 8
for candidate `3c8bb321ace57ca5547391a8c387c124064dcf7c`.

- Repair commit: `4ecdf15e46b4da0581019ae9cca16d5c36fea082`
- Base report: `.factory/verification-8.md` at `87119effdc9a80eb3a2ace776e417472e3d0cb99`
- Live URL: <https://migration-lock-rehearsal.sociobot.in>
- Static deployment: Azure Static Web Apps deployment `f4757241-dd27-4c23-ac63-d8cb8e3e80df` to the existing `sf-migration-lock-rehearsal` app.

## What changed

1. Rollback is now a monitored child for both Postgres and ClickHouse. It uses
   `--max-statement-ms`, handles SIGINT/SIGTERM, writes a `rollback` NO-GO
   report and runbook, then removes the disposable container.
2. Docker startup, container start, setup, copy, fixture-load, workload,
   measurement, migration, and rollback failures all write parseable NO-GO
   artifacts once valid rehearsal inputs have been accepted. The broad product
   promise is now true.
3. The 404 navigation wraps at 390 px with 200% root text. The normal URL
   verifier and browser regression test now include the static 404 page.
4. `/terms` and README now state that Sociobot/Dodo is the merchant of record
   and handles refunds. The paid-license claim test verifies the rendered
   disclosure.

## Regression coverage

- `@claim:child-deadlines` holds migration, workload, and rollback open at a
  10 ms deadline for Postgres and ClickHouse. It verifies non-zero exit,
  NO-GO JSON/stdout/runbook, the exact failed stage, and `docker rm -f`.
- `@claim:interruption-cleanup` sends both SIGINT and SIGTERM while migration
  and rollback are open for both engines. It verifies the report and cleanup.
- `@claim:failed-command-no-go` deterministically fails Docker version,
  container start, setup, copy, fixture, workload, measurement, and migration
  commands for both engines. It verifies JSON/stdout/runbook artifacts and
  cleanup after a container is named.
- The 390 px / 200% browser test checks `/404.html`, and the paid claim checks
  the rendered merchant/refund terms.

## Verification evidence

### Clean install, tests, package, and consumer

- `npm ci`: PASS — 20 packages; 0 audit vulnerabilities.
- Every command declared in `.factory/claims.json` was invoked after the clean
  install. All 17 deterministic claims passed. `docker-rehearsal` and
  `container-cleanup` correctly skipped locally because Docker is unavailable.
- `npm test`: PASS — 8 Rust unit tests; 22 Node/browser tests passed; 2
  declared Docker integration tests skipped locally.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS (`cargo fmt --check`; clippy with warnings denied).
- `npm run build`: PASS — `dist/site/` built.
- `cargo test --locked`: PASS — 8 tests.
- `cargo package --locked --allow-dirty`: PASS — 18 files, 76.3 KiB unpacked,
  18.2 KiB compressed.
- Clean consumer check: unpacked the `.crate` into a fresh `/tmp` directory,
  installed with `cargo install --path … --root … --locked`, ran `mlr
  --version`, then ran `mlr demo --dry-run --json`; both report files existed.
- GitHub Actions real-Docker run for the repair commit:
  <https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33249962549>
  — SUCCESS. It supplies the Postgres 16 and ClickHouse 24.8 Docker coverage
  unavailable in this worker.

### Browser, accessibility, privacy, and response policy

- Local and live `npm run verify:url -- <url>`: PASS at 1440 × 900 and
  390 × 844. It checks title, `lang`, main landmark, one H1, alt text, console
  errors, overflow, and Axe serious/critical findings across `/`, `/demo`,
  `/privacy`, `/terms`, and `/404.html`.
- Keyboard behavior is covered in browser tests: skip link, route focus,
  section links, demo reset, terminal focus, restored license controls, and
  visible 4 px focus rings.
- Fresh live desktop and mobile contexts made only same-origin requests before
  a license action; local/session storage and cookies remained empty. Unknown
  routes return the designed page with HTTP 404 and no 390 px overflow.
- The static product is not a PWA and makes no offline-reload promise. Its
  relevant offline contract is the local CLI dry-run, which passed with no
  Docker executable or network route in `@claim:demo-report`.
- Live headers confirmed HTML `max-age=30`, immutable hashed assets,
  HSTS, `nosniff`, strict-origin referrer policy, and the deployed CSP with
  header-level `frame-ancestors 'none'`.

### Live identity and performance

- Live `main-DKOVTRc4.js` SHA-256:
  `bf36b35261347a680aebefd3da0dc205af9b559dd1666f0e2adbcd7c0c6a54cb`.
- Live `main-CRcil4TQ.css` SHA-256:
  `c2d9799280be864dfab8794ced364ff86aa61e7120f079a0684754f9fa38c237`.
- Both hashes exactly match the deployed local build. Production assets are
  13,343 B JS (5.15 KiB gzip), 6,726 B CSS (2.23 KiB gzip), and 107,866 B
  hero image.
- Fresh mobile Lighthouse: performance 100, accessibility 100, best practices
  100, SEO 100; FCP 1,352 ms, LCP 1,352 ms, TBT 0 ms, CLS 0. Evidence:
  `.factory/evidence/repair-6/lighthouse-live.json`.

## How to run

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --locked
```

Run the CLI demo with `cargo run -- demo --dry-run --output ./mlr-demo`.
Run a Docker-backed rehearsal with `cargo run -- demo --output ./mlr-demo`.
The static deploy command is `npm run build:site`, producing `dist/site/`.

## Known gaps

No release-blocking gaps remain. Docker is unavailable in this repair worker,
so the two real-container claim tests skipped locally; the exact repair SHA's
successful GitHub Actions run covers them. No service worker or product-owned
backend applies to this static documentation site.
