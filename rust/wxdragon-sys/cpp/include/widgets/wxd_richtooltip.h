#ifndef WXD_RICHTOOLTIP_H
#define WXD_RICHTOOLTIP_H

#include "../wxdragon.h"

#ifdef __cplusplus
extern "C" {
#endif

WXD_EXPORTED wxd_RichToolTip_t*
wxd_RichToolTip_Create(const char* title, const char* message);

WXD_EXPORTED void
wxd_RichToolTip_Destroy(wxd_RichToolTip_t* self);

WXD_EXPORTED void
wxd_RichToolTip_SetBackgroundColour(wxd_RichToolTip_t* self, const wxd_Colour_t* col, const wxd_Colour_t* colEnd);

WXD_EXPORTED void
wxd_RichToolTip_SetTitleFont(wxd_RichToolTip_t* self, const wxd_Font_t* font);

WXD_EXPORTED void
wxd_RichToolTip_SetIcon(wxd_RichToolTip_t* self, int icon);

WXD_EXPORTED void
wxd_RichToolTip_SetCustomIcon(wxd_RichToolTip_t* self, const wxd_BitmapBundle_t* icon);

WXD_EXPORTED void
wxd_RichToolTip_SetTimeout(wxd_RichToolTip_t* self, unsigned int milliseconds, unsigned int millisecondsShowdelay);

WXD_EXPORTED void
wxd_RichToolTip_SetTipKind(wxd_RichToolTip_t* self, int tipKind);

WXD_EXPORTED void
wxd_RichToolTip_ShowFor(wxd_RichToolTip_t* self, wxd_Window_t* win, const wxd_Rect* rect);

#ifdef __cplusplus
}
#endif

#endif // WXD_RICHTOOLTIP_H
