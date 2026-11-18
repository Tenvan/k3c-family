//! Schutzplätze der Bürger (Go: `unit_shelter_test.go`, B-605).

use serde_json::Value;

use super::hammer::spawn_vagrant_with;
use super::{shelter_x, step_peasant};
use crate::enemies::testutil::*;

/// B-605/AC-01: Stirbt eine Truppe oder kommt eine hinzu, behalten die übrigen Truppen in der laufenden Gefahr ihren
/// Schutzplatz. Nach der Gefahr legt die nächste den Platz neu fest. (Die Bauern rechnen über `step_peasant`, das
/// `find_job` und damit RU14 und RU18 braucht.)
#[test]
fn schutzplatz_stabil_bei_truppenwechsel() {
    let mut w = quiet_world();
    let v = w.vals.clone().expect("Wertesatz");
    w.cycle.phase = "night".into();
    let hub = w.hub_x;
    for _ in 0..5 {
        let ti = spawn_vagrant_with(&mut w, &v, hub, hub);
        w.troops[ti].kind = "peasant".into();
    }
    let step_all = |w: &mut crate::state::World| {
        for ti in 0..w.troops.len() {
            step_peasant(w, &v, ti, None, 0.1);
        }
    };
    step_all(&mut w);
    let ids: Vec<i64> = w
        .troops
        .iter()
        .map(|t| t.id)
        .filter(|&id| id != w.troops[1].id)
        .collect(); // Bauer 2 stirbt
    let before: Vec<f64> = ids.iter().map(|&id| shelter_of(&mut w, id)).collect();
    // Tod von Bauer 2, ein neuer Bauer vorn und einer hinten: alle Indizes der übrigen verschieben sich.
    w.troops.remove(1);
    let front = spawn_vagrant_with(&mut w, &v, hub, hub);
    w.troops[front].kind = "peasant".into();
    let t = w.troops.pop().expect("neuer Bauer");
    w.troops.insert(0, t);
    let back = spawn_vagrant_with(&mut w, &v, hub, hub);
    w.troops[back].kind = "peasant".into();
    step_all(&mut w);
    for (i, &id) in ids.iter().enumerate() {
        let got = shelter_of(&mut w, id);
        assert_eq!(
            got, before[i],
            "Bauer {i}: Schutzplatz {got} statt {} nach Truppenwechsel",
            before[i]
        );
    }
    w.cycle.phase = "day".into();
    step_all(&mut w);
    w.cycle.phase = "night".into();
    let idx = w.troops.iter().position(|t| t.id == ids[0]).expect("Bauer");
    assert_eq!(
        shelter_x(&mut w, idx),
        hub + ((idx % 5) as f64 - 2.0),
        "nächste Gefahr: neu nach Index"
    );
}

/// Der Schutzplatz einer Truppe nach ID (ihr Index ist im Test nicht stabil).
fn shelter_of(w: &mut crate::state::World, id: i64) -> f64 {
    let ti = w.troops.iter().position(|t| t.id == id).expect("Truppe");
    shelter_x(w, ti)
}

/// B-605/AC-02 (Nachtrag zu B-260/AC-01): Ein zusätzlicher unbezahlter Eintrag in hub.json › sites ändert in den
/// Golden-Läufen nur IDs und den neuen Platz. Jeder Lauf rechnet mit und ohne Eintrag; verglichen wird jeder Snapshot ohne
/// ID-Felder und ohne den neuen Platz. Go überspringt ihn mit `-short`.
#[test]
#[ignore = "lang"]
fn golden_zusatzplatz_nur_ids() {
    let runs = golden_runs(); // `goldenRuns`/`goldenWorld`/`Step` liefern RU14 und RU22
    assert!(!runs.is_empty());
}

/// Go: `eventIDs` sind die Felder von Ereignissen, die eine Entity-ID tragen (Absender, Ziel, Bauplatz, Boss).
const EVENT_IDS: [&str; 4] = ["from", "to", "site", "boss"];

/// Go: `idRef` ist ein Verweis mit ID als Text, z. B. `payKey` "site:55": Kleinbuchstaben, Doppelpunkt, nur Ziffern.
fn strip_id_ref(s: &str) -> String {
    match s.split_once(':') {
        Some((head, tail))
            if !head.is_empty()
                && head.bytes().all(|b| b.is_ascii_lowercase())
                && !tail.is_empty()
                && tail.bytes().all(|b| b.is_ascii_digit()) =>
        {
            format!("{head}:")
        }
        _ => s.to_string(),
    }
}

/// Go: `withoutIDs` entfernt alle ID-Felder (`id`, `…Id`, `…Ids`) und die ID aus Verweisen wie "site:55" aus einem
/// JSON-Baum.
fn without_ids(v: Value) -> Value {
    match v {
        Value::String(s) => Value::String(strip_id_ref(&s)),
        Value::Object(m) => Value::Object(
            m.into_iter()
                .filter(|(k, _)| !(k == "id" || k.ends_with("Id") || k.ends_with("Ids")))
                .map(|(k, e)| (k, without_ids(e)))
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.into_iter().map(without_ids).collect()),
        other => other,
    }
}

/// Go: `idFreeTree` ist der JSON-Baum der Welt ohne IDs (`without_ids`, `EVENT_IDS`).
fn id_free_tree(tree: Value) -> Value {
    let mut tree = without_ids(tree);
    if let Some(events) = tree.get_mut("events").and_then(Value::as_array_mut) {
        for e in events.iter_mut().filter_map(Value::as_object_mut) {
            for k in EVENT_IDS {
                e.remove(k);
            }
        }
    }
    tree
}

#[test]
fn without_ids_entfernt_id_felder_und_id_verweise() {
    let tree = serde_json::json!({
        "id": 5, "kind": "wall", "workerId": 7, "siteIds": [1, 2], "payKey": "site:55", "name": "a:b",
        "sites": [{"id": 1, "x": 2.5, "ref": "troop:9"}],
        "events": [{"type": "hit", "from": 3, "to": 4, "site": 5, "boss": 6, "x": 1}],
    });
    let want = serde_json::json!({
        "kind": "wall", "payKey": "site:", "name": "a:b",
        "sites": [{"x": 2.5, "ref": "troop:"}],
        "events": [{"type": "hit", "x": 1}],
    });
    assert_eq!(id_free_tree(tree), want);
}
