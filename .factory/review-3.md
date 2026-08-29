# Adversarial first-read review 3

- Product: Migration Lock Rehearsal
- Live URL: <https://migration-lock-rehearsal.sociobot.in>
- Candidate: `3e474b1ca097ba9600772ba36df528e1cae5347e`
- Review date: 2026-08-29 UTC
- Viewports: fresh Chromium contexts at 390 × 844 and 1440 × 900

## Verdict: FAIL

There are three findings: one blocking, one major, and one minor. The cold first
screen, demo, routing, accessibility, and deterministic claim tests pass. The
product still fails because a previously reported payment/legal claim has
regressed, the README's post-install command does not use the installed binary,
and the first-screen privacy fact leaves the behavior after a license action
unclear.

## Cold first read, before scrolling

### 390 px

- What does it do? It rehearses a Postgres or ClickHouse migration before
  production and reports lock waits, table growth, and rollback results.
- For whom? Postgres and ClickHouse maintainers preparing a release.
- What should I click first? **Try it with sample data**.

All three answers are visible without scrolling. The exact supporting text is
“Rehearse your migration before production,” “For Postgres and ClickHouse
maintainers who need lock waits, table growth, and rollback results before
release,” and “Try it with sample data.” The adjacent caption says what the
click does: “Watch the bundled go/no-go report.”

### Desktop

The same answers, action caption, three facts, and original product artwork are
visible without scrolling. This check passes at both widths.

## Findings

### Blocking

#### F-3-1 — F-1-6 regressed: the merchant and refund claims are still not proved

- Exact locations: live `/terms`, `README.md`, `.factory/claims.json`, and
  `tests/claims.test.mjs:473-517`.
- Exact live quotes: “Sociobot/Dodo is the merchant of record.” and “Refunds
  are handled by Sociobot/Dodo through the hosted checkout.”
- Evidence: the live checkout says, “This order process is conducted by our
  online reseller & Merchant of Record, dodopayments.com, who also handles
  order-related inquiries and returns.” It names Dodo Payments alone, not the
  combined entity “Sociobot/Dodo,” and it says returns rather than proving the
  site's refund process. The `paid-license` test verifies the price and then
  only waits for the two disputed terms sentences to render. It does not verify
  a billing contract, refund outcome, or refunded-license behavior.
- Why this fails: a buyer is given an imprecise legal counterparty and an
  unsupported refund statement. This is the substance of F-1-6, which was
  marked fixed and later reintroduced. A claims entry that asserts the sentence
  exists is not a test of the promised behavior.
- Concrete fix: name the checkout-supported party exactly: “Dodo Payments is
  the merchant of record and handles order-related inquiries and returns.” Link
  the applicable refund policy. Remove “refunds are handled” unless an
  integration test completes the documented refund flow and confirms the
  resulting license state.

### Major

#### F-3-2 — The README's first command after installation fails outside the source tree

- Exact location: `README.md`, “Install and use your migration.”
- Exact sequence: “Install the CLI from this repository:” followed by
  `cargo install --git … --locked`, then “Then run your migration:” followed by
  `cargo run -- rehearse …`.
- Evidence: `cargo install --git` installs `mlr` but does not clone or enter the
  repository. Running the documented next command in a fresh directory exits
  101 with `could not find Cargo.toml`.
- Why this fails: a first-time user who follows the installation sequence
  cannot start the real rehearsal. The live landing command correctly uses the
  installed `mlr` binary, but the README contradicts it.
- Concrete fix: change the post-install command to `mlr rehearse …`. Keep
  `cargo run -- …` only in the contributor/development instructions.

### Minor

#### F-3-3 — The first-screen privacy fact is incomplete

- Exact location: landing first-screen facts.
- Exact quote: “No tracking before a license action.”
- Why this fails: “before” leaves a phone visitor to infer that tracking may
  start after checkout or license verification. The privacy page instead says
  there is no analytics and that a license check contacts Sociobot.
- Concrete fix: use “No analytics; license checks contact Sociobot.” Add that
  exact behavior to the existing privacy claim if needed.

## Demo and sandbox verification

- One-click path: pass. **Try it with sample data** opens `/?demo=1`.
- Immediate sample: pass. At 390 px, the first demo screen contains the banner
  and the release-CLI recording writing `report.json` and `runbook.md`.
