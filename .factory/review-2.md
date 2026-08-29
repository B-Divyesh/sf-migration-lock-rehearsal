# Adversarial first-read review 2

- Product: Migration Lock Rehearsal
- Live URL: <https://migration-lock-rehearsal.sociobot.in>
- Candidate: `a7848388ad4b93ddbbb7d151fe117df26971bc64`
- Review date: 2026-08-29 UTC
- Viewports: fresh Chromium contexts at 390 × 844 and 1440 × 900

## Verdict: FAIL

One blocking finding remains. It is a regression of prior finding F-1-18, which
the repository says was fixed. All declared claim tests passed or were executed
successfully in the required Docker CI run, but the output terminology is still
inconsistent on live pages and in the README.

## Cold first read, before scrolling

### 390 px

- **What it does:** rehearses a database migration before production and gives a go/no-go result.
- **For whom:** Postgres and ClickHouse maintainers preparing a release.
- **What to click first:** **Try it with sample data**.

The first phone screen contains the exact text: “Rehearse your migration before
production”; “For Postgres and ClickHouse maintainers who need lock waits,
table growth, and rollback results before release.”; and “Try it with sample
data.” The action caption tells the visitor what follows: “Watch the bundled
go/no-go report.” This passes the first-screen clarity check.

### Desktop

The same headline, audience sentence, primary action, action result, and three
facts are visible without scrolling. This also passes.

## Findings

### Blocking

#### F-2-1 — Reopened F-1-18: the same output still has several names

- **Locations and exact quotes:**
  - Live `/demo` h1: “Read a sample migration report”.
  - Live `/404` text: “That address does not point to a migration card.”
  - `README.md`, opening description: “...need a go/no-go report before a schema change.”
- **Why this fails:** F-1-18 required one name for the JSON decision document:
  “go/no-go report”, while reserving “runbook” for the Markdown document. The
  current demo calls that same visible JSON result a “migration report”; the
  404 calls it a “migration card”; and the README swaps the product’s central
  job term, “migration”, for “schema change”. A first-time visitor cannot tell
  whether these are different artifacts or different jobs.
- **Code confirmation:** `src/main.ts` renders both “Read a sample migration
  report” and the nearby “Go/no-go report”; `public/404.html` renders
  “migration card”; `README.md` contains “schema change”. This is not only a
  historical document issue: all three strings are live.
- **Concrete fix:** use this exact vocabulary everywhere: “Read a sample
  go/no-go report”; “That address does not point to a Migration Lock Rehearsal
  page.”; and “...need a go/no-go report before a migration.” Then update the
  terminology table and add a text regression test that rejects `migration
  report`, `migration card`, and `schema change` when they name these concepts.

## Demo and sandbox verification

- One-click path: pass. **Try it with sample data** opens `/?demo=1` in one click.
- Immediate realistic sample: pass. The first demo screen shows an invented
  Postgres 16 report with statement time, lock wait, table growth, rollback,
  and a GO verdict.
- Isolation banner: pass. “Demo — sample data, nothing is saved” persists on
  the demo route.
- Reset: pass. The recorded terminal progresses from its first command; Reset
  returns it to `$ mlr demo --dry-run --output ./mlr-demo` and announces
  “Sample recording restarted.”
- Browser storage and privacy: pass. Fresh desktop and phone contexts made only
  same-origin requests (`/`, the self-hosted JS/CSS, and hero asset), with empty
  localStorage, sessionStorage, and cookies. There were no console errors.
- CLI sandbox: pass. The declared dry-run claim creates only the named report
  and runbook in a fresh temporary output directory with Docker and network
  proxies unavailable.
- Real data separation: pass for the shipped demo. The browser demo has no
  writable data store; the CLI fixture claim confirms invented records and no
  connection URL.

## Claims verification

Every command listed in `.factory/claims.json` was invoked in clean clone
`/tmp/mlr-review2-clean-kxDDle` at the candidate commit. Docker is absent in
this verifier, so its two integration tests correctly report `SKIP`; their
required GitHub Actions run for this exact SHA completed successfully:
<https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33242247797>.
Thus no declared claim is left without an executed test result.

