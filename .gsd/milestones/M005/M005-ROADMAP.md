# M005: M005: Auto-Clear Timer with Yeti Countdown

**Vision:** Single-line input auto-clears after a configurable timeout with an animated Yeti countdown ring and blow animation, providing a safety net for sensitive text.

## Success Criteria

- User can enable auto-clear in Settings and configure the timeout in seconds
- A countdown ring animates around the Yeti while the timer is active
- The Yeti blows a smoke puff and the text is instantly deleted when the timer expires
- Timer resets when the user types new text, and stops when switching to multi-line mode

## Slices

- [x] **S01: Auto-clear settings UI and persistence** `risk:low` `depends:[]`
  > After this: User can toggle auto-clear on/off and set the timeout in Settings; values persist and are emitted to the main window via settings-changed event

- [x] **S02: Countdown ring animation around Yeti** `risk:medium` `depends:[S01]`
  > After this: When auto-clear is enabled and text is present in single-line mode, a colored ring animates around the Yeti counting down; ring resets on new input and hides when timer is disabled or mode switches to multi-line

- [x] **S03: Yeti blow animation and text clearing** `risk:medium` `depends:[S02]`
  > After this: When countdown reaches zero, the Yeti opens its mouth, emits a smoke puff, and the text is instantly deleted; animation adapts when eyes are covered (skips cheek/eye animations)

## Boundary Map

## Boundary Map

### S01 → S02

Produces:
- `autoClearEnabled` (boolean) and `autoClearTimeout` (number, seconds) settings keys in the store
- `settings-changed` event payload extended with `autoClearEnabled` and `autoClearTimeout` fields

Consumes:
- nothing (first slice)

### S02 → S03

Produces:
- `startCountdown(seconds)`, `resetCountdown()`, `stopCountdown()` functions on `window.yetiCountdown`
- `onCountdownExpired` callback hook for S03 to attach the blow animation

Consumes:
- S01: `autoClearEnabled` and `autoClearTimeout` settings values from the store and settings-changed event
