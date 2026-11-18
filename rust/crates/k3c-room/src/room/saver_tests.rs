//! Tests aus `saver_test.go` (B-276, N1/AC-03). Abweichung: Rust hat keine Schreib-Goroutine, der Standard-Schreiber
//! `InlineWriter` schreibt im Tick. Gos Prüfung „Tick wartet nicht auf die Datei“ (`blockStore`) und „Herunterfahren
//! wartet auf die laufende Speicherung“ (`firstBlockStore`) entfallen; geprüft wird die Reihenfolge, die ohne Thread
//! bleibt: erst der Zustand, dann die Datei, das Ereignis `saved` im Zustand des nächsten Ticks.
use super::save_testutil::{create_with, leave, saved};
use super::testutil::{join_room, manager, move_to_exit, rm, ticks};
use super::types::Options;

// N1/AC-03 (B-276/AC-03): Ein Stufenwechsel speichert; der Zustand des Wechsel-Ticks geht vor der Datei hinaus (er trägt
// noch kein `saved`), der Zustand des nächsten Ticks meldet sie. `save_now` schreibt synchron.
#[test]
fn stufenwechsel_speichert_erst_zustand_dann_datei() {
    let (mut m, _) = manager();
    let (code, mut xi) = create_with(&mut m, "xbox", 1, "hinten", true, Options::default());
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70);
    assert_eq!(rm(&mut m, &code).isl.stage_of(0), Some(1), "Stufe");
    assert_eq!(m.store.saves, 1, "Speicherungen nach dem Stufenwechsel");
    assert_eq!(saved(&m, "hinten").players[0].depth, 1);
    let events = xi.events();
    let switch = events
        .iter()
        .filter(|(_, stage, _)| *stage == 1)
        .map(|(tick, _, _)| *tick)
        .min()
        .expect("Zustand der Stufe 1");
    let saved_ticks: Vec<i64> = events
        .iter()
        .filter(|(_, _, evs)| evs.iter().any(|e| e["type"] == "saved"))
        .map(|(tick, _, _)| *tick)
        .collect();
    assert_eq!(
        saved_ticks,
        [switch + 1],
        "saved erst im Zustand nach dem Wechsel-Tick {switch}"
    );
    assert_eq!(m.with_room(&code, |r, env| r.save_now(env).is_ok()), Some(true));
    assert_eq!(m.store.saves, 2, "save_now");
}

// Herunterfahren speichert nach der Speicherung beim Verlassen (ohne Thread: beide liegen nacheinander im Store).
#[test]
fn herunterfahren_speichert_nach_verlassen() {
    let (mut m, _) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "warten", true, Options::default());
    let _hi = join_room(&mut m, "handy", 2, &code, &[0]);
    leave(&mut m, &code, "handy", 2);
    assert_eq!(m.store.saves, 1, "Speicherung beim Verlassen");
    m.close();
    assert_eq!(m.store.saves, 2, "Speicherung beim Herunterfahren");
}

// Ein Panic beim Speichern beendet nicht den Raum; die nächste Speicherung läuft (der Schreiber ist danach nicht mehr
// belegt, sonst schriebe er nie wieder).
#[test]
fn panic_beim_speichern() {
    let (mut m, _) = manager();
    let (code, _xi) = create_with(&mut m, "xbox", 1, "panik", true, Options::default());
    m.store.panic_once = true;
    let before = m.store.saves;
    for _ in 0..2 {
        m.with_room(&code, |r, env| r.save(env)).expect("Raum");
    }
    assert_eq!(m.store.saves, before + 1, "nach dem Panic");
    let line = m
        .log
        .lines
        .iter()
        .find(|l| l.contains("💥 Spielstand nicht gespeichert, Speichern abgestürzt"));
    assert!(
        line.is_some_and(|l| l.contains("err=Platte weg")),
        "Log: {:?}",
        m.log.lines
    );
}
