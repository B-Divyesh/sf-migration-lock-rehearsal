# Demo sandbox

Open `/?demo=1` or `/demo`, or run `mlr demo --output ./mlr-demo`. The site demo uses no storage and makes only same-origin requests. Reset restarts the self-hosted terminal recording and restores the first line of its bundled sample. The CLI demo copies only the bundled, invented fixture and writes its report below the output directory.

The browser recording reads `public/demo-recording.json`. It is a checked-in transcript of `mlr demo --dry-run --output ./mlr-demo` from the bundled release binary. Demo mode has no storage namespace because it does not write browser data.

`mlr demo` starts a disposable Docker database. `mlr demo --dry-run` writes the same sample go/no-go report and runbook without Docker. `mlr demo --output ./mlr-demo --reset` removes only a marked directory created by an earlier demo. Reset refuses unmarked directories, roots, workspaces, home/current directories, aliases, and symlinks.

The sample report applies the shipped limits: 30,000 ms statement time, 1,000 ms lock wait, and 104,857,600 bytes table growth. A failed Docker command at any rehearsal stage writes `NO-GO`, records the failed stage, and exits non-zero. The JSON uses `null` when a measurement could not complete.

Sample data: `examples/postgres/fixture.sql`, `examples/postgres/add_customer_flag.sql`, `examples/postgres/rollback_customer_flag.sql`, and `examples/postgres/read_workload.sql`. It contains invented account names and no production connection details.
