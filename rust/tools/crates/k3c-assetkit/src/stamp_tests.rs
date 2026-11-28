//! Port von `stamp_test.go`.

use std::fs::File;
use std::sync::mpsc::{Receiver, channel};

use super::*;
use crate::register::set_choices;
use crate::testutil::{repo, tempdir, write_file};

fn well() -> Vec<Choice> {
    vec![Choice {
        file: "grafik/town/well.png".to_string(),
        ..Choice::default()
    }]
}

#[test]
fn test_find_root() {
    let dir = tempdir();
    let root = dir.path();
    mark_root(root);
    let sub = root.join("rust").join("tools");
    fs::create_dir_all(&sub).unwrap();
    assert_eq!(find_root(&sub).unwrap(), root);
    // Ein Worktree darunter ist seine eigene Wurzel.
    let wt = root.join(".claude").join("worktrees").join("wt1");
    mark_root(&wt);
    let deep = wt.join("tools").join("k3c-studio").join("frontend");
    fs::create_dir_all(&deep).unwrap();
    assert_eq!(find_root(&deep).unwrap(), wt);
    // Das Spielpaket hat nur data/ und ist keine Wurzel. Einen Fall ganz ohne Wurzel prüft der Test nicht: der
    // Temp-Ordner liegt im Repo; is_repo_root deckt den Negativfall ab.
    let package = root.join("dist").join("k3c-game");
    fs::create_dir_all(package.join("data")).unwrap();
    assert!(!is_repo_root(&package));
    assert_eq!(find_root(&package).unwrap(), root);
}

/// Macht `dir` zu einer Repo-Wurzel für `find_root`: Datei `rust/Cargo.toml` und Ordner `data/`.
fn mark_root(dir: &Path) {
    write_file(&dir.join("rust").join("Cargo.toml"), "[workspace]\n");
    fs::create_dir_all(dir.join("data")).unwrap();
}

/// B-459/AC-07: Speichern auf veraltetem Stand wird abgelehnt und schreibt nichts; auf aktuellem Stand gibt es den
/// neuen Stand zurück, mit dem das nächste Speichern gelingt.
#[test]
fn set_choices_if_schreibregel() {
    let dir = repo();
    let root = dir.path();
    let stand = stamp(root).unwrap();
    let (slot, next) = set_choices_if(root, "farm", &well(), &stand).unwrap();
    assert!(
        slot.choices().len() == 1 && next != stand && !next.is_empty(),
        "{slot:?} {next}"
    );
    // Ein anderer Prozess (MCP) schreibt dazwischen.
    let wagon = Choice {
        file: "grafik/town/wagon.png".to_string(),
        ..Choice::default()
    };
    set_choices(root, "farm", &[wagon]).unwrap();
    let before = fs::read(root.join(REGISTER_PATH)).unwrap();
    let err = set_choices_if(root, "farm", &well(), &next).unwrap_err();
    assert!(matches!(err, Error::Stale), "{err}");
    assert_eq!(
        fs::read(root.join(REGISTER_PATH)).unwrap(),
        before,
        "abgelehntes Speichern hat geschrieben"
    );
    assert_eq!(Error::Stale.to_string(), "Register geändert, bitte erneut speichern");
}

fn watched(root: &Path) -> (WatchHandle, Receiver<()>) {
    let (send, receive) = channel();
    let handle = watch(root, Duration::from_millis(10), move || {
        let _ = send.send(());
    });
    (handle, receive)
}

/// B-459/AC-07: Eine Änderung durch einen anderen Schreiber löst `on_change` aus, ein Stand ohne Änderung nicht.
#[test]
fn watch_meldet_aenderung() {
    let dir = repo();
    let (_handle, changed) = watched(dir.path());
    assert!(
        changed.recv_timeout(Duration::from_millis(60)).is_err(),
        "on_change ohne Änderung"
    );
    set_choices(dir.path(), "farm", &well()).unwrap();
    changed
        .recv_timeout(Duration::from_secs(2))
        .expect("keine Meldung nach Änderung");
}

/// Gleicher Inhalt mit neuer Änderungszeit ist keine Änderung (Hash entscheidet).
#[test]
fn watch_gleicher_inhalt() {
    let dir = repo();
    let (_handle, changed) = watched(dir.path());
    let later = SystemTime::now() + Duration::from_secs(60);
    let file = File::options()
        .write(true)
        .open(dir.path().join(REGISTER_PATH))
        .unwrap();
    file.set_modified(later).unwrap();
    drop(file);
    assert!(
        changed.recv_timeout(Duration::from_millis(80)).is_err(),
        "on_change bei gleichem Inhalt"
    );
}

/// Eigener Test: nach `stop` meldet nichts mehr, der Thread gibt `on_change` frei.
#[test]
fn watch_endet_mit_stop() {
    let dir = repo();
    let (handle, changed) = watched(dir.path());
    handle.stop();
    let gone = changed.recv_timeout(Duration::from_secs(2));
    assert_eq!(gone, Err(std::sync::mpsc::RecvTimeoutError::Disconnected));
}
