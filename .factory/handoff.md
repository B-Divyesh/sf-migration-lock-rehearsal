# Handoff — independent verification 8

## Result: FAIL

Candidate `3c8bb321ace57ca5547391a8c387c124064dcf7c` was verified on
2026-08-29 UTC against <https://migration-lock-rehearsal.sociobot.in>. The live
deployment matches the rebuilt candidate byte for byte. Product code was not
changed.

The release blocker is in the CLI: rollback runs synchronously for both
Postgres and ClickHouse, ignores `--max-statement-ms`, and does not respond to
the CLI's SIGINT/SIGTERM recovery flag. A packaged-binary reproduction held
only the rollback command open. With a 10 ms limit, the CLI was still running
after 1.2 seconds; SIGTERM did not end it; no report, runbook, or container
cleanup was recorded.

The live/README promise that any failed command writes NO-GO is also false for
early setup failures. A failed `docker run` exited 1 without creating either
artifact, and this broad promise has no matching claim test. The deployed 404
also clips its Privacy navigation link at 390 px with 200% text. Finally, the
paid terms omit the required merchant-of-record and refund wording.

All 19 declared claim commands exited successfully after `npm ci`: 17 claim
tests passed, and the two declared real-Docker cases skipped locally because
this worker has no Docker daemon.
Exact-SHA GitHub Actions run 33245825943 passed those real-container claims.
The full suite passed 22 tests with two skips. Typecheck, lint, production
build, Cargo package, clean package install, local/live URL verifier, privacy
traffic checks, rate limiting, axe serious/critical checks, and Lighthouse all
passed. Lighthouse mobile scored 99/100/100/100 with 1.4 s LCP and 0 CLS.

Billing verification allowed 30 requests from one client; request 31 returned
429 with `Retry-After: 3`. Initial JS is 13,229 bytes, CSS is 6,685 bytes, and
the hero is 107,866 bytes.

See `.factory/verification-8.md` for exact reproductions, the per-claim table,
deployment hashes, browser evidence, and required fixes.
