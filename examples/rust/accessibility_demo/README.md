# Accessibility demo

Run `cargo run -p accessibility_demo` on a desktop with wxWidgets 3.3.4.

## Manual checks

- **macOS / VoiceOver:** Language, City, and Destinations should be announced with
  their visible labels without calling `set_accessibility_label`. Each `StaticText`
  is created immediately before its control, with the same parent. Visual adjacency
  or sizer order alone does not establish this association. Keep the labels accessible.
- **Windows / NVDA or Narrator:** Tab to the single-line read-only reservation
  field and hear its value. Toggle its Tab access, then check Tab and Shift+Tab:
  disabling skips the field, enabling makes it reachable again.
- **Windows and macOS:** The Continue button should have the explicit Unicode name
  "Continue to payment — 支払い". Restoring its default name should announce
  "Continue" again. On Windows, its description must not erase the name override.
- **Linux:** The demo should build and keyboard-focus controls should work normally.
  Explicit accessible-name overrides remain a no-op on this platform.

Use explicit names for controls without suitable labels, or when the announced name
needs to differ from the visible label. Empty names restore the native default.

Upstream fixes (no extra wxDragon API needed) also warrant manual smoke tests:
VoiceOver context-menu keys, Tab navigation/announcements in generic `wxInfoBar`,
`wxCollapsibleHeaderCtrl` button semantics, and bitmap-only button names.
