# Polish 1 — review finding closure

Candidate repaired from `501229c57c286192d870877186ac6825b52fd7d4`; review baseline `4c852068a60a7a2965cc29006a739c350ba74772`.

| Finding | Change made | Evidence |
|---|---|---|
| F-1-1 | Added the GitHub source link, tested `cargo install --git … --locked` command, and a complete first `mlr rehearse` command. | Cold live home screenshot `evidence/polish-1/live-mobile-home.png`; landing source/link check. |
| F-1-2 | Replaced the static terminal with a self-hosted release-CLI dry-run recording. Reset restarts it observably; `?demo=1` is a direct isolated path with banner. | `@claim:browser-demo-reset`, `@claim:demo-recording`; live `live-mobile-demo-reset.png`. |
| F-1-3 | Added Docker-required GitHub Action integration claims for bundled Postgres 16 and ClickHouse 24.8 samples; created `/work` before copying SQL. | `.github/workflows/docker-claims.yml`; `@claim:docker-rehearsal`, `@claim:container-cleanup`; local command-double coverage and clean-clone run. |
| F-1-4 | Rewrote the first-screen audience sentence to lock waits, table growth, and rollback results. | Live home screenshot; `threshold-verdict` and `rollback-no-go` claims. |
| F-1-5 | Kept the required $29 one-time disclosure and made it a paid-license assertion against the hosted checkout HTML. | `@claim:paid-license` asserts `$29.00` and the one-time product text. |
| F-1-6 | Removed untestable merchant-of-record, refund, and revocation language from landing, README, and terms. | Copy audit; live `/terms` check in `verify:url`. |
| F-1-7 | Added an explicit free CLI claim and offline/no-license test for demo, guard, and rehearsal validation. | `@claim:free-cli`. |
| F-1-8 | Extended the limits claim to exact defaults plus configured values in CLI help, JSON report, and runbook. | `@claim:threshold-verdict`. |
| F-1-9 | Replaced “reusable” with the precise browser checklist behavior. | `@claim:paid-license`; live home screenshot. |
| F-1-10 | Unified standalone 404 canonical and OG URL as `/404` and added apple-touch icon metadata. | Route metadata test; live `live-mobile-404.png`; live curl check. |
| F-1-11 | Replaced “Start for real” with “Install the CLI” and linked it to the install section. | `@claim:browser-demo-reset`; live demo screenshot. |
| F-1-12 | Replaced the metaphor eyebrow with `POSTGRES + CLICKHOUSE / v0.1.0`. | Live home screenshot; copy audit. |
| F-1-13 | Replaced the artwork slogan with the measurable limits caption. | Live home screenshot; copy audit. |
| F-1-14 | Replaced `THREE MOVES` with `HOW IT WORKS`. | Live home screenshot; copy audit. |
| F-1-15 | Rewrote README’s subjective sample wording as measured report output. | README and `.factory/copy-audit.md`. |
| F-1-16 | Split the long README Docker behavior sentence into three plain sentences. | README; copy audit. |
| F-1-17 | Split the long README engine behavior sentence into two plain sentences. | README; copy audit. |
| F-1-18 | Standardized `report.json` and browser output as “go/no-go report”; reserved “runbook” for `runbook.md`. | README, CLI output, demo, and copy-audit terminology table. |

Live URL recheck: `https://migration-lock-rehearsal.sociobot.in`, `https://migration-lock-rehearsal.sociobot.in/?demo=1`, and an unknown route were cold-opened after deployment. The local live verifier command was `npm run verify:url -- https://migration-lock-rehearsal.sociobot.in`.
