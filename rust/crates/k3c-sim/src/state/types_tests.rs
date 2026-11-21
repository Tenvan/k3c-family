//! JSON-Rundlauf der Zustands-Typen mit Beispielen aus den Sim-Golden (`testdata/golden/sim-*.json.gz`, nur lesen):
//! Wert lesen, in den Rust-Typ laden, wieder schreiben, als JSON-Baum vergleichen.

use k3c_det::golden;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::*;

/// `sim_golden` liest alle Sim-Golden (Name, Datei als Baum).
pub(crate) fn sim_golden() -> Vec<(String, Value)> {
    let dir = golden::testdata().join("golden");
    let mut out = Vec::new();
    for e in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let name = e.unwrap().file_name().to_string_lossy().into_owned();
        if name.starts_with("sim-") && name.ends_with(".json.gz") {
            out.push((name.clone(), golden::read(dir.join(&name))));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// `roundtrip` prüft, dass `v` als `T` geladen und wieder geschrieben denselben Baum ergibt.
fn roundtrip<T: DeserializeOwned + Serialize>(at: &str, v: &Value) {
    let t: T = serde_json::from_value(v.clone()).unwrap_or_else(|e| panic!("{at}: {e}"));
    let d = golden::diff(at, v, &golden::tree(&t));
    assert!(d.is_empty(), "{d}");
}

/// `each` ruft roundtrip für jeden Eintrag der Liste `field` in jedem Snapshot jedes Golden auf; gibt die Zahl zurück.
fn each<T: DeserializeOwned + Serialize>(field: &str) -> usize {
    let mut n = 0;
    for (name, g) in sim_golden() {
        for (i, s) in g["snapshots"].as_array().unwrap().iter().enumerate() {
            for (j, v) in s["world"][field].as_array().into_iter().flatten().enumerate() {
                roundtrip::<T>(&format!("{name} snapshots[{i}].{field}[{j}]"), v);
                n += 1;
            }
        }
    }
    n
}

#[test]
fn player_rundlauf() {
    assert!(each::<Player>("players") > 0);
}

#[test]
fn coin_rundlauf() {
    assert!(each::<Coin>("coins") > 0);
}

#[test]
fn troop_und_job_rundlauf() {
    assert!(each::<Troop>("troops") > 0);
}

#[test]
fn node_rundlauf() {
    assert!(each::<ResourceNode>("nodes") > 0);
}

#[test]
fn site_rundlauf() {
    assert!(each::<Site>("sites") > 0);
}

#[test]
fn enemy_rundlauf() {
    assert!(each::<Enemy>("enemies") > 0);
}

#[test]
fn projectile_pickup_camp_rundlauf() {
    assert!(each::<Projectile>("projectiles") > 0);
    assert!(each::<Pickup>("pickups") > 0);
    assert!(each::<Camp>("camps") > 0);
}

#[test]
fn storm_hazard_drop_rundlauf() {
    // Nicht jedes Golden hat Stürme, Flächen oder Ausrüstung am Boden; die Typen laufen dann leer durch.
    each::<Storm>("storms");
    each::<Hazard>("hazards");
    each::<ItemDrop>("drops");
}

#[test]
fn spawn_queue_rundlauf() {
    each::<QueuedSpawn>("spawnQueue");
}

#[test]
fn einzelteile_rundlauf() {
    let mut travel = 0;
    for (name, g) in sim_golden() {
        for (i, s) in g["snapshots"].as_array().unwrap().iter().enumerate() {
            let w = &s["world"];
            roundtrip::<Castle>(&format!("{name} {i}.castle"), &w["castle"]);
            roundtrip::<CycleInfo>(&format!("{name} {i}.cycle"), &w["cycle"]);
            roundtrip::<k3c_data::hub::Stock>(&format!("{name} {i}.stock"), &w["stock"]);
            if !w["travel"].is_null() {
                roundtrip::<Travel>(&format!("{name} {i}.travel"), &w["travel"]);
                travel += 1;
            }
        }
        for run in g["inputs"].as_array().unwrap() {
            for c in run["commands"].as_array().unwrap() {
                roundtrip::<PlayerCommand>(&format!("{name} inputs"), c);
            }
        }
    }
    let _ = travel; // Reise kommt in den Golden womöglich nie vor
}

#[test]
fn event_ist_sortierte_map() {
    let e: Event = serde_json::from_str(r#"{"type":"night","day":1,"player":0}"#).unwrap();
    assert_eq!(
        serde_json::to_string(&e).unwrap(),
        r#"{"day":1,"player":0,"type":"night"}"#
    );
}

#[test]
fn omitempty_und_skip() {
    let p = Player {
        protected: true,
        hits: 3,
        ..Default::default()
    };
    let v = golden::tree(&p);
    let keys: Vec<&String> = v.as_object().unwrap().keys().collect();
    assert!(
        !keys
            .iter()
            .any(|k| ["free", "skills", "shield", "crown", "banner", "protected", "hits"].contains(&k.as_str()))
    );
    assert_eq!(v["payKey"], Value::Null);
}

#[test]
fn new_id_zaehlt_wie_go() {
    let mut w = World {
        next_id: 7,
        ..Default::default()
    };
    assert_eq!((w.new_id(), w.new_id(), w.next_id), (7, 8, 9));
}

#[test]
fn listen_behalten_die_reihenfolge() {
    let v: Value = serde_json::json!({"players": [{"id": 9}, {"id": 3}, {"id": 5}]});
    let w: World = serde_json::from_value(v).unwrap();
    let ids: Vec<i64> = w.players.iter().map(|p| p.id).collect();
    assert_eq!(ids, [9, 3, 5]);
}

/// FG1.1: Im Zustand heißen Bürger `vagrant` (wild) und `peasant` (im Dienst); beim Lesen wird wieder `citizen`.
#[test]
fn buerger_im_zustand_wie_go_und_zurueck() {
    use super::{CITIZEN, World};
    let mut w = World::default();
    let mut wild = Troop::default();
    wild.go_wild();
    let mut serving = Troop::default();
    serving.serve();
    w.troops = vec![
        wild,
        serving,
        Troop {
            kind: "archer".into(),
            ..Troop::default()
        },
    ];
    let v = serde_json::to_value(&w).expect("Zustand");
    let kinds: Vec<&str> = v["troops"]
        .as_array()
        .expect("Truppen")
        .iter()
        .map(|t| t["kind"].as_str().unwrap_or(""))
        .collect();
    assert_eq!(kinds, ["vagrant", "peasant", "archer"]);
    assert!(v["troops"][0].get("wild").is_none(), "wild steckt im Namen");
    let back: World = serde_json::from_value(v).expect("lesen");
    assert!(back.troops[0].is_wild() && back.troops[1].serving() && back.troops[2].kind != CITIZEN);
}
