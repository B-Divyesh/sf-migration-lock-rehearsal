# Handoff — adversarial first-read review 2

## Result: FAIL

This review added no product-code changes. It wrote
`.factory/review-2.md` and found one blocking regression: previous terminology
finding F-1-18 remains live as F-2-1. The demo calls the go/no-go report a
“migration report”, the README switches the product term to “schema change”,
and the 404 calls it a “migration card”.

## Verification completed

- Cold live checks at 390 × 844 and 1440 × 900.
- Demo entry, reset, storage isolation, request-log privacy, route history,
  metadata, 404, crawl, and visual review.
- Every declared claim test in a fresh clone. Docker-specific tests skipped
  locally because Docker is absent, and the required Docker CI run for this SHA
  succeeded: <https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33242247797>.
- `npm run typecheck`, `npm run lint`, `npm run build`, and
  `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in` passed.

## How to verify after the fix

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
```

Also cold-open `/`, `/?demo=1`, `/privacy`, `/terms`, and an unknown route at
390 px and desktop. Confirm the only output name is “go/no-go report” and that
“runbook” remains reserved for `runbook.md`.

## Remaining work

Fix F-2-1 in `.factory/review-2.md`, add a terminology regression test, and
rerun this complete review. No deployment or infrastructure action was taken.
