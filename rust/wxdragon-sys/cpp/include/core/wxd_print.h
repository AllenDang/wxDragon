#ifndef WXD_PRINT_H
#define WXD_PRINT_H

#include "../wxd_types.h"

// --- PrintData ---
WXD_EXPORTED wxd_PrintData_t* wxd_PrintData_Create();
WXD_EXPORTED void wxd_PrintData_Destroy(wxd_PrintData_t* self);
WXD_EXPORTED bool wxd_PrintData_IsOk(wxd_PrintData_t* self);

// --- PrintDialogData ---
WXD_EXPORTED wxd_PrintDialogData_t* wxd_PrintDialogData_Create();
WXD_EXPORTED wxd_PrintDialogData_t* wxd_PrintDialogData_CreateFromData(wxd_PrintData_t* data);
WXD_EXPORTED void wxd_PrintDialogData_Destroy(wxd_PrintDialogData_t* self);
WXD_EXPORTED wxd_PrintData_t* wxd_PrintDialogData_GetPrintData(wxd_PrintDialogData_t* self);

// --- PageSetupDialogData ---
WXD_EXPORTED wxd_PageSetupDialogData_t* wxd_PageSetupDialogData_Create();
WXD_EXPORTED wxd_PageSetupDialogData_t* wxd_PageSetupDialogData_CreateFromData(wxd_PrintData_t* data);
WXD_EXPORTED void wxd_PageSetupDialogData_Destroy(wxd_PageSetupDialogData_t* self);
WXD_EXPORTED wxd_PrintData_t* wxd_PageSetupDialogData_GetPrintData(wxd_PageSetupDialogData_t* self);

// --- Printout ---
WXD_EXPORTED wxd_Printout_t* wxd_Printout_CreateWithCallbacks(
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
);
WXD_EXPORTED void wxd_Printout_Destroy(wxd_Printout_t* self);
WXD_EXPORTED wxd_DC_t* wxd_Printout_GetDC(wxd_Printout_t* self);
WXD_EXPORTED void wxd_Printout_GetPageSizePixels(wxd_Printout_t* self, int* w, int* h);
WXD_EXPORTED void wxd_Printout_GetPageSizeMM(wxd_Printout_t* self, int* w, int* h);
WXD_EXPORTED void wxd_Printout_GetPPIScreen(wxd_Printout_t* self, int* x, int* y);
WXD_EXPORTED void wxd_Printout_GetPPIPrinter(wxd_Printout_t* self, int* x, int* y);
WXD_EXPORTED bool wxd_Printout_IsPreview(wxd_Printout_t* self);

// --- Printer ---
WXD_EXPORTED wxd_Printer_t* wxd_Printer_Create(wxd_PrintDialogData_t* data);
WXD_EXPORTED void wxd_Printer_Destroy(wxd_Printer_t* self);
WXD_EXPORTED bool wxd_Printer_Print(wxd_Printer_t* self, wxd_Window_t* parent, wxd_Printout_t* printout, bool prompt);
WXD_EXPORTED wxd_PrintDialogData_t* wxd_Printer_GetPrintDialogData(wxd_Printer_t* self);

// --- PrintDialog ---
WXD_EXPORTED wxd_PrintDialog_t* wxd_PrintDialog_Create(wxd_Window_t* parent, wxd_PrintDialogData_t* data);
WXD_EXPORTED void wxd_PrintDialog_Destroy(wxd_PrintDialog_t* self);
WXD_EXPORTED int wxd_PrintDialog_ShowModal(wxd_PrintDialog_t* self);
WXD_EXPORTED wxd_PrintDialogData_t* wxd_PrintDialog_GetPrintDialogData(wxd_PrintDialog_t* self);
WXD_EXPORTED wxd_DC_t* wxd_PrintDialog_GetPrintDC(wxd_PrintDialog_t* self);

// --- PageSetupDialog ---
WXD_EXPORTED wxd_PageSetupDialog_t* wxd_PageSetupDialog_Create(wxd_Window_t* parent, wxd_PageSetupDialogData_t* data);
WXD_EXPORTED void wxd_PageSetupDialog_Destroy(wxd_PageSetupDialog_t* self);
WXD_EXPORTED int wxd_PageSetupDialog_ShowModal(wxd_PageSetupDialog_t* self);
WXD_EXPORTED wxd_PageSetupDialogData_t* wxd_PageSetupDialog_GetPageSetupDialogData(wxd_PageSetupDialog_t* self);

#endif // WXD_PRINT_H
