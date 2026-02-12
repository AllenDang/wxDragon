#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/print.h>
#include <wx/printdlg.h>
#include "../include/wxdragon.h"

// --- WxdPrintout Proxy Class ---

class WxdPrintout : public wxPrintout {
public:
    WxdPrintout(const wxString& title,
                void* userData,
                wxd_Printout_OnPreparePrinting_Callback onPreparePrinting,
                wxd_Printout_OnBeginPrinting_Callback onBeginPrinting,
                wxd_Printout_OnEndPrinting_Callback onEndPrinting,
                wxd_Printout_OnBeginDocument_Callback onBeginDocument,
                wxd_Printout_OnEndDocument_Callback onEndDocument,
                wxd_Printout_OnPrintPage_Callback onPrintPage,
                wxd_Printout_HasPage_Callback hasPage,
                wxd_Printout_GetPageInfo_Callback getPageInfo)
        : wxPrintout(title), m_userData(userData),
          m_onPreparePrinting(onPreparePrinting),
          m_onBeginPrinting(onBeginPrinting),
          m_onEndPrinting(onEndPrinting),
          m_onBeginDocument(onBeginDocument),
          m_onEndDocument(onEndDocument),
          m_onPrintPage(onPrintPage),
          m_hasPage(hasPage),
          m_getPageInfo(getPageInfo)
    {}

    virtual void OnPreparePrinting() override {
        if (m_onPreparePrinting) m_onPreparePrinting(m_userData);
        else wxPrintout::OnPreparePrinting();
    }

    virtual void OnBeginPrinting() override {
        if (m_onBeginPrinting) m_onBeginPrinting(m_userData);
        else wxPrintout::OnBeginPrinting();
    }

    virtual void OnEndPrinting() override {
        if (m_onEndPrinting) m_onEndPrinting(m_userData);
        else wxPrintout::OnEndPrinting();
    }

    virtual bool OnBeginDocument(int startPage, int endPage) override {
        if (m_onBeginDocument) {
            m_onBeginDocument(m_userData, startPage, endPage);
            return true;
        }
        return wxPrintout::OnBeginDocument(startPage, endPage);
    }

    virtual void OnEndDocument() override {
        if (m_onEndDocument) m_onEndDocument(m_userData);
        else wxPrintout::OnEndDocument();
    }

    virtual bool OnPrintPage(int pageNum) override {
        if (m_onPrintPage) return m_onPrintPage(m_userData, pageNum);
        return false;
    }

    virtual bool HasPage(int pageNum) override {
        if (m_hasPage) return m_hasPage(m_userData, pageNum);
        return wxPrintout::HasPage(pageNum);
    }

    virtual void GetPageInfo(int* minPage, int* maxPage, int* pageFrom, int* pageTo) override {
        if (m_getPageInfo) m_getPageInfo(m_userData, minPage, maxPage, pageFrom, pageTo);
        else wxPrintout::GetPageInfo(minPage, maxPage, pageFrom, pageTo);
    }

private:
    void* m_userData;
    wxd_Printout_OnPreparePrinting_Callback m_onPreparePrinting;
    wxd_Printout_OnBeginPrinting_Callback m_onBeginPrinting;
    wxd_Printout_OnEndPrinting_Callback m_onEndPrinting;
    wxd_Printout_OnBeginDocument_Callback m_onBeginDocument;
    wxd_Printout_OnEndDocument_Callback m_onEndDocument;
    wxd_Printout_OnPrintPage_Callback m_onPrintPage;
    wxd_Printout_HasPage_Callback m_hasPage;
    wxd_Printout_GetPageInfo_Callback m_getPageInfo;
};

// --- C API Implementation ---

// PrintData
extern "C" wxd_PrintData_t* wxd_PrintData_Create() {
    return reinterpret_cast<wxd_PrintData_t*>(new wxPrintData());
}

extern "C" void wxd_PrintData_Destroy(wxd_PrintData_t* self) {
    delete reinterpret_cast<wxPrintData*>(self);
}

extern "C" bool wxd_PrintData_IsOk(wxd_PrintData_t* self) {
    return reinterpret_cast<wxPrintData*>(self)->IsOk();
}

