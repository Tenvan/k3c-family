//! Port von `record_test.go` (Attrappen aus `rig_tests.rs`).

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::time::Duration;

use super::*;
use crate::playtest::encoders::{ENCODER_QSV, ENCODER_X264};
use crate::playtest::exec::parse_mics;
use crate::playtest::list_devices;
use crate::playtest::playtest::{STAGE_RECORDED, STAND_RUNNING};
use crate::playtest::rig_tests::{Rig, mic};

fn args(monitor: i64, microphone: &str, encoder: &str) -> String {
    ffmpeg_args("ffmpeg", Path::new("D"), microphone, monitor, encoder).join(" ")
}

#[test]
fn ffmpeg_args_qsv() {
    let a = args(1, "Mikro (USB)", ENCODER_QSV);
    let voice = format!(
        "-ac 1 -ar 16000 -c:a pcm_s16le -flush_packets 1 {}",
        Path::new("D").join("voice.wav").display()
    );
    let video = Path::new("D").join("video.mkv").to_string_lossy().into_owned();
    for want in [
        "-f dshow -i audio=Mikro (USB)",
        "ddagrab=output_idx=1:framerate=30",
        &voice,
        "scale=1920:1080",
        "-c:v h264_qsv",
        "-r 30",
        &video,
    ] {
        assert!(a.contains(want), "fehlt {want:?} in {a}");
    }
    assert!(!a.contains("libx264"), "{a}");
}

/// Der ganze Aufruf Zeichen für Zeichen wie `FFmpegArgs` in Go.
#[test]
fn ffmpeg_args_ganz() {
    let sep = std::path::MAIN_SEPARATOR;
    let want = format!(
        "ffmpeg -hide_banner -loglevel warning -nostats -y -f dshow -i audio=M -f lavfi -i \
         ddagrab=output_idx=0:framerate=30 -map 0:a -ac 1 -ar 16000 -c:a pcm_s16le -flush_packets 1 D{sep}voice.wav \
         -map 1:v -vf hwdownload,format=bgra,scale=1280:720:flags=fast_bilinear,format=yuv420p -r 30 -c:v libx264 \
         -preset ultrafast -crf 28 -flush_packets 1 -cluster_time_limit 1000 D{sep}video.mkv"
    );
    assert_eq!(args(0, "M", ENCODER_X264), want);
}

#[test]
fn ffmpeg_args_fallback() {
    let a = args(0, "M", ENCODER_X264);
    for want in ["libx264 -preset ultrafast", "scale=1280:720", "output_idx=0"] {
        assert!(a.contains(want), "fehlt {want:?} in {a}");
    }
    assert!(!a.contains("qsv"), "{a}");
}

#[test]
fn start_stopp_mit_spielende() {
    let mut rig = Rig::new();
    let rec = rig.rec();
    let mut options = mic("Mikro");
    options.monitor = 1;
    let st = rec.start(&options).unwrap();
    assert!(
        st.running && st.id == "2026-11-22-1" && st.encoder == ENCODER_QSV,
        "{st:?}"
    );
    let session = rig.store().root.join("2026-11-22-1");
    assert_eq!(rig.procs().len(), 2);
    let playtest = format!("--playtest {}", session.display());
    assert!(rig.game().unwrap().line().contains(&playtest));
    assert_eq!(rig.game().unwrap().dir, Path::new("C:/g"));
    assert_eq!(rig.ffmpeg().unwrap().dir, session);
    assert_eq!(rec.start(&mic("Mikro")).unwrap_err(), Error::Busy);
    let m = rig.meta(&st.id);
    assert!(m.stand == STAND_RUNNING && m.start_ms != 0 && m.end_ms == 0, "{m:?}");
    assert!(
        m.encoder == ENCODER_QSV && m.mic == "Mikro" && m.monitor == "1",
        "{m:?}"
    );
    assert!(
        rig.store().get(&st.id).unwrap().running,
        "Liste zeigt keine laufende Aufnahme"
    );
}

