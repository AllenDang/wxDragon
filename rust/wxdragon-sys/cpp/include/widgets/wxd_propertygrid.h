#ifndef WXD_PROPERTYGRID_H
#define WXD_PROPERTYGRID_H

#include "../wxd_types.h"
#include "../wxd_variant.h"

// wxPropertyGrid window styles. Manager-only styles are intentionally omitted.
#define WXD_PG_DEFAULT_STYLE 0x00000000L
#define WXD_PG_AUTO_SORT 0x00000010L
#define WXD_PG_HIDE_CATEGORIES 0x00000020L
#define WXD_PG_ALPHABETIC_MODE (WXD_PG_HIDE_CATEGORIES | WXD_PG_AUTO_SORT)
#define WXD_PG_BOLD_MODIFIED 0x00000040L
#define WXD_PG_SPLITTER_AUTO_CENTER 0x00000080L
#define WXD_PG_TOOLTIPS 0x00000100L
#define WXD_PG_HIDE_MARGIN 0x00000200L
#define WXD_PG_STATIC_SPLITTER 0x00000400L
#define WXD_PG_STATIC_LAYOUT (WXD_PG_HIDE_MARGIN | WXD_PG_STATIC_SPLITTER)
#define WXD_PG_LIMITED_EDITING 0x00000800L

#ifdef __cplusplus
extern "C" {
#endif

/** Create a wxPropertyGrid owned by parent. */
WXD_EXPORTED wxd_PropertyGrid_t*
wxd_PropertyGrid_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                        wxd_Style_t style);

/** Return whether a property with this case-sensitive name exists. */
WXD_EXPORTED bool
wxd_PropertyGrid_Contains(wxd_PropertyGrid_t* self, const char* name);

/*
 * Property insertion.
 *
 * parent_name may be NULL or empty to append at the root. name may be NULL or
 * empty, in which case wxWidgets derives the property name from label. The grid
 * takes ownership of every successfully appended property.
 */
