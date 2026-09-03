//! `wxGLCanvas` and friends: a window OpenGL draws into.
//!
//! Four types, mirroring wxWidgets:
//!
//! * [`GLAttributes`] — the pixel format a canvas is created for.
//! * [`GLContextAttrs`] — the version and profile a context is created for.
//! * [`GLCanvas`] — the window.
//! * [`GLContext`] — the GL state, made current for a canvas before drawing.
//!
//! Load GL entry points through [`GLContext::proc_address`] rather than per-platform symbol
//! lookup; wxWidgets resolves it the same way everywhere.

use crate::event::WxEvtHandler;
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::window::{WindowHandle, WxWidget};
use std::os::raw::{c_char, c_int, c_void};
use wxdragon_sys as ffi;

widget_style_enum!(
    name: GLCanvasStyle,
    doc: "Style flags for GLCanvas.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

/// Attribute tokens, as `WX_GL_*` in wxWidgets. A hand-built list is terminated by `0`.
///
/// Prefer [`GLAttributes`], which builds a list without the terminator being the caller's problem.
pub mod attr {
    use std::os::raw::c_int;
    use wxdragon_sys as ffi;

    macro_rules! gl_attr {
        ($($name:ident = $ffi:ident, $doc:literal;)*) => {
            $(
                #[doc = $doc]
                pub const $name: c_int = ffi::$ffi as c_int;
            )*
        };
    }

    // The values come from the C header, which static_asserts each against the wxWidgets enum —
    // so a token that moves upstream fails the build rather than producing a list wxWidgets
    // rejects at run time.
    gl_attr! {
        RGBA = WXD_GL_RGBA, "Use the RGBA colour model rather than an indexed palette.";
        BUFFER_SIZE = WXD_GL_BUFFER_SIZE, "Colour buffer size, followed by the number of bits.";
        LEVEL = WXD_GL_LEVEL, "Plane, followed by `0` for the main buffer, `>0` overlay, `<0` underlay.";
        DOUBLEBUFFER = WXD_GL_DOUBLEBUFFER, "Request a double-buffered format.";
        STEREO = WXD_GL_STEREO, "Request a stereoscopic format.";
        AUX_BUFFERS = WXD_GL_AUX_BUFFERS, "Auxiliary buffers, followed by the count.";
        MIN_RED = WXD_GL_MIN_RED, "Minimum red bits, followed by the count.";
        MIN_GREEN = WXD_GL_MIN_GREEN, "Minimum green bits, followed by the count.";
        MIN_BLUE = WXD_GL_MIN_BLUE, "Minimum blue bits, followed by the count.";
        MIN_ALPHA = WXD_GL_MIN_ALPHA, "Minimum alpha bits, followed by the count.";
        DEPTH_SIZE = WXD_GL_DEPTH_SIZE, "Depth buffer, followed by the number of bits.";
        STENCIL_SIZE = WXD_GL_STENCIL_SIZE, "Stencil buffer, followed by the number of bits.";
        MIN_ACCUM_RED = WXD_GL_MIN_ACCUM_RED, "Minimum accumulation-buffer red bits.";
        MIN_ACCUM_GREEN = WXD_GL_MIN_ACCUM_GREEN, "Minimum accumulation-buffer green bits.";
        MIN_ACCUM_BLUE = WXD_GL_MIN_ACCUM_BLUE, "Minimum accumulation-buffer blue bits.";
        MIN_ACCUM_ALPHA = WXD_GL_MIN_ACCUM_ALPHA, "Minimum accumulation-buffer alpha bits.";
        SAMPLE_BUFFERS = WXD_GL_SAMPLE_BUFFERS, "Multisampling on, followed by `1`.";
        SAMPLES = WXD_GL_SAMPLES, "Samples per pixel, followed by the count.";
        FRAMEBUFFER_SRGB = WXD_GL_FRAMEBUFFER_SRGB, "Request an sRGB-capable framebuffer.";
        CORE_PROFILE = WXD_GL_CORE_PROFILE, "Ask for a core-profile context.";
        MAJOR_VERSION = WXD_GL_MAJOR_VERSION, "Context major version, followed by the number.";
        MINOR_VERSION = WXD_GL_MINOR_VERSION, "Context minor version, followed by the number.";
        COMPAT_PROFILE = WXD_GL_COMPAT_PROFILE, "Ask for a compatibility-profile context.";
        FORWARD_COMPAT = WXD_GL_FORWARD_COMPAT, "Forward-compatible context, OpenGL 3.0 and later.";
        ES2 = WXD_GL_ES2, "Ask for an OpenGL ES or ES2 context.";
        DEBUG = WXD_GL_DEBUG, "Ask for a debug context.";
        ROBUST_ACCESS = WXD_GL_ROBUST_ACCESS, "Ask for robust buffer access.";
        NO_RESET_NOTIFY = WXD_GL_NO_RESET_NOTIFY, "Never deliver notification of reset events.";
        LOSE_ON_RESET = WXD_GL_LOSE_ON_RESET, "Lose all context state on a graphics reset.";
        RESET_ISOLATION = WXD_GL_RESET_ISOLATION, "Isolate this context from another's reset.";
        RELEASE_FLUSH = WXD_GL_RELEASE_FLUSH, "Flush pending commands when the context is released.";
        RELEASE_NONE = WXD_GL_RELEASE_NONE, "Do not flush pending commands on release.";
    }

    /// A double-buffered RGBA format with a 16-bit depth buffer — wxWidgets' own `Defaults()`.
    pub const DEFAULT: [c_int; 5] = [RGBA, DOUBLEBUFFER, DEPTH_SIZE, 16, 0];
}

/// What [`GLCanvas::set_swap_interval`] managed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwapInterval {
    /// The platform does not support setting it.
    NotSet,
    /// Set to the requested value.
    Set,
    /// Set, but adaptive VSync was refused and a non-adaptive interval used instead.
    NonAdaptive,
}

