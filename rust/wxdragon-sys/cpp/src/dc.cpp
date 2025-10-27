#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dcmemory.h>
#include <wx/dcscreen.h>
#include <wx/dcbuffer.h>

// Type aliases for easier reference
using wxd_DC_t = struct wxd_DC_t;
using wxd_WindowDC_t = struct wxd_WindowDC_t;
using wxd_ClientDC_t = struct wxd_ClientDC_t;
using wxd_PaintDC_t = struct wxd_PaintDC_t;
using wxd_MemoryDC_t = struct wxd_MemoryDC_t;
using wxd_ScreenDC_t = struct wxd_ScreenDC_t;
using wxd_AutoBufferedPaintDC_t = struct wxd_AutoBufferedPaintDC_t;

// The wxWidgets DC hierarchy provides some challenges for C FFI
// We need to use the opaque type pattern and typecasts:
// - wxDC (base class)
//   - wxWindowDC
//     - wxClientDC
//     - wxPaintDC
//   - wxMemoryDC
//   - wxScreenDC

// Since wxDC is abstract, we need to ensure we only expose classes that wxWidgets actually provides

// DC Creation/Destruction
wxd_WindowDC_t*
wxd_WindowDC_Create(wxd_Window_t* window)
{
    if (!window)
        return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return reinterpret_cast<wxd_WindowDC_t*>(new wxWindowDC(wx_window));
}

void
wxd_WindowDC_Destroy(wxd_WindowDC_t* dc)
{
    if (dc) {
        delete reinterpret_cast<wxWindowDC*>(dc);
    }
}

wxd_ClientDC_t*
wxd_ClientDC_Create(wxd_Window_t* window)
{
    if (!window)
        return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return reinterpret_cast<wxd_ClientDC_t*>(new wxClientDC(wx_window));
}

void
wxd_ClientDC_Destroy(wxd_ClientDC_t* dc)
{
    if (dc) {
        delete reinterpret_cast<wxClientDC*>(dc);
    }
}

wxd_PaintDC_t*
wxd_PaintDC_Create(wxd_Window_t* window)
{
    if (!window)
        return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return reinterpret_cast<wxd_PaintDC_t*>(new wxPaintDC(wx_window));
}

void
wxd_PaintDC_Destroy(wxd_PaintDC_t* dc)
{
    if (dc) {
        delete reinterpret_cast<wxPaintDC*>(dc);
    }
}

wxd_MemoryDC_t*
wxd_MemoryDC_Create(void)
{
    return reinterpret_cast<wxd_MemoryDC_t*>(new wxMemoryDC());
}

void
wxd_MemoryDC_Destroy(wxd_MemoryDC_t* dc)
{
    if (dc) {
        delete reinterpret_cast<wxMemoryDC*>(dc);
    }
}

wxd_ScreenDC_t*
wxd_ScreenDC_Create(void)
{
    return reinterpret_cast<wxd_ScreenDC_t*>(new wxScreenDC());
}

void
wxd_ScreenDC_Destroy(wxd_ScreenDC_t* dc)
{
    if (dc) {
        delete reinterpret_cast<wxScreenDC*>(dc);
    }
}

wxd_AutoBufferedPaintDC_t*
wxd_AutoBufferedPaintDC_Create(wxd_Window_t* window)
{
    if (!window)
        return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    return reinterpret_cast<wxd_AutoBufferedPaintDC_t*>(new wxAutoBufferedPaintDC(wx_window));
}

void
wxd_AutoBufferedPaintDC_Destroy(wxd_AutoBufferedPaintDC_t* dc)
{
    if (dc) {
        delete reinterpret_cast<wxAutoBufferedPaintDC*>(dc);
    }
}

// Type casting functions
wxd_DC_t*
wxd_WindowDC_AsDC(wxd_WindowDC_t* dc)
{
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxWindowDC*>(dc)));
}

wxd_DC_t*
wxd_ClientDC_AsDC(wxd_ClientDC_t* dc)
{
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxClientDC*>(dc)));
}

wxd_DC_t*
wxd_PaintDC_AsDC(wxd_PaintDC_t* dc)
{
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxPaintDC*>(dc)));
}

