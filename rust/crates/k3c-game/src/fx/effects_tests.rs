//! Port von `native/k3c-game/fx/effects_test.go`.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use k3c_sim::state::Event;

use k3c_sim::state::{Player, World};

use super::effects::*;
use super::looks::*;
use super::paint::{ENEMY_PROJECTILE, OWN_PROJECTILE, projectile_color};
use super::stage::{StageFx, cast_for};
use super::testutil::{event_of, new_fx, stage_of};
use crate::view::frame::{Frame, Seat};
use crate::view::layer::Listener;

fn ev(v: Value) -> Event {
    v.as_object().expect("Objekt").clone()
}

fn fx_of(d: &Event, spots: &mut BTreeMap<String, f64>) -> (Effect, bool) {
    let px = |i: i64| (5.0, i == 0);
    effect_for("de", d, &str_of(d, "type"), &px, spots)
}

/// effectFor (effects.test.ts, GR5.1 AC-01/AC-03): je Feedback-Ereignis genau ein Effekt am Ort des Ereignisses.
#[test]
fn effekt_je_ereignis() {
    let cases = [
        (
            json!({"type": "hit", "x": 3.0, "target": "enemy", "id": 1, "damage": 2.0}),
            Kind::Hit,
            3.0,
        ),
        (
            json!({"type": "kill", "kind": "gnoll", "x": 4.0, "gold": 1}),
            Kind::Kill,
            4.0,
        ),
        (json!({"type": "coinPickup", "player": 0, "x": 2.0}), Kind::Coin, 2.0),
        (
            json!({"type": "coinGive", "player": 0, "x": 2.0, "to": "site"}),
            Kind::CoinGive,
            2.0,
        ),
        (
            json!({"type": "buildProgress", "site": 1, "kind": "wall", "x": 7.0, "percent": 25}),
            Kind::Build,
            7.0,
        ),
        (json!({"type": "crownFallen", "player": 0}), Kind::Down, 5.0),
        (json!({"type": "crownFallen", "player": 3, "x": 4.0}), Kind::Down, 4.0),
        (json!({"type": "built", "kind": "tower", "x": 9.0}), Kind::Built, 9.0),
        (
            json!({"type": "strike", "from": 4, "x": 10.0, "hit": false}),
            Kind::Swing,
            10.0,
        ),
    ];
    for (d, kind, x) in cases {
        let d = ev(d);
        let (e, ok) = fx_of(&d, &mut BTreeMap::new());
        assert!(ok && e.kind == kind && e.x == x, "{d:?}: {e:?} {ok}");
    }
    for d in [
        json!({"type": "dusk"}),
        json!({"type": "zukunft", "x": 1.0}),
        json!({"type": "crownFallen", "player": 3}),
        json!({"type": "strike", "from": 7, "x": 3.0}),
    ] {
        let d = ev(d);
        let (_, ok) = fx_of(&d, &mut BTreeMap::new());
        assert!(!ok, "{d:?}: kein Effekt");
    }
}

#[test]
fn effekt_bau_ort_und_rueckmeldung() {
    let mut spots = BTreeMap::new();
    let (_, ok) = fx_of(&ev(json!({"type": "built", "kind": "wall"})), &mut spots);
    assert!(!ok, "built ohne Ort und ohne Baufortschritt: nichts");
    fx_of(
        &ev(json!({"type": "buildProgress", "kind": "wall", "x": 7.0})),
        &mut spots,
    );
    let (e, ok) = fx_of(&ev(json!({"type": "built", "kind": "wall"})), &mut spots);
    assert!(ok && e.x == 7.0, "built am Ort des letzten Baufortschritts: {e:?}");
    let (e, _) = fx_of(
        &ev(json!({"type": "strike", "from": 9, "x": 12.0, "hit": true})),
        &mut spots,
    );
    assert_eq!(e.by, 9, "Ausholen gehört dem Spieler from");
    let (a, _) = fx_of(&ev(json!({"type": "castFailed", "from": 1, "x": 10.0})), &mut spots);
    let nobody = |_: i64| (0.0, false);
    let (b, _) = effect_for("en", &ev(json!({"x": 40.0})), "castFailed", &nobody, &mut spots);
    assert!(
        a.text == "kein Ziel" && b.text == "no target" && a.x != b.x,
        "„kein Ziel“ am eigenen Ort: {a:?} {b:?}"
    );
}

