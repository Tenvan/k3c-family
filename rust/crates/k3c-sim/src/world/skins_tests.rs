//! Tests des Welt-Felds `skins` (Go: `world_skins_test.go`; B-984, WE5.5, AC-01 bis AC-03): laden, prüfen, mit `extends`
//! mischen; Name-Override für Gegner. `ResolveWorld` und `ResolveWorldValues` kommen seit RU14.4 aus `values_room` und
//! `values`; der Fall mit Override wartet auf `values_from` (RU18). Die `own_*`-Tests prüfen Laden, Prüfen, `chain_skins`.

use k3c_data::Source;
use serde_json::json;

use super::base::base;
use super::skins::{chain_skins, decode_world_skins};
use super::{values, values_room};
use crate::island::WorldDef;
use crate::island::switch_testutil::{test_island, test_world};
use crate::island::testutil::parse_world_in;
use crate::island::worlds::world_path;
use crate::state::shared::WorldValues;
use std::sync::Arc;

/// Go: `skinsWorld`: die Test-Welt `id` mit dem zusätzlichen Kopf-Feld `extra` (beginnt mit `"` oder ist leer).
fn skins_world(id: &str, extra: &str) -> WorldDef {
    let raw = test_world(&test_island("a", ""))
        .replacen("\"islands\"", &format!("{extra}\"islands\""), 1)
        .replacen(r#""id":"test""#, &format!(r#""id":"{id}""#), 1);
    match parse_world_in(&raw, id) {
        Ok(w) => w,
        Err(e) => panic!("{e}"),
    }
}

/// Go: `testWorlds`: `lookup` über die Welten `worlds`.
fn lookup_in(worlds: &[WorldDef]) -> impl Fn(&str) -> Result<WorldDef, String> + '_ {
    move |id| {
        worlds
            .iter()
            .find(|w| w.id == id)
            .cloned()
            .ok_or_else(|| format!("{}: Datei fehlt", world_path(id)))
    }
}

/// Go: `ResolveWorld(w, testWorlds(worlds...))` auf den eingebetteten Daten.
fn resolve_world(w: &WorldDef, worlds: &[WorldDef]) -> Result<WorldDef, String> {
    let b = base()?;
    values_room::resolve_world(w, &Source::Embedded, &b.data, &b.rules, &lookup_in(worlds))
}

/// Go: `ResolveWorldValues(data.Files, testWorlds(worlds...), id)`.
fn resolve_world_values(worlds: &[WorldDef], id: &str) -> Result<Arc<WorldValues>, String> {
    values::resolve_world_values(&|name| Source::Embedded.read(name), &lookup_in(worlds), id).map(Arc::new)
}

fn map(pairs: &[(&str, &str)]) -> std::collections::BTreeMap<String, String> {
    pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
}

#[test]
fn skins_laedt_und_mischt_mit_extends() {
    let parent = skins_world(
        "eltern",
        r#""skins":{"gegner/wolf":"gegner/ninja","objekte/tanne":"objekte/kiefer"},"#,
    );
    let child = skins_world(
        "kind",
        r#""extends":"eltern","skins":{"gegner/wolf":"gegner/fuchs","gegner/goblin":"gegner/oni"},"#,
    );
    assert!(
        parent.skins.get("gegner/wolf").map(String::as_str) == Some("gegner/ninja") && parent.skins.len() == 2,
        "Eltern-Skins = {:?}",
        parent.skins
    );
    assert!(
        skins_world("leer", "").skins.is_empty(),
        "ohne skins muss die Zuordnung leer sein"
    );
    let r = resolve_world(&child, &[parent, child.clone()]).expect("auflösen");
    let want = map(&[
        ("gegner/wolf", "gegner/fuchs"),
        ("gegner/goblin", "gegner/oni"),
        ("objekte/tanne", "objekte/kiefer"),
    ]);
    assert_eq!(r.skins, want, "gemischt");
    assert_eq!(
        child.skins.len(),
        2,
        "ResolveWorld darf die übergebene Zuordnung nicht verändern"
    );
}

