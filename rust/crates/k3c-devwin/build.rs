//! Bettet das Manifest (Common Controls 6, Per-Monitor-DPI v2) mit dem MSVC-Linker in `k3c-devwin.exe` ein;
//! ohne Manifest bekämen die Controls den alten Stil ohne Dark-Themes. Kein zusätzliches Crate nötig.

use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=k3c-devwin.manifest");
    println!("cargo:rerun-if-changed=build.rs");
    let windows = std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|v| v == "windows");
    let msvc = std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|v| v == "msvc");
    let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") else {
        return;
    };
    if windows && msvc {
        let manifest = PathBuf::from(dir).join("k3c-devwin.manifest");
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", manifest.display());
    }
}
