//! Port von `transcribe_auswertung_test.go`, `transcribe_frames_test.go` und dem `Start`-Teil von
//! `TestCheckModellFehlt`.

use std::sync::atomic::Ordering;

use super::*;
use crate::playtest::eval_rig_tests::{EvalRig, SESSION};
use crate::playtest::rig_tests::{put, until};

fn start(transcriber: &Arc<Transcriber>, model: &str, language: &str) -> Result<JobStatus, Error> {
    let options = TranscribeOptions {
        model: model.to_owned(),
        language: language.to_owned(),
    };
    transcriber.start(SESSION, &options)
}

#[test]
fn auswertung_transkript_und_bilder() {
    let rig = EvalRig::new(true);
    let status = rig.evaluate().status();
    assert!(!status.running && status.error.is_empty(), "{status:?}");
    assert_eq!((status.done, status.total), (2, 2));
    let calls = rig.calls.lines();
    assert_eq!(calls.len(), 3, "{calls:?}");
    assert!(
        calls[1].contains("-ss 5.000") && calls[2].contains("-ss 20.500"),
        "{calls:?}"
    );
    assert!(!rig.dir.join("transcript.part").exists(), "Rohdatei bleibt liegen");
    assert_eq!(read_meta(&rig.dir).stand, "transkribiert");
    assert_eq!(rig.store().get(SESSION).unwrap().stage, "transkribiert");
    assert_eq!(
        rig.text("transcript.txt"),
        "[00:00] Hallo Welt.\n[00:09] Der Turm fehlt.\n"
    );
}

#[test]
fn auswertung_zeitleiste() {
    let rig = EvalRig::new(true);
    rig.evaluate();
    let timeline = rig.store().timeline(SESSION).unwrap();
    assert!(timeline.has_transcript);
    let stats = timeline.stats.expect("Statistik der Auswertung");
    assert_eq!((stats.segments, stats.frames), (2, 2));
    assert!(stats.total_ms >= 0);
    let got: Vec<(&str, i64)> = timeline.entries.iter().map(|e| (e.kind.as_str(), e.video_ms)).collect();
    assert_eq!(
        got,
        [("sprache", 0), ("marker", 5000), ("sprache", 9000), ("marker", 20500)]
    );
}

#[test]
fn auswertung_bilder_und_text() {
    let rig = EvalRig::new(true);
    rig.evaluate();
    let store = rig.store();
    let entries = store.timeline(SESSION).unwrap().entries;
    assert_eq!(entries.len(), 4);
    assert_eq!((entries[1].frame.as_str(), entries[1].nr), ("frames/marker-1.jpg", 1));
    assert_eq!(entries[3].frame, "frames/marker-2.jpg");
    assert_eq!(entries[1].text, "Marker · Spieler 1 · Insel 2 Stufe 3");
    assert_eq!(entries[3].text, "Ruckler · Spiel");
    assert!(store.frame_path(SESSION, 2).is_ok());
    assert_eq!(store.frame_path(SESSION, 9).unwrap_err(), Error::NotFound);
}

