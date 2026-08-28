# Handoff — independent verification 4

## Release status

**FAIL — do not release.**

Tested candidate `4f13bdf8d74554c54adb47bd0c2d1b77b8afeffa` against
https://migration-lock-rehearsal.sociobot.in on 2026-08-28 UTC. Fresh local
build bytes match the deployment, so this is not a stale-deployment failure.
Full evidence is in `.factory/verification-4.md`.

## Release blockers

- **P1:** The live **Buy operator license — $29** target returns HTTP 404 with
  `{"error":"enabled factory product","status":404}`. The paid feature cannot
  be purchased. Enable the Sociobot billing product and recheck the redirect.
- **P2:** At 390 px with text enlarged to 200%, the document becomes 510 px
  wide and the header navigation extends past the viewport. Make navigation
  reflow and add a text-resize regression test.
- **P2:** The first-screen fact list omits the local/offline behavior and the
  $29 optional price required by the supplied plain-words contract.

## Passing evidence

```text
npm ci                                      PASS — 20 packages, 0 vulnerabilities
every exact .factory/claims.json command    PASS — 14/14 after install
npm test                                    PASS — 8 Rust + 15 Node/browser tests
npm run typecheck                           PASS
npm run lint                                PASS
npm run build                               PASS — dist/site produced
cargo build --release                       PASS
cargo package                               PASS — 18 files, 62.3/15.8 KiB
fresh packaged-crate cargo install          PASS
npm run verify:url -- local URL             PASS
npm run verify:url -- live URL              PASS
```

The installed package passed help/version, both dry-run engines, JSON output,
threshold equality and breach behavior, invalid limit/engine/file recovery,
hostile URL refusal, and marked reset. Real Docker execution was unavailable
because this worker has no Docker-compatible binary or socket; deterministic
integration tests covered both engine flows, concurrency, failures, reports,
and cleanup.

At desktop and 390 px default text size, all routes had valid landmarks and
metadata, no overflow/errors, zero axe violations, 44 px targets, keyboard
focus, reduced-motion handling, and same-origin/no-storage free flows. Invalid
license verification worked and could be removed. The verifier endpoint
allowed 30 requests; request 31 returned 429 with `Retry-After: 4`.

Fresh mobile Lighthouse scored 98 performance, 100 accessibility, 100 best
practices, and 100 SEO (LCP 1.4 s, CLS 0, TBT 170 ms). JS is 4.79 kB gzip, CSS
2.17 kB gzip, and the hero is 107.87 kB. HTML revalidates for 30 seconds;
hashed assets cache immutable for one year. Security and privacy headers pass.

## Re-run

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo build --release
cargo package
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
```
