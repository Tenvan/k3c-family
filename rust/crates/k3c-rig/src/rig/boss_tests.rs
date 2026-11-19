//! Rust-eigene Konsistenztests der Bosse (B-885, RU61.6): Die Bosse sind nach MG10 auf Spec-`version` 3 neu exportiert,
//! ihre Go-Referenzwerte (`testdata/rust/rig/`) sind veraltet und werden in den `*_gegen_go`-Tests übersprungen
//! (`testutil::is_boss`). Hier gilt die Spec als Wahrheit; die Soll-Matrizen prüft `expected_tests` (tools/paper).

use std::path::Path;

use serde_json::Value;

use super::FigurePose;
use super::testutil::{assets, is_boss, load_rig, rig_dirs};

const BOSSE: [&str; 5] = [
    "goblin-anfuehrer",
    "lava-golem",
    "ratten-koenigin",
    "splitter-titan",
    "troll-koenig",
];

fn json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))).expect("JSON")
}

#[test]
fn bosse_sind_version_3() {
    let dirs: Vec<String> = rig_dirs().into_iter().filter(|d| is_boss(d)).collect();
    for name in BOSSE {
        let dir = format!("bosse/{name}");
        assert!(dirs.contains(&dir), "{dir}: Export fehlt (task gen:rig)");
        assert_eq!(
            json(&assets().join(&dir).join("rig.json"))["version"],
            3,
            "{dir}: rig.json"
        );
        let spec = assets().join("../grafik/bosse").join(format!("{name}.paper.json"));
        assert_eq!(json(&spec)["version"], 3, "{name}: Spec");
    }
}

#[test]
fn bosse_laden_und_posieren_fehlerfrei() {
    for name in BOSSE {
        let dir = format!("bosse/{name}");
        let rig = load_rig(&dir);
        assert_eq!(rig.kind, "figur", "{dir}: Art");
        assert!(rig.parts.len() >= 8, "{dir}: nur {} Teile", rig.parts.len());
        let ids: Vec<&str> = rig.parts.iter().map(|p| p.id.as_str()).collect();
        for p in &rig.parts {
            assert!(p.pivot.iter().all(|c| c.is_finite()), "{dir}/{}: Pivot", p.id);
            assert!(
                p.parent.is_empty() || ids.contains(&p.parent.as_str()),
                "{dir}/{}: Eltern {} unbekannt",
                p.id,
                p.parent
            );
            if p.image.file.is_empty() {
                continue; // Teil ohne eigenes Bild (reiner Knochen)
            }
            let png = assets().join(&dir).join(&p.image.file);
            assert!(png.is_file(), "{dir}/{}: Bild {} fehlt", p.id, png.display());
        }
        assert!(!rig.animations.is_empty(), "{dir}: keine Animationen");
        for anim in rig.animations.keys() {
            for k in 0..8 {
                let mut fp = FigurePose::default();
                let t = f64::from(k) * 0.17;
                rig.figure_pose(anim, t, &mut fp)
                    .unwrap_or_else(|e| panic!("{dir} {anim} t={t}: {e}"));
                assert!(fp.angles.iter().all(|a| a.is_finite()), "{dir} {anim}: Winkel");
            }
        }
    }
}