#[test]
fn own_skins_laedt_und_chain_skins_mischt_mit_extends() {
    let parent = skins_world(
        "eltern",
        r#""skins":{"gegner/wolf":"gegner/ninja","objekte/tanne":"objekte/kiefer"},"#,
    );
    let child = skins_world(
        "kind",
        r#""extends":"eltern","skins":{"gegner/wolf":"gegner/fuchs","gegner/goblin":"gegner/oni"},"#,
    );
    assert!(
        parent.skins.get("gegner/wolf").map(String::as_str) == Some("gegner/ninja") && parent.skins.len() == 2,
        "Eltern-Skins = {:?}",
        parent.skins
    );
    assert!(
        skins_world("leer", "").skins.is_empty(),
        "ohne skins muss die Zuordnung leer sein"
    );
    let chain = [parent.clone(), child.clone()]; // älteste Welt zuerst
    let want = map(&[
        ("gegner/wolf", "gegner/fuchs"),
        ("gegner/goblin", "gegner/oni"),
        ("objekte/tanne", "objekte/kiefer"),
    ]);
    assert_eq!(chain_skins(&chain), want, "das Kind überschreibt je Schlüssel");
    assert_eq!(child.skins.len(), 2, "chain_skins verändert die Welten nicht");
    assert_eq!(parent.skins.len(), 2);
    assert!(chain_skins(&[]).is_empty(), "ohne Skins leer");
    assert!(chain_skins(&[skins_world("leer", "")]).is_empty());
}

#[test]
fn skins_fehlerfaelle() {
    let cases = [
        (
            r#"{"gegner/wolf":"ninja"}"#,
            r#"skins › gegner/wolf: Ersatz-Asset "ninja" ist kein <gruppe>/<asset>"#,
        ),
        (
            r#"{"wolf":"gegner/ninja"}"#,
            r#"skins › wolf: Basis-Asset "wolf" ist kein <gruppe>/<asset>"#,
        ),
        (
            r#"{"gegner/wolf":"objekte/tanne"}"#,
            r#"skins › gegner/wolf: Ersatz-Asset "objekte/tanne" wechselt die Gruppe (gegner → objekte)"#,
        ),
        (
            r#"{"gegner/wolf":"gegner/wolf"}"#,
            "skins › gegner/wolf: verweist auf sich selbst",
        ),
        (
            r#"{"gegner/wolf":"Gegner/Ninja"}"#,
            r#"skins › gegner/wolf: Ersatz-Asset "Gegner/Ninja" ist kein <gruppe>/<asset>"#,
        ),
        (
            r#"{"gegner/wolf":3}"#,
            "skins › gegner/wolf: Ersatz-Asset muss ein Text sein",
        ),
        (
            r#"["gegner/wolf"]"#,
            "skins: Objekt Basis-Asset → Ersatz-Asset erwartet",
        ),
    ];
    for (skins, want) in cases {
        let raw =
            test_world(&test_island("a", "")).replacen("\"islands\"", &format!("\"skins\":{skins},\"islands\""), 1);
        let err = parse_world_in(&raw, "test").err();
        let wanted = format!("data/worlds/test.json: {want}");
        assert!(
            err.as_ref().is_some_and(|e| e.contains(&wanted)),
            "skins {skins}: Fehler {err:?}, erwartet {want:?}"
        );
    }
}

