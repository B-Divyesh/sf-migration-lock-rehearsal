# Independent verification 5 — PASS

**Candidate:** `501229c57c286192d870877186ac6825b52fd7d4`

**Implementation under test:** `4a49a78` (the candidate adds only repair evidence and the previous handoff)

**Live URL:** https://migration-lock-rehearsal.sociobot.in

**Verified:** 2026-08-28 UTC

**Result:** **PASS — release accepted**

Fresh evidence confirms the repaired candidate works as the brief's local,
Docker-first migration rehearsal CLI. The live documentation and demo match the
candidate build byte for byte. There are no release-blocking defects.

## Cold first-read gate

PASS at 1440×900 and 390×844 in fresh browser contexts.

- What: **“Rehearse your migration before production.”**
- For whom and why: database maintainers who need lock, rewrite, and rollback
  estimates before a release.
- First action: **Try it with sample data**, beside “See the bundled go/no-go
  card.”
- One click opened `/demo`, already showing the sample terminal, measurements,
  rollback result, and GO card.
- Demo mode displayed **“Demo — sample data, nothing is saved”**, **Reset demo**,
  and **Start for real**. Reset worked with Enter and Space.

The three first-screen facts are “Local dry-run works offline,” “No tracking,”
and “$29 once; checklist optional.” The action was above the fold at both tested
widths.

## Mandatory claims gate

`.factory/claims.json` exists and contains 14 claims. Literal test invocations
against the untouched checkout first reported the expected missing-package
error because dependencies had not yet been installed. `npm ci` then installed
the locked 20-package tree with zero audit findings. Every exact claim command
subsequently passed from that clean candidate. This installed result is the
release result:

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

Each command selected exactly one matching Node test and also ran the eight
Rust unit tests. Landing-page and README promises map to these claims; no
unlisted claim-like promise was found.

## Clean build and package evidence

```text
npm ci                    PASS — 20 packages; 0 vulnerabilities
npm test                  PASS — 8 Rust tests; 16 Node/browser tests
npm run typecheck         PASS
npm run lint              PASS — rustfmt + clippy with warnings denied
npm run build             PASS — exact production output in dist/site
cargo build --release     PASS — isolated target directory
cargo package             PASS — 18 files; 62.5 KiB / 16.0 KiB compressed
clean cargo install       PASS — packaged crate installed into a fresh prefix
```

The production site payload is 12,254 bytes JavaScript (4.80 kB gzip), 6,511
bytes CSS (2.19 kB gzip), and a 107,866-byte hero WebP. The 1200×630 social
image and 180×180 touch icon have the documented dimensions.

## Installed CLI exercise

The packaged crate was unpacked and installed into a clean Cargo prefix. Its
`mlr` binary was run from an unrelated temporary directory rather than the
repository.

- `--help` and `--version` are useful and return zero.
- Postgres and ClickHouse `demo --dry-run --json` each write parseable
  `report.json` and `runbook.md` GO cards.
- Exact limits of 184 ms statement time, 0 ms lock wait, and 8,192 bytes growth
  remain GO. A 183 ms statement limit writes NO-GO and exits 1.
- Negative, fractional, and overflowing limits fail with actionable messages.
- MySQL and unknown options fail before creating an output directory.
- Exact `localhost`, IPv4 loopback, and IPv6 loopback URLs pass the guard.
  Hostname, percent-encoding, query, and whitespace decoys fail.
- A marked demo resets successfully. An unmarked directory survives.
- Without Docker, a real rehearsal exits 1 with “Docker is required” and points
  to the dry-run demo.

No Docker, Podman, Nerdctl, or Docker socket is available in this verifier
container, so an independent real-container run was not possible. The passing
deterministic process integration covers both engines, SQL load, workload and
migration overlap, measurements, command failure, threshold failure, rollback,
JSON/runbook output, exit status, and container cleanup.

## Browser, accessibility, privacy, and routing

