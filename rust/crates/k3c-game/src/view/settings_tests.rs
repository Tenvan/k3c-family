use super::*;

fn s(f: impl FnOnce(&mut Settings)) -> Settings {
    let mut s = Settings::default();
    f(&mut s);
    s
}

#[test]
fn settings_normalized() {
    let cases = [
        (Settings::default(), s(|s| s.language = DEFAULT_LANGUAGE.into())),
        (default_settings(), default_settings()),
        // unbekannter Anzeige-Modus: Fenster (B-826)
        (
            s(|s| s.window = "kino".into()),
            s(|s| s.language = DEFAULT_LANGUAGE.into()),
        ),
        (
            s(|s| {
                (s.music, s.effects) = (1.5, -0.2);
                (s.window, s.language) = (WINDOW_FULLSCREEN.into(), "en".into());
            }),
            s(|s| {
                (s.music, s.effects) = (1.0, 0.0);
                (s.window, s.language) = (WINDOW_FULLSCREEN.into(), "en".into());
            }),
        ),
        (
            s(|s| (s.music, s.effects, s.language) = (f64::NAN, 0.4, "fr".into())),
            s(|s| (s.music, s.effects, s.language) = (1.0, 0.4, "de".into())),
        ),
        (
            s(|s| (s.flashes_off, s.guide_off, s.guide_seen) = (true, true, -3)),
            s(|s| (s.flashes_off, s.guide_off, s.language) = (true, true, "de".into())),
        ),
        (
            s(|s| s.guide_seen = 5),
            s(|s| (s.guide_seen, s.language) = (5, "de".into())),
        ),
    ];
    for (input, mut want) in cases {
        want.controls = super::super::profile::default_controls();
        want.stress = super::super::stress_limits::default_stress();
        assert_eq!(input.normalized(), want, "{input:?}.normalized()");
    }
}

/// River: nur die zwei Fassungen bleiben, alles andere fällt auf "" (Startwert); NextRiver wechselt von Wert oder Start.
#[test]
fn settings_river() {
    for v in ["", RIVER_SHADER, RIVER_TILES] {
        assert_eq!(s(|s| s.river = v.into()).normalized().river, v, "River {v:?}");
    }
    assert_eq!(s(|s| s.river = "lava".into()).normalized().river, "", "unbekannt");
    let cases = [
        ("", RIVER_SHADER, RIVER_TILES),
        ("", RIVER_TILES, RIVER_SHADER),
        (RIVER_TILES, RIVER_SHADER, RIVER_SHADER),
        (RIVER_SHADER, RIVER_TILES, RIVER_TILES),
    ];
    for (river, start, want) in cases {
        assert_eq!(
            s(|s| s.river = river.into()).next_river(start),
            want,
            "NextRiver({river:?}, start {start:?})"
        );
    }
}
