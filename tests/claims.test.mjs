import assert from 'node:assert/strict'
import { execFileSync, spawn, spawnSync } from 'node:child_process'
import { once } from 'node:events'
import { chmodSync, existsSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import test from 'node:test'
import AxeBuilder from '@axe-core/playwright'
import { chromium } from 'playwright'

const root = process.cwd()
const cargoTarget = process.env.CARGO_TARGET_DIR ?? join(root, 'target', 'test-suite')
execFileSync('cargo', ['build', '--quiet'], { cwd: root, stdio: 'pipe', env: { ...process.env, CARGO_TARGET_DIR: cargoTarget } })
const cli = join(cargoTarget, 'debug', 'mlr')

function runCli(args, options = {}) {
  return spawnSync(cli, args, {
    cwd: options.cwd ?? root,
    env: options.env ?? process.env,
    encoding: 'utf8',
  })
}

function assertSuccess(result) {
  assert.equal(result.status, 0, `${result.stdout}\n${result.stderr}`)
}

function makeFakeDocker({ failMigration = false, failRollback = false, failWorkload = false, delayWorkloadFailure = false, failMeasurement = false, highLock = false, highTable = false } = {}) {
  const sandbox = mkdtempSync(join(tmpdir(), 'mlr-docker-'))
  const bin = join(sandbox, 'bin')
  const state = join(sandbox, 'state')
  execFileSync('mkdir', ['-p', bin, state])
  const docker = join(bin, 'docker')
  writeFileSync(docker, `#!/bin/sh
set -eu
printf '%s\\n' "$*" >> "$MLR_FAKE_LOG"
if test "\${1:-}" = cp; then
  case "\${2:-}" in
    *fixture.sql) grep -q '@example.test' "$2" && touch "$MLR_FAKE_STATE/invented-fixture" ;;
  esac
  exit 0
fi
test "\${1:-}" = exec || exit 0
case "$*" in
  *"ProfileEvents"*)
    test "\${MLR_FAIL_MEASUREMENT:-0}" = 1 && exit 23
    test "\${MLR_HIGH_LOCK:-0}" = 1 && printf '900000\\n' || printf '37\\n'
    exit 0 ;;
  *"pg_stat_activity"*)
    test "\${MLR_FAIL_MEASUREMENT:-0}" = 1 && exit 23
    test "\${MLR_HIGH_LOCK:-0}" = 1 && printf '900000\\n' || printf '31\\n'
    exit 0 ;;
  *"pg_total_relation_size"*)
    test "\${MLR_FAIL_MEASUREMENT:-0}" = 1 && exit 23
    counter="$MLR_FAKE_STATE/pg-measurements"; count=0; test ! -f "$counter" || count=$(cat "$counter"); count=$((count + 1)); printf '%s' "$count" > "$counter"
    if test "\${MLR_HIGH_TABLE:-0}" = 1 && test "$count" -gt 1; then printf '999999999999\\n'; elif test "$count" -gt 1; then printf '40960\\n'; else printf '32768\\n'; fi
    exit 0 ;;
  *"bytes_on_disk"*)
    test "\${MLR_FAIL_MEASUREMENT:-0}" = 1 && exit 23
    counter="$MLR_FAKE_STATE/ch-measurements"; count=0; test ! -f "$counter" || count=$(cat "$counter"); count=$((count + 1)); printf '%s' "$count" > "$counter"
    if test "\${MLR_HIGH_TABLE:-0}" = 1 && test "$count" -gt 1; then printf '999999999999\\n'; elif test "$count" -gt 1; then printf '49152\\n'; else printf '40960\\n'; fi
    exit 0 ;;
  *"/work/workload.sql"*)
    touch "$MLR_FAKE_STATE/workload-running"
    if test "\${MLR_FAIL_WORKLOAD:-0}" = 1; then test "\${MLR_DELAY_WORKLOAD_FAILURE:-0}" = 0 || sleep 0.35; exit 19; fi
    sleep 1; exit 0 ;;
  *"/work/migration.sql"*)
    sleep 0.05
    test -f "$MLR_FAKE_STATE/workload-running" && touch "$MLR_FAKE_STATE/overlap"
    sleep 0.12
    test "\${MLR_FAIL_MIGRATION:-0}" = 1 && exit 1
    exit 0 ;;
  *"/work/rollback.sql"*) test "\${MLR_FAIL_ROLLBACK:-0}" = 1 && exit 1; exit 0 ;;
esac
exit 0
`)
  chmodSync(docker, 0o755)
  return {
    sandbox,
    state,
    log: join(state, 'docker.log'),
    env: {
      ...process.env,
      PATH: `${bin}:${process.env.PATH}`,
      MLR_FAKE_LOG: join(state, 'docker.log'),
      MLR_FAKE_STATE: state,
      MLR_FAIL_MIGRATION: failMigration ? '1' : '0',
      MLR_FAIL_ROLLBACK: failRollback ? '1' : '0',
      MLR_FAIL_WORKLOAD: failWorkload ? '1' : '0',
      MLR_DELAY_WORKLOAD_FAILURE: delayWorkloadFailure ? '1' : '0',
      MLR_FAIL_MEASUREMENT: failMeasurement ? '1' : '0',
      MLR_HIGH_LOCK: highLock ? '1' : '0',
      MLR_HIGH_TABLE: highTable ? '1' : '0',
    },
  }
}

test('@claim:demo-report bundled demo writes a go/no-go runbook', () => {
  const parent = mkdtempSync(join(tmpdir(), 'mlr-claim-'))
  const out = join(parent, 'mlr-demo')
  try {
    const result = runCli(['demo', '--dry-run', '--output', out])
    assertSuccess(result)
    assert.ok(existsSync(join(out, 'report.json')))
    const runbook = readFileSync(join(out, 'runbook.md'), 'utf8')
    const report = JSON.parse(readFileSync(join(out, 'report.json'), 'utf8'))
    assert.match(runbook, /Verdict: GO/)
    assert.match(runbook, /Rollback checked: true/)
    assert.deepEqual({ duration: report.duration_ms, lock: report.max_lock_wait_ms, before: report.table_bytes_before, after: report.table_bytes_after }, { duration: 184, lock: 0, before: 32768, after: 40960 })
  } finally { rmSync(parent, { recursive: true, force: true }) }
})

test('@claim:local-only only exact loopback database hosts pass the guard', () => {
  const refused = [
    'postgres://ops@localhost.prod.example.com/app',
    'postgres://disposable@production.example.com/app',
    'postgres://admin@db.internal.example.test/app',
    'postgres://prod.example.com/app?next=localhost',
    'postgres://localhost@production.example.com/app',
    'postgres://127.0.0.1.example.com/app',
  ]
  for (const url of refused) {
    const result = runCli(['guard', url])
    assert.notEqual(result.status, 0, url)
    assert.match(result.stderr, /parsed host is localhost or a loopback IP/)
  }
  for (const url of ['postgres://ops@localhost/app', 'postgres://ops@127.0.0.1:5432/app', 'clickhouse://default@[::1]:9000/default']) {
    assertSuccess(runCli(['guard', url]))
  }
  const noTargetOption = runCli(['rehearse', '--database-url', 'postgres://localhost/app'])
  assert.notEqual(noTargetOption.status, 0)
  assert.match(noTargetOption.stderr, /unknown option --database-url/)
})

async function withSite(run) {
  execFileSync('npm', ['run', 'build:site'], { cwd: root, stdio: 'pipe' })
  const child = spawn(process.execPath, [join(root, 'node_modules/vite/bin/vite.js'), 'preview', '--host', '127.0.0.1', '--port', '4173', '--outDir', 'dist/site'], { cwd: root, stdio: 'pipe' })
  try {
    let ready = false
    for (let attempt = 0; attempt < 40; attempt += 1) {
      try {
        if ((await fetch('http://127.0.0.1:4173/')).ok) { ready = true; break }
      } catch {}
      await new Promise(resolve => setTimeout(resolve, 100))
    }
    assert.ok(ready, 'site preview did not start')
    await run('http://127.0.0.1:4173')
  } finally {
    if (child.exitCode === null) {
      child.kill('SIGTERM')
      await once(child, 'exit')
    }
  }
}

test('@claim:site-private static site stays same-origin and stores no visitor data', async () => {
  await withSite(async origin => {
    const browser = await chromium.launch({ headless: true })
    try {
      for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
        const context = await browser.newContext({ viewport })
        const page = await context.newPage()
        const requests = []
        const errors = []
        page.on('request', request => requests.push(request.url()))
        page.on('console', message => { if (message.type() === 'error') errors.push(message.text()) })
        page.on('pageerror', error => errors.push(error.message))
        for (const path of ['/', '/demo', '/privacy', '/terms']) {
          await page.goto(origin + path, { waitUntil: 'networkidle' })
          assert.equal(await page.locator('h1').count(), 1, path)
          assert.equal(await page.locator('main').count(), 1, path)
          assert.ok((await page.title()).includes('Migration Lock Rehearsal'))
          assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth))
          const axe = await new AxeBuilder({ page }).analyze()
          assert.deepEqual(axe.violations.map(item => item.id), [])
        }
        assert.ok(requests.length > 0)
        assert.ok(requests.every(url => new URL(url).origin === origin), requests.join('\n'))
        assert.deepEqual(await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length, cookies: document.cookie })), { local: 0, session: 0, cookies: '' })
        assert.deepEqual(errors, [])
        if (viewport.width === 390) {
          await page.goto(origin + '/')
          for (const selector of ['.wordmark', 'nav a', 'footer a']) {
            for (const box of await page.locator(selector).evaluateAll(nodes => nodes.map(node => {
              const rect = node.getBoundingClientRect()
              return { width: rect.width, height: rect.height }
            }))) {
              assert.ok(box.width >= 44, `${selector} width ${box.width}`)
              assert.ok(box.height >= 44, `${selector} height ${box.height}`)
            }
          }
        }
        await context.close()
      }

      const context = await browser.newContext()
      const page = await context.newPage()
      await page.goto(origin + '/')
      await page.keyboard.press('Tab')
      assert.equal(await page.evaluate(() => document.activeElement?.getAttribute('href')), '#main')
      await page.keyboard.press('Enter')
      await page.waitForFunction(() => document.activeElement?.id === 'main')
      assert.equal(await page.evaluate(() => document.activeElement?.id), 'main')
      await page.getByRole('link', { name: 'Try it with sample data' }).click()
      await page.waitForFunction(() => document.activeElement?.tagName === 'H1')
      assert.equal(await page.evaluate(() => document.activeElement?.tagName), 'H1')
      await page.goto(origin + '/demo')
      await page.locator('#reset-demo').focus()
      await page.keyboard.press('Enter')
      await page.getByRole('button', { name: 'Demo reset' }).waitFor()
      await page.emulateMedia({ reducedMotion: 'reduce' })
      const motion = await page.locator('.cursor').evaluate(node => getComputedStyle(node).animationDuration)
      assert.equal(motion, '1e-05s')
      await context.close()
    } finally { await browser.close() }
  })

  const policy = JSON.parse(readFileSync(join(root, 'public', 'staticwebapp.config.json'), 'utf8'))
  assert.equal(policy.responseOverrides['404'].statusCode, 404)
  assert.match(policy.globalHeaders['Content-Security-Policy'], /connect-src 'self'/)
  assert.equal(policy.globalHeaders['X-Content-Type-Options'], 'nosniff')
})

