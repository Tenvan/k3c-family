//! Tests zu `replay.rs` (Port von `engine/balance/replay_test.go`).

use k3c_sim::state::PlayerCommand;
use serde_json::{Value, json};

use crate::replay::{REPLAY_VERSION, Replay, data_hash, play, read_replay};
use crate::run::{RunResult, Scenario, run_one};

/// Spielt einen sparsamen Lauf (Seed 1, 2 Spieler, 1 Tag) und liefert ihn samt Datei (Go: `recorded`).
fn recorded() -> (RunResult, String) {
    let sc = Scenario {
        seed: "1".into(),
        players: 2,
        bot: "saver".into(),
        depth: 0,
        days: 1,
        ..Scenario::default()
    };
    let res = run_one(&sc, 0);
    assert!(res.valid && res.replay.is_some(), "Lauf ungültig: {}", res.error);
    let text = res.replay.as_ref().expect("Aufnahme").json().expect("JSON");
    (res, text)
}

fn must_read(text: &str) -> Replay {
    read_replay(text).unwrap_or_else(|e| panic!("{e}"))
}

/// BAL1/AC-03: Aufnahme eines Bot-Laufs und Wiedergabe aus der Datei → gleicher Endzustand-Hash und Burgfall-Tick.
#[test]
fn aufnahme_und_wiedergabe_gleicher_hash() {
    let (res, text) = recorded();
    let pb = play(&must_read(&text)).expect("Wiedergabe");
    assert!(pb.warnings.is_empty(), "unerwartete Warnungen: {:?}", pb.warnings);
    let rp = res.replay.as_ref().expect("Aufnahme");
    assert!(
        !pb.end_hash.is_empty() && pb.end_hash == rp.end_hash,
        "Endzustand-Hash: Wiedergabe {:?}, Aufnahme {:?}",
        pb.end_hash,
        rp.end_hash
    );
    let fall = res.metrics.as_ref().and_then(|m| m.castle_fall_tick);
    assert_eq!(pb.ticks, res.ticks, "Ticks");
    assert_eq!(pb.castle_fall_tick, fall, "Burgfall");
}

/// Die Lauflängen ergeben bei der Wiedergabe exakt dieselben Befehle je Tick.
#[test]
fn lauflaengen_exakt() {
    let cmd = |move_x: f64, sprint: bool, pay: bool| PlayerCommand {
        move_x,
        sprint,
        pay,
        ..PlayerCommand::default()
    };
    let ticks = [
        [cmd(1.0, false, false), cmd(0.0, false, false)],
        [cmd(1.0, false, false), cmd(0.0, false, true)],
        [cmd(0.0, false, false), cmd(0.0, false, true)],
        [cmd(-1.0, true, false), cmd(0.0, false, false)],
    ];
    let mut r = Replay::default();
    for cmds in &ticks {
        r.record(cmds);
    }
    for i in 0..2 {
        let got: Vec<&PlayerCommand> = r.inputs[i]
            .iter()
            .flat_map(|s| std::iter::repeat_n(&s.command, s.n as usize))
            .collect();
        for (tick, c) in got.iter().enumerate() {
            assert_eq!(**c, ticks[tick][i], "Spieler {i} Tick {tick}");
        }
        assert!(
            got.len() == ticks.len() && r.ticks == ticks.len() as i64,
            "Spieler {i}: {} Ticks, erwartet {}",
            got.len(),
            ticks.len()
        );
    }
    assert!(
        r.inputs[0].len() == 3 && r.inputs[1].len() == 3,
        "Lauflängen nicht zusammengefasst: {:?}",
        r.inputs
    );
}

/// BAL1/AC-04: Die Datei enthält Version, Seed, Parameter, Datenstand-Hash und Eingaben.
#[test]
fn datei_enthaelt_seed_parameter_hash_eingaben() {
    let (_, text) = recorded();
    let f: Value = serde_json::from_str(&text).expect("JSON");
    let want = json!({"version": REPLAY_VERSION, "seed": "1", "players": 2, "bot": "saver", "depth": 0, "days": 1,
        "dataHash": data_hash()});
    for (k, v) in want.as_object().expect("Objekt") {
        assert_eq!(&f[k], v, "{k}");
    }
    assert_eq!(f["inputs"].as_array().map(Vec::len), Some(2), "je Spieler eine Liste");
    assert_eq!(
        data_hash().len(),
        64,
        "Datenstand-Hash {:?} ist kein SHA-256",
        data_hash()
    );
}

/// BAL1/AC-05: unbekannte Version → Fehler mit Versionsangabe; kaputtes JSON → Fehler mit Zeilennummer.
#[test]
fn unbekannte_version_und_kaputte_datei() {
    let (_, text) = recorded();
    let err = read_replay(&text.replacen("\"version\": 1", "\"version\": 99", 1)).expect_err("Version 99");
    assert!(err.to_string().contains("Version 99"), "Fehler {err}");
    let err = read_replay("{\n  \"version\": 1,\n  \"seed\": 1\n}").expect_err("Typfehler");
    assert!(err.to_string().contains("Zeile 3"), "Typfehler: {err}");
    let err = read_replay("{\n  \"version\": 1\n  \"seed\": \"1\"\n}").expect_err("Syntaxfehler");
    assert!(err.to_string().contains("Zeile 3"), "Syntaxfehler: {err}");
}

/// BAL1/AC-05: anderer Datenstand-Hash → Warnung, die Wiedergabe läuft trotzdem durch.
#[test]
fn anderer_datenstand_warnt() {
    let (res, text) = recorded();
    let mut r = must_read(&text);
    r.data_hash = "anders".into();
    let pb = play(&r).expect("Wiedergabe");
    assert!(
        pb.warnings.len() == 1 && pb.warnings[0].contains("Datenstand"),
        "Warnungen {:?}, erwartet eine zum Datenstand",
        pb.warnings
    );
    assert_eq!(
        pb.end_hash,
        res.replay.expect("Aufnahme").end_hash,
        "Wiedergabe mit Warnung lief nicht bis zum Ende"
    );
}

/// Riesige Tick-Zahl aus einer Datei → Fehler statt endloser Wiedergabe.
#[test]
fn zu_viele_ticks_abgelehnt() {
    let err = read_replay(r#"{"version":1,"players":1,"ticks":2000000000,"inputs":[[{"n":2000000000}]]}"#)
        .expect_err("Tick-Obergrenze");
    assert!(err.to_string().contains("höchstens"), "Fehler {err}");
}

/// Die Datei ist in Rust gelesen und wieder geschrieben dieselbe (Lauflängen, Szenario, Hashes).
#[test]
fn datei_hin_und_zurueck() {
    let (res, text) = recorded();
    let r = must_read(&text);
    assert_eq!(&r, res.replay.as_ref().expect("Aufnahme"));
    assert_eq!(r.json().expect("JSON"), text);
}
