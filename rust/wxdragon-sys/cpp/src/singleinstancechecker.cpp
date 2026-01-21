#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"

#if wxUSE_SNGLINST_CHECKER
#include <wx/snglinst.h>

extern "C" {

wxd_SingleInstanceChecker_t*
wxd_SingleInstanceChecker_Create(const char* name, const char* path)
{
    if (!name)
        return nullptr;

    wxString wx_name = wxString::FromUTF8(name);
    wxString wx_path;
    if (path && path[0] != '\0') {
        wx_path = wxString::FromUTF8(path);
    }

    wxSingleInstanceChecker* checker = new wxSingleInstanceChecker();
    if (!checker->Create(wx_name, wx_path)) {
        delete checker;
        return nullptr;
    }

    return reinterpret_cast<wxd_SingleInstanceChecker_t*>(checker);
}

wxd_SingleInstanceChecker_t*
wxd_SingleInstanceChecker_CreateDefault()
{
    wxSingleInstanceChecker* checker = new wxSingleInstanceChecker();
    if (!checker->CreateDefault()) {
        delete checker;
        return nullptr;
    }

    return reinterpret_cast<wxd_SingleInstanceChecker_t*>(checker);
}

void
wxd_SingleInstanceChecker_Destroy(wxd_SingleInstanceChecker_t* checker)
{
    if (!checker)
        return;
    wxSingleInstanceChecker* wx_checker =
        reinterpret_cast<wxSingleInstanceChecker*>(checker);
    delete wx_checker;
}

bool
wxd_SingleInstanceChecker_IsAnotherRunning(wxd_SingleInstanceChecker_t* checker)
{
    if (!checker)
        return false;
    wxSingleInstanceChecker* wx_checker =
        reinterpret_cast<wxSingleInstanceChecker*>(checker);
    return wx_checker->IsAnotherRunning();
}

} // extern "C"

#else // !wxUSE_SNGLINST_CHECKER

// Stub implementations when wxSingleInstanceChecker is not available
extern "C" {

wxd_SingleInstanceChecker_t*
wxd_SingleInstanceChecker_Create(const char* name, const char* path)
{
    (void)name;
    (void)path;
    return nullptr;
}

wxd_SingleInstanceChecker_t*
wxd_SingleInstanceChecker_CreateDefault()
{
    return nullptr;
}

void
wxd_SingleInstanceChecker_Destroy(wxd_SingleInstanceChecker_t* checker)
{
    (void)checker;
}

bool
wxd_SingleInstanceChecker_IsAnotherRunning(wxd_SingleInstanceChecker_t* checker)
{
    (void)checker;
    return false;
}

} // extern "C"

#endif // wxUSE_SNGLINST_CHECKER