impl SwapInterval {
    fn from_raw(v: c_int) -> Self {
        match v {
            1 => SwapInterval::Set,
            2 => SwapInterval::NonAdaptive,
            _ => SwapInterval::NotSet,
        }
    }
}

/// Passed to [`GLCanvas::set_swap_interval`] to leave the platform's own VSync handling alone.
pub const DEFAULT_SWAP_INTERVAL: c_int = i32::MAX;

/// The methods `wxGLAttribsBase` gives both attribute lists.
///
/// One arm per type rather than one generic body: the FFI names differ by type, and spelling them
/// out is what keeps the expansion readable.
macro_rules! attrib_list_common {
    ($ty:ident, $raw:ty, $add:path, $bits:path, $needs:path, $set_needs:path,
     $reset:path, $size:path, $end:path, $destroy:path) => {
        impl $ty {
            /// Append a raw attribute value.
            pub fn add_attribute(&mut self, attribute: c_int) -> &mut Self {
                unsafe { $add(self.ptr, attribute) };
                self
            }

            /// Find `search_val` and OR `combine_val` into the value after it.
            pub fn add_attrib_bits(&mut self, search_val: c_int, combine_val: c_int) -> &mut Self {
                unsafe { $bits(self.ptr, search_val, combine_val) };
                self
            }

            /// Whether creation needs the ARB entry points.
            pub fn needs_arb(&self) -> bool {
                unsafe { $needs(self.ptr) }
            }

            /// Declare that creation needs the ARB entry points.
            pub fn set_needs_arb(&mut self, needs_arb: bool) -> &mut Self {
                unsafe { $set_needs(self.ptr, needs_arb) };
                self
            }

            /// Empty the list.
            pub fn reset(&mut self) -> &mut Self {
                unsafe { $reset(self.ptr) };
                self
            }

            /// How many values the list holds.
            pub fn len(&self) -> usize {
                unsafe { $size(self.ptr).max(0) as usize }
            }

            /// Whether the list is empty.
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Close the list. Nothing may be appended afterwards.
            pub fn end_list(&mut self) -> &mut Self {
                unsafe { $end(self.ptr) };
                self
            }

            pub(crate) fn as_ptr(&self) -> *mut $raw {
                self.ptr
            }
        }

        impl Drop for $ty {
            fn drop(&mut self) {
                unsafe { $destroy(self.ptr) };
            }
        }
    };
}

