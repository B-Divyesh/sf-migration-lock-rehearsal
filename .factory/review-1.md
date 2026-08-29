# Adversarial first-read review 1

- Product: Migration Lock Rehearsal
- Live URL: https://migration-lock-rehearsal.sociobot.in
- Repository candidate: `62ca9640c4912fb02a61c41fddd32f6333da74a0`
- Review date: 2026-08-29 UTC
- Viewports: 390 × 844 and 1440 × 900, each in a fresh Chromium context

## Verdict: FAIL

There are 18 findings: 7 blocking, 3 major, and 8 minor. The first screen is clear, the declared claim commands pass, and the static site is accessible. The product still fails because the live site does not provide an executable path to install the CLI, the browser demo reset is a no-op, real Docker behavior remains unverified, and several live, paid, legal, and safety claims are absent from the claims contract.

## Cold first read, before scrolling

### 390 px

- What does it do? It rehearses a database migration before production and reports lock, rewrite, and rollback estimates.
- For whom? Database maintainers preparing a release.
- What should I click first? **Try it with sample data**.

All three answers are available above the fold from: “Rehearse your migration before production,” “For database maintainers who need lock, rewrite, and rollback estimates before a release,” and “Try it with sample data.” The action caption, three facts, and the top of the original lock/database artwork are also visible without scrolling.

### Desktop

The same three answers are clear. The entire hero, action caption, facts, and original artwork are visible before scrolling. This check passes at both sizes.

## Findings

### Blocking

#### F-1-1 — The live site provides no usable way to obtain the CLI

- Exact location: landing section “Install and rehearse”; demo action “Start for real.”
- Exact quote: `cargo run -- rehearse --fixture fixture.sql --migration change.sql --rollback down.sql --workload read.sql`
- Evidence: “Start for real” returns to `/`. The install section has no repository link, clone command, `cargo install` command, binary download, or package name. `cargo run` only works after a visitor somehow obtains this source tree.
- Impact: a first-time visitor can inspect a sample but cannot start the real job from the product site.
- Fix: link the source repository and provide a tested install command, for example `cargo install --git https://github.com/B-Divyesh/sf-migration-lock-rehearsal --locked`. Then show a complete first rehearsal command that works after installation.

#### F-1-2 — “Reset demo” does not reset anything, and the CLI demo is a static snapshot

- Exact location: `/demo`; button “Reset demo”; terminal headed “DISPOSABLE RUN / postgres.”
- Evidence: clicking Reset changes only the button label to “Demo reset” for 1.3 seconds. The terminal, report, DOM, storage, and URL remain identical. The terminal is static HTML, not the required recording of the real CLI.
- Impact: the visitor cannot tell whether the sample was produced by the shipped binary, and the required reset action has no observable result.
- Fix: generate a self-hosted terminal recording from the release binary and let Reset restart that recording and restore the initial report state. Keep the existing banner and show the exact matching `mlr demo` command.

#### F-1-3 — The real Docker rehearsal claim remains untested

- Exact claim: “A Docker rehearsal loads supplied SQL, overlaps the workload with the migration, and records time, lock waits, table bytes, and rollback status.”
- Evidence: `@claim:docker-rehearsal` passed, but `tests/claims.test.mjs` replaces Docker with a shell double. No Docker, Podman, Nerdctl, or Docker socket exists in this worker. The prior handoff explicitly recorded the same real-container gap.
- Impact: command construction is tested, but compatibility with Postgres 16, ClickHouse 24.8, readiness, SQL execution, measurements, and cleanup in real containers is not proven.
- Fix: add a CI claim test that runs the bundled fixtures in real Postgres and ClickHouse containers and asserts the generated JSON/runbook. Keep the command-double test as a fast unit test.

#### F-1-4 — The first screen makes an unlisted rewrite-estimate claim

- Exact quote: “For database maintainers who need lock, rewrite, and rollback estimates before a release.”
- Evidence: `.factory/claims.json` covers lock waits, table bytes/growth, and rollback status, but no output or test defines or measures a “rewrite estimate.”
- Impact: the first screen promises a result the report does not name.
- Fix: use the tested terms: “For Postgres and ClickHouse maintainers who need lock waits, table growth, and rollback results before release.” Alternatively add a defined rewrite metric and a claim test for it.

