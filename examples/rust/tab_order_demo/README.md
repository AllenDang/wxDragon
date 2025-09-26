# Tab Order Demo

This example demonstrates the tab order functionality in wxdragon, showing how to control and manipulate the order in which controls receive focus when the user presses Tab or Shift+Tab.

## Features Demonstrated

### Tab Order Control Functions
- `move_after_in_tab_order()` - Move a control to appear after another in tab order
- `move_before_in_tab_order()` - Move a control to appear before another in tab order

### Navigation Functions
- `navigate(forward: bool)` - Programmatically navigate to next/previous focusable control
- `get_next_sibling()` - Get the next sibling window in the parent's child list
- `get_prev_sibling()` - Get the previous sibling window in the parent's child list

### Window Styles
- `PanelStyle::TabTraversal` - Enables tab navigation within the panel
- `WindowStyle::TabStop` - Controls whether a window can receive focus via Tab key

## How to Run

```bash
cd examples/rust/tab_order_demo
cargo run
```

## What the Demo Shows

The application creates a window with various controls (buttons, text fields, checkboxes, radio buttons, combo box) and provides several buttons to demonstrate different tab order manipulations:

1. **Reset to Default Order** - Restores the original tab order
2. **Reverse Tab Order** - Reverses the tab order using `move_before_in_tab_order()`
3. **Custom Tab Order** - Creates a custom order (text controls first, then buttons) using `move_after_in_tab_order()`
4. **Test Navigate() Function** - Demonstrates programmatic navigation

## Interactive Testing

- Use **Tab** and **Shift+Tab** to navigate between controls
- Click the control buttons to change the tab order
- Watch the status bar to see which control currently has focus
- Click "Button 1" to test sibling navigation functions

## Key Learning Points

1. **Tab Order Manipulation**: You can change the tab order at runtime using the move functions
2. **Programmatic Navigation**: Use `navigate()` to move focus programmatically
3. **Sibling Access**: Access sibling windows in the parent's child list
4. **Focus Events**: Bind to focus events to track tab navigation
5. **Style Requirements**: Use `TabTraversal` style on panels and `TabStop` on focusable controls

## Implementation Notes

- All tab order functions work on any widget implementing the `WxWidget` trait
- The functions are object-safe (use `&dyn WxWidget` parameters)
- Tab order changes take effect immediately
- Focus events help visualize the current tab order
- The demo uses proper wxdragon patterns with builders and event handlers