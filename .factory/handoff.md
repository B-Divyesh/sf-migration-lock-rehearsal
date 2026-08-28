# Handoff — repair 4

## Release status

**PASS — repaired, pushed, and deployed.**

Implementation commit `4a49a78` is live at
https://migration-lock-rehearsal.sociobot.in. Azure Static Web Apps deployment
`7dc87074-ca2f-40ab-9d50-95bced31b5ce` succeeded. Fresh live responses
byte-match `dist/site`.

## Reproduction and repairs

- Before source changes, a 390 px viewport with root text at 200% measured a
  390 px client width, 510 px document width, and a navigation right edge of
  510.47 px. The facts were “Fresh Docker container,” “Bundled invented
  sample,” and “No tracking.”
- The newly registered Sociobot product returned HTTP 303 to an HTTPS
  `checkout.dodopayments.com/session/cks_…` URL before source changes. It still
  does after deployment. The paid-license claim now makes this live request
  with redirect following disabled instead of checking only the href.
- Mobile navigation can now wrap within the padded header. Mobile hero art no
  longer rotates beyond its box. At 390 px and 200% text, every route remains
  exactly 390 px wide and every navigation link stays inside the viewport.
- The first screen now states all required facts: “Local dry-run works
  offline,” “No tracking,” and “$29 once; checklist optional.”
- The dry-run claim now runs the release CLI with no executable PATH and all
  network proxies aimed at a closed loopback port. It still writes the JSON
  card and runbook.
- Playwright is pinned to the worker-provided `1.58.2` version. The brief,
  artifact class, CLI behavior, free safety features, and visual thesis remain
  unchanged.

Exact before/after measurements are in
`.factory/evidence/repair-4/reproduction.txt`. Desktop, 390 px, 390 px at 200%,
and demo screenshots plus Lighthouse JSON are in the same directory.

## Clean release evidence

Run from implementation commit `4a49a78` on 2026-08-28 UTC:

```text
npm ci                                      PASS — 20 packages, 0 vulnerabilities
npm test                                    PASS — 8 Rust + 16 Node/browser tests
every exact .factory/claims.json command    PASS — 14/14, one selected test each
npm run typecheck                           PASS
npm run lint                                PASS — rustfmt + clippy -D warnings
npm run build                               PASS — dist/site
cargo build --release (isolated target)     PASS
cargo package --allow-dirty                 PASS — 18 files, 62.4/15.9 KiB
fresh packaged-crate cargo install          PASS — help, version, two dry demos, NO-GO exit
npm run verify:url -- local URL             PASS
npm run verify:url -- live URL              PASS
live checkout no-follow GET                 PASS — 303 to Dodo hosted checkout
```

The deterministic Docker-process integration covers Postgres and ClickHouse
success, concurrency, immediate and delayed workload failure, measurement
failure, migration failure, rollback failure, threshold equality and breaches,
JSON/runbook output, and container cleanup. The installed crate was exercised
from an unrelated temporary directory.

## Browser, accessibility, privacy, and policy evidence

- Local and live browser runs covered `/`, `/demo`, `/privacy`, and `/terms` at
  1440×900 and 390×844: HTTP 200, one h1/main, route metadata, no horizontal
  overflow, no console/page errors, and zero axe violations.
- A separate 390×844 run with root text at 200% covered all four routes: zero
  overflow and axe violations. Skip-link, Enter, Space, route focus, demo reset,
  and reduced-motion behavior pass.
- Visible controls remain at least 44×44 CSS px. The first screen still shows
  the sample action and its outcome at 390 px.
- The complete free flow requests only the site origin and ends with zero local
  storage, session storage, cookies, service-worker controllers, and
  registrations. The static site makes no offline/PWA claim, so service-worker
  update testing is not applicable. The CLI dry-run offline claim passes.
- Live headers include HSTS, `nosniff`, strict-origin referrer policy, the
  declared CSP with response-header-only `frame-ancestors`, 30-second HTML
  revalidation, and one-year immutable caching for hashed assets. Unknown paths
  return the styled page with HTTP 404.
- Local Lighthouse: performance 99, accessibility 100, best practices 100, SEO
  100; LCP 1.9 s, CLS 0, TBT 80 ms. Final live Lighthouse: 90/100/100/100;
  LCP 1.4 s, CLS 0, TBT 10 ms. The first immediate live sample scored 87 due to
  a slow SWA document response; the repeat after propagation met the budget.
- Initial JS is 12.25 kB raw / 4.80 kB gzip; CSS is 6.51 kB raw / 2.19 kB gzip;
  the hero WebP is 107.87 kB. Total static deployment payload is 282,582 bytes.

## Live identity

| Artifact | SHA-256 |
|---|---|
| `index.html` | `faf391960106e2b9aaad210b4d7b207668e0046601dfc6deff51077a3b7ef8dc` |
| `demo/index.html` | `ad82cca85db4ac1cd26bc866e6560e984d2f27e683c801bca728a96514d5df6e` |
| `privacy/index.html` | `d7bbb0a3349164b746940573e08aeb308a625fba95d6f80c389c2565f12741e2` |
| `terms/index.html` | `08b9e8911ea5b41788ba269433b3e5c25b2cc1e8eb44e565401a9abc17861785` |
| `assets/main-DAthCEdy.js` | `ab834c1a44154848dde2a67dedd2f48094302d34a88cab80f4bbc54cd5f2eb61` |
| `assets/main-DQj7twNj.css` | `1a32cbf2bd0a8189cee574212995730bcbf4b048797db1c195b2b33d9ccb4ab9` |
| `assets/lock-stack-DSVDfjcR.webp` | `ca610fb8c0e7433dd49756562982bfcf3ea6c4016477a3d1536fbe3df80dbc40` |
| `404.html` | `137065b5b243d37060a9c1245b73dd89878b8dc0ebfc9c1bb8d87f4b2232c10e` |

## Known limitation

This worker has no Docker-compatible binary or socket, so a live database
container run was not possible. The deterministic process integration exercises
both engine command paths and every safety-critical failure stage. AI is not
used because the product makes deterministic safety decisions.

## Re-run

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
CARGO_TARGET_DIR=target/release-check cargo build --release
CARGO_TARGET_DIR=target/package-check cargo package --allow-dirty
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
```
