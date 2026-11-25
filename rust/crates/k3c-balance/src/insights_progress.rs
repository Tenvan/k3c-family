//! Fortschritt im Bericht (Port von `engine/balance/insights_progress.go`, B-648): Urteil über das Ziel, Pfeile der
//! Meilenstein-Leiter gegen den letzten Lauf gleicher Konfiguration, kompaktes result.json für Agenten.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::fehler::Fehler;
use crate::insights::Insights;
use crate::progress::{GoalView, MilestoneRow, Progress, QualityView, RunConfig};
use crate::summary::{Summary, read_summary};

/// result.json neben dem Bericht: alles, was ein Agent zum Lesen eines Laufs braucht (Go: `ResultFile`).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultFile {
    pub run: String,
    pub generated: String,
    pub goal_reached: bool,
    pub goal: Option<GoalView>,
    pub quality: Option<QualityView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RunConfig>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub test_match: String,
    pub seeds: i64,
    pub milestones: Vec<MilestoneRow>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub blocker: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub previous: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub next_lever: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub report: String,
}

/// Der Satz über das Ziel; er nennt die Hilfe-Stufe, weil ein Lauf ohne Hilfen Fortschritt zeigt statt Regression
/// (Go: `goalText`).
pub fn goal_text(inn: &Insights) -> String {
    let Some(g) = &inn.goal else { return String::new() };
    let cfg = inn.progress.as_ref().map(|p| &p.config);
    let helps = match cfg {
        Some(c) if !c.helps.is_empty() => format!(" (mit Hilfen {})", c.helps),
        _ => String::new(),
    };
    if g.total == 0 {
        return "Kein Ziel definiert (keine Rolle goal in der Szenario-Auswahl): kein Urteil möglich.".into();
    }
    if g.reached {
        return format!("Ziel erreicht{helps}: {} von {} Zielen.", g.hit, g.total);
    }
    if cfg.is_some_and(|c| !c.world.is_empty() && c.helps.is_empty()) {
        return format!(
            "Ziel nicht erreicht (ohne Hilfen): {} von {} Zielen. Gilt als Fortschritt gegen den offenen Meilenstein, nicht als Regression.",
            g.hit, g.total
        );
    }
    format!("Ziel nicht erreicht{helps}: {} von {} Zielen.", g.hit, g.total)
}

/// Unterscheidet Läufe, die sich vergleichen lassen: Konfiguration, Seeds und Varianten (Go: `configKey`).
fn config_key(s: &Summary) -> String {
    let Some(p) = &s.progress else { return String::new() };
    let c = &p.config;
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        c.world,
        c.bot,
        c.players,
        c.days,
        c.grade,
        c.helps,
        s.seeds,
        s.varianten.join(";")
    )
}

/// Go: `filepath.Dir` (ohne Ordner „.“, Wurzel bleibt Wurzel).
fn dir_of(p: &Path) -> PathBuf {
    match p.parent() {
        Some(d) if d.as_os_str().is_empty() => PathBuf::from("."),
        Some(d) => d.to_path_buf(),
        None => p.to_path_buf(),
    }
}

fn base_of(p: &Path) -> String {
    p.file_name()
        .map_or_else(|| ".".into(), |n| n.to_string_lossy().into_owned())
}

fn is_report(name: &str) -> bool {
    name.starts_with("balance-") && name.ends_with(".json")
}

/// Die Berichte `balance-*.json` in `dir` und in dessen Unterordnern (Go: die zwei `filepath.Glob`).
fn reports_in(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    for p in &entries {
        if is_report(&base_of(p)) {
            out.push(p.clone());
        }
    }
    for p in entries.iter().filter(|p| p.is_dir()) {
        let Ok(sub) = std::fs::read_dir(p) else { continue };
        let mut inner: Vec<PathBuf> = sub.filter_map(|e| e.ok().map(|e| e.path())).collect();
        inner.sort();
        out.extend(inner.into_iter().filter(|q| is_report(&base_of(q))));
    }
}

