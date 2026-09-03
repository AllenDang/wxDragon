#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"

#if wxUSE_GLCANVAS

#include <wx/glcanvas.h>

// The header's WXD_GL_* values are what a caller builds an attribute list from, and wxWidgets
// rejects a list holding anything it does not recognise. Checked here so a value that moves
// upstream is a build failure rather than a run-time assert inside ParseAttribList.
#define WXD_GL_CHECK(ours, theirs) \
    static_assert((ours) == (theirs), #theirs " does not match the wxWidgets enum")

WXD_GL_CHECK(WXD_GL_RGBA, WX_GL_RGBA);
WXD_GL_CHECK(WXD_GL_BUFFER_SIZE, WX_GL_BUFFER_SIZE);
WXD_GL_CHECK(WXD_GL_LEVEL, WX_GL_LEVEL);
WXD_GL_CHECK(WXD_GL_DOUBLEBUFFER, WX_GL_DOUBLEBUFFER);
WXD_GL_CHECK(WXD_GL_STEREO, WX_GL_STEREO);
WXD_GL_CHECK(WXD_GL_AUX_BUFFERS, WX_GL_AUX_BUFFERS);
WXD_GL_CHECK(WXD_GL_MIN_RED, WX_GL_MIN_RED);
WXD_GL_CHECK(WXD_GL_MIN_GREEN, WX_GL_MIN_GREEN);
WXD_GL_CHECK(WXD_GL_MIN_BLUE, WX_GL_MIN_BLUE);
WXD_GL_CHECK(WXD_GL_MIN_ALPHA, WX_GL_MIN_ALPHA);
WXD_GL_CHECK(WXD_GL_DEPTH_SIZE, WX_GL_DEPTH_SIZE);
WXD_GL_CHECK(WXD_GL_STENCIL_SIZE, WX_GL_STENCIL_SIZE);
WXD_GL_CHECK(WXD_GL_MIN_ACCUM_RED, WX_GL_MIN_ACCUM_RED);
WXD_GL_CHECK(WXD_GL_MIN_ACCUM_GREEN, WX_GL_MIN_ACCUM_GREEN);
WXD_GL_CHECK(WXD_GL_MIN_ACCUM_BLUE, WX_GL_MIN_ACCUM_BLUE);
WXD_GL_CHECK(WXD_GL_MIN_ACCUM_ALPHA, WX_GL_MIN_ACCUM_ALPHA);
WXD_GL_CHECK(WXD_GL_SAMPLE_BUFFERS, WX_GL_SAMPLE_BUFFERS);
WXD_GL_CHECK(WXD_GL_SAMPLES, WX_GL_SAMPLES);
WXD_GL_CHECK(WXD_GL_FRAMEBUFFER_SRGB, WX_GL_FRAMEBUFFER_SRGB);
WXD_GL_CHECK(WXD_GL_CORE_PROFILE, WX_GL_CORE_PROFILE);
WXD_GL_CHECK(WXD_GL_MAJOR_VERSION, WX_GL_MAJOR_VERSION);
WXD_GL_CHECK(WXD_GL_MINOR_VERSION, WX_GL_MINOR_VERSION);
WXD_GL_CHECK(WXD_GL_COMPAT_PROFILE, WX_GL_COMPAT_PROFILE);
WXD_GL_CHECK(WXD_GL_FORWARD_COMPAT, WX_GL_FORWARD_COMPAT);
WXD_GL_CHECK(WXD_GL_ES2, WX_GL_ES2);
WXD_GL_CHECK(WXD_GL_DEBUG, WX_GL_DEBUG);
WXD_GL_CHECK(WXD_GL_ROBUST_ACCESS, WX_GL_ROBUST_ACCESS);
WXD_GL_CHECK(WXD_GL_NO_RESET_NOTIFY, WX_GL_NO_RESET_NOTIFY);
WXD_GL_CHECK(WXD_GL_LOSE_ON_RESET, WX_GL_LOSE_ON_RESET);
WXD_GL_CHECK(WXD_GL_RESET_ISOLATION, WX_GL_RESET_ISOLATION);
WXD_GL_CHECK(WXD_GL_RELEASE_FLUSH, WX_GL_RELEASE_FLUSH);
WXD_GL_CHECK(WXD_GL_RELEASE_NONE, WX_GL_RELEASE_NONE);

static_assert((int)wxGLCanvas::SwapInterval::NotSet == WXD_GL_SWAP_INTERVAL_NOT_SET, "SwapInterval");
static_assert((int)wxGLCanvas::SwapInterval::Set == WXD_GL_SWAP_INTERVAL_SET, "SwapInterval");
static_assert((int)wxGLCanvas::SwapInterval::NonAdaptive == WXD_GL_SWAP_INTERVAL_NON_ADAPTIVE, "SwapInterval");
static_assert(wxGLCanvas::DefaultSwapInterval == WXD_GL_DEFAULT_SWAP_INTERVAL, "DefaultSwapInterval");

#define WXD_GLATTRS(p) ((wxGLAttributes*)(p))
#define WXD_GLCTXATTRS(p) ((wxGLContextAttrs*)(p))
#define WXD_GLCANVAS(p) ((wxGLCanvas*)(p))
#define WXD_GLCONTEXT(p) ((wxGLContext*)(p))

// --- wxGLAttributes ---

WXD_EXPORTED wxd_GLAttributes_t* wxd_GLAttributes_Create(void)
{
    return (wxd_GLAttributes_t*)new wxGLAttributes();
}

WXD_EXPORTED void wxd_GLAttributes_Destroy(wxd_GLAttributes_t* self)
{
    delete WXD_GLATTRS(self);
}

WXD_EXPORTED void wxd_GLAttributes_RGBA(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->RGBA();
}

WXD_EXPORTED void wxd_GLAttributes_BufferSize(wxd_GLAttributes_t* self, int val)
{
    if (self) WXD_GLATTRS(self)->BufferSize(val);
}

WXD_EXPORTED void wxd_GLAttributes_Level(wxd_GLAttributes_t* self, int val)
{
    if (self) WXD_GLATTRS(self)->Level(val);
}

WXD_EXPORTED void wxd_GLAttributes_DoubleBuffer(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->DoubleBuffer();
}

WXD_EXPORTED void wxd_GLAttributes_Stereo(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->Stereo();
}

WXD_EXPORTED void wxd_GLAttributes_AuxBuffers(wxd_GLAttributes_t* self, int val)
{
    if (self) WXD_GLATTRS(self)->AuxBuffers(val);
}

WXD_EXPORTED void wxd_GLAttributes_MinRGBA(wxd_GLAttributes_t* self, int r, int g, int b, int a)
{
    if (self) WXD_GLATTRS(self)->MinRGBA(r, g, b, a);
}

WXD_EXPORTED void wxd_GLAttributes_Depth(wxd_GLAttributes_t* self, int val)
{
    if (self) WXD_GLATTRS(self)->Depth(val);
}

WXD_EXPORTED void wxd_GLAttributes_Stencil(wxd_GLAttributes_t* self, int val)
{
    if (self) WXD_GLATTRS(self)->Stencil(val);
}

WXD_EXPORTED void wxd_GLAttributes_MinAcumRGBA(wxd_GLAttributes_t* self, int r, int g, int b, int a)
{
    if (self) WXD_GLATTRS(self)->MinAcumRGBA(r, g, b, a);
}

WXD_EXPORTED void wxd_GLAttributes_PlatformDefaults(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->PlatformDefaults();
}

WXD_EXPORTED void wxd_GLAttributes_SampleBuffers(wxd_GLAttributes_t* self, int val)
{
    if (self) WXD_GLATTRS(self)->SampleBuffers(val);
}

WXD_EXPORTED void wxd_GLAttributes_Samplers(wxd_GLAttributes_t* self, int val)
{
    if (self) WXD_GLATTRS(self)->Samplers(val);
}

WXD_EXPORTED void wxd_GLAttributes_FrameBuffersRGB(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->FrameBuffersRGB();
}

WXD_EXPORTED void wxd_GLAttributes_Defaults(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->Defaults();
}

WXD_EXPORTED void wxd_GLAttributes_EndList(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->EndList();
}

WXD_EXPORTED void wxd_GLAttributes_AddAttribute(wxd_GLAttributes_t* self, int attribute)
{
    if (self) WXD_GLATTRS(self)->AddAttribute(attribute);
}

WXD_EXPORTED void wxd_GLAttributes_AddAttribBits(wxd_GLAttributes_t* self, int searchVal, int combineVal)
{
    if (self) WXD_GLATTRS(self)->AddAttribBits(searchVal, combineVal);
}

WXD_EXPORTED void wxd_GLAttributes_SetNeedsARB(wxd_GLAttributes_t* self, bool needsARB)
{
    if (self) WXD_GLATTRS(self)->SetNeedsARB(needsARB);
}

WXD_EXPORTED bool wxd_GLAttributes_NeedsARB(wxd_GLAttributes_t* self)
{
    return self && WXD_GLATTRS(self)->NeedsARB();
}

WXD_EXPORTED void wxd_GLAttributes_Reset(wxd_GLAttributes_t* self)
{
    if (self) WXD_GLATTRS(self)->Reset();
}

WXD_EXPORTED int wxd_GLAttributes_GetSize(wxd_GLAttributes_t* self)
{
    return self ? WXD_GLATTRS(self)->GetSize() : 0;
}

WXD_EXPORTED const int* wxd_GLAttributes_GetGLAttrs(wxd_GLAttributes_t* self)
{
    return self ? WXD_GLATTRS(self)->GetGLAttrs() : nullptr;
}

// --- wxGLContextAttrs ---

WXD_EXPORTED wxd_GLContextAttrs_t* wxd_GLContextAttrs_Create(void)
{
    return (wxd_GLContextAttrs_t*)new wxGLContextAttrs();
}

WXD_EXPORTED void wxd_GLContextAttrs_Destroy(wxd_GLContextAttrs_t* self)
{
    delete WXD_GLCTXATTRS(self);
}

WXD_EXPORTED void wxd_GLContextAttrs_CoreProfile(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->CoreProfile();
}

WXD_EXPORTED void wxd_GLContextAttrs_MajorVersion(wxd_GLContextAttrs_t* self, int val)
{
    if (self) WXD_GLCTXATTRS(self)->MajorVersion(val);
}

WXD_EXPORTED void wxd_GLContextAttrs_MinorVersion(wxd_GLContextAttrs_t* self, int val)
{
    if (self) WXD_GLCTXATTRS(self)->MinorVersion(val);
}

WXD_EXPORTED void wxd_GLContextAttrs_OGLVersion(wxd_GLContextAttrs_t* self, int major, int minor)
{
    if (self) WXD_GLCTXATTRS(self)->OGLVersion(major, minor);
}

WXD_EXPORTED void wxd_GLContextAttrs_CompatibilityProfile(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->CompatibilityProfile();
}

WXD_EXPORTED void wxd_GLContextAttrs_ForwardCompatible(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->ForwardCompatible();
}

WXD_EXPORTED void wxd_GLContextAttrs_ES2(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->ES2();
}

WXD_EXPORTED void wxd_GLContextAttrs_DebugCtx(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->DebugCtx();
}

WXD_EXPORTED void wxd_GLContextAttrs_Robust(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->Robust();
}

WXD_EXPORTED void wxd_GLContextAttrs_NoResetNotify(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->NoResetNotify();
}

WXD_EXPORTED void wxd_GLContextAttrs_LoseOnReset(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->LoseOnReset();
}

WXD_EXPORTED void wxd_GLContextAttrs_ResetIsolation(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->ResetIsolation();
}

WXD_EXPORTED void wxd_GLContextAttrs_ReleaseFlush(wxd_GLContextAttrs_t* self, int val)
{
    if (self) WXD_GLCTXATTRS(self)->ReleaseFlush(val);
}

WXD_EXPORTED void wxd_GLContextAttrs_PlatformDefaults(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->PlatformDefaults();
}

WXD_EXPORTED void wxd_GLContextAttrs_EndList(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->EndList();
}

WXD_EXPORTED void wxd_GLContextAttrs_AddAttribute(wxd_GLContextAttrs_t* self, int attribute)
{
    if (self) WXD_GLCTXATTRS(self)->AddAttribute(attribute);
}

WXD_EXPORTED void wxd_GLContextAttrs_AddAttribBits(wxd_GLContextAttrs_t* self, int searchVal, int combineVal)
{
    if (self) WXD_GLCTXATTRS(self)->AddAttribBits(searchVal, combineVal);
}

WXD_EXPORTED void wxd_GLContextAttrs_SetNeedsARB(wxd_GLContextAttrs_t* self, bool needsARB)
{
    if (self) WXD_GLCTXATTRS(self)->SetNeedsARB(needsARB);
}

WXD_EXPORTED bool wxd_GLContextAttrs_NeedsARB(wxd_GLContextAttrs_t* self)
{
    return self && WXD_GLCTXATTRS(self)->NeedsARB();
}

WXD_EXPORTED void wxd_GLContextAttrs_Reset(wxd_GLContextAttrs_t* self)
{
    if (self) WXD_GLCTXATTRS(self)->Reset();
}

WXD_EXPORTED int wxd_GLContextAttrs_GetSize(wxd_GLContextAttrs_t* self)
{
    return self ? WXD_GLCTXATTRS(self)->GetSize() : 0;
}

WXD_EXPORTED const int* wxd_GLContextAttrs_GetGLAttrs(wxd_GLContextAttrs_t* self)
{
    return self ? WXD_GLCTXATTRS(self)->GetGLAttrs() : nullptr;
}

// --- wxGLCanvas ---

WXD_EXPORTED wxd_GLCanvas_t*
wxd_GLCanvas_Create(wxd_Window_t* parent, int id, const int* attribList, int x, int y, int w, int h,
                    int64_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin) {
        WXD_LOG_ERROR("wxd_GLCanvas_Create: parent is null.");
        return nullptr;
    }

    // Refused rather than built blind: a canvas made for an unsupported format fails later, at the
    // first draw, where the caller can no longer choose a different way of presenting.
    if (!wxGLCanvas::IsDisplaySupported(attribList)) {
        WXD_LOG_ERROR("wxd_GLCanvas_Create: the display does not support the requested attributes.");
        return nullptr;
    }

    wxGLCanvas* canvas = new wxGLCanvas(parentWin, (wxWindowID)id, attribList, wxPoint(x, y),
                                        wxSize(w, h), (long)style);
    return (wxd_GLCanvas_t*)canvas;
}

