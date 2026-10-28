# Native accessibility regressions

These tests query the actual native accessibility name (MSAA on Windows, Cocoa on
macOS), check property/name setter ordering and custom accessible preservation,
and exercise keyboard-focus controls. On Windows they also check that each state
flag wxWidgets maps to an MSAA state reaches MSAA as itself when set on a window,
and that the states a custom accessible returns for two rows of a check list reach
MSAA as themselves. They run on the OS main thread.

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

## Printing (#246)

The printing regression checks that the printout shim starts and ends the
document through `wxPrintout`, which is what calls `StartDoc` and `EndDoc` on the
printer DC, and that the begin-document callback's answer is passed on. It uses
a DC that records those calls, so no printer is involved. It needs the CMake
configure step and the notes above:

```sh
cmake --build target/debug/wxdragon_sys_cmake_build/build --target printout_document_test --config Debug
ctest --test-dir target/debug/wxdragon_sys_cmake_build/build -C Debug -R '^printout_document$' --output-on-failure
cargo test -p wxdragon --lib printing::
```

The Rust tests check that `Printout::on_begin_document`'s answer reaches the
shim, and that a panic in it answers `false`.