#### F-1-5 — The $29 checkout price is an unlisted, unverified payment claim

- Exact locations: hero fact, paid section, buy action, README, and `/terms`.
- Exact quotes: “$29 once; checklist optional,” “Buy operator license — $29,” and “The optional operator license costs $29 once.”
- Evidence: `@claim:paid-license` confirms a 303 redirect to Dodo and checks the site’s link label. It does not verify that the hosted checkout charges $29 once.
- Impact: the site asks for payment without testing the amount and billing cadence shown to the buyer.
- Fix: add the exact $29 one-time price to the paid claim and assert the registered checkout product/price through the Sociobot API or hosted checkout response.

#### F-1-6 — Merchant and refund behavior are unlisted legal claims

- Exact locations: landing paid section, README, and `/terms`.
- Exact quotes: “Sociobot and Dodo are the merchant of record,” “refunds are handled there,” and “Refunds are handled there and revoke the license.”
- Evidence: the paid claim verifies only the Dodo checkout redirect and browser license lifecycle. It does not verify merchant-of-record status, refund handling, or license revocation after refund.
- Impact: a buyer could rely on unsupported payment and refund terms.
- Fix: confirm the responsible legal entity in the billing contract, use singular and exact wording, and add an integration test for refunded-license revocation. Remove any statement that cannot be tested.

#### F-1-10 — The prior 404 metadata defect is still live

- Exact location: any unknown URL such as `/definitely-missing-review-1`; `public/404.html`.
- Evidence: the response is a designed 404, but canonical is `/404.html`, Open Graph URL is `/404`, and the apple-touch-icon link is absent. This is the same issue disclosed in the prior handoff, where it had no finding ID.
- Impact: crawlers receive conflicting canonical identities, and the 404 does not use the full site metadata set.
- Fix: use one canonical/OG 404 URL and add `<link rel="apple-touch-icon" href="/apple-touch-icon.png">`.

### Major

#### F-1-7 — The free-use boundary is an unlisted claim

- Exact quote: “CLI reports and safety checks stay free.”
- Location: landing and README.
- Evidence: no claim entry states or tests that every CLI report and guard path works without a license.
- Impact: visitors may choose the tool based on a pricing boundary that is not part of the verified contract.
- Fix: add a claim test that runs `demo`, `rehearse`, and `guard` without a token or network license request, or rewrite narrowly to the behavior tested.

#### F-1-8 — Default limit values and report/runbook inclusion are not fully claimed or tested

- Exact README quotes: “The default release limits are 30,000 ms statement time, 1,000 ms lock wait, and 104,857,600 bytes table growth.” “Every configured limit appears in the JSON report and runbook.”
- Evidence: `threshold-verdict` verifies override values in JSON and only the presence of limit labels in the runbook. Its claim text does not include the three default values or promise that values appear in both files.
- Impact: these are safety-critical defaults and audit outputs.
- Fix: extend the claim text and test to assert all three exact defaults in `--help`, `report.json`, and `runbook.md`, plus exact overridden values in both output files.

#### F-1-9 — “Reusable” overstates the tested paid checklist behavior

- Exact quote: “The license adds a reusable release checklist.”
- Evidence: the paid claim shows the checklist after license verification, but it does not test checklist state, saved answers, export, or reuse across rehearsals.
- Impact: “reusable” suggests persistence or repeated workflow support that the four static list items do not provide.
- Fix: write “The license shows an operator review checklist in this browser,” or implement and test saved checklist runs.

### Minor

#### F-1-11 — “Start for real” is not a result-naming action

- Exact location: `/demo`.
- Exact quote: “Start for real.”
- Impact: the label does not say that it returns to documentation, and it does not start a real rehearsal.
- Fix: after F-1-1 is addressed, label it “Install the CLI” and link directly to the installation section.

#### F-1-12 — The hero eyebrow uses metaphor instead of product information

- Exact quote: “MIGRATION PRE-FLIGHT / 0.1.0.”
- Impact: “pre-flight” repeats brand mood instead of naming supported databases or output.
- Fix: “POSTGRES + CLICKHOUSE / v0.1.0.”

