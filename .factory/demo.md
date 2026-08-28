# Demo sandbox

Open `/demo` or run `mlr demo --output ./mlr-demo`. The site uses no storage and makes only same-origin requests. The CLI demo copies only the bundled, invented fixture and writes its report below the output directory.

`mlr demo` starts a disposable Docker database. `mlr demo --dry-run` writes the same sample go/no-go card without Docker. `mlr demo --output ./mlr-demo --reset` removes only a marked directory created by an earlier demo. Reset refuses unmarked directories, roots, workspaces, home/current directories, aliases, and symlinks.

The sample card applies the shipped limits: 30,000 ms statement time, 1,000 ms lock wait, and 104,857,600 bytes table growth. A failed workload, measurement, migration, or rollback writes `NO-GO`, records the failed stage, and exits non-zero. The JSON uses `null` when a measurement could not complete.

Sample data: `examples/postgres/fixture.sql`, `examples/postgres/add_customer_flag.sql`, `examples/postgres/rollback_customer_flag.sql`, and `examples/postgres/read_workload.sql`. It contains invented account names and no production connection details.
