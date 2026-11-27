//! Port von `app_studio_test.go` (WB1.4): was vom Asset-Studio blieb, ist lesend; „Asset-Studio öffnen“ (`StudioOpen`)
//! gibt es nicht mehr.

use serde_json::{Value, json};

use crate::app::{App, rpc};
use crate::test_helpers::tmp_dir;

fn studio_root() -> tempfile::TempDir {
    let tmp = tmp_dir();
    for (rel, content) in [
        (
            "data/sounds.json",
            r#"{"_comment": "", "slots": [{"id": "music:tag", "art": "sound", "kategorie": "musik", "beschreibung": "Tag", "wahl": [{"datei": "audio/tag.ogg"}]}]}"#,
        ),
        ("public/audio/tag.ogg", "OggS"),
    ] {
        let full = tmp.path().join(rel);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(full, content).unwrap();
    }
    tmp
}

fn app(root: &tempfile::TempDir) -> App {
    let app = App::new(root.path().to_path_buf(), 5281, None);
    app.mark_ready();
    app
}

fn call(app: &App, method: &str, args: &[Value]) -> Result<Value, String> {
    rpc::call(app, method, args)
}

#[test]
fn studio_lese_bindings() {
    let root = studio_root();
    let app = app(&root);
    let sounds = call(&app, "StudioSounds", &[]).unwrap();
    assert_eq!(
        sounds,
        json!([{ "id": "music:tag", "kategorie": "musik", "beschreibung": "Tag", "status": "zugeordnet", "dateien": ["audio/tag.ogg"] }])
    );
    let data = call(&app, "StudioSoundData", &[json!("audio/tag.ogg")]).unwrap();
    assert!(
        data.as_str().unwrap().starts_with("data:audio/ogg;base64,"),
        "Hörprobe {data}"
    );
    assert!(
        call(&app, "StudioSoundData", &[json!("../data/sounds.json")]).is_err(),
        "Pfad außerhalb von public/ angenommen"
    );
    let sources = call(&app, "StudioSources", &[]).unwrap();
    assert!(sources["katalog"].is_array(), "Quellen {sources}");
    assert!(call(&app, "StudioSoundData", &[]).is_err(), "ohne Argument angenommen");
}

#[test]
fn studio_game_open_ohne_client_nennt_build() {
    let root = studio_root();
    let app = app(&root);
    let error = call(&app, "StudioGameOpen", &[json!("abc")]).unwrap_err();
    assert!(error.contains("task build"), "ohne native EXE: {error}");
    app.close_studio();
}

#[tokio::test]
async fn beenden_schliesst_im_spiel_ansehen() {
    let root = studio_root();
    let app = app(&root);
    app.shutdown().await;
    let error = app.studio_game_open("abc").unwrap_err();
    assert!(error.contains("beendet"), "{error}");
}

#[test]
fn studio_open_entfernt() {
    let names: Vec<&str> = rpc::METHODS.iter().map(|(name, _)| *name).collect();
    assert!(!names.contains(&"StudioOpen"), "StudioOpen ist noch eine Methode");
    for method in ["StudioSounds", "StudioSoundData", "StudioSources", "StudioGameOpen"] {
        assert!(names.contains(&method), "Methode {method} fehlt");
    }
}
