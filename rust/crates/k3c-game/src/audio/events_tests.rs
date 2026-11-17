//! Port von `native/k3c-game/audio/events_test.go` (plus Register-Tests aus `atlas_test.go`/`klang_test.go`-Umfeld).

use std::collections::BTreeMap;

use serde_json::json;

use k3c_sim::state::{Player, World};

use super::duck::{DEFAULT_DUCK, DuckWindow, duck_gain, extend_duck};
use super::events::*;
use super::register::{DEFAULT_MIXING, Register, SKILL_FALLBACK, SkillInfo, parse_register};
use super::testutil::real_register;
use crate::view::frame::{CueKind, Event};

/// Die Effekt-Slots, die das Spiel aus den nativen Tabellen abspielt: Ereignisse und Menü-Klänge.
fn effect_slots() -> BTreeMap<String, String> {
    let mut slots = BTreeMap::new();
    slots.insert("cue:move".to_string(), cue_slot(CueKind::Move).to_string());
    slots.insert("cue:confirm".to_string(), cue_slot(CueKind::Confirm).to_string());
    for (name, c) in EVENT_CUES {
        slots.insert(name.to_string(), c.slot.to_string());
    }
    slots
}

/// Nennt sortiert jeden Eintrag aus slots (Name → Slot), dessen Slot leer ist oder in files keine Datei hat.
pub(super) fn missing_files(slots: &BTreeMap<String, String>, files: &BTreeMap<String, String>) -> Vec<String> {
    slots
        .iter()
        .filter(|(_, slot)| slot.is_empty() || files.get(*slot).is_none_or(|f| f.is_empty()))
        .map(|(name, slot)| format!("{name} → {slot}"))
        .collect()
}

// AC-01: Jeder Slot aus EVENT_CUES hat eine Datei im Register; global (Warnung) sind genau die vier Warn-Ereignisse.
#[test]
fn event_cues_have_files() {
    assert!(!EVENT_CUES.is_empty());
    let missing = missing_files(&effect_slots(), &real_register().effects);
    assert!(missing.is_empty(), "Slots ohne Datei im Register: {missing:?}");
    let warn = ["bossSpawned", "dusk", "wave", "castleFallen"];
    for (name, c) in EVENT_CUES {
        assert_eq!(c.global, warn.contains(&name), "{name}");
    }
}

// AC-02: Fehlt einem Slot die Datei im Register, nennt die Prüfung genau diesen Eintrag.
#[test]
pub(super) fn missing_files_names_slot() {
    let mut files = real_register().effects;
    files.remove(event_cue("hit").unwrap().slot);
    assert_eq!(missing_files(&effect_slots(), &files), ["hit → sfx:hit"]);
    let leer = BTreeMap::from([("leer".to_string(), String::new())]);
    assert_eq!(missing_files(&leer, &files).len(), 1, "leerer Slot nicht gemeldet");
}

// Menü-Klänge: nur Auswahl bewegt und Bestätigen klingen, je mit eigenem Slot; alle anderen Cues bleiben still.
#[test]
fn cue_slots() {
    assert_eq!(cue_slot(CueKind::Move), SLOT_UI_SELECT);
    assert_eq!(cue_slot(CueKind::Confirm), SLOT_UI_CONFIRM);
    assert_ne!(SLOT_UI_SELECT, SLOT_UI_CONFIRM);
    use CueKind::*;
    for k in [
        Back,
        Denied,
        Pause,
        Resume,
        Joined,
        Left,
        Saved,
        SaveFailed,
        Error,
        DeviceConnected,
        DeviceLost,
        Rebound,
    ] {
        assert_eq!(cue_slot(k), "", "Cue {k:?} klingt, erwartet still");
    }
}

// Skill-Klang: aktive Skills über die Linie, passive still, unbekannte Linie → gemeinsamer Slot.
#[test]
fn skill_slot() {
    let reg = real_register();
    let cases = [
        ("fireball", Some("sfx:skill:mage")),
        ("taunt", Some("sfx:skill:tank")),
        ("heal", Some("sfx:skill:healer")),
        ("backstab", Some("sfx:skill:thief")),
        ("armorAura", None),
        ("gibtsnicht", None),
    ];
    for (id, want) in cases {
        assert_eq!(reg.skill_slot(id).as_deref(), want, "{id}");
    }
    let info = |kind: &str, line: &str| SkillInfo {
        kind: kind.into(),
        line: line.into(),
    };
    let r = Register {
        effects: BTreeMap::from([("sfx:skill:x".to_string(), "f".to_string())]),
        skills: BTreeMap::from([
            ("a".to_string(), info("active", "neu")),
            ("x".to_string(), info("passive", "")),
        ]),
        ..Register::default()
    };
    assert_eq!(r.skill_slot("a").as_deref(), Some(SKILL_FALLBACK));
    assert_eq!(r.skill_slot("x").as_deref(), Some("sfx:skill:x"));
}

