# Handoff — adversarial first-read review 3

## Result: FAIL

Review 3 was completed against candidate
`3e474b1ca097ba9600772ba36df528e1cae5347e` and the live product at
<https://migration-lock-rehearsal.sociobot.in>. Product code was not modified.
The full report is `.factory/review-3.md`.

## What was done

- Cold-opened fresh 390 × 844 and 1440 × 900 Chromium contexts.
- Exercised the one-click demo, recording reset, seeded-storage isolation, and
  live request log.
- Audited every landing and README sentence and reader-facing label.
- Invoked all 19 `claims.json` commands separately in clean clone
  `/tmp/mlr-review3-clean.BV21SX`.
- Confirmed the two declared local Docker skips against successful real-Docker
  Actions run 33249962549; relevant product/claim code is unchanged.
- Rechecked every finding in reviews 1 and 2, both polish reports, and the prior
  handoff against live behavior and source.
- Crawled routes/assets/links and checked metadata, history focus, 404 behavior,
  privacy, accessibility, and visual identity.

## Verification

```sh
npm run verify:url -- https://migration-lock-rehearsal.sociobot.in
# PASS: title, lang, landmarks, alt, console, mobile overflow, axe
```

All deterministic claim tests passed. `docker-rehearsal` and
`container-cleanup` skipped locally because Docker is unavailable; their
required real-Docker CI run passed. The direct CLI dry-run in a temporary
directory wrote a GO report and runbook only under the chosen output.

## Findings left

1. Blocking F-3-1 reopens F-1-6: “Sociobot/Dodo” merchant/refund wording is not
   supported by the hosted checkout, and its test only checks that the wording
   renders.
2. Major F-3-2: README installs `mlr` but tells the user to run `cargo run` next;
   that fails outside a source checkout.
3. Minor F-3-3: “No tracking before a license action” leaves post-action privacy
   behavior unclear.

Next work should fix those three items and rerun review 3 from scratch. Only
`.factory/review-3.md` and this handoff were changed for this review.