test('route metadata, section links, and ARIA remain valid at desktop and mobile widths', async () => {
  await withSite(async origin => {
    const routes = {
      '/': ['Migration Lock Rehearsal — Test database changes', 'https://migration-lock-rehearsal.sociobot.in/'],
      '/demo': ['Demo — Migration Lock Rehearsal', 'https://migration-lock-rehearsal.sociobot.in/demo'],
      '/privacy': ['Privacy — Migration Lock Rehearsal', 'https://migration-lock-rehearsal.sociobot.in/privacy'],
      '/terms': ['Terms — Migration Lock Rehearsal', 'https://migration-lock-rehearsal.sociobot.in/terms'],
    }
    for (const [path, [title, canonical]] of Object.entries(routes)) {
      const documentPath = path === '/' ? '/' : `${path}/index.html`
      const html = await (await fetch(origin + documentPath)).text()
      assert.match(html, new RegExp(`<title>${title}</title>`))
      assert.match(html, new RegExp(`rel="canonical" href="${canonical.replaceAll('/', '\\/')}"`))
      assert.match(html, new RegExp(`property="og:url" content="${canonical.replaceAll('/', '\\/')}"`))
    }
    const policy = JSON.parse(readFileSync(join(root, 'public', 'staticwebapp.config.json'), 'utf8'))
    for (const path of ['/demo', '/privacy', '/terms']) {
      assert.equal(policy.routes.find(route => route.route === path)?.rewrite, `${path}/index.html`)
    }

    const browser = await chromium.launch({ headless: true })
    try {
      for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
        const context = await browser.newContext({ viewport })
        const page = await context.newPage()
        await page.goto(origin + '/')
        await page.getByRole('link', { name: 'How it works' }).click()
        assert.equal(new URL(page.url()).hash, '#how')
        assert.equal(await page.evaluate(() => document.activeElement?.id), 'how')
        await page.waitForFunction(() => scrollY > 100)
        assert.ok(await page.evaluate(() => scrollY > 100), `section link did not scroll at ${viewport.width}px`)
        await page.goto(origin + '/demo')
        const axe = await new AxeBuilder({ page }).analyze()
        assert.ok(!axe.violations.some(item => item.id === 'aria-allowed-role'), JSON.stringify(axe.violations))
        assert.equal(await page.locator('aside[role="status"]').count(), 0)
        await context.close()
      }
    } finally { await browser.close() }
  })
})

