import './style.css'
import './paid.css'
import './accessibility.css'
import './touch-targets.css'
import './forms.css'
import heroArt from './assets/lock-stack.webp'

const app = document.querySelector<HTMLDivElement>('#app')!
const productSlug = 'migration-lock-rehearsal'
const licenseKey = `sb_license:${productSlug}`
const verificationKey = `${licenseKey}:verification`
const billingBase = `https://api.sociobot.in/api/v1/products/${productSlug}`
const checkout = `${billingBase}/checkout`
const day = 86_400_000

type LicenseCache = { valid: boolean; checkedAt: number }

const nav = '<header class="site-header"><a class="wordmark" href="/" aria-label="Migration Lock Rehearsal home">MLR<span>///</span></a><nav aria-label="Main navigation"><a href="/demo">Demo</a><a href="/#how">How it works</a><a href="/privacy">Privacy</a></nav></header>'
const footer = '<footer><p>Rehearse database migrations before production.</p><p><a href="/privacy">Privacy</a> · <a href="/terms">Terms</a> · Built by Param Factory · v0.1.0</p></footer>'
const terminal = '<section class="terminal" aria-labelledby="terminal-title"><div class="terminal-bar"><span></span><span></span><span></span><b id="terminal-title">DISPOSABLE RUN / postgres</b></div><pre tabindex="0" aria-label="Sample terminal output"><code><i>$</i> mlr demo --output ./mlr-demo\nstarting a fresh Postgres container\nloading invented fixture: 6 customers\nrunning add_customer_flag.sql\nstatement time: 184 ms / limit: 30,000 ms\nlock wait: 0 ms / limit: 1,000 ms\ntable growth: 8,192 bytes / limit: 104,857,600 bytes\nrollback: checked\n<span class="go">VERDICT: GO</span>\nwrote ./mlr-demo/runbook.md<span class="cursor" aria-hidden="true">_</span></code></pre></section>'

function shell(content: string) {
  return nav + '<main id="main" tabindex="-1">' + content + '</main>' + footer + '<div id="route-note" class="sr-only" aria-live="polite"></div>'
}

function landing() {
  return shell('<section class="hero"><div><p class="eyebrow">MIGRATION PRE-FLIGHT / 0.1.0</p><h1>Rehearse your migration before production</h1><p class="lead">For database maintainers who need lock, rewrite, and rollback estimates before a release.</p><p class="action-row"><a class="button primary" href="/demo">Try it with sample data</a><span>See the bundled go/no-go card.</span></p><ul class="facts"><li>Fresh Docker container</li><li>Bundled invented sample</li><li>No tracking</li></ul></div><figure><img src="' + heroArt + '" width="1024" height="1024" fetchpriority="high" alt="A database cylinder held in an orange padlock with blue diagnostic tape."><figcaption>Measure the risk before the window opens.</figcaption></figure></section>' + terminal + '<section id="how" class="steps" aria-labelledby="how-title" tabindex="-1"><p class="eyebrow">THREE MOVES</p><h2 id="how-title">Run a migration rehearsal</h2><ol><li><b>Bring a fixture.</b><span>Use sanitized, production-shaped data.</span></li><li><b>Supply SQL.</b><span>Add the migration, rollback, and optional workload.</span></li><li><b>Read the card.</b><span>Compare timings, lock waits, and table growth with clear limits.</span></li></ol></section><section class="limits" aria-labelledby="limits-title"><h2 id="limits-title">What this tool does not do</h2><div><p>The rehearsal has no database URL option. It runs your SQL in the new container it creates.</p><p>Results are estimates. A failed command or exceeded limit always writes NO-GO.</p></div></section><section class="install" aria-labelledby="install-title"><h2 id="install-title">Install and rehearse</h2><pre><code>cargo run -- rehearse --fixture fixture.sql --migration change.sql --rollback down.sql --workload read.sql</code></pre><p>Docker must be running. The CLI creates a container and removes it after the run.</p></section><section class="paid" aria-labelledby="paid-title"><p class="eyebrow">OPERATOR LICENSE</p><h2 id="paid-title">Add the operator review checklist</h2><p>$29 once. The license adds a reusable release checklist. CLI reports and safety checks stay free.</p><p class="action-row"><a class="button primary" href="' + checkout + '">Buy operator license — $29</a><span id="license-state" role="status">No license saved.</span></p><form id="restore-license"><label for="license-token">Have a license? Paste it.</label><p id="license-help">The token stays in this browser and goes only to Sociobot for verification.</p><div><input id="license-token" name="license" autocomplete="off" aria-describedby="license-help license-state" required><button class="button" type="submit">Restore license</button><button class="button quiet" id="remove-license" type="button" hidden>Remove saved license</button></div></form><div id="operator-checklist" class="operator-checklist" hidden><h3>Operator review checklist</h3><ol><li>Attach the JSON card to the change ticket.</li><li>Name the owner who can stop the release.</li><li>Record the tested rollback command.</li><li>Compare every limit with the approved release budget.</li></ol></div><p class="legal-links">Sociobot and Dodo are the merchant of record. <a href="/privacy">Read privacy</a> and <a href="/terms">terms</a>.</p></section>')
}

