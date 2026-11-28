//! Port von `game_test.go` (Prozess-Dienste wie das Spiel). Keine Spiel-EXE wird gestartet.

use std::path::Path;
use std::sync::atomic::Ordering;

use super::config::{load, shift};
use super::config_tests::{load_text, real_config};
use super::controller_tests::controller;
use super::outlog_tests::read_out;
use super::process::program;
use super::testutil::{Fake, options, svc, wait_for};
use super::*;
use crate::testutil::tmp_dir;

/// Ohne Port, gesund, solange der Prozess läuft, nur von Hand gestartet.
fn game(name: &str) -> Service {
    Service {
        name: name.to_owned(),
        command: vec!["x".to_owned()],
        health: HEALTH_PROCESS.to_owned(),
        manual: true,
        stop_before: vec!["build".to_owned(), "rust:game:build".to_owned()],
        ..Service::default()
    }
}

#[test]
fn services_json_kennt_die_spiel_dienste() {
    let list = load(&real_config()).unwrap();
    let find = |name: &str| list.iter().find(|s| s.name == name).unwrap();
    let (dev, release) = (find("game-dev"), find("game-release"));
    assert_eq!(dev.command.join(" "), "task rust:game:dev");
    assert_eq!((dev.health.as_str(), dev.port, dev.manual), ("process", 0, true));
    let exe = "dist/k3c-game/k3c-game.exe";
    assert_eq!(release.command.join(" "), exe);
    assert_eq!((release.health.as_str(), release.manual), ("process", true));
    assert_eq!(release.needs, exe);
    assert!(release.needs_hint.contains("task build"), "{release:?}");
    for task in ["build", "rust:game:build"] {
        assert!(release.stop_before.iter().any(|t| t == task), "{release:?}");
    }
}

/// Zwei Prozess-Dienste sind kein „port doppelt“, ein Port bei health process ist ein Fehler.
#[test]
fn laden_prozess_dienste() {
    let two = r#"[{"name":"A","command":["x"],"health":"process"},{"name":"B","command":["x"],"health":"process"}]"#;
    load_text(two).unwrap();
    for (content, want) in [
        (
            r#"[{"name":"A","command":["x"],"port":5,"health":"process"}]"#,
            "ohne port",
        ),
        (r#"[{"name":"A","command":["x"],"health":"http"}]"#, "port 0 ungültig"),
        (
            r#"[{"name":"A","command":["x"],"health":"process","needs":"../x"}]"#,
            r#"needs "../x""#,
        ),
        (
            r#"[{"name":"A","command":["x"],"health":"process","stopBefore":["a b"]}]"#,
            r#"stopBefore "a b""#,
        ),
    ] {
        let error = load_text(content).expect_err(content);
        assert!(error.contains(want), "{content}: {error}");
    }
    let shifted = shift(&[game("g")], 10);
    assert_eq!((shifted[0].port, shifted[0].health_url()), (0, String::new()));
}

#[tokio::test(flavor = "multi_thread")]
async fn prozess_dienst_start_stop_und_ende() {
    let f = Fake::new(false); // die Prüfung schlüge fehl: ein Prozess-Dienst braucht sie nicht
    f.busy.store(true, Ordering::SeqCst);
    let c = controller(&f, vec![game("game-dev")]);
    c.adopt().await;
    assert_eq!(c.statuses()[0].state, State::Stopped, "Adopt übernimmt das Spiel");
    let ctx = Cancel::default();
    c.start_all(&ctx).await.unwrap();
    assert_eq!(f.count(), 0, "StartAll startet das Spiel");
    let (_, health) = c.health("game-dev").await.unwrap();
    assert_eq!(health, Err("prozess läuft nicht".to_owned()));
    assert_eq!(c.start(&ctx, "game-dev").await.unwrap().state, State::Running);
    assert_eq!(c.health("game-dev").await.unwrap().1, Ok(()));
    let st = c.stop("game-dev", false).await.unwrap();
    assert_eq!(st.state, State::Stopped);
    assert!(f.last().killed.load(Ordering::SeqCst));
    c.start(&ctx, "game-dev").await.unwrap();
    f.last().exit();
    wait_for("gestoppt nach Ende", || {
        let st = c.statuses().remove(0);
        st.state == State::Stopped && st.pid == 0
    })
    .await;
    assert_eq!(c.statuses()[0].last_error, "", "Ende ohne Fehler ist kein Ausfall");
}

/// Fehlt die EXE (`needs`), meldet der Start den Hinweis statt eines Prozessfehlers.
#[tokio::test(flavor = "multi_thread")]
async fn prozess_dienst_ohne_exe() {
    let root = tmp_dir();
    let f = Fake::new(false);
    let service = Service {
        needs: "dist/k3c-game/k3c-game.exe".to_owned(),
        needs_hint: "erst task build".to_owned(),
        ..game("game-release")
    };
    let mut opts = options(&f);
    opts.root = root.path().to_path_buf();
    let c = Controller::new(vec![service], opts);
    let ctx = Cancel::default();
    let error = c.start(&ctx, "game-release").await.unwrap_err();
    let st = error.status.unwrap();
    assert_eq!((st.state, f.count()), (State::Failed, 0));
    assert_eq!(st.last_error, "dist/k3c-game/k3c-game.exe fehlt: erst task build");
    let exe = root.path().join("dist").join("k3c-game").join("k3c-game.exe");
    std::fs::create_dir_all(exe.parent().unwrap()).unwrap();
    std::fs::write(&exe, "x").unwrap();
    assert_eq!(c.start(&ctx, "game-release").await.unwrap().state, State::Running);
    c.stop_all().await;
}

/// Die Ausgabe landet in `logs/<dienst>.jsonl`, die Emojis unverändert.
#[tokio::test(flavor = "multi_thread")]
async fn prozess_dienst_ausgabe_im_log() {
    let root = tmp_dir();
    let mut opts = options(&Fake::new(false));
    opts.root = root.path().to_path_buf();
    let c = Controller::new(vec![game("game-dev")], opts);
    c.start(&Cancel::default(), "game-dev").await.unwrap();
    c.stop("game-dev", false).await.unwrap();
    wait_for("Ausgabe-Datei", || read_out(root.path(), "game-dev").len() == 1).await;
    let entries = read_out(root.path(), "game-dev");
    assert_eq!(entries[0]["msg"], "hallo von game-dev");
    assert_eq!(entries[0]["stream"], "stdout");
}

/// Vor einem Neubau-Task beendet die Workbench den laufenden Release-Dienst.
#[tokio::test(flavor = "multi_thread")]
async fn stop_for_task() {
    let f = Fake::new(false);
    let c = controller(&f, vec![game("game-release"), svc("Vite", false)]);
    assert!(c.stop_for_task("build").await.is_empty());
    c.start(&Cancel::default(), "game-release").await.unwrap();
    assert!(c.stop_for_task("test").await.is_empty(), "fremder Task stoppt");
    assert_eq!(c.stop_for_task("rust:game:build").await, ["game-release"]);
    assert_eq!(c.statuses()[0].state, State::Stopped);
    assert!(f.last().killed.load(Ordering::SeqCst));
}

/// Ein Programm mit Pfad liegt relativ zur Repo-Wurzel, ein Name ohne Pfad kommt aus `PATH`.
#[test]
fn programm_relativ_zur_wurzel() {
    let root = Path::new("C:").join("repo");
    let want = root.join("dist").join("k3c-game").join("k3c-game.exe");
    assert_eq!(program("dist/k3c-game/k3c-game.exe", &root), want);
    assert_eq!(program("task", &root), Path::new("task"));
}
