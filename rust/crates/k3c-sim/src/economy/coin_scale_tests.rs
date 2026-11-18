//! Tests zu den Goldpreisen (Go: `coin_scale_data_test.go`): BS3.1/AC-01 (B-635), BS3.2/AC-03 (B-638).

use k3c_data::buildings::BuildingData;
use k3c_data::troops::TroopData;
use serde_json::Value;

use crate::enemies::testutil::test_values;

/// Reguläre Goldpreise liegen auf der Münzskala (wirtschaft.md § 8); Ausnahmen stehen ausdrücklich in der Liste.
const COIN_SCALE: [i64; 11] = [2, 3, 4, 6, 8, 10, 12, 15, 20, 25, 30];

/// Goldpreise außerhalb der Skala mit Grund (Kleinbeträge nach § 8).
const SCALE_EXCEPTIONS: [&str; 1] = [
    "troops.vagrant.recruitCost", // Rekrutieren: 1 Münze bleibt (Kleinbetrag)
];

fn on_scale(name: &str, gold: i64, errors: &mut Vec<String>) {
    if SCALE_EXCEPTIONS.contains(&name) {
        return;
    }
    if !COIN_SCALE.contains(&gold) {
        errors.push(format!(
            "{name}: {gold} Gold liegt nicht auf der Münzskala {COIN_SCALE:?}"
        ));
    }
}

fn check_building_scale(kind: &str, b: &BuildingData, errors: &mut Vec<String>) {
    if b.cost.gold > 0 {
        on_scale(&format!("buildings.{kind}"), b.cost.gold, errors);
    }
    for c in &b.cost_by_depth {
        if c.gold > 0 {
            // Tiefen ohne Treppe haben keinen Eintrag
            on_scale(&format!("buildings.{kind}.costByDepth"), c.gold, errors);
        }
    }
    for l in &b.levels {
        on_scale(&format!("buildings.{kind}.levels"), l.cost.gold, errors);
    }
    for a in &b.armor {
        on_scale(&format!("buildings.{kind}.armor"), a.cost.gold, errors);
    }
}

fn check_troop_scale(kind: &str, tr: &TroopData, errors: &mut Vec<String>) {
    if tr.cost.gold > 0 {
        on_scale(&format!("troops.{kind}.cost"), tr.cost.gold, errors);
    }
    if tr.recruit_cost.gold > 0 {
        on_scale(&format!("troops.{kind}.recruitCost"), tr.recruit_cost.gold, errors);
    }
    for (name, p) in tr.professions.iter().flatten() {
        on_scale(&format!("troops.{kind}.professions.{name}"), p.cost.gold, errors);
    }
}

#[test]
fn goldpreise_auf_der_muenzskala() {
    let v = test_values();
    let mut errors = Vec::new();
    for (kind, b) in &v.buildings {
        check_building_scale(kind, b, &mut errors);
    }
    let wall = &v.buildings["wall"];
    assert!(
        !wall.levels.is_empty() && wall.levels[0].cost.gold == 2 && wall.cost.gold == 2,
        "Mauer Stufe 1 muss 2 Gold kosten: {:?}",
        wall.cost
    );
    for lv in &v.hub.levels {
        for st in &lv.steps {
            on_scale("hub.levels.steps", st.cost.gold, &mut errors);
        }
    }
    for (kind, tr) in &v.troops {
        check_troop_scale(kind, tr, &mut errors);
    }
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

/// Go: `data.Files.ReadFile("ship.json")`: die Datei aus data/ des Repos.
fn ship_json() -> Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../data/ship.json");
    let raw = std::fs::read_to_string(path).expect("data/ship.json");
    serde_json::from_str(&raw).expect("Schema")
}

/// Go: `d2.DisallowUnknownFields` für die Felder von ship.json.
fn check_ship_schema(root: &Value) {
    let known = |obj: &serde_json::Map<String, Value>, keys: &[&str], what: &str| {
        for k in obj.keys() {
            assert!(
                keys.contains(&k.to_lowercase().as_str()),
                "Schema: unbekanntes Feld {k} in {what}"
            );
        }
    };
    let obj = root.as_object().expect("Schema: Objekt");
    known(obj, &["version", "notes", "stages"], "ship.json");
    for st in obj["stages"].as_array().expect("Schema: stages") {
        let st = st.as_object().expect("Schema: Abschnitt");
        known(st, &["id", "name", "cost", "hp", "buildseconds"], "stages");
    }
}

#[test]
fn schiffskosten_in_den_daten() {
    let root = ship_json();
    check_ship_schema(&root);
    let stages = root["stages"].as_array().expect("stages");
    let ids = ["keel", "hull", "sail"];
    assert_eq!(
        stages.len(),
        ids.len(),
        "{} Abschnitte, erwartet {}",
        stages.len(),
        ids.len()
    );
    let mut errors = Vec::new();
    for (i, st) in stages.iter().enumerate() {
        let (id, name) = (st["id"].as_str().unwrap_or(""), st["name"].as_str().unwrap_or(""));
        assert!(id == ids[i] && !name.is_empty(), "Abschnitt {i}: {st}");
        let cost = &st["cost"];
        let n = |k: &str| cost[k].as_i64().unwrap_or(0);
        on_scale(&format!("ship.{id}"), n("gold"), &mut errors);
        assert!(
            n("wood") + n("stone") + n("copper") + n("iron") + n("crystal") != 0,
            "Abschnitt {id} ohne Material"
        );
    }
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}