/// The pixel format a [`GLCanvas`] is created for.
///
/// Setters chain. Call [`GLAttributes::end_list`] when done, then hand it to
/// [`GLCanvasBuilder::build_with_attributes`] or [`GLCanvas::is_display_supported_attrs`].
///
/// # Example
/// ```ignore
/// let mut attrs = GLAttributes::new();
/// attrs.platform_defaults().rgba().double_buffer().depth(16).end_list();
/// ```
pub struct GLAttributes {
    ptr: *mut ffi::wxd_GLAttributes_t,
}

impl GLAttributes {
    /// An empty list.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_GLAttributes_Create() };
        assert!(!ptr.is_null(), "wxd_GLAttributes_Create returned null");
        GLAttributes { ptr }
    }

    /// wxWidgets' own defaults — RGBA, a 16-bit depth buffer and double buffering.
    pub fn defaults(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_Defaults(self.ptr) };
        self
    }

    /// The platform's preferred baseline. Append before anything else.
    pub fn platform_defaults(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_PlatformDefaults(self.ptr) };
        self
    }

    /// Use the RGBA colour model.
    pub fn rgba(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_RGBA(self.ptr) };
        self
    }

    /// Colour buffer size, in bits.
    pub fn buffer_size(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_BufferSize(self.ptr, val) };
        self
    }

    /// Overlay/underlay plane: `0` is the main plane.
    pub fn level(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_Level(self.ptr, val) };
        self
    }

    /// Ask for a back buffer.
    pub fn double_buffer(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_DoubleBuffer(self.ptr) };
        self
    }

    /// Ask for a stereoscopic format.
    pub fn stereo(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_Stereo(self.ptr) };
        self
    }

    /// Number of auxiliary buffers.
    pub fn aux_buffers(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_AuxBuffers(self.ptr, val) };
        self
    }

    /// Minimum bits per colour channel.
    pub fn min_rgba(&mut self, r: c_int, g: c_int, b: c_int, a: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_MinRGBA(self.ptr, r, g, b, a) };
        self
    }

    /// Depth buffer, in bits.
    pub fn depth(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_Depth(self.ptr, val) };
        self
    }

    /// Stencil buffer, in bits.
    pub fn stencil(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_Stencil(self.ptr, val) };
        self
    }

    /// Minimum accumulation-buffer bits per channel.
    pub fn min_acum_rgba(&mut self, r: c_int, g: c_int, b: c_int, a: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_MinAcumRGBA(self.ptr, r, g, b, a) };
        self
    }

    /// Turn on multisampling. Pair with [`GLAttributes::samplers`].
    pub fn sample_buffers(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_SampleBuffers(self.ptr, val) };
        self
    }

    /// Samples per pixel for multisampling.
    pub fn samplers(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_Samplers(self.ptr, val) };
        self
    }

    /// Ask for an sRGB-capable framebuffer.
    pub fn frame_buffers_rgb(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLAttributes_FrameBuffersRGB(self.ptr) };
        self
    }

    /// The list as wxWidgets holds it. Empty when nothing has been appended.
    ///
    /// ⚠ These are the **platform's** values after translation — EGL or GLX or WGL tokens — not
    /// the `WX_GL_*` ones that were appended. Useful for diagnostics, not for round-tripping.
    pub fn as_slice(&self) -> &[c_int] {
        let ptr = unsafe { ffi::wxd_GLAttributes_GetGLAttrs(self.ptr) };
        if ptr.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(ptr, self.len()) }
    }
}

impl Default for GLAttributes {
    fn default() -> Self {
        Self::new()
    }
}

attrib_list_common!(
    GLAttributes,
    ffi::wxd_GLAttributes_t,
    ffi::wxd_GLAttributes_AddAttribute,
    ffi::wxd_GLAttributes_AddAttribBits,
    ffi::wxd_GLAttributes_NeedsARB,
    ffi::wxd_GLAttributes_SetNeedsARB,
    ffi::wxd_GLAttributes_Reset,
    ffi::wxd_GLAttributes_GetSize,
    ffi::wxd_GLAttributes_EndList,
    ffi::wxd_GLAttributes_Destroy
);