#[test]
fn zeitleiste_json_wie_go_mit_beschreibung_und_kaputten_zeilen() {
    let rig = EvalRig::new(true);
    put(
        &rig.dir,
        "markers.jsonl",
        "kaputt\n{\"ts\":1005000,\"kind\":\"pad\",\"player\":1}\n",
    );
    put(&rig.dir, "frames/marker-1.jpg", "jpg");
    put(&rig.dir, "descriptions.json", r#"{"1":{"text":"Ein  Turm"},"x":{}}"#);
    let value = serde_json::to_value(rig.store().timeline(SESSION).unwrap()).unwrap();
    assert_eq!(
        value,
        serde_json::json!({"id": SESSION, "captureMs": 1_000_000, "hasTranscript": false, "entries": [{
            "kind": "marker", "ms": 1_005_000, "videoMs": 5000, "text": "Marker · Spieler 1", "nr": 1,
            "frame": "frames/marker-1.jpg", "desc": "Ein  Turm",
            "marker": {"ts": 1_005_000, "kind": "pad", "player": 1, "world": "", "island": 0, "stage": 0,
                "night": false, "phase": "", "worst_frame_ms": 0.0}
        }]})
    );
}

#[test]
fn auswertung_ohne_video() {
    let rig = EvalRig::new(false);
    let status = rig.evaluate().status();
    assert!(status.error.is_empty(), "{status:?}");
    assert!(status.message.contains("video.mkv fehlt"), "{status:?}");
    assert_eq!(rig.calls.lines().len(), 1);
    assert!(
        rig.dir.join("transcript.json").exists(),
        "Transkript muss trotzdem entstehen"
    );
}

#[test]
fn auswertung_whisper_fehler_laesst_kein_transkript() {
    let rig = EvalRig::new(true);
    "whisper=".clone_into(&mut rig.calls.fail.lock().unwrap());
    let status = rig.evaluate().status();
    assert_eq!(status.error, "ffmpeg: exit status 1: Fehlerzeile");
    assert!(!rig.dir.join("transcript.json").exists());
}

#[test]
fn auswertung_abbrechen() {
    let rig = EvalRig::new(true);
    rig.calls.block.store(true, Ordering::SeqCst);
    let transcriber = rig.transcriber();
    start(&transcriber, "", "").unwrap();
    assert_eq!(start(&transcriber, "", "").unwrap_err(), Error::TranscribeBusy);
    assert!(until(|| !rig.calls.lines().is_empty()), "kein Aufruf");
    transcriber.cancel().unwrap();
    transcriber.wait();
    let status = transcriber.status();
    assert!(!status.running, "{status:?}");
    assert_eq!(status.error, "abgebrochen");
    for name in ["transcript.json", "transcript.part"] {
        assert!(!rig.dir.join(name).exists(), "{name} bleibt liegen");
    }
    assert_eq!(transcriber.cancel().unwrap_err(), Error::NoJob);
}

#[test]
fn modell_und_sprache_waehlbar_nur_aus_der_liste() {
    let rig = EvalRig::new(true);
    put(rig.root.path(), "models/whisper/anderes.bin", "zz");
    let transcriber = rig.transcriber();
    let other = rig.root.path().join("models").join("whisper").join("anderes.bin");
    let other = other.to_string_lossy().into_owned();
    for bad in ["../etc/passwd", &other, "C:/Windows/x.bin", "models/../x.bin", "env"] {
        let error = start(&transcriber, bad, "").unwrap_err().to_string();
        assert!(error.contains("unbekanntes Modell"), "{bad}: {error}");
    }
    let error = start(&transcriber, "", "fr").unwrap_err().to_string();
    assert_eq!(error, "Sprache \"fr\" ungültig: erlaubt de, en, auto");
    start(&transcriber, "models/whisper/anderes.bin", "en").unwrap();
    transcriber.wait();
    let line = &rig.calls.lines()[0];
    assert!(line.contains("anderes.bin") && line.contains("language=en"), "{line}");
    let raw = rig.text("transcript.json");
    assert!(
        raw.contains(r#""language": "en""#) && raw.contains("anderes.bin"),
        "{raw}"
    );
}

#[test]
fn start_ablehnungen() {
    let rig = EvalRig::new(true);
    let transcriber = rig.transcriber();
    let bad = transcriber.start("../x", &TranscribeOptions::default());
    assert_eq!(bad.unwrap_err(), Error::BadId);
    std::fs::remove_file(rig.dir.join("voice.wav")).unwrap();
    assert_eq!(start(&transcriber, "", "").unwrap_err(), Error::NoVoice);
    assert!(rig.calls.lines().is_empty());
}

#[test]
fn start_ohne_modell_lehnt_ab() {
    let mut rig = EvalRig::new(true);
    rig.model = rig.root.path().join("fehlt.bin").to_string_lossy().into_owned();
    let error = start(&rig.transcriber(), "", "").unwrap_err().to_string();
    assert!(error.starts_with("Modell fehlt"), "{error}");
    assert!(rig.calls.lines().is_empty());
}

#[test]
fn set_frame_skip() {
    let rig = EvalRig::new(true);
    rig.evaluate();
    let store = rig.store();
    // Doppelt ausblenden ändert nichts.
    for _ in 0..2 {
        store.set_frame_skip(SESSION, 1, true).unwrap();
    }
    assert!(
        rig.dir.join("frames").join("marker-1.jpg").exists(),
        "Bilddatei muss bleiben"
    );
    let entries = store.timeline(SESSION).unwrap().entries;
    assert!(entries[1].skip && !entries[1].frame.is_empty() && !entries[3].skip);
    assert_eq!(read_meta(&rig.dir).skip_frames, [1]);
    store.set_frame_skip(SESSION, 1, false).unwrap();
    assert!(!store.timeline(SESSION).unwrap().entries[1].skip);
    assert_eq!(store.set_frame_skip(SESSION, 0, true).unwrap_err(), Error::BadId);
    assert_eq!(store.set_frame_skip("../x", 1, true).unwrap_err(), Error::BadId);
}

/// Scheitert nur ein Marker-Bild (Marker nach dem Ende des Videos), bleibt das Transkript.
#[test]
fn auswertung_bild_fehlt_kippt_nicht_die_auswertung() {
    let rig = EvalRig::new(true);
    "marker-2".clone_into(&mut rig.calls.fail.lock().unwrap());
    let status = rig.evaluate().status();
    assert!(status.error.is_empty(), "{status:?}");
    assert_eq!(
        status.message,
        "Bild fehlt bei Marker 2 (liegt er nach dem Ende von video.mkv?)"
    );
    assert!(rig.dir.join("transcript.json").exists());
    assert!(rig.dir.join("frames").join("marker-1.jpg").exists());
}
