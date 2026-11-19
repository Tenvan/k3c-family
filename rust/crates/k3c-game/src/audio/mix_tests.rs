//! Port von `native/k3c-game/audio/mix_test.go`.

use std::rc::Rc;
use std::sync::Arc;

use serde_json::{Value, json};

use k3c_sim::state::{CycleInfo, Enemy, Player, World};

use super::*;
use crate::audio::events::{EDGE_GAIN, NIGHT_GAIN_VAL, SLOT_UI_CONFIRM, SLOT_UI_SELECT};
use crate::audio::music::CROSSFADE_TICKS;
use crate::audio::register::Register;
use crate::audio::testutil::real_register;
use crate::view::cell::{Camera, Cell};
use crate::view::frame::{Cue, CueKind, Seat, Stage};
use crate::view::settings::default_settings;

fn planner() -> Planner {
    Planner::new(Arc::new(real_register()))
}

/// Baut eine Welt mit Monarchen an den Orten `xs` (Index = Position) zur Tageszeit `phase`.
fn stage_world(phase: &str, xs: &[f64]) -> World {
    let mut w = World {
        cycle: CycleInfo {
            phase: phase.to_string(),
            ..CycleInfo::default()
        },
        ..World::default()
    };
    for (i, x) in xs.iter().enumerate() {
        w.players.push(Player {
            index: i as i64,
            x: *x,
            ..Player::default()
        });
    }
    w
}

fn ev(v: &Value) -> Event {
    Event {
        data: v.as_object().unwrap().clone(),
        ..Event::default()
    }
}

/// Ein Frame mit einer Stufe und n lokalen Spielern darin.
fn game_frame(w: &World, n: i64, events: &[Value]) -> Frame {
    Frame {
        settings: default_settings(),
        stages: vec![Stage {
            index: 0,
            world: Some(Rc::new(w.clone())),
            ..Stage::default()
        }],
        seats: (0..n)
            .map(|i| Seat {
                player: i,
                monarch: i,
                ..Seat::default()
            })
            .collect(),
        events: events.iter().map(ev).collect(),
        ..Frame::default()
    }
}

fn menu_frame() -> Frame {
    Frame {
        settings: default_settings(),
        ..Frame::default()
    }
}

/// Pegel und Anzahl der Aufträge eines Slots.
fn effect(m: &Mix, slot: &str) -> (f64, usize) {
    let mut r = (0.0, 0);
    for q in m.effects.iter().filter(|q| q.slot == slot) {
        r = (q.gain, r.1 + 1);
    }
    r
}

// Beispiel B-801: Monarch zahlt eine Münze → Münz-Klang, voll im Bild, nachts leiser.
#[test]
fn plan_coin_give() {
    let mut p = planner();
    let m = p.plan(&game_frame(
        &stage_world("day", &[10.0]),
        1,
        &[json!({"type": "coinGive", "x": 10.0})],
    ));
    assert_eq!(effect(m, "sfx:coinGive"), (1.0, 1), "Tag");
    let m = p.plan(&game_frame(
        &stage_world("night", &[10.0]),
        1,
        &[json!({"type": "coinGive", "x": 10.0})],
    ));
    assert_eq!(effect(m, "sfx:coinGive").0, NIGHT_GAIN_VAL, "Nacht");
}

// Drosselung: viele gleiche Klänge in einem Takt klingen einmal, mit dem lautesten Pegel; ungleiche bleiben getrennt.
#[test]
fn plan_throttles_same_slot_per_tick() {
    let mut p = planner();
    let far = 10.0 + span_per_seat(); // außerhalb des Bilds: gedämpft
    let w = stage_world("day", &[10.0]);
    let m = p.plan(&game_frame(
        &w,
        1,
        &[
            json!({"type": "hit", "x": far}),
            json!({"type": "hit", "x": 10.0}),
            json!({"type": "hit", "x": far}),
            json!({"type": "kill", "x": far}),
            json!({"type": "unbekannt", "x": 10.0}),
        ],
    ));
    assert_eq!(effect(m, "sfx:hit"), (1.0, 1));
    let (g, n) = effect(m, "sfx:kill");
    assert!(n == 1 && g > 0.0 && g < EDGE_GAIN, "kill gedämpft: {g} ×{n}");
    assert_eq!(m.effects.len(), 2, "Aufträge {:?}", m.effects);
    let m = p.plan(&game_frame(&w, 1, &[]));
    assert!(m.effects.is_empty(), "nächster Takt ohne Ereignis: still");
}

