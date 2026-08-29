# Copy audit — polish 3

Word counts treat a hyphenated term, URL, path, or command flag as one word. Commands are checked as instructions but are not prose sentences. No reader-facing sentence exceeds 22 words, and no banned marketing term appears.

## Landing page

| Exact text | Words | Result |
|---|---:|---|
| MLR/// | 1 | Wordmark |
| Demo | 1 | Navigation label |
| How it works | 3 | Navigation label |
| Privacy | 1 | Navigation label |
| POSTGRES + CLICKHOUSE / v0.1.0 | 3 | Supported engines and version |
| Rehearse your migration before production | 5 | Job-first h1 |
| For Postgres and ClickHouse maintainers who need lock waits, table growth, and rollback results before release. | 15 | Audience and result |
| Try it with sample data | 5 | Primary action |
| Watch the bundled go/no-go report. | 5 | Action outcome |
| Local dry-run works offline | 4 | `demo-report` claim |
| No analytics; license checks contact Sociobot | 6 | `site-private` claim |
| $29 once; browser checklist | 4 | `paid-license` claim |
| A database cylinder held in an orange padlock with blue diagnostic tape. | 12 | Image alternative |
| Compare measured results with your release limits. | 7 | Artwork caption |
| RECORDED DRY RUN / postgres | 4 | Terminal label |
| `$ mlr demo --dry-run --output ./mlr-demo` | 5 | Recorded command |
| `wrote ./mlr-demo/report.json` | 2 | Recorded output |
| `wrote ./mlr-demo/runbook.md` | 2 | Recorded output |
| `$ cat ./mlr-demo/report.json` | 3 | Recorded command |
| `engine: postgres \| statement time: 184 ms \| lock wait: 0 ms` | 10 | Sample result |
| `table growth: 8,192 bytes \| rollback: checked \| verdict: GO` | 8 | Sample result |
| HOW IT WORKS | 3 | Section label |
| Run a migration rehearsal | 4 | Section heading |
| Bring a fixture. | 3 | Step |
| Use sanitized, production-shaped data. | 4 | Step detail |
| Supply SQL. | 2 | Step |
| Add the migration, rollback, and optional workload. | 7 | Step detail |
| Read the report. | 3 | Step |
| Compare timings, lock waits, and table growth with clear limits. | 10 | Step detail |
| What this tool does not do | 6 | Limits heading |
| The rehearsal has no database URL option. | 7 | `local-only` claim |
| It runs your SQL in the new container it creates. | 10 | Docker scope |
| Results are estimates. | 3 | Limitation |
| A failed Docker command or exceeded limit writes NO-GO. | 9 | Failure behavior |
| Install and rehearse | 3 | Install heading |
| Get the source on GitHub (external). | 6 | Source action |
| Docker must be running. | 4 | Requirement |
| The CLI creates a container and removes it after the run. | 11 | `container-cleanup` claim |
| OPERATOR LICENSE | 2 | Paid section label |
| Add the operator review checklist | 5 | Paid result heading |
| $29 once. | 2 | Price claim |
| A valid license shows the operator review checklist in this browser. | 11 | Paid result claim |
| Reports and safety checks do not require a license. | 9 | `free-cli` claim |
| Buy operator license — $29 | 5 | Purchase action |
| No license saved. | 3 | Empty state |
| Have a license? | 3 | Form label |
| Paste it. | 2 | Form instruction |
| The token stays in this browser and goes only to Sociobot for verification. | 13 | Privacy claim |
| Restore license | 2 | Form action |
| Remove saved license | 3 | Removal action |
| Paste a license token to restore it. | 7 | Form error |
| Checking license… | 2 | Loading state |
| License active. | 2 | Success state |
| License no longer active. | 4 | Invalid state |
| Buy a new license. | 4 | Invalid-state action |
| Verification will retry when online. | 5 | Offline state |
| License saved. | 2 | Offline state |
| License removed from this browser. | 5 | Removal result |
| Operator review checklist | 3 | Paid content heading |
| Attach the JSON report to the change ticket. | 8 | Checklist item |
| Name the owner who can stop the release. | 8 | Checklist item |
| Record the tested rollback command. | 5 | Checklist item |
| Compare every limit with the approved release budget. | 8 | Checklist item |
| Read privacy and terms. | 4 | Legal links |
| Rehearse database migrations before production. | 5 | Footer description |
| Privacy · Terms · Built by Param Factory · v0.1.0 | 7 | Footer links and build |

## Demo and legal routes

| Exact text | Words | Result |
|---|---:|---|
| Read a sample go/no-go report | 6 | Demo h1 |
| This preview uses invented customer records and does not save anything. | 11 | Demo isolation |
| Reset demo | 2 | Reset action |
| Install the CLI | 3 | Real-use action |
| Demo — sample data, nothing is saved | 7 | Persistent demo banner |
| Privacy for a local migration tool | 6 | Privacy h1 |
| The site has no analytics. | 5 | Privacy fact |
| Before a license action, it makes only same-origin requests and stores no visitor data. | 14 | Privacy fact |
| A license check sends only that token to api.sociobot.in. | 10 | License request fact |
| Terms for Migration Lock Rehearsal | 5 | Terms h1 |
| Dodo Payments is the merchant of record and handles order-related inquiries and returns. | 13 | Hosted checkout wording |
| Read Dodo Payments’ buyer terms and refund policy (external). | 9 | Policy link |
| Find the rehearsal page | 4 | 404 h1 |
| That address does not point to a Migration Lock Rehearsal page. | 11 | 404 explanation |
| Return home | 2 | 404 recovery action |

## README check

The README uses the same terms and sentence limits. Its longest prose sentence has 19 words. The post-install example uses `mlr rehearse`; `cargo run` appears only in source-tree demo instructions. The payment section names Dodo Payments exactly and links its buyer terms without promising a refund outcome.

## Terminology

| Concept | One term used |
|---|---|
| Supplied database change | migration |
| Temporary database input | fixture |
| Concurrent database reads | workload |
| JSON decision document | go/no-go report |
| Markdown operator document | runbook |
| Reversal SQL | rollback |
| Decision boundary | limit |
| Paid browser content | operator review checklist |
| Purchase proof | license |
| Checkout counterparty | Dodo Payments |

The regression test `product copy uses one name for the JSON decision document` rejects every retired output name and the unsupported merchant/refund wording.
