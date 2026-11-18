//! Tests aus `pause_test.go` (BD3.1/BD3.3, B-214): Pause setzt den Zustand des eigenen Geräts, im Couch-Raum steht der
//! Zustand, online stehen die Monarchen des pausierten Geräts still und sind `PROTECT_TICKS` lang geschützt. Die Räume sind
//! von Hand gebaut (`testutil::room`); die Schritte der Insel (`step_players`) kommen mit RU18. Tests mit Schritten warten,
//! der Schutz und seine Frist laufen über `pause_tick` direkt (er zählt Ticks, ohne dass ein Schritt rechnet).
use std::collections::BTreeMap;

use k3c_sim::state::PlayerCommand;

use super::testutil::{Fixture, Inbox, TestManager, create_room, join_room, manager, peer, rm, room, ticks};
use super::{PROTECT_TICKS, Room};
use crate::fehler::Fehler;

fn go(slot: i64, move_x: f64) -> BTreeMap<i64, PlayerCommand> {
    BTreeMap::from([(
        slot,
        PlayerCommand {
            move_x,
            ..PlayerCommand::default()
        },
    )])
}

fn joined(f: &mut Fixture, r: &mut Room, id: &str, conn: u64, slots: &[i64]) -> Inbox {
    let (p, i) = peer(conn);
    r.join(&mut f.env(), id, p, slots).expect(id);
    i
}

/// Zustand aller Stufen als Text (Go `worldJSON`).
fn stages_text(r: &Room) -> String {
    format!("{:?}", r.isl.stages)
}

/// Zwei Geräte mit je einem Monarchen (Go `onlineRoom`): `xbox` hat Monarch 0, `handy` Monarch 1.
fn online_room(f: &mut Fixture) -> Room {
    let mut r = room("KRNZ", "online", 2);
    joined(f, &mut r, "xbox", 1, &[0]);
    joined(f, &mut r, "handy", 2, &[0]);
    r
}

fn protected(r: &Room) -> (bool, bool) {
    let players = r.isl.players();
    (players[0].protected, players[1].protected)
}

// BD3.1, B-214: Pause setzt den Zustand des eigenen Geräts; zweites pause und resume ohne Pause ändern nichts, eine fremde
// Verbindung ist bad_request.
#[test]
fn pause_setzt_geraet() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "pause", 1);
    joined(&mut f, &mut r, "xbox", 1, &[0]);
    for (on, want) in [(false, false), (true, true), (true, true), (false, false)] {
        assert_eq!(r.pause(&mut f.env(), "xbox", 1, on), Ok(()));
        assert_eq!(r.devices["xbox"].paused, want, "pause({on})");
    }
    assert_eq!(
        r.pause(&mut f.env(), "xbox", 99, true),
        Err(Fehler::BadRequest),
        "fremde Verbindung"
    );
    assert_eq!(
        r.pause(&mut f.env(), "unbekannt", 1, true),
        Err(Fehler::BadRequest),
        "fremdes Gerät"
    );
    assert!(!r.devices["xbox"].paused);
}

#[test]
fn pause_beobachter_ist_bad_request() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "beobachter", 1);
    joined(&mut f, &mut r, "xbox", 1, &[0]);
    if let Some(d) = r.devices.get_mut("xbox") {
        d.observer = true;
    }
    assert_eq!(r.pause(&mut f.env(), "xbox", 1, true), Err(Fehler::BadRequest));
}

// B-214: Beim Anhalten und Fortsetzen verfallen die letzten Eingaben der Monarchen des Geräts, auch die aus der Pause; das
// andere Gerät behält seine Eingabe. Die Meldung trägt Gerät, Pause und Couch.
#[test]
fn pause_verwirft_eingaben_des_geraets() {
    let mut f = Fixture::new();
    let mut r = online_room(&mut f);
    assert_eq!(r.input("xbox", 1, &go(0, 1.0)), Ok(()));
    assert_eq!(r.input("handy", 2, &go(0, 1.0)), Ok(()));
    r.pause(&mut f.env(), "xbox", 1, true).expect("pause");
    assert_eq!((r.monarchs[0].input.move_x, r.monarchs[1].input.move_x), (0.0, 1.0));
    assert_eq!(r.input("xbox", 1, &go(0, 1.0)), Ok(())); // während der Pause empfangen
    r.pause(&mut f.env(), "xbox", 1, false).expect("resume");
    assert_eq!((r.monarchs[0].input.move_x, r.monarchs[1].input.move_x), (0.0, 1.0));
    assert!(
        f.log
            .lines
            .iter()
            .any(|l| l.contains("🚦 Gerät pausiert") && l.contains("device=xbox pausiert=true couch=false")),
        "{:?}",
        f.log.lines
    );
}

