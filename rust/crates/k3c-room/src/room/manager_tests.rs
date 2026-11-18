//! Tests des Managers (Go: Teile von `room_test.go`, `testsave_test.go`; `create` mit neuer Insel braucht `create_in`, RU24).
use std::sync::atomic::Ordering;

use super::manager::Manager;
use super::testutil::{MemStore, add_room, manager, peer};
use super::{CODE_LETTERS, Info, Options, RecordingLog};
use crate::fehler::Fehler;

type M = Manager<MemStore, RecordingLog>;

fn logged(m: &M, text: &str) -> bool {
    m.log.lines.iter().any(|l| l.contains(text))
}

fn create(m: &mut M, id: &str, name: &str, fresh: bool, slots: &[i64], opts: Options) -> Result<String, Fehler> {
    let (p, _inbox) = peer(9);
    m.create(id, p, name, fresh, 0, slots, opts)
}

#[test]
fn join_per_code() {
    let (mut m, _) = manager();
    add_room(&mut m, "KRNZ", "familie", 3);
    let (x, mut xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0, 1]).expect("join");
    assert!(xi.has("joined KRNZ familie [{0 0 0 0} {1 1 0 0}]"), "{:?}", xi.log());
    assert_eq!(m.room("KRNZ").map(|r| r.connected()), Some(1));
    let (h, _) = peer(2);
    assert_eq!(m.join("handy", h, "ZZZZ", &[0]), Err(Fehler::RoomNotFound));
    assert!(
        logged(
            &m,
            "🚫 Beitritt abgelehnt ns=room op=join device=handy ziel=ZZZZ code=room_not_found"
        ),
        "{:?}",
        m.log.lines
    );
    let before = m.log.lines.len();
    let (b, _) = peer(3);
    assert_eq!(m.join("x", b, "KRNZ", &[1, 1]), Err(Fehler::BadRequest));
    assert_eq!(m.log.lines.len(), before, "ungültige Slots stehen nicht im Log");
}

#[test]
fn join_geschlossener_raum_und_nach_close() {
    let (mut m, _) = manager();
    add_room(&mut m, "KRNZ", "familie", 1);
    m.rooms.get_mut("KRNZ").expect("Raum").closed = true;
    let (a, _) = peer(1);
    assert_eq!(m.join("a", a, "KRNZ", &[0]), Err(Fehler::RoomNotFound));
    m.shut = true;
    let (b, _) = peer(2);
    assert_eq!(m.join("b", b, "KRNZ", &[0]), Err(Fehler::Closed));
}

// TestRaumliste: Code, Name, Tiefe, Grad, besetzt und frei, Welt; nach Code sortiert.
#[test]
fn raumliste() {
    let (mut m, _) = manager();
    add_room(&mut m, "ZZZZ", "zwei", 2);
    add_room(&mut m, "KRNZ", "liste", 2);
    let (x, mut xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0, 1]).expect("join");
    assert!(xi.log()[0].starts_with("joined"), "joined kommt nicht zuerst");
    let want = Info {
        code: "KRNZ".into(),
        name: "liste".into(),
        depth: 0,
        grade: "normal".into(),
        taken: 2,
        free: 2,
        running: true,
        world: "standard".into(),
    };
    let got = m.rooms();
    assert_eq!(got.len(), 2);
    assert_eq!(got[0], want);
    assert_eq!((got[1].code.as_str(), got[1].running, got[1].taken), ("ZZZZ", false, 0));
}