wxd_DC_t*
wxd_MemoryDC_AsDC(wxd_MemoryDC_t* dc)
{
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxMemoryDC*>(dc)));
}

wxd_DC_t*
wxd_ScreenDC_AsDC(wxd_ScreenDC_t* dc)
{
    return reinterpret_cast<wxd_DC_t*>(static_cast<wxDC*>(reinterpret_cast<wxScreenDC*>(dc)));
}

wxd_DC_t*
wxd_AutoBufferedPaintDC_AsDC(wxd_AutoBufferedPaintDC_t* dc)
{
    return reinterpret_cast<wxd_DC_t*>(
        static_cast<wxDC*>(reinterpret_cast<wxAutoBufferedPaintDC*>(dc)));
}

// Common DC operations
void
wxd_DC_Clear(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->Clear();
    }
}

void
wxd_DC_SetBackground(wxd_DC_t* dc, wxd_Colour_t background)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(background.r, background.g, background.b, background.a);
        wx_dc->SetBackground(wxBrush(wx_color));
    }
}

void
wxd_DC_SetBackgroundMode(wxd_DC_t* dc, int mode)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        int wx_mode = mode;
        wx_dc->SetBackgroundMode(wx_mode);
    }
}

void
wxd_DC_SetTextBackground(wxd_DC_t* dc, wxd_Colour_t colour)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);
        wx_dc->SetTextBackground(wx_color);
    }
}

void
wxd_DC_SetTextForeground(wxd_DC_t* dc, wxd_Colour_t colour)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);
        wx_dc->SetTextForeground(wx_color);
    }
}

void
wxd_DC_SetFont(wxd_DC_t* dc, const wxd_Font_t* font)
{
    if (dc && font) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxFont* wx_font = reinterpret_cast<wxFont*>(const_cast<wxd_Font_t*>(font));
        wx_dc->SetFont(*wx_font);
    }
}

void
wxd_DC_SetPen(wxd_DC_t* dc, wxd_Colour_t colour, int width, int style)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);

        wxPenStyle wx_style = static_cast<wxPenStyle>(style);

        wx_dc->SetPen(wxPen(wx_color, width, wx_style));
    }
}

void
wxd_DC_SetBrush(wxd_DC_t* dc, wxd_Colour_t colour, int style)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_color(colour.r, colour.g, colour.b, colour.a);

        wxBrushStyle wx_style = static_cast<wxBrushStyle>(style);

        wx_dc->SetBrush(wxBrush(wx_color, wx_style));
    }
}

// Advanced drawing operations
void
wxd_DC_DrawRoundedRectangle(wxd_DC_t* dc, int x, int y, int width, int height, double radius)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawRoundedRectangle(x, y, width, height, radius);
    }
}

void
wxd_DC_DrawArc(wxd_DC_t* dc, int x1, int y1, int x2, int y2, int xc, int yc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawArc(x1, y1, x2, y2, xc, yc);
    }
}

void
wxd_DC_DrawEllipticArc(wxd_DC_t* dc, int x, int y, int width, int height, double start, double end)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawEllipticArc(x, y, width, height, start, end);
    }
}

void
wxd_DC_DrawPolygon(wxd_DC_t* dc, int n, wxd_Point* points, int xoffset, int yoffset, int fill_style)
{
    if (dc && points) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxPoint* wx_points = reinterpret_cast<wxPoint*>(points);
        wxPolygonFillMode wx_fill_style = static_cast<wxPolygonFillMode>(fill_style);
        wx_dc->DrawPolygon(n, wx_points, xoffset, yoffset, wx_fill_style);
    }
}

void
wxd_DC_DrawLines(wxd_DC_t* dc, int n, wxd_Point* points, int xoffset, int yoffset)
{
    if (dc && points) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxPoint* wx_points = reinterpret_cast<wxPoint*>(points);
        wx_dc->DrawLines(n, wx_points, xoffset, yoffset);
    }
}

