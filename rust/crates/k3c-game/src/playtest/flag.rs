//! Schalter `--playtest <ordner>` (B-1220): wird vor den übrigen Schaltern des Spiels herausgelöst, damit
//! `game::scene::parse_args` ihn nicht kennen muss.

use std::path::PathBuf;

use super::marker::JANK_MS;

/// Die Einstellungen des Spieltests aus der Kommandozeile.
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub dir: PathBuf,
    /// Schwelle des Ruckler-Markers in ms (`--playtest-jank-ms`, Standard 25)
    pub jank_ms: f64,
}

/// Löst `--playtest <ordner>` und `--playtest-jank-ms <ms>` (auch `-playtest`, `--playtest=<ordner>`) aus `rest` und gibt den Ordner (sonst `None`)
/// und die übrigen Argumente zurück. Ohne Ordner ist es ein Fehler.
pub fn extract(rest: &[String]) -> Result<(Option<Options>, Vec<String>), String> {
    let mut dir = None;
    let mut jank_ms = JANK_MS;
    let mut out = Vec::with_capacity(rest.len());
    let mut it = rest.iter();
    while let Some(a) = it.next() {
        if a == "--" {
            out.push(a.clone());
            out.extend(it.by_ref().cloned());
            break;
        }
        let name = a.trim_start_matches('-');
        let dashes = a.len() - name.len();
        if !(1..=2).contains(&dashes) {
            out.push(a.clone());
            continue;
        }
        if name == "playtest-jank-ms" || name.starts_with("playtest-jank-ms=") {
            let v = name
                .strip_prefix("playtest-jank-ms=")
                .map(str::to_string)
                .or_else(|| it.next().cloned());
            jank_ms = v
                .and_then(|v| v.parse::<f64>().ok())
                .filter(|v| *v > 0.0)
                .ok_or("--playtest-jank-ms braucht eine Zahl in ms über 0")?;
            continue;
        }
        let value = if name == "playtest" {
            it.next().cloned()
        } else if let Some(v) = name.strip_prefix("playtest=") {
            Some(v.to_string())
        } else {
            out.push(a.clone());
            continue;
        };
        match value.filter(|v| !v.is_empty()) {
            Some(v) => dir = Some(PathBuf::from(v)),
            None => return Err("--playtest braucht einen Ordner".to_string()),
        }
    }
    Ok((dir.map(|dir| Options { dir, jank_ms }), out))
}
