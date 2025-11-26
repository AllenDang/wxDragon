const WX_SRC_URL: &str = "https://github.com/wxWidgets/wxWidgets/releases/download/v3.3.1/wxWidgets-3.3.1.zip";
const WX_VERSION: &str = "3.3.1";
const WX_SRC_URL_SHA256: &str = "c25311fecbc4b508577bdee4e90660da8d0d79f4099bc40e9cac8338879f9334";

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    println!("Building wxdragon-sys...");

    println!("cargo::rerun-if-changed=cpp");
    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target = std::env::var("TARGET").unwrap();
    let profile = std::env::var("PROFILE").unwrap();

    let mut bindings_builder = bindgen::Builder::default()
        .header("cpp/include/wxdragon.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("--target={target}"));

    // Feature flags for conditional compilation in headers
    bindings_builder = bindings_builder
        .clang_arg(format!("-DwxdUSE_AUI={}", if cfg!(feature = "aui") { 1 } else { 0 }))
        .clang_arg(format!(
            "-DwxdUSE_MEDIACTRL={}",
            if cfg!(feature = "media-ctrl") { 1 } else { 0 }
        ))
        .clang_arg(format!("-DwxdUSE_WEBVIEW={}", if cfg!(feature = "webview") { 1 } else { 0 }))
        .clang_arg(format!("-DwxdUSE_STC={}", if cfg!(feature = "stc") { 1 } else { 0 }))
        .clang_arg(format!("-DwxdUSE_XRC={}", if cfg!(feature = "xrc") { 1 } else { 0 }))
        .clang_arg(format!(
            "-DwxdUSE_RICHTEXT={}",
            if cfg!(feature = "richtext") { 1 } else { 0 }
        ));

    // Skip library setup for docs.rs and rust-analyzer
    if std::env::var("DOCS_RS").is_ok() || std::env::var("RUST_ANALYZER") == Ok("true".to_string()) {
        println!("info: docs/IDE mode - generating minimal bindings only");

        let bindings = bindings_builder
            .generate()
            .expect("Unable to generate bindings (docs.rs mode)");
        bindings
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");
        println!("cargo::warning=Successfully generated FFI bindings (docs/IDE)");
        return Ok(());
    }

    let dest_bin_dir = std::path::Path::new(&out_dir)
        .ancestors()
        .find(|p| p.file_name().map(|n| *n == *profile).unwrap_or(false))
        .expect("Could not find destination binary directory");
    // Download wxWidgets to target directory to avoid including it in published crate
    let wxwidgets_dir = dest_bin_dir.join("wxWidgets");

    let wxwidgets_dir_str = wxwidgets_dir.display().to_string();

    let ver_matched = chk_wx_version(&wxwidgets_dir, WX_VERSION).unwrap_or(false);
    if !ver_matched {
        std::fs::remove_dir_all(&wxwidgets_dir).ok();

        let archive_dest_path = std::env::temp_dir().join("wxWidgets.zip");

        #[allow(clippy::print_literal)]
        if let Err(e) = download_file_with_git_http_proxy(WX_SRC_URL, &archive_dest_path, WX_SRC_URL_SHA256) {
            println!(
                "cargo::error=Could not download wxWidgets source archive from {WX_SRC_URL}: {e}\n{}\n{}",
                "Potential solutions: Check your network connectivity, ensure the URL is accessible,",
                "and verify any proxy settings and set it via `git config --global http.proxy http://your-proxy:port`."
            );
            return Err(Box::new(e));
        }

        if let Err(e) = extract_zip_archive(&archive_dest_path, &wxwidgets_dir) {
            println!("cargo::error=Could not extract wxWidgets source archive: {e}");
            if wxwidgets_dir.exists()
                && let Err(remove_err) = std::fs::remove_dir_all(&wxwidgets_dir)
            {
                println!("cargo::warning=Failed to clean up {wxwidgets_dir:?} directory after extraction error: {remove_err}");
            }
            return Err(Box::new(e));
        }
    }

    // --- 1. Generate FFI Bindings ---
    println!("info: Generating FFI bindings...");

    bindings_builder = bindings_builder.clang_arg(format!("-I{wxwidgets_dir_str}/include"));

    // From this point on we assume full build (not docs/IDE mode)

    let mut bindings_builder2 = bindings_builder.clone();
    let bindings = match bindings_builder.generate() {
        Ok(bindings) => bindings,
        Err(_e) => {
            // To avoid the problem of header file conflicts caused by the coexistence of GCC and CLang.
            if target_os == "windows" && target_env == "gnu" {
                // `gcc -xc -E -v nul` to get include paths
                let output = std::process::Command::new("gcc")
                    .args(["-xc", "-E", "-v", "nul"])
                    .output()
                    .expect("Failed to run gcc to get include path");
                let stderr = String::from_utf8_lossy(&output.stderr);
                let mut in_search = false;
                for line in stderr.lines() {
                    if line.contains("#include <...> search starts here:") {
                        in_search = true;
                        continue;
                    }
                    if line.contains("End of search list.") {
                        break;
                    }
                    if in_search {
                        let path = line.trim();
                        bindings_builder2 = bindings_builder2.clang_arg(format!("-I{path}"));
                    }
                }
            }

            bindings_builder2.generate().expect("Unable to generate bindings")
        }
    };

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("info: Successfully generated FFI bindings");

    // --- 4. Build wxDragon Wrapper ---
    build_wxdragon_wrapper(dest_bin_dir, &target, &wxwidgets_dir, &target_os, &target_env)
        .expect("Failed to build wxDragon wrapper library");
    Ok(())
}