- Banner: pass. “Demo — sample data, nothing is saved” is present.
- Reset: pass. After the recording advanced, Reset returned it to
  `$ mlr demo --dry-run --output ./mlr-demo`, announced the restart, and then
  advanced again.
- Browser isolation: pass. Fresh phone and desktop contexts made only
  same-origin requests and kept localStorage, sessionStorage, and cookies empty.
  A separate check seeded `real:*` local/session values before demo entry; demo
  and Reset left both values unchanged.
- CLI isolation: pass. The release binary ran in
  `/tmp/mlr-review3-cli.eIrzxb` and wrote only the selected output's marker,
  `report.json`, and `runbook.md`; the redirected stdout was the only verifier
  file outside that output.
- Sample result: pass. The report was Postgres, GO, 184 ms statement time,
  0 ms lock wait, 8,192 bytes growth, and rollback checked.

## Claims verification

Every command in `.factory/claims.json` was invoked separately after `npm ci`
in clean clone `/tmp/mlr-review3-clean.BV21SX`.

| Claim ID | Sandbox result | Evidence |
|---|---|---|
| `demo-report` | PASS | Offline dry-run wrote a GO JSON report and runbook. |
| `local-only` | PASS | Exact loopback hosts passed; remote-looking decoys failed. |
| `site-private` | PASS | Fresh desktop/mobile routes used same-origin requests and empty storage. |
| `supported-engines` | PASS | Postgres and ClickHouse passed; MySQL was rejected. |
| `demo-reset` | PASS | Only a marked demo directory was removed. |
| `browser-demo-reset` | PASS | Query demo started isolated and Reset restarted it. |
| `demo-recording` | PASS | Checked-in recording matched release CLI output names. |
| `invented-sample` | PASS | Both fixtures used invented records and no connection URL. |
| `chosen-output` | PASS | Reports stayed below the non-blank chosen directory. |
| `docker-rehearsal` | LOCAL SKIP; CI PASS | Docker is absent locally. Required real-Docker run 33249962549 passed for product-code commit `4ecdf15`; product and claim code are unchanged at this candidate. |
| `container-cleanup` | LOCAL SKIP; CI PASS | The same real-Docker run covered success/failure cleanup. |
| `rollback-no-go` | PASS | Missing/failed rollback produced non-zero NO-GO for both engines. |
| `failed-command-no-go` | PASS | Every modeled Docker stage failure wrote NO-GO artifacts. |
| `child-deadlines` | PASS | Hung migration, workload, and rollback commands were terminated. |
| `interruption-cleanup` | PASS | SIGINT/SIGTERM wrote NO-GO and invoked cleanup. |
| `threshold-verdict` | PASS | Exact defaults/overrides appeared and determined the verdict. |
| `safe-json` | PASS | A control-character filename remained valid JSON. |
| `paid-license` | COMMAND PASS; COVERAGE FAIL | Price, redirect, cache, checklist, and removal passed; merchant/refund truth is not tested (F-3-1). |
| `free-cli` | PASS | Demo, guard, and argument validation ran without a license. |

No declared command returned failure. F-3-1 is an untested substantive claim,
so the review cannot treat the payment contract as fully verified.

## History check

I read `review-1.md`, `review-2.md`, `polish-1.md`, `polish-2.md`, and the full
current handoff. Each earlier finding was checked in the live site and source.

