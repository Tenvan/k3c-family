//! Port von `engine/simcli/hub_test.go` (alle sieben Tests, RU33.2): Matrix, Befunde und erlaubte Arten des Gebäude-Planers.

use serde_json::Value;

use super::EXIT_USAGE;
use super::testutil::{call, report, root};

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).expect("Datei")
}

fn plan_of(raw: &str) -> Value {
    let input = serde_json::json!({"op": "plan", "raw": raw}).to_string();
    let (code, out, err) = call(&input, &["hub"]);
    assert_eq!(code, 0, "{err}");
    report(&out)
}

fn row<'a>(m: &'a Value, key: &str) -> &'a Value {
    m["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .find(|r| r["key"] == key)
        .unwrap_or_else(|| panic!("Zeile {key} fehlt"))
}

fn cells(r: &Value) -> &Vec<Value> {
    r["cells"].as_array().expect("cells")
}

/// Beispiel 1 aus B-462: Zuordnung Hub-Stufe × Bauplatz mit Platz und Tiefe der heutigen data/hub.json.
#[test]
fn hub_plan_matrix_der_heutigen_datei() {
    let rep = plan_of(&read("data/hub.json"));
    assert!(
        rep["parseError"] == "" && rep["findings"].as_array().expect("findings").is_empty(),
        "{rep}"
    );
    let m = &rep["matrix"];
    let want = [
        ("workshop@-16", 1),
        ("barracks@24", 2),
        ("tavern@-32", 2),
        ("storage@-24", 2),
        ("bowyer@-28", 1),
        ("stairsDown@8", 2),
        ("stairsUp@16", 2),
        ("smithy@28", 3),
        ("healer@-40", 3),
        ("armory@40", 4),
        ("zone1", 1),
        ("merchantBuy@16", 1),
        ("merchantSell@20", 1),
    ];
    assert_eq!(m["levels"], 5);
    assert_eq!(m["rows"].as_array().expect("rows").len(), want.len());
    for (key, level) in want {
        let r = row(m, key);
        assert_eq!(r["hubLevel"], level, "{key}");
        assert_eq!(cells(r).len(), 5, "{key}");
        for (i, c) in cells(r).iter().enumerate() {
            assert_eq!(c["available"], i + 1 >= level, "{key}: Stufe {}", i + 1);
        }
    }
}

#[test]
fn hub_plan_platz_tiefe_baugrund_und_haendler() {
    let rep = plan_of(&read("data/hub.json"));
    let m = &rep["matrix"];
    let armory = cells(row(m, "armory@40"));
    assert!(armory[2]["available"] == false && armory[3]["available"] == true);
    assert!(armory[3]["offsetUnits"] == 40 && armory[3]["onlyDepth"] == -1 && armory[3]["fromDepth"] == 0);
    let up = &cells(row(m, "stairsUp@16"))[1];
    assert!(up["fromDepth"] == 1 && up["needsDeeper"] == false, "Treppe hoch: {up}");
    let down = &cells(row(m, "stairsDown@8"))[4];
    assert!(
        down["needsDeeper"] == true && down["offsetUnits"] == 8,
        "Treppe runter: {down}"
    );
    let storage = row(m, "storage@-24");
    assert!(
        storage["source"] == "islandSites" && storage["index"] == 0,
        "Lager: {storage}"
    );
}

#[test]
fn hub_plan_baugrund_und_haendler() {
    let rep = plan_of(&read("data/hub.json"));
    let zone = row(&rep["matrix"], "zone1");
    assert!(
        zone["bothSides"] == true
            && zone["source"] == "sitePlacement"
            && zone["towerUnits"] == 36
            && zone["line"] == 1
            && cells(zone)[0]["offsetUnits"] == 44,
        "Baugrund Zone 1: {zone}"
    );
    let buy = row(&rep["matrix"], "merchantBuy@16");
    assert!(
        cells(buy)[0]["onlyDepth"] == 0 && buy["source"] == "merchant",
        "Händler: {buy}"
    );
}