void
wxd_DC_DrawSpline(wxd_DC_t* dc, int n, wxd_Point* points)
{
    if (dc && points) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxPoint* wx_points = reinterpret_cast<wxPoint*>(points);
        wx_dc->DrawSpline(n, wx_points);
    }
}

// Text drawing operations
void
wxd_DC_DrawRotatedText(wxd_DC_t* dc, const char* text, int x, int y, double angle)
{
    if (dc && text) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(text);
        wx_dc->DrawRotatedText(wx_text, x, y, angle);
    }
}

void
wxd_DC_DrawLabel(wxd_DC_t* dc, const char* text, wxd_Rect rect, int alignment, int indexAccel)
{
    if (dc && text) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(text);
        wxRect wx_rect(rect.x, rect.y, rect.width, rect.height);
        int wx_alignment = alignment;

        wx_dc->DrawLabel(wx_text, wx_rect, wx_alignment, indexAccel);
    }
}

// Bitmap operations
bool
wxd_DC_Blit(wxd_DC_t* dest_dc, int xdest, int ydest, int width, int height, wxd_DC_t* source_dc,
            int xsrc, int ysrc, int logical_func, bool use_mask, int xsrc_mask, int ysrc_mask)
{
    if (!dest_dc || !source_dc)
        return false;

    wxDC* wx_dest_dc = reinterpret_cast<wxDC*>(dest_dc);
    wxDC* wx_source_dc = reinterpret_cast<wxDC*>(source_dc);

    wxRasterOperationMode wx_logical_func = static_cast<wxRasterOperationMode>(logical_func);

    return wx_dest_dc->Blit(xdest, ydest, width, height, wx_source_dc, xsrc, ysrc, wx_logical_func,
                            use_mask, xsrc_mask, ysrc_mask);
}

bool
wxd_DC_StretchBlit(wxd_DC_t* dest_dc, int xdest, int ydest, int dstWidth, int dstHeight,
                   wxd_DC_t* source_dc, int xsrc, int ysrc, int srcWidth, int srcHeight,
                   int logical_func, bool use_mask, int xsrc_mask, int ysrc_mask)
{
    if (!dest_dc || !source_dc)
        return false;

    wxDC* wx_dest_dc = reinterpret_cast<wxDC*>(dest_dc);
    wxDC* wx_source_dc = reinterpret_cast<wxDC*>(source_dc);

    wxRasterOperationMode wx_logical_func = static_cast<wxRasterOperationMode>(logical_func);

    return wx_dest_dc->StretchBlit(xdest, ydest, dstWidth, dstHeight, wx_source_dc, xsrc, ysrc,
                                   srcWidth, srcHeight, wx_logical_func, use_mask, xsrc_mask,
                                   ysrc_mask);
}

// Clipping operations
void
wxd_DC_SetClippingRegion(wxd_DC_t* dc, int x, int y, int width, int height)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->SetClippingRegion(x, y, width, height);
    }
}

void
wxd_DC_SetClippingRegionFromPoints(wxd_DC_t* dc, int n, wxd_Point* points)
{
    if (dc && points) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxPoint* wx_points = reinterpret_cast<wxPoint*>(points);
        // Create region and get its bounding box to use the non-deprecated method
        wxRegion region(n, wx_points);
        wxRect rect = region.GetBox();
        wx_dc->SetClippingRegion(rect.x, rect.y, rect.width, rect.height);
    }
}

void
wxd_DC_DestroyClippingRegion(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DestroyClippingRegion();
    }
}

void
wxd_DC_GetClippingBox(wxd_DC_t* dc, int* x, int* y, int* width, int* height)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxCoord wx_x, wx_y, wx_width, wx_height;
        wx_dc->GetClippingBox(&wx_x, &wx_y, &wx_width, &wx_height);
        if (x)
            *x = wx_x;
        if (y)
            *y = wx_y;
        if (width)
            *width = wx_width;
        if (height)
            *height = wx_height;
    }
}

// Coordinate transformation
void
wxd_DC_SetDeviceOrigin(wxd_DC_t* dc, int x, int y)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->SetDeviceOrigin(x, y);
    }
}

void
wxd_DC_SetLogicalOrigin(wxd_DC_t* dc, int x, int y)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->SetLogicalOrigin(x, y);
    }
}

