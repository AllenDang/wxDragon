#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"

#if wxUSE_UIACTIONSIMULATOR
#include <wx/uiaction.h>

extern "C" {

wxd_UIActionSimulator_t*
wxd_UIActionSimulator_Create()
{
    return reinterpret_cast<wxd_UIActionSimulator_t*>(new wxUIActionSimulator());
}

void
wxd_UIActionSimulator_Destroy(wxd_UIActionSimulator_t* sim)
{
    if (sim) {
        delete reinterpret_cast<wxUIActionSimulator*>(sim);
    }
}

// --- Mouse Simulation ---

bool
wxd_UIActionSimulator_MouseMove(wxd_UIActionSimulator_t* sim, long x, long y)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->MouseMove(x, y);
}

bool
wxd_UIActionSimulator_MouseDown(wxd_UIActionSimulator_t* sim, int button)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->MouseDown(button);
}

bool
wxd_UIActionSimulator_MouseUp(wxd_UIActionSimulator_t* sim, int button)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->MouseUp(button);
}

bool
wxd_UIActionSimulator_MouseClick(wxd_UIActionSimulator_t* sim, int button)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->MouseClick(button);
}

bool
wxd_UIActionSimulator_MouseDblClick(wxd_UIActionSimulator_t* sim, int button)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->MouseDblClick(button);
}

bool
wxd_UIActionSimulator_MouseDragDrop(wxd_UIActionSimulator_t* sim,
                                     long x1, long y1, long x2, long y2,
                                     int button)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->MouseDragDrop(x1, y1, x2, y2, button);
}

// --- Keyboard Simulation ---

bool
wxd_UIActionSimulator_KeyDown(wxd_UIActionSimulator_t* sim, int keycode, int modifiers)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->KeyDown(keycode, modifiers);
}

bool
wxd_UIActionSimulator_KeyUp(wxd_UIActionSimulator_t* sim, int keycode, int modifiers)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->KeyUp(keycode, modifiers);
}

bool
wxd_UIActionSimulator_Char(wxd_UIActionSimulator_t* sim, int keycode, int modifiers)
{
    if (!sim)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->Char(keycode, modifiers);
}

bool
wxd_UIActionSimulator_Text(wxd_UIActionSimulator_t* sim, const char* text)
{
    if (!sim || !text)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->Text(text);
}

bool
wxd_UIActionSimulator_Select(wxd_UIActionSimulator_t* sim, const char* text)
{
    if (!sim || !text)
        return false;
    wxUIActionSimulator* wx_sim = reinterpret_cast<wxUIActionSimulator*>(sim);
    return wx_sim->Select(wxString::FromUTF8(text));
}

} // extern "C"

#else // !wxUSE_UIACTIONSIMULATOR

// Stub implementations when UIActionSimulator is not available
extern "C" {

wxd_UIActionSimulator_t*
wxd_UIActionSimulator_Create()
{
    return nullptr;
}

void
wxd_UIActionSimulator_Destroy(wxd_UIActionSimulator_t* /*sim*/)
{
}

bool
wxd_UIActionSimulator_MouseMove(wxd_UIActionSimulator_t* /*sim*/, long /*x*/, long /*y*/)
{
    return false;
}

bool
wxd_UIActionSimulator_MouseDown(wxd_UIActionSimulator_t* /*sim*/, int /*button*/)
{
    return false;
}

bool
wxd_UIActionSimulator_MouseUp(wxd_UIActionSimulator_t* /*sim*/, int /*button*/)
{
    return false;
}

bool
wxd_UIActionSimulator_MouseClick(wxd_UIActionSimulator_t* /*sim*/, int /*button*/)
{
    return false;
}

bool
wxd_UIActionSimulator_MouseDblClick(wxd_UIActionSimulator_t* /*sim*/, int /*button*/)
{
    return false;
}

bool
wxd_UIActionSimulator_MouseDragDrop(wxd_UIActionSimulator_t* /*sim*/,
                                     long /*x1*/, long /*y1*/, long /*x2*/, long /*y2*/,
                                     int /*button*/)
{
    return false;
}

bool
wxd_UIActionSimulator_KeyDown(wxd_UIActionSimulator_t* /*sim*/, int /*keycode*/, int /*modifiers*/)
{
    return false;
}

bool
wxd_UIActionSimulator_KeyUp(wxd_UIActionSimulator_t* /*sim*/, int /*keycode*/, int /*modifiers*/)
{
    return false;
}

bool
wxd_UIActionSimulator_Char(wxd_UIActionSimulator_t* /*sim*/, int /*keycode*/, int /*modifiers*/)
{
    return false;
}

bool
wxd_UIActionSimulator_Text(wxd_UIActionSimulator_t* /*sim*/, const char* /*text*/)
{
    return false;
}

bool
wxd_UIActionSimulator_Select(wxd_UIActionSimulator_t* /*sim*/, const char* /*text*/)
{
    return false;
}

} // extern "C"

#endif // wxUSE_UIACTIONSIMULATOR
