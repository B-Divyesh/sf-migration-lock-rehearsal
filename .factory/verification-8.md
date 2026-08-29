# Independent verification 8

## Verdict: FAIL

Candidate `3c8bb321ace57ca5547391a8c387c124064dcf7c` is not ready for
release. The live site at <https://migration-lock-rehearsal.sociobot.in>
matches the candidate byte for byte, so this is not a deployment-only failure.
The installed CLI can still hang forever during rollback and bypass its signal
recovery path. Additional contract failures are documented below.

Verified independently on 2026-08-29 UTC from a clean checkout at the exact
candidate SHA. Product code was not changed.

## Release-blocking findings

### High — rollback has no deadline or working signal recovery

The migration and workload children are polled against
`--max-statement-ms`, but rollback is run synchronously. Postgres calls the
blocking `psql(...).is_ok()` path and ClickHouse calls the blocking
`clickhouse_file(...).is_ok()` path. Neither path checks the configured
deadline or the interrupt flag.

I installed the packaged crate into an empty prefix and ran it with a
deterministic Docker command double. The double completed container setup,
fixture load, migration, measurements, and then held the rollback command
open. For both Postgres and ClickHouse I used
`--max-statement-ms 10`, waited 300 ms, sent SIGTERM directly to the CLI, and
observed it again 900 ms later:

```text
exited: false
report.json: absent
runbook.md: absent
docker rm -f: absent from command log
last command: ... /work/rollback.sql
```

The verifier then force-killed the isolated process. A rollback that blocks on
a lock therefore prevents the product from reporting rollback risk, and an
operator interrupt during that stage does not run the promised recovery path.
This is central to the researched job-to-be-done.

Run rollback as a monitored child for both engines. Enforce the statement
deadline, handle SIGINT/SIGTERM, write `failure_stage: "rollback"` NO-GO
artifacts, and guarantee container cleanup. Add a claim test that holds the
rollback child open for both engines.

### High — the broad failed-command promise is false and unlisted

The live page says, “A failed command or exceeded limit writes NO-GO.” The
README says failed commands are always NO-GO. The `failed-command-no-go` claim
and test cover workload, measurement, and migration only.

With the installed package and a Docker command double that failed `docker
run`, the CLI exited 1 with a clear stderr error, but did not create the output
directory, `report.json`, or `runbook.md`:

```text
mlr: docker command failed: docker run ... postgres:16-alpine
EXIT=1 OUTPUT_EXISTS=no REPORT_EXISTS=no RUNBOOK_EXISTS=no
```

Container startup, readiness, copy, and fixture-load failures use the same
early-return pattern. Either make every rehearsal command failure write NO-GO
artifacts and test those stages, or narrow the landing and README copy to the
stages actually covered. The current statement violates the claims contract.

## Other findings

### Medium — the 404 page clips navigation at 200% text

At a 390 px viewport with the root text size set to 200%, `/`, `/demo`,
`/privacy`, and `/terms` retain a 390 px scroll width. The deployed 404 page
widens to 432 px. Its navigation spans from x=102 to x=432 and the Privacy link
is clipped off the right edge. `public/404.css` does not carry the wrapping
rules used by the main site. The current regression test omits the 404 route.

### Medium — paid terms omit required merchant and refund information

The site correctly states the $29 one-time price and what the license adds, but
`/terms` only says checkout and verification use Sociobot. It does not state
that Sociobot/Dodo is the merchant of record or that refunds are handled there,
as required by the paid-unlock contract.

## Mandatory claims gate

`.factory/claims.json` exists with 19 entries, and each ID occurs exactly once
as an `@claim:<id>` test. To honor the requested order, every command was
invoked before other QA. The literal pre-install invocation could not load the
Node suite because a clean clone has no `node_modules`. After the required
`npm ci`, every command was run again separately as the authoritative
clean-install gate.

| Claim | Installed clean-clone result |
|---|---|
| `demo-report` | PASS |
| `local-only` | PASS |
| `site-private` | PASS |
| `supported-engines` | PASS |
| `demo-reset` | PASS |
| `browser-demo-reset` | PASS |
| `demo-recording` | PASS |
| `invented-sample` | PASS |
| `chosen-output` | PASS |
| `docker-rehearsal` | Local command passed with its declared no-Docker skip; exact-SHA CI passed the real-container run |
| `container-cleanup` | Local command passed with its declared no-Docker skip; exact-SHA CI passed the real-container run |
| `rollback-no-go` | PASS |
| `failed-command-no-go` | PASS for its listed stages; does not cover the broader live claim above |
| `child-deadlines` | PASS for migration and workload; rollback is not covered |
| `interruption-cleanup` | PASS during migration; rollback is not covered |
| `threshold-verdict` | PASS |
| `safe-json` | PASS |
| `paid-license` | PASS |
| `free-cli` | PASS |

