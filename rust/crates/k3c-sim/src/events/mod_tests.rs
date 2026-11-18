//! Go: `events_test.go` (F3.1, F3.2, B-139): Aufzeichnung, Schlüssel-Reihenfolge, Ort-Rundung und die Obergrenze je Tick.
//! Tests, die `step` über Stufen mit Gegnern, Bau oder Krone brauchen, stehen in den Gruppen, die das Verhalten liefern.

use serde_json::{Value, json};

use super::{MAX_EVENTS_PER_TICK, build_progress_event, cap_events, coin_give_event, emit, hit_event, unit_x};
use crate::economy::PayTarget;
use crate::state::common::event;
use crate::state::{Event, Player, Projectile, Site, World};

fn kinds(w: &World) -> Vec<Value> {
    w.events.iter().map(|e| e["type"].clone()).collect()
}

#[test]
fn unit_x_rundet_auf_eine_nachkommastelle() {
    assert_eq!(unit_x(7.25), 7.3);
    assert_eq!(unit_x(-7.25), -7.3, "wie math.Round: weg von null");
    assert_eq!(unit_x(3.04), 3.0);
    assert_eq!(unit_x(0.0), 0.0);
}

#[test]
fn emit_setzt_den_typ_und_haengt_an() {
    let mut w = World::default();
    emit(&mut w, "dusk", Event::new());
    emit(&mut w, "wave", event(json!({"wave": 2})));
    assert_eq!(kinds(&w), vec![json!("dusk"), json!("wave")]);
    assert_eq!(w.events[1]["wave"], json!(2));
    let keys: Vec<&String> = w.events[1].keys().collect();
    assert_eq!(keys, ["type", "wave"], "Schlüssel sortiert wie Gos map[string]any");
}

#[test]
fn hit_und_arrow_tragen_gerundete_orte() {
    let mut w = World::default();
    hit_event(&mut w, "player", 1, 12.34, 3.0);
    let p = Projectile {
        target_id: 9,
        x: 5.55,
        team: "enemy".into(),
        ..Default::default()
    };
    super::arrow_event(&mut w, &p, 4);
    assert_eq!(
        Value::Object(w.events[0].clone()),
        json!({"type": "hit", "x": 12.3, "target": "player", "id": 1, "damage": 3.0})
    );
    assert_eq!(
        Value::Object(w.events[1].clone()),
        json!({"type": "arrow", "from": 4, "to": 9, "x": 5.6, "team": "enemy"})
    );
}

#[test]
fn muenze_geben_nennt_das_ziel() {
    let mut w = World::default();
    w.players.push(Player {
        index: 1,
        x: 8.04,
        ..Default::default()
    });
    for (t, to) in [
        (PayTarget::Site(3), "site"),
        (PayTarget::Troop(3), "recruit"),
        (PayTarget::Sword(0), "offer"),
        (PayTarget::Node(3), "mark"),
    ] {
        w.events.clear();
        coin_give_event(&mut w, 0, &t);
        assert_eq!(
            Value::Object(w.events[0].clone()),
            json!({"type": "coinGive", "player": 1, "x": 8.0, "to": to})
        );
    }
}

#[test]
fn baufortschritt_nur_beim_ueberschreiten_von_25_50_75_prozent() {
    let mut w = World::default();
    w.sites.push(Site {
        id: 5,
        kind: "wall".into(),
        x: 10.0,
        build_progress: 0.3,
        ..Default::default()
    });
    build_progress_event(&mut w, 5, 0.2);
    build_progress_event(&mut w, 5, 0.26); // schon über 25 %: nichts
    w.sites[0].build_progress = 1.0;
    build_progress_event(&mut w, 5, 0.9); // 100 % ist `built`
    build_progress_event(&mut w, 99, 0.0); // unbekannter Platz
    assert_eq!(w.events.len(), 1);
    assert_eq!(
        Value::Object(w.events[0].clone()),
        json!({"type": "buildProgress", "site": 5, "kind": "wall", "x": 10.0, "percent": 25})
    );
}

/// Go: `TestEventsObergrenzePrioritaet`: Priorität auch dann, wenn Tod und Bau am Ende der Liste stehen; Reihenfolge der
/// behaltenen bleibt.
#[test]
fn obergrenze_haelt_prioritaet_und_reihenfolge() {
    let mut w = World::default();
    for i in 0..MAX_EVENTS_PER_TICK + 5 {
        w.events.push(event(json!({"type": "hit", "id": i})));
    }
    for t in ["built", "crownFallen", "buildProgress"] {
        w.events.push(event(json!({ "type": t })));
    }
    let total = w.events.len();
    cap_events(&mut w);
    let n = w.events.len();
    assert_eq!(n, MAX_EVENTS_PER_TICK);
    assert_eq!(w.events_dropped, (total - n) as i64);
    let tail: Vec<&str> = w.events[n - 3..].iter().filter_map(|e| e["type"].as_str()).collect();
    assert_eq!(tail, ["built", "crownFallen", "buildProgress"]);
    assert_eq!(w.events[0]["id"], json!(0));
    w.events = vec![event(json!({"type": "hit"}))];
    cap_events(&mut w);
    assert!(
        w.events_dropped == 0 && w.events.len() == 1,
        "unter K: nichts verworfen"
    );
}
