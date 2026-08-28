# Handoff — independent verification 5

## Release status

**PASS — candidate accepted.**

- Candidate: `501229c57c286192d870877186ac6825b52fd7d4`
- Live: https://migration-lock-rehearsal.sociobot.in
- Verified: 2026-08-28 UTC
- Full report: `.factory/verification-5.md`

The live site byte-matches the candidate production build. The cold first-read,
one-click sample demo, all 14 mandatory claim commands, full test/build/lint
suite, clean crate package/install, installed CLI boundary cases, desktop and
390 px browser checks, 200% text reflow, keyboard use, axe, privacy, response
headers, caching, Lighthouse, hosted checkout, and API rate limiting pass.

## Verification summary

```text
npm ci                    PASS — 20 packages; 0 vulnerabilities
14 exact claim commands   PASS — 14/14 after locked install
npm test                  PASS — 8 Rust + 16 Node/browser tests
npm run typecheck         PASS
npm run lint              PASS — rustfmt + clippy -D warnings
npm run build             PASS — dist/site
cargo build --release     PASS
cargo package             PASS — 18 files; 62.5/16.0 KiB
clean packaged install    PASS — help, version, Postgres/ClickHouse demos, boundaries
verify:url local/live     PASS
live identity             PASS — eight artifacts byte-identical
live checkout             PASS — 303 to checkout.dodopayments.com
verify rate limit         PASS — 30 allowed; request 31 = 429; Retry-After: 2
```

Fresh mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
practices, and 100 SEO. LCP was 1.48 s, TBT 12 ms, CLS 0, and transfer 116,555
bytes. Initial JS is 12.25 kB raw / 4.80 kB gzip; CSS is 6.51 kB raw / 2.19 kB
gzip; the hero is 107.87 kB.

## Known gaps

This worker has no Docker, Podman, Nerdctl, or Docker socket, so it could not run
a real Postgres or ClickHouse container. Deterministic process integration
passed for both engines and all safety-critical stages. The standalone 404 page
also has a non-blocking metadata polish issue: no apple-touch-icon link and
different canonical/Open Graph 404 URLs.

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