| Claim ID | Result | Evidence checked |
|---|---|---|
| `demo-report` | PASS | Offline dry run wrote a GO JSON report and runbook. |
| `local-only` | PASS | Exact loopback passed; remote-looking hosts failed. |
| `site-private` | PASS | Same-origin requests and empty browser storage. |
| `supported-engines` | PASS | Postgres and ClickHouse accepted; MySQL rejected. |
| `demo-reset` | PASS | Only a marked demo directory could be removed. |
| `browser-demo-reset` | PASS | `?demo=1` starts isolated recording; Reset restarts it. |
| `demo-recording` | PASS | Recording command and named outputs match release dry run. |
| `invented-sample` | PASS | Shipped fixtures are invented and have no connection URL. |
| `chosen-output` | PASS | Reports stay below the named non-blank output directory. |
| `docker-rehearsal` | PASS in required CI | Local test skipped only because Docker is unavailable. |
| `container-cleanup` | PASS in required CI | Local test skipped only because Docker is unavailable. |
| `rollback-no-go` | PASS | Both engines emit non-zero NO-GO on missing/failed rollback. |
| `failed-command-no-go` | PASS | Failed workload, measurement, and migration emit artifacts and NO-GO. |
| `threshold-verdict` | PASS | Exact defaults and overrides appear and determine verdict. |
| `safe-json` | PASS | Control-character filename remains parseable JSON. |
| `paid-license` | PASS | Checkout/price and local license lifecycle assertions pass. |
| `free-cli` | PASS | Demo, guard, and validation run without license activity. |

The live landing, `/demo`, `/privacy`, `/terms`, and README claim-like copy was
cross-checked against this contract. No additional unlisted behavior claim was
found. F-2-1 is terminology, not a new unlisted behavioral claim.

## History check

I read `review-1.md`, `polish-1.md`, and `handoff.md` in full, then checked
their findings against live pages and code.

| Earlier finding | Result in this round |
|---|---|
| F-1-1 install path | Fixed: live source link, `cargo install` command, and rehearsal command work. |
| F-1-2 demo/reset | Fixed: self-hosted release recording visibly resets. |
| F-1-3 real Docker coverage | Fixed: real-container tests are required by CI; exact candidate CI run is successful. |
| F-1-4 rewrite estimate | Fixed: first-screen wording names lock waits, table growth, rollback. |
| F-1-5 price | Fixed: `paid-license` asserts the $29 one-time checkout disclosure. |
| F-1-6 merchant/refund wording | Fixed: unsupported wording is absent. |
| F-1-7 free boundary | Fixed: `free-cli` claim covers it. |
| F-1-8 default limits | Fixed: exact values and outputs are asserted. |
| F-1-9 reusable checklist | Fixed: copy now describes the browser checklist precisely. |
| F-1-10 404 metadata | Fixed: canonical/OG agree on `/404`; apple icon exists. |
| F-1-11 result-naming demo action | Fixed: “Install the CLI” names the result. |
| F-1-12 through F-1-17 | Fixed: first-screen, headings, and long/subjective README copy are corrected. |
| F-1-18 output terminology | **Regressed / not fully fixed: reopened as F-2-1.** |

## Structure, routing, privacy, accessibility, and visual identity

- `/`, `/demo`, `/privacy`, `/terms`, and `/404` return 200. An unknown deep
  link returns designed 404 content with HTTP 404. The GitHub source link
  returns 200; the hosted checkout is covered by its claim test.
- Route titles, descriptions, canonicals, Open Graph values, favicon, 180 px
  apple icon, `lang="en"`, one h1, and a main landmark pass. The OG image is
  original and is 1200 × 630.
- A privacy navigation click focuses the new h1. Browser Back restores the home
  h1 and title. No 390 px horizontal overflow or console errors occurred.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed,
  including axe checks. The response CSP, `frame-ancestors`, referrer policy,
  and nosniff header are present as response headers.
- The warm-paper, hard-rule, offset-shadow operations card and original
  padlocked database art match the design thesis and are distinct from a generic
  SaaS template.

