use embed_manifest::manifest::{ActiveCodePage, Setting, SupportedOS::*};
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    // Tell Cargo to rerun this build script if the build script changes
    println!("cargo:rerun-if-changed=build.rs");

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
        println!("cargo:warning=Failed to embed manifest: {e}");
        println!("cargo:warning=The application will still work but may lack optimal Windows theming");
    }
}

/// Compile and embed wx.rc resources for wxWidgets
fn embed_wx_resources() {
    // Find the wxWidgets directory
    let wx_dir = get_dest_bin_dir()
        .expect("Failed to get destination binary directory")
        .join("wxWidgets");

    let Ok(wx_rc_path) = get_wx_rc_path(&wx_dir) else {
        // If wx.rc is not found, skip embedding resources
        return;
    };

    let wx_include_path = wx_dir.join("include");

    use embed_resource::{CompilationResult, ParamsIncludeDirs, compile};
    let res = compile(&wx_rc_path, ParamsIncludeDirs([&wx_include_path]));
    if res != CompilationResult::Ok {
        println!("cargo::warning=Compile resources with embed_resource: {res:?}");
    }
}

#[allow(dead_code)]
fn get_crate_dir(crate_name: &str) -> std::io::Result<std::path::PathBuf> {
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()?;

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let packages = metadata["packages"].as_array().ok_or(std::io::Error::other("packages"))?;

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

fn get_wx_rc_path(wx_dir: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    let wx_rc_path = wx_dir.join("include").join("wx").join("msw").join("wx.rc");

    // Retry logic: Check if wx.rc exists, retry up to 10 times with a 5-second delay
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 10;
    const RETRY_DELAY_SECS: u64 = 5;

    while !wx_rc_path.exists() && retry_count < MAX_RETRIES {
        if retry_count == 0 {
            let p = wx_rc_path.display();
            println!("cargo::warning=wx.rc not found at {p}, waiting and retrying...");
        }

        println!(
            "cargo::warning=Retry {}/{MAX_RETRIES}: Waiting {RETRY_DELAY_SECS} seconds before checking again...",
            retry_count + 1
        );

        std::thread::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECS));
        retry_count += 1;
    }

    if !wx_rc_path.exists() {
        println!("cargo::warning=wx.rc not found at {wx_rc_path:?} after {MAX_RETRIES} retries, skipping resource embedding");
        return Err(std::io::Error::other("wx.rc not found"));
    }
    Ok(wx_rc_path)
}

fn get_dest_bin_dir() -> std::io::Result<std::path::PathBuf> {
    use std::env::var;
    use std::io::Error;
    use std::path::{Path, PathBuf};
    let out_dir = PathBuf::from(var("OUT_DIR").map_err(|e| Error::other(format!("OUT_DIR: {e}")))?);
    let profile = var("PROFILE").map_err(|e| Error::other(format!("PROFILE env var: {e}")))?;

    let dest_bin_dir = Path::new(&out_dir)
        .ancestors()
        .find(|p| p.file_name().map(|n| *n == *profile).unwrap_or(false))
        .ok_or(Error::other("destination binary directory not found"))?;
    Ok(dest_bin_dir.to_path_buf())
}
