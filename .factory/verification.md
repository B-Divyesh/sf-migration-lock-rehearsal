# Independent verification — FAIL

**Candidate:** `9de38a35115afeedc61a59e98443f496e9c6f6e6`  
**Live URL:** https://migration-lock-rehearsal.sociobot.in  
**Verified:** 2026-08-28 (UTC)  
**Result:** **FAIL — do not release**

## Cold first read

The cold landing page says: “Rehearse your migration before production,” for
“database maintainers” who need “lock, rewrite, and rollback estimates,” and
its first primary action is **Try it with sample data** with the immediate
outcome “See the bundled go/no-go card.” This part passes the plain-words and
one-click demo gate.

## Mandatory claims gate

`npm ci` was run from the clean candidate, then every exact command in
`.factory/claims.json` was run before broader QA:

| Claim | Command | Result |
| --- | --- | --- |
| `demo-report` | `npm test -- --test-name-pattern @claim:demo-report` | PASS |
| `local-only` | `npm test -- --test-name-pattern @claim:local-only` | PASS |
| `site-no-third-party` | `npm test -- --test-name-pattern @claim:site-no-third-party` | PASS |
| `license-checkout` | `npm test -- --test-name-pattern @claim:license-checkout` | PASS |

Each invocation ran the two Rust tests and all four Node claim tests because
of the package test command. The claim suite is insufficient for the checkout:
it checks only that the source contains the URL; the live link is dead (P1).

## Local build and CLI checks

- `npm test`: PASS (2 Rust + 4 claim tests).
- `npm run build`: PASS. `dist/site/` is produced.
- `cargo build --release`: PASS.
- `cargo package --allow-dirty`: PASS; package verification compiled it.
- `cargo fmt --check && cargo clippy --all-targets -- -D warnings`: PASS.
- Installed into a clean temporary Cargo root with `cargo install --path . --root <temp>`: PASS. `mlr --help`, `--version`, Postgres dry-run demo, ClickHouse dry-run demo, local guard, remote refusal, and missing-fixture recovery were exercised.
- Real Docker engine rehearsals could not be run because this verifier container has no `docker` binary. This does not excuse the deterministic safety failure below.

## Live deployment and browser checks

Fresh production-build asset hashes exactly matched the live assets:

- JS `index-CQ5ZCVxT.js`: `66ab1cf94c40e897d18fff2052d59fbf593d67917b57eb1285e64aa5a644771e`
- CSS `index-BIwoHYxj.css`: `e5c2155f953c572b429ad4e2e12f3bafc3c8dde1caa93fc4459c60d030ab5463`
- Hero `lock-stack-DSVDfjcR.webp`: `ca610fb8c0e7433dd49756562982bfcf3ea6c4016477a3d1536fbe3df80dbc40`

Desktop and 390 px Playwright coverage of `/`, `/demo`, `/privacy`, and
`/terms` found one h1 and one main per page, no horizontal overflow, no page
or console errors, and zero axe serious/critical violations. Keyboard tab
order, the skip link, focus styling, the one-click demo/reset flow, and
reduced-motion CSS were exercised. Initial landing/demo request logs contained
only same-origin document/assets; no analytics or third-party font/script
requests were observed. The generated report uses 3.68 KB gzip JS, 1.99 KB
gzip CSS, and a 107.87 KB WebP hero, within stated budgets.

Live response headers include HSTS, `nosniff`, strict-origin referrer policy,
and a restrictive CSP. The license verification path was also exercised with
an invalid token: it called only `api.sociobot.in`, stripped the query token,
and displayed the invalid-license state. The factory verification API allowed
30 sequential requests and returned `429` with `Retry-After: 4` on request
31; this rate-limit requirement passes.

## Release-blocking defects

### P0 — a failed rollback is declared GO

The core promise is an operator-facing go/no-go card that reveals rollback
risk. In `src-cli/main.rs`, both real-engine paths unconditionally construct
`verdict: "GO"`, even when `rollback` is false. I exercised the shipped
release binary with a Docker command test double that permits fixture and
migration commands but makes `/work/rollback.sql` fail. The CLI exited 0 and
wrote:

```json
{"rollback_checked": false, "verdict": "GO"}
```

Its runbook simultaneously says `**Verdict: GO**` and `Rollback checked:
false`. This can actively approve a migration whose supplied rollback failed.
It is unsafe for the job described in the brief. A rollback failure must
produce a non-GO verdict and a non-zero/explicitly actionable outcome, with a
regression test for both engines.

### P1 — the live $29 checkout link is dead

`GET` and `HEAD` on the exact linked URL
`https://api.sociobot.in/api/v1/products/migration-lock-rehearsal/checkout`
returned HTTP 404 with `{"error":"enabled factory product","status":404}`.
This violates the no-dead-links requirement and makes the advertised license
unbuyable. Register/enable the product in the billing engine and replace the
source-string claim test with a live/staged observable checkout assertion.

### P1 — unsupported MySQL can receive a fabricated rehearsal card

The documented engines are Postgres and ClickHouse, but
`mlr demo --engine mysql --dry-run` exits 0 and writes a report labelled
`"engine": "mysql"` containing the bundled Postgres-shaped sample metrics.
This is a misleading estimate for an unsupported engine. Validate the engine
before dry-run/report generation and reject every value other than `postgres`
or `clickhouse`.

### P1 — documented CLI reset command does not exist

`.factory/demo.md` says `mlr demo --reset` removes the named demo directory.
The released binary responds `mlr: unknown option --reset` and exits 1. The
documented demo reset/recovery path is broken. Implement it safely or remove
the claim and documentation.

### P1 — unknown paths are HTTP 200, not a real 404

Live navigation to `/does-not-exist` renders the styled not-found screen but
the response status is HTTP 200. The deploy therefore fails the explicit real
404 requirement and can mislead crawlers/clients. Configure the hosting
fallback/response override so unknown routes serve the styled page with HTTP
404.

### P1 — required mobile touch targets are too small

At 390 px, live measured target sizes include header `Demo` 40 × 21.6 px,
`How it works` 88.8 × 21.6 px, `Privacy` 51.2 × 21.6 px, wordmark 69.4 ×
38.4 px, and footer `Privacy` 45.1 × 15 px / `Terms` 37.4 × 15 px. The
accessibility contract requires every touch target to be at least 44 px.
Give these links sufficient inline-block padding/min-height while preserving
spacing.

## Non-blocking follow-ups

- Hashed live JS/CSS assets use `Cache-Control: public, must-revalidate,
  max-age=30`, rather than long-lived immutable caching required for hashed
  assets.
- `mlr` accepts `--json` but ignores it and prints its normal human output.
  Either implement documented JSON stdout for scripting or reject the option;
  the JSON report file is useful but is not equivalent CLI flag behavior.
- Several visible assertions, including “Sanitized sample included” and paid
  “future engine checks,” are not represented as observable claims in
  `.factory/claims.json`. Audit/remediate per the claims contract.

## Acceptance conclusion

The deployment matches the requested commit, and much of the static-site QA
passes, but the P0 false-GO rollback result alone makes this unsuitable for
release. The candidate is **FAIL** until every release-blocking defect above
is fixed and independently reverified.
