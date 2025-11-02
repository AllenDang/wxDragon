# wxDragon Event Handling Guide

This guide maps wxWidgets’ event system to wxDragon (the Rust bindings), explains the concrete Rust-side APIs and safety considerations, and provides common patterns with examples.

## wxWidgets → wxDragon mapping

- Native platform message → wxEvent → Rust Event wrapper
  - The C++ layer converts native messages into wxEvent, then dispatches them via a custom WxdEventHandler.
  - On the Rust side, events are thin wrappers around `event::Event(*mut wxd_Event_t)`; additional classification wraps it into a typed data structure per event family.

- Event types
  - wxEventType → `EventType` bitflags (values come from a stable C enum: `ffi::WXDEventTypeCEnum_*`).
  - Common types: button/menu/tool, mouse, keyboard, idle, paint, close, destroy, tree/list/dataview, timer, AUI, STC, RichText, etc.

- Binding and routing
  - wxEvtHandler::Bind/Connect → bindings via the `WxEvtHandler` trait:
    - Generic: `bind_internal(event_type, closure)` (crate-internal; public APIs expose on_* helpers)
    - With ID: `bind_with_id_internal(event_type, id, closure)` (for tools/menus, etc.)
  - Public Rust APIs are generated via traits/macros:
    - Window-level: `WindowEvents` (e.g. `window.on_mouse_left_down(...)`)
    - Class-level: `TextEvents`, `ButtonEvents`, `TreeEvents`, `MenuEvents`, … (e.g. `text.on_text(...)`)
    - Widget-local: each widget module exposes `on_*` methods

- Propagation and skip
  - The C++ side sets `event.Skip(true)` before invoking each handler; Rust controls this via `event.skip(bool)`.
  - If a handler calls `skip(false)`, the event is treated as consumed: subsequent handlers for the same key stop, and default processing/bubbling is prevented.

- Veto-able events
  - Common APIs: `event.can_veto()`, `event.veto()`, `event.is_vetoed()`.
  - After dispatch, the C++ layer checks veto; if vetoed, it blocks default processing.

- Sync/async
  - Recommended in Rust: use `wxdragon::call_after(Box<dyn FnOnce + Send>)` to switch work back to the GUI thread.
  - Idle: use `IdleEvent::set_mode(IdleMode)` plus `Window::set_extra_style(ExtraWindowStyle::ProcessIdle)`; inside handlers you can call `event.request_more(true)`.

- Binding handles and unbinding
  - Each binding returns an `EventToken` (opaque/usize); use `widget.unbind(token)` or `widget.unbind_all()`.
  - Automatic cleanup: on `DESTROY`, or when the handler is destroyed, C++ invokes Rust’s `drop_rust_event_closure_box` to free the boxed closure.

- Cross-platform example (tools/menus)
  - Internally, `widgets::tool::Tool::on_click` uses the `MENU` route on Windows and the `TOOL` route on macOS/Linux, binding to the right handler via `bind_with_id_internal`.

## Rust-side implementation notes and safety

### Event lifetime

- `event::Event` is a Copy/Clone wrapper around a transient wxEvent pointer. It’s only valid during the callback.
  - Do not store `Event` or its `*mut wxd_Event_t` beyond the callback, and do not move it across threads.
  - If you need data later, extract it in place (id, string, coordinates, key code, …) and store Rust values rather than `Event`.

- Family data wrappers such as `TextEventData/MouseEventData/KeyEventData/...` still hold an `Event`. Same rule: valid only during the callback.

- After a `DESTROY` event, `unbind_all` is triggered automatically, and the C++ handler destructor iterates through and frees all bound Rust closure boxes to prevent leaks/dangling references.

### Closure capture and ownership

- Handler types are `FnMut(Event) + 'static` or the corresponding family wrapper (e.g. `FnMut(TextEventData)`).
  - `'static` is required: don’t capture non-`'static` references. Use `Rc<RefCell<T>>` (within the GUI thread) or `Arc<Mutex<T>>` (cross-thread) for persistent state.

- GUI-thread only
  - Callbacks run on the GUI thread; don’t do long-running work inside callbacks.
  - When background work finishes, use `wxdragon::call_after` to jump back to the GUI and update the UI.

- Panic boundary
  - The trampoline uses `catch_unwind` to keep panics from crossing FFI. Avoid panics; prefer logging and early returns.

### Skip, propagation, and consumption

- Handlers start with `Skip(true)` by default. Three common strategies:
  - Observe-only: don’t call `skip(false)` (or explicitly `skip(true)`), allowing default behavior and parent-chain handling to continue.
  - Fully consume: call `event.skip(false)` after handling.
  - Conditional consume: decide dynamically whether to `skip(false)`.

- Command events (buttons, menus) bubble up the parent chain per wxWidgets rules; non-command events (mouse, keyboard, paint, etc.) typically don’t bubble.

### Veto pattern

- Check `if event.can_veto()` before calling `event.veto()`, e.g., “unsaved changes when closing a window.”
  - After veto, C++ blocks the default behavior (e.g., window won’t close). Your handler should also inform the user.

