//! Spielstand der Insel (B-1083): Kopf-Typ und `null`-Felder wie in Go. Go liest `version` und `island` in ein `int`
//! (falscher Typ = Fehler, fehlend oder `null` = 0) und lässt `null` in jedem Feld auf dem Standardwert; nur Pflichtfelder
//! mit `null` gelten als fehlend. Rohdaten sind die Fixture der Version 9 (`save_tests::fixture`).

use serde_json::{Value, json};

use super::save_tests::{Env, fixture};

fn base() -> Value {
    serde_json::from_str(&fixture(9)).expect("Fixture ist JSON")
}

fn err_of(env: &Env, v: &Value) -> String {
    env.parse(&v.to_string()).expect_err("Fehler erwartet").to_string()
}

/// AC-01: ein vorhandener Kopf-Wert, der kein ganzzahliges JSON ist, bricht ab (Go: `json.Unmarshal` in `int`).
#[test]
fn kopf_typ_nicht_ganzzahl_ist_fehler() {
    let env = Env::new();
    let cases = [
        (
            "version",
            json!("9"),
            "spielstand: json: cannot unmarshal string into Go struct field .version of type int",
        ),
        (
            "version",
            json!(1.5),
            "spielstand: json: cannot unmarshal number 1.5 into Go struct field .version of type int",
        ),
        (
            "version",
            json!({}),
            "spielstand: json: cannot unmarshal object into Go struct field .version of type int",
        ),
        (
            "island",
            json!("0"),
            "spielstand: json: cannot unmarshal string into Go struct field .island of type int",
        ),
        (
            "island",
            json!(1.5),
            "spielstand: json: cannot unmarshal number 1.5 into Go struct field .island of type int",
        ),
        (
            "island",
            json!({"a": 1}),
            "spielstand: json: cannot unmarshal object into Go struct field .island of type int",
        ),
    ];
    for (key, val, want) in cases {
        let mut m = base();
        m[key] = val.clone();
        assert_eq!(err_of(&env, &m), want, "{key} = {val}");
    }
}

/// AC-01: fehlend oder `null` ist 0; `version` 0 ist unbekannt, `island` 0 ist bei Version 9 ohne Wirkung.
#[test]
fn kopf_fehlend_ist_null() {
    let env = Env::new();
    for (key, how) in [("island", "fehlt"), ("island", "null")] {
        let mut m = base();
        if how == "fehlt" {
            m.as_object_mut().expect("Objekt").remove(key);
        } else {
            m[key] = Value::Null;
        }
        assert!(env.parse(&m.to_string()).is_ok(), "{key} {how}");
    }
    let mut m = base();
    m.as_object_mut().expect("Objekt").remove("version");
    assert!(err_of(&env, &m).starts_with("spielstand: Version 0, erwartet"));
    m["version"] = Value::Null;
    assert!(err_of(&env, &m).starts_with("spielstand: Version 0, erwartet"));
}

/// AC-02: `null` in String-, Vec- und Zahl-Feld von `IslandSave` bleibt der Standardwert, kein Fehler.
#[test]
fn null_in_island_save_ist_standardwert() {
    let env = Env::new();
    let mut m = base();
    m["savedAt"] = Value::Null; // String
    m["defeatedBosses"] = Value::Null; // Vec
    m["goldCollected"] = Value::Null; // Zahl
    m["merchant"] = Value::Null; // Option
    m["completed"] = Value::Null;
    let s = env.parse(&m.to_string()).expect("null ist kein Fehler");
    assert_eq!(s.saved_at, "");
    assert!(s.defeated_bosses.is_empty() && s.completed.is_empty());
    assert_eq!((s.gold_collected, s.merchant), (0, None));
}

/// AC-02: dasselbe für einen Spieler (`IslandPlayerSave`).
#[test]
fn null_in_player_save_ist_standardwert() {
    let env = Env::new();
    let mut m = base();
    m["players"][0]["gold"] = Value::Null; // Zahl
    m["players"][0]["skills"] = Value::Null; // Vec
    m["players"][0]["slots"] = Value::Null;
    m["players"][0]["skillBase"] = Value::Null;
    let s = env.parse(&m.to_string()).expect("null ist kein Fehler");
    assert_eq!(s.players[0].gold, 0);
    assert!(s.players[0].skills.is_empty() && s.players[0].slots.is_empty());
    assert_eq!(s.players[0].skill_base, 0);
    assert_eq!(s.players[1].gold, 8, "der andere Spieler bleibt");
}

/// AC-02: ein Pflichtfeld mit `null` gilt weiter als fehlend.
#[test]
fn pflichtfeld_null_ist_fehler() {
    let env = Env::new();
    let mut m = base();
    m["seed"] = Value::Null;
    assert_eq!(err_of(&env, &m), "spielstand: Feld seed fehlt");
}
