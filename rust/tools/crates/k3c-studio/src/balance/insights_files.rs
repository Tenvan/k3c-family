//! Dateien der Auswertung (B-632): neben `balance-<Zeit>.json` stehen `.md` und `.html`. [`reevaluate`] erzeugt beide
//! für einen vorhandenen Bericht neu (auch für alte ohne Kennzahlen), z. B. nach geänderten Richtwerten.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use chrono::{Local, NaiveDateTime};

use super::hints::default_hints;
use super::insights::{Meta, build_insights};
use super::insights_progress::{base_of, dir_of, find_previous, mark_arrows, write_result};
use super::summary::{Summary, read_summary};
use super::targets::default_targets;
use super::writeguard::write_file;

/// Die Baseline des Repos (`task balance:baseline`), relativ zum Checkout.
pub const BASELINE_PATH: &str = "testdata/balance/baseline.json";

/// `stem` mit angehängter Endung (anders als `with_extension` bleibt ein Punkt im Namen).
fn suffixed(stem: &Path, ext: &str) -> PathBuf {
    let mut name = OsString::from(stem);
    name.push(ext);
    PathBuf::from(name)
}

/// Schreibt `result.json`, `stem.md` und `stem.html` (`stem` = Pfad ohne Endung); `root` ist der Checkout.
pub fn write_reports(
    stem: &Path,
    s: &Summary,
    base: Option<&Summary>,
    base_name: &str,
    root: &Path,
) -> Result<(), String> {
    let hints = default_hints(root)?;
    let mut insights = build_insights(root, s, base, &hints, &meta_of(root, stem, base_name));
    if let Some((prev, name)) = find_previous(stem, s) {
        mark_arrows(insights.progress.as_mut(), prev.progress.as_ref());
        insights.previous = name;
    }
    write_result(root, stem, &insights)?;
    let write =
        |ext: &str, text: &str| write_file(root, suffixed(stem, ext), text.as_bytes()).map_err(|e| e.to_string());
    write(".md", &s.markdown_with(base, Some(&insights)))?;
    write(".html", &insights.html(root)?)
}

/// Liest einen `balance-*.json` und schreibt `.md` und `.html` daneben neu; Ergebnis ist der Pfad der HTML-Datei. Die
/// Baseline (`root`/[`BASELINE_PATH`]) fehlt still, wenn es sie nicht gibt.
pub fn reevaluate(json_path: &Path, root: &Path) -> Result<PathBuf, String> {
    let name = base_of(json_path);
    if !name.starts_with("balance-") || !name.ends_with(".json") {
        return Err(format!("{name}: nur balance-*.json"));
    }
    let raw = fs::read(json_path).map_err(|e| format!("open {}: {e}", json_path.display()))?;
    let s = read_summary(&raw).map_err(|e| format!("{name}: {e}"))?;
    let base = read_baseline(&root.join(BASELINE_PATH))?;
    let stem = json_path.with_extension("");
    write_reports(&stem, &s, base.as_ref(), BASELINE_PATH, root)?;
    Ok(suffixed(&stem, ".html"))
}

/// Liest eine Baseline; `None`, wenn die Datei fehlt.
pub fn read_baseline(path: &Path) -> Result<Option<Summary>, String> {
    match fs::read(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("open {}: {e}", path.display())),
        Ok(raw) => read_summary(&raw).map(Some),
    }
}

/// Lauf = Ordner des Berichts, Zeit aus dem Dateinamen `balance-JJJJMMTT-hhmmss` (sonst jetzt).
pub fn meta_of(root: &Path, stem: &Path, base_name: &str) -> Meta {
    let file = base_of(stem);
    let stamp = file.strip_prefix("balance-").unwrap_or(&file);
    // chrono nimmt auch kürzere Felder, Go nicht
    let time = Some(stamp)
        .filter(|s| s.len() == "20060102-150405".len())
        .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y%m%d-%H%M%S").ok())
        .unwrap_or_else(|| Local::now().naive_local());
    Meta {
        run: base_of(dir_of(stem)),
        generated: time.format("%d.%m.%Y %H:%M").to_string(),
        baseline: base_name.to_owned(),
        frozen: default_targets(root).ok().and_then(|t| t.frozen).unwrap_or_default(),
    }
}
