//! Tests aus `room_test.go`: `join` auf dem Raum läuft hier direkt, `Manager` in `manager_tests.rs`, Aktionen und Takt in
//! `actions_tests.rs`; Speichern und neue Insel-Spieler über den Manager mit neuer Insel.
use k3c_sim::island::ISLAND_SAVE_VERSION;

use super::save_testutil::saved;
use super::testutil::{Fixture, create_room, join_room, manager, move_to_exit, peer, rm, room, ticks};
use super::{Fehler, MAX_MONARCHS, MonarchState, Peer, TEST_PREFIX, valid_slots};

fn join(f: &mut Fixture, r: &mut super::Room, id: &str, p: Box<dyn Peer>, slots: &[i64]) -> Result<(), Fehler> {
    r.join(&mut f.env(), id, p, slots)
}

// Ablauf aus docs/protocol.md › Beispiel: 2 Controller an der Xbox + 1 Handy (Beitreten; Drop und Wiederverbinden
// stehen in `actions_tests.rs`).
#[test]
fn xbox_und_handy_beitreten() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "familie", 3);
    let (x, mut xi) = peer(1);
    join(&mut f, &mut r, "xbox", x, &[0, 1]).expect("xbox");
    assert!(xi.has("joined KRNZ familie [{0 0 0 0} {1 1 0 0}]"), "{:?}", xi.log());
    assert!(xi.has("level 0") && xi.has("state 0"), "{:?}", xi.log());
    let (h, mut hi) = peer(2);
    join(&mut f, &mut r, "handy", h, &[0]).expect("handy");
    assert_eq!(
        (hi.you().as_str(), xi.monarchs().as_str()),
        ("[{0 2 0 0}]", "[taken taken taken]")
    );
    assert!(
        r.isl.players().iter().all(|p| !p.free),
        "alle drei Monarchen sind besetzt"
    );
    assert_eq!(r.connected(), 2);
    assert!(
        f.log
            .lines
            .iter()
            .any(|l| l.contains("👑 Gerät im Raum") && l.contains("ns=room room=KRNZ save=familie device=handy"))
    );
}

#[test]
fn wiederverbinden_mit_geaenderten_slots() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "slots", 2);
    let (x, _xi) = peer(1);
    join(&mut f, &mut r, "xbox", x, &[0, 1]).expect("xbox");
    let (x2, mut x2i) = peer(2);
    join(&mut f, &mut r, "xbox", x2, &[1, 2]).expect("xbox neu");
    // Slot 1 behält Monarch 1, Slot 0 fällt weg (Monarch 0 frei), Slot 2 bekommt den kleinsten freien: 0.
    assert_eq!(
        (x2i.you().as_str(), x2i.monarchs().as_str()),
        ("[{1 1 0 0} {2 0 0 0}]", "[taken taken]")
    );
    let (x3, mut x3i) = peer(3);
    join(&mut f, &mut r, "xbox", x3, &[1]).expect("xbox weniger");
    assert_eq!(
        (x3i.you().as_str(), x3i.monarchs().as_str()),
        ("[{1 1 0 0}]", "[free taken]")
    );
    assert!(r.isl.players()[0].free && !r.isl.players()[1].free);
}

// Ein Gerät, das nach der Frist zurückkehrt, bekommt seine alten Monarchen bevorzugt.
#[test]
fn rueckkehr_bevorzugt_alte_monarchen() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "frist", 3);
    let (x, _xi) = peer(1);
    join(&mut f, &mut r, "xbox", x, &[0]).expect("xbox");
    let (h, _hi) = peer(2);
    join(&mut f, &mut r, "handy", h, &[0, 1]).expect("handy");
    r.release(1);
    r.release(2); // Frist abgelaufen
    r.devices.remove("handy"); // wie `sweep` ein getrenntes Gerät ohne Plätze entfernt
    let (h2, mut h2i) = peer(3);
    join(&mut f, &mut r, "handy", h2, &[1, 0]).expect("Rückkehr");
    assert_eq!(h2i.you(), "[{0 1 0 0} {1 2 0 0}]");
}

