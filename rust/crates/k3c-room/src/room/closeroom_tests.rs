//! Tests für `Manager::close_room` (`closeroom.go`, B-581, BD9.1). Go prüft es nur mittelbar (`review_test.go`,
//! `testsave_test.go`: Aufräumen und Test-Stand); hier steht der Ablauf auch für sich: speichern, `closed` melden, Raum
//! entfernen, Test-Stand löschen, Fehler wie in Go.
use super::Options;
use super::testutil::{manager, peer};
use crate::fehler::Fehler;

#[test]
fn close_room_speichert_meldet_closed_und_gibt_den_code_frei() {
    let (mut m, _) = manager();
    let (x, mut xi) = peer(1);
    let code = m
        .create("xbox", x, "familie", true, 0, &[0], Options::default())
        .expect("create");
    assert_eq!(m.close_room("familie"), Ok(()));
    assert!(xi.has("closed"), "das Gerät bekommt room_closed");
    assert!(m.rooms.is_empty() && m.room(&code).is_none(), "Raum bleibt im Manager");
    assert!(
        m.store.data.contains_key("familie"),
        "Spielstand wurde nicht gespeichert"
    );
    assert!(
        m.log
            .lines
            .iter()
            .any(|l| l.contains("🛑 Raum schließt") && l.contains("grund=")),
        "{:?}",
        m.log.lines
    );
    let (y, _yi) = peer(2);
    m.create("xbox", y, "familie", false, 0, &[0], Options::default())
        .expect("der Stand lässt sich wieder öffnen");
}

#[test]
fn close_room_loescht_test_staende() {
    let (mut m, _) = manager();
    let (x, _xi) = peer(1);
    m.create("xbox", x, "test-abc", true, 0, &[0], Options::default())
        .expect("create");
    assert_eq!(m.close_room("test-abc"), Ok(()));
    assert!(!m.store.data.contains_key("test-abc"), "Test-Stand nicht gelöscht");
}

#[test]
fn close_room_lehnt_ab() {
    let (mut m, _) = manager();
    assert_eq!(m.close_room("../x"), Err(Fehler::BadRequest), "ungültiger Name");
    assert_eq!(m.close_room("nirgends"), Err(Fehler::RoomNotFound), "kein Raum offen");
    assert!(
        m.log
            .lines
            .iter()
            .any(|l| l.contains("🚫 Raum schließen abgelehnt, kein Raum offen") && l.contains("save=nirgends")),
        "{:?}",
        m.log.lines
    );
}