/// AC-08: Die 7-Stufen-Fixture liefert 7 Spalten, ohne feste 5 im Code.
#[test]
fn hub_plan_sieben_stufen() {
    let rep = plan_of(&read("testdata/hub/hub7.json"));
    assert!(
        rep["parseError"] == ""
            && rep["findings"].as_array().expect("findings").is_empty()
            && rep["matrix"]["levels"] == 7,
        "{rep}"
    );
    for r in rep["matrix"]["rows"].as_array().expect("rows") {
        assert_eq!(cells(r).len(), 7, "{}", r["key"]);
    }
    let armory = row(&rep["matrix"], "armory@40");
    assert!(armory["hubLevel"] == 7 && cells(armory)[6]["available"] == true && cells(armory)[5]["available"] == false);
    let zone = row(&rep["matrix"], "zone1");
    assert!(
        zone["hubLevel"] == 1 && cells(zone)[0]["offsetUnits"] == 60 && zone["towerUnits"] == 52,
        "{zone}"
    );
}

#[test]
fn hub_plan_befunde_fehler_und_arten() {
    for bad in [
        "{kaputt",
        r#"{"levels": "fünf"}"#,
        r#"{"levels": [], "sites": [{"kind": "armory", "hubLevel": 1.5}]}"#,
    ] {
        let rep = plan_of(bad);
        assert!(
            rep["parseError"] != "" && rep["matrix"]["rows"].as_array().expect("rows").is_empty(),
            "{bad}: {rep}"
        );
    }
    let raw = read("data/hub.json").replacen(r#""kind": "workshop""#, r#""kind": "smithy""#, 1);
    let rep = plan_of(&raw);
    let has_required = rep["findings"]
        .as_array()
        .expect("findings")
        .iter()
        .any(|f| f["rule"] == k3c_sim::hub::validate::RULE_REQUIRED);
    assert!(
        rep["parseError"] == "" && !rep["matrix"]["rows"].as_array().expect("rows").is_empty() && has_required,
        "{rep}"
    );
    assert!(
        rep["notes"].as_array().expect("notes").is_empty(),
        "Hinweise {}",
        rep["notes"]
    );
    let kinds: Vec<&str> = rep["kinds"]
        .as_array()
        .expect("kinds")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    for kind in ["armory", "nursery", "healer", "storage", "workshop"] {
        assert!(kinds.contains(&kind), "Art {kind} fehlt in {kinds:?}");
    }
    for kind in ["castle", "wall", "tower"] {
        assert!(!kinds.contains(&kind), "Art {kind} darf kein Platz sein: {kinds:?}");
    }
}

/// Der Mock der Workbench (tools/k3c-studio/frontend/src/api/hubFixture.json, seit WZ63 im Studio; Go liest noch den alten Pfad) ist die Antwort für die 7-Stufen-Fixture: Ändert sich das
/// Matrix-Modell, muss die Datei neu erzeugt werden, sonst laufen die Frontend-Tests gegen ein veraltetes Format.
#[test]
fn hub_fixture_des_mocks_ist_die_antwort_der_engine() {
    let fixture: Value =
        serde_json::from_str(&read("tools/k3c-studio/frontend/src/api/hubFixture.json")).expect("Fixture");
    let plan = &fixture["plan"];
    let raw = read("testdata/hub/hub7.json").replace("\r\n", "\n");
    assert_eq!(
        plan["raw"].as_str().expect("raw"),
        raw,
        "hubFixture.json: raw weicht von hub7.json ab"
    );
    let rep = plan_of(&raw);
    for name in ["matrix", "findings", "notes", "kinds"] {
        assert_eq!(
            plan[name], rep[name],
            "hubFixture.json: {name} weicht von sim hub ab (Datei neu erzeugen)"
        );
    }
}

#[test]
fn hub_unbekannte_op() {
    assert_eq!(call(r#"{"op":"fly"}"#, &["hub"]).0, EXIT_USAGE);
}
