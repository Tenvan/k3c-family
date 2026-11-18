use std::path::Path;

use k3c_det::golden;

use crate::testutil::{biome, biome_ids, generate};

/// RU45.2 (B-1080/AC-02): vergleicht die von Go geschriebenen Level (`TestVergleichDump`) mit der Rust-Erzeugung.
/// Ohne `K3C_LEVEL_VERGLEICH` übersprungen; `K3C_LEVEL_SEEDS` setzt die Zahl der Seeds je Biom (Standard 1000).
#[test]
fn vergleich_go() {
    let Ok(dir) = std::env::var("K3C_LEVEL_VERGLEICH") else {
        eprintln!("K3C_LEVEL_VERGLEICH nicht gesetzt, Vergleich übersprungen");
        return;
    };
    let seeds: usize = std::env::var("K3C_LEVEL_SEEDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    let (mut verglichen, mut abweichend) = (0usize, 0usize);
    let mut meldungen: Vec<String> = Vec::new();
    for id in biome_ids() {
        let b = biome(&id);
        for i in 0..seeds {
            let seed = format!("vergleich-{i:04}");
            let d = vergleiche(&dir, &id, &seed, &b);
            verglichen += 1;
            if !d.is_empty() {
                abweichend += 1;
                meldungen.push(format!("Biom {id}, Seed {seed:?}: {d}"));
            }
        }
    }
    for m in meldungen.iter().take(20) {
        eprintln!("{m}");
    }
    eprintln!("verglichen: {verglichen}, abweichend: {abweichend}");
    assert!(abweichend == 0, "{abweichend} von {verglichen} Leveln weichen ab");
}

fn vergleiche(dir: &str, id: &str, seed: &str, b: &crate::biome::Biome) -> String {
    let file = Path::new(dir).join(id).join(format!("{seed}.json"));
    let raw = match std::fs::read_to_string(&file) {
        Ok(r) => r,
        Err(e) => return format!("{}: {e}", file.display()),
    };
    let want = match serde_json::from_str(&raw) {
        Ok(w) => w,
        Err(e) => return format!("{}: {e}", file.display()),
    };
    let got = generate(b, seed);
    golden::diff("level", &want, &golden::tree(&got))
}