// BD3.3, B-214/AC-01: Im Couch-Raum (ein Gerät, 2 Spieler) steht der Zustand während der Pause, auch mit Eingaben; es gibt
// keine Schritte und keinen Schutz.
#[test]
fn pause_couch_raum_steht() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "couch", 2);
    joined(&mut f, &mut r, "xbox", 1, &[0, 1]);
    r.pause(&mut f.env(), "xbox", 1, true).expect("pause");
    assert!(r.couch_paused());
    assert_eq!(r.steps(), 0);
    let both = BTreeMap::from([
        (
            0,
            PlayerCommand {
                move_x: 1.0,
                ..PlayerCommand::default()
            },
        ),
        (
            1,
            PlayerCommand {
                move_x: -1.0,
                ..PlayerCommand::default()
            },
        ),
    ]);
    assert_eq!(r.input("xbox", 1, &both), Ok(()));
    let before = stages_text(&r);
    for _ in 0..90 {
        assert!(r.tick(&mut f.env()));
    }
    assert_eq!(
        stages_text(&r),
        before,
        "Zustand hat sich während der Couch-Pause geändert"
    );
    assert!(
        r.isl.players().iter().all(|p| !p.protected),
        "Couch-Pause schützt Monarchen (unnötig, der Raum steht)"
    );
    r.pause(&mut f.env(), "xbox", 1, false).expect("resume");
    assert!(!r.couch_paused());
    assert_eq!(r.steps(), 1);
}

// BD3.3, B-214/AC-02 (Schutz, zwei Geräte): Pausiert das eine Gerät, steht sein Monarch geschützt, das andere Gerät läuft
// unverändert und ungeschützt (der Raum rechnet weiter: `steps`). Der Schutz gilt `PROTECT_TICKS` Ticks, danach ist der
// Monarch verwundbar; ein zweites pause verlängert nicht. `pause_tick` zählt, ohne dass ein Schritt rechnet.
#[test]
fn pause_online_schutz_60s() {
    let mut f = Fixture::new();
    let mut r = online_room(&mut f);
    r.pause(&mut f.env(), "xbox", 1, true).expect("pause");
    assert!(!r.couch_paused(), "zwei Geräte: kein Couch-Raum");
    assert_eq!(r.steps(), 1, "der Raum läuft weiter");
    assert!(r.device_paused("xbox") && !r.device_paused("handy"));
    for _ in 0..100 {
        r.pause_tick(&mut f.env());
    }
    assert_eq!(protected(&r), (true, false), "A geschützt, B nicht");
    r.pause(&mut f.env(), "xbox", 1, true).expect("zweites pause"); // Frist läuft weiter
    assert_eq!(r.devices["xbox"].protect, PROTECT_TICKS - 100);
    for _ in 100..PROTECT_TICKS - 1 {
        r.pause_tick(&mut f.env());
    }
    assert_eq!(protected(&r), (true, false), "Tick {}", PROTECT_TICKS - 1);
    r.pause_tick(&mut f.env()); // Tick 1800: letzter geschützter Tick
    assert_eq!(
        (protected(&r), r.devices["xbox"].protect),
        ((true, false), 0),
        "Schutz endet vor 1800 Ticks"
    );
    assert!(
        f.log
            .lines
            .iter()
            .any(|l| l.contains("⏳ Schutz der Pause endet") && l.contains("device=xbox frist=1m0s")),
        "{:?}",
        f.log.lines
    );
    r.pause_tick(&mut f.env());
    assert_eq!(protected(&r), (false, false), "nach 60 s verwundbar");
    assert!(r.device_paused("xbox"), "das Gerät bleibt pausiert, steht weiter");
    r.pause(&mut f.env(), "xbox", 1, false).expect("resume");
    r.pause_tick(&mut f.env());
    assert_eq!(protected(&r), (false, false));
    assert_eq!(r.devices["xbox"].protect, 0);
}

// BD3.3, B-214: Bricht die Verbindung des pausierten Geräts ab, endet die Pause (Frist laut § 3).
#[test]
fn pause_endet_bei_trennung() {
    let mut f = Fixture::new();
    let mut r = online_room(&mut f);
    r.pause(&mut f.env(), "xbox", 1, true).expect("pause");
    for _ in 0..10 {
        r.pause_tick(&mut f.env());
    }
    assert!(r.isl.players()[0].protected);
    r.isl.over = true; // Trennen speichert; die Test-Insel lässt sich nicht kodieren (wie in `actions_tests.rs`)
    r.drop_conn(&mut f.env(), "xbox", 1);
    r.pause_tick(&mut f.env());
    assert!(
        !r.devices["xbox"].paused && !r.isl.players()[0].protected,
        "Pause oder Schutz besteht nach der Trennung"
    );
}