WXD_EXPORTED bool
wxd_PropertyGrid_AppendCategory(wxd_PropertyGrid_t* self, const char* parent_name,
                                const char* label, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_AppendString(wxd_PropertyGrid_t* self, const char* parent_name,
                              const char* label, const char* name, const char* value);

WXD_EXPORTED bool
wxd_PropertyGrid_AppendInt(wxd_PropertyGrid_t* self, const char* parent_name,
                           const char* label, const char* name, int64_t value);

WXD_EXPORTED bool
wxd_PropertyGrid_AppendUInt(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name, uint64_t value);

WXD_EXPORTED bool
wxd_PropertyGrid_AppendFloat(wxd_PropertyGrid_t* self, const char* parent_name,
                             const char* label, const char* name, double value);

WXD_EXPORTED bool
wxd_PropertyGrid_AppendBool(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name, bool value);

/**
 * Append an enum property. values may be NULL to use 0..choice_count-1.
 * Each labels entry must be a valid UTF-8 string.
 */
WXD_EXPORTED bool
wxd_PropertyGrid_AppendEnum(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name,
                            const char* const* labels, const int32_t* values,
                            size_t choice_count, int32_t value);

/**
 * Append a bit-flags property. wxFlagsProperty uses a 32-bit-compatible value
 * set on Windows, so flag values and the combined value are explicitly int32_t.
 */
WXD_EXPORTED bool
wxd_PropertyGrid_AppendFlags(wxd_PropertyGrid_t* self, const char* parent_name,
                             const char* label, const char* name,
                             const char* const* labels, const int32_t* values,
                             size_t choice_count, int32_t value);

WXD_EXPORTED bool
wxd_PropertyGrid_AppendFile(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name, const char* value);

WXD_EXPORTED bool
wxd_PropertyGrid_AppendDir(wxd_PropertyGrid_t* self, const char* parent_name,
                           const char* label, const char* name, const char* value);

/* Value access. Returned variants are owned by the caller. */
WXD_EXPORTED wxd_Variant_t*
wxd_PropertyGrid_GetValue(const wxd_PropertyGrid_t* self, const char* name);

/** Set without validation or a change event. */
WXD_EXPORTED bool
wxd_PropertyGrid_SetValue(wxd_PropertyGrid_t* self, const char* name,
                          const wxd_Variant_t* value);

/** Set through validation and emit the normal property-changing events. */
WXD_EXPORTED bool
wxd_PropertyGrid_ChangeValue(wxd_PropertyGrid_t* self, const char* name,
                             const wxd_Variant_t* value);

/** Make a property's value unspecified/null. */
WXD_EXPORTED bool
wxd_PropertyGrid_ClearValue(wxd_PropertyGrid_t* self, const char* name);

/** Get the displayed value as UTF-8 using the standard two-call buffer API. */
WXD_EXPORTED int
wxd_PropertyGrid_GetValueAsString(const wxd_PropertyGrid_t* self, const char* name,
                                  char* out, size_t out_len);

/* Property metadata and state. */
WXD_EXPORTED int
wxd_PropertyGrid_GetLabel(const wxd_PropertyGrid_t* self, const char* name,
                          char* out, size_t out_len);

WXD_EXPORTED bool
wxd_PropertyGrid_SetLabel(wxd_PropertyGrid_t* self, const char* name, const char* label);

WXD_EXPORTED int
wxd_PropertyGrid_GetHelpString(const wxd_PropertyGrid_t* self, const char* name,
                               char* out, size_t out_len);

WXD_EXPORTED bool
wxd_PropertyGrid_SetHelpString(wxd_PropertyGrid_t* self, const char* name,
                               const char* help_string);

WXD_EXPORTED bool
wxd_PropertyGrid_SetAttribute(wxd_PropertyGrid_t* self, const char* name,
                              const char* attribute_name, const wxd_Variant_t* value);

WXD_EXPORTED bool
wxd_PropertyGrid_EnableProperty(wxd_PropertyGrid_t* self, const char* name, bool enable);

WXD_EXPORTED bool
wxd_PropertyGrid_HideProperty(wxd_PropertyGrid_t* self, const char* name, bool hide);

WXD_EXPORTED bool
wxd_PropertyGrid_SetPropertyReadOnly(wxd_PropertyGrid_t* self, const char* name,
                                     bool read_only);

WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyEnabled(const wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyHidden(const wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyExpanded(const wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyCategory(const wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyModified(const wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_Expand(wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_Collapse(wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED bool
wxd_PropertyGrid_ExpandAll(wxd_PropertyGrid_t* self, bool expand);

WXD_EXPORTED bool
wxd_PropertyGrid_SelectProperty(wxd_PropertyGrid_t* self, const char* name, bool focus);

WXD_EXPORTED int
wxd_PropertyGrid_GetSelectedPropertyName(const wxd_PropertyGrid_t* self,
                                         char* out, size_t out_len);

WXD_EXPORTED bool
wxd_PropertyGrid_DeleteProperty(wxd_PropertyGrid_t* self, const char* name);

WXD_EXPORTED void
wxd_PropertyGrid_Clear(wxd_PropertyGrid_t* self);

WXD_EXPORTED void
wxd_PropertyGrid_ClearModifiedStatus(wxd_PropertyGrid_t* self);

/* Layout helpers. */
WXD_EXPORTED int
wxd_PropertyGrid_GetSplitterPosition(const wxd_PropertyGrid_t* self, unsigned int column);

WXD_EXPORTED void
wxd_PropertyGrid_SetSplitterPosition(wxd_PropertyGrid_t* self, int position,
                                     unsigned int column);

WXD_EXPORTED int
wxd_PropertyGrid_GetColumnProportion(const wxd_PropertyGrid_t* self, unsigned int column);

WXD_EXPORTED bool
wxd_PropertyGrid_SetColumnProportion(wxd_PropertyGrid_t* self, unsigned int column,
                                     int proportion);

WXD_EXPORTED void
wxd_PropertyGrid_CenterSplitter(wxd_PropertyGrid_t* self, bool enable_auto_resizing);

WXD_EXPORTED void
wxd_PropertyGrid_Refresh(wxd_PropertyGrid_t* self);

/* wxPropertyGridEvent accessors. Event values are cloned for the caller. */
WXD_EXPORTED int
wxd_PropertyGridEvent_GetPropertyName(wxd_Event_t* event, char* out, size_t out_len);

WXD_EXPORTED wxd_Variant_t*
wxd_PropertyGridEvent_GetValue(wxd_Event_t* event);

WXD_EXPORTED unsigned int
wxd_PropertyGridEvent_GetColumn(wxd_Event_t* event);

WXD_EXPORTED bool
wxd_PropertyGridEvent_CanVeto(wxd_Event_t* event);

WXD_EXPORTED void
wxd_PropertyGridEvent_Veto(wxd_Event_t* event, bool veto);

WXD_EXPORTED bool
wxd_PropertyGridEvent_WasVetoed(wxd_Event_t* event);

#ifdef __cplusplus
}
#endif

#endif // WXD_PROPERTYGRID_H