/// The version, profile and flags a [`GLContext`] is created with.
///
/// # Example
/// ```ignore
/// let mut ctx_attrs = GLContextAttrs::new();
/// ctx_attrs.platform_defaults().core_profile().ogl_version(3, 3).end_list();
/// ```
pub struct GLContextAttrs {
    ptr: *mut ffi::wxd_GLContextAttrs_t,
}

impl GLContextAttrs {
    /// An empty list.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_GLContextAttrs_Create() };
        assert!(!ptr.is_null(), "wxd_GLContextAttrs_Create returned null");
        GLContextAttrs { ptr }
    }

    /// The platform's preferred baseline. Append before anything else.
    pub fn platform_defaults(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_PlatformDefaults(self.ptr) };
        self
    }

    /// Ask for a core-profile context.
    pub fn core_profile(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_CoreProfile(self.ptr) };
        self
    }

    /// Ask for a compatibility-profile context.
    pub fn compatibility_profile(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_CompatibilityProfile(self.ptr) };
        self
    }

    /// Major version of the context to create.
    pub fn major_version(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_MajorVersion(self.ptr, val) };
        self
    }

    /// Minor version of the context to create.
    pub fn minor_version(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_MinorVersion(self.ptr, val) };
        self
    }

    /// Both version numbers at once.
    pub fn ogl_version(&mut self, major: c_int, minor: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_OGLVersion(self.ptr, major, minor) };
        self
    }

    /// Forbid the deprecated functionality of the requested version.
    pub fn forward_compatible(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_ForwardCompatible(self.ptr) };
        self
    }

    /// Ask for an OpenGL ES 2 context.
    pub fn es2(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_ES2(self.ptr) };
        self
    }

    /// Ask for a debug context.
    pub fn debug_ctx(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_DebugCtx(self.ptr) };
        self
    }

    /// Ask for robust buffer access.
    pub fn robust(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_Robust(self.ptr) };
        self
    }

    /// No notification on graphics reset. Pair with [`GLContextAttrs::robust`].
    pub fn no_reset_notify(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_NoResetNotify(self.ptr) };
        self
    }

    /// Lose the context on graphics reset. Pair with [`GLContextAttrs::robust`].
    pub fn lose_on_reset(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_LoseOnReset(self.ptr) };
        self
    }

    /// Isolate this context's resources from a graphics reset elsewhere.
    pub fn reset_isolation(&mut self) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_ResetIsolation(self.ptr) };
        self
    }

    /// Flush behaviour when the context is released. `0` is no flush.
    pub fn release_flush(&mut self, val: c_int) -> &mut Self {
        unsafe { ffi::wxd_GLContextAttrs_ReleaseFlush(self.ptr, val) };
        self
    }

    /// The list as wxWidgets holds it. Empty when nothing has been appended.
    ///
    /// ⚠ These are the **platform's** values after translation — EGL or GLX or WGL tokens — not
    /// the `WX_GL_*` ones that were appended. Useful for diagnostics, not for round-tripping.
    pub fn as_slice(&self) -> &[c_int] {
        let ptr = unsafe { ffi::wxd_GLContextAttrs_GetGLAttrs(self.ptr) };
        if ptr.is_null() {
            return &[];
        }
        unsafe { std::slice::from_raw_parts(ptr, self.len()) }
    }
}

impl Default for GLContextAttrs {
    fn default() -> Self {
        Self::new()
    }
}

attrib_list_common!(
    GLContextAttrs,
    ffi::wxd_GLContextAttrs_t,
    ffi::wxd_GLContextAttrs_AddAttribute,
    ffi::wxd_GLContextAttrs_AddAttribBits,
    ffi::wxd_GLContextAttrs_NeedsARB,
    ffi::wxd_GLContextAttrs_SetNeedsARB,
    ffi::wxd_GLContextAttrs_Reset,
    ffi::wxd_GLContextAttrs_GetSize,
    ffi::wxd_GLContextAttrs_EndList,
    ffi::wxd_GLContextAttrs_Destroy
);

pub type RawGLCanvas = ffi::wxd_GLCanvas_t;

