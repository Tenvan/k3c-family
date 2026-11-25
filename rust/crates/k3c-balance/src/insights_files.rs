//! Dateien der Auswertung (Port von `engine/balance/insights_files.go`, B-632): neben balance-<Zeit>.json stehen .md und
//! .html. `reevaluate` erzeugt beide für einen vorhandenen Bericht neu (auch für alte ohne Kennzahlen), z. B. nach
//! geänderten Richtwerten.
//!
//! Abweichung von Go: Rust-std kennt keine Zeitzone, `generated` ist UTC (Go: Ortszeit); der Zeitstempel aus dem Dateinamen
//! wird unverändert als Wandzeit übernommen, wie Go ihn mit `time.Local` liest und wieder ausgibt.

use std::io::ErrorKind;
use std::path::Path;
use std::time::SystemTime;

use crate::baselines::utc_parts;
use crate::fehler::Fehler;
use crate::hints::default_hints;
use crate::insights::{Meta, build_insights};
use crate::insights_markdown::insights_markdown;
use crate::insights_progress::{find_previous, mark_arrows, write_result};
use crate::report_html::report_html;
use crate::summary::{Summary, read_summary};
use crate::targets::default_targets;

/// Die Baseline des Repos (`task balance:baseline`), relativ zum Checkout.
pub const BASELINE_PATH: &str = "testdata/balance/baseline.json";

fn io(e: std::io::Error) -> Fehler {
    Fehler::Daten(e.to_string())
}

/// Schreibt `stem.md` und `stem.html` (`stem` = Pfad ohne Endung); `root` ist der Checkout (Grafiken; Go: `WriteReports`).
pub fn write_reports(
    stem: &str,
    s: &Summary,
    base: Option<&Summary>,
    base_name: &str,
    root: &str,
) -> Result<(), Fehler> {
    let h = default_hints()?;
    let mut inn = build_insights(s, base, &h, &meta_of(stem, base_name));
    if let Some((prev, name)) = find_previous(stem, s) {
        if let (Some(cur), Some(p)) = (inn.progress.as_mut(), prev.progress.as_ref()) {
            mark_arrows(cur, p);
        }
        inn.previous = name;
    }
    write_result(stem, &inn)?;
    let md = s.markdown_with(base, Some(&insights_markdown(&inn)));
    std::fs::write(format!("{stem}.md"), md).map_err(io)?;
    let page = report_html(&inn, root)?;
    std::fs::write(format!("{stem}.html"), page).map_err(io)
}

/// Liest einen balance-*.json und schreibt .md und .html daneben neu; Ergebnis ist der Pfad der HTML-Datei. Die Baseline
/// (`root/BASELINE_PATH`) fehlt still, wenn es sie nicht gibt (Go: `Reevaluate`).
pub fn reevaluate(json_path: &str, root: &str) -> Result<String, Fehler> {
    let p = Path::new(json_path);
    let base_name = p
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    if !base_name.starts_with("balance-") || p.extension().is_none_or(|e| e != "json") {
        return Err(Fehler::Daten(format!("{base_name}: nur balance-*.json")));
    }
    let raw = std::fs::read_to_string(p).map_err(io)?;
    let s = read_summary(&raw).map_err(|e| Fehler::Daten(format!("{base_name}: {e}")))?;
    let base = read_baseline(&Path::new(root).join(BASELINE_PATH))?;
    let stem = json_path.strip_suffix(".json").unwrap_or(json_path);
    write_reports(stem, &s, base.as_ref(), BASELINE_PATH, root)?;
    Ok(format!("{stem}.html"))
}

/// Liest eine Baseline; `None`, wenn die Datei fehlt (Go: `ReadBaseline`).
pub fn read_baseline(path: &Path) -> Result<Option<Summary>, Fehler> {
    match std::fs::read_to_string(path) {
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(io(e)),
        Ok(raw) => read_summary(&raw).map(Some),
    }
}

/// Lauf = Ordner des Berichts, Zeit aus dem Dateinamen balance-JJJJMMTT-hhmmss (sonst jetzt; Go: `MetaOf`).
pub fn meta_of(stem: &str, base_name: &str) -> Meta {
    let path = Path::new(stem);
    let run = match path.parent() {
        Some(d) if d.as_os_str().is_empty() => ".".to_string(),
        Some(d) => d
            .file_name()
            .map_or_else(|| "/".into(), |n| n.to_string_lossy().into_owned()),
        None => ".".into(),
    };
    let file = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let stamp = file.strip_prefix("balance-").unwrap_or(&file);
    let [y, mo, d, h, mi, _] = utc_parts(SystemTime::now());
    let generated = parse_stamp(stamp).unwrap_or_else(|| format_time(y, mo, d, h, mi));
    Meta {
        run,
        generated,
        baseline: base_name.into(),
        frozen: default_targets().map(|t| t.frozen).unwrap_or_default(),
    }
}

fn format_time(y: i64, mo: i64, d: i64, h: i64, mi: i64) -> String {
    format!("{d:02}.{mo:02}.{y:04} {h:02}:{mi:02}")
}

/// `20060102-150405` → `02.01.2006 15:04`; `None` bei einem anderen Format oder einer Zeit, die es nicht gibt.
fn parse_stamp(s: &str) -> Option<String> {
    let b = s.as_bytes();
    if b.len() != 15 || b[8] != b'-' || !b.iter().enumerate().all(|(i, c)| i == 8 || c.is_ascii_digit()) {
        return None;
    }
    let n = |a: usize, z: usize| s[a..z].parse::<i64>().ok();
    let (y, mo, d, h, mi, sec) = (n(0, 4)?, n(4, 6)?, n(6, 8)?, n(9, 11)?, n(11, 13)?, n(13, 15)?);
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let days = match mo {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return None,
    };
    ((1..=days).contains(&d) && h < 24 && mi < 60 && sec < 60).then(|| format_time(y, mo, d, h, mi))
}
