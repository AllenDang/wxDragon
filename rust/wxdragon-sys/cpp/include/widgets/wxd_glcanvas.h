#ifndef WXD_GLCANVAS_H
#define WXD_GLCANVAS_H

#include "../wxd_types.h"

// Return values of wxd_GLCanvas_SetSwapInterval, matching wxGLCanvas::SwapInterval.
#define WXD_GL_SWAP_INTERVAL_NOT_SET 0
#define WXD_GL_SWAP_INTERVAL_SET 1
#define WXD_GL_SWAP_INTERVAL_NON_ADAPTIVE 2

// wxGLCanvas::DefaultSwapInterval — disables the automatic VSync handling some platforms apply.
#define WXD_GL_DEFAULT_SWAP_INTERVAL 2147483647

// The WX_GL_* attribute list tokens, in the order wx/glcanvas.h declares them. glcanvas.cpp
// static_asserts every one against the wxWidgets enum, so a value that moves upstream breaks the
// build rather than producing an attribute list wxWidgets rejects at run time.
#define WXD_GL_RGBA 1
#define WXD_GL_BUFFER_SIZE 2
#define WXD_GL_LEVEL 3
#define WXD_GL_DOUBLEBUFFER 4
#define WXD_GL_STEREO 5
#define WXD_GL_AUX_BUFFERS 6
#define WXD_GL_MIN_RED 7
#define WXD_GL_MIN_GREEN 8
#define WXD_GL_MIN_BLUE 9
#define WXD_GL_MIN_ALPHA 10
#define WXD_GL_DEPTH_SIZE 11
#define WXD_GL_STENCIL_SIZE 12
#define WXD_GL_MIN_ACCUM_RED 13
#define WXD_GL_MIN_ACCUM_GREEN 14
#define WXD_GL_MIN_ACCUM_BLUE 15
#define WXD_GL_MIN_ACCUM_ALPHA 16
#define WXD_GL_SAMPLE_BUFFERS 17
#define WXD_GL_SAMPLES 18
#define WXD_GL_FRAMEBUFFER_SRGB 19
#define WXD_GL_CORE_PROFILE 20
#define WXD_GL_MAJOR_VERSION 21
#define WXD_GL_MINOR_VERSION 22
#define WXD_GL_COMPAT_PROFILE 23
#define WXD_GL_FORWARD_COMPAT 24
#define WXD_GL_ES2 25
#define WXD_GL_DEBUG 26
#define WXD_GL_ROBUST_ACCESS 27
#define WXD_GL_NO_RESET_NOTIFY 28
#define WXD_GL_LOSE_ON_RESET 29
#define WXD_GL_RESET_ISOLATION 30
#define WXD_GL_RELEASE_FLUSH 31
#define WXD_GL_RELEASE_NONE 32

// --- wxGLAttributes: pixel-format attributes for a canvas ---
//
// Each setter appends to the list and returns the same object, so calls chain. EndList closes it;
// a list is not usable until then.

WXD_EXPORTED wxd_GLAttributes_t* wxd_GLAttributes_Create(void);
WXD_EXPORTED void wxd_GLAttributes_Destroy(wxd_GLAttributes_t* self);

