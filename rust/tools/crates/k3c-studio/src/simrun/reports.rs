//! Berichte lesen (Port von `reports.go`).
//!
//! Aus RU70.19 vorgezogen (V11): alles, was Status, Liste und Baseline brauchen; [`read_tuning`], [`read_insights`] und
//! [`write_html`] sind noch Stubs.

use std::fs;
use std::io::ErrorKind;
use std::path::{Component, MAIN_SEPARATOR_STR, Path, PathBuf};
use std::time::{Duration, SystemTime};

use serde::Deserialize;
use serde_json::Value;

use super::stamp::{StampRaw, read_object};
use crate::balance::insights::{Insights, Meta};
use crate::balance::progress::{GoalView, Progress, QualityView};
use crate::balance::summary::{Summary, read_summary};
use crate::balance::tune::{TuneBest, Tuning};
use crate::kitx::go_quoted;

/// Teil von `reports/…/balance-*.json`, den Status und Liste lesen (Format: `balance::summary`).
#[derive(Default, Deserialize)]
#[serde(default)]
pub(super) struct BalanceBody {
    pub seeds: i64,
    pub pass: bool,
    pub varianten: Vec<String>,
    pub goal: Option<GoalView>,
    pub quality: Option<QualityView>,
    pub progress: Option<Progress>,
    pub targets: Vec<TargetLine>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub(super) struct TargetLine {
    pub role: String,
    pub name: String,
    pub kind: String,
    pub value: Option<f64>,
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub pass: bool,
}

/// Teil von `tuning-*.json`, den Status und Liste lesen (Format: `balance::tune`).
#[derive(Default, Deserialize)]
#[serde(default)]
pub(super) struct TuningBody {
    pub mode: String,
    pub points: Vec<Value>,
    pub best: TuneBest,
    pub reason: String,
}

/// Liest Auftrags-Stempel und Rumpf eines Berichts.
pub(super) fn parse_report<T: serde::de::DeserializeOwned + Default>(raw: &[u8]) -> Result<(StampRaw, T), String> {
    let body = read_object(raw)?;
    Ok((StampRaw::read(raw)?, body))
}

/// Dateien `<prefix>*.json` in `dir`, nach Name sortiert.
pub(super) fn files_in(dir: &Path, prefix: &str) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|e| e.file_name())
        .filter(|n| {
            let n = n.to_string_lossy();
            n.len() >= prefix.len() + ".json".len() && n.starts_with(prefix) && n.ends_with(".json")
        })
        .collect();
    names.sort();
    names.into_iter().map(|n| dir.join(n)).collect()
}

/// Neuester Bericht `<prefix>*.json` in `dir`, der nicht vor `since` geschrieben wurde (ein alter Bericht derselben
/// Lauf-ID aus einer früheren Sitzung zählt nicht).
pub(super) fn newest_file(dir: &Path, prefix: &str, since: SystemTime) -> Result<PathBuf, String> {
    let limit = since.checked_sub(Duration::from_secs(1)).unwrap_or(since);
    let fresh = |f: &PathBuf| fs::metadata(f).and_then(|m| m.modified()).is_ok_and(|t| t >= limit);
    files_in(dir, prefix).into_iter().rev().find(fresh).ok_or_else(|| {
        let dir = dir.to_string_lossy().replace('\\', "/");
        format!("kein Bericht {prefix}*.json in {dir}")
    })
}

fn read_newest<T: serde::de::DeserializeOwned + Default>(
    dir: &Path,
    prefix: &str,
    since: SystemTime,
) -> Result<(PathBuf, T), String> {
    let file = newest_file(dir, prefix, since)?;
    let raw = fs::read(&file).map_err(|e| e.to_string())?;
    let (_, body) = parse_report(&raw).map_err(|e| format!("bericht unlesbar: {e}"))?;
    Ok((file, body))
}

