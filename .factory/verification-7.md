# Independent verification 7

## Verdict: FAIL

Candidate `23971587b6dc981ee4718f2e87014317685754c0` is not ready for release. The live site at <https://migration-lock-rehearsal.sociobot.in> matches the candidate and the declared claim tests pass, but an independently exercised blocked migration has no deadline or recovery path. The CLI can wait forever without writing the promised NO-GO report or runbook. Terminating that stuck run also bypasses container cleanup.

Verified on 2026-08-29 UTC from a clean worktree at the exact candidate SHA. Product code was not changed.

## Release-blocking findings

### High — a blocked migration bypasses the statement limit, report, and cleanup

The Postgres and ClickHouse migration loops poll the child until it exits. `--max-statement-ms` is evaluated only in `completed_report`, after the child has finished. `finish_workload` likewise waits without a deadline. A DDL statement blocked indefinitely on a production-shaped lock therefore never reaches a GO/NO-GO decision.

Independent deterministic reproduction used an installed release CLI and a Docker command double whose migration remained active:

```text
mlr rehearse ... --max-statement-ms 10
external watchdog after 1 second: exit 124
report.json: absent
runbook.md: absent
docker rm -f mlr-...: not called
```

The command log showed repeated lock-wait measurements after the 10 ms limit, with no cancellation. This is the central risk named by the brief, not an edge case. Add an enforced migration/workload deadline, write a NO-GO report on expiry, terminate both children, and handle SIGINT/SIGTERM so the disposable container is removed.

### High — the dry-run's “measured results” claim is not listed or proved

README line 11 says the offline dry-run writes a sample report “with measured results.” The dry-run does not run a database; `sample()` writes fixed values (`184`, `0`, `32768`, `40960`). No `.factory/claims.json` entry names or proves provenance for “measured results.” The manifest's `demo-report` claim proves that files and fixed sample fields exist, not that the values were measured.

This violates the claims contract's unlisted-claim rule. Describe these as sample values, or add truthful provenance and a matching claim test.

## Other findings

### Medium — reduced-motion mode still declares an infinite animation

With `prefers-reduced-motion: reduce`, the live cursor computes to `animation: 1e-05s steps(2) infinite blink`. The design contract says reduced-motion users get a static cursor. A 20-frame sample changed opacity once and then remained at `0.5`, but the animation remains infinite. Disable the cursor animation in the reduced-motion query rather than shortening every iteration.

### Medium — two mobile/keyboard focus targets miss the accessibility baseline

- The live “Get the source on GitHub” link is `193 × 19` CSS px at 390 px, below the required 44 px touch height.
- The focusable terminal `<pre tabindex="0">` receives only the browser's `1px` black `auto` outline, unlike the designed 4 px blue focus ring used elsewhere.

Keyboard order, skip navigation, buttons, route focus, and form controls otherwise worked without a trap.

### Low — missing required rehearsal flags produce an incomplete error

Running `mlr rehearse` exits 1 with `mlr: read : file not found`. It should name the missing `--fixture` and `--migration` flags and tell the operator how to proceed.

## Mandatory claims gate

`.factory/claims.json` exists with 17 entries. Every listed command was run separately, exactly as declared, after `npm ci`.

| Claim | Result |
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
| `docker-rehearsal` | PASS in exact-SHA CI; locally skipped because Docker is unavailable |
| `container-cleanup` | PASS in exact-SHA CI; locally skipped because Docker is unavailable |
| `rollback-no-go` | PASS |
| `failed-command-no-go` | PASS |
| `threshold-verdict` | PASS |
| `safe-json` | PASS |
| `paid-license` | PASS |
| `free-cli` | PASS |

The exact candidate's GitHub Actions run [33243716627](https://github.com/B-Divyesh/sf-migration-lock-rehearsal/actions/runs/33243716627) completed successfully. Its `real-containers` job ran `@claim:(docker-rehearsal|container-cleanup)` with Docker, Postgres 16, and ClickHouse 24.8.

## First-read and demo

