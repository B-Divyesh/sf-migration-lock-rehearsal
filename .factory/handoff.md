# Handoff — Migration Lock Rehearsal

## Release status — **FAIL**

Independent verification of candidate
`f86ac9ff0cad67b08b61a3b98e59f8e9eb4d9352` at
https://migration-lock-rehearsal.sociobot.in on 2026-08-28 UTC **failed**.
The deployed JavaScript SHA-256 exactly matches the production build of this
commit, so this is not a deployment-only mismatch.

Read the complete evidence and reproducible commands in
`.factory/verification-2.md`.

## What passed

- `npm ci`, all seven commands from `.factory/claims.json`, `npm test`,
  `npm run typecheck`, `npm run lint`, `npm run build`, `cargo build --release`,
  and `cargo package --allow-dirty` passed.
- A fresh `cargo install --path . --root <temp>` consumer exercised help,
  version, Postgres/ClickHouse dry-run demos, JSON report output, and normal
  invalid-input failures.
- Live desktop and 390px checks passed for the supported pages: one h1/main,
  no overflow, no serious/critical axe findings, keyboard demo/reset and
  visible focus, reduced motion, same-origin-only requests, security headers,
  styled HTTP 404, immutable hashed asset caching, and no application console
  errors.
- The cold page explains the product/audience/first action plainly and has a
  one-click sample-data demo.

## Release blockers

1. **P0:** `mlr demo --output <any existing directory> --reset` can recursively
   remove an arbitrary path. It rejects only a few literal spellings and the
   current directory, not canonical broad paths or other directories.
2. **P1:** `mlr guard` accepts remote-looking URLs containing strings such as
   `localhost`, `.test`, or `disposable`; this falsifies the local-only claim.
3. **P1:** `mlr demo --dry-run --output ''` writes reports into the current
   directory rather than rejecting a blank path.
4. **P1:** ClickHouse runs the workload before its migration and reports lock
   wait as a constant zero, so it does not deliver the required concurrent
   workload/lock-risk rehearsal for that advertised engine.
5. **P1:** Claims coverage is incomplete for several displayed operational and
   privacy assertions; the local-only claim’s current test does not prove it.

## Required next steps

Fix every blocker, add adversarial regression coverage, then run a real Docker
Postgres and ClickHouse rehearsal (Docker was unavailable to this verifier),
rerun all claims from a clean install, and request a new independent QA pass.

## Run locally

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo build --release
cargo package --allow-dirty
cargo run -- demo --dry-run --output ./mlr-demo
```
