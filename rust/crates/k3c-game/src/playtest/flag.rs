//! Schalter `--playtest <ordner>` (B-1220): wird vor den übrigen Schaltern des Spiels herausgelöst, damit
//! `game::scene::parse_args` ihn nicht kennen muss.

use std::path::PathBuf;

/// Löst `--playtest <ordner>` (auch `-playtest`, `--playtest=<ordner>`) aus `rest` und gibt den Ordner (sonst `None`)
/// und die übrigen Argumente zurück. Ohne Ordner ist es ein Fehler.
pub fn extract(rest: &[String]) -> Result<(Option<PathBuf>, Vec<String>), String> {
    let mut dir = None;
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
    Ok((dir, out))
}
