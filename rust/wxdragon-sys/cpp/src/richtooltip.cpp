#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#if wxUSE_RICHTOOLTIP
#include <wx/richtooltip.h>
#include <wx/bmpbndl.h>
#include "wxd_utils.h"

extern "C" {

WXD_EXPORTED wxd_RichToolTip_t*
wxd_RichToolTip_Create(const char* title, const char* message)
{
	wxString wxTitle = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title);
	wxString wxMessage = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message);
	wxRichToolTip* tip = new wxRichToolTip(wxTitle, wxMessage);
	return reinterpret_cast<wxd_RichToolTip_t*>(tip);
}

WXD_EXPORTED void
wxd_RichToolTip_Destroy(wxd_RichToolTip_t* self)
{
	if (self) {
		delete reinterpret_cast<wxRichToolTip*>(self);
	}
}

WXD_EXPORTED void
wxd_RichToolTip_SetBackgroundColour(wxd_RichToolTip_t* self, const wxd_Colour_t* col, const wxd_Colour_t* colEnd)
{
	if (!self || !col)
		return;
	wxColour c1(col->r, col->g, col->b, col->a);
	wxColour c2 = colEnd ? wxColour(colEnd->r, colEnd->g, colEnd->b, colEnd->a) : wxColour();
	reinterpret_cast<wxRichToolTip*>(self)->SetBackgroundColour(c1, c2);
}

WXD_EXPORTED void
wxd_RichToolTip_SetTitleFont(wxd_RichToolTip_t* self, const wxd_Font_t* font)
{
	if (!self || !font)
		return;
	const wxFont* f = reinterpret_cast<const wxFont*>(font);
	reinterpret_cast<wxRichToolTip*>(self)->SetTitleFont(*f);
}

WXD_EXPORTED void
wxd_RichToolTip_SetIcon(wxd_RichToolTip_t* self, int icon)
{
	if (!self)
		return;
	reinterpret_cast<wxRichToolTip*>(self)->SetIcon(icon);
}

WXD_EXPORTED void
wxd_RichToolTip_SetCustomIcon(wxd_RichToolTip_t* self, const wxd_BitmapBundle_t* icon)
{
	if (!self || !icon)
		return;
	const wxBitmapBundle* bundle = reinterpret_cast<const wxBitmapBundle*>(icon);
	reinterpret_cast<wxRichToolTip*>(self)->SetIcon(*bundle);
}

WXD_EXPORTED void
wxd_RichToolTip_SetTimeout(wxd_RichToolTip_t* self, unsigned int milliseconds, unsigned int millisecondsShowdelay)
{
	if (!self)
		return;
	reinterpret_cast<wxRichToolTip*>(self)->SetTimeout(milliseconds, millisecondsShowdelay);
}

WXD_EXPORTED void
wxd_RichToolTip_SetTipKind(wxd_RichToolTip_t* self, int tipKind)
{
	if (!self)
		return;
	reinterpret_cast<wxRichToolTip*>(self)->SetTipKind(static_cast<wxTipKind>(tipKind));
}

WXD_EXPORTED void
wxd_RichToolTip_ShowFor(wxd_RichToolTip_t* self, wxd_Window_t* win, const wxd_Rect* rect)
{
	if (!self || !win)
		return;
	wxWindow* w = reinterpret_cast<wxWindow*>(win);
	if (rect) {
		wxRect r(rect->x, rect->y, rect->width, rect->height);
		reinterpret_cast<wxRichToolTip*>(self)->ShowFor(w, &r);
	} else {
		reinterpret_cast<wxRichToolTip*>(self)->ShowFor(w);
	}
}

}

#endif // wxUSE_RICHTOOLTIP
