//! Tests zu Mischungen je Spieler (Port von `engine/balance/mix_test.go`, WZ28.12/AC-13, B-611).

use crate::bots::{Bot, bot_by_name, check_players};
use crate::run::{Matrix, Scenario, new_island, run_one};

/// Eine Mischung „a+b“ gibt Spieler 1 das Profil a, Spieler 2 das Profil b.
#[test]
fn mischung_je_spieler() {
    let sc = Scenario {
        seed: "1".into(),
        players: 2,
        days: 1,
        ..Scenario::default()
    };
    let isl = new_island(&sc).expect("Insel");
    let mix = bot_by_name("walls+passive").expect("walls+passive unbekannt");
    let w = &isl.stages[0];
    let (p0, p1) = (&w.players[0], &w.players[1]);
    let (m0, m1) = (mix.decide(&isl, w, p0), mix.decide(&isl, w, p1));
    assert!(
        m0 == Bot::Walls.decide(&isl, w, p0) && m1 == Bot::Passive.decide(&isl, w, p1),
        "Mischung steuert nicht je Spieler: {m0:?} {m1:?}"
    );
}

#[test]
fn mischung_ungueltig() {
    for bad in [
        "saver+",
        "+saver",
        "saver+raten",
        "saver+coop2",
        "coop4+saver",
        "saver,passive",
    ] {
        assert!(bot_by_name(bad).is_none(), "bot_by_name({bad:?}): abgelehnt erwartet");
    }
    let err = check_players("saver+passive", 3).expect_err("Anzahl ≠ Spieler");
    assert!(err.to_string().contains("2 Profile"), "Anzahl ≠ Spieler: {err}");
    check_players("saver+passive+walls", 3).expect("drei Profile, drei Spieler");
}

/// Lauf nimmt die Mischung an; ein Lauf mit Mischung ist gültig. Der Teil mit den Zielen (`DefaultTargets`, `Restrict`)
/// steht in `mischung_im_lauf_ziele`.
#[test]
fn mischung_im_lauf() {
    let m = Matrix {
        seeds: vec!["1".into()],
        players: vec![2],
        bots: vec!["saver+passive".into()],
        depths: vec![0],
        days: 1,
        ..Matrix::default()
    };
    m.check_bots().expect("Mischung");
    let sc = Scenario {
        seed: "1".into(),
        players: 2,
        bot: "saver+passive".into(),
        days: 1,
        ..Scenario::default()
    };
    let res = run_one(&sc, 0);
    assert!(res.valid, "Lauf mit Mischung ungültig: {}", res.error);
}

/// Go: `ts.Restrict(Scope{Players: 2, Bot: "saver+passive"})` nimmt die Mischung an, mit drei Spielern lehnt es sie ab.
#[test]
#[ignore = "wartet auf RU28"]
fn mischung_im_lauf_ziele() {
    // `DefaultTargets` und `Targets::restrict` kommen mit RU28.
}