void
wxd_DC_SetUserScale(wxd_DC_t* dc, double x_scale, double y_scale)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->SetUserScale(x_scale, y_scale);
    }
}

void
wxd_DC_SetLogicalScale(wxd_DC_t* dc, double x_scale, double y_scale)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->SetLogicalScale(x_scale, y_scale);
    }
}

void
wxd_DC_SetMapMode(wxd_DC_t* dc, int mode)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxMappingMode wx_mode = static_cast<wxMappingMode>(mode);
        wx_dc->SetMapMode(wx_mode);
    }
}

// Drawing operations
void
wxd_DC_DrawPoint(wxd_DC_t* dc, int x, int y)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawPoint(x, y);
    }
}

void
wxd_DC_DrawLine(wxd_DC_t* dc, int x1, int y1, int x2, int y2)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawLine(x1, y1, x2, y2);
    }
}

void
wxd_DC_DrawRectangle(wxd_DC_t* dc, int x, int y, int width, int height)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawRectangle(x, y, width, height);
    }
}

void
wxd_DC_DrawCircle(wxd_DC_t* dc, int x, int y, int radius)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawCircle(x, y, radius);
    }
}

void
wxd_DC_DrawEllipse(wxd_DC_t* dc, int x, int y, int width, int height)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wx_dc->DrawEllipse(x, y, width, height);
    }
}

void
wxd_DC_DrawText(wxd_DC_t* dc, const char* text, int x, int y)
{
    if (dc && text) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(text ? text : "");
        wx_dc->DrawText(wx_text, x, y);
    }
}

void
wxd_DC_DrawBitmap(wxd_DC_t* dc, const wxd_Bitmap_t* bitmap, int x, int y, bool transparent)
{
    if (dc && bitmap) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxBitmap* wx_bitmap = reinterpret_cast<wxBitmap*>(const_cast<wxd_Bitmap_t*>(bitmap));
        wx_dc->DrawBitmap(*wx_bitmap, x, y, transparent);
    }
}

// Device capabilities
wxd_Size
wxd_DC_GetSize(wxd_DC_t* dc)
{
    wxd_Size size = { 0, 0 };
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxSize wx_size = wx_dc->GetSize();
        size.width = wx_size.x;
        size.height = wx_size.y;
    }
    return size;
}

void
wxd_DC_GetTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h)
{
    if (dc && string && w && h) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(string);
        int width, height;
        wx_dc->GetTextExtent(wx_text, &width, &height);
        *w = width;
        *h = height;
    }
}

// MemoryDC specific operations
void
wxd_MemoryDC_SelectObject(wxd_MemoryDC_t* dc, const wxd_Bitmap_t* bitmap)
{
    if (dc && bitmap) {
        wxMemoryDC* wx_dc = reinterpret_cast<wxMemoryDC*>(dc);
        const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(bitmap);
        // Note: wxMemoryDC::SelectObject takes a non-const wxBitmap*, strange.
        wx_dc->SelectObject(*(const_cast<wxBitmap*>(wx_bitmap)));
    }
}

void
wxd_MemoryDC_SelectObjectAsSource(wxd_MemoryDC_t* dc, const wxd_Bitmap_t* bitmap)
{
    if (dc && bitmap) {
        wxMemoryDC* wx_dc = reinterpret_cast<wxMemoryDC*>(dc);
        const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(bitmap);
        wx_dc->SelectObjectAsSource(*wx_bitmap);
    }
}

// Get transformation info
wxd_Point
wxd_DC_GetDeviceOrigin(wxd_DC_t* dc)
{
    wxd_Point origin = { 0, 0 };
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxPoint wx_origin = wx_dc->GetDeviceOrigin();
        origin.x = wx_origin.x;
        origin.y = wx_origin.y;
    }
    return origin;
}

wxd_Point
wxd_DC_GetLogicalOrigin(wxd_DC_t* dc)
{
    wxd_Point origin = { 0, 0 };
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxPoint wx_origin = wx_dc->GetLogicalOrigin();
        origin.x = wx_origin.x;
        origin.y = wx_origin.y;
    }
    return origin;
}