WXD_EXPORTED wxd_GLCanvas_t*
wxd_GLCanvas_CreateWithAttributes(wxd_Window_t* parent, wxd_GLAttributes_t* dispAttrs, int id,
                                  int x, int y, int w, int h, int64_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin || !dispAttrs) {
        WXD_LOG_ERROR("wxd_GLCanvas_CreateWithAttributes: parent or attributes is null.");
        return nullptr;
    }

    const wxGLAttributes& attrs = *WXD_GLATTRS(dispAttrs);
    if (!wxGLCanvas::IsDisplaySupported(attrs)) {
        WXD_LOG_ERROR("wxd_GLCanvas_CreateWithAttributes: the display does not support the requested attributes.");
        return nullptr;
    }

    wxGLCanvas* canvas = new wxGLCanvas(parentWin, attrs, (wxWindowID)id, wxPoint(x, y),
                                        wxSize(w, h), (long)style);
    return (wxd_GLCanvas_t*)canvas;
}

WXD_EXPORTED bool wxd_GLCanvas_IsDisplaySupported(const int* attribList)
{
    return wxGLCanvas::IsDisplaySupported(attribList);
}

WXD_EXPORTED bool wxd_GLCanvas_IsDisplaySupportedAttrs(wxd_GLAttributes_t* dispAttrs)
{
    return dispAttrs && wxGLCanvas::IsDisplaySupported(*WXD_GLATTRS(dispAttrs));
}

