use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target = env::var("TARGET").unwrap();

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
    if cfg!(feature = "aui") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_AUI=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_AUI=0");
    }

    if cfg!(feature = "media-ctrl") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_MEDIACTRL=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_MEDIACTRL=0");
    }

    if cfg!(feature = "webview") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_WEBVIEW=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_WEBVIEW=0");
    }

    if cfg!(feature = "stc") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_STC=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_STC=0");
    }

    if cfg!(feature = "xrc") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_XRC=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_XRC=0");
    }

    bindings_builder = bindings_builder.clang_arg(format!("--target={}", target));

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("info: Successfully generated FFI bindings");

    // Skip library setup for docs.rs and rust-analyzer
    if env::var("DOCS_RS").is_ok() || env::var("RUST_ANALYZER") == Ok("true".to_string()) {
        return;
    }

    // --- 2. Download and Setup Pre-built Libraries ---
    let wx_version = "3.3.0";

    download_prebuilt_libraries(wx_version, &out_dir, &target_os, &target_env)
        .expect("Failed to download pre-built wxWidgets libraries");

    // --- 3. Build wxDragon Wrapper ---
    build_wxdragon_wrapper(&out_dir, &target_os, &target_env)
        .expect("Failed to build wxDragon wrapper library");

    // --- 4. Setup Linking ---
    setup_linking(&target_os, &target_env, &out_dir);
}

fn download_prebuilt_libraries(
    wx_version: &str,
    out_dir: &Path,
    target_os: &str,
    target_env: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let profile = env::var("PROFILE").unwrap_or_else(|_| "release".to_string());

    let artifact_name = match (target_os, target_arch.as_str(), target_env) {
        ("linux", "x86_64", _) => format!("wxwidgets-{}-linux-x64-{}", wx_version, profile),
        ("macos", "x86_64", _) => format!("wxwidgets-{}-macos-x64-{}", wx_version, profile),
        ("macos", "aarch64", _) => format!("wxwidgets-{}-macos-arm64-{}", wx_version, profile),
        ("windows", "x86_64", "msvc") => format!("wxwidgets-{}-windows-msvc-x64-{}", wx_version, profile),
        ("windows", "x86_64", "gnu") => format!("wxwidgets-{}-windows-gnu-x64-{}", wx_version, profile),
        _ => {
            return Err(format!(
                "Unsupported target platform: {}-{}-{}",
                target_os, target_arch, target_env
            )
            .into())
        }
    };

    let download_url = format!(
        "https://github.com/AllenDang/wxDragon/releases/download/wxwidgets-{}/{}.tar.gz",
        wx_version, artifact_name
    );

    let tarball_dest_path = out_dir.join(format!("{}.tar.gz", artifact_name));
    let extracted_path = out_dir.join(&artifact_name);

    // Skip download if already extracted
    if extracted_path.exists() {
        println!(
            "info: Using cached pre-built libraries at {:?}",
            extracted_path
        );
        return Ok(());
    }

    println!(
        "info: Downloading pre-built libraries from: {}",
        download_url
    );

    // Download the pre-built libraries
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .expect("Failed to build reqwest client");

    let resp = client
        .get(&download_url)
        .send()
        .map_err(|e| format!("Failed to download {}: {}", download_url, e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Failed to download {}: HTTP {}",
            download_url,
            resp.status()
        )
        .into());
    }

    // Save the tarball
    let mut out_file = File::create(&tarball_dest_path).map_err(|e| {
        format!(
            "Failed to create destination file {:?}: {}",
            tarball_dest_path, e
        )
    })?;
    let content = resp
        .bytes()
        .map_err(|e| format!("Failed to read response content: {}", e))?;
    std::io::copy(&mut content.as_ref(), &mut out_file)
        .map_err(|e| format!("Failed to write downloaded content: {}", e))?;

    println!(
        "info: Downloaded pre-built libraries to {:?}",
        tarball_dest_path
    );

    // Extract the tarball
    let tarball_file = File::open(&tarball_dest_path)
        .map_err(|e| format!("Failed to open tarball {:?}: {}", tarball_dest_path, e))?;
    let gz_decoder = flate2::read::GzDecoder::new(tarball_file);
    let mut archive = tar::Archive::new(gz_decoder);

    archive.unpack(&out_dir).map_err(|e| {
        format!(
            "Failed to extract {} to {:?}: {}",
            artifact_name, out_dir, e
        )
    })?;

    if !extracted_path.exists() {
        return Err(format!(
            "Extraction did not result in expected directory: {:?}. Check tarball structure.",
            extracted_path
        )
        .into());
    }

    println!(
        "info: Successfully extracted pre-built libraries to {:?}",
        extracted_path
    );
    Ok(())
}

