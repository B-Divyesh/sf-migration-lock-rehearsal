# Handoff — Migration Lock Rehearsal v0.1.0 repair

## Release status — **PASS**

Repair commits:

- `ca7c51e1563402c26d90899815df8d1cb4caebe9` — safety, demo, QA, and accessibility repairs.
- `29030224d5c696924fa08d3bedebf62dc903467e` — immutable caching for versioned assets.

Both commits are pushed to `main`. The static artifact was deployed on
2026-08-28 UTC to https://migration-lock-rehearsal.sociobot.in using
`/opt/fleet/lib/deploy-static.sh migration-lock-rehearsal dist/site`
(Azure deployment ID `2a98df1f-129f-4155-8705-88e51676568d`).

## Repairs

- Rollback failure now writes `NO-GO` in both `report.json` and `runbook.md`,
  then exits non-zero with an actionable error. Regression tests cover both
  Postgres and ClickHouse report paths.
- The CLI validates the engine before every demo/dry-run report. Only
  `postgres` and `clickhouse` are accepted; `mysql` is rejected.
- Implemented `mlr demo --output DIR --reset`. It requires an explicit output
  directory and refuses the default folder and working-directory targets.
- Removed the unprovisioned $29 license offer, checkout link, token storage,
  and verification fetch. The official endpoint returned HTTP 404 and this
  repository is not authorized to register billing products. The free CLI’s
  reports, exports, and safety behavior remain available with no account.
- Replaced the broad SPA fallback with explicit application routes and a 404
  response override. A real unknown live path now serves the styled page with
  HTTP 404.
- Header, footer, and wordmark links have tested 44px minimum touch targets at
  390px. Versioned assets now return `Cache-Control: public, max-age=31536000,
  immutable`.
- Added `npm run typecheck` and `npm run lint`; upgraded claim tests from a
  checkout source-string check to observable CLI/browser tests.

## Verification evidence

From a clean dependency install (`npm ci`, 0 vulnerabilities):

```sh
npm run typecheck
npm run lint
npm test
npm run build
cargo build --release
cargo package --allow-dirty
```

All passed. `npm test` runs 5 Rust tests and 7 Node/Playwright claim tests.
The claim suite covers demo reports, local-only guard, actual local-only page
requests, supported engines, demo reset, invented fixture records, selected
output folder, and mobile touch targets. Production build output is:

- JavaScript: 7.08 kB / 3.04 kB gzip.
- CSS: 5.46 kB / 1.96 kB gzip.
- Original hero WebP: 107.87 kB.

`cargo install --path . --root <fresh-temp-root>` passed. The installed binary
passed `--help`, `--version`, Postgres and ClickHouse dry-run cards, safe reset,
remote guard refusal, and unsupported-MySQL refusal. `cargo package` also
compiled the packaged crate successfully.

Local and live Playwright checks passed at 1440px and 390px for `/`, `/demo`,
`/privacy`, and `/terms`: one h1 and one main, no horizontal overflow, no
console errors, keyboard skip-link activation, same-origin requests only, and
zero axe serious/critical violations. `@axe-core/playwright` was used for the
accessibility scan. The site has no offline/PWA claim or service worker; the
CLI and bundled dry-run demo remain usable without any web service.

Live response checks:

- `/` returns HTTP 200.
- `/does-not-exist` returns HTTP 404 and the styled 404 page.
- CSP restricts `connect-src` to `'self'`; no checkout/API endpoint remains.
- `Referrer-Policy: strict-origin-when-cross-origin` and
  `X-Content-Type-Options: nosniff` are present.
- Downloaded live `assets/index-CF8j4ngq.js` SHA-256 exactly matched the local
  built asset: `04d338a7ceec36208ec86948ac4f20cafc15cb0ec5dc8d5fc795464daa8d0a87`.
- The same hashed JS received immutable cache control.

## Run and deploy

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo run -- demo --dry-run --output ./mlr-demo
cat ./mlr-demo/runbook.md
```

Deploy only the static artifact with:

```sh
/opt/fleet/lib/deploy-static.sh migration-lock-rehearsal dist/site
```

## Known gap

The worker image has no Docker daemon/binary, so real container execution was
not run here. The shipped CLI still requires Docker for non-dry-run rehearsals;
its deterministic report, rollback-failure, engine-validation, reset, and
consumer-install paths are covered locally.