WXD_EXPORTED bool wxd_GLCanvas_SwapBuffers(wxd_GLCanvas_t* self)
{
    return self && WXD_GLCANVAS(self)->SwapBuffers();
}

WXD_EXPORTED bool wxd_GLCanvas_SetCurrent(wxd_GLCanvas_t* self, wxd_GLContext_t* context)
{
    if (!self || !context) {
        return false;
    }
    return WXD_GLCANVAS(self)->SetCurrent(*WXD_GLCONTEXT(context));
}

WXD_EXPORTED int wxd_GLCanvas_SetSwapInterval(wxd_GLCanvas_t* self, int interval)
{
    if (!self) {
        return WXD_GL_SWAP_INTERVAL_NOT_SET;
    }
    return (int)WXD_GLCANVAS(self)->SetSwapInterval(interval);
}

WXD_EXPORTED int wxd_GLCanvas_GetSwapInterval(wxd_GLCanvas_t* self)
{
    return self ? WXD_GLCANVAS(self)->GetSwapInterval() : WXD_GL_DEFAULT_SWAP_INTERVAL;
}

WXD_EXPORTED bool wxd_GLCanvas_SetColour(wxd_GLCanvas_t* self, const char* colour)
{
    if (!self || !colour) {
        return false;
    }
    return WXD_GLCANVAS(self)->SetColour(wxString::FromUTF8(colour));
}

