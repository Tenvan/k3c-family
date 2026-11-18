//! Port von `native/k3c-game/local/worlds.go`: Woher die Welten kommen. Die Standard-Welt steckt in den Spieldaten
//! (`k3c_data`, eingebettet oder `K3C_DATA_DIR`), weitere Welten liegen als `<id>.json` in einem Ordner und gelten nur im
//! Dev-Mode (`docs/rules/welten.md` § 6).

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use k3c_data::{Data, Source, load_all};

use super::error::{UserError, user_err};

/// Nennt den Ordner der Welt-Dateien (wie `Manager::worlds`, `K3C_WORLDS_DIR`); ohne ihn gilt `data/worlds`, falls es neben
/// dem Arbeitsordner liegt. Gelesen wird er nur im Dev-Mode (`Settings::dev_mode`), sonst gilt nur die Standard-Welt.
pub const WORLDS_DIR_ENV: &str = "K3C_WORLDS_DIR";

/// Liefert den Ordner der lokalen Welten oder `None`, wenn es keinen gibt. Go liefert einen `fs.FS`; Rust braucht nur den
/// Pfad. Die eingebetteten Daten enthalten nur die Standard-Welt, die `load_world_def` ohnehin aus den Spieldaten liest.
pub fn worlds_fs() -> Option<PathBuf> {
    let env = std::env::var(WORLDS_DIR_ENV).unwrap_or_default();
    let dir = if env.is_empty() {
        PathBuf::from("data/worlds")
    } else {
        PathBuf::from(env)
    };
    dir.is_dir().then_some(dir)
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
