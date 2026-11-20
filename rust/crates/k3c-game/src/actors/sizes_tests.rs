//! LA1.1 (B-1163): Größenstufen im Katalog und in der Grafik.

use std::path::{Path, PathBuf};

use k3c_data::Source;
use k3c_data::look::{GO, K2C, with_profile};
use k3c_data::sizes::{self, Sizes};

use crate::actors::catalog::{Look, UNIT_PX, load_catalog};

fn table() -> Sizes {
    sizes::from_source(&Source::Embedded).expect("sizes.json")
}

fn grafik() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik")
}

/// Sichtbare Höhe einer Figur in M: Rahmenhöhe des Looks × Anteil der sichtbaren Höhe.
fn visible_m(l: &Look, t: &Sizes) -> f64 {
    l.height * UNIT_PX * t.assets[&l.asset].visible / t.reference_px
}

#[test]
fn k2c_zeichnet_figuren_in_ihrer_stufe() {
    let t = table();
    let c = with_profile(K2C, || load_catalog(&Source::Embedded).unwrap());
    for (kind, l) in c.troops.iter().chain(c.enemies.iter()) {
        let Some(a) = t.assets.get(&l.asset) else { continue };
        let want = t.stages[&a.stage].m;
        assert!(
            (visible_m(l, &t) - want).abs() < 1e-9,
            "{kind} ({}): {} M statt {want} M",
            l.asset,
            visible_m(l, &t)
        );
    }
    for (id, l) in &c.bosses {
        if l.asset.starts_with("bosse/") {
            assert!((visible_m(l, &t) - t.stages["boss"].m).abs() < 1e-9, "Boss {id}");
        }
    }
    let peasant = &c.troops["peasant"];
    assert!((0.55..=0.65).contains(&visible_m(peasant, &t)), "Bürger 0,55–0,65 M");
}

#[test]
fn monarch_bleibt_der_bezugswert() {
    let c = with_profile(K2C, || load_catalog(&Source::Embedded).unwrap());
    assert_eq!(c.players[0].height * UNIT_PX, table().reference_px);
}

#[test]
fn go_profil_behaelt_die_alten_hoehen() {
    let k2c = with_profile(K2C, || load_catalog(&Source::Embedded).unwrap());
    let go = with_profile(GO, || load_catalog(&Source::Embedded).unwrap());
    assert_eq!(
        go.troops["peasant"].height * UNIT_PX,
        72.9,
        "paper.json › figuren.truppen.peasant"
    );
    assert_ne!(go.troops["peasant"].height, k2c.troops["peasant"].height);
    assert_eq!(go.bosses.values().next().map(|l| l.height * UNIT_PX), Some(150.0));
}

/// Der Anteil `sicht` stimmt mit der gerenderten Grafik überein (neu messen, wenn ein Asset neu gezeichnet wird).
#[test]
fn sicht_stimmt_mit_der_grafik() {
    let t = table();
    if !grafik().join("figuren/monarch/manifest.json").exists() {
        eprintln!("🐛 Grafik nicht gerendert (task gen:paper), Abgleich übersprungen");
        return;
    }
    let mut bad = Vec::new();
    for (asset, a) in &t.assets {
        let got = measure(asset);
        if (got - a.visible).abs() > 0.02 {
            bad.push(format!("{asset}: Tabelle {} gemessen {got:.3}", a.visible));
        }
    }
    assert!(bad.is_empty(), "sicht veraltet:\n{}", bad.join("\n"));
}

/// Sichtbare Höhe (Alpha > 16) des ersten Bildes der Ruhe-Animation geteilt durch die Rahmenhöhe.
fn measure(asset: &str) -> f64 {
    let dir = grafik().join(asset);
    let m: serde_json::Value = serde_json::from_slice(&std::fs::read(dir.join("manifest.json")).unwrap()).unwrap();
    let (fw, fh) = (
        m["frameW"].as_u64().unwrap() as usize,
        m["frameH"].as_u64().unwrap() as usize,
    );
    let anims = m["animationen"].as_object().unwrap();
    let name = if anims.contains_key("idle") {
        "idle"
    } else {
        anims.keys().next().unwrap()
    };
    let img = crate::art::read_png(&dir.join(anims[name]["datei"].as_str().unwrap())).unwrap();
    let (pix, w) = (&img.pix, img.width);
    let w = w as usize;
    let rows: Vec<usize> = (0..fh)
        .filter(|&y| (0..fw).any(|x| pix[(y * w + x) * 4 + 3] > 16))
        .collect();
    (rows.last().unwrap() - rows.first().unwrap() + 1) as f64 / fh as f64
}

/// Gebäude und Objekte: `apply_sizes` bringt die sichtbare Höhe auf Stufe × M, ersetzt den Faktor aus paper.json und
/// lässt Figuren-Sheets in Ruhe.
#[test]
fn sheets_bekommen_die_stufenhoehe() {
    use crate::art::{Sheet, apply_sizes};
    let t = table();
    let mk = |asset: &str| {
        let mut s = Sheet::default();
        (s.asset, s.fh, s.units_per_px) = (asset.into(), 200, 1.0);
        s
    };
    let mut sheets = std::collections::BTreeMap::new();
    for a in ["gebaeude-paper/burg", "truppen/bauer"] {
        sheets.insert((a.to_string(), "stand".to_string()), mk(a));
    }
    apply_sizes(&mut sheets, &t);
    let burg = &sheets[&("gebaeude-paper/burg".to_string(), "stand".to_string())];
    let vis = t.assets["gebaeude-paper/burg"].visible * burg.fh as f64 * burg.units_per_px;
    let want = t.target_px("gebaeude-paper/burg").unwrap() / t.reference_px;
    assert!((vis / t.reference_px - want).abs() < 1e-9, "Burg {want} M");
    assert_eq!(
        sheets[&("truppen/bauer".to_string(), "stand".to_string())].units_per_px,
        1.0
    );
}
