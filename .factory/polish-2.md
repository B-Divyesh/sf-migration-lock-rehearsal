# Polish 2 — cumulative finding closure

Release candidate `df61c11fedb5abd73fced60521c3798edbc8fe8c` was reviewed through `1d89ff1a8bb708a378127fcbed0c3d01c417eaf9`. Repair code commit: `009754cfc49ccb352f6ca326a2af4faf464f60b9`.

| Finding | Change made | Evidence |
|---|---|---|
| F-1-1 | Added a working GitHub source link, locked `cargo install` command, and complete `mlr rehearse` command. | `@claim:free-cli`; [mobile home](evidence/polish-2/live-mobile-home.png); live `/#install` and GitHub link checked. |
| F-1-2 | Uses the release CLI recording; Reset restores its first line and announces the restart. | `@claim:browser-demo-reset`, `@claim:demo-recording`; [reset state](evidence/polish-2/live-mobile-demo-reset.png); live `/?demo=1`. |
| F-1-3 | Requires real Postgres 16 and ClickHouse 24.8 Docker claims in CI. | `@claim:docker-rehearsal`, `@claim:container-cleanup`; exact repair SHA run [33243574925](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33243574925) passed. |
| F-1-4 | First screen names tested lock waits, table growth, and rollback results. | `@claim:threshold-verdict`, `@claim:rollback-no-go`; [mobile home](evidence/polish-2/live-mobile-home.png); live `/`. |
| F-1-5 | Lists the $29 one-time price in the paid claim and verifies the hosted checkout disclosure. | `@claim:paid-license`; live `/` checkout action checked. |
| F-1-6 | Removed unsupported merchant, refund, and revocation statements. | `@claim:paid-license`; production-copy scan; live `/terms`. |
| F-1-7 | Declares and proves the CLI report and safety-check boundary without a license. | `@claim:free-cli`; live paid section on `/`. |
| F-1-8 | Claims and asserts exact default and overridden limits in help, JSON, and runbook output. | `@claim:threshold-verdict`; live `/?demo=1` report. |
| F-1-9 | Describes the paid result precisely as a browser operator checklist. | `@claim:paid-license`; live paid section on `/`. |
| F-1-10 | Unified 404 canonical/OG metadata and added the Apple touch icon. | Route metadata test; [live 404](evidence/polish-2/live-mobile-404.png); unknown URL returned HTTP 404. |
| F-1-11 | Replaced “Start for real” with “Install the CLI,” linked to `/#install`. | `@claim:browser-demo-reset`; [live demo](evidence/polish-2/live-mobile-demo-reset.png). |
| F-1-12 | Replaced metaphor with `POSTGRES + CLICKHOUSE / v0.1.0`. | Product-copy regression test; [mobile home](evidence/polish-2/live-mobile-home.png). |
| F-1-13 | Replaced the slogan with “Compare measured results with your release limits.” | `@claim:threshold-verdict`; [mobile home](evidence/polish-2/live-mobile-home.png). |
| F-1-14 | Replaced the decorative section label with `HOW IT WORKS`. | Route metadata/section-link test; live `/#how`. |
| F-1-15 | Removed README’s subjective “usable” wording. | Product-copy regression test; `README.md` checked in the clean clone. |
| F-1-16 | Split the five-action README sentence into short statements. | `.factory/copy-audit.md`; clean-clone copy check. |
| F-1-17 | Split the engine behavior sentence and retained one idea per sentence. | `.factory/copy-audit.md`; clean-clone copy check. |
| F-1-18 | Standardized the JSON decision document as “go/no-go report”; reserves “runbook” for Markdown. | Product-copy regression test; live `/`, `/?demo=1`, and README checked. |
| F-2-1 | Replaced “migration report,” “migration card,” “schema change,” and the remaining “measured report.” Added a regression test rejecting every retired name in production copy. | `product copy uses one name for the JSON decision document`; [live demo](evidence/polish-2/live-desktop-demo.png), [live 404](evidence/polish-2/live-mobile-404.png); live `/?demo=1` and unknown URL checked. |

## Final verification

- Every one of the 17 commands in `.factory/claims.json` passed separately in clean clone `/tmp/mlr-polish2-clean.QhT05d`.
- The unfiltered clean-clone suite passed: 8 Rust tests and 22 Node/browser tests. The two Docker-only local tests skipped as designed; the exact repair SHA’s required Docker CI run passed.
- `npm run typecheck`, `npm run lint`, `npm run build`, and `cargo package --allow-dirty` passed in that clone.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed title, language, landmarks, alt text, console, mobile overflow, and axe checks.
- Cold Playwright checks passed at 390 × 844 and 1440 × 900. Main routes returned 200; an unknown route returned 404; demo reset, empty storage, same-origin requests, route focus, history, and mobile overflow all passed.
- Fresh live Lighthouse: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.4 s, CLS 0, total blocking time 70 ms. Raw report: `evidence/polish-2/lighthouse-live.json`.

No finding remains open.
