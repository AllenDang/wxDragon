use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let bindings_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target = env::var("TARGET").unwrap();
    let out_dir = extract_matching_parent_dir(&bindings_out_dir, "target").unwrap();

    // Tell Cargo to rerun this build script if any files in following directories change
    println!("cargo:rerun-if-changed=cpp/src");
    println!("cargo:rerun-if-changed=cpp/include");
    println!("cargo:rerun-if-changed=build.rs");

    // --- 1. Generate FFI Bindings ---
    println!("info: Generating FFI bindings...");

    let mut bindings_builder = bindgen::Builder::default()
        .header("cpp/include/wxdragon.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Platform-specific bindgen configuration
    if target_os == "macos" {
        bindings_builder = bindings_builder
            .clang_arg("-D__WXOSX_COCOA__")
            .clang_arg("-D__WXMAC__")
            .clang_arg("-D__WXOSX__")
            .clang_arg("-D_FILE_OFFSET_BITS=64")
            .clang_arg("-DNDEBUG");
    } else if target_os == "windows" {
        bindings_builder = bindings_builder
            .clang_arg("-D__WXMSW__")
            .clang_arg("-D_FILE_OFFSET_BITS=64")
            .clang_arg("-DwxUSE_UNICODE=1")
            .clang_arg("-DNDEBUG")
            .clang_arg("-D__WXD_BINDGEN__=1"); // Tell our headers this is bindgen

        // Add MSVC-specific configuration for bindgen
        if target_env == "msvc" {
            bindings_builder = bindings_builder
                .clang_arg("-D_WIN32")
                .clang_arg("-D_WINDOWS")
                .clang_arg("-DWIN32");
        }
    } else if target_os == "linux" {
        bindings_builder = bindings_builder
            .clang_arg("-D__WXGTK__")
            .clang_arg("-D_FILE_OFFSET_BITS=64")
            .clang_arg("-DNDEBUG");
    }

    // Add feature flags for conditional compilation
    bindings_builder = bindings_builder
        .clang_arg(format!(
            "-DwxdUSE_AUI={}",
            if cfg!(feature = "aui") { 1 } else { 0 }
        ))
        .clang_arg(format!(
            "-DwxdUSE_MEDIACTRL={}",
            if cfg!(feature = "media-ctrl") { 1 } else { 0 }
        ))
        .clang_arg(format!(
            "-DwxdUSE_WEBVIEW={}",
            if cfg!(feature = "webview") { 1 } else { 0 }
        ))
        .clang_arg(format!(
            "-DwxdUSE_STC={}",
            if cfg!(feature = "stc") { 1 } else { 0 }
        ))
        .clang_arg(format!(
            "-DwxdUSE_XRC={}",
            if cfg!(feature = "xrc") { 1 } else { 0 }
        ))
        .clang_arg(format!(
            "-DwxdUSE_RICHTEXT={}",
            if cfg!(feature = "richtext") { 1 } else { 0 }
        ));

    bindings_builder = bindings_builder.clang_arg(format!("--target={target}"));

    // Skip library setup for docs.rs and rust-analyzer
    if env::var("DOCS_RS").is_ok() || env::var("RUST_ANALYZER") == Ok("true".to_string()) {
        let bindings = bindings_builder
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(bindings_out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");

        println!("info: Successfully generated FFI bindings");
        return;
    }

    // --- 2. Download and Setup Pre-built Libraries ---
    let wx_version = "3.3.1";

    download_prebuilt_libraries(wx_version, &out_dir)
        .expect("Failed to download pre-built wxWidgets libraries");

    // --- 3. Add wxWidgets Include Paths to Bindgen ---
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    // Check for official Windows 7 targets first
    let target_triple = env::var("TARGET").unwrap_or_default();
    let artifact_name = format!("wxwidgets-{wx_version}-{target_triple}-{profile}");

    let wx_lib_dir = out_dir.join(&artifact_name);

    // Add main wxWidgets headers
    let wx_main_include = wx_lib_dir.join("include");
    if wx_main_include.exists() {
        bindings_builder = bindings_builder.clang_arg(format!("-I{}", wx_main_include.display()));
        println!(
            "info: Added wxWidgets main include path: {}",
            wx_main_include.display()
        );
    }

    // Add platform-specific headers
    if target_os == "windows" {
        // For Windows builds, copy the working setup.h from include/wx/msw/setup.h to include/wx/setup.h
        // This allows wx/platform.h to find wx/setup.h via relative include
        let msw_setup = wx_lib_dir
            .join("include")
            .join("wx")
            .join("msw")
            .join("setup.h");
        let target_setup = wx_lib_dir.join("include").join("wx").join("setup.h");

        if msw_setup.exists() && !target_setup.exists() {
            if let Err(e) = std::fs::copy(&msw_setup, &target_setup) {
                println!("cargo:warning=Failed to copy setup.h: {e}");
            } else {
                println!("info: Copied Windows setup.h from msw to wx directory");
            }
        }
    } else if target_os == "macos" {
        // For macOS, look for the platform-specific paths
        let osx_setup_dirs = vec![wx_lib_dir
            .join("lib")
            .join("wx")
            .join("include")
            .join("osx_cocoa-unicode-static-3.3")];
        for setup_dir in osx_setup_dirs {
            if setup_dir.exists() {
                bindings_builder = bindings_builder.clang_arg(format!("-I{}", setup_dir.display()));
                println!(
                    "info: Added macOS setup include path: {}",
                    setup_dir.display()
                );
                break;
            }
        }
    } else if target_os == "linux" {
        // For Linux, look for GTK-specific paths
        let gtk_setup_dirs = vec![wx_lib_dir
            .join("lib")
            .join("wx")
            .join("include")
            .join("gtk3-unicode-static-3.3")];
        for setup_dir in gtk_setup_dirs {
            if setup_dir.exists() {
                bindings_builder = bindings_builder.clang_arg(format!("-I{}", setup_dir.display()));
                println!(
                    "info: Added Linux GTK setup include path: {}",
                    setup_dir.display()
                );
                break;
            }
        }
    }

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(bindings_out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("info: Successfully generated FFI bindings");

    // --- 4. Build wxDragon Wrapper ---
    build_wxdragon_wrapper(wx_version, &out_dir, &target_os, &target_env)
        .expect("Failed to build wxDragon wrapper library");

    // --- 5. Setup Linking ---
    setup_linking(wx_version, &target_os, &target_env, &out_dir);
}

pub fn extract_matching_parent_dir<P: AsRef<std::path::Path>>(
    path: P,
    match_name: &str,
) -> std::io::Result<std::path::PathBuf> {
    let mut sub_path = path.as_ref();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(match_name) {
            return Ok(parent.to_path_buf());
        }
        sub_path = parent;
    }
    let info = format!("No parent directory matching '{match_name}' found");
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, info))
}

