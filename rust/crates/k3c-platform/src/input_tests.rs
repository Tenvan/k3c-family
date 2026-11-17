use super::*;

#[test]
fn kurzer_tipp_bleibt_bis_zum_takt_erhalten() {
    let mut i = InputState::new();
    i.keys.press(Key::Space);
    i.keys.release(Key::Space);
    assert!(i.just_pressed(Key::Space), "Tipp kürzer als ein Takt");
    assert!(!i.held(Key::Space));
    assert!(i.keys.just_released(Key::Space));
    i.end_step();
    assert!(!i.just_pressed(Key::Space));
    assert!(!i.keys.just_released(Key::Space));
}

#[test]
fn gehaltene_taste_hat_nur_eine_flanke() {
    let mut i = InputState::new();
    i.keys.press(Key::D);
    i.end_step();
    i.keys.press(Key::D);
    assert!(i.held(Key::D));
    assert!(!i.just_pressed(Key::D), "Wiederholung ist keine neue Flanke");
    i.keys.release_all();
    assert!(!i.held(Key::D));
    assert!(i.keys.just_released(Key::D));
}

#[test]
fn alle_tasten_passen_ins_bitfeld() {
    assert!(Key::ALL.len() <= 64);
    let mut i = InputState::new();
    for &k in Key::ALL {
        i.keys.press(k);
    }
    assert!(Key::ALL.iter().all(|&k| i.held(k)));
}

#[test]
fn pads_kommen_und_gehen() {
    let mut i = InputState::new();
    i.pad_mut(3).set_button(PadButton::A, true);
    i.pad_mut(3).set_axis(PadAxis::LX, -0.5);
    i.pad_mut(7);
    assert_eq!(i.pads().iter().map(|p| p.id).collect::<Vec<_>>(), [3, 7]);
    let p = i.pad(3).unwrap();
    assert!(p.held(PadButton::A) && p.just_pressed(PadButton::A));
    assert_eq!(p.axis(PadAxis::LX), -0.5);
    i.end_step();
    assert!(!i.pad(3).unwrap().just_pressed(PadButton::A));
    i.disconnect(3);
    assert!(i.pad(3).is_none());
    assert_eq!(i.pads().len(), 1);
}

const SCRIPT: &str = r#"[
    { "step": 10, "keys": ["Space"] },
    { "step": 0, "keys": ["D"], "pads": [{ "id": 0, "buttons": ["A"], "axes": { "LX": 1.0 } }] },
    { "step": 11 },
    { "step": 20, "pads": [{ "id": 1, "buttons": ["B", "Menu"] }] }
]"#;

#[test]
fn skript_lesen_und_abspielen() {
    let s = InputScript::from_json(SCRIPT).unwrap();
    let mut i = InputState::new();
    let mut seen = Vec::new();
    for step in 0..25 {
        s.apply(step, &mut i);
        seen.push((
            step,
            i.held(Key::D),
            i.just_pressed(Key::Space),
            i.pad(0).map(|p| p.axis(PadAxis::LX)),
            i.pad(1).is_some_and(|p| p.held(PadButton::Menu)),
        ));
        i.end_step();
    }
    assert_eq!(seen[0], (0, true, false, Some(1.0), false));
    assert_eq!(
        seen[9],
        (9, true, false, Some(1.0), false),
        "Eintrag gilt bis zum nächsten"
    );
    assert_eq!(seen[10], (10, false, true, None, false), "Pad 0 fehlt ab Takt 10");
    assert_eq!(seen[11], (11, false, false, None, false));
    assert_eq!(seen[24], (24, false, false, None, true));
}

#[test]
fn falsches_skript_nennt_den_fehler() {
    let e = InputScript::from_json(r#"[{ "step": 0, "keys": ["Gibtsnicht"] }]"#).unwrap_err();
    assert!(e.contains("Eingabe-Skript") && e.contains("Gibtsnicht"), "{e}");
}
