import './style.css'
import './paid.css'
import './accessibility.css'
import './touch-targets.css'
import './forms.css'
import heroArt from './assets/lock-stack.webp'
import recording from '../public/demo-recording.json'

const app = document.querySelector<HTMLDivElement>('#app')!
const productSlug = 'migration-lock-rehearsal'
const licenseKey = `sb_license:${productSlug}`
const verificationKey = `${licenseKey}:verification`
const billingBase = `https://api.sociobot.in/api/v1/products/${productSlug}`
const checkout = `${billingBase}/checkout`
const day = 86_400_000

type LicenseCache = { valid: boolean; checkedAt: number }

const nav = '<header class="site-header"><a class="wordmark" href="/" aria-label="Migration Lock Rehearsal home">MLR<span>///</span></a><nav aria-label="Main navigation"><a href="/?demo=1">Demo</a><a href="/#how">How it works</a><a href="/privacy">Privacy</a></nav></header>'
const footer = '<footer><p>Rehearse database migrations before production.</p><p><a href="/privacy">Privacy</a> · <a href="/terms">Terms</a> · Built by Param Factory · v0.1.0</p></footer>'
type DemoRecording = { command: string; transcript: string[] }
const demoRecording = recording as DemoRecording

function terminal(recordingMode = false) {
  const output = recordingMode ? '' : demoRecording.transcript.join('\n')
  const recordingClass = recordingMode ? ' terminal-recording' : ''
  return '<section class="terminal' + recordingClass + '" aria-labelledby="terminal-title"><div class="terminal-bar"><span></span><span></span><span></span><b id="terminal-title">RECORDED DRY RUN / postgres</b></div><pre tabindex="0" aria-label="Recorded sample terminal output"><code id="terminal-output">' + output + '</code><span class="cursor" aria-hidden="true">_</span></pre>' + (recordingMode ? '<p class="recording-note">Recorded from <code>' + demoRecording.command + '</code> using the bundled release binary.</p>' : '') + '</section>'
}

function shell(content: string) {
  return nav + '<main id="main" tabindex="-1">' + content + '</main>' + footer + '<div id="route-note" class="sr-only" aria-live="polite"></div>'
}

function landing() {
  return shell('<section class="hero"><div><p class="eyebrow">POSTGRES + CLICKHOUSE / v0.1.0</p><h1>Rehearse your migration before production</h1><p class="lead">For Postgres and ClickHouse maintainers who need lock waits, table growth, and rollback results before release.</p><p class="action-row"><a class="button primary" href="/?demo=1">Try it with sample data</a><span>Watch the bundled go/no-go report.</span></p><ul class="facts"><li>Local dry-run works offline</li><li>No analytics; license checks contact Sociobot</li><li>$29 once; browser checklist</li></ul></div><figure><img src="' + heroArt + '" width="1024" height="1024" fetchpriority="high" alt="A database cylinder held in an orange padlock with blue diagnostic tape."><figcaption>Compare measured results with your release limits.</figcaption></figure></section>' + terminal() + '<section id="how" class="steps" aria-labelledby="how-title" tabindex="-1"><p class="eyebrow">HOW IT WORKS</p><h2 id="how-title">Run a migration rehearsal</h2><ol><li><b>Bring a fixture.</b><span>Use sanitized, production-shaped data.</span></li><li><b>Supply SQL.</b><span>Add the migration, rollback, and optional workload.</span></li><li><b>Read the report.</b><span>Compare timings, lock waits, and table growth with clear limits.</span></li></ol></section><section class="limits" aria-labelledby="limits-title"><h2 id="limits-title">What this tool does not do</h2><div><p>The rehearsal has no database URL option. It runs your SQL in the new container it creates.</p><p>Results are estimates. A failed Docker command or exceeded limit writes NO-GO.</p></div></section><section id="install" class="install" aria-labelledby="install-title" tabindex="-1"><h2 id="install-title">Install and rehearse</h2><p><a href="https://github.com/B-Divyesh/sf-migration-lock-rehearsal" rel="external">Get the source on GitHub (external)</a>.</p><pre><code>cargo install --git https://github.com/B-Divyesh/sf-migration-lock-rehearsal --locked\nmlr rehearse --fixture ./fixture.sql --migration ./change.sql --rollback ./down.sql --workload ./read.sql --output ./rehearsal-report</code></pre><p>Docker must be running. The CLI creates a container and removes it after the run.</p></section><section class="paid" aria-labelledby="paid-title"><p class="eyebrow">OPERATOR LICENSE</p><h2 id="paid-title">Add the operator review checklist</h2><p>$29 once. A valid license shows the operator review checklist in this browser. Reports and safety checks do not require a license.</p><p class="action-row"><a class="button primary" href="' + checkout + '">Buy operator license — $29</a><span id="license-state" role="status">No license saved.</span></p><form id="restore-license"><label for="license-token">Have a license? Paste it.</label><p id="license-help">The token stays in this browser and goes only to Sociobot for verification.</p><div><input id="license-token" name="license" autocomplete="off" aria-describedby="license-help license-state" required><button class="button" type="submit">Restore license</button><button class="button quiet" id="remove-license" type="button" hidden>Remove saved license</button></div></form><div id="operator-checklist" class="operator-checklist" hidden><h3>Operator review checklist</h3><ol><li>Attach the JSON report to the change ticket.</li><li>Name the owner who can stop the release.</li><li>Record the tested rollback command.</li><li>Compare every limit with the approved release budget.</li></ol></div><p class="legal-links"><a href="/privacy">Read privacy</a> and <a href="/terms">terms</a>.</p></section>')
}

