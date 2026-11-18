//! Tests des Session-Gerüsts (Go: die Teile von `local_test.go` ohne Takt: `TestJoinLeaveRejects`, `TestJoinLeave`; der Takt
//! steht in `step_tests.rs`).
//!
//! `Manager::create` und `Manager::load` brauchen die Standard-Welt, und `standard_world` bricht bis RU19
//! (`decode_world_story`) mit `unimplemented!` ab. Die Tests starten darum über `start_test_game` mit einer Insel von Hand
//! (`testutil.rs`); Tests, die wirklich `new_game` oder `load` brauchen, sind mit `ignore` auf RU19 markiert. Auch das
//! Speichern (`Island::to_save`) bricht bis RU18 (`new_hub_site`) ab; die Tests setzen darum `Island::over`: Ein Raum
//! einer verlorenen Insel speichert nicht (Go: B-344, `Room::game_over`), alles andere am Beenden läuft echt.

use super::session::{DEVICE_ID, Session};
use super::testutil::{TWO_DEVICES, TempDir, start_test_game, test_session};
use super::watched::lock;
use crate::view::frame::CueKind;
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::session::{NewGame, Session as _};

fn game(name: &str, devices: &[DeviceID]) -> NewGame {
    NewGame {
        name: name.into(),
        devices: devices.to_vec(),
        ..NewGame::default()
    }
}

/// Zahl der Plätze, die der Raum dem Peer zuletzt meldete.
fn peer_seats(s: &Session) -> usize {
    s.peer.as_ref().map_or(0, |p| lock(p).seats.len())
}

#[test]
fn start_pause_close() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(!s.active());
    start_test_game(&mut s, "couch", &TWO_DEVICES[..1]);
    assert!(s.active());
    assert_eq!(peer_seats(&s), 1);
    s.pause(true);
    s.pause(true); // ein zweites Pause ändert nichts
    assert!(s.paused && s.with_room(|r, _| r.device_paused(DEVICE_ID)) == Some(true));
    s.pause(false);
    assert_eq!(s.with_room(|r, _| r.device_paused(DEVICE_ID)), Some(false));
    let kinds: Vec<CueKind> = s.cues.iter().map(|c| c.kind).collect();
    assert_eq!(kinds, [CueKind::Pause, CueKind::Resume]);
    s.with_room(|r, _| r.isl.over = true); // kein Speichern, siehe Kopf der Datei
    s.close().expect("beenden");
    assert!(!s.active() && s.code.is_empty() && s.m.is_none() && s.peer.is_none());
    assert!(
        !dir.path().join("couch.json").exists(),
        "verlorene Insel: nicht gespeichert"
    );
    s.close().expect("zweites Beenden ist leer");
    s.pause(true); // ohne Spiel: nichts
    assert!(!s.paused);
}

#[test]
fn start_with_two_devices() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "zu-zweit", &TWO_DEVICES);
    assert_eq!(s.devices(), TWO_DEVICES);
    assert_eq!(peer_seats(&s), 2);
    let seats = s.with_room(|r, _| r.seats(DEVICE_ID)).expect("Raum");
    assert_eq!(seats.iter().map(|t| t.slot).collect::<Vec<_>>(), [0, 1]);
    assert_ne!(seats[0].monarch, seats[1].monarch, "zwei Monarchen");
}

// Fehler vor dem Anlegen des Raums; die Meldungen stehen wörtlich in Go.
#[test]
fn new_game_rejects() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    let three = [TWO_DEVICES[0], TWO_DEVICES[1], DeviceID::new(DeviceKind::Gamepad, 1)];
    let bad_name = "Ungültiger Name oder ungültige Optionen (Name: a–z, 0–9, -, höchstens 32 Zeichen)";
    let cases: [(NewGame, &str); 4] = [
        (game("x", &[]), "1 bis 2 Geräte nötig"),
        (game("x", &three), "1 bis 2 Geräte nötig"),
        (
            game("x", &[TWO_DEVICES[0], TWO_DEVICES[0]]),
            "Ein Gerät kann nur einen Spieler steuern",
        ),
        (game("Groß!", &TWO_DEVICES[..1]), bad_name),
    ];
    for (g, want) in cases {
        assert_eq!(s.new_game(g).expect_err("abgelehnt").to_string(), want);
        assert!(!s.active());
    }
}

// Der Name eines vorhandenen Spielstands ist vergeben (der Raum legt vor der Welt an: kein Zugriff auf die Standard-Welt).
#[test]
fn new_game_name_taken() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    std::fs::write(dir.path().join("doppelt.json"), r#"{"campaignId":"a","version":1}"#).expect("schreiben");
    let err = s.new_game(game("doppelt", &TWO_DEVICES[..1])).expect_err("vergeben");
    assert_eq!(err.to_string(), "Spielstand „doppelt“ gibt es schon");
    assert!(!s.active());
}

