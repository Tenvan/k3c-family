//! Port von `basis_test.go`.

use super::super::test_helpers::{leere_wurzel, schreibe, spec_root, test_repo};
use super::*;

#[test]
fn basis() {
    for ok in ["HEAD", "HEAD~1", "abc1234", "origin/main"] {
        assert!(check_basis(ok).is_ok(), "{ok}");
    }
    let err = check_basis("main").unwrap_err();
    assert_eq!(
        err.to_string(),
        "Basis „main“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/main"
    );
}

fn asset(id: &str, rig: &str, gruppe: &str) -> Asset {
    Asset {
        id: id.to_owned(),
        rig: rig.to_owned(),
        gruppe: gruppe.to_owned(),
        pfad: format!("public/grafik/{gruppe}/{id}.paper.json"),
    }
}

#[test]
fn assets_listen_und_finden() {
    let w = spec_root();
    let root = &w.pfad;
    schreibe(root, "public/grafik/figuren/index.paper.json", "[]");
    let want = vec![
        asset("mauer", "gebaeude", "gebaeude"),
        asset("monarch", "figur", "figuren"),
        asset("pferd", "vierbeiner", "reittiere"),
    ];
    assert_eq!(assets(root).unwrap(), want);
    assert_eq!(finde(root, "pferd").unwrap().gruppe, "reittiere");
    assert_eq!(finde(root, "x").unwrap_err().to_string(), "Asset „x“ fehlt");
    // Neu: ohne Grafik-Ordner ist die Liste leer.
    assert!(assets(&leere_wurzel().pfad).unwrap().is_empty());
}

#[test]
fn geaendert_gegen_head() {
    let Some((w, _)) = test_repo() else { return };
    let root = &w.pfad;
    assert!(geaendert(root).unwrap().is_empty(), "frisch");
    schreibe(root, "public/grafik/figuren/monarch.paper.json", r#"{"id":"monarch"}"#);
    schreibe(root, "public/grafik/requisiten/fass.paper.json", r#"{"id":"fass"}"#);
    schreibe(root, "public/grafik/gebaeude/mauer/idle.png", "neu");
    schreibe(root, "public/grafik/stil.paper.json", "{}");
    let ids: Vec<String> = geaendert(root).unwrap().into_keys().collect();
    assert_eq!(ids, ["fass", "mauer", "monarch"]);
}

/// Neu: Pfade aus `git status`, die keinem Asset gehören.
#[test]
fn asset_von_pfad_wie_go() {
    let faelle = [
        ("public/grafik/a/b.paper.json", Some("b")),
        ("public/grafik/a/b/x/y.png", Some("b")),
        ("public/grafik/a/index.paper.json", None),
        ("public/grafik/a/golden.json", None),
        ("public/grafik/stil.paper.json", None),
        ("public/grafikx/a/b.paper.json", None),
    ];
    for (pfad, want) in faelle {
        assert_eq!(asset_von_pfad(pfad), want, "{pfad}");
    }
}
