//! Port von `native/k3c-game/actors/marks_test.go`.

use std::rc::Rc;

use k3c_sim::state::{Enemy, Job, Player, Troop, World as SimWorld};

use super::*;
use crate::actors::figures::{Class, FLEE_ALPHA, Key, STEALTH_MAX};
use crate::actors::new;
use crate::view::frame::{Frame, Seat, Stage};

fn frame_of(clock: f64, w: &SimWorld) -> Frame {
    Frame {
        clock,
        stages: vec![Stage {
            index: 0,
            level: "wald".to_string(),
            world: Some(Rc::new(w.clone())),
            ..Stage::default()
        }],
        ..Frame::default()
    }
}

/// Fälle aus src/scenes/render/worldRenderer.ts: setBar(hp / maxHp, sichtbar bei hp < maxHp), Monarchen zusätzlich
/// nur für Truppen (36) und Gegner (44), Höhe 8; Monarchen haben keine HP und keinen Balken mehr (B-930).
#[test]
fn bar_rules_like_browser() {
    // (name, hp, max, sichtbar, Anteil)
    let cases = [
        ("voll", 100.0, 100.0, false, 1.0),
        ("halb", 50.0, 100.0, true, 0.5),
        ("fast tot", 1.0, 100.0, true, 0.01),
        ("tot", 0.0, 100.0, true, 0.0),
        ("Monarch im Respawn", 0.0, 100.0, true, 0.0),
        ("über Maximum", 120.0, 100.0, false, 1.0),
        ("ohne Lebenswert", 0.0, 0.0, false, 0.0),
    ];
    for (name, hp, max, visible, frac) in cases {
        assert_eq!(bar_visible(hp, max), visible, "{name}: sichtbar");
        assert!((bar_fraction(hp, max) - frac).abs() <= 1e-9, "{name}: Anteil");
    }
}

#[test]
fn bar_boxes_width_and_placement() {
    for (look, width, above) in [(TROOP_BAR, 36.0, 16.0), (ENEMY_BAR, 44.0, 32.0)] {
        let (back, fill) = bar_boxes(look, 100.0, 0.5);
        assert!(
            back.w == width && back.h == BAR_HEIGHT_PX && back.cx == 0.0 && back.cy == 100.0 + above,
            "Hintergrund {back:?}: Breite {width}, Mitte {above} über der Oberkante erwartet"
        );
        assert!(
            fill.w == width / 2.0 && fill.h == BAR_HEIGHT_PX && fill.cy == back.cy,
            "Füllung {fill:?}: halbe Breite erwartet"
        );
        assert_eq!(fill.cx - fill.w / 2.0, back.cx - back.w / 2.0, "Füllung beginnt links");
    }
    let (_, full) = bar_boxes(TROOP_BAR, 0.0, 1.0);
    assert!(
        full.cx == 0.0 && full.w == 36.0,
        "voller Balken deckt den Hintergrund: {full:?}"
    );
    // Balkenfarben wie im Browser: Truppe grün, Gegner rot
    const { assert!(TROOP_BAR.color == 0x52b788 && ENEMY_BAR.color == 0xe63946) };
}

#[test]
fn loot_only_with_carried_gold() {
    assert!(
        !has_loot(0) && !has_loot(-1) && has_loot(1) && has_loot(40),
        "Goldkreis nur bei carriedGold > 0"
    );
    assert_eq!(loot_center(80.0), (0.0, 92.0), "Goldkreis 12 px über der Oberkante");
}

#[test]
fn player_marker_color_and_seats() {
    let want = [0x3a86ff_u32, 0xff006e, 0xfb5607, 0x8338ec, 0x3a86ff];
    for (i, w) in want.iter().enumerate() {
        #[allow(clippy::cast_possible_wrap)]
        let got = player_color(i as i64);
        assert_eq!(got, *w, "Spieler {i}");
    }
    assert_ne!(
        player_color(0),
        player_color(1),
        "zwei Spieler haben verschiedene Farben"
    );
    let b = marker_box();
    assert!(
        b.cy == -4.0 && b.w == 56.0 && b.h == 12.0,
        "Punkt 4 px unter dem Boden, 56 × 12: {b:?}"
    );
    let seats = [
        Seat {
            player: 0,
            monarch: 0,
            ..Seat::default()
        },
        Seat {
            player: 1,
            monarch: 2,
            ..Seat::default()
        },
    ];
    assert!(
        seated(&seats, 0) && seated(&seats, 2) && !seated(&seats, 1) && !seated(&[], 0),
        "freier Sitz (Monarch ohne Platz) wird nicht gezeichnet"
    );
}

#[test]
fn load_only_when_carrying() {
    let job = |t: &str| Job {
        r#type: t.to_string(),
        ..Job::default()
    };
    assert!(!carrying(None));
    assert!(!carrying(Some(&job("gather"))));
    assert!(!carrying(Some(&job("build"))));
    assert!(carrying(Some(&job("carry"))));
    // Hinter dem Rücken: Blick nach rechts → links der Figur (-14), nach links → rechts (+14); 16 × 24 auf 55 % Höhe.
    assert!(
        load_offset(false) == -14.0 && load_offset(true) == 14.0,
        "Last sitzt hinter dem Rücken"
    );
    let b = load_box(100.0, true);
    assert!(
        b.cx == 14.0 && (b.cy - 55.0).abs() <= 1e-9 && b.w == 16.0 && b.h == 24.0,
        "Last: {b:?}"
    );
}

/// Die Deckkraft der Anzeigen folgt dem Container des Browsers: fliehende Gegner 0,7, Monarch in der Smoke Bomb 0,35.
#[test]
fn ui_alpha_follows_container() {
    let mut d = new(None);
    assert!(d.cat.is_some(), "kein Katalog");
    let w = SimWorld {
        players: vec![Player {
            id: 1,
            index: 0,
            stealth_for: 3.0,
            ..Player::default()
        }],
        troops: vec![Troop {
            id: 2,
            kind: "archer".to_string(),
            ..Troop::default()
        }],
        enemies: vec![
            Enemy {
                id: 3,
                kind: "goblin".to_string(),
                fleeing: true,
                ..Enemy::default()
            },
            Enemy {
                id: 4,
                kind: "goblin".to_string(),
                ..Enemy::default()
            },
        ],
        ..SimWorld::default()
    };
    d.listen(&frame_of(0.0, &w));
    let key = |class, id| Key { stage: 0, class, id };
    for (k, want) in [
        (key(Class::Player, 1), STEALTH_MAX),
        (key(Class::Troop, 2), 1.0),
        (key(Class::Enemy, 3), FLEE_ALPHA),
        (key(Class::Enemy, 4), 1.0),
    ] {
        let fg = d.figs.get(&k).unwrap_or_else(|| panic!("{k:?}: keine Figur"));
        assert_eq!(fg.ui, want, "{k:?}: Anzeigen-Deckkraft");
    }
}