WXD_EXPORTED void wxd_GLAttributes_RGBA(wxd_GLAttributes_t* self);
WXD_EXPORTED void wxd_GLAttributes_BufferSize(wxd_GLAttributes_t* self, int val);
WXD_EXPORTED void wxd_GLAttributes_Level(wxd_GLAttributes_t* self, int val);
WXD_EXPORTED void wxd_GLAttributes_DoubleBuffer(wxd_GLAttributes_t* self);
WXD_EXPORTED void wxd_GLAttributes_Stereo(wxd_GLAttributes_t* self);
WXD_EXPORTED void wxd_GLAttributes_AuxBuffers(wxd_GLAttributes_t* self, int val);
WXD_EXPORTED void wxd_GLAttributes_MinRGBA(wxd_GLAttributes_t* self, int r, int g, int b, int a);
WXD_EXPORTED void wxd_GLAttributes_Depth(wxd_GLAttributes_t* self, int val);
WXD_EXPORTED void wxd_GLAttributes_Stencil(wxd_GLAttributes_t* self, int val);
WXD_EXPORTED void wxd_GLAttributes_MinAcumRGBA(wxd_GLAttributes_t* self, int r, int g, int b, int a);
WXD_EXPORTED void wxd_GLAttributes_PlatformDefaults(wxd_GLAttributes_t* self);
WXD_EXPORTED void wxd_GLAttributes_SampleBuffers(wxd_GLAttributes_t* self, int val);
WXD_EXPORTED void wxd_GLAttributes_Samplers(wxd_GLAttributes_t* self, int val);
WXD_EXPORTED void wxd_GLAttributes_FrameBuffersRGB(wxd_GLAttributes_t* self);
WXD_EXPORTED void wxd_GLAttributes_Defaults(wxd_GLAttributes_t* self);
WXD_EXPORTED void wxd_GLAttributes_EndList(wxd_GLAttributes_t* self);

// Shared with wxGLAttribsBase.
WXD_EXPORTED void wxd_GLAttributes_AddAttribute(wxd_GLAttributes_t* self, int attribute);
WXD_EXPORTED void wxd_GLAttributes_AddAttribBits(wxd_GLAttributes_t* self, int searchVal, int combineVal);
WXD_EXPORTED void wxd_GLAttributes_SetNeedsARB(wxd_GLAttributes_t* self, bool needsARB);
WXD_EXPORTED bool wxd_GLAttributes_NeedsARB(wxd_GLAttributes_t* self);
WXD_EXPORTED void wxd_GLAttributes_Reset(wxd_GLAttributes_t* self);
WXD_EXPORTED int wxd_GLAttributes_GetSize(wxd_GLAttributes_t* self);
// Borrowed, and valid only while the object is unchanged. Null for an empty list.
WXD_EXPORTED const int* wxd_GLAttributes_GetGLAttrs(wxd_GLAttributes_t* self);

// --- wxGLContextAttrs: attributes for a rendering context ---

WXD_EXPORTED wxd_GLContextAttrs_t* wxd_GLContextAttrs_Create(void);
WXD_EXPORTED void wxd_GLContextAttrs_Destroy(wxd_GLContextAttrs_t* self);

