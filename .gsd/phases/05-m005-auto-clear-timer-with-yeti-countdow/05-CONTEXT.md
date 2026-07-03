# M005: Auto-Clear Timer with Yeti Countdown

**Gathered:** 2026-05-31
**Status:** Ready for planning

## Project Description

Add an optional auto-clear timer to single-line input mode. When enabled, the input field is automatically cleared after a configurable timeout. A countdown ring animates around the Yeti mascot, and when time expires, the Yeti "blows" the text away with a smoke puff animation before the field is cleared.

## Why This Milestone

Users frequently paste sensitive text (passwords, tokens) into the single-line input. Forgetting to clear the field leaves sensitive data visible. An auto-clear timer provides a safety net — the text disappears automatically after a configured period, reducing the risk of accidental exposure.

## User-Visible Outcome

### When this milestone is complete, the user can:

- Enable "Auto-Clear" in Settings with a configurable timeout (in seconds)
- See a countdown ring animate around the Yeti when text is present in single-line mode
- Watch the Yeti blow a smoke puff and the text disappears when the timer expires
- Reset the timer by typing new text or manually clearing the field

### Entry point / environment

- Entry point: Main window (single-line input mode) + Settings window
- Environment: Windows desktop (Tauri app)
- Live dependencies involved: none

## Completion Class

- Contract complete means: settings persist, timer starts/resets/clears correctly, animation triggers
- Integration complete means: timer interacts correctly with paste flow, mask toggle, mode switching, and window focus
- Operational complete means: none

## Final Integrated Acceptance

To call this milestone complete, we must prove:

- User enables auto-clear in settings, types text, countdown ring appears, Yeti blows, text is cleared
- Timer resets when user types new text mid-countdown
- Switching to multi-line mode stops/hides the timer
- Timer does not interfere with mask (eyes-covered) mode — Yeti blow animation adapts

## Architectural Decisions

### Countdown visualization as SVG ring around the Yeti

**Decision:** Use an SVG circle overlay around the Yeti container, animated via GSAP, with color transitions (blue → yellow → red).

**Rationale:** Leverages existing GSAP dependency, no additional libraries needed. The Yeti is always visible, so the ring is always noticeable without taking extra screen space.

**Alternatives Considered:**
- Progress bar under input field — takes vertical space, changes input field border radius
- Inline badge inside input field — shifts text, easily missed

### Yeti blow animation

**Decision:** On timer expiry, the Yeti opens its mouth briefly, emits a small smoke cloud (SVG circles), and the text is instantly deleted (no fade/clone).

**Rationale:** User-approved mockup (docs/mockups/countdown-yeti-blow.html). Simple deletion avoids the "double text" visual glitch of clone-based approaches. Smoke cloud provides visual feedback that something happened.

**Alternatives Considered:**
- Text fade-out — caused visual "doubling" artifact, rejected by user
- Text slide-out — user preferred instant deletion

## Error Handling Strategy

Timer is purely frontend. If GSAP is unavailable, the timer still clears the text (just without animation). If the user switches modes mid-countdown, the timer is silently cancelled.

## Risks and Unknowns

- SVG ring positioning around Yeti across different window sizes — low risk, Yeti container has fixed dimensions
- GSAP timeline coordination between countdown ring and blow animation — medium risk, needs careful sequencing

## Existing Codebase / Prior Art

- `src/animation.js` — existing Yeti animation controller (eye tracking, arm cover), will be extended with blow animation
- `src/app.js` — main app controller, handles input mode switching and clear logic
- `src/settings.js` — settings controller, will add new auto-clear options
- `src/settings.html` — settings UI, will add new section
- `docs/mockups/countdown-yeti-blow.html` — approved mockup for the blow animation

## Relevant Requirements

- No formal requirements registered yet

## Scope

### In Scope

- Settings: enable/disable auto-clear, configurable timeout (seconds)
- Countdown ring SVG animation around Yeti
- Yeti blow animation (mouth open, smoke puff, instant text delete)
- Timer reset on new input
- Timer cancel on mode switch to multi-line
- Adapted blow animation when eyes are covered (skip cheek/eye animations)

### Out of Scope / Non-Goals

- Auto-clear in multi-line mode
- Server-side or backend timer logic
- Clipboard clearing (only the input field is cleared)

## Technical Constraints

- Must use existing GSAP v3 (no new animation libraries)
- Must work within the fixed 300px window width
- SVG modifications must not break existing Yeti eye-tracking and arm-cover animations

## Integration Points

- `settings-changed` event — new fields for auto-clear settings
- `yetiAnimation` global — new `blowText()` method
- Input mode switching — timer must stop when switching to multi-line

## Testing Requirements

Manual testing in the running app:
- Enable auto-clear, set timer, verify countdown and clearing
- Type during countdown → timer resets
- Switch to multi-line during countdown → timer stops
- Toggle mask during countdown → blow animation adapts
- Disable auto-clear → no countdown appears

## Acceptance Criteria

- S01: Settings UI shows auto-clear toggle and time slider, values persist
- S02: Countdown ring animates around Yeti, color transitions work, timer resets on input
- S03: Yeti blow animation plays on timer expiry, text is instantly cleared, adapts to eyes-covered state

## Open Questions

- None — all design decisions resolved via mockup iteration