// Zwei Spieler: Klänge beider Ausschnitte, nichts doppelt; das nähere Ohr bestimmt den Pegel.
#[test]
fn plan_two_players() {
    let mut p = planner();
    let w = stage_world("day", &[0.0, 500.0]);
    let evs = [
        json!({"type": "coinPickup", "x": 500.0}),
        json!({"type": "arrow", "x": 0.0}),
    ];
    let m = p.plan(&game_frame(&w, 2, &evs));
    for slot in ["sfx:coin", "sfx:arrow"] {
        assert_eq!(effect(m, slot), (1.0, 1), "{slot}");
    }
    let mut solo = planner();
    let m = solo.plan(&game_frame(&w, 1, &evs[..1]));
    assert_eq!(effect(m, "sfx:coin").0, 0.0, "allein weit weg");
}

// Warnung: global (ungedämpft) und senkt die Musik; skillCast nach Linie.
#[test]
fn plan_warn_and_skill() {
    let mut p = planner();
    let w = stage_world("day", &[0.0]);
    for _ in 0..CROSSFADE_TICKS {
        p.plan(&game_frame(&w, 1, &[]));
    }
    assert_eq!(p.mix.music[0].gain, 1.0, "Tagesmusik");
    p.plan(&game_frame(
        &w,
        1,
        &[
            json!({"type": "dusk"}),
            json!({"type": "skillCast", "skill": "fireball", "x": 0.0}),
        ],
    ));
    let duck = p.reg.mix.duck;
    for _ in 0..(duck.attack * TPS as f64).ceil() as i64 {
        p.plan(&game_frame(&w, 1, &[]));
    }
    assert!(
        (p.mix.music[0].gain - duck.level).abs() < 1e-9,
        "Ducking: {}",
        p.mix.music[0].gain
    );
    let m = p.plan(&game_frame(
        &w,
        1,
        &[
            json!({"type": "wave", "x": 900.0}),
            json!({"type": "skillCast", "skill": "heal", "x": 0.0}),
        ],
    ));
    assert_eq!(effect(m, "sfx:warn"), (1.0, 1), "Warnung");
    assert_eq!(effect(m, "sfx:skill:healer").1, 1, "Skill-Klang der Linie fehlt");
}

#[test]
fn plan_hub_and_menu_cues() {
    let mut p = planner();
    let mut w = stage_world("day", &[0.0]);
    w.hub_level = 1;
    assert_eq!(
        effect(p.plan(&game_frame(&w, 1, &[])), SLOT_HUB).1,
        0,
        "erstes Sehen klingt nicht"
    );
    w.hub_level = 2;
    assert_eq!(effect(p.plan(&game_frame(&w, 1, &[])), SLOT_HUB).1, 1, "Ausbau klingt");
    let cue = |kind| Cue {
        kind,
        player: -1,
        ..Cue::default()
    };
    let mut f = menu_frame();
    f.cues = vec![cue(CueKind::Move), cue(CueKind::Confirm), cue(CueKind::Pause)];
    let m = p.plan(&f);
    let slots: Vec<&str> = m.effects.iter().map(|r| r.slot.as_ref()).collect();
    assert_eq!(slots, [SLOT_UI_SELECT, SLOT_UI_CONFIRM], "Menü-Klänge");
}