#### F-1-13 — The artwork caption is an untestable slogan

- Exact quote: “Measure the risk before the window opens.”
- Impact: “window opens” is metaphorical and the line does not name what is measured.
- Fix: “Compare measured results with your release limits.”

#### F-1-14 — “THREE MOVES” is a decorative section label

- Exact quote: “THREE MOVES.”
- Impact: it does not identify the section when read out of context.
- Fix: use “HOW IT WORKS” or remove the redundant eyebrow above “Run a migration rehearsal.”

#### F-1-15 — README uses a subjective marketing adjective

- Exact quote: “It gives a usable sample card:”
- Impact: “usable” does not describe a concrete result.
- Fix: “It writes a sample card with measured results and a GO verdict:”

#### F-1-16 — One README sentence exceeds 22 words

- Exact quote, 26 words: “It starts a fresh Docker database, loads the fixture you provide, runs the migration under an optional workload, checks rollback SQL, and writes a measured report.”
- Impact: five actions are packed into one sentence.
- Fix: “It starts a fresh Docker database and loads your fixture. It runs the migration with an optional workload. It checks rollback SQL and writes a measured report.”

#### F-1-17 — A second README sentence exceeds 22 words

- Exact quote, 23 words: “Both engines run the workload while the migration executes and record statement time, observed lock waits, table bytes, table growth, and rollback status.”
- Impact: the concurrency behavior and output list compete in one sentence.
- Fix: “Both engines run the workload while the migration executes. They record statement time, lock waits, table bytes, table growth, and rollback status.”

#### F-1-18 — The same output has inconsistent names

- Exact locations: landing, demo, README, and hidden paid checklist.
- Exact terms: “go/no-go card,” “sample migration card,” “measured report,” “JSON report,” “rehearsal card,” and “JSON card.”
- Impact: a new visitor cannot tell which terms mean `report.json` and which refer to `runbook.md`.
- Fix: use “go/no-go report” for `report.json` and its browser rendering; reserve “runbook” for `runbook.md`. Define both once near the first command.

## Demo and sandbox verification

- One-click entry: pass. “Try it with sample data” opens `/demo` in one click.
- Immediate sample: pass. At 390 px, the banner and realistic Postgres terminal output are visible on the first screen.
- Banner: pass. “Demo — sample data, nothing is saved” remains visible on the demo page.
- Reset: fail; see F-1-2.
- Browser isolation: pass. Fresh mobile and desktop contexts made only same-origin requests and had zero localStorage, sessionStorage, and cookies throughout the demo.
- CLI isolation: pass for the dry run. `@claim:demo-report` ran in a fresh temporary directory with an unusable executable `PATH` and proxies pointed at closed loopback; it wrote only the named output report and runbook.
- Real data: pass for observed paths. The web preview has no writable project data. The CLI demo used bundled invented records and temporary output.

## Claims verification

All commands were run exactly as listed in `.factory/claims.json` after `npm ci` in fresh clone `/tmp/mlr-review1-clean-xiSSPg` at commit `62ca9640c4912fb02a61c41fddd32f6333da74a0`.

| Claim ID | Result | Observable evidence |
|---|---|---|
| `demo-report` | PASS | Offline dry run wrote parseable `report.json` and a GO runbook. |
| `local-only` | PASS | Exact loopback URLs passed; remote-looking decoys failed. |
| `site-private` | PASS | Fresh desktop/mobile site runs used same-origin requests and empty browser storage. |
| `supported-engines` | PASS | Postgres and ClickHouse cards succeeded; MySQL failed before output. |
| `demo-reset` | PASS | Marked demo output was removed; unsafe and unmarked targets survived. |
| `invented-sample` | PASS | Both fixtures used six/example records and contained no connection URL. |
| `chosen-output` | PASS | Both reports stayed below the named directory; blank output failed. |
| `docker-rehearsal` | PASS WITH COVERAGE GAP | The Docker command double showed ordering, overlap, and measured fields; no real container ran (F-1-3). |
| `container-cleanup` | PASS WITH COVERAGE GAP | The command double saw `docker rm -f` after success and migration failure; no real container ran (F-1-3). |
| `rollback-no-go` | PASS | Missing and failed rollback produced non-zero NO-GO for both engines. |
| `failed-command-no-go` | PASS | Workload, measurement, and migration failures produced non-zero NO-GO artifacts. |
| `threshold-verdict` | PASS WITH COVERAGE GAP | Threshold overrides drove NO-GO; exact default/output promises remain uncovered (F-1-8). |
| `safe-json` | PASS | A newline-bearing migration filename remained valid in file/stdout JSON. |
| `paid-license` | PASS WITH COVERAGE GAP | Checkout returned a live 303 to Dodo and browser token lifecycle passed; price and refund claims remain uncovered (F-1-5, F-1-6). |