fn download_prebuilt_libraries(
    wx_version: &str,
    target_dir: &std::path::Path,
) -> std::io::Result<()> {
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    // Check for official Windows 7 targets first
    let target_triple = std::env::var("TARGET").unwrap_or_default();
    let artifact_name = format!("wxwidgets-{wx_version}-{target_triple}-{profile}");

    let download_url = format!(
        "https://github.com/AllenDang/wxDragon/releases/download/wxwidgets-{wx_version}/{artifact_name}.tar.gz"
    );

    let tarball_dest_path = target_dir.join(format!("{artifact_name}.tar.gz"));
    let extracted_path = target_dir.join(&artifact_name);

    // Skip download if already extracted
    if extracted_path.exists() {
        println!("info: Using cached pre-built libraries at {extracted_path:?}");
        return Ok(());
    }

    println!("info: Downloading pre-built libraries from: {download_url}");

    // Download the pre-built libraries
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .expect("Failed to build reqwest client");

    let resp = client
        .get(&download_url)
        .send()
        .map_err(|e| std::io::Error::other(format!("Failed to download {download_url}: {e}")))?;

    if !resp.status().is_success() {
        return Err(std::io::Error::other(format!(
            "Failed to download {download_url}: HTTP {}",
            resp.status()
        )));
    }

    // Save the tarball
    let content = resp
        .bytes()
        .map_err(|e| std::io::Error::other(format!("Failed to read response content: {e}")))?;
    // create the destination file after downloading successfully to avoid creating an empty file
    let mut out_file = std::fs::File::create(&tarball_dest_path)?;
    std::io::copy(&mut content.as_ref(), &mut out_file)?;

    println!("info: Downloaded pre-built libraries to {tarball_dest_path:?}");

    if extracted_path.exists() {
        println!("info: Using cached pre-built libraries at {extracted_path:?}");
        return Ok(());
    }

    // Extract the tarball
    let tarball_file = std::fs::File::open(&tarball_dest_path).map_err(|e| {
        std::io::Error::other(format!("Failed to open tarball {tarball_dest_path:?}: {e}"))
    })?;
    let gz_decoder = flate2::read::GzDecoder::new(tarball_file);
    let mut archive = tar::Archive::new(gz_decoder);

    archive.unpack(target_dir)?;

    if !extracted_path.exists() {
        return Err(std::io::Error::other(format!("Extraction did not result in expected directory: {extracted_path:?}. Check tarball structure.")));
    }

    println!("info: Successfully extracted pre-built libraries to {extracted_path:?}");

    Ok(())
}

fn setup_linking(wx_version: &str, target_os: &str, target_env: &str, out_dir: &Path) {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    // Check for official Windows 7 targets first (same logic as download function)
    let target_triple = env::var("TARGET").unwrap_or_default();
    let artifact_name = format!("wxwidgets-{wx_version}-{target_triple}-{profile}");

    let lib_dir = out_dir.join(&artifact_name);

    // For Windows, libraries are in platform-specific subdirectories
    let actual_lib_dir = if target_os == "windows" {
        // Get target architecture for directory naming
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

        // For 32-bit Windows packages, check if they use generic "vc_lib" instead of "vc_x86_lib"
        let arch_suffix = if target_arch == "i686" || target_arch == "x86" {
            "x86"
        } else {
            "x64"
        };
        let generic_lib_dir = match target_env {
            "gnu" => lib_dir.join("gcc_lib"),
            "msvc" => lib_dir.join("vc_lib"),
            _ => lib_dir.clone(),
        };
        let arch_specific_lib_dir = match target_env {
            "gnu" => lib_dir.join(format!("gcc_{arch_suffix}_lib")),
            "msvc" => lib_dir.join(format!("vc_{arch_suffix}_lib")),
            _ => lib_dir.clone(),
        };

        // Try arch-specific first, then fall back to generic
        if arch_specific_lib_dir.exists() {
            arch_specific_lib_dir
        } else if generic_lib_dir.exists() {
            println!(
                "info: Using generic library directory: {}",
                generic_lib_dir.display()
            );
            generic_lib_dir
        } else {
            lib_dir
        }
    } else {
        lib_dir
    };

    // Add library search path
    println!(
        "cargo:rustc-link-search=native={}",
        actual_lib_dir.display()
    );

    // Debug: Show what libraries are actually available in the directory
    if actual_lib_dir.exists() {
        println!(
            "info: Library directory exists: {}",
            actual_lib_dir.display()
        );
        if let Ok(entries) = std::fs::read_dir(&actual_lib_dir) {
            let mut lib_files: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.ends_with(".a") || file_name.ends_with(".lib") {
                    lib_files.push(file_name);
                }
            }
            lib_files.sort();
            println!("info: Available library files: {lib_files:?}");
        }
    } else {
        println!(
            "cargo:warning=Library directory does not exist: {}",
            actual_lib_dir.display()
        );
    }

    // Link wxdragon wrapper library (will be built separately or included in pre-built package)
    println!("cargo:rustc-link-lib=static=wxdragon");

    // Platform-specific library linking
    match target_os {
        "macos" => link_macos_libraries(),
        "windows" => link_windows_libraries(target_env),
        "linux" => link_linux_libraries(),
        _ => panic!("Unsupported target OS: {target_os}"),
    }

    if target_os == "windows" && target_env == "gnu" {
        println!("cargo:rustc-link-arg=-v");
    }
}