WXD_EXPORTED bool wxd_GLCanvas_IsExtensionSupported(const char* extension)
{
    return extension && wxGLCanvas::IsExtensionSupported(extension);
}

WXD_EXPORTED bool wxd_GLCanvas_IsExtensionInList(const char* list, const char* extension)
{
    return list && extension && wxGLCanvas::IsExtensionInList(list, extension);
}

WXD_EXPORTED bool wxd_GLCanvas_ParseAttribList(const int* attribList, wxd_GLAttributes_t* dispAttrs,
                                               wxd_GLContextAttrs_t* ctxAttrs)
{
    if (!attribList || !dispAttrs) {
        return false;
    }
    return wxGLCanvas::ParseAttribList(attribList, *WXD_GLATTRS(dispAttrs),
                                       ctxAttrs ? WXD_GLCTXATTRS(ctxAttrs) : nullptr);
}

WXD_EXPORTED wxd_GLContextAttrs_t* wxd_GLCanvas_GetGLCTXAttrs(wxd_GLCanvas_t* self)
{
    if (!self) {
        return nullptr;
    }
    return (wxd_GLContextAttrs_t*)&WXD_GLCANVAS(self)->GetGLCTXAttrs();
}

WXD_EXPORTED void wxd_GLCanvas_GetPixelSize(wxd_GLCanvas_t* self, int* width, int* height)
{
    if (!self) {
        if (width) *width = 0;
        if (height) *height = 0;
        return;
    }
    // Logical units times the content scale: on a Retina or fractional-scaling display the two
    // differ, and a viewport sized in logical units renders to part of the surface.
    wxGLCanvas* canvas = WXD_GLCANVAS(self);
    const wxSize size = canvas->GetClientSize() * canvas->GetContentScaleFactor();
    if (width) *width = size.x;
    if (height) *height = size.y;
}

