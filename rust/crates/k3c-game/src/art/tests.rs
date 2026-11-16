use std::path::{Path, PathBuf};

use super::*;

fn fixture() -> (PathBuf, PathBuf) {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/art");
    (base.join("grafik"), base.join("paper.json"))
}

#[test]
fn fixture_asset_laedt() {
    let (g, p) = fixture();
    let lib = Library::open(&g, &p).unwrap();
    assert_eq!(lib.assets().unwrap(), vec!["figuren/test".to_string()]);
    let a = lib.load("figuren/test@spieler2").unwrap();
    assert_eq!(a.sheets.len(), 1);
    assert_eq!((a.sheets[0].width, a.sheets[0].height), (8, 4));
    assert_eq!(a.sheets[0].rgba.len(), 8 * 4 * 4);
    let idle = &a.animationen["idle"];
    assert_eq!((idle.start, idle.frames), (0, 2));
    assert_eq!(
        a.frames[1],
        Frame {
            sheet: 0,
            rect: [4, 0, 4, 4],
            anker: [0.5, 1.0]
        }
    );
    assert_eq!(a.sheets[0].rgba[0..4], [0, 0, 100, 255]);
}

#[test]
fn gruppen_index_nennt_assets() {
    let (g, _) = fixture();
    assert_eq!(group_assets(&g, "figuren").unwrap(), vec!["figuren/test".to_string()]);
}

#[test]
fn fehlender_ordner_nennt_gen_paper() {
    let (_, p) = fixture();
    let e = Library::open(Path::new("gibt/es/nicht"), &p).err().unwrap();
    assert!(e.to_string().contains("task gen:paper"));
    let (g, _) = fixture();
    let e = group_assets(&g, "leer").unwrap_err();
    assert!(e.to_string().contains("task gen:paper"));
}

#[test]
fn fehlendes_asset_ist_fehler() {
    let (g, p) = fixture();
    let lib = Library::open(&g, &p).unwrap();
    assert!(matches!(lib.load("figuren/nope"), Err(ArtError::Missing(_))));
}

#[test]
fn echte_grafik_laedt_alle_assets() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let g = std::env::var_os("K3C_GRAFIK_DIR").map_or_else(|| root.join("public/grafik"), PathBuf::from);
    let p = root.join("data/paper.json");
    if !g.join("figuren/monarch/manifest.json").is_file() {
        eprintln!("übersprungen: keine gerenderte Grafik, erst `task gen:paper`");
        return;
    }
    let lib = Library::open(&g, &p).unwrap();
    let names = lib.assets().unwrap();
    for n in &names {
        lib.load(n).unwrap_or_else(|e| panic!("{n}: {e}"));
    }
    eprintln!("{} Assets aus data/paper.json geladen", names.len());
    assert!(!names.is_empty());
}