The candidate's GitHub Actions run
[33245825943](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33245825943)
completed successfully at this exact SHA. Its `real-containers` job ran the
required Docker claim pattern on Ubuntu.

## First-read and one-click demo

The first-read gate passes. Cold, the page says “Rehearse your migration before
production,” identifies Postgres and ClickHouse maintainers, and says the
result includes lock waits, table growth, and rollback results. “Try it with
sample data” is the first action and says it will show the bundled go/no-go
report. The action and all three facts fit in the 390 × 844 first screen.

One keyboard-activated click entered `/?demo=1`, showed the persistent
sample-data warning, and started the recorded CLI output. Reset demo restored
the first command, and Install the CLI provided the real-use path. Demo storage
and cookies remained empty. Back navigation restored focus to the demo H1.

## Clean source, build, package, and CLI evidence

- Initial SHA and remote `main`:
  `3c8bb321ace57ca5547391a8c387c124064dcf7c`.
- `npm ci`: PASS; 20 packages installed and 0 audit vulnerabilities.
- `npm test`: PASS; 8 Rust unit tests and 22 Node/browser tests passed; the two
  declared real-Docker tests skipped locally because no Docker daemon exists.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS (`cargo fmt --check` and clippy with warnings denied).
- `npm run build`: PASS; exact deployment output produced in `dist/site/`.
- `cargo package --locked --allow-dirty`: PASS; 18 files, 71.0 KiB unpacked,
  17.6 KiB compressed.
- Clean consumer install from the packaged crate: PASS; installed `mlr` binary
  size 713,144 bytes.
- Installed Postgres and ClickHouse dry-runs wrote parseable GO JSON and
  runbooks. `--json`, exact IPv4/IPv6 loopback guards, and help passed.
- A remote-looking host, MySQL, missing required flags, blank output, and a
  statement threshold below the sample value all exited non-zero. The
  threshold case wrote parseable NO-GO artifacts.

Docker is unavailable in this verifier, so a fresh real-container run could
not be repeated locally. The exact-SHA CI result above covers the declared
Postgres 16 and ClickHouse 24.8 container claims. The blocking rollback
reproduction is deterministic and does not depend on a Docker daemon.

## Live deployment, privacy, accessibility, and performance

- Rebuilt `/`, `/demo`, `/privacy`, `/terms`, hashed JS/CSS, hero art,
  `demo-recording.json`, `robots.txt`, and `sitemap.xml` are byte-for-byte
  identical to the live responses.
- `/`, `/demo`, `/privacy`, `/terms`, and `/404` return 200. An unknown route
  returns the designed page with HTTP 404. All expected internal links and the
  GitHub link resolve; checkout returns 303 to `checkout.dodopayments.com`.
- Desktop 1440 × 900 and mobile 390 × 844 have one H1, one main landmark,
  `lang=en`, correct route titles, no ordinary-width overflow, and no
  console/page errors.
- Axe found zero serious or critical violations on all five routes at both
  widths. The project URL verifier passed locally and live.
- Keyboard order, skip link, navigation, demo, terminal, purchase, restore,
  legal, and footer controls are operable. Tested focus indicators are 4 px
  blue. Normal-width touch targets are at least 44 px.
- Reduced motion computes the terminal cursor animation to `none`.
- Before any license action, all requests are same-origin and local storage,
  session storage, and cookies remain empty. There are no third-party fonts,
  scripts, or analytics.
- A fake returned license was stripped from the URL, stored only in the two
  documented namespaced keys, sent only to `api.sociobot.in`, cached across a
  reload, and removed by the UI.
- Billing verification allowance observed: requests 1–30 returned 200;
  request 31 returned 429 with `Retry-After: 3`.
- HTML responses use a 30-second revalidation cache. Hashed assets use
  `max-age=31536000, immutable`. CSP, HSTS, `nosniff`, strict-origin referrer
  policy, and header-level `frame-ancestors 'none'` are present.
- Initial production assets: JS 13,229 bytes (5.10 KiB gzip), CSS 6,685 bytes
  (2.23 KiB gzip), hero 107,866 bytes. All are within budget.
- Fresh Lighthouse mobile run: performance 99, accessibility 100, best
  practices 100, SEO 100; FCP 0.8 s, LCP 1.4 s, TBT 120 ms, CLS 0; total
  transfer 114 KiB.

The product is not a PWA, has no product-owned backend, and has no sign-in, so
service-worker/offline reload, backend persistence/concurrency/health, and
Entra authority checks do not apply. The Sociobot billing endpoint was checked
for its required request allowance as described above.

## Release decision

FAIL until rollback has a bounded, interruptible recovery path with claim
coverage; the broad failed-command promise is made true or narrowed and
tested; the 404 reflows at 200% text; and the paid terms contain the required
merchant-of-record and refund information.
