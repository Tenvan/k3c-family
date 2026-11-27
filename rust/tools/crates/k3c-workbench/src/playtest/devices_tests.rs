//! Port von `devices_test.go` (die Suche nach ffmpeg steht in `ffmpeg_find_tests.rs`).

use std::sync::{Arc, Mutex};

use super::*;

const OK_SIZE: &str = "Stream #0:0: Video: wrapped_avframe, 1920x1080 [SAR 1:1]";

fn probe(run: impl Fn(&[String]) -> (&'static str, bool) + Send + Sync + 'static) -> Probe {
    Arc::new(move |args| {
        let (out, ok) = run(args);
        let result = if ok { Ok(()) } else { Err("exit status 1".to_owned()) };
        (out.to_owned(), result)
    })
}

#[test]
fn parse_monitor_size_aus_der_ausgabe() {
    let out = "Input #0, lavfi, from 'ddagrab=output_idx=0':\n  Stream #0:0: Video: wrapped_avframe, \
               bgra(tv, bt709), 2560x1440 [SAR 1:1 DAR 16:9], 1 fps\n";
    assert_eq!(parse_monitor_size(out), (2560, 1440));
    assert_eq!(parse_monitor_size("nichts"), (0, 0));
}

#[test]
fn list_monitors_bis_zum_ersten_fehler() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&calls);
    let run = probe(move |args| {
        seen.lock().unwrap().push(args[5].clone());
        if args[5].contains("output_idx=2") {
            ("Invalid output index", false)
        } else {
            (OK_SIZE, true)
        }
    });
    let monitors = list_monitors(&run, "ff");
    assert_eq!(monitors.len(), 2, "{monitors:?}");
    assert_eq!(monitors[1].index, 1);
    assert_eq!(monitors[0].label, "Monitor 1 (1920×1080)");
    assert_eq!(calls.lock().unwrap().len(), 3);
    assert!(list_monitors(&probe(|_| ("", false)), "ff").is_empty(), "ohne Desktop");
}

#[test]
fn probe_aufrufe_wie_go() {
    assert_eq!(
        monitor_probe_args("ff", 1).join(" "),
        "ff -hide_banner -f lavfi -i ddagrab=output_idx=1:framerate=1 -frames:v 1 -f null -"
    );
    assert_eq!(
        mic_probe_args("ff", "M").join(" "),
        "ff -hide_banner -f dshow -i audio=M -t 0.4 -f null -"
    );
}

#[test]
fn check_devices_nennt_geraet_und_grund() {
    check_devices(&probe(|_| ("", true)), "ff", "Raummikro", 0).unwrap();
    let bad_mic = probe(|a| {
        if a[3] == "dshow" {
            ("Could not run graph\nI/O error", false)
        } else {
            ("", true)
        }
    });
    let error = check_devices(&bad_mic, "ff", "Raummikro", 0).unwrap_err().to_string();
    assert_eq!(
        error,
        "Mikrofon „Raummikro“ lässt sich nicht öffnen (abgezogen, belegt oder umbenannt?): I/O error"
    );
    let bad_monitor = probe(|a| {
        if a[3] == "lavfi" {
            ("Invalid output index\n\n", false)
        } else {
            ("", true)
        }
    });
    let error = check_devices(&bad_monitor, "ff", "M", 3).unwrap_err().to_string();
    assert_eq!(
        error,
        "Monitor 4 lässt sich nicht aufnehmen (nicht angeschlossen?): Invalid output index"
    );
    // Mikro-Session: der Monitor wird nicht geprüft.
    check_devices(&bad_monitor, "ff", "M", -1).unwrap();
}
