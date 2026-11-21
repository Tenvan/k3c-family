//! Tests aus `metrics_test.go` (S2.3, B-150): Spielmetrik-Report je Raumlauf: Inhalt, kein Einfluss auf den Spielverlauf,
//! keine Namen. Die Ablage ist ein `SessionStore` im Speicher (Go `sessionSink`), den der Test über einen geteilten Puffer
//! ausliest, weil der Manager ihn besitzt.
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use k3c_sim::state::{Enemy, PlayerCommand, Troop};
use serde_json::{Value, json};

use super::save_testutil::{create_with, drop_device, leave};
use super::testutil::{TestManager, join_room, manager, peer, rm, ticks, wait};
use super::types::EMPTY_FOR_SECS;
use super::{Options, SessionStore};
use crate::fehler::Fehler;

/// Eine Ablage im Speicher; mit `fail` scheitert sie (Go `sessionSink`).
#[derive(Clone, Default)]
struct Sink {
    data: Rc<RefCell<Vec<Vec<u8>>>>,
    fail: Option<String>,
}

impl SessionStore for Sink {
    fn store_session(&mut self, d: &[u8]) -> Result<String, Fehler> {
        if let Some(f) = &self.fail {
            return Err(Fehler::Io(f.clone()));
        }
        self.data.borrow_mut().push(d.to_vec());
        Ok("mem".to_string())
    }
}

/// Lange Geräte-IDs, damit der Test erkennt, ob nur das Kürzel (8 Zeichen) im Report steht.
const XBOX_ID: &str = "xbox-wohnzimmer-0001";
const HANDY_ID: &str = "handy-kinderzimmer-02";

/// Lässt einen Bauplatz der Stufe 0 im nächsten Schritt fertig werden (Go `finishBuild`).
fn finish_build(m: &mut TestManager, code: &str) {
    let w = &mut rm(m, code).isl.stages[0];
    assert!(!w.sites.is_empty(), "Stufe 0 ohne Bauplatz");
    let id = 90000;
    let s = &mut w.sites[0];
    (s.state, s.hp, s.build_progress, s.worker_id) = ("waitingWorker".into(), 0.0, 0.9999, Some(id));
    let (x, site_id) = (s.x, s.id);
    w.troops.push(Troop {
        id,
        kind: "citizen".into(),
        x,
        anchor_x: x,
        hp: 1.0,
        max_hp: 1.0,
        job: Some(k3c_sim::state::Job {
            r#type: "build".into(),
            site_id,
            ..Default::default()
        }),
        ..Troop::default()
    });
}

/// Setzt neben Monarch `idx` (ohne Gold) einen Goblin: sein Treffer lässt die Krone fallen (Go `downByGoblin`).
fn down_by_goblin(m: &mut TestManager, code: &str, idx: i64) {
    let r = rm(m, code);
    let stage = r.isl.stage_of(idx).expect("Stufe");
    let w = &mut r.isl.stages[stage];
    let Some(p) = w.players.iter_mut().find(|p| p.index == idx) else {
        panic!("Monarch {idx} fehlt");
    };
    p.gold = 0;
    let x = p.x;
    w.enemies.push(Enemy {
        id: 90001,
        kind: "goblin".into(),
        x: x - 0.5,
        hp: 50.0,
        max_hp: 50.0,
        damage: 5.0,
        speed: 1.0,
        range: 1.0,
        home_x: x - 60.0,
        ..Enemy::default()
    });
}

/// Ändert die Beschleunigung des Zyklus, ohne die Stelle im Zyklus zu verschieben (Zeit mal Tempo bleibt).
fn set_cycle_speed(m: &mut TestManager, code: &str, v: f64) {
    for w in &mut rm(m, code).isl.stages {
        w.time = w.time * w.cycle_speed / v;
        w.cycle_speed = v;
    }
}

/// Tickt mit beschleunigtem Zyklus, bis Nacht ist (Go `tickToNight`).
fn tick_to_night(m: &mut TestManager, code: &str) {
    set_cycle_speed(m, code, 1000.0);
    for _ in 0..=300 {
        if rm(m, code).isl.stages[0].cycle.phase == "night" {
            return;
        }
        ticks(m, code, 1);
    }
    panic!("keine Nacht nach 10 s");
}

