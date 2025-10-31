#ifndef WXD_CLIPBOARD_H
#define WXD_CLIPBOARD_H

#include "../wxd_types.h"

// --- Clipboard Functions ---
WXD_EXPORTED wxd_Clipboard_t*
wxd_Clipboard_Get();
WXD_EXPORTED bool
wxd_Clipboard_Open(wxd_Clipboard_t* clipboard);
WXD_EXPORTED void
wxd_Clipboard_Close(wxd_Clipboard_t* clipboard);
WXD_EXPORTED bool
wxd_Clipboard_IsOpened(wxd_Clipboard_t* clipboard);
WXD_EXPORTED bool
wxd_Clipboard_AddData(wxd_Clipboard_t* clipboard, wxd_DataObject_t* data);
WXD_EXPORTED bool
wxd_Clipboard_SetData(wxd_Clipboard_t* clipboard, wxd_DataObject_t* data);
WXD_EXPORTED bool
wxd_Clipboard_IsSupported(wxd_Clipboard_t* clipboard, int format);
WXD_EXPORTED bool
wxd_Clipboard_GetData(wxd_Clipboard_t* clipboard, wxd_DataObject_t* data);
WXD_EXPORTED void
wxd_Clipboard_Clear(wxd_Clipboard_t* clipboard);
WXD_EXPORTED bool
wxd_Clipboard_Flush(wxd_Clipboard_t* clipboard);
WXD_EXPORTED void
wxd_Clipboard_UsePrimarySelection(wxd_Clipboard_t* clipboard, bool use_primary);

// --- Convenience Functions ---
WXD_EXPORTED bool
wxd_Clipboard_SetText(wxd_Clipboard_t* clipboard, const char* text);

/**
 * Gets text from the clipboard.
 * Returns the length of the text retrieved (not including the null terminator), or -1 on error.
 * If buffer is not null and buffer_len is non-zero, copies up to buffer_len-1 characters into buffer,
 * null-terminating it.
 */
WXD_EXPORTED int
wxd_Clipboard_GetText(const wxd_Clipboard_t* clipboard, char* buffer, size_t buffer_len);

#endif // WXD_CLIPBOARD_H