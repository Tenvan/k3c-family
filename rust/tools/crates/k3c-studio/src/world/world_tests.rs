use std::path::Path;
use std::sync::Arc;

use tempfile::TempDir;

use super::values::flatten_json;
use super::{Editor, Engine, Info, ValueEntry, ValuesView};
use crate::exeengine;
use crate::test_helpers::tmp_dir;

/// Fixture: eine Standard-Welt mit einer Insel und eine Kind-Welt.
pub(super) const STANDARD: &str = r#"{
  "id": "standard",
  "name": "Standard",
  "notes": "Fixture",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }, "plannedChunks": { "0": [ { "slot": -7, "kind": "harbor" } ] } }
  ]
}
"#;

const CHILD: &str = r#"{
  "id": "kind",
  "name": "Kind-Welt",
  "extends": "standard",
  "islands": []
}
"#;

/// Editor eines Temp-Checkouts; der Ordner lebt so lange wie der Wert.
pub(super) struct Fixture {
    pub editor: Editor,
    _dir: TempDir,
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// Temp-Checkout mit einer Kopie von `data/` (nur gelesen) und den Fixture-Welten statt der Welten des Repos.
pub(super) fn checkout(engine: impl FnOnce(&Path) -> Engine) -> Fixture {
    let dir = tmp_dir();
    let root = dir.path().to_path_buf();
    let repo = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../.."));
    copy_dir(&repo.join("data"), &root.join("data"));
    let worlds = root.join("data").join("worlds");
    std::fs::remove_dir_all(&worlds).unwrap();
    std::fs::create_dir_all(&worlds).unwrap();
    for (name, text) in [
        ("standard.json", STANDARD),
        ("kind.json", CHILD),
        ("kaputt.txt", "x"),
        ("Ungültig.json", "{}"),
    ] {
        std::fs::write(worlds.join(name), text).unwrap();
    }
    let editor = Editor::new(&root, engine(&root));
    Fixture { editor, _dir: dir }
}

/// Wie [`checkout`] mit der echten Engine: die Spiel-EXE der Repo-Wurzel liest die Fixture-Daten. Ohne EXE `None`.
pub(super) fn fixture() -> Option<Fixture> {
    if !exeengine::require() {
        return None;
    }
    Some(checkout(|root| {
        let root = root.to_path_buf();
        Arc::new(move |input| {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| e.to_string())?;
            runtime.block_on(exeengine::engine("world", &root, input))
        })
    }))
}

/// Engine, die jeden Aufruf mit `error` ablehnt.
pub(super) fn failing(error: &'static str) -> Engine {
    Arc::new(move |_| Err(error.to_owned()))
}

fn info(id: &str, name: &str, kind: &str) -> Info {
    Info {
        id: id.to_owned(),
        name: name.to_owned(),
        missing: kind.is_empty(),
        kind: kind.to_owned(),
        gewaesser: String::new(),
    }
}

#[test]
fn liste_standard_zuerst_und_nur_welten() {
    let Some(f) = fixture() else { return };
    let want = [
        info("standard", "Standard", "new"),
        info("sengoku", "sengoku", ""),
        info("nil", "nil", ""),
        info("britannien", "britannien", ""),
        info("kind", "Kind-Welt", "spinoff"),
    ];
    assert_eq!(f.editor.list(), want);
}

// AC-01 (B-1168): feste Welten ohne Datei fehlen nicht; weitere Dateien stehen dahinter; ohne paper.json bleibt es bei
// den Dateien.
#[test]
fn liste_feste_welten_und_dateien() {
    let Some(f) = fixture() else { return };
    let e = &f.editor;
    std::fs::write(e.file("rom"), r#"{"id":"rom","name":"Nordmeer","islands":[]}"#).unwrap();
    std::fs::write(e.file("nil"), r#"{"id":"nil","name":"Nil","islands":[]}"#).unwrap();
    let ids: Vec<String> = e
        .list()
        .into_iter()
        .map(|w| if w.missing { format!("{}!", w.id) } else { w.id })
        .collect();
    assert_eq!(ids, ["standard", "sengoku!", "nil", "britannien!", "kind", "rom"]);
    std::fs::remove_file(e.root.join("data").join("paper.json")).unwrap();
    let got = e.list();
    assert!(
        got.len() == 4 && got[0].id == "standard" && !got[0].missing,
        "ohne paper.json: {got:?}"
    );
}

#[test]
fn load_inseln_mit_biomen() {
    let Some(f) = fixture() else { return };
    let d = f.editor.load("standard");
    assert!(d.error.is_empty() && d.islands.len() == 1, "{d:?}");
    let island = &d.islands[0];
    assert_eq!(island.biomes, ["forest", "cave", "mine", "ironhold", "crystal"]);
    assert!(island.planned.contains_key("0"), "{island:?}");
}

#[test]
fn load_lehnt_pfade_ausserhalb_des_musters_ab() {
    let Some(f) = fixture() else { return };
    for id in ["../secret", "a/b", ""] {
        let detail = f.editor.load(id);
        assert!(detail.error.contains("ungültige Welt-ID"), "{id:?}: {}", detail.error);
        let level = f.editor.level(id, "island1", 0, "k3c");
        assert!(level.error.contains("ungültige Welt-ID"), "{id:?}: {}", level.error);
    }
}

#[test]
fn level_listen_nie_null() {
    let Some(mut f) = fixture() else { return };
    let v = f.editor.level("standard", "island1", 0, "k3c");
    assert!(v.error.is_empty(), "{}", v.error);
    assert!(v.chunks.len() >= 3 && v.planned.len() == 1, "{v:?}");
    f.editor.engine = failing("die Insel \"x\" gibt es nicht");
    let bad = f.editor.level("standard", "x", 0, "k3c");
    assert!(!bad.error.is_empty());
    let json = serde_json::to_value(&bad).unwrap();
    for key in [
        "chunks",
        "entities",
        "planned",
        "warnings",
        "natureTraits",
        "natureWeights",
    ] {
        assert_eq!(json[key], serde_json::json!([]), "{key}");
    }
    for key in ["nature", "natureFixed"] {
        assert_eq!(json[key], serde_json::json!({}), "{key}");
    }
}

/// Text der Welt-Datei.
pub(super) fn read_world(e: &Editor, id: &str) -> String {
    std::fs::read_to_string(e.file(id)).unwrap()
}

pub(super) fn entry<'a>(v: &'a ValuesView, file: &str, path: &[&str]) -> &'a ValueEntry {
    v.entries
        .iter()
        .find(|e| e.file == file && e.path == path)
        .unwrap_or_else(|| panic!("Feld {file} {path:?} fehlt"))
}

#[test]
fn werte_zeigen_basis_und_nachfolger() {
    let Some(f) = fixture() else { return };
    let v = f.editor.values("standard");
    assert!(v.error.is_empty(), "{}", v.error);
    assert_eq!(v.children, ["kind"]);
    let start = entry(&v, "economy", &["purse", "startGold"]);
    assert_eq!(start.base.get(), "14");
    assert!(start.override_value.is_none());
    entry(&v, "economy", &["chestGold"]);
}

#[test]
fn flatten_listen_je_id_und_index() {
    let mut got = Vec::new();
    let raw = r#"{"_c":1,"a":[{"id":"x","v":1},{"v":2}],"b":[{"v":3}],"c":[1,2]}"#;
    flatten_json(raw, &[], &mut |p, v| got.push(format!("{}={}", p.join("."), v.get())));
    assert_eq!(got, ["a.x.v=1", "b.0.v=3", "c=[1,2]"]);
}
