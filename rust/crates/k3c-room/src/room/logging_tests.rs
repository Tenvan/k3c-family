//! Tests aus `logging_test.go`. `Manager::create` (Meldung „Raum erstellt“) und der Spielstand der Test-Insel (Meldung
//! „Spielstand gespeichert“) kommen mit RU24 und RU18; der Rest des Lebenszyklus läuft hier.
use super::logging::{SLOW_EVERY_SECS, SLOW_TICK_NANOS};
use super::testutil::{Fixture, add_room, manager, peer, room};
use crate::fehler::Fehler;

// Lebenszyklus im Log: Beitritt, Trennen, abgelehnter Beitritt, Aufräumen nach der Frist.
#[test]
fn lebenszyklus_im_log() {
    let (mut m, secs) = manager();
    add_room(&mut m, "KRNZ", "logtest", 1);
    m.rooms.get_mut("KRNZ").expect("Raum").isl.over = true; // kein Spielstand: Kodieren der Test-Insel folgt mit RU18
    let (x, _xi) = peer(1);
    m.join("xbox-gerät-1", x, "KRNZ", &[0]).expect("join");
    m.with_room("KRNZ", |r, env| r.drop_conn(env, "xbox-gerät-1", 1));
    let (n, _) = peer(2);
    assert_eq!(m.join("xbox-gerät-1", n, "NOPE", &[0]), Err(Fehler::RoomNotFound));
    secs.fetch_add(601, std::sync::atomic::Ordering::SeqCst);
    m.sweep();
    let log = m.log.lines.join("\n");
    for want in [
        "Gerät im Raum",
        "room=KRNZ",
        "device=xbox-ger", // die ID erscheint auf acht Zeichen gekürzt
        "Gerät getrennt",
        "Beitritt abgelehnt",
        "code=room_not_found",
        "Raum leer seit Frist",
    ] {
        assert!(log.contains(want), "{want:?} fehlt im Log:\n{log}");
    }
    assert!(m.rooms.is_empty());
}

// Langsame Ticks werden gezählt und höchstens alle `SLOW_EVERY_SECS` gemeldet.
#[test]
fn langsamer_tick_wird_gesammelt_gemeldet() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "langsam", 1);
    r.note_tick(&mut f.env(), SLOW_TICK_NANOS / 2); // im Budget: keine Meldung
    r.note_tick(&mut f.env(), SLOW_TICK_NANOS * 3);
    r.note_tick(&mut f.env(), SLOW_TICK_NANOS * 3); // innerhalb von `SLOW_EVERY_SECS`: nur gezählt
    let n = f.log.lines.iter().filter(|l| l.contains("Tick zu langsam")).count();
    assert_eq!(n, 1, "{:?}", f.log.lines);
    assert_eq!(
        r.slow_count, 1,
        "die Meldung setzt den Zähler zurück, der zweite Tick zählt wieder"
    );
    assert!(
        f.log.lines[0]
            .starts_with("Warn 🐢 Tick zu langsam ns=room room=KRNZ save=langsam ms=99.999999 budgetMs=33.333333"),
        "{:?}",
        f.log.lines
    );
    f.now = f.now.add_secs(SLOW_EVERY_SECS);
    r.note_tick(&mut f.env(), SLOW_TICK_NANOS * 2);
    let n = f.log.lines.iter().filter(|l| l.contains("Tick zu langsam")).count();
    assert_eq!(n, 2, "nach der Frist wird wieder gemeldet");
    assert!(f.log.lines[1].contains("seitLetzterMeldung=2"), "{:?}", f.log.lines);
}
