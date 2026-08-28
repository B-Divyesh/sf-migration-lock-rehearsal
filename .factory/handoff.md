# Handoff — repair 3

## Release status

The repository repair is implemented, tested, pushed, and deployed. The live
site byte-matches the production build at
https://migration-lock-rehearsal.sociobot.in.

One external release dependency remains: the required Sociobot checkout URL
returns HTTP 404 because the billing product is not enabled. License return,
restore, verification, daily caching, invalid-license handling, removal, legal
copy, and paid checklist behavior are implemented and verified. Repository
instructions prohibit changing billing infrastructure from this worker.

## Safety repairs

- A supplied workload that exits non-zero now writes NO-GO, records
  failure_stage workload, leaves a recovery runbook, and exits 1 for both
  engines. The CLI waits for the workload status even when migration finishes
  first.
- Failed table-size and lock-wait measurements now write NO-GO. Missing values
  are JSON null; they are never replaced with zero.
- Failed migration commands now write a JSON report and runbook with the failed
  stage and recovery action before exiting non-zero.
- Verdicts now apply configurable statement-time, lock-wait, and table-growth
  limits. Defaults are 30,000 ms, 1,000 ms, and 104,857,600 bytes. The values,
  observed measurements, and decision reasons appear in both artifacts.
- serde_json now serializes reports, so control characters in valid Unix
  filenames cannot corrupt file or stdout JSON.
- The original verifier double was reproduced before repair: Postgres and
  ClickHouse each exited 0 with GO after workload exit 19. The final isolated
  release binary exits 1 with NO-GO for both. See
  evidence/repair-3/reproduction.txt.

## Site and paid-flow repairs

- Added the $29 one-time operator license section and a paid operator review
  checklist. Returned and pasted tokens use the namespaced Sociobot license
  key, go only to api.sociobot.in, verify at most once daily, reconcile invalid
  licenses, and can be removed.
- The in-page “How it works” link now scrolls to and focuses its section at
  desktop and 390 px. Cross-route and back/forward navigation retain SPA focus
  handling.
- /demo, /privacy, and /terms now ship separate HTML documents with their own
  title, description, canonical, Open Graph, and Twitter metadata.
- The demo banner now uses a valid div status-role combination.
- The 404 document now includes the standard header, navigation, skip link,
  main landmark, footer, metadata, and 44 px targets.
- CSP permits only the required Sociobot verification origin. The free flow
  still makes only same-origin requests and stores nothing.

## Verification evidence

Run from the pushed implementation commit b6b850f:

    npm ci                                      PASS — 20 packages, 0 vulnerabilities
    npm test                                    PASS — 8 Rust + 15 Node/browser tests
    every exact .factory/claims.json command    PASS — 14/14
    npm run typecheck                           PASS
    npm run lint                                PASS — rustfmt + clippy -D warnings
    npm run build                               PASS — dist/site
    cargo build --release (isolated target)     PASS
    cargo package --allow-dirty                 PASS — 18 files, 62.3 KiB / 15.8 KiB
    fresh packaged-crate cargo install          PASS — help, version, both dry demos
    npm run verify:url -- local URL             PASS
    npm run verify:url -- live URL              PASS

The deterministic Docker-process integration covers Postgres and ClickHouse
success, immediate and delayed workload failure, measurement failure, migration
failure, rollback failure, limit breaches, process overlap, and cleanup.
The exact 900,000 ms lock wait and 999,999,999,999 byte table measurement now
produce NO-GO.

Desktop 1440×900 and mobile 390×844 checks covered all routes, one h1, one
main, no overflow, every interactive target at least 44 px, keyboard skip and
section focus, reduced motion, route announcements, no console/page errors,
zero axe violations, no default storage or cookies, and no service worker.
Screenshots are in evidence/repair-3/.

Live Lighthouse: performance 100, accessibility 100, best practices 100, SEO
100; LCP 1,410 ms, CLS 0, TBT 24 ms. Initial JavaScript is 12.24 kB raw /
4.79 kB gzip; CSS is 6.41 kB raw / 2.17 kB gzip; hero WebP is 107.87 kB.

Live headers include HSTS, nosniff, strict-origin referrer policy, the declared
CSP, 30-second HTML revalidation, and one-year immutable caching for hashed
assets. /, /demo, /privacy, and /terms return 200 with their own server HTML
metadata. Unknown paths return the styled HTTP 404.

Deployment d4d176d6-742f-4462-8248-ff6d271f77e0 succeeded. Local and live
SHA-256 values match:

| Artifact | SHA-256 |
|---|---|
| index.html | 1b09ffdb55d6856b6a91ff6c048a0ea8572065c16a1eaa1b85cba744df5cbe21 |
| assets/main-DX5UjtqP.js | b01ff5afaa795b99704210e9ab34acfbcb397ba9fa7c11a6441c470f9500527b |
| assets/main-Gaq3n0nI.css | c84a478f858967317e893efc1e08f74bcae2936291cde4d0c81cfbffe9edeb43 |
| assets/lock-stack-DSVDfjcR.webp | ca610fb8c0e7433dd49756562982bfcf3ea6c4016477a3d1536fbe3df80dbc40 |
| 404.html | 137065b5b243d37060a9c1245b73dd89878b8dc0ebfc9c1bb8d87f4b2232c10e |

## Applicability and limitations

- This remains a Rust single-binary CLI plus static Vite documentation site.
- No Docker-compatible binary or socket exists in this worker, so a real
  database-container run was unavailable. Deterministic process integration
  exercises both engine command paths and all reported failures.
- There is no offline claim or service worker. The static shell has immutable
  versioned assets; absence of stale worker/update behavior was verified.
- AI would not improve this deterministic safety decision, so no model call or
  key was added.
- Factory billing must enable migration-lock-rehearsal before release. Recheck
  that the hosted checkout URL redirects; it returned 404 at final verification.

## Re-run

    npm ci
    npm test
    npm run typecheck
    npm run lint
    npm run build
    CARGO_TARGET_DIR=target/release-check cargo build --release
    CARGO_TARGET_DIR=target/package-final cargo package --allow-dirty
    npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
