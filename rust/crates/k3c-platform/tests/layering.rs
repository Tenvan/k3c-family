//! `winit`, `wgpu` und `gilrs` dürfen nur in `src/backend/` vorkommen (RU2.1, RU4.4).

use std::fs;
use std::path::Path;

fn sources(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for e in fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

#[test]
fn kein_winit_wgpu_oder_gilrs_ausserhalb_von_backend() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    sources(&root.join("src"), &mut files);
    let backend = root.join("src").join("backend");
    let mut checked = 0;
    for f in files.iter().filter(|f| !f.starts_with(&backend)) {
        checked += 1;
        let text = fs::read_to_string(f).unwrap();
        for pat in [
            "winit::",
            "wgpu::",
            "use winit",
            "use wgpu",
            "extern crate winit",
            "extern crate wgpu",
            "gilrs::",
            "use gilrs",
        ] {
            assert!(
                !text.contains(pat),
                "{} enthält `{pat}` außerhalb von backend/",
                f.display()
            );
        }
    }
    assert!(checked >= 6, "zu wenige Quelldateien gefunden: {checked}");
}
