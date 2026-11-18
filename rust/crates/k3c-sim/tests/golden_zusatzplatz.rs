//! B-605/AC-02 (Go: `TestGoldenZusatzplatzNurIDs`, `engine/sim/unit_shelter_test.go`; hier aus RU22 aktiviert): Ein
//! zusätzlicher unbezahlter Eintrag in hub.json › sites ändert in den Golden-Läufen nur IDs und den neuen Platz. Jeder
//! Lauf rechnet mit und ohne Eintrag; verglichen wird jeder Snapshot ohne ID-Felder und ohne den neuen Platz (beide
//! Seiten rechnen in Rust, darum exakt).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use std::sync::Arc;

use common::golden_run::{GoldenRun, build_world_with, load};
use k3c_data::hub::HubSite;
use k3c_det::golden;
use k3c_sim::world::step;
use k3c_sim::world::values_room::default_values;
use serde_json::Value;

/// Go: `extraSite`: frei zwischen Werkstatt (-16) und Lager (-24), erst ab Hub-Stufe 5 bezahlbar.
fn extra_site() -> HubSite {
    HubSite {
        kind: "armory".into(),
        offset_units: -20.0,
        hub_level: 5,
        ..HubSite::default()
    }
}

/// Go: `eventIDs`: Felder von Ereignissen, die eine Entity-ID tragen.
const EVENT_IDS: [&str; 4] = ["from", "to", "site", "boss"];

/// Go: `idRef`: ein Verweis mit ID als Text, z. B. `payKey` "site:55".
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

/// Go: `withoutIDs`: entfernt `id`, `…Id`, `…Ids` und die ID aus Verweisen.
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

/// Go: `idFreeTree`.
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

/// Go: `withoutSite`: nimmt den unbezahlten Zusatzplatz bei `x` aus der Liste.
fn without_site(tree: &mut Value, kind: &str, x: f64) {
    let sites = tree["sites"].as_array_mut().expect("sites");
    let i = sites
        .iter()
        .position(|s| s["kind"] == kind && s["x"].as_f64() == Some(x))
        .unwrap_or_else(|| panic!("Zusatzplatz bei x={x} fehlt"));
    assert_eq!(sites[i]["state"], "unpaid", "Zusatzplatz nicht unbezahlt: {}", sites[i]);
    sites.remove(i);
}

fn names() -> Vec<String> {
    let dir = golden::testdata().join("golden");
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| {
            let n = e.unwrap().file_name().to_string_lossy().into_owned();
            n.strip_prefix("sim-")
                .and_then(|n| n.strip_suffix(".json.gz"))
                .map(str::to_string)
        })
        .collect();
    v.sort();
    v
}

fn run_one(run: &GoldenRun) {
    let extra = extra_site();
    let mut hub = default_values().hub.clone();
    hub.sites.push(extra.clone());
    let mut base = build_world_with(run, None);
    let mut with = build_world_with(run, Some(Arc::new(hub)));
    let extra_x = with.hub_x + extra.offset_units;
    let mut tick = 0usize;
    for seg in &run.inputs {
        for _ in 0..seg.ticks {
            step(&mut base, &seg.commands, None, run.dt);
            step(&mut with, &seg.commands, None, run.dt);
            tick += 1;
            if !tick.is_multiple_of(run.snapshot_every) {
                continue;
            }
            let want = id_free_tree(golden::tree(&base));
            let mut got = id_free_tree(golden::tree(&with));
            without_site(&mut got, &extra.kind, extra_x);
            let d = golden::diff("world", &want, &got);
            assert!(d.is_empty(), "sim-{} Takt {tick}: Zusatzplatz ändert {d}", run.name);
        }
    }
}

#[test]
fn golden_zusatzplatz_nur_ids() {
    let names = names();
    assert!(names.len() >= 13, "nur {} Golden-Läufe", names.len());
    for n in names {
        run_one(&load(&n));
    }
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
