# Changelog

## Unreleased

- Make workload, measurement, migration, rollback, and exceeded-limit outcomes write NO-GO and exit non-zero.
- Add configurable statement, lock-wait, and table-growth limits to reports and runbooks.
- Serialize JSON safely for every valid filename and write recovery guidance for failed runs.
- Add the Sociobot one-time license flow and operator review checklist.
- Fix section navigation, route metadata, and demo live-region semantics.

## 0.1.0 — 2026-08-28

- Added Postgres and ClickHouse migration rehearsals in disposable Docker containers.
- Added concurrent workload and lock-wait measurement for both engines.
- Added JSON and Markdown go/no-go reports with rollback failure handling.
- Added a one-command sample demo and guarded demo reset.
- Added the static documentation and browser demo site.
