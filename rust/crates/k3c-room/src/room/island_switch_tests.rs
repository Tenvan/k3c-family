//! Tests aus `island_switch_test.go` (B-345, K4.2; WZ18.4, B-488/AC-03; KB2.1, B-385/AC-01): Der Raum tauscht die Insel erst
//! bei `switch_ready`. Die ersten drei Tests rechnen mit von Hand gebauten Räumen (`testutil::room`, Dev-Pause); die
//! `kampf_*`-Tests sind die von Go mit der Test-Welt `nordmeer` (zwei Inseln, `Manager::worlds`).
use std::collections::BTreeMap;

use k3c_sim::dev::dev_build_all;
use k3c_sim::island::Island;

use super::stages_tests::begins_with_level;
use super::testutil::{
    Fixture, Inbox, TestManager, WorldDir, create_room_in, dev_manager, einzel_manager, join_room, peer, rm, room,
    ticks,
};
use super::worlds_tests::NORDMEER_RAW;
use super::{MonarchState, Options, Room};

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

// ---- Tests mit echter Insel in der Welt `nordmeer` (Go: `nordmeerGate`, B-345, K4.2; WZ18.4, B-488/AC-03) ----

/// Raum in der Welt `nordmeer`, Xbox (Monarch 0) und Handy (Monarch 1) in der tiefsten Stufe an der Burg; die Spieler
/// stehen am Steg des Hafens (`board_harbor`). Der Ordner der Welten lebt, solange der Wert lebt.
struct Gate {
    m: TestManager,
    _dir: WorldDir,
    code: String,
    x: Inbox,
    h: Inbox,
}

fn nordmeer_gate() -> Gate {
    let dir = WorldDir::new("switch", &[("nordmeer", NORDMEER_RAW)]);
    let mut m = dev_manager();
    m.worlds = Some(dir.src.clone());
    let (p, x) = peer(1);
    let code = m
        .create(
            "xbox",
            p,
            "stufen",
            true,
            4,
            &[0],
            Options {
                world: "nordmeer".into(),
                ..Options::default()
            },
        )
        .expect("create");
    let h = join_room(&mut m, "handy", 2, &code, &[0]);
    let isl = &mut rm(&mut m, &code).isl;
    assert!(
        isl.stages[1].players.len() == 2 && isl.stages[1].biome.depth == 4,
        "Spieler in Tiefe {}: {}",
        isl.stages[1].biome.depth,
        isl.stages[1].players.len()
    );
    board_harbor(isl);
    Gate {
        m,
        _dir: dir,
        code,
        x,
        h,
    }
}

/// Baut das Schiff fertig und stellt alle Spieler (Stufe 1, tiefste) an den Steg des Hafens (Stufe 0).
pub(super) fn board_harbor(isl: &mut Island) {
    dev_build_all(isl, 5);
    let ship_x: Vec<f64> = isl.stages[0]
        .sites
        .iter()
        .filter(|s| s.kind == "ship")
        .map(|s| s.x)
        .collect();
    for x in ship_x {
        for p in &mut isl.stages[1].players {
            p.x = x;
        }
    }
    let moved = std::mem::take(&mut isl.stages[1].players);
    isl.stages[0].players.extend(moved);
}

fn island_id(m: &mut TestManager, code: &str) -> i64 {
    rm(m, code).isl.number
}

/// Nummer der Insel des Raums (Gos Zeigervergleich `r.isl != old`: der Wechsel zählt sie hoch).
/// Rechnet Takte, bis die Insel getauscht ist (höchstens 120); `true`: getauscht.
fn until_switched(g: &mut Gate, old: i64) -> bool {
    for _ in 0..120 {
        if island_id(&mut g.m, &g.code) != old {
            break;
        }
        ticks(&mut g.m, &g.code, 1);
    }
    island_id(&mut g.m, &g.code) != old
}

#[test]
fn kampf_inselwechsel_erst_bei_switch_ready() {
    let mut g = nordmeer_gate();
    let old = island_id(&mut g.m, &g.code);
    ticks(&mut g.m, &g.code, 120); // Endboss nicht besiegt: kein Wechselpunkt
    assert_eq!(island_id(&mut g.m, &g.code), old, "Insel getauscht ohne switch_ready");
}

