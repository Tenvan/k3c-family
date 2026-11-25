//! Tests von `BalanceApp` (RU40.2, AC-02): Das Ergebnis hängt weder vom Modus noch vom Zeitraffer ab und gleicht dem Bericht
//! von `sim balance` headless (Fixture `balance-matrix-klein`). Der echte Fensterlauf braucht einen Bildschirm und gehört in
//! die Abnahme.

use k3c_balance::replay::play;
use k3c_balance::run::{Matrix, Report, RunResult, Scenario, run, run_one};
use k3c_platform::run_headless;
use serde_json::Value;

use super::EXIT_USAGE;
use super::balance_run::Done;
use super::balance_window::{BalanceApp, Job, Outcome};
use super::stop::Stop;
use super::testutil::{call, root};

/// `legacy`: rechnet wie das Go-Orakel (Fixture); sonst mit den nativen Regeln.
fn matrix(legacy: bool) -> Matrix {
    Matrix {
        legacy,
        seeds: vec!["k3c-dev".into()],
        bots: vec!["saver".into()],
        players: vec![1],
        depths: vec![0],
        days: 1,
        ..Matrix::default()
    }
}

fn jobs(m: &Matrix) -> Vec<Job> {
    let scenarios = m.scenarios().expect("Szenarien");
    scenarios
        .into_iter()
        .map(|s| Job::Scenario(s, m.gold_threshold))
        .collect()
}

fn results(done: Vec<Done>) -> Vec<RunResult> {
    done.into_iter()
        .map(|d| match d {
            Done::Scenario(r) => *r,
            Done::Replay(_) => panic!("Wiedergabe statt Szenario"),
        })
        .collect()
}

fn report_text(m: &Matrix, done: Vec<Done>) -> String {
    Report {
        matrix: m.clone(),
        results: results(done),
    }
    .json()
    .expect("JSON")
}

fn finished(app: BalanceApp) -> Vec<Done> {
    match app.outcome() {
        Outcome::Finished(d) => d,
        Outcome::Aborted => panic!("abgebrochen"),
    }
}

/// Der Bericht der Fixture `balance-matrix-klein` (Go-Orakel), als Text der Zeile `result`.
fn fixture_report() -> Value {
    let path = root().join("testdata/rust/runs/balance-matrix-klein.json");
    let f: Value = serde_json::from_str(&std::fs::read_to_string(path).expect("Fixture")).expect("JSON");
    let line: Value = serde_json::from_str(f["out"][0].as_str().expect("Zeile")).expect("JSON-Zeile");
    line["report"].clone()
}

// AC-02: Matrix `balance-matrix-klein` bei Zeitraffer 1 und 16 (ohne Zeichnen), gleich headless und gleich der Fixture.
#[test]
fn zeitraffer_aendert_den_bericht_nicht() {
    let m = matrix(true);
    let want = run(&m).expect("Lauf").json().expect("JSON");
    for speed in [1, 16] {
        let mut app = BalanceApp::headless(jobs(&m), Stop::never(), speed, None);
        run_headless(&mut app, u32::MAX, None);
        let got = report_text(&m, finished(app));
        assert_eq!(got, want, "speed {speed}");
        let v: Value = serde_json::from_str(&got).expect("JSON");
        assert_eq!(v, fixture_report(), "speed {speed}: Fixture balance-matrix-klein");
    }
}

// Mit Fenster-App (Zeichnen ohne Grafik auf einer Null-Leinwand) dasselbe Ergebnis; Fortschritt zählt fertige Läufe.
#[test]
fn fenster_app_gleich_headless() {
    let m = matrix(false);
    let want = run(&m).expect("Lauf").json().expect("JSON");
    let mut seen: Vec<(usize, usize)> = Vec::new();
    let progress = Box::new(|d: usize, t: usize| seen.push((d, t)));
    let mut app = BalanceApp::windowed(jobs(&m), Stop::never(), 64, Some(progress), None);
    run_headless(&mut app, u32::MAX, None);
    assert!(app.frames > 0, "es wurde gezeichnet");
    assert_eq!(report_text(&m, finished(app)), want);
    assert_eq!(seen, vec![(1, 1)]);
}

