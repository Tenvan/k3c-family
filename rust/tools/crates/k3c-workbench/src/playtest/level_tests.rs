//! Port von `level_test.go`.

use super::*;
use crate::testutil::tmp_dir;

fn pcm(amplitude: i16, samples: usize) -> Vec<u8> {
    (0..samples)
        .flat_map(|i| {
            let v = if i % 2 == 1 { -amplitude } else { amplitude };
            v.to_le_bytes()
        })
        .collect()
}

#[test]
fn level_percent_von_still_bis_voll() {
    assert_eq!(level_percent(&pcm(0, 100)), 0);
    assert!(level_percent(&pcm(32767, 100)) >= 99);
    let (quiet, loud) = (level_percent(&pcm(100, 1000)), level_percent(&pcm(8000, 1000)));
    assert!(quiet > 0 && loud > quiet && loud < 100, "leise {quiet}, laut {loud}");
}

#[test]
fn wav_level_liest_das_ende_der_datei() {
    let dir = tmp_dir();
    let path = dir.path().join("voice.wav");
    let mut data = vec![0; usize::try_from(WAV_HEADER).unwrap()];
    // Lange Stille, dann die letzte Sekunde laut.
    data.extend(pcm(0, 40000));
    data.extend(pcm(8000, 16000));
    std::fs::write(&path, data).unwrap();
    let level = wav_level(&path);
    assert!(level >= 50, "Pegel der letzten Sekunde: {level}");
    assert_eq!(wav_level(&dir.path().join("fehlt.wav")), 0);
}
