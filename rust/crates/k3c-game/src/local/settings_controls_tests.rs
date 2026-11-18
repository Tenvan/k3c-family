use super::settings::SETTINGS_FILE;
use super::testutil::{TempDir, test_session};
use crate::view::profile::{PROFILE_ARROWS, PROFILE_GAMEPAD};
use crate::view::session::Session as _;
use crate::view::settings::default_settings;

// B-893/AC-03: Das Eingabe-Profil je Spieler überlebt Speichern und einen Neustart; zwei Tastatur-Profile aus einer
// Hand-Datei normalisiert das Laden (der spätere Spieler bekommt Gamepad), eine Datei ohne Feld gibt den Standard.
#[test]
fn settings_controls_persist() {
    let dir = TempDir::new();
    let mut want = default_settings();
    (want.controls[0], want.controls[1]) = (PROFILE_GAMEPAD.to_string(), PROFILE_ARROWS.to_string());
    test_session(&dir).set_settings(want.clone()).expect("speichern");
    let got = test_session(&dir).settings();
    assert_eq!(
        got, want,
        "nach Neustart {:?}, erwartet {:?}",
        got.controls, want.controls
    );
    let path = dir.path().join(SETTINGS_FILE);
    std::fs::write(&path, r#"{"controls":["pfeile","wasd","gamepad","gamepad","gamepad"]}"#).expect("schreiben");
    let got = test_session(&dir).settings().controls;
    assert!(
        got[0] == PROFILE_ARROWS && got[1] == PROFILE_GAMEPAD,
        "zwei Tastatur-Profile geladen: {got:?}"
    );
    std::fs::write(&path, r#"{"music":0.5}"#).expect("schreiben");
    assert_eq!(
        test_session(&dir).settings().controls,
        default_settings().controls,
        "ohne Feld"
    );
}
