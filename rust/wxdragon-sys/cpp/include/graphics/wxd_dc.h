#ifndef WXD_DC_H
#define WXD_DC_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Opaque DC type declarations
typedef struct wxd_DC_t wxd_DC_t;
typedef struct wxd_WindowDC_t wxd_WindowDC_t;
typedef struct wxd_ClientDC_t wxd_ClientDC_t;
typedef struct wxd_PaintDC_t wxd_PaintDC_t;
typedef struct wxd_MemoryDC_t wxd_MemoryDC_t;
typedef struct wxd_ScreenDC_t wxd_ScreenDC_t;
typedef struct wxd_AutoBufferedPaintDC_t wxd_AutoBufferedPaintDC_t;

// DC Creation/Destruction
WXD_EXPORTED wxd_WindowDC_t*
wxd_WindowDC_Create(wxd_Window_t* window);

WXD_EXPORTED void
wxd_WindowDC_Destroy(wxd_WindowDC_t* dc);

WXD_EXPORTED wxd_ClientDC_t*
wxd_ClientDC_Create(wxd_Window_t* window);

WXD_EXPORTED void
wxd_ClientDC_Destroy(wxd_ClientDC_t* dc);

WXD_EXPORTED wxd_PaintDC_t*
wxd_PaintDC_Create(wxd_Window_t* window);

WXD_EXPORTED void
wxd_PaintDC_Destroy(wxd_PaintDC_t* dc);

WXD_EXPORTED wxd_MemoryDC_t*
wxd_MemoryDC_Create(void);

WXD_EXPORTED void
wxd_MemoryDC_Destroy(wxd_MemoryDC_t* dc);

WXD_EXPORTED wxd_ScreenDC_t*
wxd_ScreenDC_Create(void);

WXD_EXPORTED void
wxd_ScreenDC_Destroy(wxd_ScreenDC_t* dc);

WXD_EXPORTED wxd_AutoBufferedPaintDC_t*
wxd_AutoBufferedPaintDC_Create(wxd_Window_t* window);

WXD_EXPORTED void
wxd_AutoBufferedPaintDC_Destroy(wxd_AutoBufferedPaintDC_t* dc);

// Common DC operations (operates on the base type)
WXD_EXPORTED void
wxd_DC_Clear(wxd_DC_t* dc);

WXD_EXPORTED void
wxd_DC_SetBackground(wxd_DC_t* dc, wxd_Colour_t background);

WXD_EXPORTED void
wxd_DC_SetBackgroundMode(wxd_DC_t* dc, int mode); // WXD_TRANSPARENT or WXD_SOLID

WXD_EXPORTED void
wxd_DC_SetTextBackground(wxd_DC_t* dc, wxd_Colour_t colour);

WXD_EXPORTED void
wxd_DC_SetTextForeground(wxd_DC_t* dc, wxd_Colour_t colour);

WXD_EXPORTED void
wxd_DC_SetFont(wxd_DC_t* dc, const wxd_Font_t* font);

WXD_EXPORTED void
wxd_DC_SetPen(wxd_DC_t* dc, wxd_Colour_t colour, int width, int style);

WXD_EXPORTED void
wxd_DC_SetBrush(wxd_DC_t* dc, wxd_Colour_t colour, int style);

// Basic drawing operations
WXD_EXPORTED void
wxd_DC_DrawPoint(wxd_DC_t* dc, int x, int y);

WXD_EXPORTED void
wxd_DC_DrawLine(wxd_DC_t* dc, int x1, int y1, int x2, int y2);

WXD_EXPORTED void
wxd_DC_DrawRectangle(wxd_DC_t* dc, int x, int y, int width, int height);

WXD_EXPORTED void
wxd_DC_DrawCircle(wxd_DC_t* dc, int x, int y, int radius);

WXD_EXPORTED void
wxd_DC_DrawEllipse(wxd_DC_t* dc, int x, int y, int width, int height);

// Advanced drawing operations
WXD_EXPORTED void
wxd_DC_DrawRoundedRectangle(wxd_DC_t* dc, int x, int y, int width, int height, double radius);