| Earlier finding | Current result |
|---|---|
| F-1-1 install path | Fixed live: source link, install command, and `mlr rehearse` command work. README has a separate new contradiction (F-3-2). |
| F-1-2 demo/reset | Fixed: release recording visibly restarts. |
| F-1-3 real Docker coverage | Fixed: real Postgres/ClickHouse CI run succeeded and the relevant code is unchanged. |
| F-1-4 rewrite estimate | Fixed: hero names tested lock waits, table growth, and rollback. |
| F-1-5 price | Fixed: live checkout shows $29.00 and one-time purchase text. |
| F-1-6 merchant/refund wording | **Regressed: reopened as F-3-1.** |
| F-1-7 free boundary | Fixed by `free-cli`. |
| F-1-8 default limits | Fixed by exact help/report/runbook assertions. |
| F-1-9 “reusable” checklist | Fixed: current wording names the browser checklist only. |
| F-1-10 404 metadata | Fixed: canonical/OG use `/404`; Apple touch icon exists. |
| F-1-11 demo action | Fixed: “Install the CLI” links to `/#install`. |
| F-1-12 hero metaphor | Fixed: supported databases and version are named. |
| F-1-13 artwork slogan | Fixed: caption names measured results and limits. |
| F-1-14 decorative section label | Fixed: “HOW IT WORKS.” |
| F-1-15 subjective README copy | Fixed. |
| F-1-16 and F-1-17 long sentences | Fixed; no current sentence exceeds 22 words. |
| F-1-18 output terminology | Fixed: `go/no-go report` and `runbook` are distinct. |
| F-2-1 terminology regression | Fixed live and in source; regression test rejects retired terms. |

## Structure, accessibility, links, and identity

- `/`, `/demo`, `/privacy`, `/terms`, and `/404` returned 200. An unknown deep
  link returned the designed 404 with HTTP 404.
- Every route has its route-specific title, description, canonical, Open Graph
  metadata, favicon, Apple touch icon, `lang=en`, one h1, and one main landmark.
  The sitemap, robots file, OG image, and static assets returned 200.
- The site crawl found no dead link. Internal anchors resolve; GitHub returns
  200; the checkout resolves to the hosted Dodo page.
- Navigation moves focus to the new h1. Browser Back restores the prior URL,
  title, content, and h1 focus. There is no 390 px horizontal overflow.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed
  title, language, landmarks, alt text, console, mobile overflow, and axe.
- The warm-paper operations card, hard rules, offset shadows, orange/blue
  warning palette, and original lock/database art are recognizably specific to
  migration rehearsal. This is not a generic SaaS template.

## Missed leverage

No missing AI feature is justified. Migration approval should remain
deterministic and auditable. JSON and Markdown already provide the expected
export path, and a local CLI does not imply account sync.

## Copy audit

Word counts treat hyphenated terms, URLs, paths, and command flags as one word.
Code blocks are commands, not sentences; the broken README command is still
reported as F-3-2. Headings, labels, alt text, and visible terminal lines are
included so every reader-facing copy unit is accounted for. No sentence
exceeds 22 words. No banned marketing adjective, metaphor heading, or
non-result button was found.

### Landing page copy

| Exact text | Words | Result |
|---|---:|---|
| MLR/// | 1 | Wordmark |
| Demo | 1 | Navigation label |
| How it works | 3 | Section navigation |
| Privacy | 1 | Navigation label |
| POSTGRES + CLICKHOUSE / v0.1.0 | 3 | Informative label |
| Rehearse your migration before production | 5 | Job headline |
| For Postgres and ClickHouse maintainers who need lock waits, table growth, and rollback results before release. | 15 | Pass |
| Try it with sample data | 5 | Result-naming action |
| Watch the bundled go/no-go report. | 5 | Pass |
| Local dry-run works offline | 4 | Declared CLI claim |
| No tracking before a license action | 6 | **F-3-3** |
| $29 once; browser checklist | 4 | Declared price/result |
| A database cylinder held in an orange padlock with blue diagnostic tape. | 12 | Useful alt text |
| Compare measured results with your release limits. | 7 | Informative caption |
| RECORDED DRY RUN / postgres | 4 | Terminal label |
| $ mlr demo --dry-run --output ./mlr-demo | 5 | Command |
| wrote ./mlr-demo/report.json | 2 | Terminal output |
| wrote ./mlr-demo/runbook.md | 2 | Terminal output |
| $ cat ./mlr-demo/report.json | 3 | Command |
| engine: postgres \| statement time: 184 ms \| lock wait: 0 ms | 10 | Sample output |
| table growth: 8,192 bytes \| rollback: checked \| verdict: GO | 8 | Sample output |
| HOW IT WORKS | 3 | Informative heading |
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
| Get the source on GitHub. | 5 | Working action |
| Docker must be running. | 4 | Useful requirement |
| The CLI creates a container and removes it after the run. | 11 | Declared claim |
| OPERATOR LICENSE | 2 | Informative label |
| Add the operator review checklist | 5 | Informative heading |
| $29 once. | 2 | Declared claim |
| A valid license shows the operator review checklist in this browser. | 11 | Declared claim |
| Reports and safety checks do not require a license. | 9 | Declared claim |
| Buy operator license — $29 | 5 | Result-naming action |
| No license saved. | 3 | State |
| Have a license? | 3 | Form prompt |
| Paste it. | 2 | Form instruction |
| The token stays in this browser and goes only to Sociobot for verification. | 13 | Declared claim |
| Restore license | 2 | Result-naming action |
| Remove saved license | 3 | Result-naming action |
| Paste a license token to restore it. | 7 | Error and next action |
| Checking license… | 2 | State |
| License active. | 2 | State |
| License no longer active. | 4 | State |
| Buy a new license. | 4 | Next action |
| Verification will retry when online. | 5 | State and next behavior |
| License saved. | 2 | State |
| License removed from this browser. | 5 | State |
| Operator review checklist | 3 | Informative heading |
| Attach the JSON report to the change ticket. | 8 | Useful instruction |
| Name the owner who can stop the release. | 8 | Useful instruction |
| Record the tested rollback command. | 5 | Useful instruction |
| Compare every limit with the approved release budget. | 8 | Useful instruction |
| Read privacy and terms. | 4 | Link instruction |
| Rehearse database migrations before production. | 5 | Footer description |
| Privacy · Terms · Built by Param Factory · v0.1.0 | 7 | Footer links/build |

