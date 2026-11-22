use std::path::Path;

use super::testutil::{TempDir, test_session};
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::session::{NewGame, Session as _};

// B-938: WorldList (ohne Dev-Mode nur die Standard-Welt) und World (laufendes Spiel); der Spielstand trägt seine Welt.
#[test]
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
// Umgebungsvariable im Test; Rust verbietet `set_var` (unsafe), darum setzt der Test den Ordner der Session (`worlds_dir`, er muss `worlds` heißen,
// den `Session::open` aus `worlds_fs()` liest).
#[test]
fn world_list_shows_local_worlds_in_dev_mode() {
    let dir = TempDir::new();
    let std_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/worlds/standard.json");
    let std = std::fs::read_to_string(std_file).expect("Standard-Welt");
    let raw = std.replacen(r#""id": "standard""#, r#""id": "lokal""#, 1);
    let raw = raw.replacen(r#""name": "Standard""#, r#""name": "Lokal""#, 1);
    std::fs::create_dir_all(dir.path().join("worlds")).expect("Ordner");
    std::fs::write(dir.path().join("worlds/lokal.json"), raw).expect("schreiben");
    let saves = TempDir::new();
    let mut s = test_session(&saves);
    s.worlds_dir = Some(dir.path().join("worlds"));
    let l = s.world_list();
    assert_eq!(l.len(), 1, "ohne Dev-Mode: {l:?}");
    s.settings.dev_mode = true;
    let l = s.world_list();
    assert!(l.len() == 2 && l[1].id == "lokal", "im Dev-Mode: {l:?}");
}

/// RU36.6 (B-1165): Der Welten-Ordner kommt von der Umgebung, sonst neben der EXE, egal von wo das Spiel startet.
#[test]
fn welten_ordner_neben_der_exe_und_env() {
    use super::worlds::pick_worlds_dir;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/ru36-6-ordner");
    let _ = std::fs::remove_dir_all(&root);
    let exe = root.join("exe");
    std::fs::create_dir_all(exe.join("data/worlds")).unwrap();
    std::fs::create_dir_all(root.join("env")).unwrap();
    assert_eq!(
        pick_worlds_dir("", Some(&exe)),
        Some(exe.join("data").join("worlds")),
        "neben der EXE"
    );
    assert_eq!(
        pick_worlds_dir(root.join("env").to_str().unwrap(), Some(&exe)),
        Some(root.join("env")),
        "Umgebung gewinnt"
    );
    assert_eq!(
        pick_worlds_dir(root.join("fehlt").to_str().unwrap(), Some(&exe)),
        None,
        "gesetzt, aber fehlt: keiner"
    );
    let _ = std::fs::remove_dir_all(&root);
}

/// B-1210 AC-02: Liegt die EXE in `<repo>/dist/k3c-game` (Repo an `Taskfile.yml` und `.git`), gelten die Repo-Welten;
/// eine kopierte Paketfassung liest ihre eigenen; fehlen die Repo-Welten, gilt das Paket (Rückfall).
#[test]
fn repo_welten_vor_paket_welten() {
    use super::worlds::{WorldsDir, pick_worlds};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/wl1-1-repo");
    let _ = std::fs::remove_dir_all(&root);
    let repo = root.join("repo");
    let exe = repo.join("dist").join("k3c-game");
    std::fs::create_dir_all(exe.join("data/worlds")).unwrap();
    std::fs::create_dir_all(repo.join(".git")).unwrap();
    std::fs::write(repo.join("Taskfile.yml"), "version: '3'").unwrap();
    let paket = WorldsDir {
        dir: Some(exe.join("data").join("worlds")),
        repo: false,
    };
    assert_eq!(
        pick_worlds("", Some(&exe)),
        paket,
        "Repo ohne data/worlds: Rückfall auf das Paket"
    );
    std::fs::create_dir_all(repo.join("data/worlds")).unwrap();
    let want = WorldsDir {
        dir: Some(repo.join("data").join("worlds")),
        repo: true,
    };
    assert_eq!(pick_worlds("", Some(&exe)), want, "Start aus dem Repo-dist");
    let kopie = root.join("kopie").join("k3c-game");
    std::fs::create_dir_all(kopie.join("data/worlds")).unwrap();
    let want = WorldsDir {
        dir: Some(kopie.join("data").join("worlds")),
        repo: false,
    };
    assert_eq!(pick_worlds("", Some(&kopie)), want, "kopiertes Paket");
    let env = root.join("env").join("worlds");
    std::fs::create_dir_all(&env).unwrap();
    let got = pick_worlds(env.to_str().unwrap(), Some(&exe));
    assert!(got.dir == Some(env) && !got.repo, "Umgebung gewinnt: {got:?}");
    let _ = std::fs::remove_dir_all(&root);
}

/// B-1210 AC-03: Kommen die Welten aus dem Repo, tragen die Einträge aus dem Ordner das Dev-Kennzeichen.
#[test]
fn world_list_marks_repo_worlds() {
    let dir = TempDir::new();
    let std_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/worlds/standard.json");
    let raw = std::fs::read_to_string(std_file).expect("Standard-Welt");
    let raw = raw.replacen(r#""id": "standard""#, r#""id": "lokal""#, 1);
    std::fs::create_dir_all(dir.path().join("worlds")).expect("Ordner");
    std::fs::write(dir.path().join("worlds/lokal.json"), raw).expect("schreiben");
    let saves = TempDir::new();
    let mut s = test_session(&saves);
    s.worlds_dir = Some(dir.path().join("worlds"));
    s.settings.dev_mode = true;
    let l = s.world_list();
    assert!(
        l.len() == 2 && !l[0].repo && !l[1].repo,
        "Paket-Welten ohne Kennzeichen: {l:?}"
    );
    s.worlds_repo = true;
    let l = s.world_list();
    assert!(
        l.len() == 2 && !l[0].repo && l[1].repo,
        "Repo-Welten mit Kennzeichen: {l:?}"
    );
}