Unlisted claims are recorded in F-1-4 through F-1-9. No declared command failed.

## History check

No earlier `.factory/review-*.md` or `.factory/polish-*.md` files exist in this worktree. The earlier `.factory/handoff.md` had two known gaps:

1. No real Postgres or ClickHouse container was run. This remains open and is blocking as F-1-3.
2. The standalone 404 had different canonical/Open Graph URLs and no apple-touch icon. Both remain live as F-1-10.

## Structure, routing, privacy, and accessibility

- Main routes `/`, `/demo`, `/privacy`, and `/terms` returned 200; an unknown route returned a designed 404.
- Route titles, descriptions, canonicals, Open Graph metadata, favicons, `lang="en"`, one `<h1>`, one `<main>`, ordered headings, and consistent header/footer pass on main routes.
- Internal links returned 200. The external checkout returned 303 to `checkout.dodopayments.com` in the paid claim test.
- History navigation moved focus to the new `<h1>`; back returned to the prior route; the in-page “How it works” link focused its section.
- Live request logs contained only the document, same-origin JS/CSS, and same-origin hero image before any license action. No console errors occurred on 200 routes.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed at desktop and 390 px, including axe checks. Fresh axe scans reported zero violations. The 390 px layout had no page-level horizontal overflow.
- Response headers include CSP, `frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`.
- The neo-brutalist operations-card treatment, original lock/database art, hard borders, diagnostic blue, and orange warning ink are distinct from a generic SaaS template.
- F-1-10 is the only structure failure found.

## Missed leverage

No AI feature is warranted. Migration safety decisions should remain deterministic and auditable. JSON and Markdown export already exist, and the local CLI does not imply account sync. The obvious missing leverage is distribution/installability, recorded as F-1-1.

## Copy audit

Word counts treat a hyphenated term, URL, path, or command flag as one word. The landing table includes headings, actions, labels, terminal lines, and footer text so non-sentence interface copy is also checked. Raw README code blocks are commands rather than sentences and are verified separately by the claim tests.

### Landing page

