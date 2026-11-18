//! Tests aus `review_test.go` (Befunde des Reviews SP07).
use std::collections::BTreeMap;
use std::sync::atomic::Ordering;

use k3c_sim::state::PlayerCommand;

use super::Options;
use super::testutil::{Fixture, add_room, manager, peer, room};
use crate::fehler::Fehler;

// Befund 3: Eine ersetzte Verbindung ändert nichts mehr am Gerät.
#[test]
fn ersetzte_verbindung_kann_nichts_mehr() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "alt-tab", 2);
    let ((a, mut ai), (b, _bi)) = (peer(1), peer(2));
    r.join(&mut f.env(), "tab", a, &[0]).expect("a");
    r.join(&mut f.env(), "tab", b, &[0]).expect("b");
    assert!(ai.has("replaced"));
    r.leave(&mut f.env(), "tab", 1);
    assert_eq!(
        r.add_slot("tab", 1, 1),
        Err(Fehler::BadRequest),
        "add_slot über alte Verbindung"
    );
    let cmd = BTreeMap::from([(
        0,
        PlayerCommand {
            move_x: 1.0,
            ..PlayerCommand::default()
        },
    )]);
    assert_eq!(
        r.input("tab", 1, &cmd),
        Err(Fehler::BadRequest),
        "input über alte Verbindung"
    );
    r.drop_conn(&mut f.env(), "tab", 1);
    assert!(
        r.monarchs[0].state == super::MonarchState::Taken && r.has("tab"),
        "alte Verbindung wirft das neue Gerät raus"
    );
}

// Befund 2: Ein geschlossener Raum speichert beim Verlassen nicht mehr (Absturz: Zustand nicht sicher).
#[test]
fn geschlossener_raum_speichert_nicht() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "absturz", 1);
    let (x, _xi) = peer(1);
    r.join(&mut f.env(), "xbox", x, &[0]).expect("join");
    r.closed = true;
    let lines = f.log.lines.len();
    r.leave(&mut f.env(), "xbox", 1);
    r.drop_conn(&mut f.env(), "xbox", 1);
    assert_eq!(f.store.saves, 0);
    assert_eq!(
        f.log.lines.len(),
        lines,
        "ein geschlossener Raum meldet und speichert nichts"
    );
    assert!(r.has("xbox"), "ein geschlossener Raum ändert sich nicht mehr");
}

// Befund 4: Nach `close` nimmt der Manager keine Räume mehr an.
#[test]
fn nach_close_keine_raeume() {
    let (mut m, _) = manager();
    m.close();
    let (p, _) = peer(1);
    assert_eq!(
        m.create("x", p, "spaet", true, 0, &[0], Options::default()),
        Err(Fehler::Closed)
    );
    let (p, _) = peer(2);
    assert_eq!(m.join("x", p, "KRNZ", &[0]), Err(Fehler::Closed));
}

// Ein Panic beim Aufräumen beendet nicht den Prozess; danach läuft der Manager weiter.
#[test]
fn sweep_ueberlebt_panic() {
    let (mut m, secs) = manager();
    let real = secs.clone();
    m.now = Box::new(|| panic!("Uhr kaputt"));
    m.safe_sweep();
    assert!(
        m.log
            .lines
            .iter()
            .any(|l| l.contains("💥 Sweep abgestürzt") && l.contains("Uhr kaputt")),
        "{:?}",
        m.log.lines
    );
    m.now = Box::new(move || crate::store::Stamp::new(real.load(Ordering::SeqCst), 0));
    add_room(&mut m, "KRNZ", "danach", 1);
    let (p, _) = peer(1);
    m.join("x", p, "KRNZ", &[0]).expect("der Manager läuft weiter");
}
