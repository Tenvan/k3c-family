//! Port der Verdichtungs-Tests aus `check_test.go` mit denselben Ausgaben echter Läufe (`tests/data/check/`).

use std::path::PathBuf;
use std::time::Duration;

use super::compact::{MAX_ERROR_LINES, all_patterns_compile, report};
use super::run::RunResult;
use super::targets::find_target;

fn sample(name: &str) -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/check")
        .join(format!("{name}.txt"));
    let text = std::fs::read_to_string(path).unwrap();
    text.replace("\r\n", "\n").split('\n').map(str::to_owned).collect()
}

fn failed(exit: i64, dur: Duration) -> RunResult {
    RunResult {
        exit,
        dur,
        ..RunResult::default()
    }
}

/// Ziel, Beispiel, erwartete Zeilen, Text, der fehlen muss.
const CASES: &[(&str, &str, &[&str], &str)] = &[
    (
        "task:test",
        "vitest",
        &[
            "FAIL  tests/zz-tmp-fail.test.ts > tmp > rechnet falsch",
            "AssertionError: expected 2 to be 3 // Object.is equality",
            "❯ tests/zz-tmp-fail.test.ts:4:19",
            "Test Files  1 failed (1)",
            "Tests  1 failed | 1 passed (2)",
        ],
        "Start at",
    ),
    (
        "task:typecheck",
        "tsc",
        &["src/zzTmpFail.ts(1,14): error TS2322: Type 'string' is not assignable to type 'number'."],
        "",
    ),
    (
        "task:lint",
        "oxlint",
        &[
            "src/zzTmpFail.ts:2:55: error eslint(max-depth): Blocks are nested too deeply (5). Maximum allowed is 4. help: Consider refactoring your code.",
        ],
        "warning",
    ),
    (
        "task:check:diff",
        "gotest",
        &[
            "--- FAIL: TestZZFalsch (0.00s)",
            "zz_tmp_test.go:7: 1+1 = 2, erwartet 3",
            "FAIL\tk3c/tools/k3c-workbench\t0.907s",
        ],
        "ok ",
    ),
    (
        "task:check:diff",
        "golangci",
        &[
            "zz_tmp_test.go:11:35: Error return value is not checked (errcheck)",
            "zz_tmp_test.go:13:6: func tmpErr is unused (unused)",
            "2 issues:",
        ],
        "func zz",
    ),
    (
        "task:check:dev",
        "vite",
        &[
            "error during build:",
            "Build failed with 1 error:",
            "[plugin vite:build-html] C:/WORKSPACE/k3c/game.html",
            "Error: Failed to resolve /src/gibtsnicht.ts from C:/WORKSPACE/k3c/game.html",
        ],
        "at async",
    ),
];

#[test]
fn verdichtung_echter_laeufe() {
    assert!(all_patterns_compile());
    for (target, file, want, not) in CASES {
        let text = report(
            find_target(target).unwrap(),
            &failed(1, Duration::from_secs(1)),
            &sample(file),
        );
        let want = format!("{target} · exit 1 · 1,0 s\n{}", want.join("\n"));
        assert_eq!(text, want, "{file}");
        if !not.is_empty() {
            assert!(!text.contains(not), "{file} enthält {not:?}");
        }
    }
}

#[test]
fn verdichtung_ohne_muster_und_deckel() {
    let target = find_target("task:test").unwrap();
    let lines: Vec<String> = (0..30).map(|i| format!("Zeile {i}")).collect();
    let text = report(target, &failed(2, Duration::ZERO), &lines);
    let got: Vec<&str> = text.split('\n').collect();
    assert_eq!((got.len(), got[1]), (21, "Zeile 10"), "{text}");

    let lines: Vec<String> = (0..70).map(|i| format!(" FAIL  test {i}")).collect();
    let text = report(target, &failed(1, Duration::ZERO), &lines);
    let got: Vec<&str> = text.split('\n').collect();
    assert_eq!((got.len(), got[got.len() - 1]), (MAX_ERROR_LINES + 2, "… 10 weitere"));

    let res = RunResult {
        timed_out: true,
        ..failed(-1, Duration::from_secs(300))
    };
    assert_eq!(
        report(target, &res, &[]),
        "task:test · Zeitlimit 5 min 0 s überschritten · 5 min 0 s"
    );
}