### README copy

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
| The static documentation site lives at https://migration-lock-rehearsal.sociobot.in. | 7 | Pass |
| Quick demo | 2 | Informative heading |
| The bundled dry-run demo works locally without Docker or network access. | 11 | Declared claim |
| It writes a sample go/no-go report with fixed sample values: | 10 | Declared claim |
| For the Docker-backed sample rehearsal, run: | 6 | Useful lead-in |
| The demo uses invented customer data in examples/postgres/. | 8 | Declared claim |
| It writes only to the non-blank output folder you name. | 10 | Declared claim |
| The Docker-backed command creates a disposable Postgres 16 container and removes it when the run ends. | 16 | Declared claim |
| Install and use your migration | 5 | Informative heading |
| Install the CLI from this repository: | 6 | Useful instruction |
| Then run your migration: | 4 | **F-3-2: following command uses the wrong executable** |
| Docker must be running. | 4 | Useful requirement |
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
| The exact static deploy command is npm run build:site; it places index.html at dist/site/index.html. | 14 | Tested instruction |
| npm test runs Rust tests and the claim tests. | 9 | Tested instruction |
| cargo package prepares the CLI package for registry review; do not publish it from this repository. | 16 | Useful instruction |
| Privacy | 1 | Informative heading |
| Without a license action, the site makes only same-origin requests and stores no visitor data. | 15 | Declared claim |
| The CLI writes reports to your chosen output folder and runs SQL in its new Docker container. | 17 | Declared claim |
| See the site’s /privacy and /terms pages. | 7 | Useful link instruction |
| Operator license | 2 | Informative heading |
| The optional operator license costs $29 once. | 7 | Declared claim |
| It adds the browser-based operator review checklist. | 7 | Declared claim |
| CLI reports and safety checks do not require a license. | 10 | Declared claim |
| Purchase uses Sociobot’s hosted checkout. | 5 | Declared claim |
| Sociobot/Dodo is the merchant of record, and refunds are handled there. | 11 | **F-3-1** |
| A returned or pasted token is stored under sb_license:migration-lock-rehearsal, sent only to api.sociobot.in, and verified at most once daily. | 19 | Declared claim |
| Use Remove saved license to delete it. | 7 | Useful instruction |
| License | 1 | Informative heading |
| MIT. | 1 | License statement |
| See LICENSE. | 2 | Useful link instruction |

## What would make this perfect

Correct the merchant/refund identity and test the behavior rather than the
presence of its sentence. Make the README's post-install example use `mlr`.
Replace the ambiguous first-screen privacy fact with the exact analytics and
license-request behavior. Then rerun all claim commands, the real-Docker CI,
the live mobile/desktop demo, link crawl, and accessibility verifier. Until all
three findings are gone, this is not a zero-finding release.