WXD_EXPORTED void
wxd_DC_DrawArc(wxd_DC_t* dc, int x1, int y1, int x2, int y2, int xc, int yc);

WXD_EXPORTED void
wxd_DC_DrawEllipticArc(wxd_DC_t* dc, int x, int y, int width, int height, double start, double end);

WXD_EXPORTED void
wxd_DC_DrawPolygon(wxd_DC_t* dc, int n, wxd_Point* points, int xoffset, int yoffset,
                   int fill_style);

WXD_EXPORTED void
wxd_DC_DrawLines(wxd_DC_t* dc, int n, wxd_Point* points, int xoffset, int yoffset);

WXD_EXPORTED void
wxd_DC_DrawSpline(wxd_DC_t* dc, int n, wxd_Point* points);

// Text drawing operations
WXD_EXPORTED void
wxd_DC_DrawText(wxd_DC_t* dc, const char* text, int x, int y);

WXD_EXPORTED void
wxd_DC_DrawRotatedText(wxd_DC_t* dc, const char* text, int x, int y, double angle);

WXD_EXPORTED void
wxd_DC_DrawLabel(wxd_DC_t* dc, const char* text, wxd_Rect rect, int alignment, int indexAccel);

// Bitmap operations
WXD_EXPORTED void
wxd_DC_DrawBitmap(wxd_DC_t* dc, const wxd_Bitmap_t* bitmap, int x, int y, bool transparent);

WXD_EXPORTED bool
wxd_DC_Blit(wxd_DC_t* dest_dc, int xdest, int ydest, int width, int height, wxd_DC_t* source_dc,
            int xsrc, int ysrc, int logical_func, bool use_mask, int xsrc_mask, int ysrc_mask);

WXD_EXPORTED bool
wxd_DC_StretchBlit(wxd_DC_t* dest_dc, int xdest, int ydest, int dstWidth, int dstHeight,
                   wxd_DC_t* source_dc, int xsrc, int ysrc, int srcWidth, int srcHeight,
                   int logical_func, bool use_mask, int xsrc_mask, int ysrc_mask);

// Clipping operations
WXD_EXPORTED void
wxd_DC_SetClippingRegion(wxd_DC_t* dc, int x, int y, int width, int height);

WXD_EXPORTED void
wxd_DC_SetClippingRegionFromPoints(wxd_DC_t* dc, int n, wxd_Point* points);

WXD_EXPORTED void
wxd_DC_DestroyClippingRegion(wxd_DC_t* dc);

WXD_EXPORTED void
wxd_DC_GetClippingBox(wxd_DC_t* dc, int* x, int* y, int* width, int* height);

// Coordinate transformation
WXD_EXPORTED void
wxd_DC_SetDeviceOrigin(wxd_DC_t* dc, int x, int y);

WXD_EXPORTED void
wxd_DC_SetLogicalOrigin(wxd_DC_t* dc, int x, int y);

WXD_EXPORTED void
wxd_DC_SetUserScale(wxd_DC_t* dc, double x_scale, double y_scale);

WXD_EXPORTED void
wxd_DC_SetLogicalScale(wxd_DC_t* dc, double x_scale, double y_scale);

WXD_EXPORTED void
wxd_DC_SetMapMode(wxd_DC_t* dc, int mode);

// Get transformation info
WXD_EXPORTED wxd_Point
wxd_DC_GetDeviceOrigin(wxd_DC_t* dc);

WXD_EXPORTED wxd_Point
wxd_DC_GetLogicalOrigin(wxd_DC_t* dc);

WXD_EXPORTED void
wxd_DC_GetUserScale(wxd_DC_t* dc, double* x_scale, double* y_scale);

WXD_EXPORTED void
wxd_DC_GetLogicalScale(wxd_DC_t* dc, double* x_scale, double* y_scale);

WXD_EXPORTED int
wxd_DC_GetMapMode(wxd_DC_t* dc);

// Coordinate conversion
WXD_EXPORTED int
wxd_DC_DeviceToLogicalX(wxd_DC_t* dc, int x);

