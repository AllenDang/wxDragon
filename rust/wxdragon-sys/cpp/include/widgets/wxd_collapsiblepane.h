#ifndef WXD_COLLAPSIBLEPANE_H
#define WXD_COLLAPSIBLEPANE_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// --- CollapsiblePane Functions ---
WXD_EXPORTED wxd_CollapsiblePane_t*
wxd_CollapsiblePane_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                           wxd_Size size, wxd_Style_t style, const char* name);
WXD_EXPORTED bool
wxd_CollapsiblePane_IsExpanded(wxd_CollapsiblePane_t* self);
WXD_EXPORTED bool
wxd_CollapsiblePane_IsCollapsed(wxd_CollapsiblePane_t* self);
WXD_EXPORTED void
wxd_CollapsiblePane_Expand(wxd_CollapsiblePane_t* self, bool expand);
WXD_EXPORTED void
wxd_CollapsiblePane_Collapse(wxd_CollapsiblePane_t* self, bool collapse);
WXD_EXPORTED wxd_Window_t*
wxd_CollapsiblePane_GetPane(wxd_CollapsiblePane_t* self);
WXD_EXPORTED void
wxd_CollapsiblePane_SetLabel(wxd_CollapsiblePane_t* self, const char* label);
WXD_EXPORTED char*
wxd_CollapsiblePane_GetLabel(wxd_CollapsiblePane_t* self);

#ifdef __cplusplus
}
#endif

#endif // WXD_COLLAPSIBLEPANE_H