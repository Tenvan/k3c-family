//! Tests von B-1222 (WZ66.3, AC-03): Format von `survey.json`, Überspringen ergibt `null`, atomares Schreiben, Bedienung
//! mit zwei Pads, Zeitablauf, ohne Spieler oder Flag kein Bogen.

use k3c_platform::{InputState, Key, PadButton};

use super::Playtest;
use super::survey::{IDLE_TICKS, Press, Sheet, Survey};
use super::tests::{dir, frame};
use crate::view::input::{DeviceID, DeviceKind};

fn pad(i: i64) -> DeviceID {
    DeviceID::new(DeviceKind::Gamepad, i)
}

fn two() -> Survey {
    Survey::new(&[(1, DeviceID::new(DeviceKind::Keyboard, 0)), (2, pad(7))], 123)
}

fn read(d: &std::path::Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(d.join("survey.json")).expect("Datei")).expect("JSON")
}

#[test]
fn survey_json_hat_das_format_und_ueberspringen_ergibt_null() {
    let d = dir("survey-format");
    std::fs::create_dir_all(&d).unwrap();
    let mut s = two();
    // Spieler 1: 4 Sterne, passt, ja
    for p in [Press::Right, Press::Confirm, Press::Confirm, Press::Confirm] {
        s.sheets[0].press(p);
    }
    // Spieler 2 drückt Start
    s.sheets[1].press(Press::SkipAll);
    assert!(s.sheets.iter().all(Sheet::finished));
    s.write(&d).unwrap();
    let v = read(&d);
    assert_eq!(v["ts"], 123);
    assert_eq!(v["players"][0]["player"], 1);
    assert_eq!(v["players"][0]["fun"], 4);
    assert_eq!(v["players"][0]["difficulty"], "passt");
    assert_eq!(v["players"][0]["again"], true);
    assert_eq!(v["players"][1]["player"], 2);
    for k in ["fun", "difficulty", "again"] {
        assert!(v["players"][1][k].is_null(), "{k}");
    }
    assert!(!d.join("survey.json.tmp").exists(), "keine halbe Datei, kein Rest");
}

#[test]
fn b_ueberspringt_nur_die_frage_und_auswahl_laeuft_im_kreis() {
    let mut s = Sheet::new(1, pad(0));
    s.press(Press::Left);
    s.press(Press::Left);
    s.press(Press::Left); // 3 Sterne -> 2 -> 1 -> 5
    s.press(Press::Confirm);
    assert_eq!(s.answers[0], Some(4));
    s.press(Press::SkipQuestion); // Schwierigkeit
    s.press(Press::Right); // ja -> nein
    s.press(Press::Confirm);
    assert_eq!(s.answers, [Some(4), None, Some(1)]);
    assert!(s.finished());
}

#[test]
fn zwei_pads_gleichzeitig() {
    let mut s = Survey::new(&[(1, pad(1)), (2, pad(2))], 1);
    let mut input = InputState::new();
    input.pad_mut(1);
    input.pad_mut(2);
    assert!(!s.step(&input)); // erster Takt: Zustand lesen
    for _ in 0..3 {
        input.pad_mut(1).set_button(PadButton::A, true);
        input.pad_mut(2).set_button(PadButton::B, true);
        s.step(&input);
        input.pad_mut(1).set_button(PadButton::A, false);
        input.pad_mut(2).set_button(PadButton::B, false);
        let done = s.step(&input);
        assert_eq!(done, s.sheets.iter().all(Sheet::finished));
    }
    assert!(s.sheets.iter().all(Sheet::finished));
    assert_eq!(s.sheets[0].answers, [Some(2), Some(1), Some(0)]);
    assert_eq!(s.sheets[1].answers, [None; 3]);
}

#[test]
fn schon_gehaltene_taste_wirkt_erst_nach_dem_loslassen() {
    let mut s = Sheet::new(1, pad(1));
    let mut input = InputState::new();
    input.pad_mut(1).set_button(PadButton::A, true); // A vom Spiel noch gehalten
    for _ in 0..5 {
        s.step(&input);
    }
    assert_eq!(s.q, 0);
    input.pad_mut(1).set_button(PadButton::A, false);
    s.step(&input);
    input.pad_mut(1).set_button(PadButton::A, true);
    s.step(&input);
    assert_eq!(s.q, 1);
}

#[test]
fn start_beendet_den_bogen_und_tastatur_geht_auch() {
    let mut s = Sheet::new(1, DeviceID::new(DeviceKind::Keyboard, 0));
    let mut input = InputState::new();
    s.step(&input);
    input.keys.press(Key::ArrowRight);
    s.step(&input);
    assert_eq!(s.sel[0], 3);
    input.keys.release(Key::ArrowRight);
    input.keys.press(Key::Escape);
    s.step(&input);
    assert!(s.finished() && s.answers == [None; 3]);
}

#[test]
fn nach_60_sekunden_ohne_eingabe_null() {
    let mut s = Sheet::new(1, pad(1));
    s.press(Press::Confirm);
    let input = InputState::new();
    for _ in 0..IDLE_TICKS - 1 {
        s.step(&input);
    }
    assert!(!s.finished());
    s.step(&input);
    assert!(s.finished());
    assert_eq!(s.answers, [Some(2), None, None]);
}

#[test]
fn ohne_spieler_oder_ohne_beenden_kein_bogen_und_keine_datei() {
    let d = dir("survey-leer");
    let mut pt = Playtest::open(&d);
    assert!(pt.survey().is_none(), "vor dem Beenden nichts");
    pt.survey_begin(1); // nie gespielt: nichts zu fragen
    assert!(pt.survey().is_none());
    pt.survey_step(&InputState::new());
    assert!(!d.join("survey.json").exists());
}

#[test]
fn nach_dem_spiel_fragt_der_bogen_beide_spieler_und_schreibt_die_datei() {
    let d = dir("survey-spiel");
    let mut pt = Playtest::open(&d);
    pt.step(&InputState::new(), Some(&frame()), 1_000); // Spieler: Tastatur und Pad 7
    pt.step(&InputState::new(), None, 2_000); // Menü: die Spieler bleiben bekannt
    pt.survey_begin(3_000);
    assert_eq!(pt.survey().map(|s| s.sheets.len()), Some(2));
    let mut input = InputState::new();
    input.pad_mut(7);
    pt.survey_step(&input);
    for _ in 0..3 {
        input.pad_mut(7).set_button(PadButton::A, true);
        input.keys.press(Key::Escape);
        pt.survey_step(&input);
        input.pad_mut(7).set_button(PadButton::A, false);
        input.keys.release(Key::Escape);
        pt.survey_step(&input);
    }
    assert!(pt.survey().is_none(), "alle fertig: Bogen zu");
    let v = read(&d);
    assert_eq!(v["players"][1]["again"], true);
    assert!(v["players"][0]["fun"].is_null());
    pt.survey_begin(4_000);
    assert!(pt.survey().is_none(), "nur einmal");
}
