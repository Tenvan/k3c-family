//! RU38.3 AC-03: `replay --headless` liefert zweimal denselben Prüfwert; der Bericht hat alle Felder.

use std::path::PathBuf;

use k3c_game::replay::cli::{Mode, parse_args};
use k3c_game::replay::run::{run, run_headless};
use k3c_game::replay::trace::read_lines;

fn file() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../testdata/rust/trace/nacht-300.jsonl.gz")
}

#[test]
fn replay_headless_zweimal_derselbe_pruefwert() {
    let lines = read_lines(file()).unwrap();
    let a = run_headless(&lines, 300).unwrap();
    let b = run_headless(&lines, 300).unwrap();
    assert_eq!(a.checksum, b.checksum);
    assert!(a.steps > 0);
    assert_ne!(a.checksum, 0xcbf2_9ce4_8422_2325);
}

#[test]
fn replay_headless_bericht_nach_out() {
    let out = std::env::temp_dir().join("k3c-replay-test-bericht.json");
    let o = parse_args(&[
        file().to_string_lossy().into_owned(),
        "--headless".into(),
        "--measure".into(),
        "100".into(),
        "--out".into(),
        out.to_string_lossy().into_owned(),
    ])
    .unwrap();
    assert_eq!(o.mode, Mode::Headless);
    run(&o).unwrap();
    let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&out).unwrap()).unwrap();
    assert_eq!(v["takte"], 100);
    assert_eq!(v["backend"], "headless");
    let _ = std::fs::remove_file(out);
}