/// A `wxGLCanvas`: a window OpenGL draws into.
///
/// Drawing needs a [`GLContext`] made current for the canvas, and ends with
/// [`GLCanvas::swap_buffers`].
///
/// # Example
/// ```ignore
/// if !GLCanvas::is_display_supported(&attr::DEFAULT) {
///     // fall back to another way of presenting
/// }
/// let canvas = GLCanvas::builder(&frame).build();
/// let ctx = GLContext::new(&canvas).expect("a GL context");
/// canvas.set_current(&ctx);
/// // … issue GL calls, sized by canvas.pixel_size() …
/// canvas.swap_buffers();
/// ```
#[derive(Clone, Copy)]
pub struct GLCanvas {
    handle: WindowHandle,
}

impl GLCanvas {
    pub fn builder(parent: &dyn WxWidget) -> GLCanvasBuilder<'_> {
        GLCanvasBuilder::new(parent)
    }

    /// Whether the display can provide `attrib_list`, a `0`-terminated list of [`attr`] tokens.
    ///
    /// Ask before building. A canvas for an unsupported format is refused at construction, and
    /// knowing in advance is what lets a caller present some other way instead.
    pub fn is_display_supported(attrib_list: &[c_int]) -> bool {
        let ptr = attrib_list.last().map_or(std::ptr::null(), |&last| {
            assert_eq!(last, 0, "an attribute list must end with 0");
            attrib_list.as_ptr()
        });
        unsafe { ffi::wxd_GLCanvas_IsDisplaySupported(ptr) }
    }

    /// As [`GLCanvas::is_display_supported`], for a built [`GLAttributes`].
    pub fn is_display_supported_attrs(attrs: &GLAttributes) -> bool {
        unsafe { ffi::wxd_GLCanvas_IsDisplaySupportedAttrs(attrs.as_ptr()) }
    }

    /// Whether the windowing system supports a named extension.
    ///
    /// ⚠ This asks the **platform** — EGL, GLX or WGL — not OpenGL itself. A `GL_*` extension is
    /// not in that list; query `glGetString(GL_EXTENSIONS)` for those, optionally through
    /// [`GLCanvas::is_extension_in_list`].
    ///
    /// # Panics
    /// If `extension` contains a NUL byte.
    pub fn is_extension_supported(extension: &str) -> bool {
        let c = std::ffi::CString::new(extension).expect("an extension name has no NUL");
        unsafe { ffi::wxd_GLCanvas_IsExtensionSupported(c.as_ptr()) }
    }

    /// Whether `extension` appears in a space-separated `list`, as `glGetString(GL_EXTENSIONS)`
    /// returns.
    ///
    /// # Panics
    /// If either argument contains a NUL byte.
    pub fn is_extension_in_list(list: &str, extension: &str) -> bool {
        let l = std::ffi::CString::new(list).expect("an extension list has no NUL");
        let e = std::ffi::CString::new(extension).expect("an extension name has no NUL");
        unsafe { ffi::wxd_GLCanvas_IsExtensionInList(l.as_ptr(), e.as_ptr()) }
    }

    /// Split a `0`-terminated attribute list into pixel-format and context attributes.
    ///
    /// False when the list holds an attribute wxWidgets does not recognise, in which case neither
    /// output should be used.
    pub fn parse_attrib_list(
        attrib_list: &[c_int],
        disp_attrs: &mut GLAttributes,
        ctx_attrs: Option<&mut GLContextAttrs>,
    ) -> bool {
        assert_eq!(attrib_list.last().copied(), Some(0), "an attribute list must end with 0",);
        let ctx = ctx_attrs.map_or(std::ptr::null_mut(), |c| c.as_ptr());
        unsafe { ffi::wxd_GLCanvas_ParseAttribList(attrib_list.as_ptr(), disp_attrs.as_ptr(), ctx) }
    }

    #[inline]
    fn gl_canvas_ptr(&self) -> *mut RawGLCanvas {
        self.handle
            .get_ptr()
            .map(|p| p as *mut RawGLCanvas)
            .unwrap_or(std::ptr::null_mut())
    }

    /// Present the back buffer.
    ///
    /// ⚠ **False is not necessarily an error.** Under Wayland the EGL backend waits for the
    /// compositor's frame callback and refuses to present until it arrives, so the first calls
    /// after a canvas is shown return false and a later one succeeds — three paints, in the
    /// bundled `glcanvas_demo`. Ask again on the next paint rather than treating it as a failure.
    ///
    /// Also false if the canvas has been destroyed.
    pub fn swap_buffers(&self) -> bool {
        let ptr = self.gl_canvas_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GLCanvas_SwapBuffers(ptr) }
    }

    /// Make `context` current for this canvas. False if either has been destroyed.
    pub fn set_current(&self, context: &GLContext) -> bool {
        let ptr = self.gl_canvas_ptr();
        if ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GLCanvas_SetCurrent(ptr, context.ptr) }
    }

    /// Set the buffer swap interval: `0` disables VSync, `1` syncs to every refresh, a negative
    /// value asks for adaptive VSync, and [`DEFAULT_SWAP_INTERVAL`] leaves the platform's own
    /// handling alone.
    pub fn set_swap_interval(&self, interval: c_int) -> SwapInterval {
        let ptr = self.gl_canvas_ptr();
        if ptr.is_null() {
            return SwapInterval::NotSet;
        }
        SwapInterval::from_raw(unsafe { ffi::wxd_GLCanvas_SetSwapInterval(ptr, interval) })
    }

    /// The current swap interval, or [`DEFAULT_SWAP_INTERVAL`] where it is unknown.
    pub fn swap_interval(&self) -> c_int {
        let ptr = self.gl_canvas_ptr();
        if ptr.is_null() {
            return DEFAULT_SWAP_INTERVAL;
        }
        unsafe { ffi::wxd_GLCanvas_GetSwapInterval(ptr) }
    }

    /// `glColor` for a named colour. False when the name is unknown.
    ///
    /// # Panics
    /// If `colour` contains a NUL byte.
    pub fn set_colour(&self, colour: &str) -> bool {
        let ptr = self.gl_canvas_ptr();
        if ptr.is_null() {
            return false;
        }
        let c = std::ffi::CString::new(colour).expect("a colour name has no NUL");
        unsafe { ffi::wxd_GLCanvas_SetColour(ptr, c.as_ptr()) }
    }

    /// The canvas size in **physical pixels**, which a GL viewport wants.
    ///
    /// ⚠ Not the same as [`WxWidget::get_size`], which is in logical units. The two differ on a
    /// scaled display, and a viewport sized in logical units renders to part of the surface.
    pub fn pixel_size(&self) -> Size {
        let ptr = self.gl_canvas_ptr();
        if ptr.is_null() {
            return Size::new(0, 0);
        }
        let (mut w, mut h) = (0, 0);
        unsafe { ffi::wxd_GLCanvas_GetPixelSize(ptr, &mut w, &mut h) };
        Size::new(w, h)
    }

    /// # Safety
    /// The pointer must be a valid `wxd_GLCanvas_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut RawGLCanvas) -> Self {
        assert!(!ptr.is_null());
        GLCanvas {
            handle: WindowHandle::new(ptr as *mut ffi::wxd_Window_t),
        }
    }

    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }
}