/// Urteil und Zeilen eines Laufs: `tuning-*.json` beim Abstimmen, sonst `balance-*.json`.
pub(super) fn read_result(
    root: &Path,
    dir: &Path,
    since: SystemTime,
    tune: bool,
) -> Result<(String, Vec<String>), String> {
    if !tune {
        let (file, sum) = read_newest::<BalanceBody>(dir, "balance-", since)?;
        let mut lines = insight_lines(root, &file);
        lines.extend(sum.lines());
        return Ok((verdict(sum.pass), lines));
    }
    let (_, t) = read_newest::<TuningBody>(dir, "tuning-", since)?;
    let points = format!(
        "{} Punkte ({}), bester Score {}",
        t.points.len(),
        t.mode,
        num(Some(t.best.score))
    );
    Ok((verdict(t.best.pass), vec![points, t.reason]))
}

/// So viele Sätze der Kurzauswertung nennt `sim_test status` (Grenze 10 Zeilen, B-348).
const STATUS_SENTENCES: usize = 3;

/// Kurzauswertung (B-632) und Pfad des HTML-Berichts zu `file`; ohne lesbare Auswertung leer.
fn insight_lines(root: &Path, file: &Path) -> Vec<String> {
    let Some(insights) = fs::read(file)
        .ok()
        .and_then(|raw| read_summary(&raw).ok())
        .and_then(|s| s.insights(root, None, Meta::default()).ok())
    else {
        return Vec::new();
    };
    let mut out: Vec<String> = insights.summary.into_iter().take(STATUS_SENTENCES).collect();
    let html = file.with_extension("html");
    if html.exists() {
        let folder = file.parent().and_then(Path::file_name).unwrap_or_default();
        let name = html.file_name().unwrap_or_default();
        out.push(format!(
            "HTML-Bericht: reports/{}/{}",
            folder.to_string_lossy(),
            name.to_string_lossy()
        ));
    }
    out
}

fn verdict(pass: bool) -> String {
    if pass { "Pass" } else { "Fail" }.to_owned()
}

impl BalanceBody {
    /// Urteil über das Ziel, Qualität, erreichte Meilensteine und Blocker (B-648).
    fn progress_lines(&self) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(g) = &self.goal
            && g.total > 0
        {
            let word = if g.reached { "erreicht" } else { "nicht erreicht" };
            out.push(format!("Ziel {word} ({} von {})", g.hit, g.total));
        }
        if let Some(q) = &self.quality {
            out.push(format!("Qualität {} von {} Korridoren", q.passed, q.total));
        }
        if let Some(p) = &self.progress {
            let done = p.ladder.iter().filter(|r| r.reached == r.runs && r.runs > 0).count();
            out.push(format!(
                "Meilensteine {done} von {} in allen Seeds; {}",
                p.ladder.len(),
                p.test_match
            ));
            if !p.blocker.is_empty() {
                out.push(p.blocker.clone());
            }
        }
        out
    }

    /// Eine Zeile je Ziel, z. B. „Fail Burg hält Nacht 1–5: 4 % (75–90 %)“.
    fn lines(&self) -> Vec<String> {
        let mut out = vec![format!("{} Seeds", self.seeds)];
        out.extend(self.progress_lines());
        for t in &self.targets {
            let unit = if t.kind == "share" { " %" } else { "" };
            let mark = if t.pass {
                "Pass"
            } else if t.role == "guard" || t.role == "info" {
                "Warn" // nur Ziele (goal) lassen den Lauf scheitern (B-648)
            } else {
                "Fail"
            };
            let bounds = bounds(t.lower, t.upper, unit);
            out.push(format!("{mark} {}: {}{unit} ({bounds})", t.name, num(t.value)));
        }
        out
    }
}

