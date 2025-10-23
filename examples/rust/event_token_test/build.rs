fn main() {
    // On macOS, set DYLD_LIBRARY_PATH for running the example
    #[cfg(target_os = "macos")]
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_path = format!("{}/../../..", manifest_dir);
        println!(
            "cargo:rustc-env=DYLD_LIBRARY_PATH={}/rust/wxdragon-sys/cpp/build",
            lib_path
        );
    }
}
