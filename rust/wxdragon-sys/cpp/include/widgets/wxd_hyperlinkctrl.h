#ifndef WXD_HYPERLINKCTRL_H
#define WXD_HYPERLINKCTRL_H

#include "../wxd_types.h"

// --- HyperlinkCtrl Functions ---
WXD_EXPORTED wxd_HyperlinkCtrl_t*
wxd_HyperlinkCtrl_Create(wxd_Window_t* parent, int id, const char* label, const char* url, int x,
                         int y, int w, int h, int64_t style);

/**
 * Get URL string.
 * Returns the length of the string (excluding the null terminator), if any error, -1 returned.
 */
WXD_EXPORTED int
wxd_HyperlinkCtrl_GetURL(const wxd_HyperlinkCtrl_t* self, char* buf, size_t buf_len);

WXD_EXPORTED void
wxd_HyperlinkCtrl_SetURL(wxd_HyperlinkCtrl_t* self, const char* url);
WXD_EXPORTED bool
wxd_HyperlinkCtrl_GetVisited(wxd_HyperlinkCtrl_t* self);
WXD_EXPORTED void
wxd_HyperlinkCtrl_SetVisited(wxd_HyperlinkCtrl_t* self, bool visited);
WXD_EXPORTED unsigned long
wxd_HyperlinkCtrl_GetHoverColour(wxd_HyperlinkCtrl_t* self);
WXD_EXPORTED void
wxd_HyperlinkCtrl_SetHoverColour(wxd_HyperlinkCtrl_t* self, unsigned long colour);
WXD_EXPORTED unsigned long
wxd_HyperlinkCtrl_GetNormalColour(wxd_HyperlinkCtrl_t* self);
WXD_EXPORTED void
wxd_HyperlinkCtrl_SetNormalColour(wxd_HyperlinkCtrl_t* self, unsigned long colour);
WXD_EXPORTED unsigned long
wxd_HyperlinkCtrl_GetVisitedColour(wxd_HyperlinkCtrl_t* self);
WXD_EXPORTED void
wxd_HyperlinkCtrl_SetVisitedColour(wxd_HyperlinkCtrl_t* self, unsigned long colour);

#endif // WXD_HYPERLINKCTRL_H