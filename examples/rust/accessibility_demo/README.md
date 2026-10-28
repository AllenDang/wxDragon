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
- **macOS / VoiceOver, checkable groups (#160):** Interact with the DataView and
  navigate Group A and Group B with the arrow keys. With container columns on,
  both group labels should be available, together with checkbox state and the
  disclosure triangle. Expand/collapse groups and check that leaf labels remain
  available. Toggle checkboxes and verify their state is announced.
  - Toggle container columns off to reproduce the missing group text; turn them
    on again to restore it. Changing the policy rebuilds the view, so selection
    and expansion state may reset.
  - Rename Group B and verify "Group B — 支払い" replaces its original label.
    Toggle empty group checkbox values and verify group text remains available.
  - Sort by the text column and confirm labels follow their rows.

For custom tree models with values outside column 0 on container rows, call
`model.set_container_columns(true)` before `view.associate_model(&model)`.
Setting another expander column alone does not enable those values: wxWidgets'
default `HasContainerColumns()` is false. The setting is model-wide and works on
all platforms; it does not install an explicit accessible-name override. The
native macOS regression checks the actual AX text, checkbox and disclosure
elements, but VoiceOver speech and keyboard interaction still need manual checks.

Use explicit names for controls without suitable labels, or when the announced name
needs to differ from the visible label. Empty names restore the native default.

Upstream fixes (no extra wxDragon API needed) also warrant manual smoke tests:
VoiceOver context-menu keys, Tab navigation/announcements in generic `wxInfoBar`,
`wxCollapsibleHeaderCtrl` button semantics, and bitmap-only button names.
