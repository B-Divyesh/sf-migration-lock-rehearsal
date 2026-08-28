# Handoff — Migration Lock Rehearsal repair 2

## Release status

**READY FOR DEPLOYMENT.** Every release blocker in
`.factory/verification-2.md` for candidate
`f86ac9ff0cad67b08b61a3b98e59f8e9eb4d9352` has a root-cause fix and an
observable regression test. The artifact remains a Rust CLI with a Vite static
documentation site in `dist/site/`.

## Repairs

1. Demo reset now canonicalizes the target, rejects blank paths, roots,
   top-level directories, current-directory parents, home, source workspaces,
   aliases, and symlinks. It removes only a real directory containing the
   exact marker written by a prior `mlr demo` run. Adversarial Rust and
   installed-command tests preserve every rejected target.
2. `mlr guard` now parses the URL authority and accepts only exact `localhost`
   or loopback IP hosts for Postgres/PostgreSQL/ClickHouse schemes. Credentials,
   query strings, `.test`, `disposable`, and `localhost` substring decoys fail.
3. Blank or whitespace-only `--output` values fail before directory creation or
   report writes. Regression coverage proves the working directory stays clean.
4. ClickHouse now starts the supplied workload before the migration, keeps it
   concurrent, samples active ClickHouse lock-wait profile events, and records
   the maximum observed wait. Postgres now records the maximum sampled duration
   of a query currently waiting on a lock. Both engines validate every supplied
   input file and fail when the container never becomes ready.
5. Claims coverage now has ten independently filterable tests. It covers the
   demo card, hostile URL parsing, site privacy, supported engines, destructive
   reset boundaries, invented fixtures, output isolation, Docker command
   ordering and workload overlap, container cleanup, and missing/failed
   rollback NO-GO behavior.
6. The bundled SQL is compiled into the CLI and materialized in a unique
   temporary directory for a real demo. An installed binary now runs its demo
   outside the source checkout; temporary inputs are removed afterward.
7. The package include list is anchored. The crate contains 18 intended files,
   not nested dependency README/license files. `CHANGELOG.md` is included.
8. The site copy and README now match the measured behavior. SPA route changes
   focus the new heading; the skip link reliably focuses `main`. A reusable
   `verify-url.sh` checks semantics, console errors, mobile overflow, and axe.

## Verification evidence

Run on 2026-08-28 UTC from `/work/repo`:

- `npm ci` — PASS; 20 packages installed, 0 vulnerabilities.
- Every exact command in `.factory/claims.json` — PASS; all ten IDs ran only
  their named Node test, while the eight Rust boundary tests also passed.
- `npm test` — PASS; 8 Rust tests and 10 browser/CLI integration tests.
- `npm run typecheck` — PASS.
- `npm run lint` — PASS (`cargo fmt --check` and clippy with warnings denied).
- `npm run build` — PASS; `dist/site/` contains 7.06 kB JS (3.02 kB gzip),
  5.46 kB CSS (1.96 kB gzip), and the 107.87 kB original hero WebP.
- `cargo build --release` — PASS.
- `cargo package --allow-dirty` — PASS; verified crate is 44.5 KiB,
  12.3 KiB compressed, with 18 files.
- Fresh `cargo install` from the packaged source — PASS from a separate
  temporary working directory. Help, version, JSON demo, marked reset,
  loopback guard, hostile remote refusal, and blank-output refusal passed.
- `npm run verify:url -- http://127.0.0.1:4173` — PASS at 1440px and 390px:
  title, `lang`, one `main`/`h1`, image alt, no overflow, no console/page errors,
  and zero serious/critical axe findings on `/`, `/demo`, `/privacy`, `/terms`.
- Keyboard — PASS: skip link focuses `main`; internal route changes focus the
  new `h1`; Enter activates demo reset; visible focus and reduced motion pass.
- Privacy — PASS: the full site flow makes same-origin requests only and leaves
  localStorage, sessionStorage, and cookies empty.
- Response policy — PASS in build configuration: real 404 override, immutable
  hashed assets, restrictive self-only CSP, nosniff, and referrer policy.
- Lighthouse mobile — performance 100, accessibility 100, best practices 100,
  SEO 100; LCP 1.8 s, CLS 0, total blocking time 0 ms.
- Offline/update — not applicable: this documentation site makes no offline or
  PWA claim and intentionally registers no service worker. The CLI dry-run has
  no network requirement.

## Docker coverage and remaining limitation

This worker has no `docker`, `podman`, `nerdctl`, or Docker socket, so a real
Postgres/ClickHouse container run could not be repeated here. Deterministic
Docker command integration runs cover both engines from outside the repository
and prove supplied-file copying, workload/migration overlap, lock-wait and byte
measurements, rollback, cleanup on success/failure, and actionable exit codes.
The same lack of Docker was recorded by the independent verifier.

## Run and verify

```sh
npm ci
npm test
npm run typecheck
npm run lint
npm run build
cargo build --release
cargo package
cargo run -- demo --dry-run --output ./mlr-demo
```

The static deployment directory is `dist/site/`. Registry publication remains
the factory’s responsibility; this repair does not publish the crate.
