//! `saves_list` (Port von `saves.go`).

use std::path::{Component, Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use super::{json_files, local_minute, read_json};

/// Verlegt den Spielstand-Ordner; ein relativer Pfad gilt ab der Repo-Wurzel.
pub const ENV_SAVES_DIR: &str = "K3C_SAVES_DIR";

/// Der Teil eines Spielstands, den `saves_list` zeigt.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Save {
    saved_at: String,
    depth: i64,
    unlocked_depth: i64,
    /// Sekunden der Kampagne.
    time: f64,
    players: Vec<Value>,
}

/// Spielstand-Ordner: `env` (Wert von [`ENV_SAVES_DIR`]) oder `saves/` in der Repo-Wurzel.
pub fn saves_dir(root: &Path, env: &str) -> PathBuf {
    if env.is_empty() {
        return root.join("saves");
    }
    let path = Path::new(env);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    // Wie Go `filepath.Join`: auch ein Wert mit führendem Trenner bleibt unter der Wurzel.
    let relative: PathBuf = path
        .components()
        .filter(|c| !matches!(c, Component::RootDir | Component::Prefix(_)))
        .collect();
    root.join(relative)
}

/// `saves_list`: eine Zeile je Datei im Spielstand-Ordner, Sicherungen eingeschlossen.
pub fn saves_list(root: &Path, dir: &Path) -> Result<String, String> {
    let files = json_files(dir);
    if files.is_empty() {
        return Ok("keine Spielstände".to_owned());
    }
    let mut lines = Vec::new();
    let cycle = cycle_seconds(root);
    if let Err(e) = &cycle {
        lines.push(format!("Tag unbekannt: {e}"));
    }
    let cycle = cycle.ok();
    lines.extend(files.iter().map(|(name, path)| save_line(name, path, cycle)));
    Ok(lines.join("\n"))
}

/// Das Morgengrauen (`dawn` Sekunden am Ende des Zyklus) gehört schon zum nächsten Tag.
fn save_line(name: &str, path: &Path, cycle: Option<(f64, f64)>) -> String {
    let slot = name.strip_suffix(".json").unwrap_or(name);
    let Ok(save) = read_json::<Save>(path) else {
        return format!("{slot} · nicht lesbar");
    };
    let day = cycle.map_or_else(
        || "?".to_owned(),
        |(length, dawn)| (((save.time + dawn) / length) as i64 + 1).to_string(),
    );
    let when = local_minute(&save.saved_at).unwrap_or_else(|| save.saved_at.clone());
    format!(
        "{slot} · Stufe {}/{} · Tag {day} · {} Spieler · {when}",
        save.depth,
        save.unlocked_depth,
        save.players.len()
    )
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Biome {
    cycle: Cycle,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Cycle {
    day_minutes: f64,
    twilight_minutes: f64,
    night_minutes: f64,
    dawn_minutes: f64,
}

/// Länge eines Tag/Nacht-Zyklus und des Morgengrauens aus `data/biomes/forest.json`, dem Biom der Oberwelt. Die Werte
/// stehen nur in `data/`, nicht im Code.
fn cycle_seconds(root: &Path) -> Result<(f64, f64), String> {
    let path = root.join("data").join("biomes").join("forest.json");
    if !path.is_file() {
        return Err(format!("open {}: Datei fehlt", path.display()));
    }
    let biome: Biome = read_json(&path).map_err(|e| format!("forest.json: {e}"))?;
    let c = biome.cycle;
    let length = (c.day_minutes + c.twilight_minutes + c.night_minutes + c.dawn_minutes) * 60.0;
    if length <= 0.0 {
        return Err("forest.json: Zyklus ohne Länge".to_owned());
    }
    Ok((length, c.dawn_minutes * 60.0))
}