WXD_EXPORTED int
wxd_DC_DeviceToLogicalY(wxd_DC_t* dc, int y);

WXD_EXPORTED int
wxd_DC_LogicalToDeviceX(wxd_DC_t* dc, int x);

WXD_EXPORTED int
wxd_DC_LogicalToDeviceY(wxd_DC_t* dc, int y);

// Device capabilities
WXD_EXPORTED wxd_Size
wxd_DC_GetSize(wxd_DC_t* dc);

WXD_EXPORTED wxd_Size
wxd_DC_GetSizeMM(wxd_DC_t* dc);

WXD_EXPORTED void
wxd_DC_GetTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h);

WXD_EXPORTED void
wxd_DC_GetFullTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h, int* descent,
                         int* externalLeading, const wxd_Font_t* font);

WXD_EXPORTED void
wxd_DC_GetMultiLineTextExtent(wxd_DC_t* dc, const char* string, int* w, int* h, int* heightLine,
                              const wxd_Font_t* font);

WXD_EXPORTED int
wxd_DC_GetCharHeight(wxd_DC_t* dc);

WXD_EXPORTED int
wxd_DC_GetCharWidth(wxd_DC_t* dc);

// Drawing state queries
WXD_EXPORTED wxd_Colour_t
wxd_DC_GetBackground(wxd_DC_t* dc);

WXD_EXPORTED int
wxd_DC_GetBackgroundMode(wxd_DC_t* dc);

WXD_EXPORTED wxd_Colour_t
wxd_DC_GetTextBackground(wxd_DC_t* dc);

WXD_EXPORTED wxd_Colour_t
wxd_DC_GetTextForeground(wxd_DC_t* dc);

// DPI and scaling
WXD_EXPORTED wxd_Size
wxd_DC_GetPPI(wxd_DC_t* dc);

WXD_EXPORTED double
wxd_DC_GetContentScaleFactor(wxd_DC_t* dc);

// Gradient fills
WXD_EXPORTED void
wxd_DC_GradientFillLinear(wxd_DC_t* dc, wxd_Rect rect, wxd_Colour_t initialColour,
                          wxd_Colour_t destColour, int direction);

WXD_EXPORTED void
wxd_DC_GradientFillConcentric(wxd_DC_t* dc, wxd_Rect rect, wxd_Colour_t initialColour,
                              wxd_Colour_t destColour, wxd_Point circleCenter);

// Flood fill
WXD_EXPORTED bool
wxd_DC_FloodFill(wxd_DC_t* dc, int x, int y, wxd_Colour_t colour, int style);

// Drawing modes and logical functions
WXD_EXPORTED void
wxd_DC_SetLogicalFunction(wxd_DC_t* dc, int function);

WXD_EXPORTED int
wxd_DC_GetLogicalFunction(wxd_DC_t* dc);

// MemoryDC specific operations
WXD_EXPORTED void
wxd_MemoryDC_SelectObject(wxd_MemoryDC_t* dc, const wxd_Bitmap_t* bitmap);

WXD_EXPORTED void
wxd_MemoryDC_SelectObjectAsSource(wxd_MemoryDC_t* dc, const wxd_Bitmap_t* bitmap);

// Type casting functions (for safely using base DC functions with derived types)
WXD_EXPORTED wxd_DC_t*
wxd_WindowDC_AsDC(wxd_WindowDC_t* dc);

WXD_EXPORTED wxd_DC_t*
wxd_ClientDC_AsDC(wxd_ClientDC_t* dc);

WXD_EXPORTED wxd_DC_t*
wxd_PaintDC_AsDC(wxd_PaintDC_t* dc);

WXD_EXPORTED wxd_DC_t*
wxd_MemoryDC_AsDC(wxd_MemoryDC_t* dc);

WXD_EXPORTED wxd_DC_t*
wxd_ScreenDC_AsDC(wxd_ScreenDC_t* dc);

WXD_EXPORTED wxd_DC_t*
wxd_AutoBufferedPaintDC_AsDC(wxd_AutoBufferedPaintDC_t* dc);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WXD_DC_H