fn link_macos_libraries() {
    // Core wxWidgets libraries for macOS
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_core-3.3");
    println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_adv-3.3");
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_gl-3.3");
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_propgrid-3.3");

    // Conditional feature libraries
    if cfg!(feature = "aui") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_aui-3.3");
    }
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_media-3.3");
    }
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_webview-3.3");
    }
    if cfg!(feature = "richtext") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_richtext-3.3");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_html-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
    }
    if cfg!(feature = "xrc") || cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_html-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_stc-3.3");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_xrc-3.3");
    }

    // Third-party libraries
    println!("cargo:rustc-link-lib=static=wxjpeg-3.3");
    println!("cargo:rustc-link-lib=static=wxpng-3.3");
    println!("cargo:rustc-link-lib=static=wxtiff-3.3");
    println!("cargo:rustc-link-lib=static=wxregexu-3.3");

    // System libraries
    println!("cargo:rustc-link-lib=expat");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=c++");

    // STC-specific libraries
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
        println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
    }

    // macOS frameworks
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=SystemConfiguration");

    // Conditional frameworks for macOS
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=AVKit");
        println!("cargo:rustc-link-lib=framework=CoreMedia");
    }

    // Fix for ___isPlatformVersionAtLeast undefined symbol on macOS arm64
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "macos" {
        // Use xcrun to find the toolchain path
        if let Ok(output) = std::process::Command::new("xcrun")
            .args(["--find", "clang"])
            .output()
        {
            if output.status.success() {
                let clang_path_str = String::from_utf8_lossy(&output.stdout);
                let clang_path = clang_path_str.trim();

                // Construct the clang runtime library path from the clang path
                // /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang
                // -> /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang
                if let Some(clang_dir) = std::path::Path::new(clang_path).parent() {
                    if let Some(usr_dir) = clang_dir.parent() {
                        let clang_rt_path = usr_dir.join("lib").join("clang");

                        // Try to find the clang runtime library
                        if let Ok(entries) = std::fs::read_dir(&clang_rt_path) {
                            for entry in entries.flatten() {
                                if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                                    let version_dir = entry.path();
                                    let lib_dir = version_dir.join("lib").join("darwin");
                                    let clang_rt_lib = lib_dir.join("libclang_rt.osx.a");

                                    if clang_rt_lib.exists() {
                                        println!(
                                            "cargo:rustc-link-search=native={}",
                                            lib_dir.display()
                                        );
                                        println!("cargo:rustc-link-lib=static=clang_rt.osx");
                                        println!(
                                            "info: Added clang runtime library for macOS arm64: {}",
                                            clang_rt_lib.display()
                                        );
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn link_windows_libraries(target_env: &str) {
    // Check if this is cross-compilation from macOS to Windows GNU
    let is_macos_to_windows_gnu = cfg!(target_os = "macos") && target_env == "gnu";

    // Determine if we need debug suffix based on build profile
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());
    // Note: Some prebuilt packages (especially 32-bit) might not have debug variants
    // We'll try with debug suffix first, then without if linking fails
    let debug_suffix = if profile == "debug" { "d" } else { "" };

    println!("info: Windows library linking - Profile: {profile}, Debug suffix: '{debug_suffix}'");
    println!("info: Target env: {target_env}, Cross-compilation: {is_macos_to_windows_gnu}");

    // For Windows GNU (both native and cross-compilation), use the actual library names from pre-built packages
    // Core wxWidgets libraries
    println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_core");
    println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_adv");
    println!("cargo:rustc-link-lib=static=wxbase33u{debug_suffix}");
    println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_gl");
    println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_propgrid");

    // Conditional feature libraries
    if cfg!(feature = "aui") {
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_aui");
    }
    if cfg!(feature = "richtext") {
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_richtext");
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_html");
        println!("cargo:rustc-link-lib=static=wxbase33u{debug_suffix}_xml");
    }
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_media");
    }
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_webview");
    }
    if cfg!(feature = "xrc") || cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_html");
        println!("cargo:rustc-link-lib=static=wxbase33u{debug_suffix}_xml");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_stc");
        println!("cargo:rustc-link-lib=static=wxscintilla{debug_suffix}");
        println!("cargo:rustc-link-lib=static=wxlexilla{debug_suffix}");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_xrc");
    }

    // Third-party libraries (using the actual names from pre-built packages)
    println!("cargo:rustc-link-lib=static=wxtiff{debug_suffix}");
    println!("cargo:rustc-link-lib=static=wxjpeg{debug_suffix}");
    println!("cargo:rustc-link-lib=static=wxpng{debug_suffix}");
    println!("cargo:rustc-link-lib=static=wxregexu{debug_suffix}");
    println!("cargo:rustc-link-lib=static=wxzlib{debug_suffix}");
    println!("cargo:rustc-link-lib=static=wxexpat{debug_suffix}");

    // Runtime libraries
    if target_env == "gnu" {
        if is_macos_to_windows_gnu {
            println!("info: Using static linking for cross-compilation from macOS to Windows GNU");

            // --- Dynamically find MinGW GCC library paths ---
            let gcc_path = "x86_64-w64-mingw32-gcc"; // Assume it's in PATH

            // Find the path containing libgcc.a
            let output_libgcc = std::process::Command::new(gcc_path)
                .arg("-print-libgcc-file-name")
                .output();

            if let Ok(output) = output_libgcc {
                if output.status.success() {
                    let libgcc_path_str =
                        String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !libgcc_path_str.is_empty() {
                        let libgcc_path = std::path::Path::new(&libgcc_path_str);
                        if let Some(libgcc_dir) = libgcc_path.parent() {
                            println!("cargo:rustc-link-search=native={}", libgcc_dir.display());
                            println!(
                                "info: Added GCC library search path (from libgcc): {}",
                                libgcc_dir.display()
                            );

                            // Attempt to find the path containing libstdc++.a (often one level up, in `../<target>/lib`)
                            if let Some(gcc_dir) = libgcc_dir.parent() {
                                // e.g., .../gcc/x86_64-w64-mingw32/15.1.0 -> .../gcc/x86_64-w64-mingw32
                                if let Some(toolchain_lib_dir) = gcc_dir.parent() {
                                    // e.g., .../gcc/x86_64-w64-mingw32 -> .../gcc
                                    if let Some(base_lib_dir) = toolchain_lib_dir.parent() {
                                        // e.g., .../gcc -> .../lib
                                        // Construct the expected path for libstdc++.a based on structure
                                        let libstdcpp_dir = base_lib_dir
                                            .parent()
                                            .unwrap()
                                            .join("x86_64-w64-mingw32/lib"); // ../../x86_64-w64-mingw32/lib
                                        if libstdcpp_dir.exists() && libstdcpp_dir != libgcc_dir {
                                            println!(
                                                "cargo:rustc-link-search=native={}",
                                                libstdcpp_dir.display()
                                            );
                                            println!(
                                                "info: Added GCC library search path (for libstdc++): {}",
                                                libstdcpp_dir.display()
                                            );
                                        } else {
                                            println!("info: Could not find or verify expected libstdc++ path relative to libgcc path: {}", libstdcpp_dir.display());
                                        }
                                    }
                                }
                            }
                        } else {
                            println!(
                                "cargo:warning=Could not get parent directory from libgcc path: {libgcc_path_str}"
                            );
                        }
                    } else {
                        println!(
                            "cargo:warning=Command -print-libgcc-file-name returned empty output."
                        );
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!(
                        "cargo:warning=Failed to run '{gcc_path} -print-libgcc-file-name': {stderr}"
                    );
                    println!("cargo:warning=Static linking for stdc++/gcc might fail. Falling back to hoping they are in default paths.");
                }
            } else {
                println!("cargo:warning=Could not execute x86_64-w64-mingw32-gcc. Static linking for stdc++/gcc might fail.");
            }
            // --- End dynamic path finding ---

            // Static linking for cross-compilation to avoid runtime dependencies
            println!("cargo:rustc-link-lib=static=stdc++");
            println!("cargo:rustc-link-lib=static=gcc");
            println!("cargo:rustc-link-lib=static=gcc_eh");
            println!("cargo:rustc-link-lib=static=pthread");
            // Add linker arguments for fully static C++ runtime
            println!("cargo:rustc-link-arg=-static-libgcc");
            println!("cargo:rustc-link-arg=-static-libstdc++");

            // Use UCRT instead of MSVCRT for cross-compilation (modern MinGW standard)
            // This is critical for compatibility with GCC 15.1.0 and recent MinGW distributions
            println!("cargo:rustc-link-lib=ucrt");
            println!("info: Using UCRT runtime for cross-compilation compatibility");
        } else {
            // Native Windows GNU builds
            // Check if we're in MSYS2 environment which uses UCRT
            let in_msys2 = env::var("MSYSTEM").is_ok()
                || env::var("MSYS2_PATH_TYPE").is_ok()
                || env::var("MINGW_PREFIX").is_ok();

            if in_msys2 {
                // MSYS2/MinGW64 static libraries for fully static build (dependency-free executable)
                // Use rustc-link-arg for static C++ runtime instead of static lib linking
                println!("cargo:rustc-link-arg=-static-libgcc");
                println!("cargo:rustc-link-arg=-static-libstdc++");

                // Add MSYS2 MinGW64 lib path to linker search path
                if let Ok(msys2_root) = std::env::var("MSYSTEM_PREFIX") {
                    println!("cargo:rustc-link-search=native={msys2_root}/lib");
                }

                // Standard libraries needed for MSYS2
                println!("cargo:rustc-link-lib=stdc++");
                println!("cargo:rustc-link-lib=gcc");
                println!("cargo:rustc-link-lib=mingw32");
                println!("cargo:rustc-link-lib=ucrt");
                println!("cargo:rustc-link-lib=winpthread");
                println!("info: Using MSYS2/MinGW64 static C++ runtime via rustc-link-arg and explicit lib path");
            } else {
                // Non-MSYS2 MinGW builds (dynamic linking)
                println!("cargo:rustc-link-lib=stdc++");
                println!("cargo:rustc-link-lib=gcc");
                println!("cargo:rustc-link-lib=mingw32");
                println!("cargo:rustc-link-lib=msvcrt");
            }
        }
    } else {
        // MSVC builds - use appropriate runtime based on build profile
        let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());
        if profile == "debug" {
            // Link debug runtime libraries for debug builds
            println!("cargo:rustc-link-lib=msvcrtd");
            println!("info: Using debug MSVC runtime (msvcrtd) for debug build");
        } else {
            // Link release runtime libraries for release builds
            println!("cargo:rustc-link-lib=msvcrt");
            println!("info: Using release MSVC runtime (msvcrt) for release build");
        }
    }

    // Windows system libraries
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=gdiplus");
    println!("cargo:rustc-link-lib=msimg32");
    println!("cargo:rustc-link-lib=comdlg32");
    println!("cargo:rustc-link-lib=winspool");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=shlwapi");
    println!("cargo:rustc-link-lib=comctl32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=oleaut32");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=rpcrt4");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=wininet");
    println!("cargo:rustc-link-lib=oleacc");
    println!("cargo:rustc-link-lib=uxtheme");
    println!("cargo:rustc-link-lib=imm32");
}

fn link_linux_libraries() {
    // Core wxWidgets libraries for Linux
    println!("cargo:rustc-link-lib=static=wx_gtk3u_core-3.3");
    println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
    println!("cargo:rustc-link-lib=static=wx_gtk3u_adv-3.3");
    println!("cargo:rustc-link-lib=static=wx_gtk3u_gl-3.3");
    println!("cargo:rustc-link-lib=static=wx_gtk3u_propgrid-3.3");

    // Conditional feature libraries
    if cfg!(feature = "aui") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_aui-3.3");
    }
    if cfg!(feature = "richtext") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_richtext-3.3");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_html-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
    }
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_webview-3.3");
    }
    if cfg!(feature = "xrc") || cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_html-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
    }
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_media-3.3");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_stc-3.3");
        println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
        println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_xrc-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
    }

    // System libraries
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=xkbcommon");

    // GTK and system libraries via pkg-config
    let lib = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
    for l in lib.libs {
        println!("cargo:rustc-link-lib={l}");
    }

    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=png");
    println!("cargo:rustc-link-lib=jpeg");
    println!("cargo:rustc-link-lib=expat");
    println!("cargo:rustc-link-lib=tiff");
}