void
wxd_DC_GetUserScale(wxd_DC_t* dc, double* x_scale, double* y_scale)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        double wx_x_scale, wx_y_scale;
        wx_dc->GetUserScale(&wx_x_scale, &wx_y_scale);
        if (x_scale)
            *x_scale = wx_x_scale;
        if (y_scale)
            *y_scale = wx_y_scale;
    }
}

void
wxd_DC_GetLogicalScale(wxd_DC_t* dc, double* x_scale, double* y_scale)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        double wx_x_scale, wx_y_scale;
        wx_dc->GetLogicalScale(&wx_x_scale, &wx_y_scale);
        if (x_scale)
            *x_scale = wx_x_scale;
        if (y_scale)
            *y_scale = wx_y_scale;
    }
}

int
wxd_DC_GetMapMode(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return static_cast<int>(wx_dc->GetMapMode());
    }
    return static_cast<int>(wxMM_TEXT);
}

// Coordinate conversion
int
wxd_DC_DeviceToLogicalX(wxd_DC_t* dc, int x)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return wx_dc->DeviceToLogicalX(x);
    }
    return x;
}

int
wxd_DC_DeviceToLogicalY(wxd_DC_t* dc, int y)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return wx_dc->DeviceToLogicalY(y);
    }
    return y;
}

int
wxd_DC_LogicalToDeviceX(wxd_DC_t* dc, int x)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return wx_dc->LogicalToDeviceX(x);
    }
    return x;
}

int
wxd_DC_LogicalToDeviceY(wxd_DC_t* dc, int y)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return wx_dc->LogicalToDeviceY(y);
    }
    return y;
}

// Extended device capabilities
wxd_Size
wxd_DC_GetSizeMM(wxd_DC_t* dc)
{
    wxd_Size size = { 0, 0 };
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxSize wx_size = wx_dc->GetSizeMM();
        size.width = wx_size.x;
        size.height = wx_size.y;
    }
    return size;
}

void
wxd_DC_GetFullTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h, int* descent,
                         int* externalLeading, const wxd_Font_t* font)
{
    if (dc && string) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(string);
        wxCoord wx_w, wx_h, wx_descent, wx_externalLeading;

        if (font) {
            wxFont* wx_font = reinterpret_cast<wxFont*>(const_cast<wxd_Font_t*>(font));
            wx_dc->GetTextExtent(wx_text, &wx_w, &wx_h, &wx_descent, &wx_externalLeading, wx_font);
        }
        else {
            wx_dc->GetTextExtent(wx_text, &wx_w, &wx_h, &wx_descent, &wx_externalLeading);
        }

        if (w)
            *w = wx_w;
        if (h)
            *h = wx_h;
        if (descent)
            *descent = wx_descent;
        if (externalLeading)
            *externalLeading = wx_externalLeading;
    }
}

void
wxd_DC_GetMultiLineTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h, int* heightLine,
                              const wxd_Font_t* font)
{
    if (dc && string) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxString wx_text = wxString::FromUTF8(string);
        wxCoord wx_w, wx_h, wx_heightLine;

        if (font) {
            wxFont* wx_font = reinterpret_cast<wxFont*>(const_cast<wxd_Font_t*>(font));
            wx_dc->GetMultiLineTextExtent(wx_text, &wx_w, &wx_h, &wx_heightLine, wx_font);
        }
        else {
            wx_dc->GetMultiLineTextExtent(wx_text, &wx_w, &wx_h, &wx_heightLine);
        }

        if (w)
            *w = wx_w;
        if (h)
            *h = wx_h;
        if (heightLine)
            *heightLine = wx_heightLine;
    }
}

int
wxd_DC_GetCharHeight(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return wx_dc->GetCharHeight();
    }
    return 0;
}

int
wxd_DC_GetCharWidth(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return wx_dc->GetCharWidth();
    }
    return 0;
}

