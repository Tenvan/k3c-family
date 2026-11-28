//! Testmuster (B-866). Ein Test startet wirklich `task`, nur in einem Wegwerf-Ordner unter `.work/tmp`; fehlt das
//! Werkzeug, überspringt er.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::run::{RunSpec, run_process};
use super::targets::find_target;
use crate::testutil::tmp_dir;

const PATTERNS: [&str; 3] = ["TestA|TestB", "^TestA$", "x$(exit)y"];

#[test]
fn muster_mit_oder_und_ankern() {
    let test = find_target("task:test").unwrap();
    for bad in ["TestA;rm", "TestA&TestB", "TestA`x`", r"TestA\x"] {
        let error = test.args(bad).unwrap_err();
        assert!(error.contains("| ^ $ ( )"), "{bad:?}: {error}");
    }
    for pattern in PATTERNS {
        // `CLI_ARGS` nach `--` quotet Task selbst: das Muster bleibt roh.
        assert_eq!(test.args(pattern).unwrap(), ["task", "test", "--", pattern]);
    }
}

/// Startet `args` in `dir` und liefert Exit-Code und Ausgabe.
async fn output(dir: &std::path::Path, args: Vec<String>, env: &[(&str, &str)]) -> (i64, String) {
    let lines = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&lines);
    let res = run_process(RunSpec {
        dir: dir.to_path_buf(),
        args,
        timeout: Duration::from_secs(120),
        out: Arc::new(move |_: &str, text: &str| seen.lock().unwrap().push(text.to_owned())),
        env: env.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect(),
    })
    .await;
    assert_eq!(res.err, None);
    let joined = lines.lock().unwrap().join("\n");
    (res.exit, joined)
}

/// Liegt das Werkzeug im `PATH`? Sonst meldet der Test, dass er überspringt.
fn installed(tool: &str) -> bool {
    let file = format!("{tool}{}", std::env::consts::EXE_SUFFIX);
    let path = std::env::var_os("PATH").unwrap_or_default();
    let found = std::env::split_paths(&path).any(|dir| dir.join(&file).is_file());
    if !found {
        eprintln!("übersprungen: {tool} fehlt");
    }
    found
}

#[tokio::test(flavor = "multi_thread")]
async fn muster_nach_doppelstrich_bleibt_woertlich() {
    if !installed("task") {
        return;
    }
    let dir = tmp_dir();
    let taskfile = dir.path().join("Taskfile.yml");
    let content = "version: '3'\ntasks:\n  zeig:\n    cmds:\n      - echo {{.CLI_ARGS}}\n";
    std::fs::write(&taskfile, content).unwrap();
    let test = find_target("task:test").unwrap();
    for pattern in PATTERNS {
        // Ab `--` wie in der echten Befehlszeile, davor der Probe-Task.
        let taskfile = taskfile.display().to_string();
        let mut args = ["task", "-s", "-t", &taskfile, "zeig"].map(str::to_owned).to_vec();
        args.extend(test.args(pattern).unwrap().split_off(2));
        let (exit, out) = output(dir.path(), args, &[]).await;
        assert_eq!((exit, out.trim()), (0, pattern), "{pattern}");
    }
}