// PrintDialogData
extern "C" wxd_PrintDialogData_t* wxd_PrintDialogData_Create() {
    return reinterpret_cast<wxd_PrintDialogData_t*>(new wxPrintDialogData());
}

extern "C" wxd_PrintDialogData_t* wxd_PrintDialogData_CreateFromData(wxd_PrintData_t* data) {
    return reinterpret_cast<wxd_PrintDialogData_t*>(new wxPrintDialogData(*reinterpret_cast<wxPrintData*>(data)));
}

extern "C" void wxd_PrintDialogData_Destroy(wxd_PrintDialogData_t* self) {
    delete reinterpret_cast<wxPrintDialogData*>(self);
}

extern "C" wxd_PrintData_t* wxd_PrintDialogData_GetPrintData(wxd_PrintDialogData_t* self) {
    return reinterpret_cast<wxd_PrintData_t*>(&reinterpret_cast<wxPrintDialogData*>(self)->GetPrintData());
}

// PageSetupDialogData
extern "C" wxd_PageSetupDialogData_t* wxd_PageSetupDialogData_Create() {
    return reinterpret_cast<wxd_PageSetupDialogData_t*>(new wxPageSetupDialogData());
}

extern "C" wxd_PageSetupDialogData_t* wxd_PageSetupDialogData_CreateFromData(wxd_PrintData_t* data) {
    return reinterpret_cast<wxd_PageSetupDialogData_t*>(new wxPageSetupDialogData(*reinterpret_cast<wxPrintData*>(data)));
}

extern "C" void wxd_PageSetupDialogData_Destroy(wxd_PageSetupDialogData_t* self) {
    delete reinterpret_cast<wxPageSetupDialogData*>(self);
}

extern "C" wxd_PrintData_t* wxd_PageSetupDialogData_GetPrintData(wxd_PageSetupDialogData_t* self) {
    return reinterpret_cast<wxd_PrintData_t*>(&reinterpret_cast<wxPageSetupDialogData*>(self)->GetPrintData());
}

// Printout
extern "C" wxd_Printout_t* wxd_Printout_CreateWithCallbacks(
    const char* title,
    void* userData,
    wxd_Printout_OnPreparePrinting_Callback onPreparePrinting,
    wxd_Printout_OnBeginPrinting_Callback onBeginPrinting,
    wxd_Printout_OnEndPrinting_Callback onEndPrinting,
    wxd_Printout_OnBeginDocument_Callback onBeginDocument,
    wxd_Printout_OnEndDocument_Callback onEndDocument,
    wxd_Printout_OnPrintPage_Callback onPrintPage,
    wxd_Printout_HasPage_Callback hasPage,
    wxd_Printout_GetPageInfo_Callback getPageInfo
) {
    wxString wxTitle = wxString::FromUTF8(title);
    return reinterpret_cast<wxd_Printout_t*>(new WxdPrintout(
        wxTitle, userData, onPreparePrinting, onBeginPrinting, onEndPrinting,
        onBeginDocument, onEndDocument, onPrintPage, hasPage, getPageInfo
    ));
}

extern "C" void wxd_Printout_Destroy(wxd_Printout_t* self) {
    delete reinterpret_cast<wxPrintout*>(self);
}

extern "C" wxd_DC_t* wxd_Printout_GetDC(wxd_Printout_t* self) {
    return reinterpret_cast<wxd_DC_t*>(reinterpret_cast<wxPrintout*>(self)->GetDC());
}

extern "C" void wxd_Printout_GetPageSizePixels(wxd_Printout_t* self, int* w, int* h) {
    reinterpret_cast<wxPrintout*>(self)->GetPageSizePixels(w, h);
}

extern "C" void wxd_Printout_GetPageSizeMM(wxd_Printout_t* self, int* w, int* h) {
    reinterpret_cast<wxPrintout*>(self)->GetPageSizeMM(w, h);
}

extern "C" void wxd_Printout_GetPPIScreen(wxd_Printout_t* self, int* x, int* y) {
    reinterpret_cast<wxPrintout*>(self)->GetPPIScreen(x, y);
}