test('@claim:paid-license returned and restored Sociobot licenses verify, cache, reveal, and remove the paid checklist', async () => {
  await withSite(async origin => {
    const browser = await chromium.launch({ headless: true })
    try {
      const context = await browser.newContext()
      const page = await context.newPage()
      let verifyRequests = 0
      await page.route('https://api.sociobot.in/**', async route => {
        verifyRequests += 1
        await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null }) })
      })
      await page.goto(origin + '/?license=returned-token')
      await page.getByText('License active.', { exact: true }).waitFor()
      assert.equal(page.url(), origin + '/')
      assert.equal(await page.evaluate(() => localStorage.getItem('sb_license:migration-lock-rehearsal')), 'returned-token')
      await page.getByRole('heading', { name: 'Operator review checklist', exact: true }).waitFor()
      assert.equal(verifyRequests, 1)

      await page.reload()
      await page.getByText('License active.', { exact: true }).waitFor()
      assert.equal(verifyRequests, 1, 'a fresh cached verdict should not verify twice in one day')
      assert.equal(await page.getByRole('link', { name: 'Buy operator license — $29' }).getAttribute('href'), 'https://api.sociobot.in/api/v1/products/migration-lock-rehearsal/checkout')

      await page.getByRole('button', { name: 'Remove saved license' }).click()
      assert.deepEqual(await page.evaluate(() => ({ license: localStorage.getItem('sb_license:migration-lock-rehearsal'), cache: localStorage.getItem('sb_license:migration-lock-rehearsal:verification') })), { license: null, cache: null })
      await page.getByLabel('Have a license? Paste it.').fill('restored-token')
      await page.getByRole('button', { name: 'Restore license' }).click()
      await page.getByText('License active.', { exact: true }).waitFor()
      assert.equal(verifyRequests, 2)
      assert.equal(await page.evaluate(() => localStorage.getItem('sb_license:migration-lock-rehearsal')), 'restored-token')
      await context.close()
    } finally { await browser.close() }
  })
})

