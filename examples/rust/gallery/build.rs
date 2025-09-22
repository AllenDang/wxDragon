use embed_manifest::manifest::{ActiveCodePage, Setting, SupportedOS::*};
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    // Tell Cargo to rerun this build script if the build script changes
    println!("cargo::rerun-if-changed=build.rs");

    // Check if we're building for Windows (either natively or cross-compiling)
    let target = std::env::var("TARGET").unwrap_or_default();

    if target.contains("windows") {
        let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
        embed_windows_manifest(&pkg_name);

        embed_wx_resources();
    }
}

fn embed_windows_manifest(name: &str) {
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
        println!("cargo::warning=Failed to embed manifest: {e}");
        println!(
            "cargo::warning=The application will still work but may lack optimal Windows theming"
        );
    }
}

/// Compile and embed wx.rc resources for wxWidgets
fn embed_wx_resources() {
    // Find the wxWidgets directory with wx.rc

    // The wxdragon-sys crate's root directory
    let crate_dir = get_crate_dir("wxdragon-sys").expect("Could not get wxdragon-sys crate dir");

    // Look for wxWidgets directory
    let wx_dir = crate_dir.join("wxWidgets");
    let wx_rc_path = wx_dir.join("include").join("wx").join("msw").join("wx.rc");

    let wx_include_path = wx_dir.join("include");

    use embed_resource::{compile, CompilationResult, ParamsIncludeDirs};
    let res = compile(&wx_rc_path, ParamsIncludeDirs([&wx_include_path]));
    if res != CompilationResult::Ok {
        println!("cargo::warning=Compile resources with embed_resource: {res:?}");
    }
}

fn get_crate_dir(crate_name: &str) -> std::io::Result<std::path::PathBuf> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()?;

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let packages = metadata["packages"]
        .as_array()
        .ok_or(std::io::Error::other("packages"))?;

    for package in packages {
        if package["name"] == crate_name {
            let manifest_path = package["manifest_path"]
                .as_str()
                .ok_or(std::io::Error::other("manifest_path"))?;
            return Ok(std::path::PathBuf::from(manifest_path)
                .parent()
                .ok_or(std::io::Error::other("parent"))?
                .to_path_buf());
        }
    }
    Err(std::io::Error::other("crate_dir not found"))
}