WXD_EXPORTED void wxd_GLContextAttrs_CoreProfile(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_MajorVersion(wxd_GLContextAttrs_t* self, int val);
WXD_EXPORTED void wxd_GLContextAttrs_MinorVersion(wxd_GLContextAttrs_t* self, int val);
WXD_EXPORTED void wxd_GLContextAttrs_OGLVersion(wxd_GLContextAttrs_t* self, int major, int minor);
WXD_EXPORTED void wxd_GLContextAttrs_CompatibilityProfile(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_ForwardCompatible(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_ES2(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_DebugCtx(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_Robust(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_NoResetNotify(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_LoseOnReset(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_ResetIsolation(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_ReleaseFlush(wxd_GLContextAttrs_t* self, int val);
WXD_EXPORTED void wxd_GLContextAttrs_PlatformDefaults(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_EndList(wxd_GLContextAttrs_t* self);

WXD_EXPORTED void wxd_GLContextAttrs_AddAttribute(wxd_GLContextAttrs_t* self, int attribute);
WXD_EXPORTED void wxd_GLContextAttrs_AddAttribBits(wxd_GLContextAttrs_t* self, int searchVal, int combineVal);
WXD_EXPORTED void wxd_GLContextAttrs_SetNeedsARB(wxd_GLContextAttrs_t* self, bool needsARB);
WXD_EXPORTED bool wxd_GLContextAttrs_NeedsARB(wxd_GLContextAttrs_t* self);
WXD_EXPORTED void wxd_GLContextAttrs_Reset(wxd_GLContextAttrs_t* self);
WXD_EXPORTED int wxd_GLContextAttrs_GetSize(wxd_GLContextAttrs_t* self);
WXD_EXPORTED const int* wxd_GLContextAttrs_GetGLAttrs(wxd_GLContextAttrs_t* self);

// --- wxGLCanvas ---

// Both wxWidgets constructors. `attribList` is a 0-terminated list of WX_GL_* values; null asks
// for the default format.
WXD_EXPORTED wxd_GLCanvas_t*
wxd_GLCanvas_Create(wxd_Window_t* parent, int id, const int* attribList, int x, int y, int w, int h,
                    int64_t style);
WXD_EXPORTED wxd_GLCanvas_t*
wxd_GLCanvas_CreateWithAttributes(wxd_Window_t* parent, wxd_GLAttributes_t* dispAttrs, int id,
                                  int x, int y, int w, int h, int64_t style);

// Whether the display supports the given format, without creating a canvas.
WXD_EXPORTED bool wxd_GLCanvas_IsDisplaySupported(const int* attribList);
WXD_EXPORTED bool wxd_GLCanvas_IsDisplaySupportedAttrs(wxd_GLAttributes_t* dispAttrs);

WXD_EXPORTED bool wxd_GLCanvas_SwapBuffers(wxd_GLCanvas_t* self);
WXD_EXPORTED bool wxd_GLCanvas_SetCurrent(wxd_GLCanvas_t* self, wxd_GLContext_t* context);

// One of the WXD_GL_SWAP_INTERVAL_* values above.
WXD_EXPORTED int wxd_GLCanvas_SetSwapInterval(wxd_GLCanvas_t* self, int interval);
WXD_EXPORTED int wxd_GLCanvas_GetSwapInterval(wxd_GLCanvas_t* self);

// glColor for a named colour. False when the name is unknown.
WXD_EXPORTED bool wxd_GLCanvas_SetColour(wxd_GLCanvas_t* self, const char* colour);

WXD_EXPORTED bool wxd_GLCanvas_IsExtensionSupported(const char* extension);
WXD_EXPORTED bool wxd_GLCanvas_IsExtensionInList(const char* list, const char* extension);

// Split a 0-terminated attribute list into pixel-format and context attributes. False when the
// list holds an attribute wxWidgets does not know. `ctxAttrs` may be null.
WXD_EXPORTED bool wxd_GLCanvas_ParseAttribList(const int* attribList, wxd_GLAttributes_t* dispAttrs,
                                               wxd_GLContextAttrs_t* ctxAttrs);

// The context attributes taken from the `int*` list this canvas was built with. Borrowed.
WXD_EXPORTED wxd_GLContextAttrs_t* wxd_GLCanvas_GetGLCTXAttrs(wxd_GLCanvas_t* self);

// The canvas size in physical pixels, which is what a GL viewport wants. On a scaled display this
// differs from the window size in logical units.
WXD_EXPORTED void wxd_GLCanvas_GetPixelSize(wxd_GLCanvas_t* self, int* width, int* height);

// --- wxGLContext ---

// `other` shares objects with the new context when not null; `ctxAttrs` may be null for defaults.
WXD_EXPORTED wxd_GLContext_t*
wxd_GLContext_Create(wxd_GLCanvas_t* canvas, wxd_GLContext_t* other, wxd_GLContextAttrs_t* ctxAttrs);
WXD_EXPORTED void wxd_GLContext_Destroy(wxd_GLContext_t* self);

WXD_EXPORTED bool wxd_GLContext_SetCurrent(wxd_GLContext_t* self, wxd_GLCanvas_t* canvas);
WXD_EXPORTED void wxd_GLContext_ClearCurrent(void);
WXD_EXPORTED bool wxd_GLContext_IsOK(wxd_GLContext_t* self);

// Address of a GL entry point, or null when unavailable. Feed this to a GL loader rather than
// resolving symbols per platform.
WXD_EXPORTED void* wxd_GLContext_GetProcAddress(const char* name);

#endif // WXD_GLCANVAS_H
