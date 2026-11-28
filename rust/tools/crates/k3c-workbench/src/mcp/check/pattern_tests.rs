//! Port von `check_pattern_test.go` (B-866). Zwei Tests starten wie in Go wirklich `go test` bzw. `task`, nur in
//! Wegwerf-Ordnern unter `.work/tmp`; fehlt das Werkzeug, überspringen sie.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::run::{RunSpec, run_process};
use super::targets::find_target;
use crate::testutil::tmp_dir;

const PATTERNS: [&str; 3] = ["TestA|TestB", "^TestA$", "x$(exit)y"];

#[test]
fn muster_mit_oder_und_ankern() {
    let go = find_target("go:test").unwrap();
    for bad in ["TestA;rm", "TestA&TestB", "TestA`x`", r"TestA\x"] {
        let error = go.args(bad).unwrap_err();
        assert!(error.contains("| ^ $ ( )"), "{bad:?}: {error}");
    }
    let dev = find_target("dev:test").unwrap();
    for pattern in PATTERNS {
        // `go test` startet ohne Shell: das Muster bleibt roh. Die Task-Variable braucht die Anführungszeichen.
        assert_eq!(go.args(pattern).unwrap(), ["go", "test", "./...", "-run", pattern]);
        let last = dev.args(pattern).unwrap().pop().unwrap();
        assert_eq!(last, format!("GO_TEST_FLAGS=-run='{pattern}'"));
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
async fn go_nimmt_die_muster() {
    if !installed("go") {
        return;
    }
    let dir = tmp_dir();
    // Go ignoriert ein `go.mod` im Temp-Ordner des Systems selbst, darum liegen Modul und Temp nebeneinander.
    let (module, temp) = (dir.path().join("modul"), dir.path().join("tmp"));
    std::fs::create_dir_all(&module).unwrap();
    std::fs::create_dir_all(&temp).unwrap();
    std::fs::write(module.join("go.mod"), "module muster\n\ngo 1.21\n").unwrap();
    let tests = "package muster\n\nimport \"testing\"\n\nfunc TestA(*testing.T)  {}\nfunc TestAB(*testing.T) {}\n\
                 func TestB(*testing.T)  {}\nfunc TestC(*testing.T)  {}\n";
    std::fs::write(module.join("muster_test.go"), tests).unwrap();
    let go = find_target("go:test").unwrap();
    let temp = temp.display().to_string();
    for (pattern, want) in [("TestA|TestB", "TestA,TestAB,TestB"), ("^TestA$", "TestA")] {
        let mut args = go.args(pattern).unwrap();
        args.extend(["-v".to_owned(), "-count=1".to_owned()]);
        let env = [
            ("GOWORK", "off"),
            ("GOFLAGS", ""),
            ("GOTMPDIR", temp.as_str()),
            ("TMP", temp.as_str()),
            ("TEMP", temp.as_str()),
        ];
        let (exit, out) = output(&module, args, &env).await;
        assert_eq!(exit, 0, "{pattern}: {out}");
        let ran: Vec<&str> = out
            .lines()
            .filter_map(|l| l.trim().strip_prefix("=== RUN   "))
            .collect();
        assert_eq!(ran.join(","), want, "{pattern}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn task_variable_bleibt_woertlich() {
    if !installed("task") {
        return;
    }
    let dir = tmp_dir();
    let taskfile = dir.path().join("Taskfile.yml");
    let content = "version: '3'\ntasks:\n  zeig:\n    cmds:\n      - echo [{{.GO_TEST_FLAGS}}]\n";
    std::fs::write(&taskfile, content).unwrap();
    let dev = find_target("dev:test").unwrap();
    for pattern in PATTERNS {
        let variable = dev.args(pattern).unwrap().pop().unwrap();
        let taskfile = taskfile.display().to_string();
        let args = ["task", "-s", "-t", &taskfile, "zeig", &variable]
            .map(str::to_owned)
            .to_vec();
        let (exit, out) = output(dir.path(), args, &[]).await;
        assert_eq!(
            (exit, out.trim()),
            (0, format!("[-run={pattern}]").as_str()),
            "{pattern}"
        );
    }
}
