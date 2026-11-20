//! Port von `native/k3c-game/audio/klang_test.go`: `data/sounds.json` gegen das Klangbild `docs/rules/klang.md`
//! (B-935). `soundsFile` aus register.go ist in Rust privat; der Test liest die drei Felder selbst.

use serde::Deserialize;

use super::duck::DEFAULT_DUCK;
use super::register::{DEFAULT_MIXING, Register, parse_register};
use super::testutil::{data_dir, real_register};

/// Namensteile der Kandidaten, die das Klangbild ausschließt (docs/rules/klang.md, B-935): Synth-Piepen und
/// Interface-Klänge (Kenney Digital Audio, Interface Sounds) sowie 8-Bit und Chiptune.
const EXCLUDED_SOUNDS: [&str; 5] = [
    "kenney-digital-audio-",
    "kenney-interface-sounds-",
    "8-bit",
    "chiptune",
    "nes-",
];

/// Musik-Slots, deren Stück noch gegen das Klangbild verstößt und erst mit B-936 ersetzt wird (kein ruhiger Kandidat
/// vorhanden). Nur Musik darf hier stehen; ein Effekt-Slot hat keine Ausnahme.
const PENDING_MUSIC: [&str; 11] = [
    "music:tag",
    "music:abend",
    "music:nacht",
    "music:kampf",
    "music:tiefe",
    "music:boss",
    "music:lobby",
    "music:ende",
    "music:fest",
    "music:fest:sengoku",
    "music:fest:nil",
];

#[derive(Default, Deserialize)]
#[serde(default)]
struct Choice {
    datei: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Slot {
    id: String,
    bus: String,
    wahl: Vec<Choice>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct SoundsFile {
    slots: Vec<Slot>,
}

fn excluded(file: &str) -> bool {
    let name = file.rsplit('/').next().unwrap_or(file);
    EXCLUDED_SOUNDS.iter().any(|part| name.contains(part))
}

// B-935/AC-02: Kein Slot spielt als erste Wahl einen ausgeschlossenen Kandidaten (die erste Wahl klingt, register.go
// und tools/soundatlas); die Ausnahmen für Musik bis B-936 sind ausdrücklich gelistet.
#[test]
fn first_choice_follows_sound_image() {
    let raw = std::fs::read(data_dir().join("sounds.json")).expect("sounds.json");
    let sf: SoundsFile = serde_json::from_slice(&raw).expect("sounds.json lesbar");
    for s in &sf.slots {
        let Some(first) = s.wahl.first() else {
            continue;
        };
        if PENDING_MUSIC.contains(&s.id.as_str()) {
            assert_eq!(s.bus, "music", "{}: Ausnahme nur für Musik", s.id);
            continue;
        }
        assert!(
            !excluded(&first.datei),
            "{}: erste Wahl {} verstößt gegen das Klangbild",
            s.id,
            first.datei
        );
    }
    assert!(
        excluded("audio/kandidaten/kenney-digital-audio-x.ogg") && !excluded("audio/kandidaten/heal-rpg.mp3"),
        "Namensprüfung"
    );
}

// B-935: Die Mischpegel aus sounds.json dämpfen beide Busse und das Ducking gegenüber dem Stand davor.
#[test]
fn mixing_softened() {
    let mx = real_register().mix;
    assert!(mx.effects < 1.0 && mx.music < 1.0, "Busse nicht gedämpft: {mx:?}");
    let d = mx.duck;
    assert!(
        d.attack > DEFAULT_DUCK.attack && d.release > DEFAULT_DUCK.release && d.level > DEFAULT_DUCK.level,
        "Ducking nicht sanfter: {d:?}"
    );
}

// Ungültige Mischpegel sind ein Fehler; ohne Abschnitt gilt DEFAULT_MIXING.
#[test]
fn mixing_parse() {
    let monarch = br#"{"skills":{}}"#;
    let r = parse_register(br#"{"slots":[]}"#, monarch).expect("ohne mischung");
    assert_eq!(r.mix, DEFAULT_MIXING, "ohne mischung");
    assert_eq!(
        Register::default().mixing(),
        DEFAULT_MIXING,
        "Null-Wert ohne DEFAULT_MIXING"
    );
    for bad in [
        r#"{"effekte":0,"musik":1,"duckPegel":0.5,"duckAnstieg":0.1,"duckHalten":1,"duckAbklingen":1}"#,
        r#"{"effekte":1,"musik":1.2,"duckPegel":0.5,"duckAnstieg":0.1,"duckHalten":1,"duckAbklingen":1}"#,
        r#"{"effekte":1,"musik":1,"duckPegel":1,"duckAnstieg":0.1,"duckHalten":1,"duckAbklingen":1}"#,
        r#"{"effekte":1,"musik":1,"duckPegel":0.5,"duckAnstieg":0,"duckHalten":1,"duckAbklingen":1}"#,
    ] {
        let sounds = format!(r#"{{"mischung":{bad},"slots":[]}}"#);
        assert!(parse_register(sounds.as_bytes(), monarch).is_err(), "angenommen: {bad}");
    }
}