// --- wxGLContext ---

WXD_EXPORTED wxd_GLContext_t*
wxd_GLContext_Create(wxd_GLCanvas_t* canvas, wxd_GLContext_t* other, wxd_GLContextAttrs_t* ctxAttrs)
{
    if (!canvas) {
        WXD_LOG_ERROR("wxd_GLContext_Create: canvas is null.");
        return nullptr;
    }

    wxGLContext* ctx = new wxGLContext(WXD_GLCANVAS(canvas), WXD_GLCONTEXT(other),
                                       ctxAttrs ? WXD_GLCTXATTRS(ctxAttrs) : nullptr);
    if (!ctx->IsOK()) {
        WXD_LOG_ERROR("wxd_GLContext_Create: the context was created but reports not OK.");
        delete ctx;
        return nullptr;
    }
    return (wxd_GLContext_t*)ctx;
}

WXD_EXPORTED void wxd_GLContext_Destroy(wxd_GLContext_t* self)
{
    delete WXD_GLCONTEXT(self);
}

WXD_EXPORTED bool wxd_GLContext_SetCurrent(wxd_GLContext_t* self, wxd_GLCanvas_t* canvas)
{
    if (!self || !canvas) {
        return false;
    }
    return WXD_GLCONTEXT(self)->SetCurrent(*WXD_GLCANVAS(canvas));
}

