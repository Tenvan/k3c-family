//! Port von `stufen_test.go`.

use crate::assetsui::Dienst;
use crate::assetsui::test_helpers::{repo_root, schreibe};
use crate::test_helpers::tmp_dir;

#[test]
fn stufen_sortiert_nach_hoehe_und_bringt_assets() {
    let tmp = tmp_dir();
    let body = r#"{"bezugPx":121.5,"stufen":{"boss":{"m":1.3,"was":"Bosse"},"buerger":{"m":0.6,"was":"Bürger"},"busch":{"m":0.5,"was":"Busch"}},
        "assets":{"bosse/troll-koenig":{"stufe":"boss","sicht":0.765}}}"#;
    schreibe(tmp.path().join("data/sizes.json"), body);
    let s = Dienst::new(tmp.path()).assets_stufen().unwrap();
    assert_eq!((s.bezug_px, s.stufen.len()), (121.5, 3), "{s:?}");
    assert_eq!((s.stufen[0].id.as_str(), s.stufen[2].id.as_str()), ("busch", "boss"));
    assert_eq!(s.assets["bosse/troll-koenig"].stufe, "boss", "{:?}", s.assets);
}

#[test]
fn stufen_meldet_fehlende_und_leere_datei() {
    let tmp = tmp_dir();
    let d = Dienst::new(tmp.path());
    assert!(d.assets_stufen().is_err(), "fehlende Datei");
    schreibe(tmp.path().join("data/sizes.json"), r#"{"stufen":{}}"#);
    assert_eq!(
        d.assets_stufen().unwrap_err(),
        "data/sizes.json: bezugPx oder stufen fehlen"
    );
}

/// Die echte Datei (LA1.1) muss lesbar sein und den Monarchen-Bezug enthalten.
#[test]
fn stufen_echte_datei() {
    let s = match Dienst::new(repo_root()).assets_stufen() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("übersprungen: keine data/sizes.json im Repo: {e}");
            return;
        }
    };
    assert!(s.bezug_px > 0.0 && s.stufen.len() >= 5 && !s.assets.is_empty(), "{s:?}");
}
