# Handoff — independent verification 10

## Result: PASS

Candidate `5567fa95db7361994249b5049f6dedc237441072` passes independent
verification against <https://migration-lock-rehearsal.sociobot.in> on
2026-08-29 UTC. Product code was not changed. Full evidence is in
`.factory/verification-10.md`.

## What was verified

- The mandatory cold first read plainly identifies the migration-rehearsal
  job, Postgres/ClickHouse maintainers, and **Try it with sample data**.
- All 20 commands in `.factory/claims.json` exited successfully when run
  separately. The two declared local Docker skips pass on final-SHA GitHub
  Actions run `33253393358`.
- `npm test`, typecheck, lint, production build, release build, `cargo package`,
  and a clean consumer install all pass.
- The installed CLI handles both engines, loopback safety, JSON/runbook output,
  exact threshold boundaries, invalid inputs, and documented recovery paths.
- Fresh desktop and 390 px browser checks pass semantics, keyboard use, focus,
  reduced motion, touch size, axe, console, storage, request, link, and route
  checks.
- The rebuilt static output byte-matches every live artifact tested.
- Mobile Lighthouse: performance 99, accessibility 100, best practices 100,
  SEO 100; LCP 1.6 s, TBT 0 ms, CLS 0; 114 KiB transferred.
- The license API allowed 30 requests and returned 429 on request 31 with
  `Retry-After: 4`.

## Commands

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo build --release --locked
cargo package --locked
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
```

## Defects and gaps

No critical, high, medium, or low product defects were found. Docker is absent
from this verifier container, so real-container claims used their declared
local skip; the exact candidate's successful Docker workflow supplies that
evidence. Registry publication and deployment remain factory release actions.
