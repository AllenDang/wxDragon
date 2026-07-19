#include <wx/wxprec.h>
#include <wx/wx.h>

#include <wx/propgrid/propgrid.h>
#include <wx/propgrid/props.h>

#include <cstdint>
#include <memory>
#include <new>

#include "../include/wxdragon.h"

namespace {
wxPropertyGrid*
as_grid(wxd_PropertyGrid_t* self)
{
    return reinterpret_cast<wxPropertyGrid*>(self);
}

const wxPropertyGrid*
as_grid(const wxd_PropertyGrid_t* self)
{
    return reinterpret_cast<const wxPropertyGrid*>(self);
}

const wxVariant*
as_variant(const wxd_Variant_t* value)
{
    return reinterpret_cast<const wxVariant*>(value);
}

wxString
from_utf8(const char* value)
{
    return WXD_STR_TO_WX_STRING_UTF8_NULL_OK(value);
}

wxString
property_name(const char* name)
{
    return (name && *name) ? wxString::FromUTF8(name) : wxPG_LABEL;
}

wxPGProperty*
find_property(wxPropertyGrid* grid, const char* name)
{
    if (!grid || !name || !*name)
        return nullptr;
    return grid->GetPropertyByName(wxString::FromUTF8(name));
}

const wxPGProperty*
find_property(const wxPropertyGrid* grid, const char* name)
{
    if (!grid || !name || !*name)
        return nullptr;
    return grid->GetPropertyByName(wxString::FromUTF8(name));
}

bool
property_name_available(wxPropertyGrid* grid, const char* label, const char* name)
{
    if (!grid)
        return false;

    // Passing wxPG_LABEL makes wxWidgets derive the name from the label.
    const char* effective_name = (name && *name) ? name : label;
    if (!effective_name || !*effective_name)
        return false;

    return grid->GetPropertyByName(wxString::FromUTF8(effective_name)) == nullptr;
}

bool
append_property(wxPropertyGrid* grid, const char* parent_name, const char* label,
                const char* name, std::unique_ptr<wxPGProperty> property)
{
    if (!grid || !property || !property_name_available(grid, label, name))
        return false;

    wxPGProperty* parent = nullptr;
    if (parent_name && *parent_name) {
        parent = find_property(grid, parent_name);
        if (!parent)
            return false;
    }

    wxPGProperty* raw = property.release();
    wxPGProperty* inserted = parent ? grid->AppendIn(parent, raw) : grid->Append(raw);
    if (!inserted) {
        // wxWidgets only takes ownership after successful insertion.
        delete raw;
        return false;
    }
    return true;
}

bool
make_choices(const char* const* labels, const int32_t* values, size_t choice_count,
             wxPGChoices& choices)
{
    if (choice_count > 0 && !labels)
        return false;

    for (size_t i = 0; i < choice_count; ++i) {
        if (!labels[i])
            return false;
        const int value = values ? static_cast<int>(values[i]) : static_cast<int>(i);
        choices.Add(wxString::FromUTF8(labels[i]), value);
    }
    return true;
}

wxPropertyGridEvent*
as_property_grid_event(wxd_Event_t* event)
{
    if (!event)
        return nullptr;
    wxEvent* base = reinterpret_cast<wxEvent*>(event);
    return dynamic_cast<wxPropertyGridEvent*>(base);
}
} // namespace

