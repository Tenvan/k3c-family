fn main() {
    // Testbinaries bekommen kein App-Manifest von tauri-build; ohne Common-Controls v6 brechen sie unter Windows
    // mit STATUS_ENTRYPOINT_NOT_FOUND ab (TaskDialogIndirect). Darum das Manifest nur für Tests einbetten.
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test-manifest.xml");
    println!("cargo:rerun-if-changed=test-manifest.xml");
    println!("cargo:rustc-link-arg-tests=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-tests=/MANIFESTINPUT:{}", manifest.display());
    tauri_build::build();
}