function demo() {
  return shell('<section class="page-head"><p class="eyebrow">SAMPLE SANDBOX</p><h1>Read a sample migration card</h1><p class="lead">This preview uses invented customer records and does not save anything.</p><p class="action-row"><button class="button primary" id="reset-demo">Reset demo</button><a class="button" href="/">Start for real</a></p></section><div class="demo-banner" role="status">Demo — sample data, nothing is saved</div>' + terminal + '<section class="report" aria-labelledby="report-title"><h2 id="report-title">Go/no-go card</h2><dl><div><dt>Engine</dt><dd>Postgres 16</dd></div><div><dt>Statement time</dt><dd>184 / 30,000 ms</dd></div><div><dt>Lock wait</dt><dd>0 / 1,000 ms</dd></div><div><dt>Table growth</dt><dd>8,192 / 104,857,600 bytes</dd></div><div><dt>Rollback</dt><dd>Checked</dd></div><div><dt>Verdict</dt><dd>GO</dd></div></dl><p><b>Estimate only.</b> Rehearse against a sanitized, production-shaped fixture before deployment.</p></section>')
}

function legal(kind: 'privacy' | 'terms') {
  if (kind === 'privacy') {
    return shell('<article class="legal"><p class="eyebrow">PRIVACY</p><h1>Privacy for a local migration tool</h1><h2>No tracking or account data</h2><p>The site makes only same-origin requests until you use a license action. It has no analytics.</p><h2>Your reports stay local</h2><p>The CLI reads the SQL files you name. It writes reports only to your chosen output folder.</p><h2>License storage</h2><p>A saved license token stays in this browser. Verification sends only that token to api.sociobot.in.</p><p>Use Remove saved license on the home page to delete the token and cached result.</p></article>')
  }
  return shell('<article class="legal"><p class="eyebrow">TERMS</p><h1>Terms for Migration Lock Rehearsal</h1><h2>Use disposable data only</h2><p>Use sanitized fixtures. Measurements are estimates, not a guarantee of production behavior.</p><p>Review the generated runbook before every deployment.</p><h2>License and refunds</h2><p>The operator license costs $29 once. It adds the operator review checklist in this browser.</p><p>Sociobot and Dodo are the merchant of record. Refunds are handled there and revoke the license.</p></article>')
}

function notFound() {
  return shell('<section class="page-head not-found"><p class="eyebrow">404 / ROUTE NOT FOUND</p><h1>Find the rehearsal page</h1><p class="lead">That address does not point to a migration card.</p><a class="button primary" href="/">Return home</a></section>')
}

const routeMetadata: Record<string, { title: string; description: string }> = {
  '/': { title: 'Migration Lock Rehearsal — Test database changes', description: 'Rehearse a Postgres or ClickHouse migration and write a measured go/no-go card before production.' },
  '/demo': { title: 'Demo — Migration Lock Rehearsal', description: 'Read the bundled sample migration card with statement, lock, table growth, and rollback limits.' },
  '/privacy': { title: 'Privacy — Migration Lock Rehearsal', description: 'Learn what the local CLI writes and how optional operator license verification handles its token.' },
  '/terms': { title: 'Terms — Migration Lock Rehearsal', description: 'Read the terms for migration rehearsals and the one-time operator license.' },
  '/404': { title: 'Not found — Migration Lock Rehearsal', description: 'Return to the Migration Lock Rehearsal documentation.' },
}

function setMeta(selector: string, value: string) {
  document.querySelector<HTMLMetaElement>(selector)?.setAttribute('content', value)
}

function updateMetadata(path: string) {
  const metadata = routeMetadata[path] ?? routeMetadata['/404']
  const canonical = `https://migration-lock-rehearsal.sociobot.in${path === '/' ? '/' : path}`
  document.title = metadata.title
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')?.setAttribute('href', canonical)
  setMeta('meta[name="description"]', metadata.description)
  setMeta('meta[property="og:title"]', metadata.title)
  setMeta('meta[property="og:description"]', metadata.description)
  setMeta('meta[property="og:url"]', canonical)
  setMeta('meta[name="twitter:title"]', metadata.title)
  setMeta('meta[name="twitter:description"]', metadata.description)
}

