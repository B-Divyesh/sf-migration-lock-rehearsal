# Independent verification 4 — FAIL

**Candidate:** `4f13bdf8d74554c54adb47bd0c2d1b77b8afeffa`  
**Live URL:** https://migration-lock-rehearsal.sociobot.in  
**Verified:** 2026-08-28 UTC  
**Result:** **FAIL — do not release**

The deployed product byte-matches the candidate and the repaired CLI passes its
core safety tests. Release remains blocked because the advertised $29 purchase
action returns HTTP 404. The mobile page also fails the supplied 200% text
resize baseline.

## Cold first-read gate

PASS. A fresh 1440 px browser context showed, within the first viewport:

- What: **“Rehearse your migration before production.”**
- For whom and why: database maintainers who need lock, rewrite, and rollback
  estimates before a release.
- What to click: **Try it with sample data**, beside “See the bundled go/no-go
  card.”

At 390 px the same heading and primary action were visible above the fold. One
click opened `/demo`, already showing the sample terminal and GO card. The page
kept **“Demo — sample data, nothing is saved”**, **Reset demo**, and **Start for
real** visible. Reset worked from the keyboard and changed its label to “Demo
reset.”

## Mandatory claims gate

`.factory/claims.json` exists and contains one tagged test for each of 14
claims. As expected for a bare Node checkout, literal invocations before the
documented install could not import `@axe-core/playwright`. After the locked
clean install (`npm ci`: 20 packages, zero audit findings), every exact command
from the claims file passed from the same checkout:

| Claim | Result |
| --- | --- |
| `demo-report` | PASS |
| `local-only` | PASS |
| `site-private` | PASS |
| `supported-engines` | PASS |
| `demo-reset` | PASS |
| `invented-sample` | PASS |
| `chosen-output` | PASS |
| `docker-rehearsal` | PASS |
| `container-cleanup` | PASS |
| `rollback-no-go` | PASS |
| `failed-command-no-go` | PASS |
| `threshold-verdict` | PASS |
| `safe-json` | PASS |
| `paid-license` | PASS |

Each exact command ran the eight Rust unit tests plus its one matching tagged
test. The checkout outcome itself is not tested: `paid-license` checks the link
target string and mocked verification behavior, while the live target is dead.

## Build, package, and CLI evidence

- `npm test`: PASS — 8 Rust tests and 15 Node/browser tests.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS — rustfmt and clippy with warnings denied.
- `npm run build`: PASS; produced `dist/site/`.
- `cargo build --release` with an isolated target: PASS.
- `cargo package`: PASS — 18 files, 62.3 KiB unpacked / 15.8 KiB compressed;
  Cargo's package verification compiled the crate.
- Installed that packaged crate with `cargo install --path ... --locked` into a
  clean prefix and ran it from an unrelated temporary directory: PASS.

The installed CLI returned helpful help/version output. Postgres and ClickHouse
dry-run demos produced parseable JSON GO cards and both report files. A value
equal to each configured limit remained GO; a statement limit of 183 ms against
the 184 ms sample produced a JSON NO-GO card and exit 1. Negative and fractional
limits, MySQL, missing SQL files, and a hostile `localhost.prod.example.com`
URL all failed with actionable messages. Marked demo reset removed only its
demo folder.

There is no `docker`, `podman`, `nerdctl`, or Docker socket in this verifier
container, so a real Postgres/ClickHouse container run was unavailable. The
deterministic Docker-process tests passed for both engines and cover SQL load,
workload/migration overlap, measurements, rollback, command failures, threshold
breaches, JSON/runbook output, non-zero failure status, and cleanup.

## Live deployment identity

Fresh production-build bytes exactly matched live bytes:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `1b09ffdb55d6856b6a91ff6c048a0ea8572065c16a1eaa1b85cba744df5cbe21` |
| `demo/index.html` | `b4cf746378cf2d85574da3aa49a3067daff4bdc16d17235eaabed4655fe6ef67` |
| `privacy/index.html` | `4c402754b812d9efff69c3bce7940ffba82ac6ca7a30c47094acb785ebe5388b` |
| `terms/index.html` | `dbadcdede2e8e51802685e564f35296915fbe4323e332141fd4c45cc86b2d21b` |
| `assets/main-DX5UjtqP.js` | `b01ff5afaa795b99704210e9ab34acfbcb397ba9fa7c11a6441c470f9500527b` |
| `assets/main-Gaq3n0nI.css` | `c84a478f858967317e893efc1e08f74bcae2936291cde4d0c81cfbffe9edeb43` |
| `assets/lock-stack-DSVDfjcR.webp` | `ca610fb8c0e7433dd49756562982bfcf3ea6c4016477a3d1536fbe3df80dbc40` |
| `404.html` | `137065b5b243d37060a9c1245b73dd89878b8dc0ebfc9c1bb8d87f4b2232c10e` |