function demo() {
  return shell('<section class="page-head"><p class="eyebrow">SAMPLE SANDBOX</p><h1>Read a sample go/no-go report</h1><p class="lead">This preview uses invented customer records and does not save anything.</p><p class="action-row"><button class="button primary" id="reset-demo">Reset demo</button><a class="button" href="/#install">Install the CLI</a></p></section><div class="demo-banner" role="status">Demo — sample data, nothing is saved <span id="demo-status" class="sr-only" aria-live="polite"></span></div>' + terminal(true) + '<section class="report" aria-labelledby="report-title"><h2 id="report-title">Go/no-go report</h2><dl><div><dt>Engine</dt><dd>Postgres 16</dd></div><div><dt>Statement time</dt><dd>184 / 30,000 ms</dd></div><div><dt>Lock wait</dt><dd>0 / 1,000 ms</dd></div><div><dt>Table growth</dt><dd>8,192 / 104,857,600 bytes</dd></div><div><dt>Rollback</dt><dd>Checked</dd></div><div><dt>Verdict</dt><dd>GO</dd></div></dl><p><b>Estimate only.</b> Rehearse against a sanitized, production-shaped fixture before deployment.</p></section>')
}

function legal(kind: 'privacy' | 'terms') {
  if (kind === 'privacy') {
    return shell('<article class="legal"><p class="eyebrow">PRIVACY</p><h1>Privacy for a local migration tool</h1><h2>No analytics or account data</h2><p>The site has no analytics. Before a license action, it makes only same-origin requests and stores no visitor data.</p><h2>Your reports stay local</h2><p>The CLI reads the SQL files you name. It writes reports only to your chosen output folder.</p><h2>License storage</h2><p>A saved license token stays in this browser. A license check sends only that token to api.sociobot.in.</p><p>Use Remove saved license on the home page to delete the token and cached result.</p></article>')
  }
  return shell('<article class="legal"><p class="eyebrow">TERMS</p><h1>Terms for Migration Lock Rehearsal</h1><h2>Use disposable data only</h2><p>Use sanitized fixtures. Measurements are estimates, not a guarantee of production behavior.</p><p>Review the generated runbook before every deployment.</p><h2>Operator license</h2><p>The operator license costs $29 once. A valid license shows the review checklist in this browser.</p><p>Checkout and license verification use Sociobot. Dodo Payments is the merchant of record and handles order-related inquiries and returns.</p><p><a href="https://dodopayments.com/buyer-terms" rel="external">Read Dodo Payments’ buyer terms and refund policy (external)</a>.</p></article>')
}

function notFound() {
  return shell('<section class="page-head not-found"><p class="eyebrow">404 / ROUTE NOT FOUND</p><h1>Find the rehearsal page</h1><p class="lead">That address does not point to a Migration Lock Rehearsal page.</p><a class="button primary" href="/">Return home</a></section>')
}

const routeMetadata: Record<string, { title: string; description: string }> = {
  '/': { title: 'Migration Lock Rehearsal — Test database changes', description: 'Rehearse a Postgres or ClickHouse migration and write a measured go/no-go report before production.' },
  '/demo': { title: 'Demo — Migration Lock Rehearsal', description: 'Read the bundled sample go/no-go report with statement, lock, table growth, and rollback limits.' },
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

let recordingTimer: number | undefined
function startDemoRecording() {
  window.clearInterval(recordingTimer)
  const output = document.querySelector<HTMLElement>('#terminal-output')
  const status = document.querySelector<HTMLElement>('#demo-status')
  if (!output) return
  let index = 0
  output.textContent = ''
  if (status) status.textContent = 'Sample recording restarted.'
  const reveal = () => {
    output.textContent = demoRecording.transcript.slice(0, index + 1).join('\n')
    index += 1
    if (index >= demoRecording.transcript.length) {
      window.clearInterval(recordingTimer)
      if (status) status.textContent = 'Sample recording complete.'
    }
  }
  reveal()
  recordingTimer = window.setInterval(reveal, 260)
}

function route(moveFocus = false) {
  const path = location.pathname.replace(/\/$/, '') || '/'
  const isDemo = path === '/demo' || (path === '/' && new URLSearchParams(location.search).get('demo') === '1')
  app.innerHTML = isDemo ? demo() : path === '/privacy' ? legal('privacy') : path === '/terms' ? legal('terms') : path === '/404' ? notFound() : path === '/' ? landing() : notFound()
  updateMetadata(isDemo ? '/demo' : path)
  requestAnimationFrame(() => {
    if (!focusHash() && moveFocus) {
      const heading = document.querySelector<HTMLElement>('h1')!
      heading.tabIndex = -1
      heading.focus()
    }
  })
  document.querySelector('#route-note')!.textContent = document.title
  if (isDemo) {
    startDemoRecording()
    document.querySelector('#reset-demo')?.addEventListener('click', startDemoRecording)
  }
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
