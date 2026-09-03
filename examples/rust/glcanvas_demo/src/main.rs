//! Opens a GL canvas, makes a context current, clears to a colour and swaps.
//!
//! Reports what it got — the attribute lists, the context, the driver strings and the pixel size —
//! then closes. Pass `--hold` to leave the window open instead, so the clear can be seen.
//!
//! This is the check to run first on a platform whose OpenGL support is in question: every line it
//! prints is a step that can fail on its own.

use std::os::raw::c_char;
use wxdragon::prelude::*;
use wxdragon::widgets::glcanvas::{GLAttributes, GLContextAttrs, attr};

type GlGetString = unsafe extern "C" fn(u32) -> *const c_char;
type GlClearColor = unsafe extern "C" fn(f32, f32, f32, f32);
type GlClear = unsafe extern "C" fn(u32);

const GL_VENDOR: u32 = 0x1F00;
const GL_RENDERER: u32 = 0x1F01;
const GL_VERSION: u32 = 0x1F02;
const GL_COLOR_BUFFER_BIT: u32 = 0x0000_4000;

fn gl_string(f: GlGetString, name: u32) -> String {
    let p = unsafe { f(name) };
    if p.is_null() {
        return "<null>".into();
    }
    unsafe { std::ffi::CStr::from_ptr(p) }.to_string_lossy().into_owned()
}

fn main() {
    let hold = std::env::args().any(|a| a == "--hold");

    let _ = wxdragon::main(move |_| {
        println!(
            "is_display_supported(DEFAULT): {}",
            GLCanvas::is_display_supported(&attr::DEFAULT)
        );

        let mut attrs = GLAttributes::new();
        attrs.platform_defaults().rgba().double_buffer().depth(16).end_list();
        println!("GLAttributes:    {:?}", attrs.as_slice());
        println!("supported:       {}", GLCanvas::is_display_supported_attrs(&attrs));

        let mut ctx_attrs = GLContextAttrs::new();
        ctx_attrs.platform_defaults().core_profile().ogl_version(3, 3).end_list();
        println!("GLContextAttrs:  {:?}", ctx_attrs.as_slice());

        let frame = Frame::builder()
            .with_title("wxGLCanvas")
            .with_size(Size::new(480, 320))
            .build();
        let canvas = GLCanvas::builder(&frame).build();
        frame.show(true);

        // A core-profile request is refused on some drivers; the default context is the fallback,
        // and which one was taken is the interesting part of this output.
        let Some(ctx) = GLContext::with_attrs(&canvas, &ctx_attrs).or_else(|| {
            println!("core 3.3 refused — falling back to a default context");
            GLContext::new(&canvas)
        }) else {
            println!("FAIL: no GL context could be created");
            frame.close(true);
            return;
        };
        println!("context is_ok:   {}", ctx.is_ok());

        if !canvas.set_current(&ctx) {
            println!("FAIL: set_current refused");
            frame.close(true);
            return;
        }

        let p = GLContext::proc_address("glGetString");
        if p.is_null() {
            println!("FAIL: glGetString did not resolve");
            frame.close(true);
            return;
        }
        let get_string: GlGetString = unsafe { std::mem::transmute(p) };
        println!("GL_VENDOR:       {}", gl_string(get_string, GL_VENDOR));
        println!("GL_RENDERER:     {}", gl_string(get_string, GL_RENDERER));
        println!("GL_VERSION:      {}", gl_string(get_string, GL_VERSION));

        // The two sizes differ on a scaled display, which is the mistake pixel_size exists to stop.
        println!("pixel size:      {:?}", canvas.pixel_size());
        println!("logical size:    {:?}", canvas.get_size());
        println!("swap interval:   {:?}", canvas.set_swap_interval(1));
        println!(
            "texture_storage: {}",
            GLCanvas::is_extension_supported("GL_ARB_texture_storage")
        );

        let clear_colour: GlClearColor = unsafe { std::mem::transmute(GLContext::proc_address("glClearColor")) };
        let clear: GlClear = unsafe { std::mem::transmute(GLContext::proc_address("glClear")) };

        // Drawing goes in the paint handler, not here. The EGL surface is created when the widget
        // is realized, which has not happened yet at this point in the same event-loop turn, so a
        // swap now has nothing to present to and reports false.
        // ⚠ The first swap can legitimately fail. Under Wayland wxGLCanvasEGL waits for the
        // compositor's frame callback before it will present, and returns false until then
        // (`glegl.cpp`, `m_readyToDraw`). A caller must treat that as "not yet", not as an error.
        let painted = std::cell::Cell::new(0u32);
        let done = std::cell::Cell::new(false);
        canvas.on_paint(move |_| {
            if done.get() {
                return;
            }
            if !canvas.set_current(&ctx) {
                println!("FAIL: set_current refused in paint");
                return;
            }
            unsafe {
                clear_colour(0.1, 0.2, 0.4, 1.0);
                clear(GL_COLOR_BUFFER_BIT);
            }
            let n = painted.get() + 1;
            painted.set(n);
            let swapped = canvas.swap_buffers();
            println!("paint {n}: swap_buffers {swapped}");

            if swapped {
                println!("OK — presented on paint {n}");
                done.set(true);
                if !hold {
                    frame.close(true);
                }
            } else if n >= 30 {
                println!("FAIL: nothing presented after {n} paints");
                done.set(true);
                frame.close(true);
            } else {
                canvas.refresh(false, None);
            }
        });
    });
}
