//! Port von `app_playtest_test.go` ohne den Medien-Teil (RU69.15): die Commands über die Aufruf-Tabelle, mit
//! Attrappen statt ffmpeg und Spiel, auf einer Ablage in einem Temp-Ordner.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};

use super::*;
use crate::playtest::rig_tests::{Fake, GAME, Switches, fake_spawn, put};
use crate::rpc::call;
use crate::testutil::tmp_dir;

type Started = Arc<Mutex<Vec<Arc<Fake>>>>;

/// App mit einer Session unter `playtests/`, ohne `startup`; alle Werkzeuge sind Attrappen.
fn app() -> (tempfile::TempDir, Arc<App>, Started) {
    let root = tmp_dir();
    let playtests = root.path().join("playtests");
    for (name, body) in [
        ("session.json", r#"{"startMs":1,"endMs":60001,"stand":"aufgenommen"}"#),
        ("video.mkv", "0123456789"),
        ("voice.wav", "abc"),
    ] {
        put(&playtests, &format!("2026-11-21-1/{name}"), body);
    }
    let app = App::new(root.path().to_path_buf(), 0, None).with_usage_file(root.path().join("usage.json"));
    let procs = Started::default();
    let switches = Arc::new(Switches::default());
    let answers: Probe = Arc::new(|args| {
        let line = args.join(" ");
        let out = if line.ends_with("-version") {
            "ffmpeg version 9.0 x"
        } else if line.ends_with("-filters") {
            " ... whisper A->A x"
        } else {
            "\"Raummikro\" (audio)\nStream: Video: x, 1920x1080"
        };
        // Nur Monitor 0 gibt es.
        let missing = line.contains("output_idx=1");
        (out.to_owned(), if missing { Err("exit".to_owned()) } else { Ok(()) })
    });
    app.set_playtest_tools(Tools {
        spawn: fake_spawn(&procs, &switches),
        look_path: Arc::new(|_| Ok("C:/ff/ffmpeg.exe".to_owned())),
        game: Arc::new(|| Ok((GAME.to_owned(), PathBuf::from("C:/g")))),
        probe: Arc::clone(&answers),
        list: Arc::clone(&answers),
        out: answers,
        model: Arc::new(|| "fehlt.bin".to_owned()),
    });
    app.mark_ready();
    (root, Arc::new(app), procs)
}

fn rpc(app: &Arc<App>, method: &str, args: &[Value]) -> Value {
    call(app, method, args).unwrap_or_else(|e| panic!("{method}: {}", e.0))
}

fn refused(app: &Arc<App>, method: &str, args: &[Value]) -> String {
    call(app, method, args).unwrap_err().0
}

/// Die Commands listen, planen und löschen erst mit Bestätigung.
#[test]
fn playtest_bindings_liste_plan_loeschen() {
    let (root, app, procs) = app();
    let list = rpc(&app, "PlaytestList", &[]);
    assert_eq!(list.as_array().unwrap().len(), 1);
    assert_eq!(list[0]["stage"], "aufgenommen");
    assert_eq!(list[0]["durationMs"], 60000);
    let args = [json!("video"), json!("2026-11-21-1")];
    assert_eq!(rpc(&app, "PlaytestPlan", &args)["total"], 10);
    let unconfirmed = [json!("video"), json!("2026-11-21-1"), json!(false)];
    assert_eq!(refused(&app, "PlaytestDelete", &unconfirmed), "Löschen nicht bestätigt");
    let video = root.path().join("playtests").join("2026-11-21-1").join("video.mkv");
    assert!(video.exists(), "ohne Bestätigung gelöscht");
    let escape = [json!("session"), json!(".."), json!(true)];
    assert_eq!(refused(&app, "PlaytestDelete", &escape), "ungültige Session-Kennung");
    let confirmed = [json!("video"), json!("2026-11-21-1"), json!(true)];
    assert_eq!(rpc(&app, "PlaytestDelete", &confirmed)["total"], 10);
    assert!(!video.exists());
    assert!(procs.lock().unwrap().is_empty(), "Prozess gestartet");
}

/// Die acht Commands der Auswertung; ohne Modell startet kein Prozess.
#[test]
fn playtest_auswertung_ueber_die_commands() {
    let (root, app, procs) = app();
    let session = json!("2026-11-21-1");
    put(
        &root.path().join("playtests"),
        "2026-11-21-1/markers.jsonl",
        "{\"ts\":2,\"kind\":\"pad\",\"player\":1}\n",
    );
    put(
        &root.path().join("playtests"),
        "2026-11-21-1/frames/marker-1.jpg",
        "jpg",
    );
    let idle = json!({"running": false, "id": "", "step": "", "done": 0, "total": 0, "message": "", "error": ""});
    assert_eq!(rpc(&app, "PlaytestTranscribeStatus", &[]), idle);
    assert_eq!(refused(&app, "PlaytestTranscribeCancel", &[]), "keine Auswertung aktiv");
    assert!(rpc(&app, "PlaytestModels", &[]).is_array());
    let options = json!({"model": "", "language": ""});
    let error = refused(&app, "PlaytestTranscribe", &[session.clone(), options]);
    assert!(error.starts_with("Modell fehlt: fehlt.bin"), "{error}");
    assert_eq!(rpc(&app, "PlaytestTranscribeStatus", &[]), idle);
    let timeline = rpc(&app, "PlaytestTimeline", std::slice::from_ref(&session));
    assert_eq!(timeline["entries"][0]["frame"], "frames/marker-1.jpg");
    let skip = [session.clone(), json!(1), json!(true)];
    assert_eq!(rpc(&app, "PlaytestFrameSkip", &skip), Value::Null);
    let timeline = rpc(&app, "PlaytestTimeline", std::slice::from_ref(&session));
    assert_eq!(timeline["entries"][0]["skip"], true);
    assert_eq!(rpc(&app, "PlaytestFindings", std::slice::from_ref(&session)), json!([]));
    put(
        &root.path().join("playtests"),
        "2026-11-21-1/findings.json",
        r#"{"findings":[{"id":"f1","quote":"x","ts":1}]}"#,
    );
    let patch = [session, json!("f1"), json!({"selected": true, "type": "Idee"})];
    let changed = rpc(&app, "PlaytestFindingUpdate", &patch);
    assert_eq!((&changed["selected"], &changed["type"]), (&json!(true), &json!("Idee")));
    assert!(procs.lock().unwrap().is_empty(), "Prozess gestartet");
}

#[test]
fn playtest_aufnahme_ueber_die_commands() {
    let (_root, app, procs) = app();
    assert_eq!(
        rpc(&app, "PlaytestStatus", &[]),
        json!({"running": false, "id": "", "startMs": 0, "encoder": "", "voiceOnly": false, "starting": false})
    );
    assert_eq!(refused(&app, "PlaytestStop", &[]), "keine Aufnahme aktiv");
    let devices = rpc(&app, "PlaytestDevices", &[]);
    assert_eq!(devices["mics"], json!(["Raummikro"]));
    assert_eq!(devices["monitors"][0]["label"], "Monitor 1 (1920×1080)");
    assert_eq!(devices["ffmpegPath"], "C:/ff/ffmpeg.exe");
    let check = rpc(&app, "PlaytestCheck", &[]);
    assert!(
        check["message"]
            .as_str()
            .unwrap()
            .starts_with("Modell fehlt: fehlt.bin"),
        "{check}"
    );
    // Mikro-Session: nur ffmpeg, Dauer der Anlaufprüfung (2 s).
    let options = json!({"mic": "Raummikro", "monitor": 0, "gameArgs": null, "voiceOnly": true});
    let status = rpc(&app, "PlaytestStart", std::slice::from_ref(&options));
    assert_eq!((&status["running"], &status["voiceOnly"]), (&json!(true), &json!(true)));
    assert!(status.get("stats").is_none(), "{status}");
    assert_eq!(refused(&app, "PlaytestStart", &[options]), "Aufnahme läuft schon");
    let live = rpc(&app, "PlaytestStatus", &[]);
    assert_eq!(live["stats"]["noVideo"], true);
    let id = status["id"].as_str().unwrap().to_owned();
    let delete = [json!("session"), json!(id), json!(true)];
    assert_eq!(
        refused(&app, "PlaytestDelete", &delete),
        "Aufnahme läuft, Löschen gesperrt"
    );
    assert_eq!(rpc(&app, "PlaytestStop", &[]), Value::Null);
    app.recorder().unwrap().wait();
    assert_eq!(rpc(&app, "PlaytestStatus", &[])["running"], false);
    let procs = procs.lock().unwrap();
    assert_eq!(procs.len(), 1);
    assert_eq!((procs[0].quits(), procs[0].kills()), (1, 0), "ffmpeg endet mit q");
}