#[test]
fn grenzen() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "grenzen", 4);
    let (x, _xi) = peer(1);
    join(&mut f, &mut r, "xbox", x, &[0, 1]).expect("xbox");
    let (d, _di) = peer(3);
    join(&mut f, &mut r, "drei", d, &[0, 1]).expect("zwei");
    let (v, _vi) = peer(2);
    assert_eq!(
        join(&mut f, &mut r, "voll", v, &[0, 1]),
        Err(Fehler::RoomFull),
        "5. Monarch"
    );
    assert!(!r.devices.contains_key("voll"), "Beitreten war nicht alles oder nichts");
    assert_eq!(r.count(MonarchState::Taken), MAX_MONARCHS);
    let (s, _si) = peer(4);
    assert_eq!(join(&mut f, &mut r, "x", s, &[4]), Err(Fehler::TooManySlots), "Slot 4");
    for bad in [&[][..], &[-1], &[1, 1]] {
        let (b, _bi) = peer(5);
        assert_eq!(
            join(&mut f, &mut r, "x", b, bad),
            Err(Fehler::BadRequest),
            "Slots {bad:?}"
        );
    }
    assert!(!r.devices.contains_key("x"));
}

#[test]
fn ersetzte_verbindung() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "tabs", 1);
    let (a, mut ai) = peer(1);
    join(&mut f, &mut r, "tab", a, &[0]).expect("a");
    let (b, mut bi) = peer(2);
    join(&mut f, &mut r, "tab", b, &[0]).expect("b");
    assert!(ai.has("replaced"), "{:?}", ai.log());
    assert_eq!(bi.you(), "[{0 0 0 0}]");
    assert_eq!(r.monarchs[0].state, MonarchState::Taken);
}

#[test]
fn gleiche_verbindung_wird_nicht_ersetzt() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "gleich", 1);
    let (a, mut ai) = peer(7);
    join(&mut f, &mut r, "tab", a, &[0]).expect("a");
    let (a2, _a2i) = peer(7); // gleiche conn_id: Gos `old.peer == peer`
    join(&mut f, &mut r, "tab", a2, &[0]).expect("a2");
    assert!(!ai.has("replaced"));
}

#[test]
fn valid_slots_reihenfolge() {
    assert_eq!(valid_slots(&[4, 4]), Err(Fehler::TooManySlots), "Slot 4 zuerst");
    assert_eq!(valid_slots(&[0, 0]), Err(Fehler::BadRequest));
    assert_eq!(valid_slots(&[]), Err(Fehler::BadRequest));
    assert_eq!(valid_slots(&[3, 0, 2]), Ok(()));
}

#[test]
fn save_now_meldet_fehler() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "reise", 1);
    let (x, _xi) = peer(1);
    join(&mut f, &mut r, "xbox", x, &[0]).expect("xbox");
    r.isl.over = true;
    assert_eq!(r.save_now(&mut f.env()), Err(Fehler::GameOver));
    assert_eq!(f.store.saves, 0, "komplett verloren: kein Speichern");
    assert!(
        f.log
            .lines
            .iter()
            .any(|l| l.contains("🚫 Spielstand nicht gespeichert, Insel komplett verloren"))
    );
    r.closed = true;
    assert_eq!(r.save_now(&mut f.env()), Err(Fehler::Closed));
}

// B-086: Ein Testraum (Präfix test-) löscht beim Aufräumen seinen Spielstand, andere Räume behalten ihn.
#[test]
fn test_raum_loescht_spielstand() {
    let mut f = Fixture::new();
    let test = room("KRNZ", &format!("{TEST_PREFIX}ab12"), 0);
    let real = room("BWTQ", "familie", 0);
    f.store.data.insert("familie".into(), b"{}".to_vec());
    assert!(test.is_test() && !real.is_test());
    test.drop_test_save(&mut f.env());
    real.drop_test_save(&mut f.env());
    assert_eq!(f.store.deleted, [format!("{TEST_PREFIX}ab12")]);
    assert!(
        f.store.data.contains_key("familie"),
        "der echte Spielstand muss bleiben"
    );
}

