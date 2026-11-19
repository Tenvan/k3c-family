//! B-1116/AC-02: Der Bericht der Wiedergabe trägt `bilder` aus dem `FrameLog` des Spiels.

use super::*;

#[test]
fn bericht_traegt_bilder_aus_dem_game_log() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let lines = trace::read_lines(root.join("testdata/rust/trace/nacht-300.jsonl.gz")).unwrap();
    let t = trace::parse(&lines).unwrap();
    let dir = root.join(".work/tmp/replay-bilder");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("bilder.jsonl");
    let log = FrameLog::new(&path.to_string_lossy(), 1.0);
    let mut app = Metered::new(ReplayApp::with_config(t, None), 2, 20).with_frames(log);
    let (input, mut canvas) = (InputState::default(), NullCanvas::default());
    while !app.done() {
        app.step(&input, STEP);
        app.draw(&mut canvas, 0.5);
    }
    let mut r = app.report("headless");
    assert!(r.bilder.is_none(), "vor dem Schreiben keine Kennwerte");
    write_frames(&app, &mut r).unwrap();
    let b = r.bilder.expect("bilder");
    assert!(b.bilder > 0, "Bilder {}", b.bilder);
    assert!(b.datei.ends_with("bilder.jsonl"), "Datei {}", b.datei);
    assert!(path.exists());
    let _ = std::fs::remove_dir_all(&dir);
}
