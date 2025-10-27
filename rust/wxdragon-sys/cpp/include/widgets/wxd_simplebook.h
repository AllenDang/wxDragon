#ifndef WXD_SIMPLEBOOK_H
#define WXD_SIMPLEBOOK_H

#include "../wxdragon.h" // For WXD_EXPORTED, wxd_Window_t, wxd_Id, wxd_Point, wxd_Size, wxd_Style_t

// --- SimpleBook ---
WXD_EXPORTED wxd_SimpleBook_t*
wxd_SimpleBook_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                      wxd_Style_t style);

WXD_EXPORTED bool
wxd_SimpleBook_AddPage(wxd_SimpleBook_t* self, wxd_Window_t* page, const char* text, bool select);

WXD_EXPORTED int
wxd_SimpleBook_GetSelection(wxd_SimpleBook_t* self);

WXD_EXPORTED int
wxd_SimpleBook_SetSelection(wxd_SimpleBook_t* self, int page);

WXD_EXPORTED size_t
wxd_SimpleBook_GetPageCount(wxd_SimpleBook_t* self);

WXD_EXPORTED wxd_Window_t*
wxd_SimpleBook_GetPage(wxd_SimpleBook_t* self, size_t n);

WXD_EXPORTED bool
wxd_SimpleBook_RemovePage(wxd_SimpleBook_t* self, size_t n);

WXD_EXPORTED int
wxd_SimpleBook_ChangeSelection(wxd_SimpleBook_t* self, size_t page);

WXD_EXPORTED bool
wxd_SimpleBook_InsertPage(wxd_SimpleBook_t* self, size_t index, wxd_Window_t* page,
                          const char* text, bool select);

// SimpleBook doesn't typically use images since it has no visual tabs
// but we can still support the API for consistency
WXD_EXPORTED bool
wxd_SimpleBook_AddPageWithImageId(wxd_SimpleBook_t* self, wxd_Window_t* page, const char* text,
                                  bool select, int imageId);

WXD_EXPORTED bool
wxd_SimpleBook_InsertPageWithImageId(wxd_SimpleBook_t* self, size_t index, wxd_Window_t* page,
                                     const char* text, bool select, int imageId);

#endif // WXD_SIMPLEBOOK_H