extern "C" WXD_EXPORTED wxd_PropertyGrid_t*
wxd_PropertyGrid_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                        wxd_Style_t style)
{
    if (!parent)
        return nullptr;

    wxPropertyGrid* grid = new (std::nothrow)
        wxPropertyGrid(reinterpret_cast<wxWindow*>(parent), id, wxPoint(pos.x, pos.y),
                       wxSize(size.width, size.height), static_cast<long>(style));
    return reinterpret_cast<wxd_PropertyGrid_t*>(grid);
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_Contains(wxd_PropertyGrid_t* self, const char* name)
{
    return find_property(as_grid(self), name) != nullptr;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendCategory(wxd_PropertyGrid_t* self, const char* parent_name,
                                const char* label, const char* name)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxPropertyCategory>(from_utf8(label),
                                                                 property_name(name)));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendString(wxd_PropertyGrid_t* self, const char* parent_name,
                              const char* label, const char* name, const char* value)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxStringProperty>(from_utf8(label),
                                                               property_name(name),
                                                               from_utf8(value)));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendInt(wxd_PropertyGrid_t* self, const char* parent_name,
                           const char* label, const char* name, int64_t value)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxIntProperty>(from_utf8(label),
                                                            property_name(name),
                                                            wxLongLong(value)));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendUInt(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name, uint64_t value)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxUIntProperty>(from_utf8(label),
                                                             property_name(name),
                                                             wxULongLong(value)));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendFloat(wxd_PropertyGrid_t* self, const char* parent_name,
                             const char* label, const char* name, double value)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxFloatProperty>(from_utf8(label),
                                                              property_name(name), value));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendBool(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name, bool value)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxBoolProperty>(from_utf8(label),
                                                             property_name(name), value));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendEnum(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name,
                            const char* const* labels, const int32_t* values,
                            size_t choice_count, int32_t value)
{
    wxPGChoices choices;
    if (!make_choices(labels, values, choice_count, choices))
        return false;

    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxEnumProperty>(from_utf8(label),
                                                             property_name(name), choices,
                                                             static_cast<int>(value)));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendFlags(wxd_PropertyGrid_t* self, const char* parent_name,
                             const char* label, const char* name,
                             const char* const* labels, const int32_t* values,
                             size_t choice_count, int32_t value)
{
    wxPGChoices choices;
    if (!make_choices(labels, values, choice_count, choices))
        return false;

    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxFlagsProperty>(from_utf8(label),
                                                              property_name(name), choices,
                                                              static_cast<long>(value)));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendFile(wxd_PropertyGrid_t* self, const char* parent_name,
                            const char* label, const char* name, const char* value)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxFileProperty>(from_utf8(label),
                                                             property_name(name),
                                                             from_utf8(value)));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_AppendDir(wxd_PropertyGrid_t* self, const char* parent_name,
                           const char* label, const char* name, const char* value)
{
    wxPropertyGrid* grid = as_grid(self);
    return append_property(grid, parent_name, label, name,
                           std::make_unique<wxDirProperty>(from_utf8(label),
                                                            property_name(name),
                                                            from_utf8(value)));
}

extern "C" WXD_EXPORTED wxd_Variant_t*
wxd_PropertyGrid_GetValue(const wxd_PropertyGrid_t* self, const char* name)
{
    const wxPropertyGrid* grid = as_grid(self);
    const wxPGProperty* property = find_property(grid, name);
    if (!property)
        return nullptr;

    wxVariant* value = new (std::nothrow) wxVariant(property->GetValue());
    return reinterpret_cast<wxd_Variant_t*>(value);
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_SetValue(wxd_PropertyGrid_t* self, const char* name,
                          const wxd_Variant_t* value)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    const wxVariant* variant = as_variant(value);
    if (!property || !variant)
        return false;
    grid->SetPropertyValue(property, *variant);
    return true;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_ChangeValue(wxd_PropertyGrid_t* self, const char* name,
                             const wxd_Variant_t* value)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    const wxVariant* variant = as_variant(value);
    if (!property || !variant)
        return false;
    return grid->ChangePropertyValue(property, *variant);
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_ClearValue(wxd_PropertyGrid_t* self, const char* name)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    if (!property)
        return false;
    grid->SetPropertyValueUnspecified(property);
    return true;
}

extern "C" WXD_EXPORTED int
wxd_PropertyGrid_GetValueAsString(const wxd_PropertyGrid_t* self, const char* name,
                                  char* out, size_t out_len)
{
    const wxPropertyGrid* grid = as_grid(self);
    const wxPGProperty* property = find_property(grid, name);
    if (!property) {
        if (out && out_len)
            out[0] = '\0';
        return -1;
    }
    return static_cast<int>(
        wxd_cpp_utils::copy_wxstring_to_buffer(property->GetValueAsString(), out, out_len));
}

extern "C" WXD_EXPORTED int
wxd_PropertyGrid_GetLabel(const wxd_PropertyGrid_t* self, const char* name,
                          char* out, size_t out_len)
{
    const wxPGProperty* property = find_property(as_grid(self), name);
    if (!property) {
        if (out && out_len)
            out[0] = '\0';
        return -1;
    }
    return static_cast<int>(
        wxd_cpp_utils::copy_wxstring_to_buffer(property->GetLabel(), out, out_len));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_SetLabel(wxd_PropertyGrid_t* self, const char* name, const char* label)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    if (!property)
        return false;
    grid->SetPropertyLabel(property, from_utf8(label));
    return true;
}