fn setup_linking(target_os: &str, target_env: &str, out_dir: &Path) {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Get the pre-built library directory
    let artifact_name = match (target_os, target_arch.as_str(), target_env) {
        ("linux", "x86_64", _) => "wxwidgets-linux-x64",
        ("macos", "x86_64", _) => "wxwidgets-macos-x64",
        ("macos", "aarch64", _) => "wxwidgets-macos-arm64",
        ("windows", "x86_64", "msvc") => "wxwidgets-windows-msvc-x64",
        ("windows", "x86_64", "gnu") => "wxwidgets-windows-gnu-x64",
        _ => panic!(
            "Unsupported target platform: {}-{}-{}",
            target_os, target_arch, target_env
        ),
    };

    let lib_dir = out_dir.join(artifact_name);

    // Add library search path
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // Link wxdragon wrapper library (will be built separately or included in pre-built package)
    println!("cargo:rustc-link-lib=static=wxdragon");

    // Platform-specific library linking
    match target_os {
        "macos" => link_macos_libraries(),
        "windows" => link_windows_libraries(target_env),
        "linux" => link_linux_libraries(),
        _ => panic!("Unsupported target OS: {}", target_os),
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
    if cfg!(feature = "xrc") || cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_html-3.3");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_stc-3.3");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_xrc-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
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
}

fn link_windows_libraries(target_env: &str) {
    // Check if this is cross-compilation from macOS to Windows GNU
    let is_macos_to_windows_gnu = cfg!(target_os = "macos") && target_env == "gnu";

    // For Windows GNU (both native and cross-compilation), use the actual library names from pre-built packages
    // Core wxWidgets libraries
    println!("cargo:rustc-link-lib=static=wxmsw33u_core");
    println!("cargo:rustc-link-lib=static=wxmsw33u_adv");
    println!("cargo:rustc-link-lib=static=wxbase33u");
    println!("cargo:rustc-link-lib=static=wxmsw33u_gl");
    println!("cargo:rustc-link-lib=static=wxmsw33u_propgrid");
    println!("cargo:rustc-link-lib=static=wxmsw33u_html");

    // Conditional feature libraries
    if cfg!(feature = "aui") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_aui");
    }
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_media");
    }
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_webview");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_stc");
        println!("cargo:rustc-link-lib=static=wxscintilla");
        println!("cargo:rustc-link-lib=static=wxlexilla");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_xrc");
        println!("cargo:rustc-link-lib=static=wxbase33u_xml");
    }

    // Third-party libraries (using the actual names from pre-built packages)
    println!("cargo:rustc-link-lib=static=wxtiff");
    println!("cargo:rustc-link-lib=static=wxjpeg");
    println!("cargo:rustc-link-lib=static=wxpng");
    println!("cargo:rustc-link-lib=static=wxregexu");
    println!("cargo:rustc-link-lib=static=wxzlib");
    println!("cargo:rustc-link-lib=static=wxexpat");

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
                                "cargo:warning=Could not get parent directory from libgcc path: {}",
                                libgcc_path_str
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
                        "cargo:warning=Failed to run '{} -print-libgcc-file-name': {}",
                        gcc_path, stderr
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
            let in_msys2 = env::var("MSYSTEM").is_ok() || 
                           env::var("MSYS2_PATH_TYPE").is_ok() ||
                           env::var("MINGW_PREFIX").is_ok();
            
            if in_msys2 {
                // MSYS2/MinGW64 static libraries for fully static build (dependency-free executable)
                // Use rustc-link-arg for static C++ runtime instead of static lib linking
                println!("cargo:rustc-link-arg=-static-libgcc");
                println!("cargo:rustc-link-arg=-static-libstdc++");
                
                // Add MSYS2 MinGW64 lib path to linker search path
                if let Ok(msys2_root) = std::env::var("MSYSTEM_PREFIX") {
                    println!("cargo:rustc-link-search=native={}/lib", msys2_root);
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
        // MSVC builds
        println!("cargo:rustc-link-lib=msvcrt");
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
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_webview-3.3");
    }
    if cfg!(feature = "xrc") || cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_html-3.3");
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
    let libgtk = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
    let libnotify = pkg_config::Config::new().probe("libnotify").unwrap();

    let mut libs = vec![];
    libs.extend(libgtk.libs.iter());
    libs.extend(libnotify.libs.iter());
    libs.dedup();

    for l in libs {
        println!("cargo:rustc-link-lib={}", l);
    }

    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=png");
    println!("cargo:rustc-link-lib=jpeg");
    println!("cargo:rustc-link-lib=expat");
    println!("cargo:rustc-link-lib=tiff");
}