impl WxWidget for GLCanvas {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

impl WxEvtHandler for GLCanvas {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()) as *mut ffi::wxd_EvtHandler_t
    }
}

impl crate::event::WindowEvents for GLCanvas {}

/// A `wxGLContext`: the GL state a [`GLCanvas`] is drawn with.
///
/// Owned, and destroyed on drop. One context can serve several canvases, and contexts can share
/// objects — see [`GLContext::shared_with`].
pub struct GLContext {
    ptr: *mut ffi::wxd_GLContext_t,
}

impl GLContext {
    /// A context for `canvas`, or `None` when one cannot be created.
    ///
    /// `None` is an ordinary outcome rather than an error to unwrap: a machine with no usable GL,
    /// a remote session, or a driver that refuses the format all land here, and a caller with
    /// another way of presenting should take it.
    pub fn new(canvas: &GLCanvas) -> Option<Self> {
        Self::build(canvas, std::ptr::null_mut(), None)
    }

    /// A context built to `attrs`.
    pub fn with_attrs(canvas: &GLCanvas, attrs: &GLContextAttrs) -> Option<Self> {
        Self::build(canvas, std::ptr::null_mut(), Some(attrs))
    }

    /// A context sharing display lists, textures and buffers with `other`.
    pub fn shared_with(canvas: &GLCanvas, other: &GLContext) -> Option<Self> {
        Self::build(canvas, other.ptr, None)
    }

