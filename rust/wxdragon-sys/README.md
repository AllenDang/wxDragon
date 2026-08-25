# wxdragon-sys

This crate provides raw FFI (Foreign Function Interface) bindings to `libwxdragon`, a C++ library that wraps parts of the [wxWidgets](https://www.wxwidgets.org/) cross-platform GUI toolkit. `libwxdragon` itself is built and statically linked by this crate during its build process, and it includes a vendored version of wxWidgets.

## Purpose

`wxdragon-sys` is the unsafe foundation upon which the safe Rust abstractions in the `wxdragon` crate are built. It is not typically used directly by end-users but rather as a dependency for higher-level crates aiming to use wxWidgets functionality from Rust.

## Features

*   Raw, `unsafe` bindings to the C API exposed by `libwxdragon`.
*   Build script that compiles and links `libwxdragon` and its vendored wxWidgets.
*   The `links = "wxdragon"` directive in `Cargo.toml` ensures proper linking and allows other crates to discover build metadata.

## Build Process

The build process (`build.rs`) for this crate performs the following key steps:

1.  **Downloads wxWidgets**: Fetches the wxWidgets 3.3.3 source archive.
2.  **Extracts wxWidgets**: Decompresses and extracts the sources.
3.  **Builds wxWidgets**: Compiles wxWidgets as a static library using CMake.
4.  **Builds libwxdragon**: Compiles the `wxdragon` C++ wrapper code (found in the `src` directory of the main project) which links against the just-built wxWidgets. This also becomes a static library.
5.  **Generates Bindings**: Uses `bindgen` to generate Rust FFI bindings from the `include/wxdragon.h` header file.

The directories involved can be moved through environment variables, for example to a location a CI cache preserves:

- `WXWIDGETS_DIR`: the wxWidgets source tree (default: `<target>/<profile>/wxWidgets`). An existing directory is used as-is; an absent or empty one receives the download.
- `WXWIDGETS_BUILD_DIR`: the wxWidgets CMake build tree (default: `<target>/<profile>/wxwidgets_cmake_build`).
- `WXDRAGON_SYS_BUILD_DIR`: the libwxdragon CMake build tree (default: `<target>/<profile>/wxdragon_sys_cmake_build`).

Changing any of them re-runs the build script.

## Usage

This crate is intended to be used as a dependency in your `Cargo.toml`:

```toml
[dependencies]
wxdragon-sys = "0.1.0" # Replace with the desired version from crates.io
```

Direct interaction with this crate involves `unsafe` calls to C functions. For a safer, more idiomatic Rust experience, consider using the `wxdragon` crate.

## Related Crates

*   [`wxdragon`](https://crates.io/crates/wxdragon): Safe, idiomatic Rust wrappers for wxWidgets, built on top of this `-sys` crate.

## License

This crate is licensed under the terms of the MIT license OR the Apache License 2.0 (see `LICENSE-MIT` and `LICENSE-APACHE` files in the workspace root).
The vendored wxWidgets library is distributed under its own [wxWindows Library Licence](https://www.wxwidgets.org/about/newlicen.htm). 