/// „75–90 %“, „≤ 1“ oder „≥ 20“.
fn bounds(lower: Option<f64>, upper: Option<f64>, unit: &str) -> String {
    match (lower, upper) {
        (None, _) => format!("≤ {}{unit}", num(upper)),
        (_, None) => format!("≥ {}{unit}", num(lower)),
        _ => format!("{}–{}{unit}", num(lower), num(upper)),
    }
}

/// Kürzeste Form ohne Exponent (Go `FormatFloat(v, 'f', -1, 64)`).
fn num(v: Option<f64>) -> String {
    v.map_or_else(|| "–".to_owned(), |v| v.to_string())
}

/// Wie Go `filepath.Clean` für einen relativen Pfad: Teile ohne „.“, „..“ hebt den vorigen Teil auf.
fn clean_parts(rel: &str) -> Vec<&str> {
    let mut parts: Vec<&str> = Vec::new();
    for part in rel.split(['/', '\\']) {
        match part {
            "" | "." => {}
            ".." if parts.last().is_some_and(|p| *p != "..") => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts
}

/// Prüft einen Berichtspfad relativ zum Checkout: nur `*.json` unter `reports/`, kein absoluter Pfad, kein „..“.
pub fn report_path(root: &Path, rel: &str) -> Result<PathBuf, String> {
    let quoted = go_quoted(rel);
    let rooted = rel.starts_with(['/', '\\']) || rel.as_bytes().get(1) == Some(&b':');
    if rel.is_empty() || rooted || matches!(Path::new(rel).components().next(), Some(Component::Prefix(_))) {
        return Err(format!(
            "bericht {quoted}: nur Pfade relativ zum Checkout unter reports/"
        ));
    }
    let parts = clean_parts(rel);
    if parts.len() < 2 || parts[0] != "reports" || parts.contains(&"..") {
        return Err(format!("bericht {quoted}: nur Pfade unter reports/"));
    }
    if parts.last().and_then(|n| n.rsplit_once('.')).map(|(_, ext)| ext) != Some("json") {
        return Err(format!("bericht {quoted}: nur .json"));
    }
    // als ein Text angehängt: ein Teil wie „c:“ ersetzt so nie die Wurzel
    Ok(root.join(parts.join(MAIN_SEPARATOR_STR)))
}

/// Liest einen Bericht unter `reports/` als `balance::Summary`.
pub fn read_report(root: &Path, rel: &str) -> Result<Summary, String> {
    read_summary(&read_report_file(root, rel)?)
}

/// Liest eine Datei unter `reports/` nach [`report_path`]; nur reguläre Dateien.
pub(super) fn read_report_file(root: &Path, rel: &str) -> Result<Vec<u8>, String> {
    let path = report_path(root, rel)?;
    let quoted = go_quoted(rel);
    let fail = |e: std::io::Error| format!("open {}: {e}", path.display());
    match fs::symlink_metadata(&path) {
        Err(e) if e.kind() == ErrorKind::NotFound => Err(format!("bericht {quoted} gibt es nicht")),
        Err(e) => Err(fail(e)),
        Ok(meta) if !meta.is_file() => Err(format!("bericht {quoted} ist keine Datei")),
        Ok(_) => fs::read(&path).map_err(fail),
    }
}

/// Liest ein Abstimm-Ergebnis unter `reports/` (Pfadprüfung wie [`read_report`]).
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn read_tuning(_root: &Path, _rel: &str) -> Result<Tuning, String> {
    unimplemented!("RU70.19")
}

/// Wertet einen `balance-*.json` unter `reports/` aus (B-632); die Baseline des Checkouts liefert den Trend.
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn read_insights(_root: &Path, _rel: &str) -> Result<Insights, String> {
    unimplemented!("RU70.19")
}

/// Schreibt `.md` und `.html` eines `balance-*.json` unter `reports/` neu; Ergebnis ist der Pfad der HTML-Datei.
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn write_html(_root: &Path, _rel: &str) -> Result<PathBuf, String> {
    unimplemented!("RU70.19")
}