fn build_wxdragon_wrapper(
    wx_version: &str,
    out_dir: &Path,
    target_os: &str,
    target_env: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Build wrapper library in the same mode as Cargo profile to match runtime libraries
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());
    let build_type = if profile == "debug" {
        "Debug"
    } else {
        "Release"
    };

    println!("info: Building wxDragon wrapper library in {build_type} mode");

    // Get the pre-built wxWidgets library directory (same naming as download_prebuilt_libraries)
    let target_triple = env::var("TARGET").unwrap_or_default();
    let artifact_name = format!("wxwidgets-{wx_version}-{target_triple}-{profile}");

    let wx_lib_dir = out_dir.join(artifact_name);
    let wrapper_build_dir = out_dir.join("wxdragon_wrapper_build");

    // Skip build if already built (handle different generator outputs)
    let output_lib = if target_env == "msvc" {
        // Visual Studio generator puts libraries in config subdirectories
        wrapper_build_dir.join(build_type).join("wxdragon.lib")
    } else {
        // Unix Makefiles generator puts libraries in lib/
        wrapper_build_dir.join("lib").join("libwxdragon.a")
    };

    if output_lib.exists() {
        println!("info: Using cached wxDragon wrapper library at {output_lib:?}");

        // Check for the built library in multiple possible locations
        let possible_lib_paths = if target_env == "msvc" {
            // Windows MSVC uses .lib files
            vec![
                wrapper_build_dir.join(format!("{build_type}/wxdragon.lib")),
                wrapper_build_dir.join(format!("lib/{build_type}/wxdragon.lib")),
                wrapper_build_dir.join(format!("x64/{build_type}/wxdragon.lib")),
                wrapper_build_dir.join(format!("{}/wxdragon.lib", build_type.to_lowercase())),
                wrapper_build_dir.join(format!("lib/{}/wxdragon.lib", build_type.to_lowercase())),
                wrapper_build_dir.join(format!("x64/{}/wxdragon.lib", build_type.to_lowercase())),
                wrapper_build_dir.join("wxdragon.lib"),
                wrapper_build_dir.join("lib/wxdragon.lib"),
                wrapper_build_dir.join("x64/wxdragon.lib"),
                wrapper_build_dir.join(format!("Win32/{build_type}/wxdragon.lib")),
                wrapper_build_dir.join(format!("lib/Win32/{build_type}/wxdragon.lib")),
                wrapper_build_dir.join(format!("out/{build_type}/wxdragon.lib")),
                wrapper_build_dir.join(format!("bin/{build_type}/wxdragon.lib")),
            ]
        } else {
            // Unix-like systems (Linux, macOS, Windows GNU) use .a files
            vec![
                wrapper_build_dir.join("lib/libwxdragon.a"),
                wrapper_build_dir.join("libwxdragon.a"),
                wrapper_build_dir.join(format!("lib/{build_type}/libwxdragon.a")),
                wrapper_build_dir.join(format!("{build_type}/libwxdragon.a")),
                wrapper_build_dir.join(format!("lib/{}/libwxdragon.a", build_type.to_lowercase())),
                wrapper_build_dir.join(format!("{}/libwxdragon.a", build_type.to_lowercase())),
            ]
        };

        let mut library_path = None;
        for path in &possible_lib_paths {
            if path.exists() {
                library_path = Some(path.clone());
                println!("info: Found library at: {}", path.display());
                break;
            }
        }

        let library_path = match library_path {
            Some(path) => path,
            None => {
                // List all files in build directory for debugging
                fn list_directory_recursive(dir: &Path, prefix: &str) -> String {
                    let mut result = String::new();
                    if let Ok(entries) = std::fs::read_dir(dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            result.push_str(&format!("{}  \"{}\"\n", prefix, path.display()));
                            if path.is_dir() {
                                result.push_str(&list_directory_recursive(
                                    &path,
                                    &format!("{prefix}  "),
                                ));
                            }
                        }
                    }
                    result
                }

                let build_contents = list_directory_recursive(&wrapper_build_dir, "");

                println!("Searched for library in these locations:");
                for path in &possible_lib_paths {
                    println!("  - {}", path.display());
                }

                return Err(format!(
                    "wxDragon wrapper library was not built successfully.\nExpected library file not found. Build directory contents:\n{}\nSearched locations:\n{}",
                    build_contents,
                    possible_lib_paths.iter().map(|p| format!("  - {}", p.display())).collect::<Vec<_>>().join("\n")
                ).into());
            }
        };

        let dest = if target_env == "msvc" {
            wx_lib_dir.join("wxdragon.lib")
        } else {
            wx_lib_dir.join("libwxdragon.a")
        };

        // Ensure destination directory exists
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::copy(&library_path, &dest)?;
        println!("info: Successfully built wxDragon wrapper library at {library_path:?}");

        return Ok(());
    }

    println!("info: Building wxDragon wrapper library...");

    // Create build directory
    std::fs::create_dir_all(&wrapper_build_dir)?;

    // Get absolute path to the cpp source directory
    let cpp_source_dir = env::var("CARGO_MANIFEST_DIR")
        .map(|manifest_dir| Path::new(&manifest_dir).join("cpp"))
        .unwrap_or_else(|_| Path::new("rust/wxdragon-sys/cpp").to_path_buf());

    // Prepare CMake command - use dynamic cmake detection
    let cmake_executable =
        if cfg!(target_os = "macos") && std::path::Path::new("/opt/homebrew/bin/cmake").exists() {
            "/opt/homebrew/bin/cmake"
        } else {
            "cmake" // Use cmake from PATH on other systems
        };
    let mut cmake_cmd = std::process::Command::new(cmake_executable);

    cmake_cmd
        .current_dir(&wrapper_build_dir)
        .arg(&cpp_source_dir) // Use absolute path to cpp source directory
        .arg(format!("-DCMAKE_BUILD_TYPE={build_type}"))
        .arg(format!("-DWXWIDGETS_LIB_DIR={}", wx_lib_dir.display()));

    // Pass Cargo feature flags to CMake
    cmake_cmd
        .arg(format!(
            "-DwxdUSE_AUI={}",
            if cfg!(feature = "aui") { 1 } else { 0 }
        ))
        .arg(format!(
            "-DwxdUSE_MEDIACTRL={}",
            if cfg!(feature = "media-ctrl") { 1 } else { 0 }
        ))
        .arg(format!(
            "-DwxdUSE_WEBVIEW={}",
            if cfg!(feature = "webview") { 1 } else { 0 }
        ))
        .arg(format!(
            "-DwxdUSE_STC={}",
            if cfg!(feature = "stc") { 1 } else { 0 }
        ))
        .arg(format!(
            "-DwxdUSE_XRC={}",
            if cfg!(feature = "xrc") { 1 } else { 0 }
        ))
        .arg(format!(
            "-DwxdUSE_RICHTEXT={}",
            if cfg!(feature = "richtext") { 1 } else { 0 }
        ));

    // Platform-specific CMake configuration
    // Detect host platform for cross-compilation scenarios
    let host_os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };

    // Explicitly set target system for cross-compilation
    if target_os != host_os {
        match target_os {
            "windows" => {
                cmake_cmd.arg("-DCMAKE_SYSTEM_NAME=Windows");
                // Set target architecture
                if target_arch == "x86_64" {
                    cmake_cmd.arg("-DCMAKE_SYSTEM_PROCESSOR=x86_64");
                } else if target_arch == "i686" || target_arch == "x86" {
                    cmake_cmd.arg("-DCMAKE_SYSTEM_PROCESSOR=x86");
                }

                // For cross-compilation from Unix to Windows GNU, we need to set up MinGW toolchain
                if target_env == "gnu" && host_os != "windows" {
                    // Try to find MinGW-w64 cross compiler
                    // For i686, the compiler is usually i686-w64-mingw32-g++
                    let cross_compiler = format!("{target_arch}-w64-mingw32-g++");

                    // Check if the cross compiler exists
                    if std::process::Command::new("which")
                        .arg(&cross_compiler)
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false)
                    {
                        cmake_cmd.arg(format!("-DCMAKE_CXX_COMPILER={cross_compiler}"));
                        cmake_cmd.arg(format!("-DCMAKE_C_COMPILER={target_arch}-w64-mingw32-gcc"));
                        println!("info: Using MinGW-w64 cross compiler: {cross_compiler}");
                    } else {
                        println!("cargo:warning=MinGW-w64 cross compiler not found. Cross-compilation to Windows GNU may fail.");
                        println!("cargo:warning=Consider installing mingw-w64 with: brew install mingw-w64");
                    }
                }
            }
            "linux" => {
                cmake_cmd.arg("-DCMAKE_SYSTEM_NAME=Linux");
                if target_arch == "x86_64" {
                    cmake_cmd.arg("-DCMAKE_SYSTEM_PROCESSOR=x86_64");
                } else if target_arch == "i686" || target_arch == "x86" {
                    cmake_cmd.arg("-DCMAKE_SYSTEM_PROCESSOR=x86");
                }
            }
            _ => {}
        }
    }

    match target_os {
        "windows" => {
            if target_env == "msvc" {
                // Use Visual Studio generator for better MSVC compatibility
                cmake_cmd.arg("-G").arg("Visual Studio 17 2022");
                // Set architecture for Visual Studio generator
                let vs_arch = if target_arch == "i686" || target_arch == "x86" {
                    "Win32"
                } else {
                    "x64"
                };
                cmake_cmd.arg("-A").arg(vs_arch);
            } else {
                // GNU/MinGW64 - choose generator based on environment
                if host_os == "windows" {
                    // Check if we're in MSYS2 environment (like GitHub Actions)
                    // MSYS2 usually sets these environment variables
                    let in_msys2 = env::var("MSYSTEM").is_ok()
                        || env::var("MSYS2_PATH_TYPE").is_ok()
                        || env::var("MINGW_PREFIX").is_ok();

                    if in_msys2 {
                        // In MSYS2, use Unix Makefiles for better compatibility
                        cmake_cmd.arg("-G").arg("Unix Makefiles");

                        // Let CMake find compilers naturally in MSYS2 environment
                        // Don't explicitly set compiler paths as MSYS2 handles this correctly
                        println!(
                            "info: Detected MSYS2 environment, using Unix Makefiles generator"
                        );

                        // Add static linking flags for full static build (dependency-free executable)
                        cmake_cmd.arg("-DCMAKE_CXX_FLAGS=-static-libgcc -static-libstdc++");
                        cmake_cmd.arg("-DCMAKE_EXE_LINKER_FLAGS=-static-libgcc -static-libstdc++");
                        cmake_cmd
                            .arg("-DCMAKE_SHARED_LINKER_FLAGS=-static-libgcc -static-libstdc++");
                        println!("info: Added MSYS2/MinGW64 static C++ runtime flags for dependency-free build");
                    } else {
                        // Native Windows MinGW (not MSYS2)
                        cmake_cmd.arg("-G").arg("MSYS Makefiles");
                    }
                } else {
                    // Cross-compiling from Unix-like system (macOS/Linux)
                    cmake_cmd.arg("-G").arg("Unix Makefiles");
                }
            }
        }
        "macos" => {
            cmake_cmd.arg("-DCMAKE_OSX_DEPLOYMENT_TARGET=10.13");
            if target_arch == "aarch64" {
                cmake_cmd.arg("-DCMAKE_OSX_ARCHITECTURES=arm64");
            } else {
                cmake_cmd.arg("-DCMAKE_OSX_ARCHITECTURES=x86_64");
            }
        }
        _ => {} // Linux uses default
    }

    // Configure
    let output = cmake_cmd.output()?;
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(
            format!("CMake configure failed:\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}").into(),
        );
    }

    // Build
    let mut build_cmd = std::process::Command::new(cmake_executable);
    build_cmd
        .current_dir(&wrapper_build_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg(build_type)
        .arg("--target")
        .arg("wxdragon")
        .arg("--verbose");

    println!("info: Running CMake build command: {build_cmd:?}");
    let output = build_cmd.output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Always print build output for debugging, even on success
    println!("CMake build stdout:\n{stdout}");
    if !stderr.is_empty() {
        println!("CMake build stderr:\n{stderr}");
    }

    if !output.status.success() {
        // If cmake --build fails and we're on Windows MSVC, try MSBuild directly as a fallback
        if target_env == "msvc" {
            println!("info: CMake build failed, trying MSBuild directly...");

            let mut msbuild_cmd = std::process::Command::new("msbuild");
            let msbuild_platform = if target_arch == "i686" || target_arch == "x86" {
                "Win32"
            } else {
                "x64"
            };
            msbuild_cmd
                .current_dir(&wrapper_build_dir)
                .arg("wxdragon.vcxproj")
                .arg(format!("/p:Configuration={build_type}"))
                .arg(format!("/p:Platform={msbuild_platform}"))
                .arg("/verbosity:detailed");

            println!("info: Running MSBuild command: {msbuild_cmd:?}");
            let msbuild_output = msbuild_cmd.output()?;

            let msbuild_stdout = String::from_utf8_lossy(&msbuild_output.stdout);
            let msbuild_stderr = String::from_utf8_lossy(&msbuild_output.stderr);

            println!("MSBuild stdout:\n{msbuild_stdout}");
            if !msbuild_stderr.is_empty() {
                println!("MSBuild stderr:\n{msbuild_stderr}");
            }

            if !msbuild_output.status.success() {
                return Err(format!(
                    "Both CMake build and MSBuild failed:\nCMake STDOUT:\n{stdout}\nCMake STDERR:\n{stderr}\nMSBuild STDOUT:\n{msbuild_stdout}\nMSBuild STDERR:\n{msbuild_stderr}"
                )
                .into());
            }
        } else {
            return Err(
                format!("CMake build failed:\nSTDOUT:\n{stdout}\nSTDERR:\n{stderr}").into(),
            );
        }
    } else {
        // Even if CMake reported success, check if it actually built anything
        // Look for compilation success indicators in the output
        let expected_lib_indicator = if target_env == "msvc" {
            "wxdragon.lib"
        } else {
            "libwxdragon.a"
        };
        let build_successful = stdout.contains("Build succeeded")
            || stdout.contains("succeeded")
            || stdout.contains(expected_lib_indicator)
            || stdout.contains("Building CXX object")
            || stdout.contains("Linking CXX static library");

        if !build_successful {
            println!("warn: CMake reported success but no compilation indicators found. Output may indicate a silent failure.");
        }
    }

    // Check for the built library in multiple possible locations (platform-specific)
    let possible_lib_paths = if target_env == "msvc" {
        // Windows MSVC uses .lib files
        vec![
            wrapper_build_dir.join(format!("{build_type}/wxdragon.lib")),
            wrapper_build_dir.join(format!("lib/{build_type}/wxdragon.lib")),
            wrapper_build_dir.join(format!("x64/{build_type}/wxdragon.lib")),
            wrapper_build_dir.join(format!("{}/wxdragon.lib", build_type.to_lowercase())),
            wrapper_build_dir.join(format!("lib/{}/wxdragon.lib", build_type.to_lowercase())),
            wrapper_build_dir.join(format!("x64/{}/wxdragon.lib", build_type.to_lowercase())),
            wrapper_build_dir.join("wxdragon.lib"),
            wrapper_build_dir.join("lib/wxdragon.lib"),
            wrapper_build_dir.join("x64/wxdragon.lib"),
            wrapper_build_dir.join(format!("Win32/{build_type}/wxdragon.lib")),
            wrapper_build_dir.join(format!("lib/Win32/{build_type}/wxdragon.lib")),
            wrapper_build_dir.join(format!("out/{build_type}/wxdragon.lib")),
            wrapper_build_dir.join(format!("bin/{build_type}/wxdragon.lib")),
        ]
    } else {
        // Unix-like systems (Linux, macOS, Windows GNU) use .a files
        vec![
            wrapper_build_dir.join("lib/libwxdragon.a"),
            wrapper_build_dir.join("libwxdragon.a"),
            wrapper_build_dir.join(format!("lib/{build_type}/libwxdragon.a")),
            wrapper_build_dir.join(format!("{build_type}/libwxdragon.a")),
            wrapper_build_dir.join(format!("lib/{}/libwxdragon.a", build_type.to_lowercase())),
            wrapper_build_dir.join(format!("{}/libwxdragon.a", build_type.to_lowercase())),
        ]
    };

    let mut library_path = None;
    for path in &possible_lib_paths {
        if path.exists() {
            library_path = Some(path.clone());
            println!("info: Found library at: {}", path.display());
            break;
        }
    }

    let library_path = match library_path {
        Some(path) => path,
        None => {
            // List all files in build directory for debugging
            fn list_directory_recursive(dir: &Path, prefix: &str) -> String {
                let mut result = String::new();
                if let Ok(entries) = std::fs::read_dir(dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        result.push_str(&format!("{}  \"{}\"\n", prefix, path.display()));
                        if path.is_dir() {
                            result
                                .push_str(&list_directory_recursive(&path, &format!("{prefix}  ")));
                        }
                    }
                }
                result
            }

            let build_contents = list_directory_recursive(&wrapper_build_dir, "");

            println!("Searched for library in these locations:");
            for path in &possible_lib_paths {
                println!("  - {}", path.display());
            }

            return Err(format!(
                "wxDragon wrapper library was not built successfully.\nExpected library file not found. Build directory contents:\n{}\nSearched locations:\n{}",
                build_contents,
                possible_lib_paths.iter().map(|p| format!("  - {}", p.display())).collect::<Vec<_>>().join("\n")
            ).into());
        }
    };

    let dest = if target_os == "windows" {
        // For Windows, copy to the platform-specific subdirectory where the linker expects it
        let arch_suffix = if target_arch == "i686" || target_arch == "x86" {
            "x86"
        } else {
            "x64"
        };
        // Check for both arch-specific and generic library directories
        let arch_specific_lib_dir = match target_env {
            "msvc" => wx_lib_dir.join(format!("vc_{arch_suffix}_lib")),
            "gnu" => wx_lib_dir.join(format!("gcc_{arch_suffix}_lib")),
            _ => wx_lib_dir.clone(),
        };
        let generic_lib_dir = match target_env {
            "msvc" => wx_lib_dir.join("vc_lib"),
            "gnu" => wx_lib_dir.join("gcc_lib"),
            _ => wx_lib_dir.clone(),
        };

        let platform_lib_dir = if arch_specific_lib_dir.exists() {
            arch_specific_lib_dir
        } else {
            generic_lib_dir
        };

        if target_env == "msvc" {
            platform_lib_dir.join("wxdragon.lib")
        } else {
            platform_lib_dir.join("libwxdragon.a")
        }
    } else {
        // For non-Windows platforms, use the root directory
        wx_lib_dir.join("libwxdragon.a")
    };

    // Ensure destination directory exists
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::copy(&library_path, &dest)?;
    println!("info: Successfully built wxDragon wrapper library at {library_path:?}");

    Ok(())
}
