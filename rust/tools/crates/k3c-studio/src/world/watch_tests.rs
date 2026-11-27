use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use super::{Changed, changed_ids, snapshot, watch};
use crate::test_helpers::tmp_dir;

fn write_file(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

// B-1216/AC-01: Eine geänderte Welt-Datei nennt ihre ID, eine Basis-Datei nennt keine ID (alle laden neu).
#[test]
fn geaenderte_ids() {
    let dir = tmp_dir();
    let root = dir.path();
    let worlds = root.join("data").join("worlds");
    let buildings = root.join("data").join("buildings.json");
    write_file(&worlds.join("standard.json"), "{}");
    write_file(&worlds.join("nil.json"), "{}");
    write_file(&buildings, "{}");
    let before = snapshot(root);
    write_file(&worlds.join("nil.json"), r#"{"id":"nil"}"#);
    write_file(&worlds.join("neu.json"), "{}");
    let (ids, all) = changed_ids(&before, &snapshot(root));
    assert_eq!((ids, all), (vec!["neu".to_owned(), "nil".to_owned()], false));

    let mid = snapshot(root);
    write_file(&buildings, r#"{"a":1}"#);
    assert_eq!(changed_ids(&mid, &snapshot(root)), (Vec::new(), true), "Basis-Datei");
    assert_eq!(changed_ids(&mid, &mid), (Vec::new(), false), "ohne Änderung");

    let now = snapshot(root);
    std::fs::remove_file(worlds.join("neu.json")).unwrap();
    assert_eq!(changed_ids(&now, &snapshot(root)).0, ["neu"], "gelöscht");
}

#[test]
fn waechter_meldet_aenderung() {
    let dir = tmp_dir();
    let file = dir.path().join("data").join("worlds").join("standard.json");
    write_file(&file, "{}");
    let (send, got) = mpsc::channel();
    let watcher = watch(dir.path(), Duration::from_millis(10), move |ids| {
        let _ = send.send(ids);
    });
    write_file(&file, r#"{"id":"standard","name":"S"}"#);
    let ids = got.recv_timeout(Duration::from_secs(3)).expect("keine Meldung");
    assert_eq!(ids, ["standard"]);
    drop(watcher);
    // Der Wächter endet: der Sender des Rückrufs wird mit dem Thread freigegeben.
    while got.recv_timeout(Duration::from_secs(3)).is_ok() {}
    assert_eq!(got.try_recv(), Err(mpsc::TryRecvError::Disconnected));
}

#[test]
fn nutzlast_wie_go() {
    let empty = serde_json::to_string(&Changed::default()).unwrap();
    assert_eq!(empty, "{}");
    let one = Changed {
        ids: vec!["nil".to_owned()],
    };
    assert_eq!(serde_json::to_string(&one).unwrap(), r#"{"ids":["nil"]}"#);
}
