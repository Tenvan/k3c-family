//! Tests des Zeichners und des Drawers (Go prüft sie nur über die Beispiele im Spiel): `Drawer::draw` läuft gegen
//! `NullCanvas`.

use serde_json::json;

use k3c_platform::NullCanvas;
use k3c_sim::state::{Enemy, Projectile, Storm, WarnZone, World};

use super::paint::{rgba, tint};
use super::scale_of;
use super::testutil::{event_of, new_fx, stage_of};
use crate::view::cell::{Cell, new_camera, rect};
use crate::view::frame::{Frame, Seat};
use crate::view::layer::{Drawer, Layer, Listener};
use crate::view::roles::AssetSource;

fn cell() -> Cell {
    let r = rect(0, 0, 1920, 1080);
    let mut c = Cell {
        player: 0,
        rect: r,
        ..Cell::default()
    };
    c.camera = new_camera(r, 5.0, 1080.0 * 2.0 / 3.0 / 13.0);
    c
}

fn world() -> World {
    let mut w = World::default();
    w.enemies.push(Enemy {
        x: 6.0,
        hp: 5.0,
        stun: 1.0,
        slow_for: 1.0,
        taunt_for: 1.0,
        warn: Some(WarnZone {
            x: 7.0,
            r: 2.0,
            r#in: 1.0,
        }),
        ..Enemy::default()
    });
    w.projectiles.push(Projectile {
        x: 5.0,
        team: "enemy".into(),
        ..Projectile::default()
    });
    w.storms.push(Storm {
        x: 4.0,
        radius: 2.0,
        left: 3.0,
        ..Storm::default()
    });
    w
}

#[test]
fn draw_zeichnet_mit_nullcanvas() {
    let mut d = new_fx();
    let w = world();
    let mut f = Frame {
        stages: vec![stage_of(0, &w)],
        seats: vec![Seat::default()],
        clock: 1.0,
        ..Frame::default()
    };
    f.events = vec![
        event_of(0, json!({"type": "hit", "x": 5.0, "target": "enemy", "damage": 2.0})),
        event_of(0, json!({"type": "built", "x": 4.0, "kind": "wall"})),
        event_of(
            0,
            json!({"type": "skillCast", "from": 0, "skill": "meteor", "x": 6.0, "radius": 2.0}),
        ),
    ];
    d.listen(&f);
    assert!(!d.stage(0).expect("Stufe 0").effects.is_empty(), "Effekte angelegt");
    f.clock = 1.05;
    let mut canvas = NullCanvas::default();
    d.draw(&mut canvas, &f, &cell());
    assert!(canvas.textures >= 4, "Vorlagen angelegt: {}", canvas.textures);
    assert!(
        canvas.sprites > 5,
        "Effekte, Warnkreis und Geschoss gezeichnet: {}",
        canvas.sprites
    );
    let (t, s) = (canvas.textures, canvas.sprites);
    d.draw(&mut canvas, &f, &cell());
    assert_eq!(canvas.textures, t, "Vorlagen nur einmal");
    assert!(canvas.sprites > s, "zweiter Aufruf zeichnet wieder");
}

#[test]
fn draw_ohne_spieler_oder_welt_zeichnet_nichts() {
    let d = new_fx();
    let mut canvas = NullCanvas::default();
    d.draw(&mut canvas, &Frame::default(), &cell());
    assert_eq!((canvas.textures, canvas.sprites), (0, 0));
}

#[test]
fn ebene_und_assets() {
    let d = new_fx();
    assert_eq!(d.layer(), Layer::EFFECTS);
    assert_eq!(d.assets("wald", &World::default()), vec!["icons/muenze".to_string()]);
}

#[test]
fn abgelaufene_fallen_und_stufen_kehren_leer_zurueck() {
    let mut d = new_fx();
    let w = World::default();
    let mut f = Frame {
        stages: vec![stage_of(0, &w)],
        seats: vec![Seat::default()],
        ..Frame::default()
    };
    f.events = vec![event_of(0, json!({"type": "kill", "x": 1.0}))];
    d.listen(&f);
    assert_eq!(d.stage(0).expect("Stufe 0").effects.len(), 1);
    f.events.clear();
    f.clock = 5.0;
    d.listen(&f);
    assert!(d.stage(0).expect("Stufe 0").effects.is_empty(), "abgelaufen");
    f.stages.clear();
    d.listen(&f);
    assert!(d.stage(0).is_none(), "Stufe ohne Frame ist nicht aktiv");
    f.stages = vec![stage_of(0, &w)];
    d.listen(&f);
    assert!(
        d.stage(0).is_some_and(|s| s.effects.is_empty()),
        "nach einer Lücke leer"
    );
}

#[test]
fn farben_und_masstab() {
    assert_eq!(rgba(0xff0000, 0.5), [127, 0, 0, 127]);
    assert_eq!(rgba(0xffffff, 2.0), [255, 255, 255, 255], "Deckkraft begrenzt");
    assert_eq!(tint(rgba(0, 0.0)), [0.0; 4]);
    let t = tint([127, 0, 0, 127]);
    assert!(t[0] == 1.0 && t[3] == 127.0 / 255.0, "gerade Farbe: {t:?}");
    assert_eq!(scale_of(&cell(), 1), 1.0);
    assert_eq!(scale_of(&cell(), 2), 2.0);
}