- `npm run verify:url` passed against both the built preview and the live URL.
- Local and live `/`, `/demo`, `/privacy`, and `/terms` passed at 1440×900 and
  390×844: HTTP 200, route-specific metadata, `lang=en`, one h1/main, ordered
  headings, correct image alt, no overflow, and no console or page errors.
- Axe found zero violations on all 16 local/live route-width combinations. The
  same was true at 390 px with 200% root text.
- At 390 px and 200% text, the document stayed 390 px wide and the navigation's
  right edge was 366 px. The prior reflow failure is fixed.
- Every measured visible control was at least 44 px in both dimensions.
- Keyboard checks passed: the skip link is first and appears at 8 px with a 4 px
  blue focus ring; Enter focuses `main`; route changes focus the h1; hash links
  focus their section; browser Back restores the route; demo reset works with
  Enter and Space.
- Reduced motion makes the cursor animation and button transitions 0.01 ms.
- Fresh free flows requested only the site origin and left no local/session
  storage, cookies, service-worker controller, or registrations.
- An invalid live license was stored only under the documented namespaced keys,
  sent only to `api.sociobot.in`, announced as inactive, kept the checklist
  hidden, and was removable without residue.
- Every internal link returned 200. The buy action returned HTTP 303 to an HTTPS
  `checkout.dodopayments.com/session/cks_…` URL. An unknown route returned the
  styled candidate 404 with status 404 and a way home.

The product has no sign-in, backend, or service worker and makes no web-offline
claim. Entra, backend concurrency/persistence, and PWA update tests do not
apply. AI would not improve the deterministic safety decision, so no missed AI
leverage was found.

## Live deployment identity

Fresh production output and live response bodies matched byte for byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `faf391960106e2b9aaad210b4d7b207668e0046601dfc6deff51077a3b7ef8dc` |
| `demo/index.html` | `ad82cca85db4ac1cd26bc866e6560e984d2f27e683c801bca728a96514d5df6e` |
| `privacy/index.html` | `d7bbb0a3349164b746940573e08aeb308a625fba95d6f80c389c2565f12741e2` |
| `terms/index.html` | `08b9e8911ea5b41788ba269433b3e5c25b2cc1e8eb44e565401a9abc17861785` |
| `assets/main-DAthCEdy.js` | `ab834c1a44154848dde2a67dedd2f48094302d34a88cab80f4bbc54cd5f2eb61` |
| `assets/main-DQj7twNj.css` | `1a32cbf2bd0a8189cee574212995730bcbf4b048797db1c195b2b33d9ccb4ab9` |
| `assets/lock-stack-DSVDfjcR.webp` | `ca610fb8c0e7433dd49756562982bfcf3ea6c4016477a3d1536fbe3df80dbc40` |
| `404.html` | `137065b5b243d37060a9c1245b73dd89878b8dc0ebfc9c1bb8d87f4b2232c10e` |

Live HTML uses Brotli and `Cache-Control: public, must-revalidate, max-age=30`.
Hashed assets use `public, max-age=31536000, immutable`. Responses include
HSTS, `nosniff`, strict-origin referrer policy, and the declared CSP with
`frame-ancestors 'none'` in the response header.

## Performance and billing policy

Fresh throttled mobile Lighthouse:

| Performance | Accessibility | Best practices | SEO | LCP | TBT | CLS | Transfer |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 100 | 100 | 100 | 100 | 1.48 s | 12 ms | 0 | 116,555 B |

The live Sociobot verification endpoint allowed 30 sequential requests from one
client. Request 31 returned HTTP 429 with `Retry-After: 2`.

## Defects by severity

- **P0/P1/P2:** none.
- **P3:** the standalone 404 document omits the apple-touch-icon link, and its
  canonical `/404.html` differs from its Open Graph `/404` URL. It remains a
  correctly styled, accessible HTTP 404 with working navigation. This is
  metadata polish and does not block the CLI release.

## Acceptance conclusion

**PASS.** The prior checkout, mobile reflow, and first-screen disclosure
blockers are repaired. The installed claims, core CLI behavior, package,
candidate/live identity, privacy, accessibility, performance, and billing rate
limit all pass from fresh evidence.