// Beispiel B-801: Nacht beginnt → Musik wechselt mit Überblenden; Menü ohne Spiel = Lobby; Burg fällt = Niederlage.
#[test]
fn plan_music_follows_state() {
    let mut p = planner();
    let menu = menu_frame();
    p.plan(&menu);
    assert!(
        p.state == MusicState::Lobby && p.mix.music[0].slot == "music:lobby",
        "Menü: {:?}",
        p.mix.music
    );
    let mut w = stage_world("day", &[0.0]);
    p.plan(&game_frame(&w, 1, &[]));
    w.cycle.phase = "night".into();
    let m = p.plan(&game_frame(&w, 1, &[json!({"type": "night"})]));
    assert!(
        m.music[0].slot == "music:nacht" && m.music[1].slot == "music:tag",
        "Nacht blendet über: {:?}",
        m.music
    );
    w.enemies = vec![Enemy::default()];
    p.plan(&game_frame(&w, 1, &[]));
    assert_eq!(p.state, MusicState::Battle, "Gegner");
    p.plan(&game_frame(&w, 1, &[json!({"type": "castleFallen"})]));
    w.enemies.clear();
    p.plan(&game_frame(&w, 1, &[]));
    assert_eq!(p.state, MusicState::Defeat, "Niederlage bleibt");
    p.plan(&menu);
    assert_eq!(p.state, MusicState::Lobby, "zurück im Menü");
}

// Hufschlag je reitendem Monarch; in der Pause und für Stehende keiner; Lücke im Register = Stille.
#[test]
fn plan_ride_loops_and_pause() {
    let mut p = planner();
    let mut w = stage_world("night", &[0.0, 3.0, 5.0]);
    w.players[0].vx = 2.0;
    w.players[1].vx = 0.1;
    w.players[2].vx = 2.0;
    w.players[2].crown = Some(Default::default());
    let m = p.plan(&game_frame(&w, 1, &[]));
    assert!(
        m.loops.len() == 1 && m.loops[0].key == 0 && m.loops[0].gain == NIGHT_GAIN_VAL * RIDE_GAIN,
        "Schleifen {:?}",
        m.loops
    );
    let mut f = game_frame(&w, 1, &[]);
    f.paused = true;
    assert!(p.plan(&f).loops.is_empty(), "Pause: kein Hufschlag");
    let mut empty = Planner::new(Arc::new(Register::default()));
    let m = empty.plan(&game_frame(&w, 1, &[]));
    assert_eq!(m.music[0].slot, "", "ohne Register still: {:?}", m.music);
}

// Der Listen-Pfad legt ab dem zweiten Aufruf keine Puffer neu an (Go: AllocsPerRun; hier: Kapazitäten bleiben).
#[test]
fn plan_no_new_buffers_per_frame() {
    let mut p = planner();
    let mut w = stage_world("day", &[0.0, 4.0]);
    w.players[1].vx = 2.0;
    let f = game_frame(&w, 2, &[]);
    p.plan(&f);
    p.plan(&f);
    let caps = |p: &Planner| {
        [
            p.mix.effects.capacity(),
            p.mix.loops.capacity(),
            p.hubs.capacity(),
            p.next.capacity(),
            p.ears.capacity(),
        ]
    };
    let before = caps(&p);
    assert!(before[1] > 0 && before[4] > 0, "Schleife und Ohren benutzt: {before:?}");
    for _ in 0..100 {
        p.plan(&f);
    }
    assert_eq!(caps(&p), before);
}

// Mit Zellen im Frame gilt der sichtbare Bereich der Kamera, nicht die feste Breite um den Monarchen.
#[test]
fn ears_follow_cell_camera() {
    let mut p = planner();
    let cell = |x: f64| Cell {
        player: 0,
        camera: Camera {
            x,
            scale: 10.0,
            half: 200.0,
            ..Camera::default()
        },
        ..Cell::default()
    };
    let hit = json!({"type": "hit", "x": 100.0});
    let mut f = game_frame(&stage_world("day", &[10.0]), 1, std::slice::from_ref(&hit));
    f.cells = vec![cell(100.0)];
    assert_eq!(effect(p.plan(&f), "sfx:hit").0, 1.0, "im Bild der Kamera");
    let mut f = game_frame(&stage_world("day", &[100.0]), 1, &[hit]);
    f.cells = vec![cell(10.0)];
    let g = effect(p.plan(&f), "sfx:hit").0;
    assert!(g < EDGE_GAIN, "außerhalb der Kamera, am Monarchen: {g}");
}