/// Spielende beendet die Aufnahme sauber; danach ist eine neue möglich, die Nummer zählt hoch.
#[test]
fn spielende_beendet_aufnahme() {
    let mut rig = Rig::new();
    let rec = rig.rec();
    let st = rec.start(&mic("Mikro")).unwrap();
    rig.game().unwrap().end(Ok(()));
    rec.wait();
    let ff = rig.ffmpeg().unwrap();
    assert_eq!((ff.quits(), ff.kills()), (1, 0));
    let m = rig.meta(&st.id);
    assert!(m.stand == STAGE_RECORDED && m.end_ms != 0 && !m.crash, "{m:?}");
    let got = rig.store().get(&st.id).unwrap();
    assert!(got.stage == STAGE_RECORDED && !got.running, "{got:?}");
    assert!(!rec.status().running);
    assert_eq!(rec.start(&mic("Mikro")).unwrap().id, "2026-11-22-2");
}

#[test]
fn absturz_des_spiels() {
    let mut rig = Rig::new();
    let rec = rig.rec();
    let st = rec.start(&mic("M")).unwrap();
    rig.game().unwrap().end(Err("exit status 0xc0000005"));
    rec.wait();
    let m = rig.meta(&st.id);
    assert!(m.stand == STAGE_RECORDED && m.crash && m.end_ms != 0, "{m:?}");
    assert!(m.game_exit.contains("c0000005"), "{m:?}");
    assert_eq!(rig.ffmpeg().unwrap().quits(), 1);
}

/// AC: Stopp beendet das Spiel und danach ffmpeg mit `quit` (`q`), nicht hart.
#[test]
fn stopp_beendet_spiel_ohne_absturz() {
    let mut rig = Rig::new();
    let rec = rig.rec();
    let st = rec.start(&mic("M")).unwrap();
    rec.stop().unwrap();
    rec.wait();
    let m = rig.meta(&st.id);
    assert!(!m.crash && m.stand == STAGE_RECORDED, "{m:?}");
    assert_eq!(rig.game().unwrap().kills(), 1);
    let ff = rig.ffmpeg().unwrap();
    assert_eq!((ff.quits(), ff.kills()), (1, 0));
    assert_eq!(rec.stop().unwrap_err(), Error::NotRunning);
}

#[test]
fn haengendes_ffmpeg_wird_nach_frist_beendet() {
    let mut rig = Rig::new();
    rig.switches.hang_ffmpeg.store(true, Ordering::SeqCst);
    let rec = rig.rec();
    rec.start(&mic("M")).unwrap();
    rig.game().unwrap().end(Ok(()));
    rec.wait();
    let ff = rig.ffmpeg().unwrap();
    assert_eq!((ff.quits(), ff.kills()), (1, 1));
}

#[test]
fn qsv_faellt_auf_x264_zurueck() {
    let mut rig = Rig::new();
    rig.switches.fail_qsv.store(true, Ordering::SeqCst);
    let st = rig.rec().start(&mic("M")).unwrap();
    assert_eq!(st.encoder, ENCODER_X264);
    let m = rig.meta(&st.id);
    assert!(m.encoder == ENCODER_X264 && m.ffmpeg.contains("libx264"), "{m:?}");
}

#[test]
fn ffmpeg_scheitert_beendet_das_spiel() {
    let mut rig = Rig::new();
    rig.switches.fail_all.store(true, Ordering::SeqCst);
    let rec = rig.rec();
    let error = rec.start(&mic("M")).unwrap_err().to_string();
    assert_eq!(
        error,
        "ffmpeg startet nicht (Mikro belegt oder Gerät falsch?): Error: device busy"
    );
    assert_eq!(rig.game().unwrap().kills(), 1);
    assert!(rig.store().list().unwrap().is_empty(), "Ordner blieb");
    assert!(!rec.status().running);
    // Darf nicht hängen.
    rec.wait();
}

#[test]
fn voraussetzungen() {
    let mut rig = Rig::new();
    rig.config.look_path = Arc::new(|_| Err(Error::Other("nix".to_owned())));
    assert_eq!(rig.rec().start(&mic("M")).unwrap_err(), Error::NoFfmpeg(Vec::new()));
    let mut rig = Rig::new();
    assert_eq!(rig.rec().start(&Options::default()).unwrap_err(), Error::NoMic);
    assert!(rig.procs().is_empty(), "Prozess gestartet");
    let mut rig = Rig::new();
    rig.config.game = Arc::new(|| Err(Error::NoGame));
    assert_eq!(rig.rec().start(&mic("M")).unwrap_err(), Error::NoGame);
    assert!(rig.procs().is_empty(), "Prozess gestartet");
}

