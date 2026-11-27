//! Port von `game_test.go` (B-847): „Im Spiel ansehen“ legt den Testspielstand `voll-<seed>` an und startet das native
//! Spiel mit festem Pfad, das ihn lädt; ohne EXE nennt die Meldung den Build-Task. Weder Spiel noch EXE starten wirklich.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use k3c_kit::logger::Logger;
use serde_json::Value;

use super::tests::write;
use super::{Game, default_saves_dir};
use crate::gameexe::{self, Event, Options};
use crate::test_helpers::tmp_dir;

const BUILD: &str = "rust/target/release/k3c-game.exe";

/// Ein Start: Programm, Arbeitsordner, Argumente.
type Start = (PathBuf, PathBuf, Vec<String>);
type Starts = Arc<Mutex<Vec<Start>>>;

/// Ein Checkout; `exe` legt eine native EXE (relativ zur Wurzel) an.
fn game_root(exe: &str) -> tempfile::TempDir {
    let tmp = tmp_dir();
    if !exe.is_empty() {
        write(tmp.path(), exe, "MZ");
    }
    tmp
}

/// Ersetzt `k3c-game sim save`: legt `<saves>/voll-<seed>.json` an und meldet den Namen.
fn fake_save(o: &Options) -> Result<Event, gameexe::Error> {
    assert_eq!(o.sub, "save");
    let input: Value = serde_json::from_slice(&o.input).unwrap();
    let name = format!("voll-{}", input["seed"].as_str().unwrap());
    let saves = PathBuf::from(input["saves"].as_str().unwrap());
    std::fs::write(saves.join(format!("{name}.json")), "{}")?;
    Ok(Event {
        event: "result".to_owned(),
        text: name,
        ..Event::default()
    })
}

/// `Game` mit Testspielständen in einem Temp-Ordner; gestartete Befehle landen in der Liste statt im System.
fn new_game(root: &Path) -> (Game, tempfile::TempDir, Starts) {
    let saves = tmp_dir();
    let mut game = Game::new(root, Some(saves.path().to_path_buf()), Logger::discard());
    let starts: Starts = Arc::default();
    let list = Arc::clone(&starts);
    game.launch = Box::new(move |cmd: &mut Command| {
        list.lock().unwrap().push((
            cmd.get_program().into(),
            cmd.get_current_dir().unwrap().into(),
            cmd.get_args().map(|a| a.to_string_lossy().into_owned()).collect(),
        ));
        Ok(())
    });
    game.call = Box::new(|o, _| fake_save(&o));
    (game, saves, starts)
}

/// AC-01: Die Befehlszeile zeigt auf die native EXE mit festem Pfad und lädt `voll-<seed>`.
#[test]
fn startet_natives_spiel() {
    let tmp = game_root(BUILD);
    let root = tmp.path();
    let (game, saves, starts) = new_game(root);
    let message = game.open("abc").unwrap();
    assert!(message.contains("voll-abc"), "{message}");
    {
        let starts = starts.lock().unwrap();
        assert_eq!(starts.len(), 1);
        let (exe, dir, args) = &starts[0];
        assert_eq!(
            exe,
            &root.join("rust").join("target").join("release").join("k3c-game.exe")
        );
        assert_eq!(dir.as_os_str(), root.as_os_str());
        let saves = saves.path().to_string_lossy();
        assert_eq!(args, &["--saves", &*saves, "--load", "voll-abc", "--players", "1"]);
    }
    assert!(saves.path().join("voll-abc.json").exists(), "Testspielstand fehlt");
    assert!(!root.join("saves").exists(), "saves/ im Checkout angelegt");
    // EXE läuft schon: ein zweiter Start ist erlaubt.
    game.open("abc").unwrap();
    assert_eq!(starts.lock().unwrap().len(), 2);
}

/// Ohne Build im Checkout gilt das Spiel-Paket `dist/k3c-game` mit seinem Ordner als Arbeitsordner.
#[test]
fn spiel_paket() {
    let tmp = game_root("dist/k3c-game/k3c-game.exe");
    let (game, _saves, starts) = new_game(tmp.path());
    game.open("abc").unwrap();
    let starts = starts.lock().unwrap();
    let (exe, dir, _) = &starts[0];
    assert_eq!(dir, &tmp.path().join("dist").join("k3c-game"));
    assert_eq!(exe.file_name().unwrap(), "k3c-game.exe");
}

/// AC-02: Fehlt die EXE, nennt die Meldung `task build` und startet nichts; ungültige Seeds lehnt `open` ab.
#[test]
fn ablehnungen() {
    let empty = game_root("");
    let (game, saves, starts) = new_game(empty.path());
    let error = game.open("abc").unwrap_err();
    assert!(error.contains("task build"), "ohne EXE: {error}");
    assert!(
        !saves.path().join("voll-abc.json").exists(),
        "Spielstand trotz fehlender EXE"
    );
    let tmp = game_root(BUILD);
    let (mut game, _saves, _) = new_game(tmp.path());
    let long = "a".repeat(28);
    for seed in ["", "Abc", "a b", "../x", "a\n", long.as_str()] {
        let error = game.open(seed).unwrap_err();
        assert!(error.starts_with("Seed muss"), "Seed {seed:?}: {error}");
    }
    game.launch = Box::new(|_| Err(std::io::Error::other("kaputt")));
    assert_eq!(
        game.open("abc"),
        Err("Natives Spiel nicht gestartet: kaputt".to_owned())
    );
    assert!(starts.lock().unwrap().is_empty(), "ohne EXE gestartet");
}

#[test]
fn fehler_von_sim_save_werden_hinweise() {
    let tmp = game_root(BUILD);
    let (mut game, _saves, starts) = new_game(tmp.path());
    game.call = Box::new(|_, _| Err(gameexe::Error::Msg("Exit 2".to_owned())));
    assert_eq!(game.open("abc"), Err("Seed ungültig".to_owned()));
    game.call = Box::new(|_, _| Err(gameexe::Error::Msg("kaputt".to_owned())));
    assert_eq!(game.open("abc"), Err("Spielstand nicht angelegt: kaputt".to_owned()));
    game.call = Box::new(|_, _| Ok(Event::default()));
    let error = game.open("abc").unwrap_err();
    assert_eq!(error, "Spielstand nicht angelegt: Spiel-EXE meldet keinen Namen");
    assert!(starts.lock().unwrap().is_empty());
}

/// Nach `close` startet `open` nichts mehr; doppeltes `close` ist harmlos.
#[test]
fn open_nach_close_abgelehnt() {
    let tmp = game_root(BUILD);
    let (game, _saves, _) = new_game(tmp.path());
    game.open("abc").unwrap();
    game.close();
    game.close();
    let error = game.open("abc").unwrap_err();
    assert!(error.contains("beendet"), "open nach close: {error}");
}

#[test]
fn spielstaende_unter_work() {
    let want = Path::new("r").join(".work").join("studioview").join("saves");
    assert_eq!(default_saves_dir(Path::new("r")), want);
}
