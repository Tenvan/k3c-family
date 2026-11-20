//! B-1147: Schiff-Gerüst und Überdeckung der Bauplätze im Hub (Größenstufen aus data/sizes.json).

use std::path::{Path, PathBuf};

use k3c_data::Source;
use k3c_data::look::{GO, K2C, with_profile};
use k3c_data::sizes::{self, Sizes};

use super::buildings::scaffold_look;

fn grafik() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik/gebaeude-paper")
}

fn table() -> Sizes {
    sizes::from_source(&Source::Embedded).unwrap()
}

#[test]
fn schiff_hat_eigenes_geruest_die_anderen_geruest_6() {
    with_profile(K2C, || {
        assert_eq!(scaffold_look("ship").asset, "gebaeude-paper/geruest-schiff");
        for k in [
            "workshop", "bowyer", "storage", "barracks", "tavern", "healer", "smithy", "armory",
        ] {
            assert_eq!(scaffold_look(k).asset, "gebaeude-paper/geruest-6", "{k}");
        }
    });
    with_profile(GO, || {
        assert_eq!(scaffold_look("ship").asset, "gebaeude-paper/geruest-6")
    });
}

#[test]
fn geruest_schiff_zwischen_2_5_und_3_2_m() {
    let t = table();
    let m = t
        .target_px("gebaeude-paper/geruest-schiff")
        .expect("geruest-schiff in sizes.json")
        / t.reference_px;
    assert!((2.5..=3.2).contains(&m), "{m} M");
}

/// Sichtbare Breite eines Assets im Spiel in px: Spalten mit Alpha > 16 des ersten Bildes mal Bild-px-Maß; mit Stufe
/// in der Tabelle gilt deren Maß, sonst der Faktor aus paper.json.
fn drawn_width(asset: &str, t: &Sizes) -> f64 {
    let dir = grafik().join(asset);
    let m: serde_json::Value = serde_json::from_slice(&std::fs::read(dir.join("manifest.json")).unwrap()).unwrap();
    let (fw, fh) = (
        m["frameW"].as_u64().unwrap() as usize,
        m["frameH"].as_u64().unwrap() as usize,
    );
    let anims = m["animationen"].as_object().unwrap();
    let name = if anims.contains_key("stand") {
        "stand"
    } else {
        anims.keys().next().unwrap()
    };
    let img = crate::art::read_png(&dir.join(anims[name]["datei"].as_str().unwrap())).unwrap();
    let w = img.width as usize;
    let cols = (0..fw)
        .filter(|&x| (0..fh).any(|y| img.pix[(y * w + x) * 4 + 3] > 16))
        .count() as f64;
    let upp = m["rahmen"][3].as_f64().unwrap() / fh as f64;
    let key = format!("gebaeude-paper/{asset}");
    let f = t.sheet_factor(&key, fh as i64, upp).unwrap_or(1.0);
    cols * upp * f
}

/// Platzart im Hub → Asset der fertigen Anlage (Treppen, Werkstätten).
const HUB_ASSET: [(&str, &str); 9] = [
    ("workshop", "werkstatt"),
    ("bowyer", "bogner"),
    ("healer", "heilplatz"),
    ("tavern", "taverne"),
    ("barracks", "kaserne"),
    ("smithy", "schmiede"),
    ("armory", "ruestkammer"),
    ("stairsUp", "treppe-hoch"),
    ("stairsDown", "treppe-runter"),
];

/// Die Burg und ihr Bauzustand (Gerüst) in der Mitte überdecken keinen Nachbarplatz im Hub; Plätze, die erst auf
/// verschiedenen Hub-Stufen gebaut werden und sich schon vorher überlappten, zählen nicht.
#[test]
fn burg_ueberdeckt_keinen_nachbarplatz() {
    if !grafik().join("burg/manifest.json").exists() {
        eprintln!("🐛 Grafik nicht gerendert (task gen:paper), Test übersprungen");
        return;
    }
    let t = table();
    let hub: serde_json::Value = serde_json::from_slice(
        &std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/hub.json")).unwrap(),
    )
    .unwrap();
    for center in ["burg", "hub-geruest"] {
        let half = drawn_width(center, &t) / 2.0;
        for s in hub["sites"].as_array().unwrap() {
            let asset = HUB_ASSET
                .iter()
                .find(|(k, _)| *k == s["kind"].as_str().unwrap())
                .unwrap()
                .1;
            let x = s["offsetUnits"].as_f64().unwrap() * 32.0;
            let gap = x.abs() - drawn_width(asset, &t) / 2.0 - half;
            assert!(gap >= 0.0, "{center} überdeckt {asset} um {:.0} px", -gap);
        }
    }
}
