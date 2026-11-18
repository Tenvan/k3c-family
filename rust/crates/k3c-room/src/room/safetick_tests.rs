//! Test aus `safetick_test.go` (B-803/AC-01): Ein Panic im Takt schließt den Raum wie im Server ohne Speichern. Der Absturz
//! über `guard` läuft hier; der Teil mit der begonnenen Hintergrund-Speicherung (`blockStore`, `saver`) und `Room::tick`
//! kommen mit RU21.4 und RU24.
use super::testutil::{add_room, manager, peer};
use crate::fehler::Fehler;

#[test]
fn panic_schliesst_raum_ohne_speichern() {
    let (mut m, _) = manager();
    add_room(&mut m, "KRNZ", "absturz", 1);
    let (x, mut xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0]).expect("join");
    assert!(!m.guard("KRNZ", |_, _| panic!("Absturz im Test")));
    assert!(m.room("KRNZ").is_none() && xi.has("closed"));
    assert_eq!((m.failures.len(), m.failures[0].error.as_str()), (1, "Absturz im Test"));
    assert_eq!(m.store.saves, 0);
    assert!(!m.safe_tick("KRNZ"), "geschlossener Raum tickt weiter");
}

// Der abgestürzte Raum lässt sich nicht mehr speichern (`SaveNow` -> `room_closed`).
#[test]
fn save_now_nach_absturz_ist_closed() {
    let (mut m, _) = manager();
    add_room(&mut m, "KRNZ", "absturz", 1);
    let mut room = m.rooms.remove("KRNZ").expect("Raum");
    room.closed = true;
    let (_, mut env) = m.split();
    assert_eq!(room.save_now(&mut env), Err(Fehler::Closed));
}

// Panic im Takt über `safe_tick` mit begonnener Hintergrund-Speicherung (`blockStore`): braucht `Room::tick` und `saver`.
#[test]
#[ignore = "wartet auf RU21"]
fn safe_tick_panic_schliesst_raum() {}
