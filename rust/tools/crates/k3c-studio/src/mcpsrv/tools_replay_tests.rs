//! `replay_run` (Port von `tools_replay_test.go`): kein echter Lauf, die EXE ist eine Attrappe.

use std::path::Path;
use std::sync::Mutex;
use std::time::Duration;

use serde_json::json;

use super::super::kit::Workspace;
use super::super::test_helpers::connect;
use super::{REPLAY_TIMEOUT, ReplayIn, replay_run};
use crate::gameexe::{Error, Event, Options};
use crate::paperassets::test_helpers::leere_wurzel;

const NAME: &str = "1-p1-passive-d0.replay.json";

fn write_replay(dir: &Path) {
    let body =
        r#"{"seed":"2","players":1,"bot":"passive","depth":0,"days":1,"endHash":"abc123","castleFallTick":null}"#;
    std::fs::write(dir.join(NAME), body).expect("Aufnahme");
}

fn ws(root: &Path) -> Workspace {
    Workspace {
        root: root.to_path_buf(),
        name: String::new(),
    }
}

type Seen = Mutex<Option<(Options, Duration)>>;

/// `replay_run` mit einer Attrappe, die `report` liefert und sich Optionen und Frist merkt.
async fn lauf(root: &Path, input: ReplayIn, report: &str, seen: &Seen) -> Result<String, String> {
    replay_run(&ws(root), &input, |options, limit| {
        *seen.lock().expect("Optionen") = Some((options, limit));
        let report = serde_json::value::RawValue::from_string(report.to_owned()).expect("Bericht");
        std::future::ready(Ok::<_, Error>(Event {
            event: "result".to_owned(),
            report: Some(report),
            ..Event::default()
        }))
    })
    .await
}

#[tokio::test]
async fn gleicher_hash_und_burgfall() {
    let root = leere_wurzel();
    std::fs::create_dir(root.pfad.join("replays")).expect("replays");
    write_replay(&root.pfad.join("replays"));
    let rel = format!("replays/{NAME}");
    let seen = Seen::default();
    let input = ReplayIn {
        path: rel.clone(),
        ..ReplayIn::default()
    };
    let report = r#"{"ticks":900,"endHash":"abc123","castleFallTick":null,"warnings":[]}"#;
    let text = lauf(&root.pfad, input, report, &seen).await.expect("Wiedergabe");
    assert!(text.starts_with(&format!("replay_run {rel}")), "{text}");
    for want in [
        "Endzustand-Hash abc123 · Burgfall-Tick keiner",
        "Aufnahme: gleicher Endzustand",
    ] {
        assert!(text.contains(want), "{text}");
    }
    assert!(!text.contains("Warnung"), "{text}");
    let (options, limit) = seen.lock().expect("Optionen").take().expect("Aufruf");
    assert!(options.sub == "balance" && options.root == root.pfad && !options.window && options.speed == 0);
    assert_eq!(limit, REPLAY_TIMEOUT);
    assert_eq!(options.args.len(), 2);
    assert_eq!(options.args[0], "--play");
    let want = std::fs::canonicalize(root.pfad.join("replays").join(NAME)).expect("Pfad");
    assert_eq!(std::fs::canonicalize(&options.args[1]).expect("Argument"), want);
}

#[tokio::test]
async fn abweichung_und_warnung() {
    let root = leere_wurzel();
    write_replay(&root.pfad);
    let input = ReplayIn {
        path: NAME.to_owned(),
        ..ReplayIn::default()
    };
    let report = r#"{"ticks":9,"endHash":"x","castleFallTick":4,"warnings":["anderer Datenstand"]}"#;
    let text = lauf(&root.pfad, input, report, &Seen::default())
        .await
        .expect("Wiedergabe");
    for want in [
        "Seed \"2\" · 1 Spieler · Bot passive · Tiefe 0 · 1 Tage",
        "Ticks 9 · Endzustand-Hash x · Burgfall-Tick 4",
        "Aufnahme weicht ab: Endzustand-Hash abc123 · Burgfall-Tick keiner",
        "Warnung: anderer Datenstand",
    ] {
        assert!(text.contains(want), "{text}");
    }
}

/// RU40.3/AC-03: `head` öffnet das Spiel (Fenster, Zeitraffer); `speed` ohne `head` und außerhalb 1 bis 64 ist
/// abgelehnt.
#[tokio::test]
async fn head_und_speed() {
    let root = leere_wurzel();
    write_replay(&root.pfad);
    let report = r#"{"ticks":9,"endHash":"x","castleFallTick":null}"#;
    let seen = Seen::default();
    let eingabe = |head, speed| ReplayIn {
        path: NAME.to_owned(),
        head,
        speed,
    };
    lauf(&root.pfad, eingabe(true, 8), report, &seen)
        .await
        .expect("Fenster");
    let (options, limit) = seen.lock().expect("Optionen").take().expect("Aufruf");
    assert!(options.window && options.speed == 8);
    assert_eq!(limit, super::super::tools_sim::HEAD_TIMEOUT);
    for input in [eingabe(false, 2), eingabe(true, 65)] {
        let error = lauf(&root.pfad, input, report, &seen).await.expect_err("speed");
        assert!(error.contains("speed"), "{error}");
    }
    assert!(seen.lock().expect("Optionen").is_none(), "trotzdem gestartet");
}

// Die folgenden Fälle enden vor dem Aufruf der EXE.

#[tokio::test(flavor = "multi_thread")]
async fn nur_im_repo() {
    let base = leere_wurzel();
    let root = base.pfad.join("repo");
    std::fs::create_dir(&root).expect("repo");
    std::fs::write(base.pfad.join("draussen.replay.json"), r#"{"version":1}"#).expect("draußen");
    let mut server = connect(&root).await;
    let outside = base.pfad.join("draussen.replay.json").to_string_lossy().into_owned();
    for path in ["../draussen.replay.json", outside.as_str()] {
        let text = server.abgelehnt("replay_run", json!({"path": path})).await;
        assert!(text.contains("abgelehnt"), "{path}: {text}");
    }
    let text = server.abgelehnt("replay_run", json!({"path": "fehlt.json"})).await;
    assert!(text.contains("nicht gefunden"), "{text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn kaputte_datei() {
    let root = leere_wurzel();
    std::fs::write(root.pfad.join("alt.json"), "{").expect("kaputt");
    let mut server = connect(&root.pfad).await;
    let text = server.abgelehnt("replay_run", json!({"path": "alt.json"})).await;
    assert!(text.contains("kein Replay"), "{text}");
}
