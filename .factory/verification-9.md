# Independent verification 9

## Verdict: PASS

Candidate `cca1079ceef7945b7a2f9d8a3a7ddf6cde3c2542` passes independent release
verification on 2026-08-29 UTC. Product code was not modified. The live URL is
<https://migration-lock-rehearsal.sociobot.in>.

`cca1079` changes only documentation and the preceding handoff; product code
is unchanged from `4ecdf15`. Freshly rebuilt public artifacts byte-match the
live deployment.

## First read and demo

The cold first screen passes: “Rehearse your migration before production”
explains the job; it names Postgres and ClickHouse maintainers; it promises
lock waits, table growth, and rollback results before release. The visible
first action is **Try it with sample data**, with the plain result “Watch the
bundled go/no-go report.” It remains visible at 390 px.

One click opens `?demo=1`, shows “Demo — sample data, nothing is saved,” starts
the bundled-CLI recording, and provides Reset demo and Install the CLI.

## Mandatory claims gate

`.factory/claims.json` exists and contains 19 claims. After `npm ci`, every
exact command listed in it was invoked separately. All passed. The two
Docker-backed commands passed with their declared local skip because this
worker has no `docker` executable; that skip is part of their sandbox contract.

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
| docker-rehearsal | PASS — declared local Docker skip |
| container-cleanup | PASS — declared local Docker skip |
| rollback-no-go | PASS |
| failed-command-no-go | PASS |
| child-deadlines | PASS |
| interruption-cleanup | PASS |
| threshold-verdict | PASS |
| safe-json | PASS |
| paid-license | PASS |
| free-cli | PASS |

The full `npm test` run also passed: 8 Rust unit tests and 24 Node/browser
tests, with 22 passing and the same two declared Docker skips.

## Clean build, package, and CLI evidence

- `npm ci`: PASS — 20 packages, 0 audit vulnerabilities.
- `npm test`: PASS — 22 pass, 2 declared skips, 0 fail.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS (`cargo fmt --check`, clippy warnings denied).
- `npm run build`: PASS — production `dist/site` output.
- `cargo package --allow-dirty --no-verify`: PASS — 18 files, 76.3 KiB
  unpacked / 18.2 KiB compressed.
- A clean `cargo install --path . --root /tmp/...` consumer installation
  produced `mlr` and completed the public CLI flow.

The installed binary's `--help` documents `demo`, `rehearse`, `guard`, JSON,
limits, supported engines, and failure behavior. Postgres and ClickHouse
`demo --dry-run --json` each wrote parseable GO JSON and runbooks with the
default limits of 30,000 ms statement time, 1,000 ms lock wait, and
104,857,600 bytes table growth.

Boundary/recovery checks: exact `localhost`, `127.0.0.1`, and `[::1]` URLs
passed the guard; remote-looking localhost variants were rejected; MySQL and a
blank output path exited 1 with useful messages. Real containers could not be
started in this worker because Docker is absent; the product's deterministic
timeout, interruption, failure, cleanup, and NO-GO claims passed.

## Live QA

Rebuilt `/`, `/demo`, `/privacy`, `/terms`, `/404.html`, the hashed JS/CSS/hero
asset, demo recording, robots file, and sitemap byte-match live. The live
hashes of the principal bundles are:

- `main-DKOVTRc4.js`: `bf36b35261347a680aebefd3da0dc205af9b559dd1666f0e2adbcd7c0c6a54cb`
- `main-CRcil4TQ.css`: `c2d9799280be864dfab8794ced364ff86aa61e7120f079a0684754f9fa38c237`

`npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed.
Fresh Playwright checks at 1440 × 900 and 390 × 844 found one H1, one main,
`lang=en`, no console/page errors, no horizontal overflow, and no missing alt
text on all public pages. Axe found zero serious/critical findings. Keyboard
Tab begins with the skip link and exposes a 4 px blue focus ring. Reduced
motion produced no running animations. All internal links returned 200, the
source link returned 200, and the checkout deliberately returned 303 to Dodo.

Before license use, new desktop and mobile contexts made only same-origin
requests and had no local/session storage or cookies. No third-party fonts,
scripts, analytics, or tracking appeared. The optional checkout returned 303
to `checkout.dodopayments.com`; its hosted page contained `$29.00` and the
one-time product name.

Rate-limit check: a single client made 30 successful license-verification
requests; request 31 returned `429` with `Retry-After: 4`.

Live headers have HSTS, `nosniff`, strict-origin referrer policy, a header CSP
with `frame-ancestors 'none'`, 30-second HTML revalidation, and one-year
immutable caching for hashed assets. Initial output is 13,343 B JS (5.15 KiB
gzip), 6,726 B CSS (2.23 KiB gzip), and a 107,866 B WebP hero—within budget.

## Defects and known gaps

No release-blocking, high, medium, or low defects found. Docker is unavailable
in this verifier container, so the real Postgres/ClickHouse container run was
not repeated locally; this is the explicit expected skip condition for those
two claim tests, not a product failure. This static product has no service
worker, sign-in, or product-owned backend, so PWA, Entra, and backend health /
persistence checks do not apply.
