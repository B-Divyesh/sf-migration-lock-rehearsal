# Polish 3 — cumulative finding closure

Base review commit: `023ea1837ddd1791399fcc70f5433829adcfb3f5`. Product repair commits: `d44c4b41b8d26121f43c2d541103e2ed8a27e208` and `98f49779c4d755435d7f35202d81cf423e1f3266`.

| Finding | Change made | Evidence |
|---|---|---|
| F-1-1 | Kept the tested GitHub install command and made every post-install example invoke the installed `mlr` binary. | `@claim:installed-cli` installs the package and runs it outside the source tree; [live home](evidence/polish-3/live-mobile-home.png); live `/#install`. |
| F-1-2 | Kept the release-binary terminal recording and made Reset restore its first line without changing real-prefixed browser storage. | `@claim:browser-demo-reset`, `@claim:demo-recording`; [live reset](evidence/polish-3/live-mobile-demo-reset.png); live `/?demo=1`. |
| F-1-3 | Retained required real-container coverage for Postgres 16 and ClickHouse 24.8. | `@claim:docker-rehearsal`, `@claim:container-cleanup`; final SHA Actions run [33252998067](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33252998067) passed. |
| F-1-4 | The first screen names only measured lock waits, table growth, and rollback results. | `@claim:threshold-verdict`, `@claim:rollback-no-go`; [mobile first screen](evidence/polish-3/live-mobile-home.png); live `/`. |
| F-1-5 | The claim contract and test assert the hosted checkout's $29 one-time disclosure. | `@claim:paid-license`; live `/` purchase link and hosted checkout. |
| F-1-6 | Removed the combined Sociobot/Dodo and refund-outcome claims. Terms now reproduce the checkout-supported Dodo Payments wording and link its buyer terms/refund policy. | `@claim:paid-license` asserts the checkout wording and policy page content; [live terms](evidence/polish-3/live-mobile-terms.png); live `/terms`. |
| F-1-7 | The free boundary remains explicit and covers reports, guard checks, and rehearsal validation without a license request. | `@claim:free-cli`; live `/` paid section. |
| F-1-8 | Exact defaults and overrides remain asserted in help, JSON, and runbook output. | `@claim:threshold-verdict`; live `/?demo=1`. |
| F-1-9 | Paid copy promises only the browser operator review checklist, not reuse or persistence. | `@claim:paid-license`; live `/` paid section. |
| F-1-10 | The 404 keeps one canonical/OG identity, Apple touch icon, full Twitter image metadata, and HTTP 404 for unknown paths. | `route metadata, section links, and ARIA remain valid at desktop and mobile widths`; [live 404](evidence/polish-3/live-mobile-404.png); live `/definitely-missing-final-98f4977` returned 404. |
| F-1-11 | Demo exits through the result-naming “Install the CLI” action. | `@claim:browser-demo-reset`; [live demo](evidence/polish-3/live-mobile-demo-reset.png); live `/?demo=1`. |
| F-1-12 | The hero label remains the supported engines plus version. | `product copy uses one name for the JSON decision document`; [live home](evidence/polish-3/live-mobile-home.png); live `/`. |
| F-1-13 | The artwork caption names measured results and release limits. | `product copy uses one name for the JSON decision document`; [desktop home](evidence/polish-3/live-desktop-home.png); live `/`. |
| F-1-14 | The section label remains the informative “HOW IT WORKS.” | `route metadata, section links, and ARIA remain valid at desktop and mobile widths`; live `/#how`. |
| F-1-15 | Subjective “usable sample” wording is absent and guarded against regression. | `product copy uses one name for the JSON decision document`; `.factory/copy-audit.md`. |
| F-1-16 | The five-action README sentence remains split into short statements; all README prose sentences are checked at 22 words or fewer. | `product copy uses one name for the JSON decision document`; `.factory/copy-audit.md`. |
| F-1-17 | The engine behavior remains split into separate concurrency and output sentences. | `product copy uses one name for the JSON decision document`; `.factory/copy-audit.md`. |
| F-1-18 | “Go/no-go report” names the JSON decision document; “runbook” names Markdown. Every retired term is rejected. | `product copy uses one name for the JSON decision document`; live `/`, `/?demo=1`, `/404`; [demo](evidence/polish-3/live-desktop-demo-reset.png). |
| F-2-1 | The terminology regression test now also rejects every earlier metaphor, subjective label, and unsupported legal phrase. | `product copy uses one name for the JSON decision document`; live `/`, `/?demo=1`, and the designed 404. |
| F-3-1 | Terms and README identify Dodo Payments alone. They describe only checkout-observed inquiries/returns behavior and link the actual policy; no refund result is promised. | `@claim:paid-license`; [live terms](evidence/polish-3/live-mobile-terms.png); live `/terms`. |
| F-3-2 | The README's command after `cargo install` is now `mlr rehearse`. A new claim installs the package and runs `mlr` in a separate temporary directory. | `@claim:installed-cli`; live `/#install`; README install-section regression assertion. |
| F-3-3 | The first-screen fact now says “No analytics; license checks contact Sociobot.” Privacy copy states the precise request and storage boundary. | `@claim:site-private` observes same-origin browsing and the token-only Sociobot verification URL; [mobile first screen](evidence/polish-3/live-mobile-home.png); live `/privacy`. |

## Final verification

- Final clean clone `/tmp/mlr-polish3-final.jf3Eji` at `98f49779c4d755435d7f35202d81cf423e1f3266`: all 20 commands in `.factory/claims.json` ran separately and passed. The two Docker-only local invocations skipped as declared; final-SHA real-container run 33252998067 passed both.
- Clean clone `/tmp/mlr-polish3-clean.id9A3O` passed the unfiltered suite: 8 Rust tests and 26 Node/browser tests, with 24 passing and the two declared local Docker skips.
- `npm run typecheck`, `npm run lint`, `npm run build`, and `cargo package --allow-dirty` passed. Build output is `dist/site`; JS is 5.22 kB gzip and CSS is 2.26 kB gzip.
- `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed title, language, landmarks, alt text, console, 390 px overflow, and axe checks after the final deployment.
- Fresh final live contexts passed demo isolation/reset, exact copy, route metadata, history focus, same-origin browsing, 404 status, policy link, security headers, and 390 px layout.
- Final mobile Lighthouse: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.4 s, CLS 0, TBT 20 ms. Raw report: [lighthouse-live.json](evidence/polish-3/lighthouse-live.json).

No finding from reviews 1–3 remains open.
