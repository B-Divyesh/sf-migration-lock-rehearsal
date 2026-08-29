# Handoff — independent verification 6

## Result: PASS

Candidate `df61c11fedb5abd73fced60521c3798edbc8fe8c` is accepted for
`https://migration-lock-rehearsal.sociobot.in`. Its product files are unchanged
from the deployed repair and a fresh build matched the live HTML, JS, CSS, and
hero asset byte-for-byte.

## What was verified

- Every test listed in `.factory/claims.json` ran from this clean checkout:
  15 passed; the two Docker-only claims skipped exactly as their sandbox
  permits because Docker is not installed. No claim failed.
- `npm test`, typecheck, lint, exact production build, Rust release build,
  Cargo package, and package dry-run passed.
- A newly unpacked/installed `mlr` package produced Postgres and ClickHouse
  dry-run reports, correctly rejected remote-looking URLs and unsupported
  engines, and wrote a NO-GO report/runbook with exit 1 for an exceeded limit.
- The cold live page plainly says what it does, who it is for, and offers
  **Try it with sample data**. Demo reset is keyboard-operable and stores no
  browser data.
- Live route, privacy, request-log, headers, desktop/390px, keyboard,
  reduced-motion, cache, budget, and axe checks passed. The optional license
  endpoint allowed 30 sequential calls and returned 429 with `Retry-After: 3`
  on request 31.

See `.factory/verification-6.md` for exact commands and evidence.

## How to run and verify

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo build --release
cargo package
```

Run the offline shipped sample with:

```sh
cargo run -- demo --dry-run --output ./mlr-demo
```

Use `mlr rehearse` with a sanitized fixture and Docker for a real disposable
Postgres or ClickHouse rehearsal. Do not publish from this repository.

## Known limitation

This verifier container has no Docker binary, so it could not independently
execute the real Postgres/ClickHouse container claims. They are explicitly
fail-required in `.github/workflows/docker-claims.yml` on an Ubuntu Docker
runner; the local skip is recorded in the verification report.
