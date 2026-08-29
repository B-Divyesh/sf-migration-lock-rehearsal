# Independent verification 10

## Verdict: PASS

Candidate `5567fa95db7361994249b5049f6dedc237441072` passes independent
product verification on 2026-08-29 UTC. The tested deployment is
<https://migration-lock-rehearsal.sociobot.in>. Product code was not modified.

The checkout started clean at the requested SHA. A fresh production build
byte-matches the live deployment, including every HTML route, the designed
404, demo recording, metadata images, hero image, JavaScript, and CSS.

## Mandatory first read

The cold first screen passes the release gate:

- What it does: “Rehearse your migration before production.”
- Who it is for: Postgres and ClickHouse maintainers who need lock-wait,
  table-growth, and rollback results before release.
- What to click: **Try it with sample data**, paired with “Watch the bundled
  go/no-go report.”

The action is visible at 1440 × 900 and 390 × 844. One keyboard or pointer
activation opens `/?demo=1`, immediately shows the sample recording and report,
and identifies the sandbox as “Demo — sample data, nothing is saved.” Reset
demo restarts the recording; Install the CLI is the start-for-real action.

## Claims gate

`.factory/claims.json` exists and declares 20 claims. After `npm ci`, every
listed command was run separately. All 20 commands exited successfully. The
two real-container tests used their declared local skip because this worker
has no Docker executable; the same tests passed on the requested candidate in
the final-SHA Docker workflow described below.

| Claim | Result |
| --- | --- |
| demo-report | PASS |
| local-only | PASS |
| site-private | PASS |
| supported-engines | PASS |
| demo-reset | PASS |
| browser-demo-reset | PASS |
| demo-recording | PASS |
| invented-sample | PASS |
| chosen-output | PASS |
| docker-rehearsal | PASS — declared local Docker skip; final-SHA CI passed |
| container-cleanup | PASS — declared local Docker skip; final-SHA CI passed |
| rollback-no-go | PASS |
| failed-command-no-go | PASS |
| child-deadlines | PASS |
| interruption-cleanup | PASS |
| threshold-verdict | PASS |
| safe-json | PASS |
| paid-license | PASS |
| installed-cli | PASS |
| free-cli | PASS |

The claim manifest/test one-to-one guard also passed. Landing-page and README
claims are represented in the manifest; no unlisted user-facing claim was
found.

## Clean build, package, and CLI

- `npm ci`: PASS — 20 packages installed; 0 audit vulnerabilities.
- `npm test`: PASS — 8 Rust tests; 26 Node/browser tests, 24 passed and 2
  declared Docker skips.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS — formatting and clippy with warnings denied.
- `npm run build`: PASS — exact production output in `dist/site/`.
- `cargo build --release --locked`: PASS.
- `cargo package --locked`: PASS — 18 files, 76.5 KiB unpacked / 18.3 KiB
  compressed, including package verification.
- Clean consumer install: PASS — `cargo install --path /work/repo --locked`
  produced an `mlr` binary in a temporary Cargo root and it ran from a separate
  directory.

The installed CLI produced valid GO JSON and runbooks for both Postgres and
ClickHouse dry-run demos with network proxies pointed at closed loopback. It
accepted `localhost`, `127.0.0.1`, and `::1`, while rejecting a remote address,
credential and hostname decoys, and `localhost.evil.test`. MySQL was rejected.

Boundary and recovery evidence:

- A statement limit of 184 ms passed; 183 ms wrote NO-GO and exited 1.
- A table-growth limit of 8,192 bytes passed; 8,191 wrote NO-GO and exited 1.
- A lock-wait limit of 0 ms passed for the zero-wait sample.
- Negative limits, blank output, and missing fixture/migration arguments exited
  1 with a specific corrective message and no misleading GO artifact.
- The full deterministic suite passed command-failure, rollback-failure,
  deadline, signal interruption, cleanup, and missing-measurement cases for
  both engines.

Docker is absent locally. GitHub Actions run
[`33253393358`](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33253393358)
is a fresh successful `Docker claims` run at exact SHA `5567fa95…`; its
`real-containers` job ran `@claim:(docker-rehearsal|container-cleanup)` and
completed successfully.

## Live deployment and browser QA

Fresh 1440 × 900 and 390 × 844 Chromium contexts covered `/`, `/?demo=1`,
`/demo`, `/privacy`, `/terms`, and an unknown route. Every real route returned
200; the designed unknown route returned 404. Each page has `lang=en`, one H1,
one main landmark, complete image alt attributes, no horizontal overflow, and
route-specific titles. All crawled internal and external links resolved as
designed; checkout returned 303 to `checkout.dodopayments.com`.

`npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed.
Independent Playwright axe runs found zero serious or critical issues on every
route and viewport. Visible targets were at least 44 × 44 CSS px. Keyboard-only
navigation starts at the skip link, gives links and controls a 4 px blue focus
ring, activates the demo with Enter, moves focus to its H1, and resets it with
Space. Reduced-motion contexts had no running animations. Normal pages emitted
no console or page errors.

The successful mobile Lighthouse run measured:

- Performance 99, accessibility 100, best practices 100, SEO 100.
- FCP 1.6 s, LCP 1.6 s, speed index 1.4 s, total blocking time 0 ms, CLS 0.
- 114 KiB total transfer; no run warnings.

Production assets are 13,507 B JavaScript (5.22 KiB gzip), 6,849 B CSS
(2.26 KiB gzip), and 107,866 B hero WebP. They are below the factory budgets.

## Privacy, headers, caching, and API allowance

Fresh desktop and mobile contexts made only same-origin requests while reading
the landing, demo, privacy, terms, and 404 routes. Local storage, session
storage, and cookies remained empty. There are no third-party fonts, scripts,
analytics, or tracking requests.

A deliberate invalid license return was stripped from the visible URL. The
only external request was a GET to the documented Sociobot verification path
with only the saved `license` query parameter and no body. The invalid token
and cached verdict used only the two documented namespaced keys; **Remove saved
license** deleted both.

The Sociobot verification endpoint enforces an allowance of 30 requests per
client window: requests 1–30 returned 200, and request 31 returned 429 with
`Retry-After: 4` and “Too Many Requests! Wait for 4s.”

Browser-observed response headers include HSTS, `nosniff`,
`strict-origin-when-cross-origin`, and a CSP with `frame-ancestors 'none'` and
only the documented Sociobot connection origin. HTML uses
`public, must-revalidate, max-age=30`; hashed JS, CSS, and hero assets use
`public, max-age=31536000, immutable`.

Local build/live hashes match exactly. Principal live hashes are:

- `main-DXTl9ofI.js`: `5c684c9ef12ec6e1…`
- `main-G7POQdZx.css`: `11d32af1938eb9e0…`
- `lock-stack-DSVDfjcR.webp`: `ca610fb8c0e7433d…`

This is not a PWA, has no product-owned backend, and requires no sign-in, so
service-worker lifecycle, backend persistence/health, and Entra checks do not
apply. The optional factory billing endpoint was covered above.

## Defects by severity

- Critical: none.
- High: none.
- Medium: none.
- Low: none.

Known environment limitation: the local worker has no Docker executable. This
is the claim contract's explicit skip condition, and exact-candidate Docker CI
provides fresh real-container evidence. It is not a product defect.