#[test]
fn coin_loss_for_faelle() {
    let (e, ok) = coin_loss_for(&ev(json!({"x": 3.0, "target": "player", "damage": 1.0})));
    assert!(ok && e.kind == Kind::CoinLoss, "Treffer am Monarchen: {e:?}, {ok}");
    for d in [
        json!({"x": 3.0, "target": "enemy", "damage": 1.0}),
        json!({"x": 3.0, "target": "player", "damage": 0.0}),
        json!({"target": "player", "damage": 1.0}),
    ] {
        let (_, ok) = coin_loss_for(&ev(d.clone()));
        assert!(!ok, "kein Münzschauer erwartet: {d}");
    }
}

#[test]
fn schadenszahl() {
    let (e, ok) = damage_for(&ev(json!({"type": "hit", "x": 3.0, "target": "enemy", "damage": 2.4})));
    assert!(
        ok && e.text == "3" && !e.target && e.kind == Kind::Damage,
        "Gegner: aufgerundete Zahl: {e:?}"
    );
    let (e, _) = damage_for(&ev(json!({"x": 1.0, "target": "player", "damage": 5.0})));
    assert!(e.text == "5" && e.target, "Monarch getroffen: rot, {e:?}");
    let (_, ok) = damage_for(&ev(json!({"x": 1.0, "damage": 0.0})));
    assert!(!ok, "ohne Schaden keine Zahl");
}

/// Blitzgrenze (effectRules.test.ts, GR5.2 AC-05): höchstens 3 Blitze je Sekunde.
#[test]
fn blitzgrenze() {
    let (mut last, mut ran) = (-1.0, 0);
    let mut t0 = 0.0;
    while t0 < 1.0 {
        if flash_allowed(t0, last) {
            (last, ran) = (t0, ran + 1);
        }
        t0 += 1.0 / 60.0;
    }
    assert!(ran <= MAX_FLASHES_PER_SECOND, "{ran} Blitze in einer Sekunde");
    assert!(
        !flash_allowed(FLASH_MIN_GAP - 0.01, 0.0) && flash_allowed(FLASH_MIN_GAP, 0.0),
        "Sperrfenster"
    );
}

#[test]
fn spawn_grenzen() {
    let mut s = StageFx::new(0);
    for i in 0..20 {
        s.spawn(Effect::new(Kind::Kill, 0.0, 0.0), f64::from(i));
    }
    assert!(
        s.pieces <= MAX_LIVE_PIECES && s.effects.len() as i64 == MAX_LIVE_PIECES / 7,
        "höchstens {MAX_LIVE_PIECES} Teile: {} in {} Effekten",
        s.pieces,
        s.effects.len()
    );
    let mut s = StageFx::new(0);
    let mut no = Effect::new(Kind::NoTarget, 10.0, 0.0);
    no.text = "kein Ziel".into();
    s.spawn(no.clone(), 0.0);
    no.x = 10.5;
    s.spawn(no, 0.1);
    assert_eq!(s.effects.len(), 1, "derselbe Text an derselben Stelle nur einmal");
    s.spawn(Effect::new(Kind::Hit, 0.0, 0.0), 0.0);
    s.spawn(Effect::new(Kind::Hit, 0.0, 0.0), 0.1);
    assert_eq!(s.effects.len(), 2, "Blitz im Sperrfenster verworfen");
    s.expire(0.5);
    assert!(
        s.effects.len() == 1 && s.effects[0].kind == Kind::NoTarget,
        "abgelaufene fallen weg: {:?}",
        s.effects
    );
}

#[test]
fn skill_looks_eigener_und_unbekannter() {
    let l = skill_look("fireball");
    assert!(
        l.is_some_and(|l| l.color == 0xdf7126 && l.motion == Motion::Burst),
        "eigener Look: {l:?}"
    );
    assert!(skill_look("unbekannt").is_none(), "unbekannter Skill: kein Look");
}

/// Zweiter Teil von `TestSkillLooks`: ein Flächenzauber mit dem Radius des Skills.
#[test]
fn skill_looks_cast_for() {
    let d = ev(json!({"type": "skillCast", "from": 1, "skill": "meteor", "x": 20.0, "radius": 3.0}));
    let (e, ok) = cast_for(&d);
    assert!(
        ok && e.look.radius == 3.0 * UNIT_PX && e.x == 20.0,
        "Flächenzauber: {e:?}"
    );
}