#[test]
fn free_name_counts_up() {
    let dir = TempDir::new();
    let s = test_session(&dir);
    assert_eq!(s.free_name(), "spiel-1");
    std::fs::write(dir.path().join("spiel-1.json"), "{}").expect("schreiben");
    std::fs::write(dir.path().join("spiel-3.json"), "{}").expect("schreiben");
    assert_eq!(s.free_name(), "spiel-2");
}

#[test]
fn save_without_game_and_after_game_over() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert_eq!(s.save().expect_err("kein Spiel").to_string(), "Es läuft kein Spiel");
    start_test_game(&mut s, "sichern", &TWO_DEVICES);
    s.with_room(|r, _| r.isl.over = true);
    let err = s.save().expect_err("Insel verloren");
    assert_eq!(
        err.to_string(),
        "Insel verloren, der Spielstand bleibt wie zuletzt gesichert"
    );
    assert!(s.active(), "Speichern beendet das Spiel nicht");
    assert_eq!(lock(&s.results).count(), 0);
}

#[test]
fn save_now() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "sichern", &TWO_DEVICES);
    s.save().expect("speichern");
    assert!(dir.path().join("sichern.json").is_file());
    assert_eq!(lock(&s.results).count(), 1);
    assert_eq!(lock(&s.results).take(), [None], "gespeichert ohne Fehler");
    s.close().expect("beenden");
    assert!(dir.path().join("sichern.json").is_file());
}

// Ein Spielstand aus der Session lädt mit den Geräten des letzten Spiels.
#[test]
fn load_game() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    s.new_game(game("laden", &TWO_DEVICES)).expect("neues Spiel");
    s.close().expect("beenden");
    s.load("laden", &[]).expect("laden mit den Geräten des letzten Spiels");
    assert_eq!(s.devices(), TWO_DEVICES);
    assert_eq!(s.name, "laden");
    let err = s.load("gibt-es-nicht", &[]).expect_err("fehlt");
    assert_eq!(err.to_string(), "Spielstand „gibt-es-nicht“ gibt es nicht");
    assert!(!s.active(), "ein gescheiterter Start lässt kein Spiel zurück");
}

// Ohne Spiel gibt es keinen Beitritt; der letzte Spieler bleibt.
#[test]
fn join_leave_rejects() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.join(TWO_DEVICES[0]).is_err(), "Join ohne Spiel");
    assert!(s.leave(0).is_err(), "Leave ohne Spiel");
    start_test_game(&mut s, "couch", &TWO_DEVICES[..1]);
    assert!(s.leave(0).is_err(), "letzter Spieler ging");
    let err = s.leave(1).expect_err("Spieler 2 spielt nicht");
    assert_eq!(err.to_string(), "Spieler 2 spielt nicht");
}

// Spieler kommen und gehen, höchstens zwei (ohne Frames; die Frames nach Join und Leave prüft RU27.2).
#[test]
fn join_leave() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    start_test_game(&mut s, "couch", &TWO_DEVICES[..1]);
    assert_eq!(s.join(TWO_DEVICES[1]).expect("Join"), 1);
    assert_eq!(s.join(TWO_DEVICES[1]).expect("zweites Join desselben Geräts"), 1);
    let third = DeviceID::new(DeviceKind::Gamepad, 3);
    assert_eq!(
        s.join(third).expect_err("dritter Spieler").to_string(),
        "Höchstens 2 Spieler"
    );
    assert_eq!(peer_seats(&s), 2);
    assert_eq!(s.devices(), TWO_DEVICES);
    s.leave(1).expect("Leave");
    assert_eq!(peer_seats(&s), 1);
    assert_eq!(s.devices(), &TWO_DEVICES[..1]);
    let kinds: Vec<(CueKind, i64)> = s.cues.iter().map(|c| (c.kind, c.player)).collect();
    assert_eq!(kinds, [(CueKind::Joined, 1), (CueKind::Left, 1)]);
    // Ein neuer Beitritt bekommt den freien Platz wieder.
    assert_eq!(s.join(third).expect("Join nach Leave"), 1);
    assert_eq!(s.last, [TWO_DEVICES[0], third], "Geräte des letzten Spiels");
}

#[test]
fn route_list_follows_game() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.route_list().is_empty(), "ohne Spiel keine Route");
    start_test_game(&mut s, "route", &TWO_DEVICES[..1]);
    let route = s.route_list().to_vec();
    assert!(!route.is_empty(), "Standard-Welt hat Inseln");
    assert!(
        route.iter().all(|i| i.etappe >= 1 && i.stages >= 1),
        "Etappe und Stufen: {route:?}"
    );
    s.close().expect("beenden");
    assert!(s.route_list().is_empty(), "nach dem Spiel keine Route");
}