/// Tickt, bis Stufe 0 den Tag `day` erreicht hat (Go `tickToDay`).
fn tick_to_day(m: &mut TestManager, code: &str, day: i64) {
    for i in 0.. {
        if rm(m, code).isl.stages[0].cycle.day >= day {
            return;
        }
        assert!(i <= 3000, "Tag {day} nicht erreicht");
        ticks(m, code, 1);
    }
}

/// Der Lauf aus Test (a): zwei Geräte, ein Bau, eine Nacht mit einem Sturz durch einen Goblin, ein Abbruch. Die Antwort
/// ist der Report als Text und als JSON-Baum.
fn metrik_run() -> (String, Value, String) {
    let (mut m, _) = manager();
    let sink = Sink::default();
    m.sessions = Some(Box::new(sink.clone()));
    let (code, _xi) = create_with(&mut m, XBOX_ID, 1, "metrik-seed", true, Options::default());
    let _hi = join_room(&mut m, HANDY_ID, 2, &code, &[0]);
    finish_build(&mut m, &code);
    ticks(&mut m, &code, 3);
    tick_to_night(&mut m, &code); // Zyklus mal 1000 wie in S2.2
    set_cycle_speed(&mut m, &code, 1.0); // die Nacht dauert, bis der Goblin trifft
    down_by_goblin(&mut m, &code, 1);
    for i in 0.. {
        if rm(&mut m, &code).met.as_ref().is_some_and(|met| !met.deaths.is_empty()) {
            break;
        }
        assert!(i <= 300, "Monarch 1 fällt nicht");
        ticks(&mut m, &code, 1);
    }
    set_cycle_speed(&mut m, &code, 1000.0);
    tick_to_day(&mut m, &code, 2);
    drop_device(&mut m, &code, HANDY_ID, 2);
    m.close();
    let data = sink.data.borrow();
    assert_eq!(data.len(), 1, "{} Reports, erwartet 1", data.len());
    let text = String::from_utf8(data[0].clone()).expect("UTF-8");
    (text.clone(), serde_json::from_str(&text).expect("JSON"), code)
}

// (a) AC-05: ein Report mit allen Feldern aus Schema 1 und den erwarteten Werten.
#[test]
fn metrik_report_nach_raumlauf() {
    let (_, rep, code) = metrik_run();
    for k in [
        "schema",
        "room",
        "startedAt",
        "endedAt",
        "playSeconds",
        "grade",
        "goal",
        "defeat",
        "monarchs",
        "devices",
        "reached",
        "nights",
        "deaths",
        "firstBuildSeconds",
        "goldPerDay",
        "goldEnd",
        "disconnects",
    ] {
        assert!(rep.get(k).is_some(), "Feld {k:?} fehlt");
    }
    assert!(
        rep["schema"] == 1
            && rep["room"] == code.as_str()
            && rep["startedAt"] == "2026-10-01T12:00:00.000Z"
            && rep["endedAt"] == "2026-10-01T12:00:00.000Z"
            && rep["playSeconds"].as_f64().is_some_and(|s| s > 0.0)
            && rep["grade"].as_str().is_some_and(|g| !g.is_empty())
            && rep["monarchs"] == 2,
        "Kopf: {rep}"
    );
    check_run(&rep);
}