// Drawing state queries
wxd_Colour_t
wxd_DC_GetBackground(wxd_DC_t* dc)
{
    wxd_Colour_t colour = { 0, 0, 0, 255 };
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_colour = wx_dc->GetBackground().GetColour();
        colour.r = wx_colour.Red();
        colour.g = wx_colour.Green();
        colour.b = wx_colour.Blue();
        colour.a = wx_colour.Alpha();
    }
    return colour;
}

int
wxd_DC_GetBackgroundMode(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return static_cast<int>(wx_dc->GetBackgroundMode());
    }
    return static_cast<int>(wxSOLID);
}

wxd_Colour_t
wxd_DC_GetTextBackground(wxd_DC_t* dc)
{
    wxd_Colour_t colour = { 0, 0, 0, 255 };
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_colour = wx_dc->GetTextBackground();
        colour.r = wx_colour.Red();
        colour.g = wx_colour.Green();
        colour.b = wx_colour.Blue();
        colour.a = wx_colour.Alpha();
    }
    return colour;
}

wxd_Colour_t
wxd_DC_GetTextForeground(wxd_DC_t* dc)
{
    wxd_Colour_t colour = { 0, 0, 0, 255 };
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_colour = wx_dc->GetTextForeground();
        colour.r = wx_colour.Red();
        colour.g = wx_colour.Green();
        colour.b = wx_colour.Blue();
        colour.a = wx_colour.Alpha();
    }
    return colour;
}

// DPI and scaling
wxd_Size
wxd_DC_GetPPI(wxd_DC_t* dc)
{
    wxd_Size ppi = { 96, 96 }; // Default DPI
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxSize wx_ppi = wx_dc->GetPPI();
        ppi.width = wx_ppi.x;
        ppi.height = wx_ppi.y;
    }
    return ppi;
}

double
wxd_DC_GetContentScaleFactor(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return wx_dc->GetContentScaleFactor();
    }
    return 1.0;
}

// Gradient fills
void
wxd_DC_GradientFillLinear(wxd_DC_t* dc, wxd_Rect rect, wxd_Colour_t initialColour,
                          wxd_Colour_t destColour, int direction)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxRect wx_rect(rect.x, rect.y, rect.width, rect.height);
        wxColour wx_initial(initialColour.r, initialColour.g, initialColour.b, initialColour.a);
        wxColour wx_dest(destColour.r, destColour.g, destColour.b, destColour.a);

        wxDirection wx_direction = static_cast<wxDirection>(direction);

        wx_dc->GradientFillLinear(wx_rect, wx_initial, wx_dest, wx_direction);
    }
}

void
wxd_DC_GradientFillConcentric(wxd_DC_t* dc, wxd_Rect rect, wxd_Colour_t initialColour,
                              wxd_Colour_t destColour, wxd_Point circleCenter)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxRect wx_rect(rect.x, rect.y, rect.width, rect.height);
        wxColour wx_initial(initialColour.r, initialColour.g, initialColour.b, initialColour.a);
        wxColour wx_dest(destColour.r, destColour.g, destColour.b, destColour.a);
        wxPoint wx_center(circleCenter.x, circleCenter.y);

        wx_dc->GradientFillConcentric(wx_rect, wx_initial, wx_dest, wx_center);
    }
}

// Flood fill
bool
wxd_DC_FloodFill(wxd_DC_t* dc, int x, int y, wxd_Colour_t colour, int style)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxColour wx_colour(colour.r, colour.g, colour.b, colour.a);
        wxFloodFillStyle wx_style = static_cast<wxFloodFillStyle>(style);
        return wx_dc->FloodFill(x, y, wx_colour, wx_style);
    }
    return false;
}

// Drawing modes and logical functions
void
wxd_DC_SetLogicalFunction(wxd_DC_t* dc, int function)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        wxRasterOperationMode wx_function = static_cast<wxRasterOperationMode>(function);
        wx_dc->SetLogicalFunction(wx_function);
    }
}

int
wxd_DC_GetLogicalFunction(wxd_DC_t* dc)
{
    if (dc) {
        wxDC* wx_dc = reinterpret_cast<wxDC*>(dc);
        return static_cast<int>(wx_dc->GetLogicalFunction());
    }
    return static_cast<int>(wxCOPY);
}