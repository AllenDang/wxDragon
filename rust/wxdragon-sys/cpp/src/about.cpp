#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/aboutdlg.h>
#include "../include/wxdragon.h"

extern "C" {

wxd_AboutDialogInfo_t*
wxd_AboutDialogInfo_Create(void)
{
    return reinterpret_cast<wxd_AboutDialogInfo_t*>(new wxAboutDialogInfo());
}

void
wxd_AboutDialogInfo_Destroy(wxd_AboutDialogInfo_t* info)
{
    if (!info)
        return;
    delete reinterpret_cast<wxAboutDialogInfo*>(info);
}

void
wxd_AboutDialogInfo_SetName(wxd_AboutDialogInfo_t* info, const char* name)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetName(wxString::FromUTF8(name ? name : ""));
}

void
wxd_AboutDialogInfo_SetVersion(wxd_AboutDialogInfo_t* info, const char* version)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetVersion(wxString::FromUTF8(version ? version : ""));
}

void
wxd_AboutDialogInfo_SetVersionEx(wxd_AboutDialogInfo_t* info, const char* version,
                                  const char* longVersion)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetVersion(wxString::FromUTF8(version ? version : ""),
                          wxString::FromUTF8(longVersion ? longVersion : ""));
}

void
wxd_AboutDialogInfo_SetDescription(wxd_AboutDialogInfo_t* info, const char* desc)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetDescription(wxString::FromUTF8(desc ? desc : ""));
}

void
wxd_AboutDialogInfo_SetCopyright(wxd_AboutDialogInfo_t* info, const char* copyright)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetCopyright(wxString::FromUTF8(copyright ? copyright : ""));
}

void
wxd_AboutDialogInfo_SetLicence(wxd_AboutDialogInfo_t* info, const char* licence)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetLicence(wxString::FromUTF8(licence ? licence : ""));
}

void
wxd_AboutDialogInfo_SetIcon(wxd_AboutDialogInfo_t* info, const wxd_Bitmap_t* bitmap)
{
    if (!info || !bitmap)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!bmp->IsOk())
        return;

    wxIcon icon;
    icon.CopyFromBitmap(*bmp);
    if (icon.IsOk()) {
        aboutInfo->SetIcon(icon);
    }
}

void
wxd_AboutDialogInfo_SetWebSite(wxd_AboutDialogInfo_t* info, const char* url)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetWebSite(wxString::FromUTF8(url ? url : ""));
}

void
wxd_AboutDialogInfo_SetWebSiteEx(wxd_AboutDialogInfo_t* info, const char* url,
                                  const char* desc)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->SetWebSite(wxString::FromUTF8(url ? url : ""),
                          wxString::FromUTF8(desc ? desc : ""));
}

void
wxd_AboutDialogInfo_AddDeveloper(wxd_AboutDialogInfo_t* info, const char* developer)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->AddDeveloper(wxString::FromUTF8(developer ? developer : ""));
}

void
wxd_AboutDialogInfo_AddDocWriter(wxd_AboutDialogInfo_t* info, const char* docwriter)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->AddDocWriter(wxString::FromUTF8(docwriter ? docwriter : ""));
}

void
wxd_AboutDialogInfo_AddArtist(wxd_AboutDialogInfo_t* info, const char* artist)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->AddArtist(wxString::FromUTF8(artist ? artist : ""));
}

void
wxd_AboutDialogInfo_AddTranslator(wxd_AboutDialogInfo_t* info, const char* translator)
{
    if (!info)
        return;
    wxAboutDialogInfo* aboutInfo = reinterpret_cast<wxAboutDialogInfo*>(info);
    aboutInfo->AddTranslator(wxString::FromUTF8(translator ? translator : ""));
}

void
wxd_AboutBox(const wxd_AboutDialogInfo_t* info, wxd_Window_t* parent)
{
    if (!info)
        return;
    const wxAboutDialogInfo* aboutInfo = reinterpret_cast<const wxAboutDialogInfo*>(info);
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);
    wxAboutBox(*aboutInfo, wx_parent);
}

} // extern "C"