test('@claim:supported-engines dry-run cards accept only Postgres and ClickHouse', () => {
  const out = mkdtempSync(join(tmpdir(), 'mlr-engines-'))
  try {
    for (const engine of ['postgres', 'clickhouse']) {
      const engineOut = join(out, `mlr-demo-${engine}`)
      assertSuccess(runCli(['demo', '--engine', engine, '--dry-run', '--output', engineOut]))
      assert.match(readFileSync(join(engineOut, 'report.json'), 'utf8'), new RegExp(`"engine": "${engine}"`))
    }
    const mysql = runCli(['demo', '--engine', 'mysql', '--dry-run', '--output', join(out, 'mysql')])
    assert.notEqual(mysql.status, 0)
    assert.match(mysql.stderr, /unknown engine mysql/)
    assert.ok(!existsSync(join(out, 'mysql')))
  } finally { rmSync(out, { recursive: true, force: true }) }
})

test('@claim:demo-reset reset deletes only a validated mlr demo directory', () => {
  const parent = mkdtempSync(join(tmpdir(), 'mlr-reset-'))
  const output = join(parent, 'mlr-demo')
  try {
    assertSuccess(runCli(['demo', '--dry-run', '--output', output]))
    const marker = readFileSync(join(output, '.mlr-demo'))
    assertSuccess(runCli(['demo', '--output', output, '--reset']))
    assert.ok(!existsSync(output))
    assert.ok(existsSync(parent))

    const unmarked = join(parent, 'unmarked')
    execFileSync('mkdir', ['-p', unmarked])
    writeFileSync(join(unmarked, 'keep.txt'), 'keep')
    assert.notEqual(runCli(['demo', '--output', unmarked, '--reset']).status, 0)
    assert.equal(readFileSync(join(unmarked, 'keep.txt'), 'utf8'), 'keep')

    const workspace = join(parent, 'workspace')
    execFileSync('mkdir', ['-p', workspace])
    writeFileSync(join(workspace, '.mlr-demo'), marker)
    writeFileSync(join(workspace, 'package.json'), '{}')
    assert.notEqual(runCli(['demo', '--output', '.', '--reset'], { cwd: workspace }).status, 0)
    assert.ok(existsSync(join(workspace, 'package.json')))

    const home = join(parent, 'home')
    const elsewhere = join(parent, 'elsewhere')
    execFileSync('mkdir', ['-p', home, elsewhere])
    writeFileSync(join(home, '.mlr-demo'), marker)
    const homeResult = runCli(['demo', '--output', home, '--reset'], { cwd: elsewhere, env: { ...process.env, HOME: home } })
    assert.notEqual(homeResult.status, 0)
    assert.ok(existsSync(home))

    const real = join(parent, 'real-demo')
    const alias = join(parent, 'alias-demo')
    execFileSync('mkdir', ['-p', real])
    writeFileSync(join(real, '.mlr-demo'), marker)
    symlinkSync(real, alias, 'dir')
    assert.notEqual(runCli(['demo', '--output', alias, '--reset']).status, 0)
    assert.ok(existsSync(real))
  } finally { rmSync(parent, { recursive: true, force: true }) }
})

