//! Port von `native/k3c-game/local/worlds.go`: Woher die Welten kommen. Die Standard-Welt steckt in den Spieldaten
//! (`k3c_data`, eingebettet oder `K3C_DATA_DIR`), weitere Welten liegen als `<id>.json` in einem Ordner und gelten nur im
//! Dev-Mode (`docs/rules/welten.md` § 6).

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use k3c_data::{Data, Source, load_all};

use super::error::{UserError, user_err};

/// Nennt den Ordner der Welt-Dateien (wie `Manager::worlds`, `K3C_WORLDS_DIR`); ohne ihn gilt `data/worlds` neben der EXE
/// (RU36.6: unabhängig vom Arbeitsordner), zuletzt `data/worlds` im Arbeitsordner. Gelesen wird er nur im Dev-Mode
/// (`Settings::dev_mode`), sonst gilt nur die Standard-Welt.
pub const WORLDS_DIR_ENV: &str = "K3C_WORLDS_DIR";

/// Liefert den Ordner der lokalen Welten oder `None`, wenn es keinen gibt. Go liefert einen `fs.FS`; Rust braucht nur den
/// Pfad. Die eingebetteten Daten enthalten nur die Standard-Welt, die `load_world_def` ohnehin aus den Spieldaten liest.
pub fn worlds_fs() -> Option<PathBuf> {
    let env = std::env::var(WORLDS_DIR_ENV).unwrap_or_default();
    let exe = std::env::current_exe().ok();
    pick_worlds_dir(&env, exe.as_deref().and_then(Path::parent))
}

/// Die Wahl des Ordners: `env` (gesetzt: nur dieser, fail closed), sonst `data/worlds` neben der EXE `exe_dir`, sonst
/// `data/worlds` im Arbeitsordner.
pub fn pick_worlds_dir(env: &str, exe_dir: Option<&Path>) -> Option<PathBuf> {
    if !env.is_empty() {
        let dir = PathBuf::from(env);
        return dir.is_dir().then_some(dir);
    }
    exe_dir
        .map(|d| d.join("data").join("worlds"))
        .into_iter()
        .chain([PathBuf::from("data/worlds")])
        .find(|d| d.is_dir())
}

/// Die Spieldaten, einmal je Prozess gelesen (Go: `data.Files` beim Start). Quelle wie Go: `K3C_DATA_DIR`, sonst `data`
/// neben der EXE, sonst eingebettet.
pub fn game_data() -> Result<Arc<Data>, UserError> {
    static DATA: OnceLock<Result<Arc<Data>, String>> = OnceLock::new();
    DATA.get_or_init(|| load_all(&Source::from_env()).map(Arc::new).map_err(|e| e.to_string()))
        .clone()
        .map_err(|e| user_err(format!("Spieldaten nicht lesbar: {e}"), None))
}

/// Der Pfad der Welt-Datei `id` im Ordner `dir`; `None` bei einer ungültigen ID (verhindert Pfade außerhalb von `dir`).
pub fn world_file(dir: &Path, id: &str) -> Option<PathBuf> {
    k3c_data::world::valid_id(id).then(|| dir.join(format!("{id}.json")))
}

/// Die Quelle der weiteren Welten für `Manager::set_worlds`: Der Manager liest `worlds/<id>.json` aus einem Datenordner
/// (`Source::Dir`), Go gab ihm den Ordner der Dateien selbst (`fs.FS`). Heißt `dir` darum `worlds` (wie `data/worlds`),
/// ist der Datenordner sein Elternordner; jeder andere Name ergibt `None` (keine weiteren Welten, fail closed).
pub fn worlds_source(dir: Option<&Path>) -> Option<k3c_data::Source> {
    let dir = dir?;
    if dir.file_name()? != "worlds" {
        return None;
    }
    let parent = dir
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    Some(Source::Dir(parent.to_path_buf()))
}