    /// A context sharing objects with `other` and built to `attrs`.
    pub fn shared_with_attrs(canvas: &GLCanvas, other: &GLContext, attrs: &GLContextAttrs) -> Option<Self> {
        Self::build(canvas, other.ptr, Some(attrs))
    }

    fn build(canvas: &GLCanvas, other: *mut ffi::wxd_GLContext_t, attrs: Option<&GLContextAttrs>) -> Option<Self> {
        let canvas_ptr = canvas.gl_canvas_ptr();
        if canvas_ptr.is_null() {
            return None;
        }
        let attrs_ptr = attrs.map_or(std::ptr::null_mut(), |a| a.ptr);
        let ptr = unsafe { ffi::wxd_GLContext_Create(canvas_ptr, other, attrs_ptr) };
        (!ptr.is_null()).then_some(GLContext { ptr })
    }

    /// Make this context current for `canvas`.
    pub fn set_current(&self, canvas: &GLCanvas) -> bool {
        let canvas_ptr = canvas.gl_canvas_ptr();
        if canvas_ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_GLContext_SetCurrent(self.ptr, canvas_ptr) }
    }

    /// Unset whichever context is current.
    pub fn clear_current() {
        unsafe { ffi::wxd_GLContext_ClearCurrent() };
    }

    /// Whether the context is usable.
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_GLContext_IsOK(self.ptr) }
    }

    /// The address of a GL entry point, or null when unavailable.
    ///
    /// Pass this to a GL loader. It resolves through wxWidgets, so it behaves the same on every
    /// platform and needs no `wglGetProcAddress`/`glXGetProcAddress`/`dlsym` of its own.
    ///
    /// # Panics
    /// If `name` contains a NUL byte.
    pub fn proc_address(name: &str) -> *mut c_void {
        let c = std::ffi::CString::new(name).expect("a GL entry point name has no NUL");
        unsafe { ffi::wxd_GLContext_GetProcAddress(c.as_ptr() as *const c_char) }
    }
}

impl Drop for GLContext {
    fn drop(&mut self) {
        unsafe { ffi::wxd_GLContext_Destroy(self.ptr) };
    }
}

widget_builder!(
    name: GLCanvas,
    parent_type: &'a dyn WxWidget,
    style_type: GLCanvasStyle,
    fields: {
        attrib_list: Vec<c_int> = attr::DEFAULT.to_vec()
    },
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        assert_eq!(
            slf.attrib_list.last().copied(),
            Some(0),
            "an attribute list must end with 0",
        );
        unsafe {
            let ctrl_ptr = ffi::wxd_GLCanvas_Create(
                parent_ptr,
                slf.id,
                slf.attrib_list.as_ptr(),
                slf.pos.x,
                slf.pos.y,
                slf.size.width,
                slf.size.height,
                slf.style.bits(),
            );
            assert!(
                !ctrl_ptr.is_null(),
                "wxd_GLCanvas_Create returned null — ask GLCanvas::is_display_supported first",
            );
            GLCanvas::from_ptr(ctrl_ptr)
        }
    }
);

impl GLCanvasBuilder<'_> {
    /// Build for a prepared [`GLAttributes`] instead of a raw attribute list.
    ///
    /// Consumes the builder, because wxWidgets takes the attributes at construction.
    pub fn build_with_attributes(self, attrs: &GLAttributes) -> GLCanvas {
        let parent_ptr = self.parent.handle_ptr();
        unsafe {
            let ctrl_ptr = ffi::wxd_GLCanvas_CreateWithAttributes(
                parent_ptr,
                attrs.as_ptr(),
                self.id,
                self.pos.x,
                self.pos.y,
                self.size.width,
                self.size.height,
                self.style.bits(),
            );
            assert!(
                !ctrl_ptr.is_null(),
                "wxd_GLCanvas_CreateWithAttributes returned null — ask \
                 GLCanvas::is_display_supported_attrs first",
            );
            GLCanvas::from_ptr(ctrl_ptr)
        }
    }
}

impl crate::window::FromWindowWithClassName for GLCanvas {
    fn class_name() -> &'static str {
        "wxGLCanvas"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        GLCanvas {
            handle: WindowHandle::new(ptr),
        }
    }
}