#[test]
fn own_decode_world_skins_prueft_in_schluessel_reihenfolge_und_nimmt_nur_objekte() {
    let ok = decode_world_skins(&json!({"objekte/tanne": "objekte/kiefer", "gegner/wolf": "gegner/ninja"}));
    assert_eq!(
        ok,
        Ok(map(&[
            ("gegner/wolf", "gegner/ninja"),
            ("objekte/tanne", "objekte/kiefer")
        ]))
    );
    assert_eq!(decode_world_skins(&json!({})), Ok(map(&[])));
    // zwei Fehler: gemeldet wird der erste Schlüssel in sortierter Reihenfolge
    let two = decode_world_skins(&json!({"zzz/a": "zzz/a", "aaa/b": "bbb/b"}));
    assert_eq!(
        two,
        Err(r#"skins › aaa/b: Ersatz-Asset "bbb/b" wechselt die Gruppe (aaa → bbb)"#.to_string())
    );
    for raw in [json!(null), json!([]), json!("x"), json!(3), json!(true)] {
        assert_eq!(
            decode_world_skins(&raw),
            Err("skins: Objekt Basis-Asset → Ersatz-Asset erwartet".to_string()),
            "{raw}"
        );
    }
    // null als Wert: Go lässt den Text leer, die Prüfung meldet es
    assert_eq!(
        decode_world_skins(&json!({"gegner/wolf": null})),
        Err(r#"skins › gegner/wolf: Ersatz-Asset "" ist kein <gruppe>/<asset>"#.to_string())
    );
}

#[test]
fn own_asset_form_wie_die_regex() {
    let good = ["a/b", "gegner/wolf", "0/9", "a-b_c/d_e-f", "a1/b2"];
    for g in good {
        // Ersatz in derselben Gruppe, ein anderes Asset
        let group = g.split('/').next().unwrap_or_default();
        let r = decode_world_skins(&json!({ g: format!("{group}/zz") }));
        assert!(r.is_ok(), "{g}: {r:?}");
    }
    for b in ["", "a", "a/", "/b", "a/b/c", "-a/b", "a/B", "A/b"] {
        let r = decode_world_skins(&json!({ "a/x": b }));
        assert!(r.is_err(), "Ersatz {b:?} muss abgelehnt werden: {r:?}");
    }
    let bad = [
        "", "a", "a/", "/b", "a/b/c", "-a/b", "_a/b", "a/-b", "A/b", "a/B", "a b/c", "ä/b", "a/b\n",
    ];
    for b in bad {
        let r = decode_world_skins(&json!({ b: "a/b" }));
        assert!(r.is_err(), "{b:?} muss abgelehnt werden: {r:?}");
    }
}

#[test]
fn own_ersatz_asset_gleiche_gruppe_und_nicht_selbst() {
    assert!(decode_world_skins(&json!({"gegner/wolf": "gegner/fuchs"})).is_ok());
    assert_eq!(
        decode_world_skins(&json!({"gegner/wolf": "gegner/wolf"})),
        Err("skins › gegner/wolf: verweist auf sich selbst".to_string())
    );
}

#[test]
fn skins_resolve_world_ohne_eltern() {
    let w = skins_world("test", r#""skins":{"gegner/wolf":"gegner/ninja"},"#);
    let r = resolve_world(&w, std::slice::from_ref(&w)).expect("auflösen");
    assert_eq!(
        r.skins.get("gegner/wolf").map(String::as_str),
        Some("gegner/ninja"),
        "Skins nach ResolveWorld = {:?}",
        r.skins
    );
}

// B-984/AC-03: `overrides › enemies › <art> › name` ändert den Namen, alle anderen Werte bleiben die der Basisart.
#[test]
#[ignore = "wartet auf RU18 (values_from: check_presets, check_world_hub_sites)"]
fn world_values_gegner_name_uebersteuerbar() {
    let mut w = WorldDef {
        id: "sengoku".into(),
        ..WorldDef::default()
    };
    let set = |w: &mut WorldDef, name: &str| {
        w.overrides.insert("enemies".into(), json!({"wolf": {"name": name}}));
    };
    set(&mut w, "Ninja");
    let v = resolve_world_values(std::slice::from_ref(&w), "sengoku").expect("Werte");
    let base = crate::world::vals(&super::testutil::bare_world()).enemies["wolf"].clone();
    let mut got = v.enemies["wolf"].clone();
    assert!(
        base.name == "Wolf" && got.name == "Ninja",
        "Name Basis {:?}, Welt {:?}; erwartet Wolf und Ninja",
        base.name,
        got.name
    );
    got.name = base.name.clone();
    assert_eq!(got, base, "außer dem Namen darf sich nichts ändern");
    for empty in ["", "  "] {
        set(&mut w, empty);
        let err = resolve_world_values(std::slice::from_ref(&w), "sengoku").err();
        assert!(
            err.as_ref().is_some_and(|e| e.contains("wolf: name")),
            "leerer Name {empty:?}: Fehler = {err:?}"
        );
    }
}