#[test]
fn push_state_schickt_level_nur_einmal() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "level", 1);
    let (x, mut xi) = peer(1);
    join(&mut f, &mut r, "xbox", x, &[0]).expect("xbox");
    assert!(!r.push_state(&f.env(), "xbox", None));
    let levels = xi.log().iter().filter(|e| e.starts_with("level")).count();
    assert_eq!(levels, 1, "{:?}", xi.log());
    assert_eq!(xi.log().iter().filter(|e| e.starts_with("state")).count(), 2);
}

// `save_now` schreibt den Spielstand der Insel über den Store: Version 9, die Spieler mit ihrer Tiefe.
#[test]
fn save_now_schreibt_den_stand() {
    let (mut m, _) = manager();
    let (code, _xi) = create_room(&mut m, "xbox", 1, "sofort", 0, &[0]);
    assert_eq!(m.with_room(&code, |r, env| r.save_now(env)), Some(Ok(String::new())));
    assert_eq!(m.store.saves, 1);
    let s = saved(&m, "sofort");
    assert_eq!(
        (s.version, s.campaign_id.starts_with("sofort-")),
        (ISLAND_SAVE_VERSION, true)
    );
    assert_eq!((s.players.len(), s.players[0].depth), (1, 0), "Spieler im Stand");
}

// Der Raum legt einen neuen Insel-Spieler an, wenn kein Monarch frei ist (`add_island_player`), in der Stufe des Geräts.
#[test]
fn assign_legt_neuen_spieler_an() {
    let (mut m, _) = manager();
    let (code, _xi) = create_room(&mut m, "xbox", 1, "neu", 0, &[0]);
    let _hi = join_room(&mut m, "handy", 2, &code, &[0, 1]);
    let r = rm(&mut m, &code);
    assert_eq!(r.monarchs.len(), 3, "Monarchen");
    let players: Vec<i64> = r.isl.stages[0].players.iter().map(|p| p.index).collect();
    assert_eq!(players, [0, 1, 2], "Insel-Spieler in Stufe 0");
    assert_eq!(r.devices["handy"].slots.values().copied().collect::<Vec<_>>(), [1, 2]);
}

// Stufenwechsel speichert (nach dem Tick), Herunterfahren speichert noch einmal und schließt alle Geräte (Go
// `TestSpeicherzeitpunkte`).
#[test]
fn speicherzeitpunkte() {
    let (mut m, _) = manager();
    let (code, mut xi) = create_room(&mut m, "xbox", 1, "reise", 0, &[0]);
    let saves = m.store.saves;
    move_to_exit(rm(&mut m, &code), 0);
    ticks(&mut m, &code, 70); // 2 s am Tiefen-Eingang
    assert_eq!(rm(&mut m, &code).isl.stage_of(0), Some(1), "Stufenwechsel");
    assert_eq!(m.store.saves, saves + 1, "Speicherungen beim Stufenwechsel");
    assert!(xi.has("level 1"), "{:?}", xi.log());
    let s = saved(&m, "reise");
    assert!(
        s.version == ISLAND_SAVE_VERSION && s.stages.len() == 5 && s.players[0].depth == 1,
        "gespeicherter Stand: {:?}",
        s.players
    );
    let mut hi = join_room(&mut m, "handy", 2, &code, &[0]);
    m.close();
    assert_eq!(m.store.saves, saves + 2, "Beenden");
    assert!(
        xi.has("closed") && hi.has("closed") && m.rooms.is_empty(),
        "{:?}",
        xi.log()
    );
}
