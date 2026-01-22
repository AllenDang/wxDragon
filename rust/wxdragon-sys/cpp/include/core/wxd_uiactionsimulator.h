#ifndef WXD_UIACTIONSIMULATOR_H
#define WXD_UIACTIONSIMULATOR_H

#include "../wxd_types.h"

// --- UIActionSimulator Functions ---

/**
 * Creates a new UIActionSimulator.
 * @return Pointer to the new UIActionSimulator, or NULL on failure.
 */
WXD_EXPORTED wxd_UIActionSimulator_t*
wxd_UIActionSimulator_Create();

/**
 * Destroys a UIActionSimulator.
 * @param sim Pointer to the UIActionSimulator to destroy.
 */
WXD_EXPORTED void
wxd_UIActionSimulator_Destroy(wxd_UIActionSimulator_t* sim);

// --- Mouse Simulation ---

/**
 * Move the mouse to the specified coordinates.
 * @param sim Pointer to the UIActionSimulator.
 * @param x X coordinate in screen coordinates.
 * @param y Y coordinate in screen coordinates.
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_MouseMove(wxd_UIActionSimulator_t* sim, long x, long y);

/**
 * Press a mouse button.
 * @param sim Pointer to the UIActionSimulator.
 * @param button The button to press (WXD_MOUSE_BTN_LEFT, WXD_MOUSE_BTN_MIDDLE, WXD_MOUSE_BTN_RIGHT).
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_MouseDown(wxd_UIActionSimulator_t* sim, int button);

/**
 * Release a mouse button.
 * @param sim Pointer to the UIActionSimulator.
 * @param button The button to release.
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_MouseUp(wxd_UIActionSimulator_t* sim, int button);

/**
 * Click a mouse button (press and release).
 * @param sim Pointer to the UIActionSimulator.
 * @param button The button to click.
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_MouseClick(wxd_UIActionSimulator_t* sim, int button);

/**
 * Double-click a mouse button.
 * @param sim Pointer to the UIActionSimulator.
 * @param button The button to double-click.
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_MouseDblClick(wxd_UIActionSimulator_t* sim, int button);

/**
 * Perform a drag and drop operation.
 * @param sim Pointer to the UIActionSimulator.
 * @param x1 Starting X coordinate in screen coordinates.
 * @param y1 Starting Y coordinate in screen coordinates.
 * @param x2 Ending X coordinate in screen coordinates.
 * @param y2 Ending Y coordinate in screen coordinates.
 * @param button The button to use for dragging.
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_MouseDragDrop(wxd_UIActionSimulator_t* sim,
                                     long x1, long y1, long x2, long y2,
                                     int button);

// --- Keyboard Simulation ---

/**
 * Press a key.
 * @param sim Pointer to the UIActionSimulator.
 * @param keycode The key code (wxKeyCode).
 * @param modifiers Modifier keys (combination of WXD_MOD_* flags).
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_KeyDown(wxd_UIActionSimulator_t* sim, int keycode, int modifiers);

/**
 * Release a key.
 * @param sim Pointer to the UIActionSimulator.
 * @param keycode The key code (wxKeyCode).
 * @param modifiers Modifier keys (combination of WXD_MOD_* flags).
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_KeyUp(wxd_UIActionSimulator_t* sim, int keycode, int modifiers);

/**
 * Press and release a key.
 * @param sim Pointer to the UIActionSimulator.
 * @param keycode The key code (wxKeyCode).
 * @param modifiers Modifier keys (combination of WXD_MOD_* flags).
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_Char(wxd_UIActionSimulator_t* sim, int keycode, int modifiers);

/**
 * Emulate typing the given string.
 * @param sim Pointer to the UIActionSimulator.
 * @param text The text to type (ASCII characters).
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_Text(wxd_UIActionSimulator_t* sim, const char* text);

/**
 * Select an item with the given text in the currently focused control.
 * @param sim Pointer to the UIActionSimulator.
 * @param text The text of the item to select.
 * @return true if successful, false otherwise.
 */
WXD_EXPORTED bool
wxd_UIActionSimulator_Select(wxd_UIActionSimulator_t* sim, const char* text);

#endif // WXD_UIACTIONSIMULATOR_H
