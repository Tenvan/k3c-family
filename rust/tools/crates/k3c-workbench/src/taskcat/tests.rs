//! Port von `taskcat_test.go`, dazu ein Lauf von `load` mit eigener Taskfile in einem Ordner unter `.work/` (nur
//! `--list-all`, kein Task läuft und nichts im Repo).

use std::path::Path;
use std::time::Duration;

use super::*;
use crate::testutil::tmp_dir;

const SAMPLE: &str = r#"{"tasks":[
 {"name":"check","desc":"alles prüfen","aliases":null,"location":{"line":3,"taskfile":"/r/Taskfile.yml"}},
 {"name":"dev:build","desc":"Frontend bauen","aliases":["db"],"location":{"line":9,"taskfile":"/elsewhere/T.yml"}},
 {"name":"dev:test","desc":"","location":{"line":1,"taskfile":"/r/Taskfile.yml"}}]}"#;

#[test]
fn parse_group_filter() {
    let tasks = parse(SAMPLE.as_bytes(), Path::new("/r")).unwrap();
    assert_eq!(tasks[0].namespace, WORKSPACE);
    assert_eq!((tasks[1].namespace.as_str(), tasks[1].leaf.as_str()), ("dev", "build"));
    assert!(tasks[0].aliases.is_empty());
    assert_eq!(tasks[0].file, "Taskfile.yml");
    assert_eq!(tasks[1].file, "/elsewhere/T.yml");
    let groups = group(&tasks);
    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0].name, WORKSPACE);
    assert_eq!(groups[1].tasks[0].name, "dev:build");
    let hits = filter(tasks, " FRONTEND ");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].name, "dev:build");
}

#[test]
fn workspace_steht_vor_grossbuchstaben() {
    let task = |namespace: &str| Task {
        namespace: namespace.to_owned(),
        ..Task::default()
    };
    let names: Vec<String> = group(&[task("a"), task(WORKSPACE), task("B")])
        .into_iter()
        .map(|g| g.name)
        .collect();
    assert_eq!(names, [WORKSPACE, "B", "a"]);
    assert!(group(&[]).is_empty());
}

#[test]
fn unlesbare_ausgabe() {
    let error = parse(b"kein json", Path::new("/r")).unwrap_err();
    assert!(
        error.starts_with("task --list-all --json --no-status: Ausgabe nicht lesbar: "),
        "{error}"
    );
}

#[tokio::test]
async fn load_liest_eine_eigene_taskfile() {
    let tmp = tmp_dir();
    let file = "version: '3'\ntasks:\n  hallo:\n    desc: sagt hallo\n    cmds: ['echo hallo']\n";
    std::fs::write(tmp.path().join("Taskfile.yml"), file).unwrap();
    let tasks = match load(tmp.path(), Duration::from_secs(20)).await {
        Ok(tasks) => tasks,
        Err(e) if e.starts_with("\"task\" nicht im PATH") => {
            eprintln!("🚫 übersprungen: {e}");
            return;
        }
        Err(e) => panic!("{e}"),
    };
    assert_eq!(tasks.len(), 1, "{tasks:?}");
    assert_eq!(
        (tasks[0].name.as_str(), tasks[0].desc.as_str()),
        ("hallo", "sagt hallo")
    );
    assert_eq!(tasks[0].file, "Taskfile.yml");
}
