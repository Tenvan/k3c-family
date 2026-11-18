//! Test aus `deletesave_test.go` (BD5.1, B-506/AC-01): Ein Spielstand, der in einem Raum offen ist (auch ohne verbundenes
//! Gerät), wird nicht gelöscht; sonst läuft das Löschen.
use std::cell::Cell;

use super::Options;
use super::testutil::{manager, peer};
use crate::fehler::Fehler;

#[test]
fn delete_save_lehnt_offenen_stand_ab() {
    let (mut m, _) = manager();
    let (x, _xi) = peer(1);
    m.create("xbox", x, "familie", true, 0, &[0], Options::default())
        .expect("create");
    let called = Cell::new(0);
    let del = || {
        called.set(called.get() + 1);
        Ok(())
    };
    let r = m.delete_save("familie", del);
    assert!(
        r == Err(Fehler::SaveInUse) && called.get() == 0,
        "offen: {r:?}, gelöscht {}",
        called.get()
    );
    assert!(
        m.log
            .lines
            .iter()
            .any(|l| l.contains("🚫 Spielstand löschen abgelehnt, Raum offen") && l.contains("save=familie")),
        "{:?}",
        m.log.lines
    );
    assert_eq!(m.delete_save("anderer", del), Ok(()));
    assert_eq!(called.get(), 1, "nicht offen: gelöscht {}", called.get());
    let r = m.delete_save("../x", del);
    assert!(
        r == Err(Fehler::BadRequest) && called.get() == 1,
        "ungültiger Name: {r:?}"
    );
}
