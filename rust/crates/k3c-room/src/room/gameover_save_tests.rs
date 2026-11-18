//! Tests aus `gameover_save_test.go` (KB2.2, B-344, KB2/AC-02): Nach „Komplett verloren“ (`Island::over`) schreibt der
//! Raum keinen Spielstand mehr; der Stand von vor dem Game Over bleibt bei Verlassen, Diagnose-Sicherung, Aufräumen und
//! Herunterfahren Byte für Byte gleich.
use super::save_testutil::{create_with, drop_device, leave, logged};
use super::testutil::{join_room, manager, rm, ticks, wait};
use super::types::{EMPTY_FOR_SECS, Options};
use crate::fehler::Fehler;

fn defeat_stage() -> Options {
    Options {
        defeat: "stage".into(),
        ..Options::default()
    }
}

#[test]
fn game_over_laesst_spielstand_unveraendert() {
    let (mut m, clock) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "verloren", true, defeat_stage());
    ticks(&mut m, &code, 30);
    leave(&mut m, &code, "xbox", 1);
    let prev = m.store.data["verloren"].clone();
    let writes = m.store.calls;

    let _hi = join_room(&mut m, "handy", 2, &code, &[0]);
    let _xi = join_room(&mut m, "xbox", 3, &code, &[0]);
    ticks(&mut m, &code, 30);
    rm(&mut m, &code).isl.over = true;
    ticks(&mut m, &code, 30);
    leave(&mut m, &code, "handy", 2); // Verlassen bei verbliebenem Gerät
    assert_eq!(
        m.with_room(&code, |r, env| r.save_now(env)),
        Some(Err(Fehler::GameOver)),
        "Diagnose-Sicherung nach Game Over"
    );
    drop_device(&mut m, &code, "xbox", 3); // Abbruch des letzten Geräts
    wait(&mut m, &clock, EMPTY_FOR_SECS + 60); // Aufräumen des leeren Raums (save_sync)
    assert!(m.rooms.is_empty(), "leerer Raum nach Game Over nicht aufgeräumt");
    assert_eq!(m.store.calls, writes, "Speicherungen nach Game Over");
    assert_eq!(
        m.store.data["verloren"], prev,
        "Spielstand nach Game Over überschrieben"
    );
    assert!(logged(&m, "🚫 Spielstand nicht gespeichert"), "Log ohne 🚫");
}

// Herunterfahren nach Game Over schreibt nicht; ohne vorherigen Stand entsteht keine Datei.
#[test]
fn game_over_ohne_stand_beim_herunterfahren() {
    let (mut m, _) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "neu", true, defeat_stage());
    ticks(&mut m, &code, 10);
    rm(&mut m, &code).isl.over = true;
    m.close();
    assert_eq!(m.store.calls, 0, "Speicherungen nach Game Over");
    assert!(
        !m.store.data.contains_key("neu"),
        "Spielstand-Datei nach Game Over ohne vorherigen Stand angelegt"
    );
}