#[test]
fn parse_mics_neu_und_alt() {
    let neu = "[dshow @ 000] \"Mikrofon (Realtek)\" (audio)\n\
               [dshow @ 000]   Alternative name \"@device_cm_{33D9}\\wave_{1}\"\n\
               [dshow @ 000] \"Integrated Camera\" (video)\n\
               [dshow @ 000] \"Raummikro (USB)\" (audio)";
    assert_eq!(parse_mics(neu), ["Mikrofon (Realtek)", "Raummikro (USB)"]);
    let alt = "[dshow @ 000] DirectShow video devices\n\
               [dshow @ 000]  \"Camera\"\n\
               [dshow @ 000] DirectShow audio devices\n\
               [dshow @ 000]  \"Mikro alt\"\n\
               [dshow @ 000]     Alternative name \"@device\"";
    assert_eq!(parse_mics(alt), ["Mikro alt"]);
}

#[test]
fn list_devices_ohne_ffmpeg() {
    let never: crate::playtest::Probe = Arc::new(|_| panic!("kein Aufruf ohne ffmpeg"));
    let missing: crate::playtest::LookPath = Arc::new(|_| Err(Error::Other("nix".to_owned())));
    let d = list_devices(&missing, &never, &never, &crate::playtest::PickCache::default());
    assert!(!d.ffmpeg && d.mics.is_empty() && d.message.contains("ffmpeg"), "{d:?}");
    let json = serde_json::to_string(&d).unwrap();
    assert!(
        json.contains(r#""mics":[],"monitors":[],"ffmpeg":false,"ffmpegPath":"","encoder":"""#),
        "{json}"
    );
}

#[test]
fn start_prueft_geraete_vor_dem_start() {
    let mut rig = Rig::new();
    let boom = Error::Other("Mikrofon „M“ lässt sich nicht öffnen".to_owned());
    let returned = boom.clone();
    rig.config.check = Some(Arc::new(move |_, _| Err(returned.clone())));
    let rec = rig.rec();
    assert_eq!(rec.start(&mic("M")).unwrap_err(), boom);
    assert!(!rec.status().running);
    assert!(!rig.store().root.exists(), "kein Session-Ordner erwartet");
}

#[test]
fn status_antwortet_waehrend_der_geraetepruefung() {
    let mut rig = Rig::new();
    let (entered, inside) = mpsc::channel();
    let release = crate::playtest::Signal::new();
    let wait = Arc::clone(&release);
    rig.config.check = Some(Arc::new(move |_, _| {
        entered.send(()).unwrap();
        wait.wait();
        Ok(())
    }));
    let rec = rig.rec();
    let starting = Arc::clone(&rec);
    let start = std::thread::spawn(move || starting.start(&mic("M")).map(|st| st.running));
    inside.recv_timeout(Duration::from_secs(2)).unwrap();
    let (got, status) = mpsc::channel();
    let asking = Arc::clone(&rec);
    let ask = std::thread::spawn(move || got.send(asking.status().running).unwrap());
    let answered = status.recv_timeout(Duration::from_secs(2));
    release.set(());
    assert_eq!(answered, Ok(false), "status() blockiert während der Geräteprüfung");
    ask.join().unwrap();
    assert_eq!(start.join().unwrap(), Ok(true));
}

/// Die Form von `session.json`: Feldreihenfolge, Einrückung und Auslassungen wie Go `MarshalIndent`.
#[test]
fn session_json_wie_go() {
    let mut rig = Rig::new();
    let rec = rig.rec();
    let mut options = mic("A<B>&C");
    options.voice_only = true;
    let st = rec.start(&options).unwrap();
    rec.stop().unwrap();
    rec.wait();
    let path = rig.store().root.join(&st.id).join("session.json");
    let text = std::fs::read_to_string(path).unwrap();
    let ms = 1_795_370_400_000i64;
    let head = format!(
        "{{\n  \"startMs\": {ms},\n  \"captureMs\": {ms},\n  \"endMs\": {ms},\n  \
         \"mic\": \"A\\u003cB\\u003e\\u0026C\",\n  \"monitor\": \"\",\n  \"ffmpeg\": \"ffmpeg -hide_banner"
    );
    assert!(text.starts_with(&head), "{text}");
    assert!(
        text.ends_with("\"gameArgs\": null,\n  \"stand\": \"aufgenommen\"\n}"),
        "{text}"
    );
}
