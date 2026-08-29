# Adversarial first-read review 4

- Product: Migration Lock Rehearsal
- Live URL: <https://migration-lock-rehearsal.sociobot.in>
- Candidate: `2a9a3bc9daa5bab65963cf2c2a059c90973ffb48`
- Review date: 2026-08-29 UTC
- Viewports: fresh Chromium contexts at 390 × 844 and 1440 × 900

## Verdict: PASS

There are zero findings. The first screen states the job, audience, and first
action; the one-click demo is realistic and isolated; every claim has passing
local or exact-candidate CI evidence; all earlier findings remain fixed; and
the live routes, copy, links, accessibility, metadata, and visual identity pass.

## Cold first read, before scrolling

### 390 px

- What does it do? It rehearses a Postgres or ClickHouse migration before
  production and reports lock waits, table growth, and rollback results.
- For whom? Postgres and ClickHouse maintainers preparing a release.
- What should I click first? **Try it with sample data**.

All three answers are visible without scrolling. The exact supporting text is
“Rehearse your migration before production,” “For Postgres and ClickHouse
maintainers who need lock waits, table growth, and rollback results before
release,” and “Try it with sample data.” The adjacent sentence, “Watch the
bundled go/no-go report,” names the click result. The three local/privacy/price
facts are also visible within the 844 px first screen.

### 1440 px

The same three answers, action outcome, facts, and original database-lock art
are visible without scrolling. The first-read check passes at both widths.

## Findings

None.

## Demo and sandbox verification

- One-click entry: pass. The primary action opens `/?demo=1`.
- Immediate sample: pass. At 390 px, the first demo screen already shows the
  banner and a recorded Postgres run writing `report.json` and `runbook.md`.
- Banner: pass. “Demo — sample data, nothing is saved” is visible.
- Reset: pass. After the recording advanced, **Reset demo** immediately
  restored `$ mlr demo --dry-run --output ./mlr-demo` and restarted playback.
- Browser isolation: pass. A fresh context made only same-origin requests and
  stored no data. A context seeded with `real:project` and `real:session` kept
  both values unchanged through demo entry and reset. No cookie was created.
- CLI isolation: pass. The release CLI ran in
  `/tmp/mlr-review4-cli.*` and wrote only its marker, `report.json`, and
  `runbook.md` below the selected output directory. Redirected verifier stdout
  was the only file outside that directory.
- Sample result: pass. It reported Postgres, 184 ms statement time, 0 ms lock
  wait, 8,192 bytes growth, rollback checked, and a GO verdict.
- Offline/privacy evidence: `@claim:demo-report` ran with an unusable executable
  path and closed-loopback proxies. The Playwright request log for landing and
  demo contained only the document and same-origin JS, CSS, and art.

## Claims verification

Every command in `.factory/claims.json` was run separately after `npm ci` in
clean clone `/tmp/mlr-review4-clean.6wJgGu`. All commands exited zero. The two
real-container tests made their declared local skip because Docker is absent in
this worker. GitHub Actions run
[33253393358](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33253393358)
passed both against `5567fa95db7361994249b5049f6dedc237441072`; the only changes
from that SHA to this candidate are `.factory/handoff.md` and
`.factory/verification-10.md`. Thus no product or claim code differs from the
real-Docker evidence.

