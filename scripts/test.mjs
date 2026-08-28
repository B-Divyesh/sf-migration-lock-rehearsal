import { spawnSync } from 'node:child_process'
import { join } from 'node:path'

const env = { ...process.env, CARGO_TARGET_DIR: join(process.cwd(), 'target', 'test-suite') }

function run(command, args) {
  const result = spawnSync(command, args, { stdio: 'inherit', env })
  if (result.error) throw result.error
  if (result.status !== 0) process.exit(result.status ?? 1)
}

run('cargo', ['test'])
run(process.execPath, ['--test', ...process.argv.slice(2), 'tests/claims.test.mjs'])
