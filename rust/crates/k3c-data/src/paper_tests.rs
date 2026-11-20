//! B-1146 (LA1.2): `waldwand` und `leitmotive` in data/paper.json.

use super::*;
use crate::testutil::raw;

#[test]
fn datei_liefert_waldwand_und_vier_leitmotive() {
    let p = load(&raw(FILE)).expect("paper.json");
    assert_eq!(p.waldwand, "hintergruende/hg-wald-wand");
    assert_eq!(p.leitmotive.len(), 4);
    for w in ["standard", "sengoku", "nil", "britannien"] {
        assert!(p.leitmotive[w].starts_with("hintergruende/leitmotiv-"), "{w}");
    }
}

#[test]
fn datei_ohne_die_schluessel_laedt_weiter() {
    let p = load(r#"{"spieler": ["figuren/monarch"]}"#).expect("alte Datei");
    assert!(p.waldwand.is_empty() && p.leitmotive.is_empty());
    assert_eq!(p.motif_asset("nil"), None, "ohne Leitmotive keine Schicht");
}

#[test]
fn leitmotiv_je_welt_ersatzweise_standard() {
    let p = load(&raw(FILE)).unwrap();
    assert_eq!(p.motif_asset("britannien"), Some("hintergruende/leitmotiv-britannien"));
    assert_eq!(p.motif_asset("unbekannt"), Some("hintergruende/leitmotiv-standard"));
}

/// Jede Welt mit Leitmotiv in paper.json hat Lage und Größe in look.json (`worlds.<level>.motif`).
#[test]
fn leitmotive_und_look_stimmen_ueberein() {
    let p = load(&raw(FILE)).unwrap();
    let k2c = crate::look::profile(crate::look::K2C).unwrap();
    for w in p.leitmotive.keys().filter(|w| *w != "standard") {
        let m = k2c.world(w).motif;
        assert!(
            (0.0..=1.0).contains(&m.x) && (3.0..=5.0).contains(&m.size),
            "{w}: {m:?}"
        );
    }
}
