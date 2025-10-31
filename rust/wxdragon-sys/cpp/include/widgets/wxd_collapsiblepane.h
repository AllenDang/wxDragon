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

/**
 * @brief Get the label of the CollapsiblePane
 * Returns the required UTF-8 byte length (excluding the null terminator), if any error returned -1.
 * If out is not null and out_len > 0, copies up to out_len - 1 bytes and null-terminates.
 * If out is null or out_len == 0, nothing is written.
 */
WXD_EXPORTED int
wxd_CollapsiblePane_GetLabel(const wxd_CollapsiblePane_t* self, char* out, size_t out_len);

#ifdef __cplusplus
}
#endif

#endif // WXD_COLLAPSIBLEPANE_H