| Exact text | Words | Flag |
|---|---:|---|
| MLR/// | 1 | — |
| Demo | 1 | — |
| How it works | 3 | — |
| Privacy | 1 | — |
| MIGRATION PRE-FLIGHT / 0.1.0 | 3 | F-1-12: metaphor/decorative label |
| Rehearse your migration before production | 5 | — |
| For database maintainers who need lock, rewrite, and rollback estimates before a release. | 13 | F-1-4: unlisted rewrite claim |
| Try it with sample data | 5 | — |
| See the bundled go/no-go card. | 5 | — |
| Local dry-run works offline | 4 | — |
| No tracking | 2 | — |
| $29 once; checklist optional | 4 | F-1-5: unverified price |
| Measure the risk before the window opens. | 7 | F-1-13: metaphor/slogan |
| A database cylinder held in an orange padlock with blue diagnostic tape. | 12 | — (image alternative) |
| DISPOSABLE RUN / postgres | 3 | — |
| `$ mlr demo --output ./mlr-demo` | 5 | — |
| starting a fresh Postgres container | 5 | — |
| loading invented fixture: 6 customers | 5 | — |
| running add_customer_flag.sql | 2 | — |
| statement time: 184 ms / limit: 30,000 ms | 8 | — |
| lock wait: 0 ms / limit: 1,000 ms | 8 | — |
| table growth: 8,192 bytes / limit: 104,857,600 bytes | 10 | — |
| rollback: checked | 2 | — |
| VERDICT: GO | 2 | — |
| wrote ./mlr-demo/runbook.md | 2 | — |
| THREE MOVES | 2 | F-1-14: decorative label |
| Run a migration rehearsal | 4 | — |
| Bring a fixture. | 3 | — |
| Use sanitized, production-shaped data. | 4 | — |
| Supply SQL. | 2 | — |
| Add the migration, rollback, and optional workload. | 7 | — |
| Read the card. | 3 | F-1-18: inconsistent output term |
| Compare timings, lock waits, and table growth with clear limits. | 10 | — |
| What this tool does not do | 6 | — |
| The rehearsal has no database URL option. | 7 | — |
| It runs your SQL in the new container it creates. | 10 | — |
| Results are estimates. | 3 | — |
| A failed command or exceeded limit always writes NO-GO. | 9 | — |
| Install and rehearse | 3 | F-1-1: no installation path |
| `cargo run -- rehearse --fixture fixture.sql --migration change.sql --rollback down.sql --workload read.sql` | 11 | F-1-1: assumes source tree |
| Docker must be running. | 4 | — |
| The CLI creates a container and removes it after the run. | 11 | — |
| OPERATOR LICENSE | 2 | — |
| Add the operator review checklist | 5 | — |
| $29 once. | 2 | F-1-5: unverified price |
| The license adds a reusable release checklist. | 7 | F-1-9: unsupported “reusable” |
| CLI reports and safety checks stay free. | 7 | F-1-7: unlisted free-use claim |
| Buy operator license — $29 | 5 | F-1-5: unverified price |
| No license saved. | 3 | — |
| Checking license… | 2 | — (dynamic state) |
| License active. | 2 | — (dynamic state) |
| License no longer active. | 4 | — (dynamic state) |
| Buy a new license. | 4 | — (dynamic state) |
| License saved. | 2 | — (dynamic state) |
| Verification will retry when online. | 5 | — (dynamic state) |
| Have a license? | 3 | — |
| Paste it. | 2 | — |
| Paste a license token to restore it. | 7 | — (dynamic error) |
| The token stays in this browser and goes only to Sociobot for verification. | 13 | — |
| Restore license | 2 | — |
| Remove saved license | 3 | — |
| License removed from this browser. | 5 | — (dynamic state) |
| Operator review checklist | 3 | — |
| Attach the JSON card to the change ticket. | 8 | F-1-18: inconsistent output term |
| Name the owner who can stop the release. | 8 | — |
| Record the tested rollback command. | 5 | — |
| Compare every limit with the approved release budget. | 8 | — |
| Sociobot and Dodo are the merchant of record. | 8 | F-1-6: unlisted legal claim |
| Read privacy and terms. | 4 | — |
| Rehearse database migrations before production. | 5 | — |
| Privacy · Terms · Built by Param Factory · v0.1.0 | 7 | — |

### README

