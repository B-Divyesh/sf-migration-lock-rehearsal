# Demo sandbox

Open `/demo` or run `mlr demo --output ./mlr-demo`. The site uses no storage and makes no third-party requests. The CLI demo copies only the bundled, sanitized fixture and writes its report below the output directory.

`mlr demo` starts a disposable Docker database on loopback. `mlr demo --dry-run` writes the same sample go/no-go card without Docker, for a quick documentation preview. `mlr demo --reset` removes only the demo output directory supplied with `--output`.

Sample data: `examples/postgres/fixture.sql`, `examples/postgres/add_customer_flag.sql`, `examples/postgres/rollback_customer_flag.sql`, and `examples/postgres/read_workload.sql`. It contains invented account names and no production connection details.
