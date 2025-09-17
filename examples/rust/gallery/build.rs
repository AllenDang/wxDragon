use embed_manifest::manifest::{ActiveCodePage, Setting, SupportedOS::*};
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    // Tell Cargo to rerun this build script if the build script changes
    println!("cargo:rerun-if-changed=build.rs");

    // Check if we're building for Windows (either natively or cross-compiling)
    let target = std::env::var("TARGET").unwrap_or_default();

    if target.contains("windows") {
        embed_windows_manifest("wxDragon.Gallery");

        let wx_version = "3.3.1";
        // Compile and embed wx.rc resources for wxWidgets
        embed_wx_resources(wx_version, &target);
    }
}

fn embed_windows_manifest(name: &str) {
    {
        // Create a comprehensive manifest for Windows theming and modern features
        let manifest = new_manifest(name)
            // Enable modern Windows Common Controls (v6) for theming
            // Windows10 is the latest supported in the enum
            .supported_os(Windows7..=Windows10)
            // Set UTF-8 as active code page for better Unicode support
            .active_code_page(ActiveCodePage::Utf8)
            // Enable heap type optimization for better performance (if available)
            .heap_type(embed_manifest::manifest::HeapType::SegmentHeap)
            // Enable high-DPI awareness for crisp displays
            .dpi_awareness(embed_manifest::manifest::DpiAwareness::PerMonitorV2)
            // Enable long path support (if configured in Windows)
            .long_path_aware(Setting::Enabled);

        // Embed the manifest - this works even when cross-compiling!
        if let Err(e) = embed_manifest(manifest) {
            // This should not happen with embed-manifest as it supports cross-compilation
            println!("cargo:warning=Failed to embed manifest: {e}");
            println!("cargo:warning=The application will still work but may lack optimal Windows theming");
        }
    }
}

fn embed_wx_resources(wx_version: &str, target: &str) {
    // Find the wxWidgets directory with wx.rc
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let target_dir = std::path::Path::new(&out_dir)
        .ancestors()
        .find(|p| p.file_name().map(|n| n == "target").unwrap_or(false))
        .expect("Could not find target directory");

    // Look for wxWidgets directory - try both debug and release profiles
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let wxwidgets_pattern = format!("wxwidgets-{wx_version}-{target}-{profile}");
    let wxwidgets_dir = target_dir.join(&wxwidgets_pattern);
    let wx_rc_path = wxwidgets_dir
        .join("include")
        .join("wx")
        .join("msw")
        .join("wx.rc");

    // Retry logic: Check if wx.rc exists, retry up to 10 times with a 5-second delay
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 10;
    const RETRY_DELAY_SECS: u64 = 5;

    while !wx_rc_path.exists() && retry_count < MAX_RETRIES {
        if retry_count == 0 {
            println!("cargo:warning=wx.rc not found at {wx_rc_path:?}, waiting and retrying...");
        }

        println!(
            "cargo:warning=Retry {}/{MAX_RETRIES}: Waiting {RETRY_DELAY_SECS} seconds before checking again...",
            retry_count + 1
        );

        std::thread::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECS));
        retry_count += 1;
    }

    if !wx_rc_path.exists() {
        println!("cargo:warning=wx.rc not found at {wx_rc_path:?} after {MAX_RETRIES} retries, skipping resource embedding");
        return;
    }

    //*
    let wx_include_path = wxwidgets_dir.join("include");

    let res = embed_resource::compile(&wx_rc_path, [&wx_include_path]);
    if res != embed_resource::CompilationResult::Ok {
        println!("cargo:warning=Compile resources with embed_resource: {res:?}");
    }
    // */

    /*

    // Choose the appropriate resource compiler
    let windres = match which::which("windres") {
        // For native Windows or MSVC
        Ok(path) => path,
        Err(_) => match which::which("x86_64-w64-mingw32-windres") {
            // For MinGW cross-compilation
            Ok(path) => path,
            Err(_) => {
                println!("cargo:warning=windres not found in PATH");
                return;
            }
        },
    };

    // Compile the .rc file to .res
    let res_path = std::path::Path::new(&out_dir).join("wx.res");
    let mut cmd = std::process::Command::new(windres);
    cmd.arg("-i")
        .arg(&wx_rc_path)
        .arg("-o")
        .arg(&res_path)
        .arg("-O")
        .arg("coff") // Output format
        .arg("--include-dir")
        .arg(wxwidgets_dir.join("include"));

    if target.contains("i686") || target.contains("i586") {
        cmd.arg("--target").arg("pe-i386");
    } else if target.contains("x86_64") {
        cmd.arg("--target").arg("pe-x86-64");
    }

    match cmd.output() {
        Ok(output) => {
            if !output.status.success() {
                println!(
                    "cargo:warning=windres failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return;
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to run windres: {e}");
            return;
        }
    }

    // Tell the linker to include the compiled resource
    if res_path.exists() {
        println!("cargo:rustc-link-arg={}", res_path.display());
    }
    // */
}