function readCache(): LicenseCache | null {
  try {
    const raw = localStorage.getItem(verificationKey)
    if (!raw) return null
    const value = JSON.parse(raw) as Partial<LicenseCache>
    return typeof value.valid === 'boolean' && typeof value.checkedAt === 'number' ? value as LicenseCache : null
  } catch {
    return null
  }
}

function setLicenseUi(message: string, valid: boolean) {
  const state = document.querySelector<HTMLElement>('#license-state')
  const checklist = document.querySelector<HTMLElement>('#operator-checklist')
  const remove = document.querySelector<HTMLButtonElement>('#remove-license')
  if (state) state.textContent = message
  if (checklist) checklist.hidden = !valid
  if (remove) remove.hidden = !localStorage.getItem(licenseKey)
}

async function verifyLicense(token: string) {
  const cached = readCache()
  if (cached?.valid) setLicenseUi('License active.', true)
  if (cached && Date.now() - cached.checkedAt < day) {
    setLicenseUi(cached.valid ? 'License active.' : 'License no longer active. Buy a new license.', cached.valid)
    return
  }
  setLicenseUi('Checking license…', cached?.valid === true)
  try {
    const response = await fetch(`${billingBase}/verify?license=${encodeURIComponent(token)}`)
    if (!response.ok) throw new Error(`verification returned ${response.status}`)
    const result = await response.json() as { valid?: boolean }
    const valid = result.valid === true
    localStorage.setItem(verificationKey, JSON.stringify({ valid, checkedAt: Date.now() } satisfies LicenseCache))
    setLicenseUi(valid ? 'License active.' : 'License no longer active. Buy a new license.', valid)
  } catch {
    setLicenseUi(cached?.valid ? 'License active. Verification will retry when online.' : 'License saved. Verification will retry when online.', cached?.valid === true)
  }
}

function captureReturnedLicense() {
  const url = new URL(location.href)
  const token = url.searchParams.get('license')?.trim()
  if (!token) return
  localStorage.setItem(licenseKey, token)
  localStorage.removeItem(verificationKey)
  url.searchParams.delete('license')
  history.replaceState({}, '', url.pathname + url.search + url.hash)
}

function setupLicense() {
  const form = document.querySelector<HTMLFormElement>('#restore-license')
  if (!form) return
  const token = localStorage.getItem(licenseKey)
  if (token) void verifyLicense(token)
  form.addEventListener('submit', event => {
    event.preventDefault()
    const input = document.querySelector<HTMLInputElement>('#license-token')!
    const nextToken = input.value.trim()
    if (!nextToken) {
      setLicenseUi('Paste a license token to restore it.', false)
      input.focus()
      return
    }
    localStorage.setItem(licenseKey, nextToken)
    localStorage.removeItem(verificationKey)
    input.value = ''
    void verifyLicense(nextToken)
  })
  document.querySelector<HTMLButtonElement>('#remove-license')?.addEventListener('click', () => {
    localStorage.removeItem(licenseKey)
    localStorage.removeItem(verificationKey)
    setLicenseUi('License removed from this browser.', false)
  })
}

function focusHash() {
  if (!location.hash) return false
  const target = document.getElementById(decodeURIComponent(location.hash.slice(1)))
  if (!target) return false
  target.focus({ preventScroll: true })
  target.scrollIntoView({ block: 'start' })
  return true
}

function route(moveFocus = false) {
  const path = location.pathname.replace(/\/$/, '') || '/'
  app.innerHTML = path === '/demo' ? demo() : path === '/privacy' ? legal('privacy') : path === '/terms' ? legal('terms') : path === '/404' ? notFound() : path === '/' ? landing() : notFound()
  updateMetadata(path)
  requestAnimationFrame(() => {
    if (!focusHash() && moveFocus) {
      const heading = document.querySelector<HTMLElement>('h1')!
      heading.tabIndex = -1
      heading.focus()
    }
  })
  document.querySelector('#route-note')!.textContent = document.title
  document.querySelector('#reset-demo')?.addEventListener('click', () => {
    const button = document.querySelector<HTMLButtonElement>('#reset-demo')!
    button.textContent = 'Demo reset'
    setTimeout(() => { button.textContent = 'Reset demo' }, 1300)
  })
  document.querySelectorAll<HTMLAnchorElement>('a[href^="/"]').forEach(anchor => anchor.addEventListener('click', event => {
    if (event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return
    const url = new URL(anchor.href)
    const samePath = url.pathname === location.pathname
    event.preventDefault()
    history.pushState({}, '', url.pathname + url.search + url.hash)
    if (samePath && url.hash && document.getElementById(url.hash.slice(1))) {
      focusHash()
      return
    }
    route(true)
  }))
  setupLicense()
}

captureReturnedLicense()
addEventListener('popstate', () => route(true))
route()
