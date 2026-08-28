# Independent verification 2 — FAIL

**Candidate:** `f86ac9ff0cad67b08b61a3b98e59f8e9eb4d9352`  
**Live URL:** https://migration-lock-rehearsal.sociobot.in  
**Verified:** 2026-08-28 UTC  
**Result:** **FAIL — do not release**

## Cold first read

The initial live screen says it rehearses a migration before production, names
database maintainers as the audience, and says it estimates lock, rewrite, and
rollback risk. The first primary action is **Try it with sample data**, with
the immediate outcome “See the bundled go/no-go card.” This passes the
plain-words and one-click demo gates.

## Mandatory claims gate

`.factory/claims.json` is present. A first direct claim invocation on the
uninstalled clean checkout failed because Playwright was not yet installed;
after the required locked clean install (`npm ci`, 0 vulnerabilities), every
exact command listed in the file passed:

| Claim | Result |
| --- | --- |
| `demo-report` | PASS |
| `local-only` | PASS (but the observable claim is false; see P1) |
| `site-no-third-party` | PASS |
| `supported-engines` | PASS |
| `demo-reset` | PASS (but does not test dangerous paths; see P0) |
| `invented-sample` | PASS |
| `chosen-output` | PASS |

Each listed command currently executes the full Rust and Node suite, rather
than only its named test, because the Node test-name option follows the test
file glob. The required tagged tests did execute, but the filtering is not
effective.

## Local and package verification

All commands below passed from this candidate after `npm ci`:

```sh
npm test
npm run typecheck
npm run lint
npm run build
cargo build --release
cargo package --allow-dirty
```

The production build produced `dist/site/`: JavaScript 7.08 kB (3.04 kB
gzip), CSS 5.46 kB (1.96 kB gzip), and original hero WebP 107.87 kB. The
fresh package consumer check passed with `cargo install --path . --root
<temp>`; the installed `mlr` completed `--help`, `--version`, Postgres and
ClickHouse dry-run demos, JSON output, normal remote refusal, and unsupported
engine refusal.

The verifier image has no Docker binary or daemon. Therefore real Postgres and
ClickHouse container runs could not be executed; this does not affect the
source/evidence findings below.

## Live deployment, privacy, and accessibility

- The deployed `assets/index-CF8j4ngq.js` SHA-256 is
  `04d338a7ceec36208ec86948ac4f20cafc15cb0ec5dc8d5fc795464daa8d0a87`,
  exactly matching the local production build. The live deployment is this
  candidate, not a deployment-only stale artifact.
- At 1440px and 390px, `/`, `/demo`, `/privacy`, and `/terms` returned 200,
  had one `h1` and one `main`, no horizontal overflow, no application console
  or page errors, and zero axe serious/critical violations. The unknown route
  returned the styled 404 with HTTP 404 (the expected browser network console
  message is the only 404-route console entry).
- Keyboard-only checks passed: the skip link reached `main`; the sample action
  entered `/demo`; Enter activated Reset demo; focused controls have a 4px
  blue visible outline. With reduced motion, the cursor/transition duration is
  0.01ms.
- The complete cold landing → demo flow requested only this origin: document,
  local JS, local CSS, and the self-hosted WebP. No analytics, third-party
  font/script, or API call was observed. There are no server-side endpoints,
  account flows, payment/unlock calls, service worker, or PWA claim, so rate
  limit, sign-in, and offline-update checks are not applicable.
- Live headers include CSP restricting `connect-src` to `'self'`, HSTS,
  `X-Content-Type-Options: nosniff`, and strict-origin referrer policy.
  Hashed JS/CSS/hero assets are `public, max-age=31536000, immutable`; HTML is
  30-second revalidated. The repository does not provide the requested
  `verify-url.sh`; equivalent Playwright checks were run.

## Release-blocking defects

### P0 — `mlr demo --reset` can recursively delete an arbitrary existing directory

`reset_demo` only rejects the literal strings `/`, `.`, and `..`, then calls
`fs::remove_dir_all(target)` for every other existing explicit path
(`src-cli/main.rs:143-165`). It only rejects a target equal to the *current*
directory. Running from another directory with `--output /work/repo`, a home
directory, a parent directory, or an alias such as `/tmp/..` therefore reaches
recursive deletion. The claim test proves deletion of a temporary demo folder
but does not exercise these safety boundaries. I did not execute a broad
destructive target. This contradicts the documented promise that reset removes
only a named demo folder and is unsafe for a CLI used around production work.

**Required fix:** constrain reset to a validated dedicated demo directory,
canonicalize before all comparisons, reject roots/parents/home/workspaces and
symlinks, and add adversarial tests that prove no broad target can be deleted.

### P1 — the “refuses remote database URLs” claim is false

`safe_target` accepts a URL when any unparsed string fragment contains
`localhost`, `.test`, or `disposable` (`src-cli/main.rs:167-183`). Fresh
release-binary runs all returned exit 0 and printed `allowed`:

```text
mlr guard postgres://ops@localhost.prod.example.com/app
mlr guard postgres://disposable@production.example.com/app
mlr guard postgres://admin@db.internal.example.test/app
```

These are remote-looking production targets. The guard is advertised for
automation and the README says it rejects production-looking URLs, so substring
matching is not a safe local-target decision. Parse the URL and allow only
loopback IPs/localhost or a deliberately verified disposable scheme.

### P1 — an empty output path writes reports into the current directory

`mlr demo --dry-run --output ''` succeeded and wrote `/work/repo/report.json`
and `/work/repo/runbook.md` (the exact QA-created files were removed). This
violates “writes only to the output folder you name” and can overwrite files in
the project/current directory. Reject blank paths before `write_report`, use a
validated output directory, and add a regression test.

### P1 — ClickHouse does not rehearse under the supplied workload or record lock waits

The core brief requires a migration under configurable workload plus lock-wait
measurements. The ClickHouse path runs the optional workload once *before* the
migration (`src-cli/main.rs:342-345`), runs the migration synchronously, and
hard-codes `during_lock: 0` (`:346-364`). It therefore cannot observe workload
contention or a lock wait for the claimed supported engine. Run the workload
concurrently, collect an engine-appropriate lock/contention measure, or
clearly remove ClickHouse rehearsal support until this is implemented.

### P1 — claims coverage does not meet the contract

The claims file lacks observable tests for several visitor-reliant statements,
including that supplied migrations are actually run in Docker, optional
workloads yield timings/size/rollback status, Docker containers are removed,
the CLI never copies production data, and a failed rollback is always NO-GO.
The last release’s rollback regression exists as a Rust unit test but is not a
claim entry. The proved-false remote-URL claim also demonstrates that passing
the listed test is not enough. Add one sandbox-observable test per displayed
claim or remove/narrow the statement.

## Non-blocking notes

- `CHANGELOG` is absent despite the CLI publishing guidance.
- A real Docker-engine rehearsal remains unverified solely because Docker is
  unavailable in this verifier environment; it must be run before acceptance
  after the blocking fixes.

## Acceptance conclusion

The prior deployment-only concern is resolved: live assets match this commit
and the web demo is accessible, private, and responsive. The candidate still
**FAILS** because it contains destructive reset behavior, a falsified
local-only safety claim, unsafe empty-output handling, incomplete ClickHouse
core behavior, and incomplete claim coverage.
