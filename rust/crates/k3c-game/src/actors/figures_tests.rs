//! Port von `native/k3c-game/actors/figures_test.go`.

use std::rc::Rc;

use k3c_sim::state::{Enemy, Player, Site, Troop, World as SimWorld};

use super::*;
use crate::actors::anim::{ANIM_RUN, GAIT_GALLOP, GAIT_STAND, GAIT_WALK};
use crate::actors::catalog::UNIT_PX;
use crate::actors::new;
use crate::view::frame::Stage;

/// newTestDrawer: Drawer ohne Grafik (lib `None`); Listen rechnet die Posen trotzdem.
fn new_test_drawer() -> Actors {
    let d = new(None);
    assert!(d.cat.is_some(), "kein Katalog");
    d
}

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

fn key(class: Class, id: i64) -> Key {
    Key { stage: 0, class, id }
}

fn player(id: i64, index: i64, x: f64) -> Player {
    Player {
        id,
        index,
        x,
        ..Player::default()
    }
}

// Beispiel B-796: Zwei Monarchen reiten in entgegengesetzte Richtungen; jeder blickt in seine Richtung, Gangart je
// Monarch eigen.
#[test]
fn two_monarchs_own_gait_and_facing() {
    let mut d = new_test_drawer();
    let mut w = SimWorld {
        players: vec![
            Player {
                vx: 5.0,
                facing: 1,
                ..player(1, 0, 10.0)
            },
            Player {
                vx: -1.0,
                facing: -1,
                ..player(2, 1, 12.0)
            },
        ],
        ..SimWorld::default()
    };
    d.listen(&frame_of(1.0, &w));
    let (a, b) = (&d.figs[&key(Class::Player, 1)], &d.figs[&key(Class::Player, 2)]);
    assert!(!a.left && b.left, "Blickrichtung: {} {}", a.left, b.left);
    assert!(
        a.pose.gait == GAIT_GALLOP && b.pose.gait == GAIT_WALK,
        "Gangarten {:?} {:?}",
        a.pose.gait,
        b.pose.gait
    );
    assert!(
        a.look.variant.is_empty() && b.look.variant == "spieler2" && a.shift != b.shift,
        "Variante oder Versatz gleich: {:?} {:?}",
        a.look,
        b.look
    );
    w.players[0].vx = 0.0; // steht: Blick bleibt
    w.players[0].facing = 0;
    d.listen(&frame_of(1.1, &w));
    let a = &d.figs[&key(Class::Player, 1)];
    assert!(!a.left && a.pose.gait == GAIT_STAND, "stehender Monarch: {:?}", a.pose);
}

// Smoke Bomb: Der unsichtbare Monarch wird fast durchsichtig, danach wieder deckend.
#[test]
fn monarch_stealth_alpha() {
    let mut d = new_test_drawer();
    let mut w = SimWorld {
        players: vec![Player {
            stealth_for: 2.0,
            ..player(1, 0, 10.0)
        }],
        ..SimWorld::default()
    };
    d.listen(&frame_of(1.0, &w));
    let fg = &d.figs[&key(Class::Player, 1)];
    assert!(
        fg.alpha == STEALTH_MAX,
        "unsichtbar: alpha {}, erwartet {STEALTH_MAX}",
        fg.alpha
    );
    w.players[0].stealth_for = 0.0;
    d.listen(&frame_of(1.1, &w));
    let fg = &d.figs[&key(Class::Player, 1)];
    assert!(fg.alpha == 1.0, "sichtbar: alpha {}, erwartet 1", fg.alpha);
}

// Truppen und Gegner: Richtung aus der Bewegung, Turm hebt die Truppe, Fliehen blendet, Fledermaus fliegt,
// verschwundene Figuren fallen heraus, ein Berufswechsel legt die Figur neu an.
#[test]
#[allow(clippy::too_many_lines)]
fn troops_and_enemies() {
    let mut d = new_test_drawer();
    let troop = |id: i64, kind: &str, x: f64, tower_id: Option<i64>| Troop {
        id,
        kind: kind.to_string(),
        x,
        tower_id,
        ..Troop::default()
    };
    let enemy = |id: i64, kind: &str, x: f64, fleeing: bool| Enemy {
        id,
        kind: kind.to_string(),
        x,
        fleeing,
        ..Enemy::default()
    };
    let mut w = SimWorld {
        sites: vec![Site {
            id: 7,
            x: 20.0,
            ..Site::default()
        }],
        troops: vec![troop(1, "archer", 20.0, Some(7)), troop(2, "vagrant", 5.0, None)],
        enemies: vec![enemy(3, "goblin", 30.0, true), enemy(4, "bat", 15.0, false)],
        ..SimWorld::default()
    };
    d.listen(&frame_of(1.0, &w));
    w.enemies[0].x = 30.5;
    d.listen(&frame_of(1.1, &w));
    let archer = &d.figs[&key(Class::Troop, 1)];
    let gob = &d.figs[&key(Class::Enemy, 3)];
    let bat = &d.figs[&key(Class::Enemy, 4)];
    assert!(
        archer.lift == TOWER_LIFT && archer.left, // nächster Gegner: Fledermaus links
        "Bogenschütze auf dem Turm: lift {}, links {}",
        archer.lift,
        archer.left
    );
    assert!(
        !gob.left && gob.pose.anim == ANIM_RUN && gob.alpha == FLEE_ALPHA,
        "fliehender Goblin: {:?} left {} alpha {}",
        gob.pose,
        gob.left,
        gob.alpha
    );
    assert!(
        bat.lift == 130.0 / UNIT_PX && bat.look.sheet,
        "Fledermaus: lift {}",
        bat.lift
    );
    let old_look = d.figs[&key(Class::Troop, 2)].look.clone();
    w.troops[1].kind = "peasant".to_string();
    w.enemies.truncate(1);
    d.listen(&frame_of(1.2, &w));
    assert!(
        !d.figs.contains_key(&key(Class::Enemy, 4)),
        "verschwundene Fledermaus bleibt"
    );
    let nw = &d.figs[&key(Class::Troop, 2)];
    assert!(
        nw.look != old_look && nw.look.asset == "truppen/bauer",
        "Bauer nicht neu angelegt: {:?}",
        nw.look
    );
}

// Unbekannte Arten erscheinen nicht (Meldung einmal), die Pause hält die Darstellungszeit an.
#[test]
fn unknown_kind_and_pause() {
    let mut d = new_test_drawer();
    let w = SimWorld {
        enemies: vec![Enemy {
            id: 1,
            kind: "gibtsnicht".to_string(),
            ..Enemy::default()
        }],
        ..SimWorld::default()
    };
    d.listen(&frame_of(1.0, &w));
    d.listen(&frame_of(2.0, &w));
    let wk = WarnKey {
        what: "gegner".to_string(),
        name: "gibtsnicht".to_string(),
    };
    assert!(
        d.figs.is_empty() && d.warned.get(&wk).copied().unwrap_or(false),
        "unbekannter Gegner: {} Figuren",
        d.figs.len()
    );
    let mut f = frame_of(3.0, &w);
    f.paused = true;
    d.listen(&f);
    assert!(d.now == 1.0, "Zeit läuft in der Pause: {}", d.now);
}
