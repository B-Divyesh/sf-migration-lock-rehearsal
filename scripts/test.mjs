import { spawnSync } from 'node:child_process'

function run(command, args) {
  const result = spawnSync(command, args, { stdio: 'inherit' })
  if (result.error) throw result.error
  if (result.status !== 0) process.exit(result.status ?? 1)
}

run('cargo', ['test'])
run(process.execPath, ['--test', ...process.argv.slice(2), 'tests/claims.test.mjs'])
