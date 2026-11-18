//! Tests aus `island_switch_test.go` (B-345, K4.2; WZ18.4, B-488/AC-03; KB2.1, B-385/AC-01): Der Raum tauscht die Insel erst
//! bei `switch_ready`. Die Test-Welt `nordmeer` mit zwei Inseln baut `create_island_in`, das `new_hub_site` (RU18) braucht, der
//! Hafen `dev_build_all` (RU18): diese Tests warten. Die Räume sind von Hand gebaut (`testutil::room`), die Takte laufen mit
//! der Dev-Pause (`paused`, keine Schritte; `step_players` kommt mit RU18).
use super::Room;
use super::testutil::{Fixture, peer, room};

fn joined(f: &mut Fixture, r: &mut Room, id: &str, conn: u64) -> super::testutil::Inbox {
    let (p, i) = peer(conn);
    r.join(&mut f.env(), id, p, &[0]).expect(id);
    i
}

#[test]
fn inselwechsel_erst_bei_switch_ready() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "wechsel", 1);
    r.paused = true;
    joined(&mut f, &mut r, "xbox", 1);
    assert!(!r.switch_island(&mut f.env()), "kein Wechsel ohne switch_ready");
    for _ in 0..120 {
        assert!(r.tick(&mut f.env()));
    }
    assert_eq!((r.isl.id.as_str(), r.isl.number), ("wechsel", 0));
}

// Die Insel hat keine nächste in ihrer Welt (hier eine Welt ohne Inseln): `switch_ready` tauscht nichts.
#[test]
fn inselwechsel_letzte_insel_ohne_tausch() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "letzte", 1);
    r.paused = true;
    let mut xi = joined(&mut f, &mut r, "xbox", 1);
    r.isl.switch_ready = true;
    let seats_before = xi.log().iter().filter(|e| *e == "seats").count();
    for _ in 0..5 {
        assert!(r.tick(&mut f.env()));
    }
    assert_eq!(r.isl.id, "letzte", "Insel getauscht, obwohl es keine nächste gibt");
    assert!(r.isl.switch_ready);
    assert_eq!(
        xi.log().iter().filter(|e| *e == "seats").count(),
        seats_before,
        "seats ohne Wechsel"
    );
}

// Der Raum ruft `switch_island` jeden Takt an der Stelle von Go (nach dem Zustand): auch ohne Wechsel bleiben Stufen der
// Geräte und Startstufe unverändert.
#[test]
fn inselwechsel_ohne_tausch_laesst_geraete_in_ruhe() {
    let mut f = Fixture::new();
    let mut r = room("KRNZ", "ruhe", 1);
    r.paused = true;
    joined(&mut f, &mut r, "xbox", 1);
    r.tick(&mut f.env());
    let stages = r.devices["xbox"].stages.clone();
    assert!(!stages.is_empty(), "das Gerät hat Level bekommen");
    r.start = 1;
    assert!(!r.switch_island(&mut f.env()));
    assert_eq!((r.start, r.devices["xbox"].stages.clone()), (1, stages));
}

#[test]
#[ignore = "wartet auf RU18 (create_island_in: new_hub_site, dev_build_all)"]
fn inselwechsel_tauscht_mit_geraeten() {} // nordmeerGate: Endboss besiegt, beide am Steg → Insel 2 (`eisinsel`), Level vor Zustand, Spielstand nennt Welt und Insel

#[test]
#[ignore = "wartet auf RU18 (create_island_in: new_hub_site, dev_build_all)"]
fn inselwechsel_ereignis_erreicht_geraete() {} // state, islandSwitch, level, state in dieser Reihenfolge je Gerät (B-385)