| Claim ID | Result | Observable evidence |
|---|---|---|
| `demo-report` | PASS | Offline dry-run wrote a GO JSON report and runbook. |
| `local-only` | PASS | Exact loopback hosts passed; remote-looking decoys failed. |
| `site-private` | PASS | Fresh routes stayed same-origin and storage-free; the mocked license request contained only the token. |
| `supported-engines` | PASS | Postgres and ClickHouse passed; MySQL was rejected before output. |
| `demo-reset` | PASS | Only a marked demo directory was removed. |
| `browser-demo-reset` | PASS | Query demo restarted while empty and seeded real-prefixed storage remained unchanged. |
| `demo-recording` | PASS | The checked-in recording matched release-CLI commands and output files. |
| `invented-sample` | PASS | Both fixtures contain invented records and no connection URL. |
| `chosen-output` | PASS | Reports stayed below the named non-blank output directory. |
| `docker-rehearsal` | PASS (CI) | Real Postgres 16 and ClickHouse 24.8 run passed in Actions run 33253393358. |
| `container-cleanup` | PASS (CI) | The same real-container run found no disposable container after completion. |
| `rollback-no-go` | PASS | Missing and failed rollback produced non-zero NO-GO for both engines. |
| `failed-command-no-go` | PASS | Every modeled Docker-stage failure produced NO-GO artifacts and cleanup. |
| `child-deadlines` | PASS | Hung migration, workload, and rollback commands were terminated with NO-GO. |
| `interruption-cleanup` | PASS | SIGINT and SIGTERM produced NO-GO and removed the disposable container. |
| `threshold-verdict` | PASS | Exact defaults and overrides appeared in help, JSON, and runbook and drove the verdict. |
| `safe-json` | PASS | A control-character filename remained valid in file and stdout JSON. |
| `paid-license` | PASS | Checkout price, Dodo wording/policy, token lifecycle, daily cache, checklist, and removal passed. |
| `installed-cli` | PASS | A packaged `mlr` installation ran from outside the source tree. |
| `free-cli` | PASS | Demo, guard, and validation ran without a license or network request. |

The live landing page and README contain no claim-like sentence missing from
the claims contract. Requirements and cautions such as “Docker must be
running” and “Use sanitized fixtures” are setup instructions, not product
outcome claims; the workflows that rely on them were nevertheless exercised.

## History check

I read all three earlier reviews, all three polish reports, and the current
handoff. Each earlier finding was rechecked in live rendering and source.

| Earlier finding | Current result |
|---|---|
| F-1-1 install path | Fixed. The live source link returns 200; the locked install command and first `mlr rehearse` command are present and tested. |
| F-1-2 demo/reset | Fixed. The release recording visibly restarts and leaves seeded real storage untouched. |
| F-1-3 real Docker coverage | Fixed. Exact product code passed Postgres and ClickHouse claims in run 33253393358. |
| F-1-4 rewrite estimate | Fixed. The first screen names only tested lock waits, table growth, and rollback results. |
| F-1-5 price | Fixed. The hosted checkout and claim test show a $29 one-time purchase. |
| F-1-6 merchant/refund wording | Fixed. Live terms name Dodo Payments alone and link the observed buyer policy without promising a refund result. |
| F-1-7 free boundary | Fixed. `free-cli` covers reports, guard checks, and validation without a license. |
| F-1-8 default limits | Fixed. Exact defaults and configured values are asserted in help, JSON, and runbook. |
| F-1-9 “reusable” checklist | Fixed. Copy promises only the browser operator review checklist. |
| F-1-10 404 metadata | Fixed. Canonical and OG use `/404`; favicon and Apple touch icon are present. |
| F-1-11 demo action | Fixed. “Install the CLI” names the result and focuses the install section. |
| F-1-12 hero metaphor | Fixed. The label names Postgres, ClickHouse, and version 0.1.0. |
| F-1-13 artwork slogan | Fixed. The caption names measured results and release limits. |
| F-1-14 decorative section label | Fixed. “HOW IT WORKS” names the section. |
| F-1-15 subjective README copy | Fixed. “Usable” is absent. |
| F-1-16 long README sentence | Fixed. The Docker sequence is split into short sentences. |
| F-1-17 second long README sentence | Fixed. Concurrency and recorded results are separate sentences. |
| F-1-18 inconsistent output terms | Fixed. “Go/no-go report” means JSON; “runbook” means Markdown. |
| F-2-1 terminology regression | Fixed. Live/source copy and its regression test reject every retired output name. |
| F-3-1 merchant/refund regression | Fixed. Dodo is identified exactly; the policy link returns 200; no refund outcome is claimed. |
| F-3-2 broken post-install command | Fixed. README uses `mlr rehearse`; `installed-cli` proves it outside the repository. |
| F-3-3 incomplete privacy fact | Fixed. The hero says “No analytics; license checks contact Sociobot,” matching privacy copy and `site-private`. |

## Structure, accessibility, links, and identity

