fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("TARGET").unwrap_or_default().contains("windows") {
        use embed_manifest::manifest::{ActiveCodePage, SupportedOS::*};
        let manifest = embed_manifest::new_manifest("accessibility_demo")
            .supported_os(Windows7..=Windows10)
            .active_code_page(ActiveCodePage::Utf8)
            .dpi_awareness(embed_manifest::manifest::DpiAwareness::PerMonitorV2);
        embed_manifest::embed_manifest(manifest).expect("could not embed Windows manifest");
    }
}