First-read: “Rehearse your migration before production” explains the job. The next sentence identifies Postgres and ClickHouse maintainers and names lock waits, table growth, and rollback results. “Try it with sample data” is the obvious first click and says it will show the bundled go/no-go report. The required first-read test passes on desktop and 390 px mobile; all three facts and the action fit in the 844 px mobile first screen.

The one-click action opened `/?demo=1`, moved focus to “Read a sample go/no-go report,” displayed the persistent sample-data warning, and began the bundled terminal recording. Enter activated the link; Space reset the demo to its first command. Browser storage and cookies remained empty.

## Clean source, build, and package evidence

- `git rev-parse HEAD`: `23971587b6dc981ee4718f2e87014317685754c0`; initial worktree clean.
- `npm ci`: PASS; 20 packages installed, 0 audit vulnerabilities.
- `npm test`: PASS; 8 Rust tests and 20 Node/browser tests passed, 2 documented Docker-only tests skipped locally.
- `npm run typecheck`: PASS.
- `npm run lint`: PASS (`cargo fmt --check`; clippy with warnings denied).
- `npm run build`: PASS; exact output in `dist/site/`.
- `cargo package --locked --allow-dirty`: PASS; 18 files, 63.1 KiB unpacked and 16.1 KiB compressed.
- Clean `cargo install --path /work/repo --root <temp> --locked`: PASS.
- Installed `mlr --help`, Postgres dry-run, ClickHouse dry-run, `--json`, local URL guard, and remote-decoy rejection: PASS.

Boundary tests confirmed equality is accepted at 184 ms, 0 ms lock wait, and 8,192 bytes growth; one less than either measured statement/growth value produces NO-GO and exit 1. Negative/overflow limits, blank output, unknown engine/command, missing files, remote-looking URLs, and nonempty unmarked reset targets fail without unsafe writes. A marked demo resets safely and a second reset reports nothing to remove.

## Live deployment and browser evidence

- Rebuilt route HTML, hashed JS/CSS, hero art, and `demo-recording.json` are byte-for-byte identical to the live responses.
- `/`, `/demo`, `/privacy`, `/terms`, and `/404` return 200; an unknown URL returns the designed page with HTTP 404.
- All discovered internal links return 200. GitHub returns 200. Checkout returns 303 to `checkout.dodopayments.com`.
- Desktop 1440 × 900 and mobile 390 × 844: one H1, one main landmark, `lang=en`, no horizontal overflow, no console/page errors.
- Axe: zero violations on all five routes at both widths; specifically zero serious/critical findings.
- Keyboard: skip link, navigation, demo action, terminal, install, purchase, restore, legal, and footer controls are reachable; Enter/Space work; route changes focus the new H1.
- No third-party font or script requests. With no license action, every request is same-origin and local/session storage and cookies remain empty.
- A real invalid-license action sent only the URL-encoded token to `api.sociobot.in`, stored only the two documented namespaced keys, showed the inactive state, and removed both keys on command.
- Billing verify rate limit: 30 consecutive requests returned 200; request 31 returned 429 with `Retry-After: 3` and `X-RateLimit-After: 3`.
- Response headers include CSP with `frame-ancestors 'none'`, HSTS, `nosniff`, and strict-origin referrer policy. HTML caches for 30 seconds; hashed assets use `max-age=31536000, immutable`.
- Initial assets: JS 13,229 bytes (5.10 KiB gzip), CSS 6,511 bytes (2.19 KiB gzip), hero 107,866 bytes.
- Lighthouse mobile: performance 99, accessibility 100, best practices 100, SEO 100; FCP 0.9 s, LCP 1.5 s, TBT 100 ms, CLS 0.

The product is not a PWA and has no product-owned backend or sign-in, so service-worker/offline-reload, backend persistence/concurrency/health, and Entra checks do not apply. The optional Sociobot billing endpoint was tested as described above.

## Release decision

FAIL until the blocked-command timeout/cleanup path and the unlisted dry-run measurement claim are corrected and covered by claims tests. The accessibility findings should be closed in the same repair.
