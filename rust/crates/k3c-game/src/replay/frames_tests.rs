use std::path::Path;

use super::*;

/// Beispielzeile aus `native/k3c-game/game/measure_frames.go` (Zeile 153) mit eingesetzten Werten.
const GO_LINE: &str = r#"{"t":1.250000,"dt_ms":16.6667,"draw_ms":2.5000,"takte":1,"alpha":0.5000,"gc":0}"#;

fn sample() -> Sample {
    Sample {
        t: 1.25,
        dt: 16.666_666,
        draw: 2.5,
        ticks: 1,
        alpha: 0.5,
    }
}

fn keys(l: &str) -> Vec<String> {
    let v: serde_json::Value = serde_json::from_str(l).unwrap();
    let serde_json::Value::Object(_) = v else {
        panic!("kein Objekt")
    };
    // Reihenfolge aus dem Text (serde_json sortiert ohne preserve_order)
    l.trim_matches(['{', '}'])
        .split(',')
        .map(|kv| kv.split(':').next().unwrap().trim_matches('"').to_string())
        .collect()
}

#[test]
fn zeile_wie_go() {
    let l = line(&sample());
    assert_eq!(l, GO_LINE);
    assert_eq!(keys(&l), keys(GO_LINE));
    assert_eq!(keys(&l), ["t", "dt_ms", "draw_ms", "takte", "alpha", "gc"]);
}

#[test]
fn puffer_ohne_allokation_in_der_messung() {
    let mut log = FrameLog::new(Path::new("x.jsonl"), 2.0);
    let cap = log.buf.capacity();
    assert!(cap >= 3 * FRAME_CAP, "Kapazität {cap}");
    for _ in 0..cap {
        log.add(sample());
    }
    assert_eq!((log.buf.len(), log.buf.capacity(), log.dropped), (cap, cap, 0));
    log.add(sample());
    assert_eq!((log.buf.len(), log.buf.capacity(), log.dropped), (cap, cap, 1));
}

#[test]
fn puffer_hat_obergrenze() {
    let log = FrameLog::new(Path::new("x.jsonl"), 1e9);
    assert!(log.buf.capacity() >= FRAME_CAP_MAX && log.buf.capacity() < FRAME_CAP_MAX * 2);
}

#[test]
fn schreibt_je_bild_eine_zeile() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/tmp").join(format!("k3c-frames-{}.jsonl", std::process::id()));
    let mut log = FrameLog::new(&path, 0.0);
    log.add(sample());
    log.add(sample());
    log.write().unwrap();
    let text = std::fs::read_to_string(&path).unwrap();
    assert_eq!(text, format!("{GO_LINE}\n{GO_LINE}\n"));
    let _ = std::fs::remove_file(path);
}
