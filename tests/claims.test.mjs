import assert from 'node:assert/strict'
import { execFileSync, spawn } from 'node:child_process'
import { once } from 'node:events'
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import test from 'node:test'
import { chromium } from 'playwright'

const root = process.cwd()

test('@claim:demo-report bundled demo writes a go/no-go runbook', () => {
  const out = mkdtempSync(join(tmpdir(), 'mlr-claim-'))
  try {
    execFileSync('cargo', ['run', '--quiet', '--', 'demo', '--dry-run', '--output', out], { cwd: root, stdio: 'pipe' })
    assert.ok(existsSync(join(out, 'report.json')))
    const runbook = readFileSync(join(out, 'runbook.md'), 'utf8')
    assert.match(runbook, /Verdict: GO/)
    assert.match(runbook, /Rollback checked: true/)
  } finally { rmSync(out, { recursive: true, force: true }) }
})

test('@claim:local-only remote target is refused', () => {
  let output = ''
  try { execFileSync('cargo', ['run', '--quiet', '--', 'guard', 'postgres://admin@production.example.com/app'], { cwd: root, stdio: 'pipe' }) } catch (error) { output = error.stderr.toString() }
  assert.match(output, /only loopback or explicitly disposable URLs are allowed/)
})

async function withSite(run) {
  execFileSync('npm', ['run', 'build:site'], { cwd: root, stdio: 'pipe' })
  const child = spawn(process.execPath, [join(root, 'node_modules/vite/bin/vite.js'), 'preview', '--host', '127.0.0.1', '--port', '4173', '--outDir', 'dist/site'], { cwd: root, stdio: 'pipe' })
  try {
    for (let attempt = 0; attempt < 40; attempt += 1) {
      try { if ((await fetch('http://127.0.0.1:4173/')).ok) break } catch {}
      await new Promise(resolve => setTimeout(resolve, 100))
    }
    await run('http://127.0.0.1:4173')
  } finally {
    if (child.exitCode === null) {
      child.kill('SIGTERM')
      await once(child, 'exit')
    }
  }
}

test('@claim:site-no-third-party and @regression:mobile-nav-targets use only local assets and 44px targets', async () => {
  await withSite(async origin => {
    const browser = await chromium.launch({ headless: true })
    try {
      const page = await browser.newPage({ viewport: { width: 390, height: 844 } })
      const requests = []
      page.on('request', request => requests.push(request.url()))
      await page.goto(origin + '/', { waitUntil: 'networkidle' })
      assert.ok(requests.length > 0)
      assert.ok(requests.every(url => new URL(url).origin === origin), requests.join('\n'))
      for (const selector of ['.wordmark', 'nav a', 'footer a']) {
        for (const box of await page.locator(selector).evaluateAll(nodes => nodes.map(node => {
          const rect = node.getBoundingClientRect()
          return { width: rect.width, height: rect.height }
        }))) {
          assert.ok(box.width >= 44, `${selector} width ${box.width}`)
          assert.ok(box.height >= 44, `${selector} height ${box.height}`)
        }
      }
    } finally { await browser.close() }
  })
})

test('@claim:supported-engines dry-run cards accept only Postgres and ClickHouse', () => {
  const out = mkdtempSync(join(tmpdir(), 'mlr-engines-'))
  try {
    for (const engine of ['postgres', 'clickhouse']) {
      const engineOut = join(out, engine)
      execFileSync('cargo', ['run', '--quiet', '--', 'demo', '--engine', engine, '--dry-run', '--output', engineOut], { cwd: root, stdio: 'pipe' })
      assert.match(readFileSync(join(engineOut, 'report.json'), 'utf8'), new RegExp(`"engine": "${engine}"`))
    }
    assert.throws(() => execFileSync('cargo', ['run', '--quiet', '--', 'demo', '--engine', 'mysql', '--dry-run', '--output', join(out, 'mysql')], { cwd: root, stdio: 'pipe' }), /unknown engine mysql/)
  } finally { rmSync(out, { recursive: true, force: true }) }
})

test('@claim:demo-reset documented reset removes only its explicit output folder', () => {
  const rootOut = mkdtempSync(join(tmpdir(), 'mlr-reset-'))
  const output = join(rootOut, 'demo')
  try {
    execFileSync('cargo', ['run', '--quiet', '--', 'demo', '--dry-run', '--output', output], { cwd: root, stdio: 'pipe' })
    execFileSync('cargo', ['run', '--quiet', '--', 'demo', '--output', output, '--reset'], { cwd: root, stdio: 'pipe' })
    assert.ok(!existsSync(output))
    assert.ok(existsSync(rootOut))
  } finally { rmSync(rootOut, { recursive: true, force: true }) }
})

test('@claim:invented-sample bundled fixture contains invented records and no connection URL', () => {
  const fixture = readFileSync(join(root, 'examples/postgres/fixture.sql'), 'utf8')
  assert.match(fixture, /aria@example\.test|fatima@example\.test/i)
  assert.doesNotMatch(fixture, /(?:postgres|mysql|clickhouse):\/\//i)
})

test('@claim:chosen-output dry-run writes both report files to the requested folder', () => {
  const parent = mkdtempSync(join(tmpdir(), 'mlr-output-'))
  const output = join(parent, 'chosen-folder')
  try {
    execFileSync('cargo', ['run', '--quiet', '--', 'demo', '--dry-run', '--output', output], { cwd: root, stdio: 'pipe' })
    assert.ok(existsSync(join(output, 'report.json')))
    assert.ok(existsSync(join(output, 'runbook.md')))
  } finally { rmSync(parent, { recursive: true, force: true }) }
})
