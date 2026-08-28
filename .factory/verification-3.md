# Independent verification 3 — FAIL

**Candidate:** `5a2ec643d0b042d93401427d580baebf62073466`  
**Live URL:** https://migration-lock-rehearsal.sociobot.in  
**Verified:** 2026-08-28 UTC  
**Result:** **FAIL — do not release**

The live deployment matches the candidate and the documented happy paths work,
but the CLI can issue a successful **GO** result when the supplied workload or
measurement commands fail. It also issues **GO** for extreme measured lock and
table values. Those results are unsafe for the product's core migration-risk
decision.

## Cold first-read gate

PASS. The first screen answers all three required questions in plain words:

- What: “Rehearse your migration before production.”
- For whom/outcome: database maintainers who need lock, rewrite, and rollback
  estimates before a release.
- First click: **Try it with sample data**, beside “See the bundled go/no-go
  card.”

The one click opens `/demo`, which already shows the sample terminal and
go/no-go card. It keeps the banner “Demo — sample data, nothing is saved,” plus
**Reset demo** and **Start for real**. Evidence:
`evidence/verification-3/live-cold-desktop.png`,
`evidence/verification-3/live-mobile.png`, and
`evidence/verification-3/live-demo-desktop.png`.

## Mandatory claims gate

`.factory/claims.json` exists. After the required `npm ci`, every exact listed
command passed and ran one matching tagged integration test (plus the eight
Rust unit tests invoked by the repository test wrapper):

| Claim | Exact command result |
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

A literal invocation before dependency installation could not import
`@axe-core/playwright`; `npm ci` installed the lockfile's 20 packages with zero
audit findings, after which all ten exact commands passed. The claims suite
does not cover the failing workload and measurement paths below.

## Build, package, and CLI evidence

- `npm test`: PASS — 8 Rust tests and 10 Node/browser integration tests.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS — rustfmt and clippy with warnings denied.
- `npm run build`: PASS; it produced `dist/site/`.
- `cargo build --release`: PASS.
- `cargo package --allow-dirty`: PASS; 18 files, 44.4 KiB unpacked and
  12.3 KiB compressed; Cargo's package verification compiled it.
- Fresh `cargo install` from `target/package/migration-lock-rehearsal-0.1.0`
  into an isolated Cargo root: PASS.
- The installed CLI was run from an unrelated temporary working directory.
  Help/version, Postgres and ClickHouse dry-run JSON cards, loopback acceptance,
  hostile-host refusal, unsupported-engine refusal, blank-output refusal,
  missing-input recovery, and marked demo reset behaved as documented.
- There is no `docker`, `podman`, `nerdctl`, or Docker socket in this verifier
  container. A real database-container run was therefore unavailable.
  Deterministic Docker-process integration exercised both engine paths,
  process overlap, cleanup, rollback, failed workload, failed measurement,
  failed migration, and high-risk values. The executable double used for the
  failure reproductions is `evidence/verification-3/docker`.

## Live browser, privacy, accessibility, and performance