An unknown live route returned the matching styled 404 document with HTTP 404.
The failure is not a stale-deployment or candidate mismatch.

## Browser, privacy, accessibility, and performance

- `npm run verify:url -- http://127.0.0.1:4173`: PASS.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in`: PASS.
- Fresh 1440×900 and 390×844 contexts covered `/`, `/demo`, `/privacy`, and
  `/terms`: HTTP 200, route-specific title, `lang=en`, one `main`, one `h1`, no
  missing image alt, no default-width overflow, and no console/page errors.
- Axe reported zero violations of any impact on all four routes at both widths.
- Every measured visible link, button, and input was at least 44 px in both
  dimensions; the smallest was 44.8×44 px.
- Keyboard-only use passed after allowing the animation-frame route update:
  skip link first, 4 px blue focus outline, Enter to `main`, Enter on the sample
  link, focus moved to the demo `h1`, Space reset the demo, and history restored
  the `#how` focus/scroll position.
- Reduced motion changed cursor animation and transitions to `0.01 ms`.
- The complete free route flow requested only
  `migration-lock-rehearsal.sociobot.in`; fresh contexts ended with no local or
  session storage, cookies, service-worker controller, or registration.
- A live invalid returned license was removed from the URL, stored only under
  the documented namespaced keys, sent only to `api.sociobot.in`, kept the paid
  checklist hidden, displayed the inactive notice, and was removable.
- Playwright response headers showed Brotli, 30-second HTML revalidation,
  one-year immutable caching for hashed assets, HSTS, `nosniff`, strict-origin
  referrer policy, and the declared restrictive CSP with `frame-ancestors` in
  the header.
- Initial production assets: JS 12.24 kB raw / 4.79 kB gzip, CSS 6.41 kB raw /
  2.17 kB gzip, hero WebP 107.87 kB. All stated budgets pass.
- Fresh simulated-mobile Lighthouse: performance 98, accessibility 100, best
  practices 100, SEO 100; FCP 0.8 s, LCP 1.4 s, TBT 170 ms, CLS 0.
- The license verification endpoint allowed 30 sequential requests from one
  client. Request 31 returned HTTP 429 with `Retry-After: 4`.

This is a static site plus local CLI. It has no sign-in, product backend, or
service worker and makes no offline/PWA claim, so Entra, backend concurrency,
persistence, and service-worker update tests do not apply.

## Defects by severity

### P1 — the advertised $29 purchase action is dead

Fresh GET and HEAD requests to the exact live button target,
`https://api.sociobot.in/api/v1/products/migration-lock-rehearsal/checkout`,
returned HTTP 404. The GET body was:

```json
{"error":"enabled factory product","status":404}
```

The product therefore advertises a paid checklist that cannot be bought. This
violates the no-dead-links and paid-unlock contracts. It also exposes a gap in
the passing `paid-license` claim test, which verifies only the href and a mocked
verification response. The billing product must be enabled and the live link
must redirect to hosted checkout before release.

### P2 — mobile text at 200% does not reflow

At a 390 px viewport with the root text size doubled, document width grew to
510 px. The header navigation's right edge was 510.47 px and its Privacy link
sat beyond the initial viewport, forcing horizontal page scrolling. This fails
the supplied accessibility baseline that text resize to 200% retain the mobile
layout without loss. Let the header/navigation wrap or collapse at enlarged
text sizes and add an automated resize/reflow check.

### P2 — mandatory first-screen facts omit local use and price

The first-screen facts are “Fresh Docker container,” “Bundled invented sample,”
and “No tracking.” They do not state that the CLI runs locally or that the
optional checklist costs $29 once; the price appears much farther down the
page. This misses the attached plain-words requirement for privacy,
offline/local behavior, and price as three first-screen facts.

## Acceptance conclusion

The candidate is **FAIL**. The prior CLI safety defects are repaired, all
installed claim tests pass, and the live deployment matches the candidate.
The result is still blocked by the freshly reproduced dead checkout. Mobile
200% text reflow and first-screen fact disclosure also need correction.
