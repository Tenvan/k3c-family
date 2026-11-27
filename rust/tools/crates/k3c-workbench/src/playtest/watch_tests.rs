//! Port von `watch_test.go`.

use std::time::Duration;

use super::*;
use crate::playtest::rig_tests::{Rig, mic, until};

fn sample(voice: i64, video: i64) -> Sample<'static> {
    Sample {
        voice,
        video,
        alive: true,
        tail: "",
        level: 0,
    }
}

#[test]
fn clone_liefert_nie_null_listen() {
    let json = serde_json::to_string(&Stats::default().clone()).unwrap();
    assert!(!json.contains("null"), "{json}");
    assert_eq!(
        json,
        r#"{"voiceBytes":0,"videoBytes":0,"voiceRate":[],"videoRate":[],"voiceLevel":[],"warn":"","noVideo":false}"#
    );
}

#[test]
fn advance_warnt_erst_nach_dem_anlauf() {
    let (mut s, mut w) = (Stats::default(), WatchState::default());
    for tick in 0..WARMUP {
        s.advance(&mut w, &sample(0, 0));
        assert!(s.warn.is_empty(), "Takt {tick}: {}", s.warn);
    }
}

#[test]
fn advance_misst_rate_und_warnt_bei_stillstand() {
    let (mut s, mut w) = (Stats::default(), WatchState::default());
    let (mut voice, mut video) = (0, 0);
    for _ in 0..8 {
        voice += 32000;
        video += 90000;
        s.advance(&mut w, &sample(voice, video));
    }
    assert!(s.warn.is_empty(), "{}", s.warn);
    assert_eq!((s.voice_rate.last(), s.video_rate.last()), (Some(&32000), Some(&90000)));
    // Ton steht, Bild läuft.
    for _ in 0..IDLE_VOICE {
        video += 90000;
        s.advance(&mut w, &sample(voice, video));
    }
    assert_eq!(s.warn, "Ton kommt nicht mehr an (seit 4 s) – Mikro prüfen");
    // Jetzt steht auch das Bild.
    for _ in 0..IDLE_VIDEO {
        s.advance(&mut w, &sample(voice, video));
    }
    assert_eq!(s.warn, "Ton und Bild kommen nicht mehr an (seit 5 s)");
    s.advance(&mut w, &sample(voice + 100, video + 100));
    assert!(s.warn.is_empty(), "Warnung bleibt: {}", s.warn);
}

#[test]
fn advance_ffmpeg_tot() {
    let (mut s, mut w) = (Stats::default(), WatchState::default());
    let dead = Sample {
        alive: false,
        tail: "Error while opening encoder",
        ..sample(100, 100)
    };
    s.advance(&mut w, &dead);
    assert_eq!(
        s.warn,
        "ffmpeg ist beendet – es wird nichts mehr aufgezeichnet (Error while opening encoder)"
    );
}

#[test]
fn advance_haelt_hoechstens_stat_samples() {
    let (mut s, mut w) = (Stats::default(), WatchState::default());
    for i in 1..=i64::try_from(STAT_SAMPLES + 30).unwrap() {
        s.advance(&mut w, &sample(i * 10, i * 10));
    }
    assert_eq!(
        (s.voice_rate.len(), s.video_rate.len(), s.voice_level.len()),
        (STAT_SAMPLES, STAT_SAMPLES, STAT_SAMPLES)
    );
}

#[test]
fn waechter_meldet_tote_aufnahme_im_status() {
    let mut rig = Rig::new();
    rig.config.interval = Duration::from_millis(5);
    let rec = rig.rec();
    rec.start(&mic("M")).unwrap();
    // ffmpeg stirbt mitten in der Aufnahme.
    rig.ffmpeg().unwrap().end(Ok(()));
    let warned = || {
        rec.status()
            .stats
            .is_some_and(|s| s.warn.contains("ffmpeg ist beendet"))
    };
    assert!(until(warned), "{:?}", rec.status().stats);
    rec.stop().unwrap();
    rec.wait();
}

#[test]
fn advance_mikro_session_warnt_nie_wegen_bild() {
    let mut s = Stats {
        no_video: true,
        ..Stats::default()
    };
    let mut w = WatchState::default();
    let mut voice = 0;
    for _ in 0..12 {
        voice += 32000;
        s.advance(&mut w, &sample(voice, 0));
    }
    assert!(s.warn.is_empty(), "{}", s.warn);
    for _ in 0..IDLE_VOICE {
        s.advance(&mut w, &sample(voice, 0));
    }
    assert!(s.warn.contains("Ton kommt nicht mehr an"), "{}", s.warn);
}
