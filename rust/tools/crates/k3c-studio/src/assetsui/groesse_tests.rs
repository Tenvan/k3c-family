//! Port von `groesse_test.go`.

use crate::assetsui::test_helpers::{dienst, repo_root, schreibe, test_repo};
use crate::test_helpers::tmp_dir;

/// Spielrolle, Maßstab des Monarchen und 1 px je Einheit (B-733/AC-01), gegen die echten Daten.
#[test]
fn assets_groesse() {
    let d = dienst(&repo_root());
    let monarch = d.assets_groesse("monarch").unwrap();
    // data/paper.json › figuren › spieler[0]: hoehe 121,5 (= 81 × 1,5); Spec: ausgabe.rahmen[1] = -145
    let want = 81.0 * 1.5 / 145.0;
    assert!(
        (monarch.px_pro_einheit - want).abs() <= 1e-9,
        "{monarch:?}, will {want}"
    );
    assert!(monarch.quelle.contains("Spieler"), "{monarch:?}");
    let boss = d.assets_groesse("goblin-anfuehrer").unwrap();
    assert_eq!(boss.px_pro_einheit, monarch.px_pro_einheit, "{boss:?}");
    assert!(boss.quelle.contains("Monarchen"), "{boss:?}");
    let burg = d.assets_groesse("burg").unwrap();
    assert_eq!(burg.px_pro_einheit, 1.0, "{burg:?}");
    let goblin = d.assets_groesse("goblin").unwrap();
    assert!(goblin.quelle.contains("goblin"), "{goblin:?}");
}

/// B-853: Spec und Maßstab kommen aus dem Stand der Basis, die Rolle aus dem Arbeitsstand; ein Asset, das es in der
/// Basis nicht gibt, und eine leere Basis sind Fehler.
#[test]
fn assets_groesse_basis() {
    let Some(repo) = test_repo() else { return };
    let paper = r#"{"spieler": ["figuren/monarch"], "figuren": {"spieler": [{"hoehe": 120}]}}"#;
    schreibe(repo.pfad.join("data/paper.json"), paper);
    let d = dienst(&repo.pfad);
    let monarch = d.assets_groesse_basis("monarch", "HEAD~1").unwrap();
    let rahmen = monarch.spec["ausgabe"]["rahmen"].as_array().expect("Rahmen");
    assert_eq!(rahmen.len(), 4, "{:?}", monarch.spec);
    assert!(monarch.spec.get("$schema").is_none(), "{:?}", monarch.spec);
    let want = 120.0 / -rahmen[1].as_f64().unwrap();
    let g = &monarch.groesse;
    assert!((g.px_pro_einheit - want).abs() <= 1e-9, "{g:?}, will {want}");
    assert!(g.quelle.contains("Spieler"), "{g:?}");
    // fass gibt es nur in der Basis HEAD~1 (im Arbeitsstand gelöscht): Requisiten zeichnet das Spiel 1:1.
    let fass = d.assets_groesse_basis("fass", "HEAD~1").unwrap();
    assert_eq!(fass.groesse.px_pro_einheit, 1.0);
    assert!(
        d.assets_groesse_basis("pferd", "HEAD~1").is_err(),
        "pferd fehlt in HEAD~1"
    );
    assert!(d.assets_groesse_basis("monarch", "").is_err(), "leere Basis");
}

/// Biome aus den Ordnernamen; ein fremder Name wird abgelehnt, ohne eine Datei zu lesen. Die Ordner baut der Test
/// selbst: die gerenderten PNG sind nicht eingecheckt (B-737).
#[test]
fn assets_hintergrund() {
    let tmp = tmp_dir();
    for ordner in [
        "hg-wald-fern-v1",
        "hg-wald-mitte-v1",
        "hg-wald-nah-v1",
        "hg-wald-nah-v2",
        "hg-hoehle-fern-v1",
    ] {
        let pfad = tmp.path().join("public/grafik/hintergruende").join(ordner);
        schreibe(pfad.join("stand.png"), b"\x89PNG");
    }
    let d = dienst(tmp.path());
    assert_eq!(d.assets_hintergruende().unwrap(), ["wald"]);
    let h = d.assets_hintergrund("wald").unwrap();
    for ebene in [h.fern, h.mitte, h.nah] {
        assert!(ebene.starts_with("data:image/png;base64,"), "{ebene}");
    }
    assert!(d.assets_hintergrund("../wald").is_err(), "fremder Name");
}