| Exact text | Words | Flag |
|---|---:|---|
| Migration Lock Rehearsal | 3 | — |
| Rehearse a database migration before production. | 6 | — |
| Migration Lock Rehearsal is for Postgres or ClickHouse maintainers who need a concrete go/no-go card before a schema change. | 19 | F-1-18: “schema change” and “card” vary from primary terms |
| It starts a fresh Docker database, loads the fixture you provide, runs the migration under an optional workload, checks rollback SQL, and writes a measured report. | 26 | F-1-16: over 22 words |
| Failed commands, failed rollback, and exceeded limits are always NO-GO. | 10 | — |
| Its URL guard accepts exact loopback hosts only. | 8 | — |
| The static documentation site lives at https://migration-lock-rehearsal.sociobot.in. | 7 | — |
| Quick demo | 2 | — |
| The bundled dry-run demo works locally without Docker or network access. | 11 | — |
| It gives a usable sample card: | 6 | F-1-15: subjective adjective; F-1-18 term |
| For the Docker-backed rehearsal, run: | 5 | — |
| The demo uses invented customer data in `examples/postgres/`. | 8 | — |
| It writes only to the non-blank output folder you name. | 10 | — |
| The Docker-backed command creates a disposable Postgres 16 container and removes it when the run ends. | 16 | — |
| Use your migration | 3 | — |
| Docker must be running. | 4 | — |
| Provide a sanitized fixture, the migration SQL, and optionally its rollback SQL: | 12 | — |
| Read `./rehearsal-card/report.json` in automation and `./rehearsal-card/runbook.md` during the change review. | 10 | F-1-18: “rehearsal-card” adds another name |
| When a workload, measurement, migration, or rollback command fails, the card is NO-GO. | 13 | F-1-18: “card” |
| The CLI writes both files with the failed stage and recovery step, then exits non-zero. | 15 | — |
| Missing measurements are `null`, never zero. | 6 | — |
| Use `--engine clickhouse` with a ClickHouse fixture and migration. | 9 | — |
| Both engines run the workload while the migration executes and record statement time, observed lock waits, table bytes, table growth, and rollback status. | 23 | F-1-17: over 22 words |
| Results are estimates from a new container. | 7 | — |
| Use a production-shaped sanitized fixture before relying on them. | 9 | — |
| The rehearsal command has no database URL option. | 8 | — |
| The default release limits are 30,000 ms statement time, 1,000 ms lock wait, and 104,857,600 bytes table growth. | 18 | F-1-8: unlisted quantitative claim |
| Override them with `--max-statement-ms`, `--max-lock-wait-ms`, and `--max-table-growth-bytes`. | 7 | — |
| Every configured limit appears in the JSON report and runbook. | 10 | F-1-8: incomplete claim coverage |
| An exceeded limit writes NO-GO and exits non-zero. | 8 | — |
| Commands | 1 | — |
| `mlr guard` is a safety check for automation. | 8 | — |
| It parses the URL host, accepts only exact localhost or loopback addresses, and rejects substring decoys. | 16 | — |
| The rehearsal command creates its own Docker container instead of taking a database URL. | 14 | — |
| Demo reset is deliberately narrow. | 5 | — |
| `mlr demo --output ./mlr-demo --reset` removes only a real directory marked by a prior `mlr demo` run. | 17 | — |
| It refuses roots, workspaces, home/current directories, aliases, symlinks, and unmarked folders. | 11 | — |
| Develop and verify | 3 | — |
| Requirements: Rust stable, Node 22+, npm, and Docker for a real rehearsal. | 12 | — |
| The exact static deploy command is `npm run build:site`; it places `index.html` at `dist/site/index.html`. | 14 | — |
| `npm test` runs Rust tests and the claim tests. | 9 | — |
| `cargo package` prepares the CLI package for registry review; do not publish it from this repository. | 16 | — |
| Privacy | 1 | — |
| Without a license action, the site makes only same-origin requests and stores no visitor data. | 15 | — |
| The CLI writes reports to your chosen output folder and runs SQL in its new Docker container. | 17 | — |
| See the site’s `/privacy` and `/terms` pages. | 7 | — |
| Operator license | 2 | — |
| The optional operator license costs $29 once. | 7 | F-1-5: unverified price |
| It adds the browser-based operator review checklist. | 7 | — |
| CLI reports and safety checks stay free. | 7 | F-1-7: unlisted free-use claim |
| Purchase uses Sociobot’s hosted checkout. | 5 | — |
| A returned or pasted token is stored under `sb_license:migration-lock-rehearsal`, sent only to `api.sociobot.in`, and verified at most once daily. | 19 | — |
| Use Remove saved license to delete it. | 7 | — |
| Sociobot and Dodo are the merchant of record, and refunds are handled there. | 13 | F-1-6: unlisted legal claims |
| License | 1 | — |
| MIT. | 1 | — |
| See LICENSE. | 2 | — |

## What would make this perfect

Resolve every finding: provide a tested installation path; replace the static/no-op demo behavior with a recorded, resettable CLI run; execute claim tests against real Postgres and ClickHouse containers; bring every price, legal, free-use, limit, and metric statement into `claims.json`; simplify and standardize the copy; and make the 404 metadata consistent. Re-run all 14 claim commands, the live request/accessibility/crawl checks, and this full first-read checklist from scratch.
