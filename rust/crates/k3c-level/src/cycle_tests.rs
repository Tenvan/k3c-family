use crate::biome::Cycle;
use crate::testutil::biome;

// W12.1 (B-470/AC-02): Die Dauer des Morgengrauens steht in data/biomes/*.json › cycle; fehlt eine Dauer des
// Tag/Nacht-Zyklus oder ist sie nicht positiv, lehnt das Laden ab.

#[test]
fn wald_hat_vier_phasen() {
    let c = biome("forest").cycle;
    assert!(
        c.r#type == "dayNight"
            && c.day_minutes > 0.0
            && c.twilight_minutes > 0.0
            && c.night_minutes > 0.0
            && c.dawn_minutes > 0.0,
        "Zyklus {c:?}"
    );
}

#[test]
fn cycle_check_lehnt_fehlende_oder_nicht_positive_dauer_ab() {
    let ok = Cycle {
        r#type: "dayNight".into(),
        day_minutes: 6.0,
        twilight_minutes: 2.0,
        night_minutes: 4.0,
        dawn_minutes: 2.0,
        ..Cycle::default()
    };
    ok.check().expect("gültiger Zyklus");
    type Mutate = fn(&mut Cycle);
    let cases: [(&str, Mutate); 4] = [
        ("dawnMinutes", |c| c.dawn_minutes = 0.0),
        ("nightMinutes", |c| c.night_minutes = -1.0),
        ("dayMinutes", |c| c.day_minutes = 0.0),
        ("twilightMinutes", |c| c.twilight_minutes = -0.5),
    ];
    for (name, mutate) in cases {
        let mut c = ok.clone();
        mutate(&mut c);
        let err = c.check();
        assert!(err.as_ref().is_err_and(|e| e.contains(name)), "{name}: Fehler {err:?}");
    }
    let pool = Cycle {
        r#type: "aggressionPool".into(),
        percent_per_minute: 1.0,
        ..Cycle::default()
    };
    assert!(pool.check().is_ok(), "Aggressionspool ohne Phasen abgelehnt");
}
