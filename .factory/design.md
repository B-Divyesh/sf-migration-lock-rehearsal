# Migration Lock Rehearsal design system

## Direction

**Neo-brutalist utility.** This is a pre-flight instrument, not a dashboard. It uses hard rules, offset shadows, dense data strips, and warning ink so a maintainer reads it like an operations card.

## Tokens

- Background `#f4efe3` (warm paper), surface `#fffdf6`, ink `#111111`, muted `#47433d`.
- Safety blue `#0057ff`, warning orange `#ff5c35`, success green `#167a3e`, danger red `#b51f2b`.
- Display: `Arial Black`, `Impact`, sans-serif. Body: system UI / `Arial`, sans-serif. Numbers use tabular figures.
- Spacing: 8px grid. Borders are 3px black; cards have 6px offset shadows. The content measure is 68ch.

## Interaction and motion

Buttons shift into their hard shadow on press. The terminal cursor has one restrained blink; reduced-motion users get a static cursor and no transitions. Sections use ruled divider lines instead of soft cards.

The paid checklist uses the same hard-rule instrument language. It appears only after license verification and uses a blue offset shadow to separate purchased reference content from safety-critical free output.

## Asset plan and provenance

`src/assets/lock-stack.webp` is original factory-generated artwork: a screen-print-like black database cylinder trapped in an orange padlock, blue diagnostic tape, warm paper background, no text or trademarks. Generated with `/opt/fleet/lib/gen-image.sh`, deployment `factory-image`; prompt and sidecar provenance are stored beside the asset. It is displayed as an explanatory hero, not as text.