test('@claim:invented-sample bundled fixtures contain invented records and no connection URL', () => {
  for (const engine of ['postgres', 'clickhouse']) {
    const fixture = readFileSync(join(root, 'examples', engine, 'fixture.sql'), 'utf8')
    assert.match(fixture, /aria@example\.test|fatima@example\.test/i)
    assert.doesNotMatch(fixture, /(?:postgres|mysql|clickhouse):\/\//i)
  }
  const postgresFixture = readFileSync(join(root, 'examples', 'postgres', 'fixture.sql'), 'utf8')
  assert.equal((postgresFixture.match(/@example\.test/g) ?? []).length, 6)
})

test('@claim:chosen-output reports stay in a named non-blank output directory', () => {
  const parent = mkdtempSync(join(tmpdir(), 'mlr-output-'))
  const output = join(parent, 'chosen-folder')
  try {
    assertSuccess(runCli(['demo', '--dry-run', '--output', output], { cwd: parent }))
    assert.ok(existsSync(join(output, 'report.json')))
    assert.ok(existsSync(join(output, 'runbook.md')))
    assert.ok(!existsSync(join(parent, 'report.json')))
    assert.ok(!existsSync(join(parent, 'runbook.md')))

    const blank = runCli(['demo', '--dry-run', '--output', ''], { cwd: parent })
    assert.notEqual(blank.status, 0)
    assert.match(blank.stderr, /non-empty directory/)
    assert.ok(!existsSync(join(parent, 'report.json')))
  } finally { rmSync(parent, { recursive: true, force: true }) }
})

test('@claim:docker-rehearsal supplied SQL and workload produce measured cards for both engines', () => {
  for (const engine of ['postgres', 'clickhouse']) {
    const fake = makeFakeDocker()
    const output = join(fake.sandbox, `mlr-demo-${engine}`)
    try {
      const result = runCli(['demo', '--engine', engine, '--output', output], { cwd: fake.sandbox, env: fake.env })
      assertSuccess(result)
      const log = readFileSync(fake.log, 'utf8')
      assert.match(log, engine === 'postgres' ? /postgres:16-alpine/ : /clickhouse-server:24\.8-alpine/)
      for (const file of ['fixture.sql', 'migration.sql', 'workload.sql', 'rollback.sql']) assert.match(log, new RegExp(file))
      assert.ok(existsSync(join(fake.state, 'invented-fixture')))
      assert.ok(existsSync(join(fake.state, 'overlap')), `${engine} workload did not overlap migration`)
      const report = JSON.parse(readFileSync(join(output, 'report.json'), 'utf8'))
      assert.ok(report.duration_ms >= 100)
      assert.ok(report.max_lock_wait_ms > 0)
      assert.equal(report.table_bytes_before > 0, true)
      assert.equal(report.table_bytes_after > 0, true)
      assert.equal(report.rollback_checked, true)
      assert.equal(report.verdict, 'GO')
    } finally { rmSync(fake.sandbox, { recursive: true, force: true }) }
  }
})

test('@claim:failed-command-no-go failed workload, measurement, and migration commands write NO-GO artifacts', () => {
  for (const engine of ['postgres', 'clickhouse']) {
    for (const scenario of [
      { name: 'workload', stage: 'workload', options: { failWorkload: true } },
      { name: 'workload-delayed', stage: 'workload', options: { failWorkload: true, delayWorkloadFailure: true } },
      { name: 'measurement', stage: 'measurement', options: { failMeasurement: true } },
      { name: 'migration', stage: 'migration', options: { failMigration: true } },
    ]) {
      const fake = makeFakeDocker(scenario.options)
      const output = join(fake.sandbox, `${engine}-${scenario.name}`)
      try {
        const result = runCli(['demo', '--engine', engine, '--output', output, '--json'], { cwd: fake.sandbox, env: fake.env })
        assert.notEqual(result.status, 0, `${engine} ${scenario.name} unexpectedly succeeded\n${result.stdout}\n${result.stderr}\n${readFileSync(fake.log, 'utf8')}`)
        const fileReport = JSON.parse(readFileSync(join(output, 'report.json'), 'utf8'))
        const stdoutReport = JSON.parse(result.stdout)
        assert.equal(fileReport.verdict, 'NO-GO')
        assert.equal(fileReport.failure_stage, scenario.stage)
        if (scenario.stage === 'workload') assert.match(fileReport.failure, /exit 19/)
        assert.deepEqual(stdoutReport, fileReport)
        assert.match(readFileSync(join(output, 'runbook.md'), 'utf8'), new RegExp(`Failed stage[\\s\\S]*Stage: ${scenario.stage}[\\s\\S]*Recovery:`))
        if (scenario.stage === 'measurement') {
          assert.equal(fileReport.table_bytes_before, null)
          assert.doesNotMatch(fileReport.failure, /recorded.*zero/i)
        }
      } finally { rmSync(fake.sandbox, { recursive: true, force: true }) }
    }
  }
})

test('@claim:threshold-verdict statement, lock, and table limits determine GO or NO-GO', () => {
  for (const engine of ['postgres', 'clickhouse']) {
    for (const scenario of [
      { option: '--max-statement-ms', value: '1', reason: /Statement time exceeded/ },
      { option: '--max-lock-wait-ms', value: '30', reason: /Lock wait exceeded/ },
      { option: '--max-table-growth-bytes', value: '4096', reason: /Table growth exceeded/ },
    ]) {
      const fake = makeFakeDocker()
      const output = join(fake.sandbox, `${engine}-${scenario.option.slice(2)}`)
      try {
        const result = runCli(['demo', '--engine', engine, '--output', output, scenario.option, scenario.value], { cwd: fake.sandbox, env: fake.env })
        assert.notEqual(result.status, 0)
        const report = JSON.parse(readFileSync(join(output, 'report.json'), 'utf8'))
        assert.equal(report.verdict, 'NO-GO')
        assert.match(report.decision_reasons.join(' '), scenario.reason)
        assert.equal(report.thresholds[scenario.option.slice(2).replaceAll('-', '_')], Number(scenario.value))
        assert.match(readFileSync(join(output, 'runbook.md'), 'utf8'), /Decision limits[\s\S]*Statement time:[\s\S]*Lock wait:[\s\S]*Table growth:/)
      } finally { rmSync(fake.sandbox, { recursive: true, force: true }) }
    }

    const extreme = makeFakeDocker({ highLock: true, highTable: true })
    const extremeOutput = join(extreme.sandbox, `${engine}-extreme`)
    try {
      const result = runCli(['demo', '--engine', engine, '--output', extremeOutput], { cwd: extreme.sandbox, env: extreme.env })
      assert.notEqual(result.status, 0)
      const report = JSON.parse(readFileSync(join(extremeOutput, 'report.json'), 'utf8'))
      assert.equal(report.max_lock_wait_ms, 900000)
      assert.equal(report.table_bytes_after, 999999999999)
      assert.equal(report.verdict, 'NO-GO')
      assert.match(report.decision_reasons.join(' '), /Lock wait exceeded/)
      assert.match(report.decision_reasons.join(' '), /Table growth exceeded/)
    } finally { rmSync(extreme.sandbox, { recursive: true, force: true }) }
  }
})

test('@claim:safe-json control characters in valid migration filenames stay valid JSON', () => {
  const fake = makeFakeDocker()
  const migration = join(fake.sandbox, 'change\nline.sql')
  const output = join(fake.sandbox, 'safe-json')
  try {
    writeFileSync(migration, 'ALTER TABLE customers ADD COLUMN safe_json UInt8 DEFAULT 0;\n')
    const result = runCli([
      'rehearse', '--engine', 'clickhouse',
      '--fixture', join(root, 'examples/clickhouse/fixture.sql'),
      '--migration', migration,
      '--rollback', join(root, 'examples/clickhouse/rollback_customer_flag.sql'),
      '--workload', join(root, 'examples/clickhouse/read_workload.sql'),
      '--output', output,
      '--json',
    ], { env: fake.env })
    assertSuccess(result)
    assert.equal(JSON.parse(result.stdout).migration, migration)
    assert.equal(JSON.parse(readFileSync(join(output, 'report.json'), 'utf8')).migration, migration)
  } finally { rmSync(fake.sandbox, { recursive: true, force: true }) }
})

test('@claim:container-cleanup disposable containers are removed after success and failure', () => {
  for (const failMigration of [false, true]) {
    const fake = makeFakeDocker({ failMigration })
    try {
      const output = join(fake.sandbox, 'mlr-demo-postgres')
      const result = runCli(['demo', '--output', output], { cwd: fake.sandbox, env: fake.env })
      assert.equal(result.status === 0, !failMigration, `${result.stdout}\n${result.stderr}`)
      assert.match(readFileSync(fake.log, 'utf8'), /rm -f mlr-[0-9]+/)
    } finally { rmSync(fake.sandbox, { recursive: true, force: true }) }
  }
})

test('@claim:rollback-no-go a failed rollback is NO-GO and exits non-zero for both engines', () => {
  for (const engine of ['postgres', 'clickhouse']) {
    const fake = makeFakeDocker({ failRollback: true })
    const output = join(fake.sandbox, `mlr-demo-${engine}`)
    try {
      const result = runCli(['demo', '--engine', engine, '--output', output], { cwd: fake.sandbox, env: fake.env })
      assert.notEqual(result.status, 0)
      assert.match(result.stderr, /rollback failed/)
      const report = JSON.parse(readFileSync(join(output, 'report.json'), 'utf8'))
      assert.equal(report.rollback_checked, false)
      assert.equal(report.verdict, 'NO-GO')
      assert.match(readFileSync(join(output, 'runbook.md'), 'utf8'), /Verdict: NO-GO/)
    } finally { rmSync(fake.sandbox, { recursive: true, force: true }) }

    const missing = makeFakeDocker()
    const missingOutput = join(missing.sandbox, `missing-rollback-${engine}`)
    try {
      const example = join(root, 'examples', engine)
      const result = runCli([
        'rehearse', '--engine', engine,
        '--fixture', join(example, 'fixture.sql'),
        '--migration', join(example, 'add_customer_flag.sql'),
        '--output', missingOutput,
      ], { env: missing.env })
      assert.notEqual(result.status, 0)
      assert.equal(JSON.parse(readFileSync(join(missingOutput, 'report.json'), 'utf8')).verdict, 'NO-GO')
    } finally { rmSync(missing.sandbox, { recursive: true, force: true }) }
  }
})
