//! LA1.7 (B-947, B-1160, B-1175): Mauerzug ohne Lücken, Burgbereich geschlossen, keine Platzhalter-Rechtecke.

use std::path::{Path, PathBuf};

use k3c_data::look::{GO, K2C, with_profile};
use k3c_sim::state::Site;

use super::buildings::{BackWall, back_walls, site_look};

fn wall(id: i64, x: f64, level: i64, state: &str) -> Site {
    Site {
        id,
        kind: "wall".to_string(),
        x,
        level,
        state: state.to_string(),
        ..Default::default()
    }
}

fn run(sites: &[Site]) -> Vec<BackWall> {
    back_walls(Vec::new(), &mut Vec::new(), sites, 500.0)
}

/// B-1175: Zwischen den innersten Mauern beider Seiten steht im Burgbereich eine Mauer (nur Profil k2c, beide gebaut).
#[test]
fn burgbereich_ist_geschlossen() {
    let both = [wall(1, 460.0, 2, "built"), wall(2, 540.0, 1, "built")];
    let center = BackWall {
        id: 2,
        from: 460.0,
        to: 540.0,
        stage: 1,
    };
    with_profile(K2C, || {
        assert_eq!(run(&both), vec![center], "innere Mauern links und rechts verbunden");
        let more = [
            both[0].clone(),
            both[1].clone(),
            wall(3, 420.0, 2, "built"),
            wall(4, 580.0, 2, "built"),
        ];
        let got = run(&more);
        assert!(
            got.contains(&center) && got.len() == 3,
            "{got:?}: Burgbereich plus beide Außenabschnitte"
        );
        assert!(
            run(&[wall(1, 460.0, 1, "built"), wall(2, 540.0, 1, "unpaid")]).is_empty(),
            "rechts nicht gebaut"
        );
        assert!(run(&[wall(1, 460.0, 1, "built")]).is_empty(), "nur eine Seite");
    });
    with_profile(GO, || assert!(run(&both).is_empty(), "Profil go wie Go"));
}

fn grafik() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik/gebaeude-paper")
}

/// B-947: Die Kacheln der Hintergrundmauer haben transparenten Rand; `kachelung` in paper.json nennt den sichtbaren
/// Streifen. Er ist in allen fünf Stufen lückenlos gefüllt, damit die aneinandergesetzten Streifen keine Fuge lassen.
#[test]
fn hintergrundmauer_kachelt_ohne_fugen() {
    let dir = grafik().join("hintergrundmauer");
    if !dir.join("manifest.json").exists() {
        eprintln!("🐛 Grafik nicht gerendert (task gen:paper), Test übersprungen");
        return;
    }
    let raw = std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/paper.json")).unwrap();
    let tiling = crate::art::parse_tiling(&raw).unwrap();
    let (from, to) = tiling["gebaeude-paper/hintergrundmauer"];
    let m: serde_json::Value = serde_json::from_slice(&std::fs::read(dir.join("manifest.json")).unwrap()).unwrap();
    let (fw, fh) = (
        m["frameW"].as_u64().unwrap() as usize,
        m["frameH"].as_u64().unwrap() as usize,
    );
    assert!(
        from > 0.0 && (to as usize) < fw,
        "Streifen {from}–{to} liegt im Rahmen {fw}"
    );
    for (name, a) in m["animationen"].as_object().unwrap() {
        let img = crate::art::read_png(&dir.join(a["datei"].as_str().unwrap())).unwrap();
        let w = img.width as usize;
        for x in (from as usize)..(to as usize) {
            let filled = (0..fh).any(|y| img.pix[(y * w + x) * 4 + 3] > 16);
            assert!(filled, "Stufe {name}: Spalte {x} ist leer, Fuge zwischen den Kacheln");
        }
        let edge = |x: usize| (0..fh).filter(|&y| img.pix[(y * w + x) * 4 + 3] > 16).count();
        assert_eq!(
            edge(0),
            0,
            "Stufe {name}: Rand links ist transparent (das ist der Grund für die Fugen)"
        );
        assert!(
            edge(from as usize) > 30 && edge(to as usize - 1) > 30,
            "Stufe {name}: Streifen am Rand voll"
        );
    }
}

/// B-1160: Jede Platzart des Hubs hat im gebauten Zustand ein Papierschnitt-Bild, kein farbiges Platzhalter-Rechteck.
#[test]
fn kein_platzhalter_rechteck_im_hub() {
    let hub: serde_json::Value = serde_json::from_slice(
        &std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/hub.json")).unwrap(),
    )
    .unwrap();
    let mut kinds: Vec<String> = ["wall", "tower", "nursery", "ship"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    for key in ["sites", "islandSites"] {
        for s in hub[key].as_array().unwrap() {
            kinds.push(s["kind"].as_str().unwrap().to_string());
        }
    }
    for k in kinds {
        assert!(
            site_look(&k, "built", 1).ok(),
            "{k}: gebaut ohne Bild (Platzhalter-Rechteck)"
        );
    }
}

/// WL1.4 (B-1213) AC-03: Höhe der Mauer je Ausbaustufe von halb (Zaun) bis voll (Kristall), gleichmäßig aufgeteilt.
#[test]
fn mauerhoehe_waechst_mit_der_stufe() {
    use super::buildings::wall_height;
    let got: Vec<f64> = (1..=5).map(wall_height).collect();
    assert_eq!(got, [0.5, 0.625, 0.75, 0.875, 1.0]);
    assert_eq!(wall_height(0), 1.0, "ohne Stufe volle Höhe");
    assert_eq!(wall_height(9), 1.0, "über Stufe 5 volle Höhe");
}

/// WL1.4 AC-02: Die Mauer-Grafik hat keinen leeren Rand mehr; der Rahmen ist so breit wie der Zaun samt Zinnen und
/// Stufen-Zusätzen (Stufe 5 bis 16,6), damit Nachbarn ohne Spalt anschließen.
#[test]
fn mauerrahmen_ohne_leeren_rand() {
    let spec: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik/gebaeude-paper/mauer.paper.json"),
        )
        .expect("Spec"),
    )
    .expect("JSON");
    let r = &spec["ausgabe"]["rahmen"];
    let (x, w) = (r[0].as_f64().expect("x"), r[2].as_f64().expect("b"));
    assert!(w <= 36.0, "Rahmen {w} px breit: höchstens Kunst plus 1 px Rand");
    assert!(-x >= 16.6 && x + w >= 16.6, "Stufe 5 (16,6 px) passt in den Rahmen");
}
