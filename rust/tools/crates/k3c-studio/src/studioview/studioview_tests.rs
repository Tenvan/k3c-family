//! Port von `studioview_test.go` (WB1.4): Sound und Quellen nur lesend aus `data/sounds.json`,
//! `data/assets-recherche.json` und dem Katalog unter `public/`.

use std::path::Path;

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;

use super::{sound_data, sounds, sources};
use crate::test_helpers::tmp_dir;

pub(super) fn write(root: &Path, rel: &str, content: &str) {
    let mut full = root.to_path_buf();
    full.extend(rel.split('/'));
    std::fs::create_dir_all(full.parent().unwrap()).unwrap();
    std::fs::write(full, content).unwrap();
}

fn asset_root() -> tempfile::TempDir {
    let tmp = tmp_dir();
    let root = tmp.path();
    write(
        root,
        "data/sounds.json",
        r#"{"_comment": "", "slots": [
        {"id": "music:tag", "art": "sound", "kategorie": "musik", "beschreibung": "Tag", "bus": "music", "wahl": [{"datei": "audio/tag.ogg"}]},
        {"id": "sfx:fehlt", "art": "sound", "kategorie": "sfx", "beschreibung": "Fehlt", "wahl": [{"datei": "audio/weg.ogg"}]},
        {"id": "sfx:luecke", "art": "sound", "kategorie": "sfx", "beschreibung": "Lücke", "wahl": []},
        {"id": "monarch", "art": "grafik", "kategorie": "figuren", "beschreibung": "Monarch", "wahl": []}
    ]}"#,
    );
    write(root, "public/audio/tag.ogg", "OggS-probe");
    write(
        root,
        "data/assets-recherche.json",
        r#"{"_comment": "", "treffer": {"music:tag": [
        {"id": "h1", "titel": "Morgen", "link": "https://example.org/m", "quelle": "OpenGameArt", "lizenz": "CC0", "lizenzStatus": "erkannt", "erstellt": "2026-11-01"}
    ]}}"#,
    );
    tmp
}

#[test]
fn sounds_listet_nur_sound_slots_mit_status() {
    let tmp = asset_root();
    let list = sounds(tmp.path()).unwrap();
    assert_eq!(list.len(), 3, "{list:?}");
    let get = |id: &str| list.iter().find(|s| s.id == id).unwrap();
    let day = get("music:tag");
    assert_eq!(
        (day.status.as_str(), day.kategorie.as_str(), day.bus.as_str()),
        ("zugeordnet", "musik", "music")
    );
    assert_eq!(day.dateien, ["audio/tag.ogg"]);
    assert_eq!(get("sfx:fehlt").status, "fehlt");
    assert_eq!(get("sfx:luecke").status, "luecke");
    // JSON wie Go: leerer Bus fehlt, die Liste ist nie null.
    let json = serde_json::to_value(get("sfx:luecke")).unwrap();
    assert!(
        json.get("bus").is_none() && json["dateien"] == serde_json::json!([]),
        "{json}"
    );
}

#[test]
fn sound_data_nur_audio_unter_public() {
    let tmp = asset_root();
    let root = tmp.path();
    write(root, "geheim.ogg", "x");
    write(root, "public/grafik/bild.png", "x");
    let url = sound_data(root, "audio/tag.ogg").unwrap();
    let data = url.strip_prefix("data:audio/ogg;base64,").expect("Data-URL");
    assert_eq!(STANDARD.decode(data).unwrap(), b"OggS-probe");
    for bad in [
        "../geheim.ogg",
        "/etc/x.ogg",
        r"audio\tag.ogg",
        "C:/x.ogg",
        "audio/../../geheim.ogg",
        "audio//tag.ogg",
        "./audio/tag.ogg",
        "grafik/bild.png",
        "audio/weg.ogg",
        ".",
        "",
    ] {
        assert!(sound_data(root, bad).is_err(), "{bad:?} angenommen");
    }
    assert_eq!(
        sound_data(root, "audio/weg.ogg"),
        Err("audio/weg.ogg fehlt unter public/".into())
    );
    assert_eq!(
        sound_data(root, "grafik/bild.png"),
        Err(r#"Nur Audio (.ogg, .mp3, .wav, .m4a), nicht "grafik/bild.png""#.into())
    );
}

#[test]
fn sound_data_begrenzt_die_groesse() {
    let tmp = tmp_dir();
    write(tmp.path(), "public/gross.wav", &"x".repeat((10 << 20) + 1));
    assert_eq!(
        sound_data(tmp.path(), "gross.wav"),
        Err("gross.wav ist größer als 10 MB".into())
    );
}

#[test]
fn sources_katalog_und_treffer() {
    let tmp = asset_root();
    let q = sources(tmp.path()).unwrap();
    let hits = &q.treffer["music:tag"];
    assert_eq!(hits.len(), 1);
    assert_eq!((hits[0].title.as_str(), hits[0].license.as_str()), ("Morgen", "CC0"));
}

#[test]
fn sources_ohne_recherche() {
    let tmp = tmp_dir();
    let q = sources(tmp.path()).unwrap();
    assert!(q.treffer.is_empty() && q.katalog.is_empty());
    assert_eq!(
        serde_json::to_value(q).unwrap(),
        serde_json::json!({ "katalog": [], "treffer": {} })
    );
}
