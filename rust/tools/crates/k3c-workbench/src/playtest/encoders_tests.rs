//! Port von `encoders_test.go`.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::*;
use crate::playtest::ffmpeg_args;
use crate::playtest::rig_tests::{Rig, mic};

fn failing(calls: &Arc<AtomicUsize>, works: &'static str) -> Probe {
    let calls = Arc::clone(calls);
    Arc::new(move |args| {
        calls.fetch_add(1, Ordering::SeqCst);
        if !works.is_empty() && args.join(" ").contains(works) {
            return (String::new(), Ok(()));
        }
        let out = "Error creating a MFX session\nError while opening encoder";
        (out.to_owned(), Err("exit".to_owned()))
    })
}

#[test]
fn pick_encoder_nimmt_den_ersten_funktionierenden() {
    let calls = Arc::default();
    let (encoder, failed) = pick_encoder(&failing(&calls, "h264_amf"), "ff");
    assert_eq!(encoder, ENCODER_AMF);
    assert_eq!(failed, ["h264_nvenc: Error while opening encoder"]);
}

#[test]
fn pick_encoder_faellt_auf_x264_zurueck_ohne_eine_probe() {
    let calls = Arc::default();
    let (encoder, failed) = pick_encoder(&failing(&calls, ""), "ff");
    assert_eq!(
        (encoder, failed.len(), calls.load(Ordering::SeqCst)),
        (ENCODER_X264, 3, 3)
    );
}

/// Die Probe öffnet GPU-Sitzungen: je ffmpeg-Pfad nur einmal.
#[test]
fn pick_cache_probt_einmal() {
    let calls = Arc::default();
    let run = failing(&calls, "h264_nvenc");
    let cache = PickCache::default();
    assert_eq!(cache.pick(&run, "ff", None), ENCODER_NVENC);
    assert_eq!(cache.clone().pick(&run, "ff", None), ENCODER_NVENC);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[test]
fn ffmpeg_args_jeder_encoder() {
    for encoder in ENCODER_ORDER {
        let a = ffmpeg_args("ffmpeg", Path::new("D"), "Mikro", 0, encoder).join(" ");
        let codec = encoder.split_whitespace().next().unwrap();
        assert!(a.contains(&format!("-c:v {codec}")), "{encoder}: {a}");
        assert!(
            a.contains("-cluster_time_limit 1000") && a.contains("-flush_packets 1"),
            "{a}"
        );
    }
}

#[test]
fn recorder_nutzt_geprueften_encoder() {
    let mut rig = Rig::new();
    rig.config.pick = Some(Arc::new(|_| ENCODER_AMF.to_owned()));
    let rec = rig.rec();
    assert_eq!(rec.start(&mic("M")).unwrap().encoder, ENCODER_AMF);
    rec.stop().unwrap();
    rec.wait();
}
