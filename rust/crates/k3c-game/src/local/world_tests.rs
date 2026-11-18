use std::path::Path;

use super::testutil::{TempDir, test_session};
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::session::{NewGame, Session as _};

// B-938: WorldList (ohne Dev-Mode nur die Standard-Welt) und World (laufendes Spiel); der Spielstand trägt seine Welt.
#[test]
#[ignore = "wartet auf RU27 (world_list, world, list: RU27.3) und RU19 (decode_world_story: new_game braucht die Standard-Welt)"]
fn world_list_and_world_of_running_game() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    let l = s.world_list();
    assert!(
        l.len() == 1 && l[0].id == "standard" && !l[0].name.is_empty(),
        "Weltliste {l:?}"
    );
    assert_eq!(s.world(), "", "Welt ohne Spiel");
    let g = NewGame {
        name: "welt-1".into(),
        world: "standard".into(),
        devices: vec![DeviceID::new(DeviceKind::Keyboard, 0)],
        ..NewGame::default()
    };
    s.new_game(g).expect("neues Spiel");
    assert_eq!(s.world(), "standard", "Welt im Spiel");
    s.close().expect("beenden");
    let list = s.list().expect("Liste");
    assert!(
        list.len() == 1 && list[0].loadable && list[0].world == "standard",
        "Spielstände {list:?}"
    );
    s.load("welt-1", &[]).expect("laden");
    assert_eq!(s.world(), "standard", "Welt nach Laden");
}

// Lokale Welten erscheinen nur im Dev-Mode und nur, wenn der Ordner sie enthält (K3C_WORLDS_DIR). Go setzt die
// Umgebungsvariable im Test; Rust verbietet `set_var` (unsafe), darum setzt der Test den Ordner der Session (`worlds_dir`,
// den `Session::open` aus `worlds_fs()` liest).
#[test]
#[ignore = "wartet auf RU27 (world_list: RU27.3)"]
fn world_list_shows_local_worlds_in_dev_mode() {
    let dir = TempDir::new();
    let std_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/worlds/standard.json");
    let std = std::fs::read_to_string(std_file).expect("Standard-Welt");
    let raw = std.replacen(r#""id": "standard""#, r#""id": "lokal""#, 1);
    let raw = raw.replacen(r#""name": "Standard""#, r#""name": "Lokal""#, 1);
    std::fs::write(dir.path().join("lokal.json"), raw).expect("schreiben");
    let saves = TempDir::new();
    let mut s = test_session(&saves);
    s.worlds_dir = Some(dir.path().to_path_buf());
    let l = s.world_list();
    assert_eq!(l.len(), 1, "ohne Dev-Mode: {l:?}");
    s.settings.dev_mode = true;
    let l = s.world_list();
    assert!(l.len() == 2 && l[1].id == "lokal", "im Dev-Mode: {l:?}");
}
