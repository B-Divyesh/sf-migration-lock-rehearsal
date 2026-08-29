# Independent verification 6 — PASS

**Candidate:** `df61c11fedb5abd73fced60521c3798edbc8fe8c`  
**Live URL:** https://migration-lock-rehearsal.sociobot.in  
**Verified:** 2026-08-29 UTC  
**Result:** **PASS — release accepted**

The candidate is a documentation-only successor to `ddc1ffe`; its deployable
product files are unchanged. A fresh production build matches the live HTML,
JavaScript, CSS, and hero asset byte-for-byte. No release-blocking defect was
found.

## Required first-read and demo gate

Fresh cold desktop and 390 px browser contexts answered all three required
questions on the first screen in plain words:

- **What:** “Rehearse your migration before production.”
- **For whom:** “Postgres and ClickHouse maintainers” needing lock waits,
  table growth, and rollback results before release.
- **First click:** **Try it with sample data**; adjacent text says it will show
  the bundled go/no-go report.

That one click opens `/?demo=1` (also available as `/demo`) with the invented
sample terminal and report, the persistent “Demo — sample data, nothing is
saved” state, Reset demo, and the clear real-product next step, Install the
CLI. Reset works with the keyboard and restores the first terminal line
`$ mlr demo --dry-run --output ./mlr-demo`. The demo leaves localStorage,
sessionStorage, and cookies empty.

## Mandatory claims gate

`.factory/claims.json` exists and lists 17 claims. After `npm ci` in the clean
checkout (20 packages; zero vulnerabilities), I ran every literal `test`
command separately before broader QA.

| Result | Claims |
| --- | --- |
| PASS | `demo-report`, `local-only`, `site-private`, `supported-engines`, `demo-reset`, `browser-demo-reset`, `demo-recording`, `invented-sample`, `chosen-output`, `rollback-no-go`, `failed-command-no-go`, `threshold-verdict`, `safe-json`, `paid-license`, `free-cli` |
| SKIP, explicitly permitted by the claim sandbox | `docker-rehearsal`, `container-cleanup` — Docker is not installed in this verifier container. The included GitHub Actions Ubuntu workflow makes these fail-required with `MLR_REQUIRE_DOCKER=1`. |

There were no failing claim tests. Homepage and README operational, privacy,
limit, paid-license, and CLI promises map to the claims manifest; no unlisted
reliance claim was found.

## Clean checkout, build, and CLI package

All available local quality gates passed:

```text
npm ci                                      PASS
npm test                                    PASS — 8 Rust tests, 18 Node/browser tests
npm run typecheck                           PASS
npm run lint                                PASS — rustfmt and clippy, warnings denied
npm run build                               PASS — exact production output: dist/site/
cargo build --release                       PASS
cargo package --allow-dirty --no-verify     PASS — 18 files, 63.1 KiB
npm pack --dry-run                          PASS
```

I unpacked the generated `.crate`, installed it into a new Cargo prefix, and
ran its public `mlr` binary from a separate temporary directory. `--help` is
useful; Postgres and ClickHouse `demo --dry-run --json` both wrote parseable
GO `report.json` and `runbook.md`; exact IPv6 loopback passed `guard`; a
remote-looking `localhost.evil.test` URL and `--engine mysql` failed with
actionable non-zero errors. Setting `--max-statement-ms 183` against the
184 ms sample produced exit 1, a JSON `NO-GO`, the stated limit, a reason, and
a matching recovery runbook.

The real Docker-backed rehearsal could not be executed here because `docker`
is absent. This is a verification-environment limitation, not a passing
substitute: the two declared real-container claims were skipped as their own
sandbox contract specifies, and are fail-required in CI.

## Live deployment, privacy, accessibility, and performance

Fresh local build and live artifacts had these matching SHA-256 values:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | identical (no `diff`) |
| `assets/main-CZxjB_2q.js` | `4323b7c3f609a271fe08efd58d34c08479f66444192e24acac170f2566e7ee51` |
| `assets/main-DQj7twNj.css` | `1a32cbf2bd0a8189cee574212995730bcbf4b048797db1c195b2b33d9ccb4ab9` |
| `assets/lock-stack-DSVDfjcR.webp` | `ca610fb8c0e7433dd49756562982bfcf3ea6c4016477a3d1536fbe3df80dbc40` |

- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed:
  title, `lang`, landmarks, alt text, normal-route console, mobile overflow,
  and axe.
- Independent Playwright axe scans found zero serious or critical findings on
  `/`, `/demo`, `/privacy`, `/terms`, and the styled 404. Each has one h1 and
  a main landmark. The 390 px pages have zero horizontal overflow.
- No console or page errors occurred on normal routes. The proper HTTP 404
  naturally logs its failed document request in Chromium, but its designed
  page itself has no script error.
- Keyboard order starts with the skip link. Links/buttons show a 4 px
  `#0057ff` visible keyboard focus ring; controls exercised are at least
  44 px high. Enter activates Reset demo.
- `prefers-reduced-motion: reduce` changes smooth scrolling to `auto` and the
  1 s cursor blink to `0.01 ms`.
- Fresh free flows to all normal routes made only
  `https://migration-lock-rehearsal.sociobot.in` requests, stored no browser
  data, and loaded no third-party font or script. No sign-in, PWA, AI feature,
  or product backend applies.
- HTML responses have HSTS, `nosniff`, strict-origin referrer policy, and a
  response-header CSP including `frame-ancestors 'none'`. HTML uses
  `max-age=30`; hashed assets use `max-age=31536000, immutable`.
- First-load payloads: JavaScript 13.22 kB raw / 5.10 kB gzip; CSS 6.51 kB
  raw / 2.19 kB gzip; hero WebP 107.87 kB. These are within the stated static
  budgets.

## Billing endpoint allowance

The optional Sociobot license verification endpoint is the only server-side
endpoint the product invokes. Fresh invalid-token requests returned 200 for
requests 1–30. Request 31 returned **HTTP 429** with **`Retry-After: 3`**.
The observed allowance is therefore 30 sequential requests per client window.
The free site does not call it; it is contacted only after a license action.

## Defects by severity

- **P0/P1/P2/P3:** none.
- **Validation limitation:** Docker is unavailable in this container, so the
  real Postgres 16 and ClickHouse 24.8 container runs were not independently
  observed. Their explicit claim tests skipped rather than failed and are
  fail-required on the repository's Ubuntu Docker CI workflow.

## Acceptance conclusion

**PASS.** The CLI fulfills the researched job: it creates a local, disposable
rehearsal report for Postgres or ClickHouse, fails closed for missing rollback,
failed commands, and exceeded limits, and supplies a one-click non-persistent
browser sample without tracking. Fresh byte-level evidence disproves a
deployment-only mismatch for this candidate.
