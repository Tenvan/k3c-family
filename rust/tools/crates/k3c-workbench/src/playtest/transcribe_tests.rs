//! `TestWhisperArgsRelativ`, `TestParseSegmentsZeitUmrechnung` aus `transcribe_test.go` und `TestEnvModellInListe` aus
//! `transcribe_auswertung_test.go`. Die Umgebung steht als Argument, kein Test setzt Variablen.

use super::*;
use crate::playtest::rig_tests::put;
use crate::testutil::tmp_dir;

#[test]
fn whisper_args_relativ() {
    let dir = Path::new("C:/repo/playtests/x");
    let model = "C:/repo/models/whisper/ggml-large-v3.bin";
    let line = whisper_args("ffmpeg", dir, model, "de", "transcript.part").join(" ");
    for want in [
        "-i voice.wav",
        "-af whisper=model=../../models/whisper/ggml-large-v3.bin",
        "language=de",
        "queue=30",
        "format=json",
        "destination=transcript.part",
        "-f null -",
    ] {
        assert!(line.contains(want), "Aufruf ohne {want}: {line}");
    }
    assert!(!line.contains("C:"), "Laufwerk im Filter: {line}");
}

#[cfg(windows)]
#[test]
fn whisper_args_anderes_laufwerk_maskiert() {
    let line = whisper_args("ffmpeg", Path::new("C:/repo/x"), r"D:\m\x.bin", "de", "t").join(" ");
    assert!(line.contains(r"model=D\\:/m/x.bin:language"), "{line}");
}

#[test]
fn parse_segments_zeit_umrechnung() {
    let raw = concat!(
        r#"{"start":0,"end":2500,"text":" Hallo."}"#,
        "\n",
        r#"{"start":100,"end":200,"text":""}"#,
        "\n",
        r#"{"start":9000,"end":11000,"text":"Zwei"}"#
    );
    let segments = parse_segments(raw.as_bytes(), 1_000_000).unwrap();
    assert_eq!(segments.len(), 2);
    let first = &segments[0];
    assert_eq!((first.start_ms, first.end_ms), (1_000_000, 1_002_500));
    assert_eq!((first.text.as_str(), segments[1].start_ms), ("Hallo.", 1_009_000));
    let broken = parse_segments(b"kein json", 0).unwrap_err().to_string();
    assert!(broken.starts_with("Transkript nicht lesbar: "), "{broken}");
    for form in [
        r#"[{"start":5,"end":6,"text":"a"}]"#,
        r#"{"segments":[{"start":5,"end":6,"text":"a"}]}"#,
    ] {
        let list = parse_segments(form.as_bytes(), 10).unwrap();
        assert_eq!((list.len(), list[0].start_ms), (1, 15), "{form}");
    }
}

#[test]
fn env_modell_in_liste() {
    let (root, other) = (tmp_dir(), tmp_dir());
    put(other.path(), "x.bin", "x");
    let file = other.path().join("x.bin").to_string_lossy().into_owned();
    let list = list_models(root.path(), &file);
    assert_eq!(list.len(), 1);
    assert_eq!((list[0].id.as_str(), list[0].name.as_str()), ("env", "x.bin"));
    assert!(list[0].default);
    assert_eq!(list[0].path, file);
}

#[test]
fn modelle_aus_beiden_ordnern_ohne_pfad_im_json() {
    let root = tmp_dir();
    put(root.path(), "models/b.bin", "x");
    put(root.path(), "models/whisper/a.bin", "x");
    put(root.path(), "models/whisper/liesmich.txt", "x");
    let list = list_models(root.path(), "");
    let ids: Vec<&str> = list.iter().map(|m| m.id.as_str()).collect();
    assert_eq!(ids, ["models/b.bin", "models/whisper/a.bin"]);
    assert_eq!(
        serde_json::to_value(&list[0]).unwrap(),
        serde_json::json!({"id": "models/b.bin", "name": "b.bin", "sizeMb": 0, "default": true})
    );
}
