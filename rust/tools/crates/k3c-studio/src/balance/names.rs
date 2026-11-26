//! Namen, die das Studio vor dem Start eines Laufs prüft (RS2.2, B-726): Bot-Profile, Hilfen und Welten. Die Bots
//! selbst rechnet `k3c-game sim balance`; `names_tests.rs` hält die Liste gleich mit dem Crate `k3c-balance`.

use std::path::Path;

use serde::Deserialize;

use super::datafiles;
use super::report_types::read_go;
use crate::kitx::go_quoted;

/// Profile aus `k3c-balance`, sortiert.
const BOT_NAMES: [&str; 7] = ["coop2", "coop4", "economy", "passive", "saver", "walls", "wirtschaft"];

/// Mindest-Spielerzahl der Profile (fehlt der Name: 1).
pub(crate) const MIN_PLAYERS: [(&str, i64); 2] = [("coop2", 2), ("coop4", 4)];

/// Trennt die Profile einer Mischung je Spieler (B-611): `saver+passive` = Spieler 1 saver, Spieler 2 passive.
pub const MIX_SEP: &str = "+";

/// Hilfen (BS3.4); leer = keine, nur mit Welt.
pub const HELPS_AIDS: &str = "hilfen";
pub const HELPS_FULL: &str = "voll";
pub const HELPS_BOSS: &str = "boss";

/// Die Profilnamen, sortiert.
pub fn bot_names() -> Vec<&'static str> {
    BOT_NAMES.to_vec()
}

pub(crate) fn min_players(name: &str) -> i64 {
    MIN_PLAYERS.iter().find(|(n, _)| *n == name).map_or(0, |(_, n)| *n)
}

/// Ist `name` ein Profil oder eine Mischung je Spieler? coop2 und coop4 gehören in keine Mischung.
pub fn known_bot(name: &str) -> bool {
    let parts: Vec<&str> = name.split(MIX_SEP).collect();
    if parts.len() == 1 {
        return BOT_NAMES.contains(&name);
    }
    parts.iter().all(|n| BOT_NAMES.contains(n) && min_players(n) == 0)
}

/// Fehler, wenn das Profil mehr Spieler braucht, als das Szenario hat, oder eine Mischung nicht genau ein Profil je
/// Spieler nennt.
pub(crate) fn check_players(bot: &str, players: i64) -> Result<(), String> {
    let parts = bot.split(MIX_SEP).count();
    if parts > 1 && i64::try_from(parts) != Ok(players) {
        return Err(format!(
            "mischung {bot} nennt {parts} Profile, Szenario hat {players} Spieler"
        ));
    }
    let n = min_players(bot);
    if players < n {
        return Err(format!("profil {bot} braucht {n} Spieler, Szenario hat {players}"));
    }
    Ok(())
}

/// Die gültigen Werte der Hilfen (ohne den leeren).
pub fn helps_names() -> Vec<&'static str> {
    vec![HELPS_AIDS, HELPS_FULL, HELPS_BOSS]
}

/// Prüft Wert und Zusammenhang mit der Welt.
pub(crate) fn check_helps(helps: &str, world: &str) -> Result<(), String> {
    if helps.is_empty() {
        return Ok(());
    }
    if !helps_names().contains(&helps) {
        return Err(format!(
            "unbekannte hilfen {} (bekannt: [{}])",
            go_quoted(helps),
            helps_names().join(" ")
        ));
    }
    if world.is_empty() {
        return Err(format!("hilfen {} brauchen eine Welt (ganze Insel)", go_quoted(helps)));
    }
    Ok(())
}

/// IDs der Welt-Dateien in `data/worlds` (sortiert), die ein Lauf als Welt nehmen darf.
pub fn worlds(root: &Path) -> Result<Vec<String>, String> {
    Ok(datafiles::read_dir(root, "worlds")?
        .into_iter()
        .filter(|e| !e.is_dir)
        .filter_map(|e| e.name.strip_suffix(".json").map(str::to_owned))
        .collect())
}

/// Der Teil einer Welt-Datei, den Prüfung und Meilenstein-Leiter brauchen; streng prüft `k3c-game sim`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WorldDef {
    pub islands: Vec<WorldIsland>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WorldIsland {
    pub depths: Vec<i64>,
}

/// Liest `data/worlds/<id>.json`; nur IDs vorhandener Dateien.
pub fn load_world(root: &Path, id: &str) -> Result<WorldDef, String> {
    if !worlds(root).unwrap_or_default().iter().any(|w| w == id) {
        return Err(format!("data/worlds/{id}.json fehlt"));
    }
    let raw = datafiles::read(root, &format!("worlds/{id}.json"))?;
    read_go(&raw).map_err(|e| format!("data/worlds/{id}.json: {e}"))
}