- `npm run verify:url -- http://127.0.0.1:4173`: PASS.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in`: PASS.
- At desktop and 390 px, `/`, `/demo`, `/privacy`, and `/terms` each returned
  200 with one `main`, one `h1`, `lang=en`, route title, image alt text, and no
  horizontal overflow, console errors, or page errors.
- Axe found zero serious/critical findings on all four routes. It found one
  minor `aria-allowed-role` issue on `/demo` (see P3 below).
- All measured links/buttons were at least 44 by 44 CSS px. Keyboard tests
  reached a visible skip link first; its 4 px blue focus outline was present,
  Enter focused `main`, route changes focused the new `h1`, and Enter operated
  demo reset. Reduced motion changes the cursor animation to `0.01 ms`.
- Every crawled link returned 200; an unknown route returned the styled 404
  with HTTP 404.
- The complete route flow made same-origin requests only. Fresh desktop and
  mobile contexts ended with zero localStorage/sessionStorage entries, no
  cookies, and no service-worker controller. No analytics, external scripts,
  or external fonts loaded.
- Playwright response headers showed Brotli on HTML/JS/CSS, one-year immutable
  caching on hashed assets, 30-second revalidation on HTML, HSTS,
  `X-Content-Type-Options: nosniff`, strict-origin referrer policy, and a CSP
  limited to self/data images with `frame-ancestors 'none'`.
- Production sizes: JS 7.06 kB raw / 3.02 kB gzip; CSS 5.46 kB raw / 1.96 kB
  gzip; hero WebP 107.87 kB. These pass the static budgets.
- Fresh Lighthouse mobile: performance 96, accessibility 100, best practices
  100, SEO 100; LCP 1.4 s, CLS 0, TBT 210 ms. Raw report:
  `evidence/verification-3/lighthouse-live.json`.
- This is a static documentation site plus local CLI. It exposes no product
  server endpoint, uses no sign-in, and makes no PWA/offline claim; API 429,
  Entra authority, persistence/concurrency, and service-worker tests are not
  applicable.

## Deployment identity

Fresh-build and live bytes match:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `b0d62060bafc651c6036bfdc1191efae734d17a80c7b1eb6ecffbf3f9c13a255` |
| `assets/index-Cv8VkA3S.js` | `12e768b07602f031108564b8f6c65874a88abfb6cc5fe8a4ba41e59f6252aad9` |
| `assets/index-B4BEPYFK.css` | `98f92352310865214ba5fe58d2015b788a9e6583074e047b70d26c39039286a5` |
| `assets/lock-stack-DSVDfjcR.webp` | `ca610fb8c0e7433dd49756562982bfcf3ea6c4016477a3d1536fbe3df80dbc40` |
| `404.html` | `58c8716bea14b8189dc7b27df667d318dd09002d03bf9378eaaa9ae1315dbcce` |

## Defects by severity

### P0 — failed workloads still receive GO

For each engine, the Docker test double returned exit 19 from the supplied
`/work/workload.sql` command while fixture, migration, measurement, and rollback
commands succeeded. Both release-CLI runs exited 0 and wrote
`"verdict": "GO"`. Their notes say lock waits were sampled “while the supplied
workload runs,” although it had failed. The CLI kills/waits for the workload
child but never inspects its exit status. This can approve an unrehearsed
migration.

Observed:

```text
postgres exit=0 verdict=GO workload_invocations=2
clickhouse exit=0 verdict=GO workload_invocations=2
```

### P0 — failed measurements are silently reported as zero and GO

When table-size subprocesses failed with exit 23, both engines substituted
zero before/after bytes and still exited 0 with GO. Postgres uses the same
silent-zero fallback for lock-wait measurement. A measurement failure is
indistinguishable from a measured zero in the report.

```text
postgres exit=0 {"before":0,"after":0,"lock":0,"verdict":"GO"}
clickhouse exit=0 {"before":0,"after":0,"lock":0,"verdict":"GO"}
```

Measurement commands must return errors, or the card must be explicit NO-GO /
incomplete. They must never fabricate zero.

### P0 — the verdict ignores measured lock and rewrite risk

With successful fixture/migration/rollback commands and measured lock wait of
900,000 ms plus table bytes of 999,999,999,999, both engines exited 0 and wrote
GO. `rehearsal_report` derives the verdict only from `rollback: bool`; statement
time, lock wait, and size delta never affect it. The headline product is a
go/no-go risk card, so an unconditional GO after rollback is misleading.
Define documented/configurable risk thresholds or avoid an affirmative GO when
the tool has not evaluated those measurements.

```text
postgres exit=0 {"bytes":999999999999,"lock":900000,"verdict":"GO"}
clickhouse exit=0 {"bytes":999999999999,"lock":900000,"verdict":"GO"}
```

### P1 — the one-time purchase contract is absent

The researched brief specifies one-time monetization and the supplied product
contract defines Sociobot checkout, license restore/verify, exact price, and
legal copy. The live candidate has no paid tier, price, checkout, license
handling, or restore path. `src/paid.css` is unused. This appears to remove the
previous broken checkout rather than fulfill the accepted monetization scope.

### P1 — a failed migration produces no decision artifact

When the migration subprocess failed, both engines exited 1 and cleaned up,
but neither wrote `report.json` nor `runbook.md`. A failed rehearsal is a clear
NO-GO outcome and should leave the operator-facing card promised by the brief,
including the failure stage and recovery action.

### P2 — “How it works” navigation does not reach its target

At both desktop and mobile, selecting the header link changed the URL to
`/#how`, but the SPA intercepted it, rebuilt the page, focused the hero `h1`,
and left `scrollY=0`; `#how` remained about 1,527 px below the mobile viewport.
Hash-only in-page links should retain native scrolling/focus rather than enter
the route-change handler.

### P2 — valid filenames can corrupt the public JSON output

`escape()` handles only backslash and quote. A valid Unix migration filename
containing a newline made the CLI exit 0 while both `report.json` and `--json`
stdout failed `JSON.parse`; the raw newline appeared inside the JSON string.
Use a real JSON serializer or escape every JSON control character.

### P2 — non-home route metadata points to the home page

The SPA updates `document.title`, but `/demo`, `/privacy`, and `/terms` all keep
the home canonical URL and home Open Graph metadata from `index.html`. Direct
HTTP responses therefore describe/canonicalize every route as `/`.

### P3 — demo banner uses an invalid ARIA role combination

Axe reports minor `aria-allowed-role` on `/demo`: `role="status"` is applied to
an `aside`. Use an allowed live-region element/role combination.

## Acceptance conclusion

The candidate is **FAIL**. Deployment-only concerns are not the cause: the live
site exactly matches the candidate and its static quality is strong. Release is
blocked by reproducible false-GO behavior in the CLI's core safety decision,
plus the missing purchase scope and other defects above.
