#ifndef WXD_TREEBOOK_H
#define WXD_TREEBOOK_H

#include "../wxd_types.h"

// --- Treebook Functions ---
WXD_EXPORTED wxd_Treebook_t*
wxd_Treebook_new(wxd_Window_t* parent, int id, int x, int y, int width, int height,
                 wxd_Style_t style);

WXD_EXPORTED int
wxd_Treebook_AddPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int select,
                     int imageId);

WXD_EXPORTED int
wxd_Treebook_AddSubPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int select,
                        int imageId);

WXD_EXPORTED int
wxd_Treebook_GetPageCount(wxd_Treebook_t* self);

WXD_EXPORTED int
wxd_Treebook_GetSelection(wxd_Treebook_t* self);

WXD_EXPORTED int
wxd_Treebook_SetSelection(wxd_Treebook_t* self, size_t n);

WXD_EXPORTED void
wxd_Treebook_SetPageText(wxd_Treebook_t* self, size_t n, const char* strText);

WXD_EXPORTED int
wxd_Treebook_GetPageText(wxd_Treebook_t* self, size_t n, char* buffer, int bufLen);

#endif // WXD_TREEBOOK_H