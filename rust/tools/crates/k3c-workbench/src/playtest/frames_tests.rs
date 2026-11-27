//! `TestFrameArgs` und `TestCaptureStart` aus `transcribe_test.go`; für `FrameAt` hat Go keinen eigenen Test hier.

use std::sync::{Arc, Mutex};

use super::*;
use crate::playtest::Proc;
use crate::playtest::rig_tests::put;
use crate::testutil::tmp_dir;

#[test]
fn frame_args_sucht_vor_der_eingabe() {
    assert_eq!(
        frame_args("ffmpeg", 5.5, "frames/marker-1.jpg").join(" "),
        "ffmpeg -hide_banner -loglevel error -y -ss 5.500 -i video.mkv -frames:v 1 -q:v 3 frames/marker-1.jpg"
    );
}

#[test]
fn capture_start_mit_und_ohne_capture_ms() {
    let meta = |capture_ms, start_ms| Meta {
        capture_ms,
        start_ms,
        ..Meta::default()
    };
    assert_eq!(capture_start(&meta(5, 9000)), 5);
    assert_eq!(capture_start(&meta(0, 9000)), 7000);
    assert_eq!(capture_start(&meta(0, 0)), 0);
}

/// Attrappe für ffmpeg: schreibt die Zieldatei (oder nicht) und ist schon fertig.
struct Done(Result<(), String>);

impl Proc for Done {
    fn wait(&self) -> Result<(), String> {
        self.0.clone()
    }
    fn quit(&self) {}
    fn kill(&self) {}
    fn tail(&self) -> String {
        "Fehlerzeile".to_owned()
    }
}

#[test]
fn frame_at_erzeugt_einmal_und_meldet_fehler() {
    let dir = tmp_dir();
    put(
        dir.path(),
        "2026-11-22-1/session.json",
        r#"{"captureMs":1000,"stand":"aufgenommen"}"#,
    );
    put(dir.path(), "2026-11-22-1/video.mkv", "mkv");
    put(dir.path(), "2026-11-22-2/session.json", "{}");
    let store = Store {
        root: dir.path().to_path_buf(),
        active: None,
    };
    let found: LookPath = Arc::new(|_| Ok("ffmpeg".to_owned()));
    let calls = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&calls);
    let writes: Spawn = Arc::new(move |args, cwd| {
        seen.lock().unwrap().push(args.join(" "));
        std::fs::write(cwd.join(args.last().unwrap()), "jpg").unwrap();
        Ok(Arc::new(Done(Ok(()))) as Arc<dyn Proc>)
    });
    let out = frame_at(&store, &found, &writes, "2026-11-22-1", 3500).unwrap();
    assert!(
        out.ends_with(std::path::Path::new("frames").join("t-3500.jpg")),
        "{}",
        out.display()
    );
    frame_at(&store, &found, &writes, "2026-11-22-1", 3500).unwrap();
    // Vor dem Start der Aufnahme gilt 0 s.
    frame_at(&store, &found, &writes, "2026-11-22-1", 10).unwrap();
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 2, "vorhandenes Bild wird wiederverwendet");
    assert!(
        calls[0].contains("-ss 2.500") && calls[1].contains("-ss 0.000"),
        "{calls:?}"
    );
    let fails: Spawn = Arc::new(|_, _| Ok(Arc::new(Done(Err("exit status 1".to_owned()))) as Arc<dyn Proc>));
    let error = frame_at(&store, &found, &fails, "2026-11-22-1", 4000)
        .unwrap_err()
        .to_string();
    assert_eq!(error, "Bild nicht erzeugt: exit status 1: Fehlerzeile");
    assert_eq!(
        frame_at(&store, &found, &fails, "2026-11-22-2", 1).unwrap_err(),
        Error::NoVideo
    );
    assert_eq!(frame_at(&store, &found, &fails, "../x", 1).unwrap_err(), Error::BadId);
}