extern "C" void wxd_Printout_GetPPIPrinter(wxd_Printout_t* self, int* x, int* y) {
    reinterpret_cast<wxPrintout*>(self)->GetPPIPrinter(x, y);
}

extern "C" bool wxd_Printout_IsPreview(wxd_Printout_t* self) {
    return reinterpret_cast<wxPrintout*>(self)->IsPreview();
}

// Printer
extern "C" wxd_Printer_t* wxd_Printer_Create(wxd_PrintDialogData_t* data) {
    if (data) {
        return reinterpret_cast<wxd_Printer_t*>(new wxPrinter(reinterpret_cast<wxPrintDialogData*>(data)));
    } else {
        return reinterpret_cast<wxd_Printer_t*>(new wxPrinter());
    }
}

extern "C" void wxd_Printer_Destroy(wxd_Printer_t* self) {
    delete reinterpret_cast<wxPrinter*>(self);
}

extern "C" bool wxd_Printer_Print(wxd_Printer_t* self, wxd_Window_t* parent, wxd_Printout_t* printout, bool prompt) {
    return reinterpret_cast<wxPrinter*>(self)->Print(
        reinterpret_cast<wxWindow*>(parent),
        reinterpret_cast<wxPrintout*>(printout),
        prompt
    );
}

extern "C" wxd_PrintDialogData_t* wxd_Printer_GetPrintDialogData(wxd_Printer_t* self) {
    return reinterpret_cast<wxd_PrintDialogData_t*>(&reinterpret_cast<wxPrinter*>(self)->GetPrintDialogData());
}

// PrintDialog
extern "C" wxd_PrintDialog_t* wxd_PrintDialog_Create(wxd_Window_t* parent, wxd_PrintDialogData_t* data) {
    return reinterpret_cast<wxd_PrintDialog_t*>(new wxPrintDialog(
        reinterpret_cast<wxWindow*>(parent),
        reinterpret_cast<wxPrintDialogData*>(data)
    ));
}

extern "C" void wxd_PrintDialog_Destroy(wxd_PrintDialog_t* self) {
    delete reinterpret_cast<wxPrintDialog*>(self);
}

extern "C" int wxd_PrintDialog_ShowModal(wxd_PrintDialog_t* self) {
    return reinterpret_cast<wxPrintDialog*>(self)->ShowModal();
}

extern "C" wxd_PrintDialogData_t* wxd_PrintDialog_GetPrintDialogData(wxd_PrintDialog_t* self) {
    return reinterpret_cast<wxd_PrintDialogData_t*>(&reinterpret_cast<wxPrintDialog*>(self)->GetPrintDialogData());
}

extern "C" wxd_DC_t* wxd_PrintDialog_GetPrintDC(wxd_PrintDialog_t* self) {
    return reinterpret_cast<wxd_DC_t*>(reinterpret_cast<wxPrintDialog*>(self)->GetPrintDC());
}

// PageSetupDialog
extern "C" wxd_PageSetupDialog_t* wxd_PageSetupDialog_Create(wxd_Window_t* parent, wxd_PageSetupDialogData_t* data) {
    return reinterpret_cast<wxd_PageSetupDialog_t*>(new wxPageSetupDialog(
        reinterpret_cast<wxWindow*>(parent),
        reinterpret_cast<wxPageSetupDialogData*>(data)
    ));
}

extern "C" void wxd_PageSetupDialog_Destroy(wxd_PageSetupDialog_t* self) {
    delete reinterpret_cast<wxPageSetupDialog*>(self);
}

extern "C" int wxd_PageSetupDialog_ShowModal(wxd_PageSetupDialog_t* self) {
    return reinterpret_cast<wxPageSetupDialog*>(self)->ShowModal();
}

extern "C" wxd_PageSetupDialogData_t* wxd_PageSetupDialog_GetPageSetupDialogData(wxd_PageSetupDialog_t* self) {
    return reinterpret_cast<wxd_PageSetupDialogData_t*>(&reinterpret_cast<wxPageSetupDialog*>(self)->GetPageSetupDialogData());
}