fn build_wxdragon_wrapper(
    dest_bin_dir: &std::path::Path,
    target: &str,
    wxwidgets_source_path: &std::path::Path,
    target_os: &str,
    target_env: &str,
) -> std::io::Result<()> {
    // --- 3. Configure and Build libwxdragon (and wxWidgets) using CMake ---
    let libwxdragon_cmake_source_dir = std::path::PathBuf::from("cpp");

    let wxdragon_sys_build_dir = dest_bin_dir.join("wxdragon_sys_cmake_build");
    let wxwidgets_build_dir = dest_bin_dir.join("wxwidgets_cmake_build");

    let mut cmake_config = cmake::Config::new(libwxdragon_cmake_source_dir);
    cmake_config.out_dir(&wxdragon_sys_build_dir);
    cmake_config.define("WXWIDGETS_LIB_DIR", wxwidgets_source_path);
    cmake_config.define("WXWIDGETS_BUILD_DIR", &wxwidgets_build_dir);

    // Handle CMAKE_TLS_VERIFY for SSL certificate verification during downloads
    // On Windows with webview feature, we need to download WebView2 SDK from NuGet
    // Some environments have SSL certificate issues, so we automatically disable verification
    // Users can override by setting CMAKE_TLS_VERIFY environment variable explicitly
    let tls_verify = if let Ok(val) = std::env::var("CMAKE_TLS_VERIFY") {
        // User explicitly set it, use their value
        val
    } else if target_os == "windows" && cfg!(feature = "webview") {
        // Automatically disable for Windows webview builds to avoid SSL issues with WebView2 SDK download
        "0".to_string()
    } else {
        // Default: let CMake use its default behavior (verification enabled)
        String::new()
    };

    if !tls_verify.is_empty() {
        cmake_config.env("CMAKE_TLS_VERIFY", &tls_verify);
        cmake_config.define("CMAKE_TLS_VERIFY", &tls_verify);
    }

    // Disable WebP support since we'll use the image crate for image decoding
    cmake_config.define("wxUSE_LIBWEBP", "OFF");

    cmake_config
        .define("wxdUSE_AUI", if cfg!(feature = "aui") { "1" } else { "0" })
        .define("wxdUSE_MEDIACTRL", if cfg!(feature = "media-ctrl") { "1" } else { "0" })
        .define("wxdUSE_WEBVIEW", if cfg!(feature = "webview") { "1" } else { "0" });
    cmake_config.define("wxUSE_WEBVIEW", if cfg!(feature = "webview") { "ON" } else { "OFF" });
    if cfg!(feature = "webview") {
        if target_os == "macos" {
            cmake_config.define("wxUSE_WEBVIEW_WEBKIT", "ON");
        } else if target_os == "windows" {
            cmake_config.define("wxUSE_WEBVIEW_EDGE", "ON");
        } else if target_os == "linux" {
            cmake_config.define("wxUSE_WEBVIEW_WEBKIT", "ON");
        }
    }
    cmake_config
        .define("wxdUSE_STC", if cfg!(feature = "stc") { "1" } else { "0" })
        .define("wxdUSE_XRC", if cfg!(feature = "xrc") { "1" } else { "0" })
        .define("wxdUSE_RICHTEXT", if cfg!(feature = "richtext") { "1" } else { "0" });

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    let mut is_debug = profile == "debug";

    let mut toolchain_file: Option<String> = None;

    // Try to detect zigbuild toolchain file from env
    let env_toolchain_vars = [
        format!("CMAKE_TOOLCHAIN_FILE_{target}"),
        format!("CMAKE_TOOLCHAIN_FILE_{}", target.replace('-', "_")),
        format!("CMAKE_TOOLCHAIN_FILE_{}", target.replace('-', "")),
        "CMAKE_TOOLCHAIN_FILE".to_string(),
    ];
    for var in &env_toolchain_vars {
        if let Ok(val) = std::env::var(var)
            && !val.is_empty()
        {
            toolchain_file = Some(val);
            break;
        }
    }

    let is_zigbuild = toolchain_file.as_ref().map(|s| s.contains("zigbuild")).unwrap_or_default();

    // Prefer Ninja if available, fallback to Unix Makefiles
    let ninja_available = std::process::Command::new("ninja")
        .arg("--version")
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false);

    // --- PATCH: Detect cross-compiling from Linux to Windows (zigbuild/cargo-zigbuild zig) ---
    let host_os = std::env::consts::OS;
    let is_cross_linux_to_windows = host_os == "linux" && target_os == "windows" && target_env == "gnu" && is_zigbuild;
    if is_cross_linux_to_windows {
        // Command line: cargo zigbuild --target x86_64-pc-windows-gnu

        cmake_config.cxxflag("-Wno-error=date-time");
        cmake_config.cflag("-Wno-error=date-time");

        // Use zigbuild toolchain file if present
        if let Some(file) = &toolchain_file {
            cmake_config.define("CMAKE_TOOLCHAIN_FILE", file);
        } else {
            // Set CMake compilers to zig if zig is used (cargo-zigbuild)
            if let Ok(zig_cc) = std::env::var("CC") {
                cmake_config.env("CC", zig_cc);
            }
            if let Ok(zig_cxx) = std::env::var("CXX") {
                cmake_config.env("CXX", zig_cxx);
            }
        }

        cmake_config.generator(if ninja_available { "Ninja" } else { "Unix Makefiles" });
    } else if target_os == "windows" {
        if target_env == "gnu" {
            // Potentially set MinGW toolchain for CMake if not automatically detected
            let host_os = std::env::consts::OS;
            let (generator, cc, cxx) = if host_os == "macos" {
                // On macOS, use Unix Makefiles and MinGW cross-compiler for cross-compilation to Windows
                ("Unix Makefiles", "x86_64-w64-mingw32-gcc", "x86_64-w64-mingw32-g++")
            } else {
                // On Windows, use MinGW Makefiles and native compilers
                ("MinGW Makefiles", "gcc", "g++")
            };

            cmake_config
                .generator(generator)
                .define("--config", &profile)
                .env("CXX", cxx)
                .env("CC", cc)
                .define("CMAKE_CXX_COMPILER", cxx)
                .define("CMAKE_C_COMPILER", cc);
        } else if target_env == "msvc" {
            // Rust MSVC toolchain links against release CRT (msvcrt) even in debug builds.
            // To avoid CRT mismatches (e.g., unresolved __imp__CrtDbgReport), we build
            // the C++ side (wxWidgets and wrapper) with the Release CRT and link against
            // non-"d" suffixed libs even when Rust profile is debug. We still prefer
            // RelWithDebInfo for symbols while keeping Release CRT.
            is_debug = false;

            let rt_lib = if is_debug {
                "MultiThreadedDebugDLL"
            } else {
                "MultiThreadedDLL"
            };

            let build_type = if is_debug { "Debug" } else { "RelWithDebInfo" };
            cmake_config
                .generator("Ninja")
                .define("CMAKE_BUILD_TYPE", build_type)
                .define("CMAKE_MSVC_RUNTIME_LIBRARY", rt_lib)
                .define("CMAKE_POLICY_DEFAULT_CMP0091", "NEW")
                .cxxflag("/EHsc");
        } else {
            return Err(std::io::Error::other(format!(
                "Unsupported Windows target environment: {target_env}"
            )));
        }

        if target == "i686-pc-windows-msvc" {
            cmake_config
                .generator("Visual Studio 17 2022")
                .define("CMAKE_GENERATOR_PLATFORM", "Win32")
                .define("--config", &profile)
                .cxxflag("/EHsc");
        }
    }

    if target_env != "msvc" {
        // Set CMake build type based on Rust profile
        cmake_config.define("CMAKE_BUILD_TYPE", if is_debug { "Debug" } else { "Release" });
    }

    let dst = cmake_config.build();
    let build_dir = dst.join("build");
    let lib_search_path = build_dir.join("lib").display().to_string();

    println!("info: CMake build completed. Build directory: {build_dir:?}");
    println!("info: libwxdragon should be in: {lib_search_path:?}");
    println!("info: wxDragon-sys build directory: {wxdragon_sys_build_dir:?}");

    // --- 4. Linker Instructions ---
    println!("cargo:rustc-link-search=native={lib_search_path}");

    let wx_lib = wxdragon_sys_build_dir.join("lib").display().to_string();
    println!("cargo:rustc-link-search=native={wx_lib}");

    // For Windows, wxWidgets libs might be in a subdirectory like gcc_x64_lib for MinGW
    if target_os == "windows" {
        if target_env == "gnu" {
            let wx_lib2 = wxdragon_sys_build_dir.join("lib/gcc_x64_lib").display().to_string();
            println!("cargo:rustc-link-search=native={wx_lib2}");

            // --- Dynamically find MinGW GCC library paths ---
            let host_os = std::env::consts::OS;
            let gcc_path = if host_os == "macos" || host_os == "linux" {
                // On macOS and Linux, use the cross-compiler
                "x86_64-w64-mingw32-gcc"
            } else {
                // On Windows, use the native compiler
                "gcc"
            };

            // Find the path containing libgcc.a
            let output_libgcc = std::process::Command::new(gcc_path)
                .arg("-print-libgcc-file-name")
                .output()
                .unwrap_or_else(|_| panic!("Failed to execute {gcc_path} -print-libgcc-file-name"));

            if output_libgcc.status.success() {
                let libgcc_path_str = String::from_utf8_lossy(&output_libgcc.stdout).trim().to_string();
                if !libgcc_path_str.is_empty() {
                    let libgcc_path = std::path::Path::new(&libgcc_path_str);
                    if let Some(libgcc_dir) = libgcc_path.parent() {
                        let libgcc_dir_str = libgcc_dir.display().to_string();

                        println!("cargo:rustc-link-search=native={}", libgcc_dir_str);
                        println!("info: Added GCC library search path (from libgcc): {}", libgcc_dir_str);

                        // Attempt to find the path containing libstdc++.a (often one level up, in `../<target>/lib`)
                        if let Some(gcc_dir) = libgcc_dir.parent() {
                            // e.g., .../gcc/x86_64-w64-mingw32/15.1.0 -> .../gcc/x86_64-w64-mingw32
                            if let Some(toolchain_lib_dir) = gcc_dir.parent() {
                                // e.g., .../gcc/x86_64-w64-mingw32 -> .../gcc
                                if let Some(base_lib_dir) = toolchain_lib_dir.parent() {
                                    // e.g., .../gcc -> .../lib
                                    // Construct the expected path for libstdc++.a based on `find` result structure

                                    // ../../x86_64-w64-mingw32/lib
                                    let libstdcpp_dir = base_lib_dir.parent().unwrap().join("x86_64-w64-mingw32/lib");
                                    let v = libstdcpp_dir.display();
                                    if libstdcpp_dir.exists() && libstdcpp_dir != libgcc_dir {
                                        println!("cargo:rustc-link-search=native={v}");
                                        println!("info: Add GCC lib search path(for libstdc++):{v}");
                                    } else {
                                        println!(
                                            "info: Could not find or verify expected libstdc++ path relative to libgcc path: {v}"
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                        println!("cargo:warning=Could not get parent directory from libgcc path: {libgcc_path_str}");
                    }
                } else {
                    println!("cargo:warning=Command -print-libgcc-file-name returned empty output.");
                }
            } else {
                let stderr = String::from_utf8_lossy(&output_libgcc.stderr);
                println!("cargo:warning=Failed to run '{gcc_path} -print-libgcc-file-name': {stderr}");
                println!(
                    "cargo:warning=Static linking for stdc++/gcc might fail. Falling back to hoping they are in default paths."
                );
            }
            // --- End dynamic path finding ---
        } else {
            let lib_dir = if target == "i686-pc-windows-msvc" {
                "lib/vc_lib"
            } else {
                "lib/vc_x64_lib"
            };
            let wx_lib2 = wxdragon_sys_build_dir.join(lib_dir).display().to_string();
            println!("cargo:rustc-link-search=native={wx_lib2}");

            if target == "i686-pc-windows-msvc" {
                // build/lib/Debug or build/lib/Release
                let sub_dir = format!("build/lib/{profile}");
                let wx_lib3 = wxdragon_sys_build_dir.join(sub_dir).display().to_string();
                println!("cargo:rustc-link-search=native={wx_lib3}");
            }

            // Add WebView2 SDK library path for MSVC builds
            // wxWidgets downloads WebView2 NuGet package during CMake configuration
            // The libraries are typically in build/packages/Microsoft.Web.WebView2.*/build/native/
            if cfg!(feature = "webview") {
                let webview2_arch = if target == "i686-pc-windows-msvc" { "x86" } else { "x64" };

                // wxWidgets downloads WebView2 to CMAKE_CURRENT_BINARY_DIR/packages in build/cmake/lib/webview/CMakeLists.txt
                // The wxWidgets CMake uses add_subdirectory(build/cmake/lib libs), so the binary dir is "libs" (plural)
                // Then webview is added via add_subdirectory(${LIB}) which creates "libs/webview/"
                // So the actual path is: wxwidgets_build_dir/libs/webview/packages/Microsoft.Web.WebView2.*/
                let webview2_search_paths = [
                    "libs/webview/packages", // wxWidgets webview CMake build location (note: "libs" plural)
                    "lib/webview/packages",  // Alternative path in case CMake structure changes
                    "build/packages",
                    "packages",
                ];

                // Search in both wxdragon_sys build dir and wxwidgets build dir
                let search_dirs = [&wxdragon_sys_build_dir, &wxwidgets_build_dir];

                let mut found_webview2 = false;
                'outer: for search_dir in &search_dirs {
                    for search_base in &webview2_search_paths {
                        let packages_dir = search_dir.join(search_base);
                        if packages_dir.exists()
                            && let Ok(entries) = std::fs::read_dir(&packages_dir)
                        {
                            for entry in entries.flatten() {
                                let entry_name = entry.file_name();
                                let entry_name_str = entry_name.to_string_lossy();
                                if entry_name_str.starts_with("Microsoft.Web.WebView2") {
                                    // Add architecture-specific library path for WebView2LoaderStatic.lib
                                    let webview2_lib_path = entry.path().join(format!("build/native/{}", webview2_arch));
                                    if webview2_lib_path.exists() {
                                        println!("cargo:rustc-link-search=native={}", webview2_lib_path.display());
                                        // Explicitly link WebView2LoaderStatic - the #pragma comment in wxWidgets
                                        // only works during MSVC compilation, not during Rust's final link stage
                                        println!("cargo:rustc-link-lib=static=WebView2LoaderStatic");
                                        println!(
                                            "info: Added WebView2 {} library path: {}",
                                            webview2_arch,
                                            webview2_lib_path.display()
                                        );
                                        found_webview2 = true;
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                }

                if !found_webview2 {
                    println!(
                        "cargo:warning=WebView2 SDK not found. Searched in: {:?} and {:?}",
                        wxdragon_sys_build_dir, wxwidgets_build_dir
                    );
                    println!("cargo:warning=WebView feature is enabled but WebView2LoaderStatic.lib may not be found.");
                    println!("cargo:warning=If linking fails, ensure wxWidgets downloaded the WebView2 NuGet package.");
                }
            }
        }
    }

    println!("cargo:rustc-link-lib=static=wxdragon");

    if target_os == "macos" {
        // macOS linking flags (assuming release build for wxWidgets library names here)
        // If macOS also has d suffix for debug, this section would need similar conditional logic
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_core-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_adv-3.3");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_gl-3.3");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_propgrid-3.3");

        // Conditional features for macOS
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
        if cfg!(feature = "richtext") {
            println!("cargo:rustc-link-lib=static=wx_osx_cocoau_html-3.3");
            println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
            println!("cargo:rustc-link-lib=static=wx_osx_cocoau_richtext-3.3");
        }

        println!("cargo:rustc-link-lib=static=wxjpeg-3.3");
        println!("cargo:rustc-link-lib=static=wxpng-3.3");
        println!("cargo:rustc-link-lib=static=wxtiff-3.3");
        println!("cargo:rustc-link-lib=static=wxregexu-3.3");
        println!("cargo:rustc-link-lib=expat");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=c++");

        // Conditional STC support libraries for macOS
        if cfg!(feature = "stc") {
            println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
            println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
        }

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

        if cfg!(feature = "webview") {
            println!("cargo:rustc-link-lib=framework=WebKit");
        }

        fix_isPlatformVersionAtLeast()?;
    } else if target_os == "windows" {
        // Detect cross-compilation from macOS to Windows
        let host_os = std::env::consts::OS;
        let is_macos_to_windows_gnu = host_os == "macos" && target_os == "windows" && target_env == "gnu";

        if is_macos_to_windows_gnu || is_cross_linux_to_windows {
            // Cross-compilation from macOS or Linux: libraries have -Windows suffix
            println!("cargo:rustc-link-lib=static=wx_mswu_core-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_mswu_adv-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_baseu-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_mswu_gl-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_mswu_propgrid-3.3-Windows");

            // Conditional features for cross-compilation
            if cfg!(feature = "aui") {
                println!("cargo:rustc-link-lib=static=wx_mswu_aui-3.3-Windows");
            }
            if cfg!(feature = "media-ctrl") {
                println!("cargo:rustc-link-lib=static=wx_mswu_media-3.3-Windows");
            }
            if cfg!(feature = "webview") {
                println!("cargo:rustc-link-lib=static=wx_mswu_webview-3.3-Windows");
            }
            if cfg!(feature = "xrc") || cfg!(feature = "webview") {
                println!("cargo:rustc-link-lib=static=wx_mswu_html-3.3-Windows");
            }
            if cfg!(feature = "stc") {
                println!("cargo:rustc-link-lib=static=wx_mswu_stc-3.3-Windows");
                println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
                println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
            }
            if cfg!(feature = "xrc") {
                println!("cargo:rustc-link-lib=static=wx_mswu_xrc-3.3-Windows");
                println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3-Windows");
            }
            if cfg!(feature = "richtext") {
                println!("cargo:rustc-link-lib=static=wx_mswu_html-3.3-Windows");
                println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3-Windows");
                println!("cargo:rustc-link-lib=static=wx_mswu_richtext-3.3-Windows");
            }

            println!("cargo:rustc-link-lib=static=wxpng-3.3");
            println!("cargo:rustc-link-lib=static=wxtiff-3.3");
            println!("cargo:rustc-link-lib=static=wxjpeg-3.3");
            println!("cargo:rustc-link-lib=static=wxregexu-3.3");
            println!("cargo:rustc-link-lib=static=wxzlib-3.3");
            println!("cargo:rustc-link-lib=static=wxexpat-3.3");

            if is_macos_to_windows_gnu {
                println!("info: Using static linking for cross-compilation from macOS to Windows GNU");
                // Static linking for cross-compilation to avoid runtime dependencies
                println!("cargo:rustc-link-lib=static=stdc++");
                println!("cargo:rustc-link-lib=static=gcc");
                println!("cargo:rustc-link-lib=static=gcc_eh");
                println!("cargo:rustc-link-lib=static=pthread");
                // Add linker arguments for fully static C++ runtime
                println!("cargo:rustc-link-arg=-static-libgcc");
                println!("cargo:rustc-link-arg=-static-libstdc++");
            }
            // Note: For webview feature with MinGW, wxWidgets uses dynamic loading of WebView2Loader.dll
            // at runtime (wxUSE_WEBVIEW_EDGE_STATIC=OFF in CMakeLists.txt), so no compile-time linking needed.
            // The WebView2Loader.dll must be present on the target system or alongside the executable.
        } else {
            let debug_suffix = if is_debug { "d" } else { "" };

            println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_adv");
            println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_core");
            println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_gl");
            println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_propgrid");

            if cfg!(feature = "aui") {
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_aui");
            }
            if cfg!(feature = "media-ctrl") {
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_media");
            }
            if cfg!(feature = "webview") {
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_webview");
            }
            if cfg!(feature = "xrc") || cfg!(feature = "webview") {
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_html");
            }
            if cfg!(feature = "stc") {
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_stc");
                println!("cargo:rustc-link-lib=static=wxscintilla{debug_suffix}");
                println!("cargo:rustc-link-lib=static=wxlexilla{debug_suffix}");
            }
            if cfg!(feature = "xrc") {
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_xrc");
                println!("cargo:rustc-link-lib=static=wxbase33u{debug_suffix}_xml");
            }
            if cfg!(feature = "richtext") {
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_html");
                println!("cargo:rustc-link-lib=static=wxbase33u{debug_suffix}_xml");
                println!("cargo:rustc-link-lib=static=wxmsw33u{debug_suffix}_richtext");
            }

            println!("cargo:rustc-link-lib=static=wxbase33u{debug_suffix}");
            println!("cargo:rustc-link-lib=static=wxtiff{debug_suffix}");
            println!("cargo:rustc-link-lib=static=wxjpeg{debug_suffix}");
            println!("cargo:rustc-link-lib=static=wxpng{debug_suffix}");
            println!("cargo:rustc-link-lib=static=wxregexu{debug_suffix}");
            println!("cargo:rustc-link-lib=static=wxzlib{debug_suffix}");
            println!("cargo:rustc-link-lib=static=wxexpat{debug_suffix}");

            if target_env == "gnu" {
                println!("cargo:rustc-link-lib=stdc++");
            }
        }

        // System libraries (same for debug and release)
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=gdiplus"); // Add GDI+ library for graphics support
        println!("cargo:rustc-link-lib=msimg32"); // Add for AlphaBlend and GradientFill functions
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
        println!("cargo:rustc-link-lib=imm32"); // Add IME library for Scintilla support
    } else {
        // For Linux and other Unix-like systems
        println!("cargo:rustc-link-lib=xkbcommon");
        let lib = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
        for _lib in lib.libs {
            println!("cargo:rustc-link-lib={_lib}");
        }
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=png");
        println!("cargo:rustc-link-lib=jpeg");
        println!("cargo:rustc-link-lib=expat");
        println!("cargo:rustc-link-lib=tiff");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_propgrid-3.3");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_gl-3.3");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_adv-3.3");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_core-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
        println!("cargo:rustc-link-lib=stdc++");

        if cfg!(feature = "aui") {
            println!("cargo:rustc-link-lib=static=wx_gtk3u_aui-3.3");
        }
        if cfg!(feature = "webview") {
            println!("cargo:rustc-link-lib=static=wx_gtk3u_webview-3.3");

            // Link WebKitGTK for Linux WebView support
            // Try webkit2gtk-4.1 first (newer), fall back to webkit2gtk-4.0 if not available
            if let Ok(webkit) = pkg_config::Config::new().probe("webkit2gtk-4.1") {
                for lib in webkit.libs {
                    println!("cargo:rustc-link-lib={lib}");
                }
                println!("info: Using webkit2gtk-4.1 for WebView support");
            } else if let Ok(webkit) = pkg_config::Config::new().probe("webkit2gtk-4.0") {
                for lib in webkit.libs {
                    println!("cargo:rustc-link-lib={lib}");
                }
                println!("info: Using webkit2gtk-4.0 for WebView support");
            } else {
                println!("cargo:warning=WebKitGTK not found. WebView feature may not work on Linux.");
                println!("cargo:warning=Install webkit2gtk-4.1 or webkit2gtk-4.0 development packages:");
                println!("cargo:warning=  Ubuntu/Debian: sudo apt install libwebkit2gtk-4.1-dev");
                println!("cargo:warning=  or: sudo apt install libwebkit2gtk-4.0-dev");
                println!("cargo:warning=  Fedora: sudo dnf install webkit2gtk4.1-devel");
                println!("cargo:warning=  or: sudo dnf install webkit2gtk4.0-devel");
                println!("cargo:warning=  Arch: sudo pacman -S webkit2gtk-4.1");
                println!("cargo:warning=  or: sudo pacman -S webkit2gtk");
            }
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
        if cfg!(feature = "richtext") {
            println!("cargo:rustc-link-lib=static=wx_gtk3u_html-3.3");
            println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
            println!("cargo:rustc-link-lib=static=wx_gtk3u_richtext-3.3");
        }
    }

    Ok(())
}

#[allow(non_snake_case)]
fn fix_isPlatformVersionAtLeast() -> std::io::Result<()> {
    use std::io::{Error, ErrorKind::NotFound};
    // Fix for ___isPlatformVersionAtLeast undefined symbol on macOS
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "macos" {
        return Ok(());
    }
    // Use xcrun to find the toolchain path
    use std::process::Command;
    let output = Command::new("xcrun").args(["--find", "clang"]).output()?;
    if !output.status.success() {
        return Err(Error::other("xcrun failed to find clang"));
    }
    let clang_path_str = String::from_utf8_lossy(&output.stdout);
    let clang_path = clang_path_str.trim();

    // Construct the clang runtime library path from the clang path
    // /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang
    // -> /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang
    let clang_dir = std::path::Path::new(clang_path)
        .parent()
        .ok_or_else(|| Error::new(NotFound, "Failed to get clang parent directory"))?;
    let usr_dir = clang_dir
        .parent()
        .ok_or_else(|| Error::new(NotFound, "Failed to get clang usr directory"))?;
    let clang_rt_path = usr_dir.join("lib").join("clang");

    // Try to find the clang runtime library
    let entries = std::fs::read_dir(&clang_rt_path)?;
    for entry in entries.flatten() {
        if !entry.file_type().is_ok_and(|ft| ft.is_dir()) {
            continue;
        }
        let version_dir = entry.path();
        let lib_dir = version_dir.join("lib").join("darwin");
        let clang_rt_lib = lib_dir.join("libclang_rt.osx.a");

        if clang_rt_lib.exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
            println!("cargo:rustc-link-lib=static=clang_rt.osx");
            println!("info: Added clang runtime library for macOS arm64: {clang_rt_lib:?}");
            return Ok(());
        }
    }

    Err(Error::new(NotFound, "Could not find clang runtime library"))
}

/// Try to read the proxy URL via `git config --get http.proxy`.
fn get_git_http_proxy() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["config", "--get", "http.proxy"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() { None } else { Some(stdout) }
}

/// Download a ZIP file from `url` to `dest_path`, using ~/.gitconfig [http].proxy if present.
/// Falls back to direct connection if no proxy is configured.
pub fn download_file_with_git_http_proxy<P: AsRef<std::path::Path>>(
    url: &str,
    dest_path: P,
    expected_sha: &str,
) -> std::io::Result<()> {
    use std::io::Error;

    // If the file already exists, verify its checksum first.
    let path = dest_path.as_ref();
    if path.exists() {
        match verify_downloaded_file_sha256(path, expected_sha) {
            Ok(_) => return Ok(()),
            Err(_) => std::fs::remove_file(path).ok(),
        };
    }

    // Build reqwest blocking client, optionally with proxy.
    let client = match get_git_http_proxy() {
        Some(proxy_url) => reqwest::blocking::Client::builder()
            .proxy(reqwest::Proxy::all(&proxy_url).map_err(Error::other)?)
            .build()
            .map_err(Error::other)?,
        None => reqwest::blocking::Client::new(),
    };

    // Perform GET request
    let mut resp = client.get(url).send().map_err(Error::other)?;
    if !resp.status().is_success() {
        return Err(Error::other(format!("HTTP error: {}", resp.status())));
    }

    // Stream to file to avoid loading the entire ZIP into memory.
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(path)?;
    resp.copy_to(&mut file).map_err(Error::other)?;

    // Verify SHA256 of the downloaded file before proceeding
    verify_downloaded_file_sha256(path, expected_sha)?;
    Ok(())
}

fn verify_downloaded_file_sha256<P: AsRef<std::path::Path>>(path: P, expected_sha: &str) -> std::io::Result<()> {
    let path = path.as_ref();
    let computed_sha = compute_file_sha256_hex(path)?;
    if !computed_sha.eq_ignore_ascii_case(expected_sha) {
        return Err(std::io::Error::other(format!(
            "SHA256 mismatch for file {path:?}: got {computed_sha}, expected {expected_sha}"
        )));
    }
    Ok(())
}

/// Compute the SHA256 of a file and return lowercase hex string.
fn compute_file_sha256_hex<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
    use sha2::{Digest, Sha256};
    use std::io::{Error, Read};

    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let digest = hasher.finalize();
    // Convert to lowercase hex without extra dependencies
    let mut s = String::with_capacity(digest.len() * 2);
    for b in digest {
        use std::fmt::Write as _;
        write!(&mut s, "{:02x}", b).map_err(Error::other)?;
    }
    Ok(s)
}

fn extract_zip_archive<P, T>(archive_path: P, target_dir: T) -> std::io::Result<()>
where
    P: AsRef<std::path::Path>,
    T: AsRef<std::path::Path>,
{
    use rawzip::{CompressionMethod, RECOMMENDED_BUFFER_SIZE, ZipArchive};
    use std::io::{Error, ErrorKind::InvalidData};

    let file = std::fs::File::open(archive_path)?;
    let mut buffer = vec![0_u8; RECOMMENDED_BUFFER_SIZE];
    let archive = ZipArchive::from_file(file, &mut buffer)
        .map_err(|e| Error::new(InvalidData, format!("Failed to read ZIP archive: {e}")))?;

    let mut entries = archive.entries(&mut buffer);
    while let Some(entry) = entries
        .next_entry()
        .map_err(|e| Error::new(InvalidData, format!("Failed to read entry: {e}")))?
    {
        let file_path = entry.file_path();
        let file_path = match file_path.try_normalize() {
            Ok(p) => p,
            Err(e) => {
                println!("cargo:warning=Skipping invalid file path {file_path:?} in ZIP: {e}");
                continue;
            }
        };
        let out_path = target_dir.as_ref().join(file_path.as_ref());

        if entry.is_dir() {
            std::fs::create_dir_all(&out_path)?;
            continue;
        }

        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let zip_entry = archive
            .get_entry(entry.wayfinder())
            .map_err(|e| Error::new(InvalidData, format!("Failed to get entry: {e}")))?;
        let reader = zip_entry.reader();

        let mut outfile = std::fs::File::create(&out_path)?;
        let method = entry.compression_method();
        match method {
            CompressionMethod::Store => {
                let mut verifier = zip_entry.verifying_reader(reader);
                std::io::copy(&mut verifier, &mut outfile)?;
            }
            CompressionMethod::Deflate => {
                let inflater = flate2::read::DeflateDecoder::new(reader);
                let mut verifier = zip_entry.verifying_reader(inflater);
                std::io::copy(&mut verifier, &mut outfile)?;
            }
            _ => {
                println!("cargo:warning=Unsupported compression method {method:?} for file: {file_path:?}");
            }
        }
    }

    Ok(())
}

fn chk_wx_version<P: AsRef<std::path::Path>>(wxwidgets_dir: P, expected_version: &str) -> std::io::Result<bool> {
    use std::io::{BufRead, BufReader};
    let cfg = wxwidgets_dir.as_ref().join("configure");

    let file = std::fs::File::open(cfg)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(ver) = line.strip_prefix("PACKAGE_VERSION='")
            && let Some(end) = ver.find('\'')
        {
            let found_version = &ver[..end];
            let matched = found_version == expected_version;
            return Ok(matched);
        }
    }
    Ok(false)
}