WXD_EXPORTED void wxd_GLContext_ClearCurrent(void)
{
    wxGLContext::ClearCurrent();
}

WXD_EXPORTED bool wxd_GLContext_IsOK(wxd_GLContext_t* self)
{
    return self && WXD_GLCONTEXT(self)->IsOK();
}

WXD_EXPORTED void* wxd_GLContext_GetProcAddress(const char* name)
{
    if (!name) {
        return nullptr;
    }
    return (void*)wxGLContext::GetProcAddress(wxString::FromUTF8(name));
}

#else // !wxUSE_GLCANVAS

// Built without GL. The entry points still exist so a caller links either way and learns of the
// absence from IsDisplaySupported rather than from a missing symbol.

#define WXD_GL_STUB_OFF(fn) WXD_LOG_ERROR(fn ": this build has wxUSE_GLCANVAS off.")

WXD_EXPORTED wxd_GLAttributes_t* wxd_GLAttributes_Create(void) { return nullptr; }
WXD_EXPORTED void wxd_GLAttributes_Destroy(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_RGBA(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_BufferSize(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_Level(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_DoubleBuffer(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_Stereo(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_AuxBuffers(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_MinRGBA(wxd_GLAttributes_t*, int, int, int, int) {}
WXD_EXPORTED void wxd_GLAttributes_Depth(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_Stencil(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_MinAcumRGBA(wxd_GLAttributes_t*, int, int, int, int) {}
WXD_EXPORTED void wxd_GLAttributes_PlatformDefaults(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_SampleBuffers(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_Samplers(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_FrameBuffersRGB(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_Defaults(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_EndList(wxd_GLAttributes_t*) {}
WXD_EXPORTED void wxd_GLAttributes_AddAttribute(wxd_GLAttributes_t*, int) {}
WXD_EXPORTED void wxd_GLAttributes_AddAttribBits(wxd_GLAttributes_t*, int, int) {}
WXD_EXPORTED void wxd_GLAttributes_SetNeedsARB(wxd_GLAttributes_t*, bool) {}
WXD_EXPORTED bool wxd_GLAttributes_NeedsARB(wxd_GLAttributes_t*) { return false; }
WXD_EXPORTED void wxd_GLAttributes_Reset(wxd_GLAttributes_t*) {}
WXD_EXPORTED int wxd_GLAttributes_GetSize(wxd_GLAttributes_t*) { return 0; }
WXD_EXPORTED const int* wxd_GLAttributes_GetGLAttrs(wxd_GLAttributes_t*) { return nullptr; }

WXD_EXPORTED wxd_GLContextAttrs_t* wxd_GLContextAttrs_Create(void) { return nullptr; }
WXD_EXPORTED void wxd_GLContextAttrs_Destroy(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_CoreProfile(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_MajorVersion(wxd_GLContextAttrs_t*, int) {}
WXD_EXPORTED void wxd_GLContextAttrs_MinorVersion(wxd_GLContextAttrs_t*, int) {}
WXD_EXPORTED void wxd_GLContextAttrs_OGLVersion(wxd_GLContextAttrs_t*, int, int) {}
WXD_EXPORTED void wxd_GLContextAttrs_CompatibilityProfile(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_ForwardCompatible(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_ES2(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_DebugCtx(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_Robust(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_NoResetNotify(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_LoseOnReset(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_ResetIsolation(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_ReleaseFlush(wxd_GLContextAttrs_t*, int) {}
WXD_EXPORTED void wxd_GLContextAttrs_PlatformDefaults(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_EndList(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED void wxd_GLContextAttrs_AddAttribute(wxd_GLContextAttrs_t*, int) {}
WXD_EXPORTED void wxd_GLContextAttrs_AddAttribBits(wxd_GLContextAttrs_t*, int, int) {}
WXD_EXPORTED void wxd_GLContextAttrs_SetNeedsARB(wxd_GLContextAttrs_t*, bool) {}
WXD_EXPORTED bool wxd_GLContextAttrs_NeedsARB(wxd_GLContextAttrs_t*) { return false; }
WXD_EXPORTED void wxd_GLContextAttrs_Reset(wxd_GLContextAttrs_t*) {}
WXD_EXPORTED int wxd_GLContextAttrs_GetSize(wxd_GLContextAttrs_t*) { return 0; }
WXD_EXPORTED const int* wxd_GLContextAttrs_GetGLAttrs(wxd_GLContextAttrs_t*) { return nullptr; }

WXD_EXPORTED wxd_GLCanvas_t*
wxd_GLCanvas_Create(wxd_Window_t*, int, const int*, int, int, int, int, int64_t)
{
    WXD_GL_STUB_OFF("wxd_GLCanvas_Create");
    return nullptr;
}

WXD_EXPORTED wxd_GLCanvas_t*
wxd_GLCanvas_CreateWithAttributes(wxd_Window_t*, wxd_GLAttributes_t*, int, int, int, int, int, int64_t)
{
    WXD_GL_STUB_OFF("wxd_GLCanvas_CreateWithAttributes");
    return nullptr;
}

WXD_EXPORTED bool wxd_GLCanvas_IsDisplaySupported(const int*) { return false; }
WXD_EXPORTED bool wxd_GLCanvas_IsDisplaySupportedAttrs(wxd_GLAttributes_t*) { return false; }
WXD_EXPORTED bool wxd_GLCanvas_SwapBuffers(wxd_GLCanvas_t*) { return false; }
WXD_EXPORTED bool wxd_GLCanvas_SetCurrent(wxd_GLCanvas_t*, wxd_GLContext_t*) { return false; }
WXD_EXPORTED int wxd_GLCanvas_SetSwapInterval(wxd_GLCanvas_t*, int) { return WXD_GL_SWAP_INTERVAL_NOT_SET; }
WXD_EXPORTED int wxd_GLCanvas_GetSwapInterval(wxd_GLCanvas_t*) { return WXD_GL_DEFAULT_SWAP_INTERVAL; }
WXD_EXPORTED bool wxd_GLCanvas_SetColour(wxd_GLCanvas_t*, const char*) { return false; }
WXD_EXPORTED bool wxd_GLCanvas_IsExtensionSupported(const char*) { return false; }
WXD_EXPORTED bool wxd_GLCanvas_IsExtensionInList(const char*, const char*) { return false; }
WXD_EXPORTED bool wxd_GLCanvas_ParseAttribList(const int*, wxd_GLAttributes_t*, wxd_GLContextAttrs_t*)
{
    return false;
}
WXD_EXPORTED wxd_GLContextAttrs_t* wxd_GLCanvas_GetGLCTXAttrs(wxd_GLCanvas_t*) { return nullptr; }

WXD_EXPORTED void wxd_GLCanvas_GetPixelSize(wxd_GLCanvas_t*, int* width, int* height)
{
    if (width) *width = 0;
    if (height) *height = 0;
}

WXD_EXPORTED wxd_GLContext_t*
wxd_GLContext_Create(wxd_GLCanvas_t*, wxd_GLContext_t*, wxd_GLContextAttrs_t*)
{
    WXD_GL_STUB_OFF("wxd_GLContext_Create");
    return nullptr;
}

WXD_EXPORTED void wxd_GLContext_Destroy(wxd_GLContext_t*) {}
WXD_EXPORTED bool wxd_GLContext_SetCurrent(wxd_GLContext_t*, wxd_GLCanvas_t*) { return false; }
WXD_EXPORTED void wxd_GLContext_ClearCurrent(void) {}
WXD_EXPORTED bool wxd_GLContext_IsOK(wxd_GLContext_t*) { return false; }
WXD_EXPORTED void* wxd_GLContext_GetProcAddress(const char*) { return nullptr; }

#endif // wxUSE_GLCANVAS