- `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, and `/404` return 200. A
  missing path returns the designed 404 with HTTP 404.
- Every route has a route-specific title, description, canonical, Open Graph
  and Twitter metadata, favicon, Apple touch icon, `lang=en`, one h1, and one
  main landmark. Titles follow the required product/job or route/product form.
- `robots.txt`, `sitemap.xml`, OG art, and static assets return 200. Security
  headers include CSP, `frame-ancestors 'none'`, `nosniff`, and a referrer policy.
- The crawl found no dead link. GitHub and Dodo buyer terms return 200; checkout
  returns the expected 303 to `checkout.dodopayments.com`.
- Client navigation focuses the destination h1. Browser Back restores the URL,
  content, title, and h1 focus. Section deep links resolve and receive focus.
- Fresh contexts show no 390 px horizontal overflow and no console error on
  successful routes. The 404 response is deliberately HTTP 404.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passes
  title, language, landmarks, alt text, console, mobile overflow, and axe.
- `npm test`, typecheck, lint, and production build pass. The build emits
  `dist/site`; application JS is 13.51 kB raw and 5.22 kB gzip.
- Rebuilt `index.html`, JS, and CSS SHA-256 hashes equal the live artifacts.
- The warm-paper operations card, black rules, offset shadows, warning orange,
  diagnostic blue, and original lock/database print form a distinct visual
  identity rather than a generic SaaS template.

## Missed leverage

No missing AI feature is justified. Migration approval needs deterministic,
auditable measurements. The CLI already exports JSON for automation and a
Markdown runbook for review. A local CLI does not imply account sync, and no
decorative AI or provider key is present.

## Copy audit

Word counts treat a hyphenated term, URL, path, or command flag as one word.
Headings, actions, labels, dynamic states, alt text, and terminal lines are
included so all reader-facing landing copy is accounted for. README code
blocks are commands rather than prose sentences and were executed by the claim
tests. No prose sentence exceeds 22 words. No jargon misuse, marketing
adjective, inconsistent term, metaphor heading, empty slogan, or non-result
button was found.

### Landing page

| Exact text | Words | Result |
|---|---:|---|
| MLR/// | 1 | Wordmark |
| Demo | 1 | Navigation |
| How it works | 3 | Navigation |
| Privacy | 1 | Navigation |
| POSTGRES + CLICKHOUSE / v0.1.0 | 3 | Engines and version |
| Rehearse your migration before production | 5 | Job-first h1 |
| For Postgres and ClickHouse maintainers who need lock waits, table growth, and rollback results before release. | 15 | Pass |
| Try it with sample data | 5 | Result-naming action |
| Watch the bundled go/no-go report. | 5 | Pass |
| Local dry-run works offline | 4 | Declared claim |
| No analytics; license checks contact Sociobot | 6 | Declared claim |
| $29 once; browser checklist | 4 | Declared claim |
| A database cylinder held in an orange padlock with blue diagnostic tape. | 12 | Useful alt text |
| Compare measured results with your release limits. | 7 | Informative caption |
| RECORDED DRY RUN / postgres | 4 | Terminal label |
| `$ mlr demo --dry-run --output ./mlr-demo` | 5 | Command |
| `wrote ./mlr-demo/report.json` | 2 | Output |
| `wrote ./mlr-demo/runbook.md` | 2 | Output |
| `$ cat ./mlr-demo/report.json` | 3 | Command |
| `engine: postgres \| statement time: 184 ms \| lock wait: 0 ms` | 10 | Sample result |
| `table growth: 8,192 bytes \| rollback: checked \| verdict: GO` | 8 | Sample result |
| HOW IT WORKS | 3 | Informative section label |
| Run a migration rehearsal | 4 | Informative heading |
| Bring a fixture. | 3 | Pass |
| Use sanitized, production-shaped data. | 4 | Pass |
| Supply SQL. | 2 | Pass |
| Add the migration, rollback, and optional workload. | 7 | Pass |
| Read the report. | 3 | Pass |
| Compare timings, lock waits, and table growth with clear limits. | 10 | Pass |
| What this tool does not do | 6 | Informative heading |
| The rehearsal has no database URL option. | 7 | Declared claim |
| It runs your SQL in the new container it creates. | 10 | Declared claim |
| Results are estimates. | 3 | Useful limitation |
| A failed Docker command or exceeded limit writes NO-GO. | 9 | Declared claim |
| Install and rehearse | 3 | Informative heading |
| Get the source on GitHub (external). | 6 | Result-naming link |
| Docker must be running. | 4 | Requirement |
| The CLI creates a container and removes it after the run. | 11 | Declared claim |
| OPERATOR LICENSE | 2 | Informative section label |
| Add the operator review checklist | 5 | Informative heading |
| $29 once. | 2 | Declared claim |
| A valid license shows the operator review checklist in this browser. | 11 | Declared claim |
| Reports and safety checks do not require a license. | 9 | Declared claim |
| Buy operator license — $29 | 5 | Result-naming action |
| No license saved. | 3 | Empty state |
| Have a license? | 3 | Form label |
| Paste it. | 2 | Form instruction |
| The token stays in this browser and goes only to Sociobot for verification. | 13 | Declared claim |
| Restore license | 2 | Result-naming action |
| Remove saved license | 3 | Result-naming action |
| Paste a license token to restore it. | 7 | Error and next action |
| Checking license… | 2 | Loading state |
| License active. | 2 | Success state |
| License no longer active. | 4 | Invalid state |
| Buy a new license. | 4 | Next action |
| Verification will retry when online. | 5 | Offline state |
| License saved. | 2 | Offline state |
| License removed from this browser. | 5 | Removal result |
| Operator review checklist | 3 | Informative heading |
| Attach the JSON report to the change ticket. | 8 | Useful instruction |
| Name the owner who can stop the release. | 8 | Useful instruction |
| Record the tested rollback command. | 5 | Useful instruction |
| Compare every limit with the approved release budget. | 8 | Useful instruction |
| Read privacy and terms. | 4 | Link instruction |
| Rehearse database migrations before production. | 5 | Footer description |
| Privacy · Terms · Built by Param Factory · v0.1.0 | 7 | Footer links/build |

### README

| Exact text | Words | Result |
|---|---:|---|
| Migration Lock Rehearsal | 3 | Title |
| Rehearse a database migration before production. | 6 | Pass |
| Migration Lock Rehearsal is for Postgres or ClickHouse maintainers who need a go/no-go report before a migration. | 17 | Pass |
| It starts a fresh Docker database and loads your fixture. | 10 | Declared claim |
| It runs the migration with an optional workload. | 8 | Declared claim |
| It checks rollback SQL and writes a go/no-go report. | 9 | Declared claim |
| A failed Docker command, failed rollback, or exceeded limit is always NO-GO. | 12 | Declared claim |
| Its URL guard accepts exact loopback hosts only. | 8 | Declared claim |
| The static documentation site lives at https://migration-lock-rehearsal.sociobot.in. | 7 | Verified location |
| Quick demo | 2 | Informative heading |
| The bundled dry-run demo works locally without Docker or network access. | 11 | Declared claim |
| It writes a sample go/no-go report with fixed sample values: | 10 | Declared claim |
| For the Docker-backed sample rehearsal, run: | 6 | Useful lead-in |
| The demo uses invented customer data in examples/postgres/. | 8 | Declared claim |
| It writes only to the non-blank output folder you name. | 10 | Declared claim |
| The Docker-backed command creates a disposable Postgres 16 container and removes it when the run ends. | 16 | Declared claim |
| Install and use your migration | 5 | Informative heading |
| Install the CLI from this repository: | 6 | Useful instruction |
| Then run your migration: | 4 | Useful instruction |
| Docker must be running. | 4 | Requirement |
| Provide a sanitized fixture, the migration SQL, and optionally its rollback SQL: | 12 | Useful instruction |
| Read ./rehearsal-report/report.json in automation and ./rehearsal-report/runbook.md during the change review. | 10 | Useful instruction |
| When any Docker command in a rehearsal fails, the report is NO-GO. | 12 | Declared claim |
| The CLI writes both files with the failed stage and recovery step, then exits non-zero. | 15 | Declared claim |
| Missing measurements are null, never zero. | 6 | Declared claim |
| Each migration, workload, and rollback command must finish within --max-statement-ms. | 10 | Declared claim |
| On expiry, the CLI terminates the active command, writes NO-GO, and removes the disposable container. | 15 | Declared claim |
| SIGINT and SIGTERM follow the same recovery path. | 8 | Declared claim |
| Use --engine clickhouse with a ClickHouse fixture and migration. | 9 | Useful instruction |
| Both engines run the workload while the migration executes. | 9 | Declared claim |
| They record statement time, lock waits, table bytes, table growth, and rollback status. | 13 | Declared claim |
| Results are estimates from a new container. | 7 | Useful limitation |
| Use a production-shaped sanitized fixture before relying on them. | 9 | Useful instruction |
| The rehearsal command has no database URL option. | 8 | Declared claim |
| The default release limits are 30,000 ms statement time, 1,000 ms lock wait, and 104,857,600 bytes table growth. | 18 | Declared claim |
| Override them with --max-statement-ms, --max-lock-wait-ms, and --max-table-growth-bytes. | 7 | Useful instruction |
| Every configured limit appears in the JSON report and runbook. | 10 | Declared claim |
| An exceeded limit writes NO-GO and exits non-zero. | 8 | Declared claim |
| Commands | 1 | Informative heading |
| mlr guard is a safety check for automation. | 8 | Declared claim |
| It parses the URL host, accepts only exact localhost or loopback addresses, and rejects substring decoys. | 16 | Declared claim |
| The rehearsal command creates its own Docker container instead of taking a database URL. | 14 | Declared claim |
| mlr rehearse requires --fixture and --migration. | 6 | Tested instruction |
| Run mlr rehearse --help to see a complete command. | 9 | Tested instruction |
| Demo reset is deliberately narrow. | 5 | Useful lead-in |
| mlr demo --output ./mlr-demo --reset removes only a real directory marked by a prior mlr demo run. | 17 | Declared claim |
| It refuses roots, workspaces, home/current directories, aliases, symlinks, and unmarked folders. | 11 | Declared claim |
| Develop and verify | 3 | Informative heading |
| Requirements: Rust stable, Node 22+, npm, and Docker for a real rehearsal. | 12 | Useful requirement |
| The exact static deploy command is npm run build:site; it places index.html at dist/site/index.html. | 14 | Verified instruction |
| npm test runs Rust tests and the claim tests. | 9 | Verified instruction |
| cargo package prepares the CLI package for registry review; do not publish it from this repository. | 16 | Useful instruction |
| Privacy | 1 | Informative heading |
| The site has no analytics. | 5 | Declared claim |
| Without a license action, it makes only same-origin requests and stores no visitor data. | 14 | Declared claim |
| A license check sends only the saved token to api.sociobot.in. | 10 | Declared claim |
| The CLI writes reports to your chosen output folder and runs SQL in its new Docker container. | 17 | Declared claim |
| See the site’s /privacy and /terms pages. | 7 | Useful link instruction |
| Operator license | 2 | Informative heading |
| The optional operator license costs $29 once. | 7 | Declared claim |
| It adds the browser-based operator review checklist. | 7 | Declared claim |
| CLI reports and safety checks do not require a license. | 10 | Declared claim |
| Purchase uses Sociobot’s hosted checkout. | 5 | Declared claim |
| Dodo Payments is the merchant of record and handles order-related inquiries and returns. | 13 | Declared claim |
| Read Dodo Payments’ buyer terms and refund policy. | 8 | Useful link instruction |
| A returned or pasted token is stored under sb_license:migration-lock-rehearsal, sent only to api.sociobot.in, and verified at most once daily. | 19 | Declared claim |
| Use Remove saved license to delete it. | 7 | Useful instruction |
| License | 1 | Informative heading |
| MIT. | 1 | License statement |
| See LICENSE. | 2 | Useful link instruction |

## What would make this perfect

Nothing actionable was found. Preserve the exact claim coverage, isolated demo,
plain terminology, and route behavior in future releases; rerun real-container
CI whenever product or claim code changes.
