//! Port von `native/k3c-game/fx/effects_test.go`.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use k3c_sim::state::Event;

use super::effects::*;
use super::looks::*;

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

/// Braucht `StageFx::spawn` und `expire` aus fx.go (RU17.2).
#[test]
#[ignore = "wartet auf RU17"]
fn spawn_grenzen() {}

#[test]
fn skill_looks_eigener_und_unbekannter() {
    let l = skill_look("fireball");
    assert!(
        l.is_some_and(|l| l.color == 0xdf7126 && l.motion == Motion::Burst),
        "eigener Look: {l:?}"
    );
    assert!(skill_look("unbekannt").is_none(), "unbekannter Skill: kein Look");
}

/// Zweiter Teil von `TestSkillLooks`: `cast_for` steht in fx.go (RU17.2).
#[test]
#[ignore = "wartet auf RU17"]
fn skill_looks_cast_for() {}

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

/// Zwei Läufe mit denselben Ereignissen liefern dieselben Effekte; braucht den Drawer aus RU17.2.
#[test]
#[ignore = "wartet auf RU17"]
fn listen_deterministisch() {}

/// Geschossfarbe nach Seite; `projectile_color` steht in paint.go (RU17.2).
#[test]
#[ignore = "wartet auf RU17"]
fn geschoss_farbe_nach_seite() {}

/// Optionen › Blitze aus (Settings.FlashesOff, B-809); braucht den Drawer aus RU17.2.
#[test]
#[ignore = "wartet auf RU17"]
fn blitze_aus() {}

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
