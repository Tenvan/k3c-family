//! Tests zu `run.rs` (Port von `engine/balance/balance_test.go`). Tests, die Kennzahlen lesen, warten auf RU28.1.

use k3c_sim::island::{Island, add_island_player, step_island};
use k3c_sim::state::PlayerCommand;

use crate::bots::Bot;
use crate::island_world::shared;
use crate::run::{Matrix, Report, Scenario, run, run_one};
use crate::testutil::create;

/// Zwei Seeds, 2 Spieler, beide Profile, Wald, drei Tage (bis zum vierten Morgen): Seit B-652 räumt „sparsam“ erst den
/// Freiraum eines Mauer-Platzes (markieren, Bauer holt die Bäume), bevor die erste Mauer steht.
fn one_day() -> Matrix {
    Matrix {
        seeds: vec!["1".into(), "2".into()],
        players: vec![2],
        bots: vec!["passive".into(), "saver".into()],
        depths: vec![0],
        days: 3,
        gold_threshold: 20,
        ..Matrix::default()
    }
}

fn must_run(m: &Matrix) -> Report {
    run(m).expect("Lauf")
}

/// BAL1/AC-01: gleiche Seeds, Parameter und Daten → byte-gleiche Kennzahlen.
#[test]
fn gleiche_matrix_gleiche_bytes() {
    let a = must_run(&one_day()).json().expect("JSON");
    let b = must_run(&one_day()).json().expect("JSON");
    assert_eq!(a, b, "zwei Läufe mit gleicher Matrix liefern verschiedenes JSON");
}

/// BAL1/AC-02: passiv zahlt nichts, sparsam zahlt Mauern; beide Monarchen spielen gleichzeitig.
#[test]
#[ignore = "wartet auf RU28"]
fn profile_zahlen_oder_nicht() {
    let r = must_run(&one_day());
    assert_eq!(r.results.len(), 4, "Läufe");
    for res in &r.results {
        let m = res.metrics.as_ref().filter(|_| res.valid);
        let m = m.unwrap_or_else(|| panic!("{:?} ungültig: {}", res.scenario, res.error));
        match res.scenario.bot.as_str() {
            "passive" => assert!(
                m.first_wall_tick.is_none(),
                "Seed {}: passiv hat eine Mauer gebaut",
                res.scenario.seed
            ),
            "saver" => assert!(
                m.first_wall_tick.is_some(),
                "Seed {}: sparsam hat bis Tag 2 keine Mauer gebaut",
                res.scenario.seed
            ),
            _ => {}
        }
    }
}

/// Beide Monarchen bezahlen: Mit „sparsam“ geben beide Spieler Gold aus.
#[test]
fn sparsam_zwei_spieler_zahlen_beide() {
    let mut isl = create("1", &[0]);
    let (p0, p1) = (add_island_player(&mut isl, 0), add_island_player(&mut isl, 0));
    let difficulty = &shared().expect("Daten").data.difficulty;
    let mut cmds = vec![PlayerCommand::default(); 2];
    let gold = |isl: &Island| (isl.stages[0].players[p0].gold, isl.stages[0].players[p1].gold);
    let (mut prev, mut spent) = (gold(&isl), (0, 0));
    for _ in 0..10 * 60 * 30 {
        // zehn Minuten: der helle Teil von Tag 1
        let w = &isl.stages[0];
        cmds[0] = Bot::Saver.decide(&isl, w, &w.players[p0]);
        cmds[1] = Bot::Saver.decide(&isl, w, &w.players[p1]);
        step_island(&mut isl, &cmds, difficulty, 1.0 / 30.0);
        // Gold aus Bäumen und Truhen (B-952, B-953) kann den Beutel füllen: gezahlt ist, was in einem Takt abgeht.
        let now = gold(&isl);
        spent = (spent.0 + (prev.0 - now.0).max(0), spent.1 + (prev.1 - now.1).max(0));
        prev = now;
    }
    assert!(
        spent.0 != 0 && spent.1 != 0,
        "Ausgaben {}/{}: nicht beide haben gezahlt",
        spent.0,
        spent.1
    );
}

