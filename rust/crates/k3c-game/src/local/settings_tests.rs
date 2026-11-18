use super::settings::{SETTINGS_FILE, save_settings, settings_json};
use super::testutil::{TempDir, test_session};
use crate::view::session::Session as _;
use crate::view::settings::{RIVER_TILES, default_settings};

// Fluss-Fassung und Leistungsanzeige (B-816) überleben Speichern und Laden; ohne gewählte Fassung steht kein Feld in
// der Datei (der Startwert K3C_RIVER gilt weiter), und eine alte Datei ohne die Felder bleibt gültig.
#[test]
fn settings_river_stats_persist() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    let mut want = default_settings();
    (want.river, want.stats) = (RIVER_TILES.to_string(), true);
    s.set_settings(want.clone()).expect("speichern");
    assert_eq!(test_session(&dir).settings(), want);
    want.river = String::new();
    s.set_settings(want.clone()).expect("speichern");
    let raw = std::fs::read_to_string(dir.path().join(SETTINGS_FILE)).expect("Datei");
    assert!(!raw.contains("river"), "Startwert soll kein Feld schreiben: {raw}");
    assert_eq!(test_session(&dir).settings(), want, "ohne Fassung geladen");
}

#[test]
fn settings_old_file_and_bad_river() {
    let dir = TempDir::new();
    let path = dir.path().join(SETTINGS_FILE);
    std::fs::write(&path, r#"{"music":0.5,"river":"lava","stats":true}"#).expect("schreiben");
    let got = test_session(&dir).settings();
    assert!(
        got.music == 0.5 && got.river.is_empty() && got.stats,
        "unbekannte Fassung -> Startwert: {got:?}"
    );
    std::fs::write(&path, r#"{"music":0.5}"#).expect("schreiben");
    let got = test_session(&dir).settings();
    assert!(got.river.is_empty() && !got.stats, "alte Datei: {got:?}");
}

// Blitze, Anleitung und „Anleitung gesehen“ (B-809, AC-07, AC-09) überleben Speichern und einen Neustart der Session;
// eine alte Datei ohne die Felder lässt Blitze und Anleitung an und nichts gesehen.
#[test]
fn settings_guide_flashes_persist() {
    let dir = TempDir::new();
    let mut want = default_settings();
    (want.flashes_off, want.guide_off, want.guide_seen) = (true, true, 0b110);
    test_session(&dir).set_settings(want.clone()).expect("speichern");
    assert_eq!(test_session(&dir).settings(), want, "nach Neustart");
    std::fs::write(dir.path().join(SETTINGS_FILE), r#"{"music":0.5}"#).expect("schreiben");
    let got = test_session(&dir).settings();
    assert!(
        !got.flashes_off && !got.guide_off && got.guide_seen == 0,
        "alte Datei: {got:?}"
    );
}

// AC-02: Die Datei liegt im Format von Go (Feldnamen, Reihenfolge, ganze Zahlen ohne „.0“); die Texte stammen aus einem
// Lauf von `saveSettings` in Go.
#[test]
fn settings_file_format_like_go() {
    let dir = TempDir::new();
    let path = dir.path().join(SETTINGS_FILE);
    save_settings(&path, &default_settings()).expect("speichern");
    assert_eq!(std::fs::read_to_string(&path).expect("Datei"), GO_DEFAULT);
    let mut s = default_settings();
    (s.river, s.dev_mode, s.music, s.flashes_off, s.guide_seen) = (RIVER_TILES.to_string(), true, 0.5, true, 6);
    (s.controls[0], s.controls[1]) = ("gamepad".to_string(), "pfeile".to_string());
    s.stress.factor = 1.5;
    s.window = "vollbild".to_string();
    save_settings(&path, &s).expect("speichern");
    assert_eq!(std::fs::read_to_string(&path).expect("Datei"), GO_CUSTOM);
}

// Eine Datei aus Go lädt ohne Verlust: alle Felder kommen zurück.
#[test]
fn settings_go_file_loads() {
    let dir = TempDir::new();
    std::fs::write(dir.path().join(SETTINGS_FILE), GO_CUSTOM).expect("schreiben");
    let got = test_session(&dir).settings();
    assert_eq!(got.music, 0.5);
    assert_eq!(
        (got.window.as_str(), got.river.as_str(), got.guide_seen),
        ("vollbild", "tiles", 6)
    );
    assert!(got.flashes_off && !got.guide_off && got.dev_mode);
    assert_eq!(
        (got.controls[0].as_str(), got.controls[1].as_str()),
        ("gamepad", "pfeile")
    );
    assert_eq!(got.stress.factor, 1.5);
    assert_eq!(settings_json(&got).stress.map(|s| s.waves), Some(5));
}

const GO_DEFAULT: &str = r#"{
  "music": 1,
  "effects": 1,
  "window": "",
  "language": "de",
  "stats": false,
  "flashes": true,
  "guide": true,
  "guideSeen": 0,
  "controls": [
    "wasd",
    "gamepad"
  ],
  "stress": {
    "id": "",
    "players": 1,
    "depth": 0,
    "seed": 0,
    "enemies": 10,
    "growth": "linear",
    "factor": 5,
    "build": 0,
    "troops": 0,
    "timescale": 1,
    "waves": 5,
    "minutes": 0,
    "enemy": "goblin",
    "troop": "archer",
    "pure": false
  }
}"#;

const GO_CUSTOM: &str = r#"{
  "music": 0.5,
  "effects": 1,
  "window": "vollbild",
  "language": "de",
  "river": "tiles",
  "stats": false,
  "flashes": false,
  "guide": true,
  "guideSeen": 6,
  "controls": [
    "gamepad",
    "pfeile"
  ],
  "devMode": true,
  "stress": {
    "id": "",
    "players": 1,
    "depth": 0,
    "seed": 0,
    "enemies": 10,
    "growth": "linear",
    "factor": 1.5,
    "build": 0,
    "troops": 0,
    "timescale": 1,
    "waves": 5,
    "minutes": 0,
    "enemy": "goblin",
    "troop": "archer",
    "pure": false
  }
}"#;
