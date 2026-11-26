//! Go `mix_test.go`: Mischungen je Spieler (B-611).

use super::names::{check_players, known_bot};
use super::scope::Scope;
use super::targets::default_targets;
use super::test_helpers::repo_root;

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
        assert!(!known_bot(bad), "known_bot({bad:?}): abgelehnt erwartet");
    }
    let got = check_players("saver+passive", 3).expect_err("Anzahl ≠ Spieler");
    assert!(got.contains("2 Profile"), "{got}");
    check_players("saver+passive+walls", 3).expect("drei Profile, drei Spieler");
}

/// Die Ziele nehmen die Mischung an (den Lauf prüft `k3c-game sim balance`).
#[test]
fn mischung_im_lauf() {
    let root = repo_root();
    let targets = default_targets(&root).expect("Ziele");
    let with_players = |players| {
        targets.restrict(
            &root,
            &Scope {
                players: Some(players),
                bot: "saver+passive".to_owned(),
                ..Scope::default()
            },
        )
    };
    with_players(2).expect("Ziele mit Mischung");
    assert!(
        with_players(3).is_err(),
        "Ziele: Mischung mit falscher Spielerzahl angenommen"
    );
    assert!(known_bot("walls+passive"), "walls+passive unbekannt");
}
