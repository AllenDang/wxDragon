#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

WXD_EXPORTED void
wxd_ColourDatabase_UseScheme(wxd_ColourDatabaseScheme scheme)
{
    if (!wxTheColourDatabase)
        return;
    wxTheColourDatabase->UseScheme(scheme == WXD_COLOUR_DATABASE_SCHEME_TRADITIONAL ? wxColourDatabase::Traditional
                                                                                     : wxColourDatabase::CSS);
}

WXD_EXPORTED bool
wxd_ColourDatabase_Find(const char* name, wxd_Colour_t* out_colour)
{
    if (!wxTheColourDatabase || !name || !out_colour)
        return false;

    wxColour colour = wxTheColourDatabase->Find(wxString::FromUTF8(name));
    if (!colour.IsOk())
        return false;

    *out_colour = { colour.Red(), colour.Green(), colour.Blue(), colour.Alpha() };
    return true;
}

WXD_EXPORTED int
wxd_ColourDatabase_FindName(wxd_Colour_t colour, char* buffer, size_t buffer_len)
{
    if (!wxTheColourDatabase)
        return (int)wxd_cpp_utils::copy_wxstring_to_buffer(wxString(), buffer, buffer_len);

    wxColour wx_colour(colour.r, colour.g, colour.b, colour.a);
    wxString name = wxTheColourDatabase->FindName(wx_colour);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(name, buffer, buffer_len);
}

WXD_EXPORTED void
wxd_ColourDatabase_AddColour(const char* name, wxd_Colour_t colour)
{
    if (!wxTheColourDatabase || !name)
        return;
    wxTheColourDatabase->AddColour(wxString::FromUTF8(name), wxColour(colour.r, colour.g, colour.b, colour.a));
}

WXD_EXPORTED wxd_ArrayString_t*
wxd_ColourDatabase_GetAllNames(void)
{
    wxArrayString* names = new (std::nothrow) wxArrayString();
    if (!names)
        return nullptr;
    if (wxTheColourDatabase) {
        for (const wxString& name : wxTheColourDatabase->GetAllNames())
            names->Add(name);
    }
    return reinterpret_cast<wxd_ArrayString_t*>(names);
}

} // extern "C"
