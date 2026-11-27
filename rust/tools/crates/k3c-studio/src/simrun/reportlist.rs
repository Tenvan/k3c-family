//! Liste der Berichte (Port von `reportlist.go`, Methode `BalanceReports`).

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Local};
use serde::Serialize;

use super::info::go_time;
use super::manage::load_ratings;
use super::reports::{BalanceBody, TuningBody, files_in, parse_report};
use super::spec::Spec;
use super::stamp::StampRaw;

/// Ein gespeicherter Bericht unter `reports/`.
#[derive(Clone, Debug, Serialize)]
pub struct ReportInfo {
    /// Relativ zum Checkout, mit `/`.
    pub path: String,
    /// Ordner des Laufs, z. B. `simtest-run-3`; leer bei `reports/balance-*.json`.
    pub run: String,
    #[serde(serialize_with = "go_time")]
    pub modified: DateTime<Local>,
    pub seeds: i64,
    pub pass: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub varianten: Vec<String>,
    /// balance (`balance-*.json`) oder tuning (`tuning-*.json`).
    pub kind: String,
    /// Bewertung aus `reports/ratings.json` (WZ28.13), leer = keine.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub rating: String,
    /// Label, Auftrag und Lauf-ID stempelt das Studio in den Bericht (B-662); ältere Berichte haben sie nicht.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<Spec>,
    #[serde(rename = "runId", skip_serializing_if = "String::is_empty")]
    pub run_id: String,
    /// Getroffene und alle Ziele des Berichts (0/0 bei `tuning-*.json`).
    #[serde(rename = "targetsPass")]
    pub targets_pass: i64,
    #[serde(rename = "targetsTotal")]
    pub targets_total: i64,
}

/// Berichte des Checkouts: `reports/balance-*.json`, `reports/*/balance-*.json` und ebenso `tuning-*.json`, neueste
/// zuerst. Unlesbare Dateien fehlen in der Liste.
pub fn reports(root: &Path) -> Result<Vec<ReportInfo>, String> {
    let base = root.join("reports");
    let mut folders: Vec<PathBuf> = fs::read_dir(&base)
        .map(|entries| entries.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    folders.retain(|p| p.is_dir());
    folders.sort();
    let mut out = Vec::new();
    for kind in ["balance", "tuning"] {
        let prefix = format!("{kind}-");
        for dir in std::iter::once(&base).chain(&folders) {
            let files = files_in(dir, &prefix);
            out.extend(files.iter().filter_map(|f| report_info(root, f, kind)));
        }
    }
    out.sort_by_key(|r: &ReportInfo| std::cmp::Reverse(r.modified));
    // unlesbare ratings.json: Liste ohne Bewertungen statt ohne Berichte
    let ratings = load_ratings(root).unwrap_or_default();
    for info in &mut out {
        info.rating = ratings.get(&info.path).cloned().unwrap_or_default();
    }
    Ok(out)
}

/// Ein Eintrag der Liste; `None` bei unlesbarer Datei.
fn report_info(root: &Path, file: &Path, kind: &str) -> Option<ReportInfo> {
    let meta = fs::metadata(file).ok().filter(fs::Metadata::is_file)?;
    let modified: SystemTime = meta.modified().ok()?;
    let raw = fs::read(file).ok()?;
    let rel = file.strip_prefix(root).ok()?.to_string_lossy().replace('\\', "/");
    let folder = file.parent()?.file_name()?.to_string_lossy().into_owned();
    let mut info = ReportInfo {
        path: rel,
        run: if folder == "reports" { String::new() } else { folder },
        modified: modified.into(),
        seeds: 0,
        pass: false,
        varianten: Vec::new(),
        kind: kind.to_owned(),
        rating: String::new(),
        label: String::new(),
        spec: None,
        run_id: String::new(),
        targets_pass: 0,
        targets_total: 0,
    };
    let stamp: StampRaw = if kind == "tuning" {
        let (stamp, t) = parse_report::<TuningBody>(&raw).ok()?;
        info.pass = t.best.pass;
        stamp
    } else {
        let (stamp, sum) = parse_report::<BalanceBody>(&raw).ok()?;
        let hit = sum.targets.iter().filter(|t| t.pass).count();
        info.targets_total = i64::try_from(sum.targets.len()).unwrap_or(i64::MAX);
        info.targets_pass = i64::try_from(hit).unwrap_or(i64::MAX);
        (info.seeds, info.pass, info.varianten) = (sum.seeds, sum.pass, sum.varianten);
        stamp
    };
    info.spec = stamp.spec();
    (info.label, info.run_id) = (stamp.label, stamp.run_id);
    Some(info)
}