/// Prüft die Werte des Laufs aus `metrik_run`: Geräte, Abbruch, Nacht, Bau, Sturz und Gold (Go `checkRun`).
fn check_run(rep: &Value) {
    assert_eq!(
        (&rep["devices"], &rep["disconnects"]),
        (
            &json!(["handy-ki", "xbox-woh"]),
            &json!([{"device": "handy-ki", "count": 1}])
        ),
        "Geräte, Abbrüche"
    );
    assert!(
        rep["nights"][0] == json!({"day": 1, "survived": true}) && rep["reached"]["day"].as_i64() >= Some(2),
        "Nächte {}, erreicht {}",
        rep["nights"],
        rep["reached"]
    );
    assert!(
        rep["firstBuildSeconds"].as_f64().is_some_and(|s| s > 0.0),
        "firstBuildSeconds {}",
        rep["firstBuildSeconds"]
    );
    let deaths = rep["deaths"].as_array().cloned().unwrap_or_default();
    assert!(
        deaths
            .iter()
            .any(|d| d["monarch"] == 1 && d["cause"] == "goblin" && d["phase"] == "night" && d["day"] == 1),
        "Tode {}",
        rep["deaths"]
    );
    let (gpd, end) = (&rep["goldPerDay"], &rep["goldEnd"]);
    assert!(
        gpd[0]["day"] == 2
            && gpd[0]["gold"].as_array().map(Vec::len) == Some(2)
            && end.as_array().map(Vec::len) == Some(2),
        "Gold je Tag {gpd}, am Ende {end}"
    );
}

// (c) AC-05: weder Spielstand-Name noch volle Geräte-ID, nur Index und Kürzel.
#[test]
fn metrik_ohne_namen() {
    let (text, _, _) = metrik_run();
    for bad in ["metrik-seed", XBOX_ID, HANDY_ID, "campaignId"] {
        assert!(!text.contains(bad), "Report enthält {bad:?}");
    }
}

// (b) AC-05: Gleicher Lauf mit und ohne Sammler ergibt nach N Ticks dieselben Bytes aller Stufen.
#[test]
fn metrik_aendert_spielverlauf_nicht() {
    let run = |collect: bool| -> Vec<u8> {
        let (mut m, _) = manager();
        let (x, _xi) = peer(1);
        let code = m
            .create(XBOX_ID, x, "hash", true, 0, &[0, 1], Options::default())
            .expect("create");
        if collect {
            m.sessions = Some(Box::new(Sink::default()));
        } else {
            rm(&mut m, &code).met = None;
        }
        tick_to_night(&mut m, &code);
        for i in 0..300 {
            let cmd = BTreeMap::from([
                (
                    0,
                    PlayerCommand {
                        move_x: 1.0,
                        ..PlayerCommand::default()
                    },
                ),
                (
                    1,
                    PlayerCommand {
                        move_x: f64::from(i % 3 - 1),
                        ..PlayerCommand::default()
                    },
                ),
            ]);
            assert_eq!(rm(&mut m, &code).input(XBOX_ID, 1, &cmd), Ok(()));
            ticks(&mut m, &code, 1);
        }
        serde_json::to_vec(&rm(&mut m, &code).isl.stages).expect("JSON")
    };
    assert!(run(true) == run(false), "Zustand mit Sammler weicht ab");
}

// (d) Ein Raum ohne Tick schreibt keinen Report.
#[test]
fn metrik_ohne_tick_kein_report() {
    let (mut m, _) = manager();
    let sink = Sink::default();
    m.sessions = Some(Box::new(sink.clone()));
    let (_code, _xi) = create_with(&mut m, XBOX_ID, 1, "leer", true, Options::default());
    m.close();
    assert_eq!(sink.data.borrow().len(), 0, "Reports ohne Tick");
}

// (e) Scheitert die Ablage, steht der Fehler im Log und der leere Raum ist nach `EMPTY_FOR_SECS` trotzdem aufgeräumt.
#[test]
fn metrik_ablage_fehler_raum_schliesst() {
    let (mut m, clock) = manager();
    m.sessions = Some(Box::new(Sink {
        fail: Some("platte voll".into()),
        ..Sink::default()
    }));
    let (code, _xi) = create_with(&mut m, XBOX_ID, 1, "voll", true, Options::default());
    ticks(&mut m, &code, 5);
    leave(&mut m, &code, XBOX_ID, 1);
    wait(&mut m, &clock, EMPTY_FOR_SECS + 60);
    assert!(m.rooms().is_empty(), "Raum nach Ablagefehler nicht aufgeräumt");
    let logged = m
        .log
        .lines
        .iter()
        .any(|l| l.contains("Spielmetrik-Report nicht geschrieben") && l.contains("platte voll"));
    assert!(logged, "Log ohne Fehler: {:?}", m.log.lines);
}