// Ein Szenario, das nicht startet (unbekannter Bot), ergibt dasselbe ungültige Ergebnis wie `run_one`; der Rest läuft weiter.
#[test]
fn fehlerhaftes_szenario_wie_headless() {
    let bad = Scenario {
        seed: "k3c-dev".into(),
        players: 1,
        bot: "gibt-es-nicht".into(),
        depth: 0,
        days: 1,
        ..Scenario::default()
    };
    let mut app = BalanceApp::headless(vec![Job::Scenario(bad.clone(), 0)], Stop::never(), 4, None);
    run_headless(&mut app, u32::MAX, None);
    let got = results(finished(app));
    let want = run_one(&bad, 0);
    assert_eq!(got.len(), 1);
    assert!(
        !got[0].valid && got[0].error == want.error && got[0].ticks == want.ticks,
        "{:?}",
        got[0].error
    );
}

// Eine Aufnahme spielt im Fenster-App dasselbe Ergebnis wie `play`.
#[test]
fn aufnahme_gleich_play() {
    let m = matrix(false);
    let mut app = BalanceApp::headless(jobs(&m), Stop::never(), 64, None);
    run_headless(&mut app, u32::MAX, None);
    let replay = results(finished(app))[0].replay.clone().expect("Aufnahme");
    let want = play(&replay).expect("play");
    let mut app = BalanceApp::headless(vec![Job::Replay(Box::new(replay))], Stop::never(), 16, None);
    run_headless(&mut app, u32::MAX, None);
    match finished(app).pop() {
        Some(Done::Replay(pb)) => assert_eq!(pb.expect("Wiedergabe"), want),
        _ => panic!("keine Wiedergabe"),
    }
}

// Fenster vor dem Ende geschlossen und Abbruch: `Aborted`, keine Ergebnisse.
#[test]
fn abbruch_und_fenster_zu_frueh_zu() {
    let m = matrix(false);
    let mut app = BalanceApp::headless(jobs(&m), Stop::never(), 1, None);
    run_headless(&mut app, 10, None);
    assert!(matches!(app.outcome(), Outcome::Aborted), "Fenster vor dem Ende zu");
    let stop = Stop::never();
    let mut app = BalanceApp::headless(jobs(&m), stop.clone(), 1, None);
    run_headless(&mut app, 5, None);
    stop.trigger();
    run_headless(&mut app, 5, None);
    assert!(matches!(app.outcome(), Outcome::Aborted), "Abbruch");
}

// Ohne Läufe ist die App sofort fertig; nach dem Ende bleibt das Fenster `HOLD_FRAMES` Bilder stehen.
#[test]
fn leere_warteschlange_ist_fertig() {
    let mut app = BalanceApp::windowed(Vec::new(), Stop::never(), 1, None, None);
    run_headless(&mut app, u32::MAX, None);
    assert!(matches!(app.outcome(), Outcome::Finished(d) if d.is_empty()));
}

// --window gilt nicht für --tune, --vary, --curves und --insights (Exit 2, vor dem Öffnen eines Fensters).
#[test]
fn window_nur_fuer_matrix_targets_play() {
    for extra in [
        vec!["--targets", "--tune", "sweep"],
        vec!["--curves"],
        vec!["--insights", "x.json"],
    ] {
        let mut args = vec!["balance", "--window"];
        args.extend(extra.iter());
        let (code, out, _) = call("", &args);
        assert!(
            code == EXIT_USAGE && out.contains("--window gilt"),
            "{extra:?}: Exit {code}, {out}"
        );
    }
    let (code, _, err) = call("", &["balance", "--window", "--headless"]);
    assert_eq!(code, EXIT_USAGE, "{err}");
}