### Cross-threading and scheduling

- There’s no exposed PostEvent/QueueEvent wrapper; recommended approach:
  - `wxdragon::call_after(Box::new(move || { /* update UI */ }))` to schedule an update on the main loop.
  - Idle: set `IdleEvent::set_mode(IdleMode::ProcessSpecified)` and on the target window enable `ExtraWindowStyle::ProcessIdle`; inside the idle handler, call `event.request_more(true)` to continue idling.

- Send/Sync guidance
  - Callbacks themselves don’t need Send/Sync, but data moved across threads must be `Send`.
  - The closure passed to `call_after` must be `FnOnce + Send + 'static`.

### Binding, unbinding, and cleanup

- Every `on_*` binding returns an `EventToken`; keep it if you plan to unbind later:
  - `let token = btn.on_click(|e| { /* ... */ });`
  - `btn.unbind(token);` or `btn.unbind_all();`
- Automatic cleanup:
  - When a window receives `DESTROY`, all of its bindings are automatically removed.
  - On the C++ side, the handler destructor calls `drop_rust_event_closure_box` for each bound closure, ensuring the Rust box is freed.

### Small but important pitfalls

- Don’t hold `Event` beyond the callback; don’t call UI object APIs from non-GUI threads.
- Drawing outside of paint handlers can cause flicker or no effect; do painting in `on_paint`.
- Accumulating too many handlers without unbinding increases overhead; use `EventToken` wisely.
- Avoid blocking in callbacks (I/O or heavy CPU); use worker threads + `call_after` back to the GUI.
- Prevent reference cycles: if a closure captures an `Arc` to a widget that in turn retains the closure, consider `Weak` to break strong cycles.

## Common patterns and short examples

- Button click
```rust
use wxdragon::prelude::*;
use wxdragon::event::ButtonEvents;

let token = button.on_click(|e| {
    // e: ButtonEventData
    if let Some(label) = e.get_string() {
        println!("clicked: {label}");
    }
    // Don’t consume: let default behavior/bubbling continue
    // e.skip(true); // optional
});

// Unbind later
button.unbind(token);
```

- Consume an event (prevent follow-ups and default handling)
```rust
window.on_mouse_left_down(|e| {
    // ... custom handling ...
    e.event.skip(false); // or call .skip(false) on WindowEventData
});
```

- Veto close
```rust
use wxdragon::event::{WindowEvents, Event};

frame.on_close(|e| {
    let ev: Event = e.event; // Idle/Close/General wrappers all contain Event
    if ev.can_veto() && unsaved_changes() {
        // Inform the user and veto
        ev.veto();
        // C++ will block the default close behavior
    }
});
```

- Toolbar/Menu (platform adaptation handled internally)
```rust
use wxdragon::event::EventType;
let tool = toolbar.find_tool("open"); // example; adjust to your API
let _tok = tool.on_click(|_e| {
    // On Windows it goes through MENU; on macOS/Linux it goes through TOOL
});

let _menu_tok = frame.bind_with_id_internal(EventType::MENU, MY_ID, |_e| {
    // Bind a specific menu/tool event by ID
});
```

- Background work → back to GUI
```rust
std::thread::spawn(|| {
    let result = compute();
    wxdragon::call_after(Box::new(move || {
        label.set_label(&format!("Done: {result}"));
    }));
});
```

- Idle mode
```rust
use wxdragon::event::{IdleEvent, IdleMode, WindowEvents};
IdleEvent::set_mode(IdleMode::ProcessSpecified);
window.set_extra_style(ExtraWindowStyle::ProcessIdle);

window.on_idle(|idle| {
    // Do a bit of work
    if more_work_pending() {
        idle.event.request_more(true); // keep idling
    }
});
```

## Event families and data wrappers (partial)

- WindowEvents: LeftDown/Up, RightDown/Up, Motion, Enter/Leave, KeyDown/Up/Char, Size/Move/Paint/Erase/SetFocus/KillFocus, Idle, Close, Destroy
  - Data: `WindowEventData::{MouseButton, MouseMotion, MouseEnter, MouseLeave, Keyboard, Size, Idle, General}`
- ButtonEvents, TextEvents, TreeEvents, MenuEvents, ScrollEvents, TaskBarIconEvent, DataViewEvents, Timer, AUI/STC/RichText (feature-gated where applicable)

All `on_*` methods return an `EventToken` that can be used to unbind.

## Rust/FFI guarantees and limits summary

- Rust callbacks execute on the GUI thread; the FFI trampoline catches panics; C++ ensures closure boxes are freed on destroy.
- `Event` is a transient pointer wrapper and only guaranteed to live during the callback; extract values instead of storing the pointer.
- Propagation/consumption and veto are controlled via `skip(false)` and `veto()`; the C++ layer honors these results.
- Use `call_after` for cross-thread UI updates; don’t call UI APIs from non-GUI threads.

—--

Want to see it in action? Check the `events_triple_demo` example, which demonstrates “Close veto + Background work + Idle” in one place.