// BD3.3, B-214: Die Pause gilt für alle Spieler des Geräts (Split-Screen): Beide Monarchen der Xbox sind geschützt, der des
// Handys nicht; geht das Handy, ist es ein Couch-Raum und der Raum steht.
#[test]
fn pause_zwei_spieler_am_geraet() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "split", 3);
    joined(&mut f, &mut r, "xbox", 1, &[0, 1]);
    joined(&mut f, &mut r, "handy", 2, &[0]);
    r.pause(&mut f.env(), "xbox", 1, true).expect("pause");
    r.pause_tick(&mut f.env());
    let players = r.isl.players();
    for (slot, idx) in &r.devices["xbox"].slots {
        assert!(players[*idx].protected, "Xbox-Slot {slot} ungeschützt");
    }
    assert!(!players[r.devices["handy"].slots[&0]].protected, "Handy geschützt");
    r.isl.over = true; // Gehen speichert; die Test-Insel lässt sich nicht kodieren
    r.leave(&mut f.env(), "handy", 2);
    assert!(r.couch_paused() && r.steps() == 0, "Couch-Raum steht");
    assert!(r.tick(&mut f.env())); // der Schutz verfällt im Couch-Raum, danach steht alles
    let before = stages_text(&r);
    for _ in 0..30 {
        assert!(r.tick(&mut f.env()));
    }
    assert_eq!(
        stages_text(&r),
        before,
        "Couch-Raum nach dem Gehen des Handys läuft weiter"
    );
}

// ---- Tests mit echter Insel (`Manager::create`, Schritte der Sim) ----

fn real_json(m: &mut TestManager, code: &str) -> String {
    serde_json::to_string(&rm(m, code).isl.stages).expect("Stufen")
}

fn manager_pause(m: &mut TestManager, code: &str, id: &str, conn: u64, on: bool) {
    let r = m.with_room(code, |r, env| r.pause(env, id, conn, on)).expect("Raum");
    assert_eq!(r, Ok(()));
}

// Nach resume läuft der Couch-Raum weiter (BD3.3, B-214/AC-01): Zustand steht in der Pause, danach nicht mehr.
#[test]
fn pause_couch_raum_laeuft_nach_resume() {
    let (mut m, _) = manager();
    let (code, _x) = create_room(&mut m, "xbox", 1, "couch", 0, &[0, 1]);
    ticks(&mut m, &code, 5);
    manager_pause(&mut m, &code, "xbox", 1, true);
    assert_eq!(rm(&mut m, &code).input("xbox", 1, &go(0, 1.0)), Ok(()));
    let before = real_json(&mut m, &code);
    ticks(&mut m, &code, 90);
    assert_eq!(
        real_json(&mut m, &code),
        before,
        "Zustand hat sich während der Couch-Pause geändert"
    );
    manager_pause(&mut m, &code, "xbox", 1, false);
    let before = real_json(&mut m, &code);
    assert_eq!(rm(&mut m, &code).input("xbox", 1, &go(0, 1.0)), Ok(()));
    ticks(&mut m, &code, 30);
    assert_ne!(real_json(&mut m, &code), before, "Zustand steht nach resume weiter");
}

// Der Monarch des pausierten Geräts läuft nicht, der des anderen schon (BD3.3, B-214/AC-02).
#[test]
fn pause_online_monarch_steht_still() {
    let (mut m, _) = manager();
    let (code, _x) = create_room(&mut m, "xbox", 1, "online", 0, &[0]);
    let _h = join_room(&mut m, "handy", 2, &code, &[0]);
    manager_pause(&mut m, &code, "xbox", 1, true);
    assert_eq!(rm(&mut m, &code).input("xbox", 1, &go(0, 1.0)), Ok(()));
    assert_eq!(rm(&mut m, &code).input("handy", 2, &go(0, 1.0)), Ok(()));
    let r = rm(&mut m, &code);
    let (ax, bx) = (r.isl.players()[0].x, r.isl.players()[1].x);
    ticks(&mut m, &code, 100);
    let players = rm(&mut m, &code).isl.players().iter().map(|p| p.x).collect::<Vec<_>>();
    assert!(
        players[0] == ax && players[1] > bx,
        "A x {ax} -> {} (soll stehen), B x {bx} -> {} (soll laufen)",
        players[0],
        players[1]
    );
}