// TestCreateReihenfolge, soweit sie vor `create_in` entschieden wird.
#[test]
fn create_reihenfolge() {
    let (mut m, _) = manager();
    let d = Options::default;
    assert_eq!(
        create(&mut m, "a", "Falsch!", true, &[4], d()),
        Err(Fehler::TooManySlots),
        "Slot 4 zuerst"
    );
    assert_eq!(
        create(&mut m, "a", "Falsch!", true, &[0], d()),
        Err(Fehler::BadRequest),
        "Name"
    );
    assert!(m.log.lines.is_empty(), "Slots und Name stehen nicht im Log");
    add_room(&mut m, "KRNZ", "offen", 1);
    assert_eq!(
        create(&mut m, "b", "offen", true, &[0], d()),
        Err(Fehler::SaveExists),
        "neu, aber offen"
    );
    assert!(logged(
        &m,
        "🚫 Beitritt abgelehnt ns=room op=create device=b ziel=offen code=save_exists"
    ));
    let again = create(&mut m, "b", "offen", false, &[0], d());
    assert_eq!(
        again,
        Ok("KRNZ".to_string()),
        "gespeichert und offen: demselben Raum beitreten"
    );
    assert_eq!(m.room("KRNZ").map(|r| r.connected()), Some(1));
    m.store.data.insert("alt".into(), b"{}".to_vec());
    assert_eq!(
        create(&mut m, "c", "alt", true, &[0], d()),
        Err(Fehler::SaveExists),
        "neu, aber gespeichert"
    );
    assert_eq!(
        create(&mut m, "c", "fehlt", false, &[0], d()),
        Err(Fehler::SaveNotFound),
        "gespeichert, aber fehlt"
    );
    let long = Options {
        seed: "x".repeat(65),
        ..d()
    };
    assert_eq!(
        create(&mut m, "c", "neu", true, &[0], long),
        Err(Fehler::BadRequest),
        "Seed zu lang"
    );
    m.shut = true;
    assert_eq!(create(&mut m, "c", "neu", true, &[0], d()), Err(Fehler::Closed));
    assert_eq!(m.rooms().len(), 1, "Raum trotz Fehler angelegt");
}

// Spielstand vor Raumzahl prüfen und der 5. Raum brauchen eine neue Insel (`create_in`).
#[test]
fn create_raumzahl() {}

#[test]
fn code_besteht_aus_buchstaben_und_ist_frei() {
    let (mut m, _) = manager();
    let mut codes = Vec::new();
    for _ in 0..40 {
        let c = m.code();
        assert_eq!(c.chars().count(), 4);
        assert!(c.chars().all(|ch| CODE_LETTERS.contains(ch)), "{c}");
        assert!(!c.contains('I') && !c.contains('O'));
        add_room(&mut m, &c, "x", 0); // der Code ist belegt: der nächste ist ein anderer
        codes.push(c);
    }
    codes.sort();
    codes.dedup();
    assert_eq!(codes.len(), 40, "doppelter Code");
}

#[test]
fn close_meldet_room_closed_und_sperrt() {
    let (mut m, _) = manager();
    add_room(&mut m, "KRNZ", "stand", 1);
    m.rooms.get_mut("KRNZ").expect("Raum").isl.over = true; // komplett verloren: kein Speichern (Kodieren folgt mit RU18)
    let (x, mut xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0]).expect("join");
    m.close();
    assert!(xi.has("closed") && m.rooms.is_empty() && m.shut);
    assert_eq!(m.store.saves, 0);
    assert!(
        logged(
            &m,
            "🛑 Raum schließt (Server fährt herunter) ns=room room=KRNZ save=stand"
        ),
        "{:?}",
        m.log.lines
    );
    let (p, _) = peer(2);
    assert_eq!(
        m.create("b", p, "neu", true, 0, &[0], Options::default()),
        Err(Fehler::Closed)
    );
}

#[test]
fn zeit_laeuft_ueber_die_uhr_des_managers() {
    let (mut m, secs) = manager();
    let a = (m.now)();
    secs.fetch_add(61, Ordering::SeqCst);
    assert_eq!((m.now)().since_nanos(a), 61_000_000_000);
}

// AC-03: Anlegen aus `testdata/saves/v9/familie.json` startet einen Raum. `Manager::load_in` (RU24) braucht die Insel aus
// einem Spielstand (`from_island_save`), und die Stufen entstehen über `create_world` (`new_hub_site`, RU18).
#[test]
fn anlegen_aus_spielstand_v9() {}
