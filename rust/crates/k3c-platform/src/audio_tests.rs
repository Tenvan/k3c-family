use std::path::Path;

use super::*;

fn probe(name: &str) -> Vec<u8> {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../public/audio")
        .join(name);
    std::fs::read(&p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

#[test]
fn audio_null_zaehlt_und_klingt_nie() {
    let mut a = NullAudio::default();
    let s = a.load(b"egal").unwrap();
    let (v, w) = (a.play(s, 0.5), a.play_loop(s, 1.0));
    assert_ne!(v, w, "jede Stimme neu");
    a.set_volume(v, 0.2);
    a.stop(w);
    assert!(!a.playing(v) && !a.playing(w));
    assert_eq!((a.loads, a.plays), (1, 2));
    assert_eq!(a.failed(), None);
}

#[test]
fn audio_dekodiert_ogg_und_mp3_mit_44100_hz_ohne_geraet() {
    for name in ["probe-a.ogg", "probe-a.mp3"] {
        let pcm = crate::backend::decode_sound(&probe(name)).unwrap();
        assert_eq!(pcm.rate, 44_100, "{name}");
        assert!((1..=2).contains(&pcm.channels), "{name}: {} Kanäle", pcm.channels);
        let secs = pcm.samples.len() as f64 / f64::from(pcm.channels) / 44_100.0;
        assert!(secs > 0.1, "{name}: {secs} s");
        assert!(pcm.samples.iter().any(|s| s.abs() > 0.01), "{name}: nur Stille");
    }
}

#[test]
fn audio_kaputte_datei_ist_fehler() {
    assert!(crate::backend::decode_sound(b"kein klang").is_err());
}

#[test]
fn audio_ohne_geraet_ist_stumm_statt_absturz() {
    let mut a = open_audio_with(|| Err("kein Gerät (Test)".into()));
    let s = a.load(&probe("probe-a.ogg")).unwrap();
    let v = a.play(s, 1.0);
    assert!(!a.playing(v));
    assert_eq!(a.failed(), None, "NullAudio meldet keinen Fehler");
}
