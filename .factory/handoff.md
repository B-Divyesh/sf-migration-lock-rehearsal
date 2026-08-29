# Handoff — independent verification 7

## Result: FAIL

Candidate `23971587b6dc981ee4718f2e87014317685754c0` was independently tested on 2026-08-29 against <https://migration-lock-rehearsal.sociobot.in>. The deployed HTML and assets match the candidate byte-for-byte. Product code was not modified.

The release is blocked because a migration or workload that remains blocked has no enforced deadline. `--max-statement-ms` is checked only after completion. A deterministic reproduction stayed active past a 10 ms limit; after an external watchdog stopped it, no `report.json` or `runbook.md` existed and no Docker cleanup command had run. This breaks the core lock-risk rehearsal and recovery job.

The README also calls fixed offline dry-run values “measured results” without a matching claim or measurement provenance. See `.factory/verification-7.md` for the full evidence and accessibility findings.

## Verification summary

- All 17 claim commands ran separately. Fifteen passed locally; the two Docker-only commands skipped locally as designed and passed in exact-candidate GitHub Actions run [33243716627](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33243716627).
- `npm ci`, `npm test`, `npm run typecheck`, `npm run lint`, and `npm run build`: PASS.
- `cargo package --locked --allow-dirty` and a clean consumer `cargo install --path`: PASS.
- Installed Postgres/ClickHouse dry-runs, JSON output, threshold boundaries, local-only guard, unsafe inputs, reset safety, and recovery errors were exercised.
- First-read and one-click sample demo: PASS on desktop and 390 px mobile.
- Live privacy: same-origin only before license action; no visitor storage or cookies. Real invalid-license restore/removal behaved as documented.
- Billing rate limit: requests 1–30 returned 200; request 31 returned 429 with `Retry-After: 3`.
- Live routes, links, console, headers, caching, CSP, axe, keyboard, mobile overflow, and deployment identity passed.
- Lighthouse mobile: 99 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.5 s, TBT 100 ms, CLS 0.

## Run and verify

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo package --locked --allow-dirty
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
```

The browser demo is <https://migration-lock-rehearsal.sociobot.in/?demo=1>. The CLI sample is `mlr demo --dry-run --output ./mlr-demo`.

## Required next steps

1. Enforce deadlines for migration and workload children, write NO-GO artifacts when they expire, and guarantee container cleanup on SIGINT/SIGTERM.
2. Remove or prove the dry-run “measured results” claim and add the corresponding claim test if retained.
3. Disable cursor animation under reduced motion, give the focusable terminal the designed focus ring, enlarge the GitHub touch target, and improve the missing-required-flags error.
4. Re-run all claim commands, real-container CI, package installation, and live deployment verification before release.
