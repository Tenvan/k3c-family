//! Port von `native/k3c-game/audio/atlas_test.go`.
//!
//! Nicht portiert, weil sie an `bank`, `output` und `audio` hängen (RU17.5 schaltet sie ein, Namen siehe `#[ignore]`):
//! `TestLoadBankDecodesAtlasAndSingles`, `TestLoadBankMissingFilesIsSilent`, `TestBusVolumesFromSettings`.

use std::collections::BTreeMap;

use serde_json::Value;

use super::atlas::{PCM_FRAME, Sprite, parse_atlas, sprite_range};
use super::events::SLOT_RIDE;
use super::testutil::{public_dir, real_register};

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

#[test]
#[ignore = "wartet auf RU17"]
fn load_bank_decodes_atlas_and_singles() {}

#[test]
#[ignore = "wartet auf RU17"]
fn load_bank_missing_files_is_silent() {}

#[test]
#[ignore = "wartet auf RU17"]
fn bus_volumes_from_settings() {}
