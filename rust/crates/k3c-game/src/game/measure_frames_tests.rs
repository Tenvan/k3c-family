//! Port von `native/k3c-game/game/measure_frames_test.go` (`TestFrameReport`, `TestFrameLog`). Die Datei entsteht im
//! Ordner `.work/measure-frames-test/` des Repos (ignoriert) und wird danach entfernt.

use std::path::Path;

use super::*;

/// Go: `TestFrameReport`: Kennwerte der Bilder an einer festen Folge.
#[test]
fn frame_report_rechnet_die_kennwerte() {
    let mut buf: Vec<FrameSample> = (0..96)
        .map(|i| FrameSample {
            t: f64::from(i) / 60.0,
            dt: 16.7,
            ticks: (i % 2) as u32,
            ..FrameSample::default()
        })
        .collect();
    let s = |dt, ticks, gc| FrameSample {
        dt,
        ticks,
        gc,
        ..FrameSample::default()
    };
    buf.extend([
        s(40.0, 2, 1), // lang, GC, zwei Takte
        s(30.0, 0, 0), // lang, ohne Takt
        s(26.0, 1, 0), // lang, sonst nichts
        s(16.7, 1, 2),
    ]);
    let r = frame_report(&buf);
    assert!(r.bilder == 100 && r.median_ms == 16.7 && r.max_ms == 40.0, "{r:?}");
    assert_eq!(
        (r.lang, r.lang_mit_gc, r.lang_ohne_takt, r.lang_mehr_takt),
        (3, 1, 1, 1)
    );
    assert_eq!((r.mit_gc, r.gc_laeufe, r.ohne_takt, r.mehr_takt), (2, 3, 49, 1));
    let leer = frame_report(&[]);
    assert!(leer.bilder == 0 && leer.lang == 0, "leer: {leer:?}");
}

/// Go: `TestFrameLog`: ein voller Puffer verwirft, je Bild entsteht eine Zeile; ohne Datei keine Messung je Bild.
#[test]
fn frame_log_verwirft_bei_vollem_puffer_und_schreibt_zeilen() {
    assert!(
        FrameLog::new("", 5.0).is_none(),
        "ohne Datei gibt es keine Messung je Bild"
    );
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/measure-frames-test");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bilder.jsonl").to_string_lossy().into_owned();
    let mut l = FrameLog::new(&path, 0.0).unwrap();
    l.cap = 2;
    l.start();
    for i in 0..3 {
        l.add(f64::from(i), 16.7, 2.0, 1, 0.5);
    }
    let (r, res) = l.report();
    let text = std::fs::read_to_string(&path);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(res.is_ok(), "{res:?}");
    assert!(r.bilder == 2 && r.verworfen == 1 && r.datei == path, "{r:?}");
    let text = text.unwrap();
    let lines: Vec<&str> = text.trim().split('\n').collect();
    assert_eq!(lines.len(), 2, "Datei: {text:?}");
    assert!(lines[0].contains("\"alpha\":0.5000"), "{text:?}");
    assert!(lines[1].contains("\"takte\":1"), "{text:?}");
    assert_eq!(
        lines[0],
        "{\"t\":0.000000,\"dt_ms\":16.7000,\"draw_ms\":2.0000,\"takte\":1,\"alpha\":0.5000,\"gc\":0}"
    );
}

/// Der Puffer reserviert Platz für `seconds` Sekunden und wächst im Lauf nicht (Obergrenze `FRAME_CAP_MAX`).
#[test]
fn frame_log_reserviert_vorab_und_waechst_nicht() {
    let l = FrameLog::new("x.jsonl", 20.0).unwrap();
    assert_eq!(l.cap, 21 * FRAME_CAP);
    assert!(l.buf.capacity() >= l.cap);
    let big = FrameLog::new("x.jsonl", 1e9).unwrap();
    assert_eq!(big.cap, FRAME_CAP_MAX);
}
