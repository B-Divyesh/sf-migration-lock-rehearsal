import assert from 'node:assert/strict'
import { execFileSync } from 'node:child_process'
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import test from 'node:test'

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

test('@claim:site-no-third-party built documentation has no remote runtime asset', () => {
  execFileSync('npm', ['run', 'build:site'], { cwd: root, stdio: 'pipe' })
  const index = readFileSync(join(root, 'dist/site/index.html'), 'utf8')
  assert.doesNotMatch(index, /<script[^>]+src=["']https?:\/\//i)
  assert.doesNotMatch(index, /<link[^>]+rel=["'](?:stylesheet|preload)["'][^>]+href=["']https?:\/\//i)
  assert.ok(existsSync(join(root, 'dist/site/assets')))
})

test('@claim:license-checkout the paid tier has the official checkout URL', () => {
  const code = readFileSync(join(root, 'src/main.ts'), 'utf8')
  assert.match(code, /https:\/\/api\.sociobot\.in\/api\/v1\/products\/migration-lock-rehearsal\/checkout/)
  assert.match(code, /\$29 once/)
})