## Missed leverage

No missing AI feature was found. Migration approval needs deterministic,
auditable measured output; an AI step would add risk without advancing the
brief. JSON and Markdown output already provide the implied export path, and
the local CLI does not imply account sync.

## Copy audit

Word counts treat a hyphenated term, URL, path, or command flag as one word.
Command blocks are commands rather than sentences; visible terminal lines are
included because visitors read them. No sentence exceeds 22 words and no banned
marketing adjective appears. Buttons are result-naming verbs except the
factual restore/reset actions, which name their outcomes.

### Landing page text

| Exact text | Words | Result |
|---|---:|---|
| MLR/// | 1 | Wordmark |
| Demo | 1 | Navigation label |
| How it works | 3 | Section navigation |
| Privacy | 1 | Navigation label |
| POSTGRES + CLICKHOUSE / v0.1.0 | 3 | Product information |
| Rehearse your migration before production | 5 | Plain job headline |
| For Postgres and ClickHouse maintainers who need lock waits, table growth, and rollback results before release. | 15 | Pass |
| Try it with sample data | 5 | Result-naming primary action |
| Watch the bundled go/no-go report. | 5 | Pass |
| Local dry-run works offline | 4 | Declared claim |
| No tracking before a license action | 6 | Declared claim |
| $29 once; browser checklist | 4 | Declared claim |
| Compare measured results with your release limits. | 7 | Pass |
| A database cylinder held in an orange padlock with blue diagnostic tape. | 12 | Useful image alternative |
| RECORDED DRY RUN / postgres | 3 | Terminal label |
| $ mlr demo --dry-run --output ./mlr-demo | 5 | Command |
| wrote ./mlr-demo/report.json | 2 | Terminal output |
| wrote ./mlr-demo/runbook.md | 2 | Terminal output |
| $ cat ./mlr-demo/report.json | 3 | Command |
| engine: postgres \| statement time: 184 ms \| lock wait: 0 ms | 10 | Declared sample output |
| table growth: 8,192 bytes \| rollback: checked \| verdict: GO | 8 | Declared sample output |
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
| A failed command or exceeded limit writes NO-GO. | 8 | Declared claim |
| Install and rehearse | 3 | Informative heading |
| Get the source on GitHub. | 5 | Working external link |
| Docker must be running. | 4 | Useful requirement |
| The CLI creates a container and removes it after the run. | 11 | Declared claim |
| OPERATOR LICENSE | 2 | Informative section label |
| Add the operator review checklist | 5 | Informative heading |
| $29 once. | 2 | Declared claim |
| A valid license shows the operator review checklist in this browser. | 11 | Declared claim |
| Reports and safety checks do not require a license. | 9 | Declared claim |
| Buy operator license — $29 | 5 | Result-naming action |
| No license saved. | 3 | Current state |
| Have a license? Paste it. | 5 | Form label |
| The token stays in this browser and goes only to Sociobot for verification. | 13 | Declared claim |
| Restore license | 2 | Result-naming action |
| Remove saved license | 3 | Result-naming action |
| Checking license… | 2 | Current state |
| License active. | 2 | Current state |
| License no longer active. Buy a new license. | 8 | Current state and next step |
| License saved. Verification will retry when online. | 7 | Current state and next step |
| License removed from this browser. | 5 | Current state |
| Operator review checklist | 3 | Informative heading |
| Attach the JSON report to the change ticket. | 8 | Useful checklist item |
| Name the owner who can stop the release. | 8 | Useful checklist item |
| Record the tested rollback command. | 5 | Useful checklist item |
| Compare every limit with the approved release budget. | 8 | Useful checklist item |
| Read privacy and terms. | 4 | Link label |
| Rehearse database migrations before production. | 5 | Footer one-liner |
| Privacy · Terms · Built by Param Factory · v0.1.0 | 7 | Footer links/build |

### README text