fn build_wxdragon_wrapper(
    out_dir: &Path,
    target_os: &str,
    target_env: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Always build wrapper library in Release mode to match pre-built wxWidgets libraries
    // This avoids Debug/Release runtime library mismatches on Windows MSVC
    let build_type = "Release";
    
    println!("info: Building wxDragon wrapper library in {} mode", build_type);

    // Get the pre-built wxWidgets library directory
    let artifact_name = match (target_os, target_arch.as_str(), target_env) {
        ("linux", "x86_64", _) => "wxwidgets-linux-x64",
        ("macos", "x86_64", _) => "wxwidgets-macos-x64",
        ("macos", "aarch64", _) => "wxwidgets-macos-arm64",
        ("windows", "x86_64", "msvc") => "wxwidgets-windows-msvc-x64",
        ("windows", "x86_64", "gnu") => "wxwidgets-windows-gnu-x64",
        _ => {
            return Err(format!(
                "Unsupported target platform: {}-{}-{}",
                target_os, target_arch, target_env
            )
            .into())
        }
    };

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
        println!(
            "info: Using cached wxDragon wrapper library at {:?}",
            output_lib
        );

        // Check for the built library in multiple possible locations
        let possible_lib_paths = if target_env == "msvc" {
            // Windows MSVC uses .lib files
            vec![
                wrapper_build_dir.join(format!("{}/wxdragon.lib", build_type)),
                wrapper_build_dir.join(format!("lib/{}/wxdragon.lib", build_type)),
                wrapper_build_dir.join(format!("x64/{}/wxdragon.lib", build_type)),
                wrapper_build_dir.join(format!("{}/wxdragon.lib", build_type.to_lowercase())),
                wrapper_build_dir.join(format!("lib/{}/wxdragon.lib", build_type.to_lowercase())),
                wrapper_build_dir.join(format!("x64/{}/wxdragon.lib", build_type.to_lowercase())),
                wrapper_build_dir.join("wxdragon.lib"),
                wrapper_build_dir.join("lib/wxdragon.lib"),
                wrapper_build_dir.join("x64/wxdragon.lib"),
                wrapper_build_dir.join(format!("Win32/{}/wxdragon.lib", build_type)),
                wrapper_build_dir.join(format!("lib/Win32/{}/wxdragon.lib", build_type)),
                wrapper_build_dir.join(format!("out/{}/wxdragon.lib", build_type)),
                wrapper_build_dir.join(format!("bin/{}/wxdragon.lib", build_type)),
            ]
        } else {
            // Unix-like systems (Linux, macOS, Windows GNU) use .a files
            vec![
                wrapper_build_dir.join("lib/libwxdragon.a"),
                wrapper_build_dir.join("libwxdragon.a"),
                wrapper_build_dir.join(format!("lib/{}/libwxdragon.a", build_type)),
                wrapper_build_dir.join(format!("{}/libwxdragon.a", build_type)),
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
                                result.push_str(&list_directory_recursive(&path, &format!("{}  ", prefix)));
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
        std::fs::copy(&library_path, &dest)?;
        println!("info: Successfully built wxDragon wrapper library at {:?}", library_path);

        return Ok(());
    }

    println!("info: Building wxDragon wrapper library...");

    // Create build directory
    std::fs::create_dir_all(&wrapper_build_dir)?;

    // Get absolute path to the cpp source directory
    let cpp_source_dir = env::var("CARGO_MANIFEST_DIR")
        .map(|manifest_dir| Path::new(&manifest_dir).join("cpp"))
        .unwrap_or_else(|_| Path::new("rust/wxdragon-sys/cpp").to_path_buf());

    // Prepare CMake command
    let mut cmake_cmd = std::process::Command::new("cmake");
    
    cmake_cmd
        .current_dir(&wrapper_build_dir)
        .arg(&cpp_source_dir) // Use absolute path to cpp source directory
        .arg(format!("-DCMAKE_BUILD_TYPE={}", build_type))
        .arg(format!("-DWXWIDGETS_LIB_DIR={}", wx_lib_dir.display()));

    // Pass Cargo feature flags to CMake
    cmake_cmd.arg(format!(
        "-DwxdUSE_AUI={}",
        if cfg!(feature = "aui") { "ON" } else { "OFF" }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_MEDIACTRL={}",
        if cfg!(feature = "media-ctrl") {
            "ON"
        } else {
            "OFF"
        }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_WEBVIEW={}",
        if cfg!(feature = "webview") {
            "ON"
        } else {
            "OFF"
        }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_STC={}",
        if cfg!(feature = "stc") { "ON" } else { "OFF" }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_XRC={}",
        if cfg!(feature = "xrc") { "ON" } else { "OFF" }
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
                }

                // For cross-compilation from Unix to Windows GNU, we need to set up MinGW toolchain
                if target_env == "gnu" && host_os != "windows" {
                    // Try to find MinGW-w64 cross compiler
                    let cross_compiler = format!("{}-w64-mingw32-g++", target_arch);

                    // Check if the cross compiler exists
                    if std::process::Command::new("which")
                        .arg(&cross_compiler)
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false)
                    {
                        cmake_cmd.arg(format!("-DCMAKE_CXX_COMPILER={}", cross_compiler));
                        cmake_cmd.arg(format!(
                            "-DCMAKE_C_COMPILER={}-w64-mingw32-gcc",
                            target_arch
                        ));
                        println!("info: Using MinGW-w64 cross compiler: {}", cross_compiler);
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
                cmake_cmd.arg("-A").arg("x64"); // Set architecture for Visual Studio generator
            } else {
                // GNU/MinGW64 - choose generator based on environment
                if host_os == "windows" {
                    // Check if we're in MSYS2 environment (like GitHub Actions)
                    // MSYS2 usually sets these environment variables
                    let in_msys2 = env::var("MSYSTEM").is_ok() || 
                                   env::var("MSYS2_PATH_TYPE").is_ok() ||
                                   env::var("MINGW_PREFIX").is_ok();
                    
                    if in_msys2 {
                        // In MSYS2, use Unix Makefiles for better compatibility
                        cmake_cmd.arg("-G").arg("Unix Makefiles");
                        
                        // Let CMake find compilers naturally in MSYS2 environment
                        // Don't explicitly set compiler paths as MSYS2 handles this correctly
                        println!("info: Detected MSYS2 environment, using Unix Makefiles generator");
                        
                        // Add static linking flags for full static build (dependency-free executable)
                        cmake_cmd.arg("-DCMAKE_CXX_FLAGS=-static-libgcc -static-libstdc++");
                        cmake_cmd.arg("-DCMAKE_EXE_LINKER_FLAGS=-static-libgcc -static-libstdc++");
                        cmake_cmd.arg("-DCMAKE_SHARED_LINKER_FLAGS=-static-libgcc -static-libstdc++");
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
        return Err(format!(
            "CMake configure failed:\nSTDOUT:\n{}\nSTDERR:\n{}",
            stdout, stderr
        )
        .into());
    }

    // Build
    let mut build_cmd = std::process::Command::new("cmake");
    build_cmd
        .current_dir(&wrapper_build_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg(build_type)
        .arg("--target")
        .arg("wxdragon")
        .arg("--verbose");

    println!("info: Running CMake build command: {:?}", build_cmd);
    let output = build_cmd.output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Always print build output for debugging, even on success
    println!("CMake build stdout:\n{}", stdout);
    if !stderr.is_empty() {
        println!("CMake build stderr:\n{}", stderr);
    }
    
    if !output.status.success() {
        // If cmake --build fails and we're on Windows MSVC, try MSBuild directly as a fallback
        if target_env == "msvc" {
            println!("info: CMake build failed, trying MSBuild directly...");
            
            let mut msbuild_cmd = std::process::Command::new("msbuild");
            msbuild_cmd
                .current_dir(&wrapper_build_dir)
                .arg("wxdragon.vcxproj")
                .arg(&format!("/p:Configuration={}", build_type))
                .arg("/p:Platform=x64")
                .arg("/verbosity:detailed");
                
            println!("info: Running MSBuild command: {:?}", msbuild_cmd);
            let msbuild_output = msbuild_cmd.output()?;
            
            let msbuild_stdout = String::from_utf8_lossy(&msbuild_output.stdout);
            let msbuild_stderr = String::from_utf8_lossy(&msbuild_output.stderr);
            
            println!("MSBuild stdout:\n{}", msbuild_stdout);
            if !msbuild_stderr.is_empty() {
                println!("MSBuild stderr:\n{}", msbuild_stderr);
            }
            
            if !msbuild_output.status.success() {
                return Err(format!(
                    "Both CMake build and MSBuild failed:\nCMake STDOUT:\n{}\nCMake STDERR:\n{}\nMSBuild STDOUT:\n{}\nMSBuild STDERR:\n{}",
                    stdout, stderr, msbuild_stdout, msbuild_stderr
                )
                .into());
            }
        } else {
            return Err(format!(
                "CMake build failed:\nSTDOUT:\n{}\nSTDERR:\n{}",
                stdout, stderr
            )
            .into());
        }
    } else {
        // Even if CMake reported success, check if it actually built anything
        // Look for compilation success indicators in the output
        let expected_lib_indicator = if target_env == "msvc" { "wxdragon.lib" } else { "libwxdragon.a" };
        let build_successful = stdout.contains("Build succeeded") || 
                              stdout.contains("succeeded") || 
                              stdout.contains(expected_lib_indicator) ||
                              stdout.contains("Building CXX object") ||
                              stdout.contains("Linking CXX static library");
        
        if !build_successful {
            println!("warn: CMake reported success but no compilation indicators found. Output may indicate a silent failure.");
        }
    }

    // Check for the built library in multiple possible locations (platform-specific)
    let possible_lib_paths = if target_env == "msvc" {
        // Windows MSVC uses .lib files
        vec![
            wrapper_build_dir.join(format!("{}/wxdragon.lib", build_type)),
            wrapper_build_dir.join(format!("lib/{}/wxdragon.lib", build_type)),
            wrapper_build_dir.join(format!("x64/{}/wxdragon.lib", build_type)),
            wrapper_build_dir.join(format!("{}/wxdragon.lib", build_type.to_lowercase())),
            wrapper_build_dir.join(format!("lib/{}/wxdragon.lib", build_type.to_lowercase())),
            wrapper_build_dir.join(format!("x64/{}/wxdragon.lib", build_type.to_lowercase())),
            wrapper_build_dir.join("wxdragon.lib"),
            wrapper_build_dir.join("lib/wxdragon.lib"),
            wrapper_build_dir.join("x64/wxdragon.lib"),
            wrapper_build_dir.join(format!("Win32/{}/wxdragon.lib", build_type)),
            wrapper_build_dir.join(format!("lib/Win32/{}/wxdragon.lib", build_type)),
            wrapper_build_dir.join(format!("out/{}/wxdragon.lib", build_type)),
            wrapper_build_dir.join(format!("bin/{}/wxdragon.lib", build_type)),
        ]
    } else {
        // Unix-like systems (Linux, macOS, Windows GNU) use .a files
        vec![
            wrapper_build_dir.join("lib/libwxdragon.a"),
            wrapper_build_dir.join("libwxdragon.a"),
            wrapper_build_dir.join(format!("lib/{}/libwxdragon.a", build_type)),
            wrapper_build_dir.join(format!("{}/libwxdragon.a", build_type)),
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
                            result.push_str(&list_directory_recursive(&path, &format!("{}  ", prefix)));
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
    std::fs::copy(&library_path, &dest)?;
    println!("info: Successfully built wxDragon wrapper library at {:?}", library_path);

    Ok(())
}