/// BAL1/AC-02: Die Kennzahlen aus B-099 sind gefüllt (Tag, Dämmerung, Welle, Fluss, Tiefe, Gold-Schwelle).
#[test]
#[ignore = "wartet auf RU28"]
fn kennzahlen_gefuellt() {
    // Go: Matrix{Seeds: {"1"}, Players: {2}, Bots: {"saver"}, Depths: {0}, Days: 1, GoldThreshold: 20}; prüft
    // `Days[0].DuskTick/GoldAtDusk/StockAtDusk`, `Expense`, `Income`, `Waves[0]`, `GoldThresholdTick`, `ReachedDepth`:
    // alles Felder von `Metrics` (RU28.1).
}

/// Ein abgebrochener Lauf erscheint als ungültig mit Seed, der Rest läuft weiter.
#[test]
fn ungueltiger_lauf_mit_seed() {
    // Der Bot „boom“ stürzt ab (Go trägt ihn in `bots` ein, hier `Bot::Boom` nur in Tests).
    let m = Matrix {
        seeds: vec!["s1".into()],
        players: vec![1],
        bots: vec!["boom".into(), "passive".into()],
        depths: vec![0, 99],
        days: 1,
        ..Matrix::default()
    };
    let r = must_run(&m);
    assert_eq!(r.results.len(), 4, "Läufe");
    // boom×0, boom×99, passive×0, passive×99 (Tiefe unbekannt)
    for (i, want) in [false, false, true, false].into_iter().enumerate() {
        let res = &r.results[i];
        assert!(
            res.valid == want && res.scenario.seed == "s1" && (want || !res.error.is_empty()),
            "Lauf {i} ({}, Tiefe {}): valid {}, Fehler {:?}",
            res.scenario.bot,
            res.scenario.depth,
            res.valid,
            res.error
        );
    }
    let b = r.json().expect("JSON");
    assert!(
        b.contains(r#""error": "abgebrochen: kaputt""#),
        "Abbruchgrund fehlt im JSON"
    );
}

#[test]
fn matrix_prueft() {
    let ok = || Matrix {
        seeds: vec!["1".into()],
        players: vec![2],
        bots: vec!["saver".into()],
        depths: vec![0],
        days: 1,
        ..Matrix::default()
    };
    let bad = [
        Matrix { seeds: vec![], ..ok() },
        Matrix {
            players: vec![5],
            ..ok()
        },
        Matrix {
            bots: vec!["gibtsnicht".into()],
            ..ok()
        },
        Matrix { days: 0, ..ok() },
    ];
    for m in &bad {
        assert!(run(m).is_err(), "{m:?}: Fehler erwartet");
    }
}

/// AC-01 (Einzellauf): Ein fester kurzer Lauf (ein Seed, ein Spieler, ein Tag) ist in Rust deterministisch: gleiche
/// Entscheidungen je Takt (Lauflängen der Eingaben) und gleicher Endzustand.
#[test]
fn fester_lauf_ist_deterministisch() {
    let sc = Scenario {
        seed: "1".into(),
        players: 1,
        bot: "saver".into(),
        days: 1,
        ..Scenario::default()
    };
    let (a, b) = (run_one(&sc, 0), run_one(&sc, 0));
    assert!(a.valid && b.valid, "Lauf ungültig: {} / {}", a.error, b.error);
    let (ra, rb) = (
        a.replay.as_ref().expect("Aufnahme"),
        b.replay.as_ref().expect("Aufnahme"),
    );
    assert_eq!(a.ticks, b.ticks);
    assert_eq!(ra.ticks, a.ticks, "eine Aufnahme je Tick");
    assert_eq!(ra.inputs, rb.inputs, "Entscheidungen je Takt verschieden");
    assert_eq!(ra.end_hash, rb.end_hash, "Endzustand verschieden");
}

/// AC-01 (Vergleich mit Go): Die Entscheidungen je Takt eines festen Laufs sind dieselben wie in Go. Der Vergleichsdatensatz
/// ist ein Go-Replay (`Replay.record`) desselben Szenarios; `testdata/` enthält keinen, und eine Aufnahme braucht einen
/// Go-Lauf (RU25.5 schreibt sie nicht neu). Lesen und Schreiben des Formats liefert `replay.rs`.
#[test]
#[ignore = "wartet auf RU28"]
fn fester_lauf_wie_go() {
    // Go-Replay unter `testdata/rust/balance/replay-saver-1d.json` (Szenario aus `fester_lauf_ist_deterministisch`);
    // verglichen werden die Lauflängen `inputs` je Spieler mit `read_replay`.
}