| Exact text | Words | Result |
|---|---:|---|
| Migration Lock Rehearsal | 3 | Title |
| Rehearse a database migration before production. | 6 | Pass |
| Migration Lock Rehearsal is for Postgres or ClickHouse maintainers who need a go/no-go report before a schema change. | 18 | **F-2-1** terminology |
| It starts a fresh Docker database and loads your fixture. | 10 | Declared claim |
| It runs the migration with an optional workload. | 8 | Declared claim |
| It checks rollback SQL and writes a measured report. | 9 | Declared claim |
| Failed commands, failed rollback, and exceeded limits are always NO-GO. | 10 | Declared claim |
| Its URL guard accepts exact loopback hosts only. | 8 | Declared claim |
| The static documentation site lives at https://migration-lock-rehearsal.sociobot.in. | 7 | Pass |
| Quick demo | 2 | Informative heading |
| The bundled dry-run demo works locally without Docker or network access. | 11 | Declared claim |
| It writes a sample go/no-go report with measured results: | 9 | Declared claim |
| For the Docker-backed sample rehearsal, run: | 5 | Informative lead-in |
| The demo uses invented customer data in examples/postgres/. | 8 | Declared claim |
| It writes only to the non-blank output folder you name. | 10 | Declared claim |
| The Docker-backed command creates a disposable Postgres 16 container and removes it when the run ends. | 16 | Declared claim |
| Install and use your migration | 5 | Informative heading |
| Install the CLI from this repository: | 6 | Useful instruction |
| Then run your migration: | 4 | Useful instruction |
| Docker must be running. | 4 | Useful requirement |
| Provide a sanitized fixture, the migration SQL, and optionally its rollback SQL: | 12 | Useful instruction |
| Read ./rehearsal-report/report.json in automation and ./rehearsal-report/runbook.md during the change review. | 10 | Useful instruction |
| When a workload, measurement, migration, or rollback command fails, the report is NO-GO. | 13 | Declared claim |
| The CLI writes both files with the failed stage and recovery step, then exits non-zero. | 15 | Declared claim |
| Missing measurements are null, never zero. | 6 | Declared claim |
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
| Demo reset is deliberately narrow. | 5 | Useful lead-in |
| mlr demo --output ./mlr-demo --reset removes only a real directory marked by a prior mlr demo run. | 17 | Declared claim |
| It refuses roots, workspaces, home/current directories, aliases, symlinks, and unmarked folders. | 11 | Declared claim |
| Develop and verify | 3 | Informative heading |
| Requirements: Rust stable, Node 22+, npm, and Docker for a real rehearsal. | 12 | Useful requirement |
| The exact static deploy command is npm run build:site; it places index.html at dist/site/index.html. | 14 | Useful instruction |
| npm test runs Rust tests and the claim tests. | 9 | Useful instruction |
| cargo package prepares the CLI package for registry review; do not publish it from this repository. | 16 | Useful instruction |
| Privacy | 1 | Informative heading |
| Without a license action, the site makes only same-origin requests and stores no visitor data. | 15 | Declared claim |
| The CLI writes reports to your chosen output folder and runs SQL in its new Docker container. | 17 | Declared claim |
| See the site’s /privacy and /terms pages. | 7 | Useful link instruction |
| Operator license | 2 | Informative heading |
| The optional operator license costs $29 once. | 7 | Declared claim |
| It adds the browser-based operator review checklist. | 7 | Declared claim |
| CLI reports and safety checks do not require a license. | 9 | Declared claim |
| Purchase uses Sociobot’s hosted checkout. | 5 | Declared claim |
| A returned or pasted token is stored under sb_license:migration-lock-rehearsal, sent only to api.sociobot.in, and verified at most once daily. | 19 | Declared claim |
| Use Remove saved license to delete it. | 7 | Useful instruction |
| License | 1 | Informative heading |
| MIT. | 1 | License statement |
| See LICENSE. | 2 | Useful link instruction |

## What would make this perfect

Make the three F-2-1 rewrites, add the terminology regression test, rebuild,
and rerun the clean-clone claims, live 390 px/desktop checks, link crawl, and
accessibility scan. With that one historical regression actually removed,
there is no other finding from this review.
