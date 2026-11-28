//! Port von `start_test.go` (B-1235): Spiel zuerst, Aufnahme erst nach dem ersten Bild, Mikro-Session ohne Spiel.

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

use super::*;
use crate::playtest::encoders::ENCODER_QSV;
use crate::playtest::playtest::STAGE_RECORDED;
use crate::playtest::rig_tests::{Rig, mic, until};

#[test]
fn spiel_start_scheitert_ohne_ffmpeg() {
    let mut rig = Rig::new();
    rig.switches.fail_game.store(true, Ordering::SeqCst);
    let rec = rig.rec();
    let error = rec.start(&mic("M")).unwrap_err().to_string();
    assert_eq!(error, "Spiel nicht gestartet: kaputt");
    assert!(rig.ffmpeg().is_none() && !rec.status().running);
    assert!(rig.store().list().unwrap().is_empty(), "Ordner blieb");
}

/// Die Aufnahme beginnt erst, wenn das Spiel `ready` schreibt; bis dahin meldet der Status „startet“.
#[test]
fn aufnahme_beginnt_erst_mit_dem_ersten_bild() {
    let mut rig = Rig::new();
    rig.config.ready = Duration::from_secs(5);
    let rec = rig.rec();
    let starting = Arc::clone(&rec);
    let start = std::thread::spawn(move || starting.start(&mic("M")));
    assert!(
        until(|| rec.status().starting && rig.game().is_some()),
        "{:?}",
        rec.status()
    );
    assert!(rec.status().running);
    assert_eq!(rig.procs().len(), 1, "vor dem ersten Bild darf nur das Spiel laufen");
    let session = rig.store().root.join(rec.active_id());
    assert!(!rec.active_id().is_empty());
    std::fs::write(session.join(FILE_READY), "1").unwrap();
    let st = start.join().unwrap().unwrap();
    assert!(!st.starting && st.encoder == ENCODER_QSV, "{st:?}");
    assert!(!session.join(FILE_READY).exists(), "ready-Datei blieb liegen");
}

#[test]
fn stopp_waehrend_das_spiel_laedt() {
    let mut rig = Rig::new();
    rig.config.ready = Duration::from_secs(5);
    let rec = rig.rec();
    let starting = Arc::clone(&rec);
    let start = std::thread::spawn(move || starting.start(&mic("M")));
    // Stopp ist erst möglich, wenn das Spiel läuft.
    assert!(until(|| rec.stop().is_ok()));
    let error = start.join().unwrap().unwrap_err().to_string();
    assert!(error.starts_with("Spiel vor dem ersten Bild beendet"), "{error}");
    assert!(rig.ffmpeg().is_none());
    assert!(rig.store().list().unwrap().is_empty(), "Ordner blieb");
}

#[test]
fn mikro_session_ohne_bild_und_spiel() {
    let mut rig = Rig::new();
    // Ohne Spiel-EXE muss sie trotzdem starten.
    rig.config.game = Arc::new(|| Err(Error::NoGame));
    let rec = rig.rec();
    let mut options = mic("Mikro");
    (options.monitor, options.voice_only) = (99, true);
    let st = rec.start(&options).unwrap();
    assert!(st.running && st.voice_only && st.encoder.is_empty(), "{st:?}");
    assert_eq!(rig.procs().len(), 1, "nur ffmpeg erwartet");
    let a = rig.ffmpeg().unwrap().line();
    assert!(!a.contains("ddagrab") && !a.contains("video.mkv"), "{a}");
    assert!(a.contains("audio=Mikro") && a.ends_with("voice.wav"), "{a}");
    assert!(rec.status().stats.unwrap().no_video);
}

/// Stopp beendet die Mikro-Session sauber: `session.json` nach Stopp, ffmpeg mit `q`, kein Absturz.
#[test]
fn mikro_session_stoppt_sauber() {
    let mut rig = Rig::new();
    rig.config.game = Arc::new(|| Err(Error::NoGame));
    let rec = rig.rec();
    let mut options = mic("Mikro");
    options.voice_only = true;
    let st = rec.start(&options).unwrap();
    rec.stop().unwrap();
    rec.wait();
    let m = rig.meta(&st.id);
    assert!(m.stand == STAGE_RECORDED && m.end_ms != 0 && !m.crash, "{m:?}");
    assert!(
        m.error.is_empty() && m.monitor.is_empty() && m.game_args.is_none(),
        "{m:?}"
    );
    let ff = rig.ffmpeg().unwrap();
    assert_eq!((ff.quits(), ff.kills()), (1, 0));
}
