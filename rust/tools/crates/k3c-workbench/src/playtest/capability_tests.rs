//! Aus `transcribe_test.go` (`TestCheckFehlerfaelle`, `TestCheckModellFehlt`) und `transcribe_auswertung_test.go`
//! (`TestModelPathSuchreihenfolge`, `TestCheckMeldetModellgroesse`).

use std::sync::Arc;

use super::*;
use crate::playtest::Error;
use crate::testutil::tmp_dir;

const GOOD_VERSION: &str = "ffmpeg version 9.0.2-full_build-www.gyan.dev Copyright (c) 2000-2025\n";
const GOOD_FILTERS: &str = " ... whisper           A->A       Transcribe audio using Whisper.\n ... volume  A->A  x\n";

fn found() -> LookPath {
    Arc::new(|_| Ok("ffmpeg".to_owned()))
}

fn out(version: &'static str, filters: &'static str) -> Probe {
    Arc::new(move |args| {
        let text = if args.last().is_some_and(|a| a == "-filters") {
            filters
        } else {
            version
        };
        (text.to_owned(), Ok(()))
    })
}

fn model(megabytes: u64) -> (tempfile::TempDir, String) {
    let dir = tmp_dir();
    let path = dir.path().join("m.bin");
    std::fs::File::create(&path).unwrap().set_len(megabytes << 20).unwrap();
    let path = path.to_string_lossy().into_owned();
    (dir, path)
}

#[test]
fn check_fehlerfaelle() {
    let (_dir, path) = model(0);
    let c = check(&found(), &out(GOOD_VERSION, GOOD_FILTERS), &path);
    assert!(c.ready && c.ffmpeg == "9.0.2-full_build-www.gyan.dev", "{c:?}");
    let missing: LookPath = Arc::new(|_| Err(Error::Other("nein".to_owned())));
    let c = check(&missing, &out(GOOD_VERSION, GOOD_FILTERS), &path);
    assert!(!c.ready && c.message.contains("ffmpeg nicht gefunden"), "{c:?}");
    let c = check(&found(), &out("ffmpeg version 6.1.1 x\n", ""), &path);
    assert!(!c.ready && c.message.contains("ffmpeg 6.1.1 ist zu alt"), "{c:?}");
    let c = check(&found(), &out("ffmpeg version N-1-gabc\n", ""), &path);
    assert!(c.message.starts_with("ffmpeg (unbekannt) ist zu alt"), "{c:?}");
    let c = check(&found(), &out(GOOD_VERSION, " ... volume A->A x\n"), &path);
    assert!(!c.ready && !c.whisper && c.message.contains("whisper-Filter"), "{c:?}");
}

#[test]
fn check_modell_fehlt() {
    let dir = tmp_dir();
    let path = dir.path().join("fehlt.bin").to_string_lossy().into_owned();
    let c = check(&found(), &out(GOOD_VERSION, GOOD_FILTERS), &path);
    assert!(!c.ready && !c.model_ok, "{c:?}");
    for want in [MODEL_URL, MODEL_SHA, MODEL_ENV] {
        assert!(c.message.contains(want), "{want}: {}", c.message);
    }
    let json = serde_json::to_value(&c).unwrap();
    assert_eq!((&json["modelOk"], &json["modelMb"]), (&false.into(), &0.into()));
}

#[test]
fn check_meldet_modellgroesse() {
    let (_dir, path) = model(3);
    let c = check(&found(), &out(GOOD_VERSION, GOOD_FILTERS), &path);
    assert!(c.ready && c.model_mb == 3, "{c:?}");
}

#[test]
fn model_path_suchreihenfolge() {
    let root = tmp_dir();
    let text = |p: &Path| p.to_string_lossy().into_owned();
    let default = root.path().join("models").join(MODEL_FILE);
    assert_eq!(
        model_path(root.path(), ""),
        text(&default),
        "ohne Datei der erste Kandidat"
    );
    let sub = root.path().join("models").join("whisper");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(sub.join("b.bin"), "x").unwrap();
    std::fs::write(sub.join("a.bin"), "x").unwrap();
    assert_eq!(model_path(root.path(), ""), text(&sub.join("a.bin")));
    std::fs::write(&default, "xx").unwrap();
    assert_eq!(
        model_path(root.path(), ""),
        text(&default),
        "models/<Datei> vor whisper/"
    );
    assert_eq!(model_path(root.path(), "D:/m/x.bin"), "D:/m/x.bin", "Umgebung zuerst");
}