#[test]
fn partikel() {
    let (dx, dy) = particle_offset(Motion::Rise, 0, 3, 10.0);
    assert!(dx == -8.0 && dy == -20.0, "rise: {dx} {dy}");
    let (dx, dy) = particle_offset(Motion::Wave, 1, 4, 10.0);
    assert!(dx == 12.0 && dy == 30.0, "wave: {dx} {dy}");
    let kill = config(Kind::Kill).look;
    let (ring, end) = (piece_at(kill, 0, 0.0), piece_at(kill, 0, 1.0));
    assert!(
        ring.alpha == 0.8 && end.alpha == 0.0 && end.radius == 34.0 * 1.8,
        "Ring wächst und blendet aus: {ring:?} {end:?}"
    );
    let p = piece_at(kill, 1, 2.0);
    assert!(p.radius == 2.5, "Fortschritt begrenzt: {p:?}");
}

/// Zwei Läufe mit denselben Ereignissen liefern dieselben Effekte (kein Zufall, Zeit nur aus `Frame::clock`).
/// Abweichung von Go: Die Simulation (`CreateIsland`, `Step`) gibt es in Rust noch nicht; der Monarch läuft von Hand.
#[test]
fn listen_deterministisch() {
    let run = || {
        let mut d = new_fx();
        let mut w = World::default();
        w.players.push(Player::default());
        let mut f = Frame {
            seats: vec![Seat::default()],
            ..Frame::default()
        };
        for tick in 0..90 {
            w.players[0].x += 1.0 / 30.0;
            f.stages = vec![stage_of(0, &w)];
            f.clock = f64::from(tick) / 30.0;
            f.events = vec![
                event_of(0, json!({"type": "strike", "from": 0, "x": 2.0, "hit": 1})),
                event_of(0, json!({"type": "hit", "x": 3.0, "damage": f64::from(tick)})),
            ];
            d.listen(&f);
        }
        d.stage(0).expect("Stufe 0").effects.clone()
    };
    let (a, b) = (run(), run());
    assert!(
        !a.is_empty() && a.len() == b.len(),
        "Effekte: {} und {}",
        a.len(),
        b.len()
    );
    for (i, (x, y)) in a.iter().zip(&b).enumerate() {
        assert_eq!(x, y, "Effekt {i} verschieden");
    }
}

/// Geschossfarbe nach Seite (worldRenderer.ts: team === 'player' ? weiß : 0xff4d6d).
#[test]
fn geschoss_farbe_nach_seite() {
    assert_eq!(projectile_color("enemy"), 0xff4d6d, "Gegner");
    let own = projectile_color("player");
    assert!(
        own == OWN_PROJECTILE && own != ENEMY_PROJECTILE,
        "eigene Geschosse unverändert: {own:#x}"
    );
}

/// Optionen › Blitze aus (Settings.FlashesOff, B-809): Treffer-Blitze entfallen, andere Effekte bleiben.
#[test]
fn blitze_aus() {
    let count = |off: bool| {
        let mut d = new_fx();
        let w = World::default();
        let mut f = Frame {
            stages: vec![stage_of(0, &w)],
            seats: vec![Seat::default()],
            ..Frame::default()
        };
        f.settings.flashes_off = off;
        f.events = vec![
            event_of(0, json!({"type": "hit", "x": 3.0, "target": "enemy", "damage": 2.0})),
            event_of(0, json!({"type": "built", "x": 5.0, "kind": "wall"})),
        ];
        d.listen(&f);
        let list = &d.stage(0).expect("Stufe 0").effects;
        let flashes = list.iter().filter(|e| config(e.kind).flash).count();
        (flashes, list.len() - flashes)
    };
    assert!(count(false).0 > 0, "ohne Schalter: Treffer-Blitz erwartet");
    let (off, rest) = count(true);
    assert!(off == 0 && rest > 0, "Blitze aus: {off} Blitze, {rest} andere Effekte");
}

/// Jede Skill-Linie aus monarch.json hat eine Ring-Farbe (auch die Linie purse, B-922).
#[test]
fn line_colors_cover_every_skill_line() {
    for (id, s) in skills() {
        assert!(
            line_colors().contains_key(s.line.as_str()),
            "Skill {id}: Linie {:?} ohne Farbe",
            s.line
        );
    }
}
