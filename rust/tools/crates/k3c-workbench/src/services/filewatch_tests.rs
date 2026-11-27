//! Port von `filewatch_test.go`.

use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::{Arc, PoisonError};
use std::time::Duration;

use k3c_kit::console::Store;

use super::config_tests::load_text;
use super::filewatch::{FILES_BATCH, changed, snapshot};
use super::testutil::{Fake, options, svc, wait_for};
use super::*;
use crate::testutil::tmp_dir;

fn write(root: &Path, rel: &str, text: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

fn go_files(paths: &[&str]) -> Watch {
    Watch {
        paths: paths.iter().map(|p| (*p).to_owned()).collect(),
        ext: vec![".go".to_owned()],
    }
}

/// Neue, geänderte und gelöschte Dateien zählen; Endungen, versteckte Ordner und `node_modules` nicht.
#[test]
fn snapshot_und_changed() {
    let dir = tmp_dir();
    let root = dir.path();
    let watch = go_files(&["engine", "go.mod"]);
    write(root, "engine/a.go", "package a");
    write(root, "engine/sub/b.go", "package b");
    write(root, "engine/notiz.txt", "ignoriert");
    write(root, "engine/.git/c.go", "versteckt");
    write(root, "engine/node_modules/d.go", "fremd");
    write(root, "go.mod", "module x"); // einzeln genannte Dateien zählen ohne Endungsprüfung
    let before = snapshot(root, &watch);
    let names: Vec<&str> = before.keys().map(String::as_str).collect();
    assert_eq!(names, ["engine/a.go", "engine/sub/b.go", "go.mod"]);
    let diff = || changed(&before, &snapshot(root, &watch));
    assert_eq!(diff(), None);
    write(root, "engine/notiz.txt", "anders");
    assert_eq!(diff(), None);
    write(root, "engine/a.go", "package a // neu, länger");
    assert_eq!(diff().as_deref(), Some("engine/a.go"));
    write(root, "engine/neu.go", "package neu");
    assert_eq!(diff().as_deref(), Some("engine/a.go"), "die erste in Namensreihenfolge");
    std::fs::remove_file(root.join("engine/sub/b.go")).unwrap();
    assert!(diff().is_some());
    let missing = go_files(&["gibt-es-nicht"]);
    assert!(snapshot(root, &missing).is_empty());
}

#[test]
fn watch_wird_geprueft() {
    let config = |watch: &str| {
        load_text(&format!(
            r#"[{{"name":"A","command":["x"],"port":1,"health":"http","watch":{{{watch}}}}}]"#
        ))
    };
    for bad in [
        r#""paths": []"#,
        r#""paths": [""]"#,
        r#""paths": ["/etc"]"#,
        r#""paths": [".."]"#,
        r#""paths": ["../x"]"#,
        r#""paths": ["a\\b"]"#,
        r#""paths": ["engine"], "ext": ["go"]"#,
        r#""paths": ["engine"], "ext": ["."]"#,
        r#""paths": ["engine"], "ext": [".a/b"]"#,
    ] {
        assert!(config(bad).is_err(), "{bad}: kein Fehler");
    }
    let list = config(r#""paths":["engine","go.mod"],"ext":[".go"]"#).unwrap();
    assert_eq!(list[0].watch.as_ref().map(|w| w.paths.len()), Some(2));
}

fn watch_controller(f: &Arc<Fake>, root: &Path, watch: Option<Watch>) -> (Arc<Controller>, Arc<Store>) {
    let mut opts = options(f);
    opts.root = root.to_path_buf();
    opts.watch_every = Duration::from_secs(3600);
    opts.files_every = Duration::from_millis(5);
    opts.files_quiet = Duration::from_millis(30);
    let store = Arc::clone(&opts.console);
    let service = Service {
        watch,
        ..svc("Server", false)
    };
    (Controller::new(vec![service], opts), store)
}

fn console_has(store: &Store, part: &str) -> bool {
    store
        .tail("Server", 50)
        .unwrap_or_default()
        .iter()
        .any(|l| l.text.contains(part))
}

async fn pause() {
    tokio::time::sleep(Duration::from_millis(120)).await;
}

/// Eine Änderung startet den laufenden Dienst neu, mehrere schnelle ergeben einen Neustart; Stop beendet das
/// Beobachten.
#[tokio::test(flavor = "multi_thread")]
async fn datei_aenderung_startet_neu() {
    let dir = tmp_dir();
    write(dir.path(), "engine/a.go", "package a");
    let f = Fake::new(true);
    let (c, store) = watch_controller(&f, dir.path(), Some(go_files(&["engine"])));
    c.start(&Cancel::default(), "Server").await.unwrap();
    let first = f.last();
    {
        // Der Watcher sieht beide Dateien zugleich, nie einen Zwischenstand.
        let _batch = FILES_BATCH.write().unwrap_or_else(PoisonError::into_inner);
        write(dir.path(), "engine/a.go", "package a // 1");
        write(dir.path(), "engine/b.go", "package b");
    }
    wait_for("Neustart", || f.count() >= 2 && c.statuses()[0].state == State::Running).await;
    assert!(first.killed.load(Ordering::SeqCst), "der alte Prozess läuft noch");
    pause().await;
    assert_eq!(f.count(), 2, "mehrere Änderungen auf einmal");
    assert!(console_has(&store, "Neustart wegen Änderung an engine/a.go"));
    let st = c.statuses().remove(0);
    assert_eq!(st.change_file, "engine/a.go");
    assert!(st.change_at.is_some());
    write(dir.path(), "engine/notiz.txt", "keine Go-Datei");
    pause().await;
    assert_eq!(f.count(), 2, "andere Endung");
    c.stop("Server", false).await.unwrap();
    write(dir.path(), "engine/a.go", "package a // nach dem Stop");
    pause().await;
    assert_eq!((f.count(), c.statuses()[0].state), (2, State::Stopped));
}

/// Scheitert ein Neustart (Build-Fehler), startet die nächste Änderung den Dienst wieder.
#[tokio::test(flavor = "multi_thread")]
async fn fehlgeschlagener_neustart_wird_wiederholt() {
    let dir = tmp_dir();
    write(dir.path(), "engine/a.go", "package a");
    let f = Fake::new(true);
    let (c, store) = watch_controller(&f, dir.path(), Some(go_files(&["engine"])));
    c.start(&Cancel::default(), "Server").await.unwrap();
    f.healthy.store(false, Ordering::SeqCst);
    write(dir.path(), "engine/a.go", "package a // kaputt");
    wait_for("fehlgeschlagen", || c.statuses()[0].state == State::Failed).await;
    wait_for("Meldung", || console_has(&store, "Neustart fehlgeschlagen")).await;
    f.healthy.store(true, Ordering::SeqCst);
    write(dir.path(), "engine/a.go", "package a // repariert");
    wait_for("läuft wieder", || c.statuses()[0].state == State::Running).await;
    c.stop_all().await;
}

#[tokio::test(flavor = "multi_thread")]
async fn dienst_ohne_watch_wird_nicht_beobachtet() {
    let dir = tmp_dir();
    write(dir.path(), "engine/a.go", "package a");
    let f = Fake::new(true);
    let (c, _) = watch_controller(&f, dir.path(), None);
    c.start(&Cancel::default(), "Server").await.unwrap();
    write(dir.path(), "engine/a.go", "package a // neu");
    pause().await;
    assert_eq!(f.count(), 1);
    c.stop_all().await;
}
