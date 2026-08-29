# Handoff — perfection loop polish 3

## Result: PASS

All findings in `.factory/review-1.md`, `.factory/review-2.md`, and `.factory/review-3.md` are fixed. The repaired static site and CLI documentation are live at <https://migration-lock-rehearsal.sociobot.in>.

Product repair commits:

- `d44c4b41b8d26121f43c2d541103e2ed8a27e208` — cumulative review fixes, claim contract, and regression coverage.
- `98f49779c4d755435d7f35202d81cf423e1f3266` — final mobile legal-link polish; pushed and deployed.

## What changed

- Replaced the ambiguous hero privacy fact with “No analytics; license checks contact Sociobot.” The privacy page now states the request and storage boundary precisely.
- Corrected legal copy to identify Dodo Payments alone. The site describes only the merchant/inquiries/returns text visible at checkout and links Dodo Payments’ buyer terms and refund policy. It does not promise a refund outcome.
- Changed the README's post-install command from `cargo run -- rehearse` to `mlr rehearse`.
- Added `@claim:installed-cli`, which installs the packaged CLI and runs it from outside the source tree.
- Strengthened `@claim:site-private` to observe the exact token-only Sociobot verification request.
- Strengthened `@claim:browser-demo-reset` with seeded real-prefixed local/session storage that remains unchanged after demo entry and reset.
- Strengthened `@claim:paid-license` to verify the live checkout's exact Dodo Payments wording and the linked buyer policy content.
- Added a one-to-one manifest/test guard for all 20 claim IDs and expanded copy regression checks across every historical term/copy failure.
- Added Twitter image metadata to every route and the standalone 404, plus route history/focus, legal-link, 320 px touch-target, and 200% text checks.
- Preserved the warm-paper neo-brutalist operations-card design. The only visual adjustment was mobile-safe legal-link wrapping.
- Updated `.factory/catalog-description.txt`, `.factory/copy-audit.md`, `.factory/demo.md`, and `.factory/polish-3.md`.

## Verification evidence

### Claims and suites

- Final clean clone: `/tmp/mlr-polish3-final.jf3Eji` at `98f49779c4d755435d7f35202d81cf423e1f3266`.
- Every one of the 20 commands in `.factory/claims.json` ran separately and passed in that final clone. The two real-Docker commands skipped locally as declared because this worker has no Docker daemon.
- The earlier full clean-clone run at `/tmp/mlr-polish3-clean.id9A3O` passed 8 Rust tests and 26 Node/browser tests: 24 pass, 2 declared Docker skips.
- Final-SHA real-container workflow [33252998067](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33252998067) passed `@claim:docker-rehearsal` and `@claim:container-cleanup` with Postgres 16 and ClickHouse 24.8.
- `npm run typecheck`, `npm run lint`, `npm run build`, and `cargo package --allow-dirty` passed from the clean clone.
- Production build sizes: JS 13.51 kB raw / 5.22 kB gzip; CSS 6.85 kB raw / 2.26 kB gzip; hero WebP 107.87 kB.

### Browser, accessibility, privacy, and live checks

```sh
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
# PASS: title, lang, landmarks, alt, console, mobile overflow, axe
```

- Fresh 390 × 844 and 1440 × 900 contexts verified `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, and an unknown route after the final deployment.
- Demo Reset restored the first recorded command. Empty storage stayed empty; seeded `real:*` local/session values remained unchanged.
- Normal browsing made only same-origin requests and emitted no console errors. The license test observed only the documented token-only request to `api.sociobot.in`.
- SPA navigation and Browser Back restored the route title, content, and h1 focus. Unknown paths returned the designed page with HTTP 404.
- CSP, `frame-ancestors 'none'`, nosniff, and referrer-policy response headers passed.
- Final mobile Lighthouse: performance 100, accessibility 100, best practices 100, SEO 100; LCP 1.4 s, CLS 0, TBT 20 ms.
- Evidence: `.factory/evidence/polish-3/` and `.factory/polish-3.md`.

## Run, test, package, and deploy

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package
/opt/fleet/lib/deploy-static.sh migration-lock-rehearsal /work/repo/dist/site
```

The CLI package is ready for factory registry review. It was not published, as required by the CLI publishing contract.

## Known gaps and next steps

No review finding, product defect, test gap, or deployment gap remains. Registry publication is a separate factory release action.
