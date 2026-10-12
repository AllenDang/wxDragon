# Native accessibility regressions

These tests query the actual native accessibility name (MSAA on Windows, Cocoa on
macOS), check property/name setter ordering and custom accessible preservation,
and exercise keyboard-focus controls. They run on the OS main thread.

From the repository root, after `cargo build -p wxdragon`:

```sh
cmake -S rust/wxdragon-sys/cpp -B target/debug/wxdragon_sys_cmake_build/build -DWXD_BUILD_TESTS=ON
cmake --build target/debug/wxdragon_sys_cmake_build/build --target window_accessibility_test --config Debug
ctest --test-dir target/debug/wxdragon_sys_cmake_build/build -C Debug -R '^window_accessibility$' --output-on-failure
```

The DataView regression for #160 also runs on the OS main thread. It checks the
default and opt-in container-column policies, shared-model refresh, label/state
updates, sorting, expansion, deletion and callback cleanup. On macOS it queries
the real `NSOutlineView` accessibility rows and their text/checkbox/disclosure
elements (not just model values):

```sh
cmake --build target/debug/wxdragon_sys_cmake_build/build --target dataview_container_columns_test --config Debug
ctest --test-dir target/debug/wxdragon_sys_cmake_build/build -C Debug -R '^dataview_container_columns$' --output-on-failure
cargo test -p wxdragon --test dataview_container_columns
```

The Rust test verifies the safe API through rendering callbacks, cloned model
handles, enabling/disabling after association, and release of model userdata.

This reuses Cargo's CMake cache and downloaded wxWidgets source. Adjust the build
path if using a custom `CARGO_TARGET_DIR` or `WXDRAGON_SYS_BUILD_DIR`.

A desktop session is required. On headless Linux, run the CTest command under
`xvfb-run -a`. The Windows name assertions require `wxUSE_ACCESSIBILITY`.

The Rust-side lifecycle regression runs with:

```sh
cargo test -p wxdragon --test window_accessibility
```

See `examples/rust/accessibility_demo/README.md` for screen-reader checks that
cannot be established by these automated tests.
