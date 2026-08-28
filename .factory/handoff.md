# Handoff — independent verification 3

## Release status

**FAIL — DO NOT RELEASE.**

Candidate `5a2ec643d0b042d93401427d580baebf62073466` was independently verified on
2026-08-28 UTC against
https://migration-lock-rehearsal.sociobot.in. The live site byte-matches the
candidate build, so this is not a deployment-only failure.

The complete evidence and reproductions are in
[`verification-3.md`](verification-3.md). The release blockers are:

1. A supplied workload can fail for both Postgres and ClickHouse while the CLI
   exits 0 and writes **GO**.
2. Failed table/lock measurement commands can be silently converted to zero
   while the CLI exits 0 and writes **GO**.
3. A measured 900,000 ms lock wait and 999,999,999,999 table bytes still receive
   **GO** because the verdict considers only rollback success.
4. The brief's one-time purchase flow is absent.

Additional defects: failed migrations leave no report/runbook; the header's
“How it works” link does not scroll to its target; control characters in valid
filenames corrupt JSON output; non-home routes retain home canonical/social
metadata; and axe finds one minor invalid ARIA role combination on `/demo`.

## What passed

- Cold first read and one-click sample demo.
- All ten exact claim commands after `npm ci`.
- `npm test`, `npm run typecheck`, `npm run lint`, `npm run build`.
- `cargo build --release`, `cargo package --allow-dirty`, and clean isolated
  installation/use of the packaged CLI.
- Local and live `verify-url.sh`; zero axe serious/critical findings; desktop,
  390 px, keyboard, focus, reduced motion, touch targets, 404, privacy, request
  log, storage, console, headers, caching, and bundle checks.
- Lighthouse mobile: 96 performance, 100 accessibility, 100 best practices,
  100 SEO; LCP 1.4 s and CLS 0.
- Live HTML, JS, CSS, image, 404, robots, and sitemap hashes match the fresh
  candidate build.

## Verification limitation

The verifier image has no Docker-compatible binary or socket, so a real
Postgres/ClickHouse container could not run. The release CLI was exercised with
a deterministic Docker process double for both engine paths, including the
failure modes that produced the release blockers. This limitation does not
change the FAIL result.

## Re-run

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo build --release
cargo package --allow-dirty
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
```
