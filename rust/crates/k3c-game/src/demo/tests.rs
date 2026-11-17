//! Tests der Demo mit der Fixture-Grafik (`tests/fixtures/art`), ohne Fenster.

use std::path::{Path, PathBuf};

use k3c_platform::{App, Frame, InputScript, run_headless};

use super::driver::Driver;
use super::*;

fn fixture() -> (PathBuf, PathBuf) {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/art");
    (base.join("grafik"), base.join("paper.json"))
}

/// Library über die Fixture-Grafik, ohne Schrift (Pixel-Schrift als Rückfall).
fn fixture_lib() -> Library {
    let (g, _) = fixture();
    Library::new(crate::art::Config {
        grafik_dir: g,
        font_path: PathBuf::from("gibt/es/nicht.ttf"),
        ..crate::art::Config::default()
    })
    .unwrap()
}

const SCRIPT: &str = r##"{"laenge":60,"biom":"test","boden":"#336600",
    "ebenen":[{"asset":"figuren/test","parallaxe":0.5,"skala":4.0}],
    "gebaeude":[{"x":10,"asset":"figuren/test"}],
    "figuren":[{"asset":"figuren/test","anzahl":30,"tempo":2.5,"von":0,"bis":60,"anim":"idle"}],
    "kamera":{"figur":"figuren/test","anim":"idle","tempo":7.0}}"##;

fn demo(sprites: usize) -> Demo {
    let d: Driver = serde_json::from_str(SCRIPT).unwrap();
    Demo::new(d, Some(fixture_lib()), 7.0, sprites).unwrap()
}

#[test]
fn headless_sechshundert_schritte() {
    let mut d = demo(0);
    let n = run_headless(&mut d, 600, None);
    assert_eq!(d.world.tick, 600);
    assert_eq!(n.textures, 1, "ein Asset, ein Sheet, einmal hochgeladen");
    assert!(n.sprites > 0 && n.rects >= 600 * 2, "Himmel und Boden je Bild: {n:?}");
    assert_eq!(n.texts, 600);
}

#[test]
fn zusatzlast_zeichnet_n_sprites_je_bild() {
    let mut a = demo(0);
    let mut b = demo(1000);
    let (na, nb) = (run_headless(&mut a, 10, None), run_headless(&mut b, 10, None));
    assert_eq!(nb.sprites - na.sprites, 10 * 1000);
}

#[test]
fn nur_sichtbares_wird_gezeichnet() {
    let script = SCRIPT
        .replace("\"anzahl\":30", "\"anzahl\":300")
        .replace("\"bis\":60", "\"bis\":600")
        .replace("\"skala\":4.0", "\"skala\":600.0");
    let mut d = Demo::new(serde_json::from_str(&script).unwrap(), Some(fixture_lib()), 7.0, 0).unwrap();
    let mut f = Frame::new();
    f.begin();
    d.draw(&mut f, 0.0);
    f.finish();
    let sprites = f.instances().filter(|i| i.uv[2] != 0.0).count();
    assert!(
        (1..120).contains(&sprites),
        "300 Figuren auf 600 Units, nur ein Teil im Bild: {sprites}"
    );
}

#[test]
fn ebenen_kommen_nach_malreihenfolge() {
    let mut d = demo(0);
    let mut f = Frame::new();
    f.begin();
    d.draw(&mut f, 0.5);
    f.finish();
    let layers: Vec<u32> = f.runs().iter().map(|r| r.layer).collect();
    assert!(layers.windows(2).all(|w| w[0] <= w[1]), "{layers:?}");
    assert_eq!(layers.first(), Some(&0), "Himmel zuerst");
    assert_eq!(layers.last(), Some(&30), "Schrift zuletzt");
}

#[test]
fn fehlendes_asset_nennt_den_namen() {
    let d: Driver =
        serde_json::from_str(&SCRIPT.replace("\"figuren/test\",\"anim\"", "\"figuren/gibtsnicht\",\"anim\"")).unwrap();
    let err = Demo::new(d, Some(fixture_lib()), 7.0, 0).err().unwrap();
    assert!(err.contains("figuren/gibtsnicht"), "{err}");
}

#[test]
fn esc_beendet_und_f_schaltet_vollbild() {
    let mut d = demo(0);
    assert_eq!(d.fullscreen(), Some(false));
    let s = InputScript::from_json(r#"[{"step": 3, "keys": ["F"]}, {"step": 4}, {"step": 9, "keys": ["Escape"]}]"#)
        .unwrap();
    run_headless(&mut d, 600, Some(&s));
    assert!(d.done());
    assert_eq!(d.world.tick, 10, "Esc in Takt 9 beendet nach diesem Takt");
    assert_eq!(d.fullscreen(), Some(true));
}

#[test]
fn fixture_paper_nennt_assets() {
    let (g, p) = fixture();
    let paper: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap();
    assert_eq!(paper_assets(&g, &paper).unwrap(), vec!["figuren/test".to_string()]);
}

/// Die echte Grafik: alle Assets aus `data/paper.json` planen (Manifeste, PNG-Köpfe) über die art-API.
#[test]
fn echte_grafik_plant_alle_assets() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let g = std::env::var_os("K3C_GRAFIK_DIR").map_or_else(|| root.join("public/grafik"), PathBuf::from);
    if !g.join("figuren/monarch/manifest.json").is_file() {
        eprintln!("übersprungen: keine gerenderte Grafik, erst `task gen:paper`");
        return;
    }
    let paper: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("data/paper.json")).unwrap()).unwrap();
    let names = paper_assets(&g, &paper).unwrap();
    let plan = crate::art::plan_level(&g, "alle", &names).unwrap_or_else(|e| panic!("{e}"));
    eprintln!("{} Assets aus data/paper.json geplant: {}", names.len(), plan.budget);
    assert!(!names.is_empty());
}