extern "C" WXD_EXPORTED int
wxd_PropertyGrid_GetHelpString(const wxd_PropertyGrid_t* self, const char* name,
                               char* out, size_t out_len)
{
    const wxPGProperty* property = find_property(as_grid(self), name);
    if (!property) {
        if (out && out_len)
            out[0] = '\0';
        return -1;
    }
    return static_cast<int>(
        wxd_cpp_utils::copy_wxstring_to_buffer(property->GetHelpString(), out, out_len));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_SetHelpString(wxd_PropertyGrid_t* self, const char* name,
                               const char* help_string)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    if (!property)
        return false;
    grid->SetPropertyHelpString(property, from_utf8(help_string));
    return true;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_SetAttribute(wxd_PropertyGrid_t* self, const char* name,
                              const char* attribute_name, const wxd_Variant_t* value)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    const wxVariant* variant = as_variant(value);
    if (!property || !attribute_name || !*attribute_name || !variant)
        return false;
    grid->SetPropertyAttribute(property, wxString::FromUTF8(attribute_name), *variant);
    return true;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_EnableProperty(wxd_PropertyGrid_t* self, const char* name, bool enable)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    return property ? grid->EnableProperty(property, enable) : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_HideProperty(wxd_PropertyGrid_t* self, const char* name, bool hide)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    return property ? grid->HideProperty(property, hide) : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_SetPropertyReadOnly(wxd_PropertyGrid_t* self, const char* name,
                                     bool read_only)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    if (!property)
        return false;
    grid->SetPropertyReadOnly(property, read_only);
    return true;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyEnabled(const wxd_PropertyGrid_t* self, const char* name)
{
    const wxPGProperty* property = find_property(as_grid(self), name);
    return property ? property->IsEnabled() : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyHidden(const wxd_PropertyGrid_t* self, const char* name)
{
    const wxPGProperty* property = find_property(as_grid(self), name);
    return property ? property->HasFlag(wxPGFlags::Hidden) : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyExpanded(const wxd_PropertyGrid_t* self, const char* name)
{
    const wxPGProperty* property = find_property(as_grid(self), name);
    return property ? property->IsExpanded() : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyCategory(const wxd_PropertyGrid_t* self, const char* name)
{
    const wxPGProperty* property = find_property(as_grid(self), name);
    return property ? property->IsCategory() : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_IsPropertyModified(const wxd_PropertyGrid_t* self, const char* name)
{
    const wxPropertyGrid* grid = as_grid(self);
    const wxPGProperty* property = find_property(grid, name);
    return property ? grid->IsPropertyModified(const_cast<wxPGProperty*>(property)) : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_Expand(wxd_PropertyGrid_t* self, const char* name)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    return property ? grid->Expand(property) : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_Collapse(wxd_PropertyGrid_t* self, const char* name)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    return property ? grid->Collapse(property) : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_ExpandAll(wxd_PropertyGrid_t* self, bool expand)
{
    wxPropertyGrid* grid = as_grid(self);
    return grid ? grid->ExpandAll(expand) : false;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_SelectProperty(wxd_PropertyGrid_t* self, const char* name, bool focus)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    return property ? grid->SelectProperty(property, focus) : false;
}

extern "C" WXD_EXPORTED int
wxd_PropertyGrid_GetSelectedPropertyName(const wxd_PropertyGrid_t* self,
                                         char* out, size_t out_len)
{
    const wxPropertyGrid* grid = as_grid(self);
    const wxPGProperty* property = grid ? grid->GetSelectedProperty() : nullptr;
    if (!property) {
        if (out && out_len)
            out[0] = '\0';
        return -1;
    }
    return static_cast<int>(
        wxd_cpp_utils::copy_wxstring_to_buffer(property->GetName(), out, out_len));
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_DeleteProperty(wxd_PropertyGrid_t* self, const char* name)
{
    wxPropertyGrid* grid = as_grid(self);
    wxPGProperty* property = find_property(grid, name);
    if (!property)
        return false;
    grid->DeleteProperty(property);
    return true;
}

extern "C" WXD_EXPORTED void
wxd_PropertyGrid_Clear(wxd_PropertyGrid_t* self)
{
    wxPropertyGrid* grid = as_grid(self);
    if (grid)
        grid->Clear();
}

extern "C" WXD_EXPORTED void
wxd_PropertyGrid_ClearModifiedStatus(wxd_PropertyGrid_t* self)
{
    wxPropertyGrid* grid = as_grid(self);
    if (grid)
        grid->ClearModifiedStatus();
}

extern "C" WXD_EXPORTED int
wxd_PropertyGrid_GetSplitterPosition(const wxd_PropertyGrid_t* self, unsigned int column)
{
    const wxPropertyGrid* grid = as_grid(self);
    return grid ? grid->GetSplitterPosition(column) : -1;
}

extern "C" WXD_EXPORTED void
wxd_PropertyGrid_SetSplitterPosition(wxd_PropertyGrid_t* self, int position,
                                     unsigned int column)
{
    wxPropertyGrid* grid = as_grid(self);
    if (grid)
        grid->SetSplitterPosition(position, static_cast<int>(column));
}

extern "C" WXD_EXPORTED int
wxd_PropertyGrid_GetColumnProportion(const wxd_PropertyGrid_t* self, unsigned int column)
{
    const wxPropertyGrid* grid = as_grid(self);
    return grid ? grid->GetColumnProportion(column) : -1;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGrid_SetColumnProportion(wxd_PropertyGrid_t* self, unsigned int column,
                                     int proportion)
{
    wxPropertyGrid* grid = as_grid(self);
    return grid ? grid->SetColumnProportion(column, proportion) : false;
}

extern "C" WXD_EXPORTED void
wxd_PropertyGrid_CenterSplitter(wxd_PropertyGrid_t* self, bool enable_auto_resizing)
{
    wxPropertyGrid* grid = as_grid(self);
    if (grid)
        grid->CenterSplitter(enable_auto_resizing);
}

extern "C" WXD_EXPORTED void
wxd_PropertyGrid_Refresh(wxd_PropertyGrid_t* self)
{
    wxPropertyGrid* grid = as_grid(self);
    if (grid)
        grid->Refresh();
}

extern "C" WXD_EXPORTED int
wxd_PropertyGridEvent_GetPropertyName(wxd_Event_t* event, char* out, size_t out_len)
{
    wxPropertyGridEvent* property_event = as_property_grid_event(event);
    if (!property_event) {
        if (out && out_len)
            out[0] = '\0';
        return -1;
    }
    return static_cast<int>(wxd_cpp_utils::copy_wxstring_to_buffer(
        property_event->GetPropertyName(), out, out_len));
}

extern "C" WXD_EXPORTED wxd_Variant_t*
wxd_PropertyGridEvent_GetValue(wxd_Event_t* event)
{
    wxPropertyGridEvent* property_event = as_property_grid_event(event);
    if (!property_event)
        return nullptr;
    wxVariant* value = new (std::nothrow) wxVariant(property_event->GetValue());
    return reinterpret_cast<wxd_Variant_t*>(value);
}

extern "C" WXD_EXPORTED unsigned int
wxd_PropertyGridEvent_GetColumn(wxd_Event_t* event)
{
    wxPropertyGridEvent* property_event = as_property_grid_event(event);
    return property_event ? property_event->GetColumn() : 0;
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGridEvent_CanVeto(wxd_Event_t* event)
{
    wxPropertyGridEvent* property_event = as_property_grid_event(event);
    return property_event ? property_event->CanVeto() : false;
}

extern "C" WXD_EXPORTED void
wxd_PropertyGridEvent_Veto(wxd_Event_t* event, bool veto)
{
    wxPropertyGridEvent* property_event = as_property_grid_event(event);
    if (property_event && property_event->CanVeto())
        property_event->Veto(veto);
}

extern "C" WXD_EXPORTED bool
wxd_PropertyGridEvent_WasVetoed(wxd_Event_t* event)
{
    wxPropertyGridEvent* property_event = as_property_grid_event(event);
    return property_event ? property_event->WasVetoed() : false;
}
