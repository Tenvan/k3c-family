//! Port von `basis_gruppe_test.go`: dieselbe ID in zwei Gruppen (B-819).

use super::super::test_helpers::{Wurzel, hash_baum, lies, schreibe, test_repo};
use super::*;
use crate::paperassets::{Cli, anlegen, set};

/// Legt dieselbe ID „wolf“ in zwei Gruppen an.
fn doppel_wolf() -> Option<(Wurzel, Cli)> {
    let (w, cli) = test_repo()?;
    let monarch = lies(&w.pfad, "public/grafik/figuren/monarch.paper.json");
    let wolf = monarch.replacen(r#""id": "monarch""#, r#""id": "wolf""#, 1);
    schreibe(&w.pfad, "public/grafik/gegner/wolf.paper.json", &wolf);
    schreibe(&w.pfad, "public/grafik/reittiere/wolf.paper.json", &wolf);
    Some((w, cli))
}

const UMHANG: &str = r#"{"umhang":{"material":"purpur"}}"#;

#[test]
fn gruppe_id_lesbar_und_aenderbar() {
    let Some((w, cli)) = doppel_wolf() else { return };
    let root = &w.pfad;
    let a = finde(root, "gegner/wolf").unwrap();
    assert_eq!(
        (a.gruppe.as_str(), a.pfad.as_str()),
        ("gegner", "public/grafik/gegner/wolf.paper.json")
    );
    let reittier = "public/grafik/reittiere/wolf.paper.json";
    let vorher = lies(root, reittier);
    let r = set(&cli, "gegner/wolf", UMHANG).unwrap();
    assert_eq!(r.asset.gruppe, "gegner");
    let b = lies(root, &a.pfad);
    assert!(b.contains(r#""material": "purpur""#), "nicht geändert: {b:.300}");
    assert_eq!(lies(root, reittier), vorher, "reittiere/wolf wurde verändert");
}

#[test]
fn mehrdeutige_id_abgelehnt() {
    let Some((w, cli)) = doppel_wolf() else { return };
    let root = &w.pfad;
    let err = finde(root, "wolf").unwrap_err();
    assert!(err.to_string().contains("mehrdeutig"), "{err}");
    let Error::Mehrdeutig(m) = err else {
        panic!("Finde wolf: {err}")
    };
    assert_eq!(m.treffer, ["gegner/wolf", "reittiere/wolf"]);
    let vorher = hash_baum(root, &["public/grafik"]);
    let err = set(&cli, "wolf", UMHANG).unwrap_err();
    assert!(matches!(err.fehler, Error::Mehrdeutig(_)), "Set wolf: {err}");
    assert_eq!(hash_baum(root, &["public/grafik"]), vorher, "abgelehntes Set");
    let err = anlegen(&cli, "wolf", "figur", "x", "natur").unwrap_err();
    assert!(err.to_string().contains("gibt es schon"), "Anlegen wolf: {err}");
}

#[test]
fn gruppe_id_fehlerfaelle() {
    let Some((w, _)) = doppel_wolf() else { return };
    let root = &w.pfad;
    assert_eq!(finde(root, "monarch").unwrap().gruppe, "figuren");
    let err = finde(root, "tiere/wolf").unwrap_err().to_string();
    assert!(
        err.contains("tiere") && err.contains("figuren, gebaeude, gegner, reittiere"),
        "{err}"
    );
    let err = finde(root, "gegner/monarch").unwrap_err().to_string();
    assert!(err.contains("fehlt"), "{err}");
}