// AC-03 (Raum): Endboss besiegt, beide Spieler am Wechselpunkt: gemeinsamer Wechsel zu Insel 2 der Welt; der gespeicherte
// Stand nennt Welt und Insel 2.
#[test]
fn kampf_inselwechsel_tauscht_mit_geraeten() {
    let mut g = nordmeer_gate();
    let old = island_id(&mut g.m, &g.code);
    let (xi, hi) = (g.x.msgs().len(), g.h.msgs().len());
    rm(&mut g.m, &g.code).isl.endboss_defeated = true;
    assert!(until_switched(&mut g, old), "Insel nicht getauscht");
    switched_together(&mut g);
    // Level vor Zustand: Das Gerät beginnt die neue Insel mit einem vollen Zustand.
    assert!(
        begins_with_level(&g.x.msgs()[xi..], 0) && begins_with_level(&g.h.msgs()[hi..], 0),
        "Xbox {:?}, Handy {:?}",
        &g.x.msgs()[xi..],
        &g.h.msgs()[hi..]
    );
    let code = g.code.clone();
    g.m.with_room(&code, |r, env| r.save_now(env))
        .expect("Raum")
        .expect("speichern");
    let s: serde_json::Value = serde_json::from_slice(&g.m.store.data["stufen"]).expect("JSON");
    assert_eq!(
        (s["world"].as_str(), s["islandId"].as_str()),
        (Some("nordmeer"), Some("eisinsel")),
        "Spielstand"
    );
}

/// Der Raum steht auf Insel 2 der Welt `nordmeer`, beide Monarchen gesteuert in ihrer ersten Stufe, Slots unverändert.
fn switched_together(g: &mut Gate) {
    let r = rm(&mut g.m, &g.code);
    assert!(
        r.isl.number == 1 && r.isl.island_id() == "eisinsel" && r.isl.world_id() == "nordmeer" && !r.isl.switch_ready,
        "Nummer {}, Insel {:?}, Welt {:?}, bereit {}",
        r.isl.number,
        r.isl.island_id(),
        r.isl.world_id(),
        r.isl.switch_ready
    );
    for i in 0..2 {
        assert!(
            r.isl.stage_of(i as i64) == Some(0) && r.monarchs[i].state == MonarchState::Taken,
            "Monarch {i}: Stufe {:?}, Zustand {:?}",
            r.isl.stage_of(i as i64),
            r.monarchs[i].state
        );
    }
    assert_eq!(r.devices["xbox"].slots, BTreeMap::from([(0, 0)]), "Slots Xbox");
    assert_eq!(r.devices["handy"].slots, BTreeMap::from([(0, 1)]), "Slots Handy");
}

// Die letzte Insel der Welt: `switch_ready` tauscht nichts (zwei Stufen der Standard-Welt, kein Nachfolger).
#[test]
fn kampf_inselwechsel_letzte_insel_ohne_tausch() {
    let (mut m, _dir) = einzel_manager("letzte"); // Welt mit einer Insel
    let (code, _x) = create_room_in(&mut m, "einzel", "xbox", 1, "letzte", 0, &[0]); // Welt mit einer Insel
    let old = island_id(&mut m, &code);
    rm(&mut m, &code).isl.switch_ready = true;
    ticks(&mut m, &code, 5);
    assert_eq!(
        island_id(&mut m, &code),
        old,
        "Insel getauscht, obwohl es keine nächste gibt"
    );
}

// AC-01 (KB2.1, B-385): Beim Inselwechsel bekommt jedes Gerät den letzten Zustand der alten Insel mit dem Ereignis
// `islandSwitch`, danach `seats`, Level und Zustand der neuen Insel.
#[test]
fn kampf_inselwechsel_ereignis_erreicht_geraete() {
    let mut g = nordmeer_gate();
    let old = island_id(&mut g.m, &g.code);
    let (xi, hi) = (g.x.msgs().len(), g.h.msgs().len());
    rm(&mut g.m, &g.code).isl.endboss_defeated = true;
    assert!(until_switched(&mut g, old), "Insel nicht getauscht");
    let want: Vec<String> = ["state 0", "islandSwitch 0", "level 0", "state 0"]
        .iter()
        .map(|s| s.to_string())
        .collect(); // der Steg liegt in Stufe 0 (Hafen)
    for (name, got) in [
        ("Xbox", g.x.msgs()[xi..].to_vec()),
        ("Handy", g.h.msgs()[hi..].to_vec()),
    ] {
        let i = got.iter().position(|m| m == "islandSwitch 0");
        assert!(i.is_some_and(|i| i >= 1 && got[i - 1..] == want[..]), "{name}: {got:?}");
    }
}
