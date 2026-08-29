# Handoff — polish 1

## Result

Released repair commit: `ddc1ffec48dc60fc17bba7f4b57416348d57e078` (plus documentation evidence in the following commit).

All 18 findings in `.factory/review-1.md` are addressed. The static site was deployed through `/opt/fleet/lib/deploy-static.sh` to `https://migration-lock-rehearsal.sociobot.in` (Azure deployment `50659fcb-f285-44c2-b4a7-35bb334fbf8e`) and cold-checked after deployment.

## Verification

- Fresh final clone: `/tmp/mlr-clean-ddc1ffe`; `npm ci && npm test` exited `0`. 21 tests passed; two real-container tests were intentionally skipped because this disposable worker lacks kernel namespace permissions. The exact output is `clean-test.log` in that temporary clone.
- Real-container gate: `.github/workflows/docker-claims.yml` runs `@claim:docker-rehearsal` and `@claim:container-cleanup` with `MLR_REQUIRE_DOCKER=1` on GitHub's Ubuntu Docker runner. The tests run bundled Postgres 16 and ClickHouse 24.8 samples and now exercise both success and SQL-failure cleanup.
- Local gates passed: `npm run typecheck`, `npm run lint`, `npm run build`, `cargo build --release`, `cargo package --allow-dirty --no-verify`, and `npm pack --dry-run`.
- Live checks passed: `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in`; zero serious/critical axe findings, no console errors, correct title/lang/main/alt, and no 390px overflow.
- Cold live browser flow passed: `/?demo=1` showed the persistent banner; Reset demo restarted the terminal at `$ mlr demo --dry-run --output ./mlr-demo` with empty storage; the designed unknown-route page had the expected title. Screenshots: `.factory/evidence/polish-1/live-mobile-home.png`, `live-mobile-demo-reset.png`, and `live-mobile-404.png`.
- Build budget: initial JS `13.22 kB` raw / `5.10 kB` gzip; CSS `6.51 kB` raw / `2.19 kB` gzip; original hero art `107.87 kB`.

## Run and deploy

```sh
npm ci
npm test
npm run lint
npm run build
cargo build --release
/opt/fleet/lib/deploy-static.sh migration-lock-rehearsal dist/site
```

The CLI is ready for registry review with `cargo package`; do not publish it from this repository.

## Known gaps

No known product gaps. This worker cannot run a nested container image because its kernel denies `unshare`; the real Docker claims are fail-required on the included GitHub Actions Ubuntu runner rather than treated as passing locally.
