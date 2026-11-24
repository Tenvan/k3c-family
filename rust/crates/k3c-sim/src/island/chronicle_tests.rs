//! Chronik der Insel (GS3.7, B-966, AC-01): Eintrag beim Ablegen, Reihenfolge und Mitziehen, Speichern und Laden,
//! ungültige Stände und ältere Stände ohne Chronik.

use super::chronicle::ChronicleEntry;
use super::switch_testutil::{gate_island, test_islands};
use super::testutil::{ISLAND_SPEED, test_data};
use super::{ISLAND_SAVE_VERSION, Island, SaveTables, WorldDef, from_island_save_in, next_island, parse_island_save};
use crate::state::links::values_of;
use k3c_data::Data;

/// Zwei Inseln, zwei Monarchen am Steg, Endboss der ersten besiegt: der Wechsel zur zweiten.
fn wechsel(data: &Data) -> (WorldDef, Island, Island) {
    let (world, _two) = test_islands(data);
    let mut isl = gate_island(data, &world, 2);
    isl.stages[0].cycle.day = 4;
    let def = isl.next_def().expect("Ziel");
    let next = next_island(data, &isl, &def).expect("nächste Insel");
    (world, isl, next)
}

fn reload(data: &Data, world: &WorldDef, json: &str) -> Result<Island, String> {
    let defaults = values_of(world).expect("Wertesatz");
    let t = SaveTables {
        data,
        defaults: &defaults,
        standard_world: world,
    };
    let s = parse_island_save(json, &t).map_err(|e| e.to_string())?;
    from_island_save_in(&t, &s, ISLAND_SPEED, world).map_err(|e| e.to_string())
}

#[test]
fn ablegen_haelt_insel_tage_und_monarchen_fest() {
    let data = test_data();
    let (_, isl, next) = wechsel(&data);
    let want = ChronicleEntry {
        island: isl.island_id.clone(),
        days: 4,
        monarchs: 2,
    };
    assert_eq!(isl.chronicle_entry(), want);
    assert_eq!(
        next.chronicle,
        std::slice::from_ref(&want),
        "genau ein Eintrag, gleiche Reihenfolge wie `completed`"
    );
    assert_eq!(next.completed, std::slice::from_ref(&isl.island_id));
    assert!(isl.chronicle.is_empty(), "die alte Insel bleibt unverändert");
}

#[test]
fn ohne_besiegten_endboss_kein_eintrag() {
    let data = test_data();
    let (world, _) = test_islands(&data);
    let mut isl = gate_island(&data, &world, 1);
    isl.endboss_defeated = false;
    let def = isl.next_def().expect("Ziel");
    let next = next_island(&data, &isl, &def).expect("nächste Insel");
    assert!(next.completed.is_empty() && next.chronicle.is_empty());
}

#[test]
fn chronik_uebersteht_speichern_und_laden() {
    let data = test_data();
    let (world, isl, next) = wechsel(&data);
    let s = next.to_save("2026-11-24T00:00:00Z").expect("Spielstand");
    assert_eq!((s.version, ISLAND_SAVE_VERSION), (14, 14));
    let json = serde_json::to_value(&s).expect("json");
    assert_eq!(
        json["chronicle"],
        serde_json::json!([{"islandId": isl.island_id, "days": 4, "monarchs": 2}])
    );
    let loaded = reload(&data, &world, &json.to_string()).expect("laden");
    assert_eq!(loaded.chronicle, next.chronicle);
    assert_eq!(loaded.completed, next.completed);
    // zweiter Durchgang: byte-gleich
    let again = serde_json::to_value(loaded.to_save("2026-11-24T00:00:00Z").expect("Spielstand")).expect("json");
    assert_eq!(again, json);
}

#[test]
fn ohne_abgeschlossene_insel_fehlt_das_feld() {
    let data = test_data();
    let (world, _) = test_islands(&data);
    let isl = gate_island(&data, &world, 1);
    let json = serde_json::to_value(isl.to_save("x").expect("Spielstand")).expect("json");
    assert!(json.get("chronicle").is_none());
}

#[test]
fn eintrag_ohne_abgeschlossene_insel_oder_doppelt_wird_abgelehnt() {
    let data = test_data();
    let (world, isl, next) = wechsel(&data);
    let mut json = serde_json::to_value(next.to_save("x").expect("Spielstand")).expect("json");
    json["chronicle"] = serde_json::json!([{"islandId": "nirgends", "days": 1, "monarchs": 1}]);
    let e = reload(&data, &world, &json.to_string()).expect_err("unbekannte Insel");
    assert!(e.starts_with("spielstand:") && e.contains("nicht abgeschlossen"), "{e}");
    let one = serde_json::json!({"islandId": isl.island_id, "days": 4, "monarchs": 2});
    json["chronicle"] = serde_json::json!([one, one]);
    let e = reload(&data, &world, &json.to_string()).expect_err("doppelt");
    assert!(e.contains("doppelt"), "{e}");
}

#[test]
fn staende_der_version_12_laden_ohne_chronik() {
    let data = test_data();
    let (world, _, next) = wechsel(&data);
    let mut json = serde_json::to_value(next.to_save("x").expect("Spielstand")).expect("json");
    json["version"] = serde_json::json!(12);
    json.as_object_mut().expect("Objekt").remove("chronicle");
    let loaded = reload(&data, &world, &json.to_string()).expect("Version 12 laden");
    assert!(loaded.chronicle.is_empty(), "ältere Stände: kein Eintrag");
    assert_eq!(loaded.completed.len(), 1, "abgeschlossene Inseln bleiben");
}

#[test]
fn nur_lebende_gekroente_monarchen_zaehlen() {
    let data = test_data();
    let (world, _) = test_islands(&data);
    let mut isl = gate_island(&data, &world, 3);
    isl.stages[0].players[1].free = true; // frei: zählt nicht
    super::switch_testutil::fell(&mut isl.stages[0], 2); // Krone am Boden: zählt nicht
    assert_eq!(isl.chronicle_entry().monarchs, 1);
}

#[test]
fn zweiter_wechsel_haengt_den_naechsten_eintrag_an() {
    let data = test_data();
    let (w, _, _) = super::switch_testutil::etappen_welt(&data);
    let mut isl = gate_island(&data, &w, 2);
    isl.stages[0].cycle.day = 4;
    let first = isl.chronicle_entry();
    let def = isl.next_def().expect("Ziel");
    let mut second = next_island(&data, &isl, &def).expect("zweite Insel");
    // auf der zweiten Insel ist der Endboss besiegt und eine der beiden Folge-Inseln gewählt
    second.endboss_defeated = true;
    second.stages[0].cycle.day = 9;
    let def2 = second.next_def().expect("nächstes Ziel");
    let third = next_island(&data, &second, &def2).expect("dritte Insel");
    assert_eq!(third.chronicle.len(), 2, "{:?}", third.chronicle);
    assert_eq!(third.chronicle[0], first);
    assert_eq!(
        (third.chronicle[1].island.as_str(), third.chronicle[1].days),
        (second.island_id.as_str(), 9)
    );
    assert_eq!(third.completed, [first.island.clone(), second.island_id.clone()]);
}
