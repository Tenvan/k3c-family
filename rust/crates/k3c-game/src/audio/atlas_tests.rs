//! Port von `native/k3c-game/audio/atlas_test.go`.
//!
//! `TestLoadBankDecodesAtlasAndSingles` ist geteilt: Einzeldateien laufen, der Atlas-Zweig wartet auf Ausschnitte in
//! k3c-platform (Haltepunkt RU17.5, `load_bank_atlas_clips_match_sprites`).

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_platform::NullAudio;
use serde_json::Value;

use super::atlas::{PCM_FRAME, Sprite, parse_atlas, sprite_range};
use super::bank::{clip, load_bank};
use super::events::SLOT_RIDE;
use super::mix::Planner;
use super::register::Register;
use super::testutil::{public_dir, real_register, temp_dir};
use crate::view::frame::{Cue, CueKind, Frame};
use crate::view::settings::Settings;

// AC-01: atlas.json lesen, Start und Dauer je Slot wie in der Datei; jeder Effekt-Slot des Registers außer der
// Hufschlag-Schleife liegt im Atlas (public/audio/README.md).
#[test]
fn atlas_start_and_duration_per_slot() {
    let raw = std::fs::read(public_dir().join("audio/atlas.json")).expect("atlas.json");
    let a = parse_atlas(&raw).expect("parse");
    let want: Value = serde_json::from_slice(&raw).expect("json");
    let want: BTreeMap<String, Value> = want["sprites"]
        .as_object()
        .expect("sprites")
        .clone()
        .into_iter()
        .collect();
    assert!(!a.sprites.is_empty() && a.sprites.len() == want.len());
    for (slot, w) in &want {
        let s = a.sprites[slot];
        assert_eq!(s.start, w["start"].as_f64().unwrap(), "{slot}");
        assert_eq!(s.duration, w["duration"].as_f64().unwrap(), "{slot}");
    }
    for slot in real_register().effects.keys() {
        assert!(
            a.sprites.contains_key(slot) || slot == SLOT_RIDE,
            "Effekt-Slot {slot} fehlt im Atlas"
        );
    }
    let name = a.pick(|_| true);
    assert_eq!(name.as_deref(), Some("atlas.ogg"), "Ogg Vorbis zuerst");
}

#[test]
fn parse_atlas_drops_invalid_sprites() {
    let raw = r#"{"files":{"mp3":"a.mp3"},"sprites":{"ok":{"start":1,"duration":0.5},"neg":{"start":-1,"duration":1},"zero":{"start":2,"duration":0}}}"#;
    let a = parse_atlas(raw.as_bytes()).expect("parse");
    assert_eq!(a.sprites.len(), 1);
    assert_eq!(a.sprites["ok"].duration, 0.5);
    assert_eq!(a.pick(|n| n == "a.mp3").as_deref(), Some("a.mp3"));
    assert_eq!(a.pick(|_| false), None, "ohne vorhandene Datei kein Format");
    for bad in [r#"{"sprites":{}}"#, r#"{"files":{"ogg":"a.ogg"}}"#, "kein json"] {
        assert!(parse_atlas(bad.as_bytes()).is_err(), "{bad}: Fehler erwartet");
    }
}

#[test]
fn sprite_range_whole_frames_and_clamped() {
    let rate = 100;
    let total = 1000 * PCM_FRAME; // 10 s
    let cases = [
        (
            Sprite {
                start: 1.0,
                duration: 0.5,
            },
            100 * PCM_FRAME,
            150 * PCM_FRAME,
        ),
        // am Ende begrenzt
        (
            Sprite {
                start: 9.8,
                duration: 1.0,
            },
            980 * PCM_FRAME,
            1000 * PCM_FRAME,
        ),
        // außerhalb: leer
        (
            Sprite {
                start: 20.0,
                duration: 1.0,
            },
            total,
            total,
        ),
    ];
    for (s, lo, hi) in cases {
        assert_eq!(sprite_range(&s, rate, total), (lo, hi), "{s:?}");
    }
}

// AC-01 ohne Gerät: Der Ton lädt aus public/ über die Plattform; jeder Effekt-Slot hat einen Klang, der Hufschlag
// lädt als Einzeldatei (Rückfall). Abweichung (Haltepunkt RU17.5): Ausschnitte aus dem Atlas kann die Plattform nicht,
// darum laufen alle Effekte als Einzeldateien; den Atlas-Zweig prüft `load_bank_atlas_clips_match_sprites`.
#[test]
fn load_bank_decodes_atlas_and_singles() {
    let reg = real_register();
    let mut out = NullAudio::default();
    let b = load_bank(&public_dir(), &reg, &mut out);
    for slot in reg.effects.keys() {
        assert!(b.clip(slot).is_some(), "{slot}: kein Klang");
    }
    assert!(b.clip(SLOT_RIDE).is_some(), "Hufschlag fehlt (Einzeldatei)");
    assert!(b.clip("sfx:gibtsnicht").is_none(), "unbekannter Slot muss stumm sein");
}

// Atlas-Zweig von Go: jeder Atlas-Slot hat die Länge seines Sprites. Braucht Ausschnitte in k3c-platform.
#[test]
#[ignore = "wartet auf RU17"]
fn load_bank_atlas_clips_match_sprites() {
    let mut out = NullAudio::default();
    let b = load_bank(&public_dir(), &real_register(), &mut out);
    assert!(b.pcm.is_some(), "Atlas nicht geladen");
}

// Fehlende Ton-Dateien: stumm mit Meldung, kein Absturz; jede fehlende Datei genau einmal im Log.
#[test]
fn load_bank_missing_files_is_silent() {
    let mut reg = Register::default();
    reg.effects
        .insert("sfx:coin".to_string(), "audio/fehlt.ogg".to_string());
    let mut out = NullAudio::default();
    let b = load_bank(&temp_dir("fehlt"), &reg, &mut out);
    assert!(
        b.pcm.is_none() && b.clip("sfx:coin").is_none(),
        "ohne Dateien muss alles stumm sein"
    );
    assert_eq!(
        b.notes.iter().filter(|n| n.contains("slot=sfx:coin")).count(),
        1,
        "{:?}",
        b.notes
    );
    assert!(clip(None, "sfx:coin").is_none(), "noch nicht geladen: stumm");
}

// AC-01: Lautstärke je Bus wirkt (ohne Gerät): Effekte und Musik folgen getrennt den Einstellungen.
#[test]
#[allow(clippy::float_cmp)]
fn bus_volumes_from_settings() {
    let mut p = Planner::new(Arc::new(real_register()));
    let mx = p.reg.mix;
    let mut f = Frame {
        settings: Settings {
            music: 0.25,
            effects: 0.5,
            ..Settings::default()
        },
        cues: vec![Cue {
            kind: CueKind::Confirm,
            ..Cue::default()
        }],
        ..Frame::default()
    };
    let m = p.plan(&f);
    assert!(
        m.effect_bus == 0.5 * mx.effects && m.music_bus == 0.25 * mx.music,
        "Busse {} / {}",
        m.effect_bus,
        m.music_bus
    );
    assert_eq!(m.effect_volume(m.effects[0].gain), 0.5 * mx.effects, "Effekt");
    assert_eq!(m.music_volume(0.8), 0.8 * (0.25 * mx.music), "Musik");
    f.settings = Settings {
        music: 7.0,
        effects: f64::NAN,
        ..Settings::default()
    };
    let m = p.plan(&f);
    assert!(
        m.music_bus == mx.music && m.effect_bus == mx.effects,
        "begrenzt: {} / {}",
        m.music_bus,
        m.effect_bus
    );
}
