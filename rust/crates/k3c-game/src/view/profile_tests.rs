use super::*;

/// controls baut Settings.Controls aus den ersten Werten, der Rest ist Gamepad.
pub(crate) fn controls(ps: &[&str]) -> Controls {
    std::array::from_fn(|i| ps.get(i).copied().unwrap_or(PROFILE_GAMEPAD).to_string())
}

fn with_controls(c: Controls) -> Settings {
    Settings {
        controls: c,
        ..Default::default()
    }
}

/// Standard: Spieler 1 WASD, alle anderen Gamepad; unbekannte Werte fallen auf den Standard des Platzes.
#[test]
fn default_controls_test() {
    assert_eq!(
        super::super::settings::default_settings().controls,
        controls(&[PROFILE_WASD]),
        "Standard"
    );
    let mut unknown: Controls = Default::default();
    unknown[0] = "joystick".into();
    assert_eq!(normalize_controls(unknown), controls(&[PROFILE_WASD]), "unbekannt");
}

/// B-893/AC-02: Zwei Spieler haben nie beide ein Tastatur-Profil: Normalized macht das spätere zum Gamepad,
/// NextProfile überspringt Tastatur-Profile, solange ein anderer Spieler eins hat.
#[test]
fn one_keyboard_player() {
    let both = controls(&[PROFILE_WASD, PROFILE_ARROWS]);
    assert_eq!(
        normalize_controls(both),
        controls(&[PROFILE_WASD, PROFILE_GAMEPAD]),
        "zwei Tastatur-Profile"
    );
    let s = with_controls(controls(&[PROFILE_WASD]));
    for dir in [1, -1] {
        assert_eq!(
            s.next_profile(1, dir),
            PROFILE_GAMEPAD,
            "Spieler 2 blättert ({dir}), Tastatur ist vergeben"
        );
    }
    assert_eq!(s.next_profile(0, 1), PROFILE_ARROWS, "Spieler 1 WASD → pfeile");
    assert_eq!(
        s.next_profile(0, -1),
        PROFILE_GAMEPAD,
        "Spieler 1 WASD rückwärts → gamepad"
    );
    let free = with_controls(controls(&[PROFILE_GAMEPAD]));
    assert_eq!(free.next_profile(1, 1), PROFILE_WASD, "Spieler 2 bei freier Tastatur");
    assert_eq!(
        free.next_profile(MAX_PLAYERS as i64, 1),
        PROFILE_GAMEPAD,
        "Spieler außerhalb"
    );
}

/// Die Tastatur folgt dem Tastatur-Profil; ohne eins bleibt sie WASD. Pads haben immer das Profil gamepad.
#[test]
fn keyboard_profile_and_profile_of() {
    let cases = [
        (controls(&[PROFILE_WASD]), PROFILE_WASD),
        (controls(&[PROFILE_GAMEPAD, PROFILE_ARROWS]), PROFILE_ARROWS),
        (controls(&[PROFILE_GAMEPAD]), PROFILE_WASD),
    ];
    for (c, want) in cases {
        let s = with_controls(c.clone());
        assert_eq!(s.keyboard_profile(), want, "{c:?}: Tastatur");
        assert_eq!(
            s.profile_of(DeviceID::new(DeviceKind::Keyboard, 0)),
            want,
            "{c:?}: ProfileOf(keys1)"
        );
        assert_eq!(
            s.profile_of(DeviceID::new(DeviceKind::Gamepad, 3)),
            PROFILE_GAMEPAD,
            "{c:?}: ProfileOf(pad)"
        );
    }
}
