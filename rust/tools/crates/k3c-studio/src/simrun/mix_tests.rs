//! Port von `mix_test.go`.

use super::*;
use crate::simrun::test_helpers::repo_root;

fn bot(name: &str) -> Spec {
    Spec {
        bot: name.to_owned(),
        ..Spec::default()
    }
}

/// WZ28.12/AC-13 (B-611): eine Mischung je Spieler annehmen, unbekannte Profile darin ablehnen.
#[test]
fn spec_mischung() {
    let mix = Spec {
        players: 2,
        ..bot("saver+passive")
    };
    assert_eq!(mix.check(&repo_root()), Ok(()));
    for bad in ["saver+raten", "saver+coop2", "saver+"] {
        assert!(bot(bad).check(&repo_root()).is_err(), "{bad:?}: abgelehnt erwartet");
    }
}

/// BS11.4/AC-05 (B-642/AC-05): world und days werden Flags der EXE; eine unbekannte Welt wird abgelehnt.
#[test]
fn spec_welt_und_tage() {
    let world = |name: &str| Spec {
        world: name.to_owned(),
        ..Spec::default()
    };
    let ok = Spec {
        bot: "coop2".to_owned(),
        days: 40,
        ..world("standard")
    };
    assert_eq!(ok.check(&repo_root()), Ok(()));
    let args = ok.args("dir").join(" ");
    for want in ["--bot coop2", "--days 40", "--world standard"] {
        assert!(args.contains(want), "{want:?} fehlt in {args}");
    }
    let long = Spec {
        days: 101,
        ..Spec::default()
    };
    for bad in [world("nordmeer"), world("../x"), long] {
        assert!(bad.check(&repo_root()).is_err(), "{bad:?}: Fehler erwartet");
    }
}

/// Alle Felder werden eigene Argumente in der Reihenfolge von Go.
#[test]
fn spec_args_vollstaendig() {
    let spec = Spec {
        seeds: 2,
        players: 2,
        days: 3,
        depth: Some(0),
        bot: "saver".to_owned(),
        grade: "leicht".to_owned(),
        world: "standard".to_owned(),
        helps: "voll".to_owned(),
        vary: vec!["a.json:b=1".to_owned()],
        tune: Some(crate::simrun::Tune {
            mode: "sweep".to_owned(),
            params: vec!["a.json:b=1..2".to_owned()],
            steps: 3,
            budget: 9,
        }),
        ..Spec::default()
    };
    let want = "balance --targets --report-dir dir --progress --baseline testdata/balance/baseline.json --seeds 2 \
                --players 2 --days 3 --depth 0 --bot saver --grade leicht --world standard --helps voll \
                --vary a.json:b=1 --tune sweep --param a.json:b=1..2 --steps 3 --budget 9";
    assert_eq!(spec.args("dir").join(" "), want);
    assert_eq!(spec.check(&repo_root()), Ok(()));
    let json = serde_json::to_string(&Spec {
        depth: Some(0),
        ..Spec::default()
    });
    assert_eq!(json.expect("JSON"), r#"{"depth":0}"#);
}
