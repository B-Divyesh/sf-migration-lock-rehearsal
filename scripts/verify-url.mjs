import assert from 'node:assert/strict'
import AxeBuilder from '@axe-core/playwright'
import { chromium } from 'playwright'

const base = (process.argv[2] ?? 'http://127.0.0.1:4173').replace(/\/$/, '')
const browser = await chromium.launch({ headless: true })

try {
  for (const viewport of [{ width: 1440, height: 900 }, { width: 390, height: 844 }]) {
    const context = await browser.newContext({ viewport })
    const page = await context.newPage()
    const errors = []
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()) })
    page.on('pageerror', error => errors.push(error.message))

    for (const path of ['/', '/demo', '/privacy', '/terms']) {
      const response = await page.goto(base + path, { waitUntil: 'networkidle' })
      assert.equal(response?.status(), 200, `${path} status`)
      assert.equal(await page.locator('html').getAttribute('lang'), 'en', `${path} lang`)
      assert.ok((await page.title()).length > 0, `${path} title`)
      assert.equal(await page.locator('main').count(), 1, `${path} main`)
      assert.equal(await page.locator('h1').count(), 1, `${path} h1`)
      assert.equal(await page.locator('img:not([alt])').count(), 0, `${path} image alt`)
      assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth), `${path} overflow`)
      const axe = await new AxeBuilder({ page }).analyze()
      const serious = axe.violations.filter(item => ['serious', 'critical'].includes(item.impact ?? ''))
      assert.deepEqual(serious.map(item => item.id), [], `${path} axe`)
    }

    assert.deepEqual(errors, [], `console errors at ${viewport.width}px`)
    await context.close()
  }
  console.log(`PASS ${base}: title, lang, landmarks, alt, console, mobile overflow, axe`)
} finally {
  await browser.close()
}
