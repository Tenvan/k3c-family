//! B-982: `events` als Override-Datei einer Welt (Go: `events_values_test.go`, `docs/rules/welten.md` § 1.1). Die Tests
//! lösen die Werte der Welt auf (`resolve_world_values`); die Raum-Tests des Go-Originals (`TestEventsOverrideAendertAbstand`
//! Teil Raum, `TestEventsAbstandNullIstAus` Teil Händler) laufen mit dem Vollmond-Rhythmus in `events/moon_tests.rs`.

use std::collections::BTreeMap;

use k3c_data::events::NightEvent;
use k3c_data::{Fehler, Source};
use serde_json::Value;

use super::values::{LookupFn, resolve_world_values};
use crate::island::WorldDef;
use crate::island::worlds::{STANDARD_WORLD_ID, world_path};
use crate::state::WorldValues;

fn base_read(name: &str) -> Result<String, Fehler> {
    Source::Embedded.read(name)
}

fn world(id: &str, extends: &str, events: Option<&str>) -> WorldDef {
    let mut overrides: BTreeMap<String, Value> = BTreeMap::new();
    if let Some(raw) = events {
        overrides.insert("events".into(), serde_json::from_str(raw).unwrap_or(Value::Null));
    }
    WorldDef {
        id: id.into(),
        extends: extends.into(),
        overrides,
        ..WorldDef::default()
    }
}

fn lookup_of(ws: Vec<WorldDef>) -> impl Fn(&str) -> Result<WorldDef, String> {
    move |id| {
        ws.iter()
            .find(|w| w.id == id)
            .cloned()
            .ok_or_else(|| format!("{}: Datei fehlt", world_path(id)))
    }
}

fn resolve(lookup: LookupFn, id: &str) -> Result<WorldValues, String> {
    resolve_world_values(&base_read, lookup, id)
}

fn event<'a>(v: &'a WorldValues, id: &str) -> &'a NightEvent {
    v.events
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("Event {id} fehlt"))
}

fn standard() -> WorldDef {
    world(STANDARD_WORLD_ID, "", None)
}

/// B-982/AC-01: Der Override ändert den Abstand des Vollmonds; Blutmond, Belohnung und die Basis bleiben.
#[test]
fn events_override_aendert_abstand() {
    let lookup = lookup_of(vec![
        standard(),
        world(
            "nacht",
            STANDARD_WORLD_ID,
            Some(r#"{"events": {"fullMoon": {"everyNights": 5}}}"#),
        ),
    ]);
    let v = resolve(&lookup, "nacht").expect("Welt löst auf");
    let base = resolve(&lookup, STANDARD_WORLD_ID).expect("Standard-Welt");
    assert_eq!(event(&v, "fullMoon").every_nights, 5);
    assert_eq!(event(&v, "fullMoon").reward_gold, event(&base, "fullMoon").reward_gold);
    assert_eq!(event(&v, "bloodMoon"), event(&base, "bloodMoon"));
    assert_eq!(event(&base, "fullMoon").every_nights, 7, "die Basis bleibt unverändert");
}

/// B-982/AC-01: Abstand 0 ist erlaubt und steht in den Werten (das Event ist aus: `moon.rs` prüft `every_nights > 0`).
#[test]
fn events_abstand_null_wird_uebernommen() {
    let lookup = lookup_of(vec![
        standard(),
        world(
            "still",
            STANDARD_WORLD_ID,
            Some(r#"{"events": {"bloodMoon": {"everyNights": 0}, "merchantRaid": {"everyVisits": 0}}}"#),
        ),
    ]);
    let v = resolve(&lookup, "still").expect("Abstand 0 ist erlaubt");
    assert_eq!(event(&v, "bloodMoon").every_nights, 0);
    assert_eq!(event(&v, "merchantRaid").every_visits, 0);
    assert!(event(&v, "fullMoon").every_nights > 0, "Vollmond bleibt an");
}

/// B-982: Eine Kind-Welt erbt die Event-Overrides der Eltern-Welt.
#[test]
fn events_kind_erbt_override() {
    let lookup = lookup_of(vec![
        standard(),
        world(
            "eltern",
            STANDARD_WORLD_ID,
            Some(r#"{"events": {"fullMoon": {"everyNights": 3}}}"#),
        ),
        world("kind", "eltern", None),
    ]);
    let v = resolve(&lookup, "kind").expect("Kind löst auf");
    assert_eq!(event(&v, "fullMoon").every_nights, 3);
}

/// B-982/AC-02: Fehlerfälle laden nicht; die Meldung nennt Welt und Pfad.
#[test]
fn events_override_abgelehnt() {
    let cases = [
        (r#"{"events": {"taifun": {"everyNights": 3}}}"#, "taifun"),
        (r#"{"events": {"fullMoon": {"everyNights": -1}}}"#, "Abstand"),
        (r#"{"events": {"merchantRaid": {"everyVisits": -2}}}"#, "Abstand"),
        (r#"{"events": {"bloodMoon": {"damageFactor": 0}}}"#, "bloodMoon"),
        (r#"{"events": {"fullMoon": {"kind": "drache"}}}"#, "drache"),
        (r#"{"events": {"fullMoon": {"gibtEsNicht": 1}}}"#, "gibtEsNicht"),
        (r#"{"notes": "x"}"#, "notes"),
    ];
    for (raw, want) in cases {
        let lookup = lookup_of(vec![standard(), world("kaputt", STANDARD_WORLD_ID, Some(raw))]);
        let err = resolve(&lookup, "kaputt").expect_err(raw);
        assert!(err.contains("kaputt"), "{raw}: Meldung nennt die Welt nicht: {err}");
        assert!(err.contains(want), "{raw}: Meldung {err:?} enthält {want:?} nicht");
    }
}