// Dämpfung: im Bild 1, am Rand 0,5, nach einer Breite 0; Warnung und ohne Ort 1.
#[test]
fn attenuation_cases() {
    let ears = [Ear {
        center: 0.0,
        span: 40.0,
    }];
    let cases = [
        (10.0, true, false, 1.0),
        (20.4, true, false, EDGE_GAIN * (1.0 - 0.4 / 40.0)),
        (60.0, true, false, EDGE_GAIN * (1.0 - 40.0 / 40.0)),
        (100.0, true, false, 0.0),
        (100.0, true, true, 1.0),
        (100.0, false, false, 1.0),
    ];
    for (x, has_x, global, want) in cases {
        let got = attenuation(x, has_x, &ears, global);
        assert!((got - want).abs() <= 1e-9, "x={x}: {got}, erwartet {want}");
    }
    let two = [
        Ear {
            center: 0.0,
            span: 40.0,
        },
        Ear {
            center: 200.0,
            span: 40.0,
        },
    ];
    assert_eq!(attenuation(200.0, true, &two, false), 1.0, "das lauteste Ohr gilt");
    assert_eq!(attenuation(500.0, true, &[], false), 1.0, "ohne Ohr keine Dämpfung");
}

#[test]
fn event_x_and_night_gain() {
    let w = World {
        players: vec![Player {
            index: 1,
            x: 42.0,
            ..Player::default()
        }],
        ..World::default()
    };
    let ev = |v: serde_json::Value| Event {
        data: v.as_object().unwrap().clone(),
        ..Event::default()
    };
    let cases = [
        (json!({"type": "hit", "x": 3.5}), 3.5, true),
        (json!({"type": "crownFallen", "player": 1}), 42.0, true),
        (json!({"type": "crownFallen", "player": 7}), 0.0, false),
        (json!({"type": "dusk"}), 0.0, false),
    ];
    for (data, x, ok) in cases {
        assert_eq!(event_x(&ev(data.clone()), Some(&w)), (x, ok), "{data}");
    }
    assert_eq!(night_gain("night"), NIGHT_GAIN_VAL);
    assert_eq!(night_gain("dawn"), 1.0, "Nacht leise, sonst voll");
}

#[test]
fn riding_needs_living_moving_monarch() {
    let p = |vx: f64| Player {
        vx,
        ..Player::default()
    };
    assert!(riding(Some(&p(-RIDE_MIN_SPEED))));
    assert!(!riding(Some(&p(0.1))));
    assert!(!riding(None));
}

#[test]
fn register_defaults_and_errors() {
    let monarch = br#"{"skills":{}}"#;
    let r = parse_register(br#"{"slots":[]}"#, monarch).expect("ohne mischung");
    assert_eq!(r.mixing(), DEFAULT_MIXING);
    assert_eq!(Register::default().mixing(), DEFAULT_MIXING);
    let bad =
        br#"{"mischung":{"effekte":2,"musik":1,"duckPegel":0.3,"duckAnstieg":1,"duckHalten":1,"duckAbklingen":1}}"#;
    let e = parse_register(bad, monarch).unwrap_err();
    assert_eq!(e, "❌ data/sounds.json › mischung: effekte 2 / musik 1 nicht in (0, 1]");
    assert!(
        parse_register(b"x", monarch)
            .unwrap_err()
            .starts_with("❌ data/sounds.json: ")
    );
    let real = real_register();
    assert_ne!(real.mix, super::register::Mixing::ZERO);
    assert!(!real.music.is_empty() && !real.effects.is_empty());
}

#[test]
fn duck_gain_ramps_and_extends() {
    let w = extend_duck(&DuckWindow::default(), 10.0, DEFAULT_DUCK);
    assert_eq!(duck_gain(9.0, &w), 1.0);
    assert!((duck_gain(10.0 + DEFAULT_DUCK.attack, &w) - DEFAULT_DUCK.level).abs() < 1e-9);
    let w2 = extend_duck(&w, 10.5, DEFAULT_DUCK);
    assert_eq!(w2.start, w.start, "laufende Absenkung wird verlängert");
    assert!(w2.end > w.end);
    assert_eq!(duck_gain(w2.end + DEFAULT_DUCK.release + 0.1, &w2), 1.0);
}