/// Sucht den letzten Bericht gleicher Konfiguration vor `stem`: im Ordner von `stem` und in dessen Geschwister-Ordnern
/// (reports/simtest-run-N). Liefert den Bericht und seinen Namen; `None`, wenn keiner passt (Go: `findPrevious`).
pub fn find_previous(stem: &str, s: &Summary) -> Option<(Summary, String)> {
    let key = config_key(s);
    if key.is_empty() {
        return None;
    }
    let stem = Path::new(stem);
    let own = base_of(stem);
    let first = dir_of(stem);
    let mut files = Vec::new();
    reports_in(&first, &mut files);
    reports_in(&dir_of(&first), &mut files);
    let mut seen = BTreeSet::new();
    files.retain(|f| seen.insert(f.clone()));
    files.sort_by_key(|f| std::cmp::Reverse(base_of(f))); // neueste zuerst
    for f in files {
        let name = base_of(&f);
        let name = name.strip_suffix(".json").unwrap_or(&name).to_string();
        if name.as_str() >= own.as_str() {
            continue; // nur frühere Läufe
        }
        let Ok(raw) = std::fs::read_to_string(&f) else { continue };
        let Ok(prev) = read_summary(&raw) else { continue };
        if config_key(&prev) != key {
            continue;
        }
        return Some((prev, format!("{}/{name}", base_of(&dir_of(&f)))));
    }
    None
}

/// Setzt ▲ (besser), ▼ (schlechter), = (gleich) je Meilenstein gegen den früheren Lauf (Go: `markArrows`).
pub fn mark_arrows(cur: &mut Progress, prev: &Progress) {
    let before: HashMap<&str, &MilestoneRow> = prev.ladder.iter().map(|r| (r.id.as_str(), r)).collect();
    for r in &mut cur.ladder {
        if let Some(b) = before.get(r.id.as_str()) {
            r.arrow = arrow(b, r).into();
        }
    }
}

fn arrow(before: &MilestoneRow, now: &MilestoneRow) -> &'static str {
    if now.reached > before.reached {
        return "▲";
    }
    if now.reached < before.reached {
        return "▼";
    }
    match (now.median_day, before.median_day) {
        (Some(n), Some(b)) if n < b => "▲",
        (Some(n), Some(b)) if n > b => "▼",
        _ => "=",
    }
}

/// Baut result.json aus Auswertung und Bericht (Go: `resultFile`).
fn result_file(inn: &Insights, report: &str) -> ResultFile {
    let mut r = ResultFile {
        run: inn.run.clone(),
        generated: inn.generated.clone(),
        goal_reached: inn.goal.as_ref().is_some_and(|g| g.reached),
        goal: inn.goal.clone(),
        quality: inn.quality.clone(),
        config: None,
        test_match: String::new(),
        seeds: inn.seeds,
        milestones: Vec::new(),
        blocker: String::new(),
        previous: inn.previous.clone(),
        next_lever: String::new(),
        report: report.into(),
    };
    if let Some(p) = &inn.progress {
        r.config = Some(p.config.clone());
        r.test_match = p.test_match.clone();
        r.milestones = p.ladder.clone();
        r.blocker = p.blocker.clone();
    }
    for s in &inn.summary {
        if s.starts_with("Größter Hebel") {
            r.next_lever = s.clone();
        }
    }
    r
}

/// Schreibt result.json in den Ordner des Berichts (Go: `writeResult`).
pub fn write_result(stem: &str, inn: &Insights) -> Result<(), Fehler> {
    let stem = Path::new(stem);
    let report = format!("{}.html", base_of(stem));
    let mut text =
        serde_json::to_string_pretty(&result_file(inn, &report)).map_err(|e| Fehler::Daten(e.to_string()))?;
    text.push('\n');
    std::fs::write(dir_of(stem).join("result.json"), text).map_err(|e| Fehler::Daten(e.to_string()))
}
