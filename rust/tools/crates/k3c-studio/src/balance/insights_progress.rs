//! Fortschritt im Bericht (B-648): Urteil über das Ziel, Meilenstein-Leiter mit Pfeilen gegen den letzten Lauf gleicher
//! Konfiguration, kompaktes `result.json` für Agenten.

use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::insights::Insights;
use super::progress::{GoalView, MilestoneRow, Progress, QualityView, RunConfig};
use super::summary::{Summary, read_summary};
use super::writeguard::write_file;
use crate::kitx::go_marshal_indent;

/// `result.json` neben dem Bericht: alles, was ein Agent zum Lesen eines Laufs braucht.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
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

/// Der Satz über das Ziel; er nennt die Hilfe-Stufe, weil ein Lauf ohne Hilfen Fortschritt zeigt statt Regression.
pub(crate) fn goal_text(insights: &Insights) -> String {
    let Some(g) = insights.goal else {
        return String::new();
    };
    let config = insights.progress.as_ref().map(|p| &p.config);
    let helps = match config {
        Some(c) if !c.helps.is_empty() => format!(" (mit Hilfen {})", c.helps),
        _ => String::new(),
    };
    let (hit, total) = (g.hit, g.total);
    if total == 0 {
        return "Kein Ziel definiert (keine Rolle goal in der Szenario-Auswahl): kein Urteil möglich.".to_owned();
    }
    if g.reached {
        return format!("Ziel erreicht{helps}: {hit} von {total} Zielen.");
    }
    if config.is_some_and(|c| !c.world.is_empty() && c.helps.is_empty()) {
        return format!(
            "Ziel nicht erreicht (ohne Hilfen): {hit} von {total} Zielen. Gilt als Fortschritt gegen den offenen \
             Meilenstein, nicht als Regression."
        );
    }
    format!("Ziel nicht erreicht{helps}: {hit} von {total} Zielen.")
}

impl Insights {
    /// Urteil, Konfiguration, Leiter und Blocker (Reihenfolge laut B-648: Ergebnis, Leiter, Blocker).
    pub(crate) fn ladder_markdown(&self, b: &mut String) {
        let Some(goal) = self.goal else {
            return;
        };
        let mark = if goal.reached { "✅" } else { "❌" };
        b.push_str(&format!("## Ergebnis\n\n{mark} {}\n\n", self.goal_text));
        if let Some(q) = self.quality {
            b.push_str(&format!(
                "Qualität: {} von {} Korridoren im Korridor.\n\n",
                q.passed, q.total
            ));
        }
        let Some(p) = &self.progress else {
            return;
        };
        let c = &p.config;
        b.push_str(&format!(
            "Konfiguration: Welt {} · Bot {} · {} Spieler · {} Tage{}{} – {}.\n\n",
            c.world,
            c.bot,
            c.players,
            c.days,
            opt_text(" · Grad ", &c.grade),
            opt_text(" · Hilfen ", &c.helps),
            p.test_match
        ));
        b.push_str("### Meilensteine\n\n| | Meilenstein | Seeds | Median-Tag |\n|---|---|---|---|\n");
        for r in &p.ladder {
            let arrow = if r.arrow.is_empty() { "·" } else { &r.arrow };
            let day = r.median_day.map_or_else(|| "–".to_owned(), |d| format!("Tag {d}"));
            b.push_str(&format!(
                "| {arrow} | {} | {}/{} | {day} |\n",
                r.label, r.reached, r.runs
            ));
        }
        if !self.previous.is_empty() {
            b.push_str(&format!(
                "\nPfeile gegen den letzten Lauf gleicher Konfiguration ({}).\n",
                self.previous
            ));
        }
        if !p.blocker.is_empty() {
            b.push_str(&format!("\n**Blocker:** {}\n", p.blocker));
        }
        b.push('\n');
    }
}

fn opt_text(prefix: &str, v: &str) -> String {
    if v.is_empty() {
        String::new()
    } else {
        format!("{prefix}{v}")
    }
}

/// Unterscheidet Läufe, die sich vergleichen lassen: Konfiguration, Seeds und Varianten.
fn config_key(s: &Summary) -> Option<String> {
    let c = &s.progress.as_ref()?.config;
    Some(format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        c.world,
        c.bot,
        c.players,
        c.days,
        c.grade,
        c.helps,
        s.seeds,
        s.varianten.join(";")
    ))
}

/// Das Elternverzeichnis wie Go `filepath.Dir` (`.` statt leer).
pub(crate) fn dir_of(path: &Path) -> &Path {
    match path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        Some(_) => Path::new("."),
        None => path,
    }
}

/// Der letzte Teil eines Pfads als Text; `.` ohne Namen.
pub(crate) fn base_of(path: &Path) -> String {
    path.file_name()
        .map_or_else(|| ".".to_owned(), |n| n.to_string_lossy().into_owned())
}

fn is_report(path: &Path) -> bool {
    let name = base_of(path);
    name.starts_with("balance-") && name.ends_with(".json")
}

/// Die Einträge eines Ordners; ein unlesbarer Ordner hat keine (wie Go `filepath.Glob`).
fn entries(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir).map_or_else(|_| Vec::new(), |list| list.flatten().map(|e| e.path()).collect())
}

/// Alle `balance-*.json` in `dir` und eine Ebene darunter.
fn report_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for path in entries(dir) {
        if path.is_dir() {
            out.extend(entries(&path).into_iter().filter(|p| is_report(p)));
        }
        if is_report(&path) {
            out.push(path);
        }
    }
    out
}

/// Der letzte Bericht gleicher Konfiguration vor `stem`: im Ordner von `stem` und in dessen Geschwister-Ordnern
/// (`reports/simtest-run-N`). Liefert den Bericht und seinen Namen.
pub(crate) fn find_previous(stem: &Path, s: &Summary) -> Option<(Summary, String)> {
    let key = config_key(s)?;
    let own = base_of(stem);
    let dir = dir_of(stem);
    let mut files = report_files(dir);
    files.extend(report_files(dir_of(dir)));
    // neueste zuerst
    files.sort_by_cached_key(|f| (Reverse(base_of(f)), f.clone()));
    files.dedup();
    files.iter().find_map(|f| {
        let mut name = base_of(f);
        name.truncate(name.len() - ".json".len());
        if name >= own {
            return None; // nur frühere Läufe
        }
        let prev = read_summary(&fs::read(f).ok()?).ok()?;
        (config_key(&prev).as_ref() == Some(&key)).then(|| (prev, format!("{}/{name}", base_of(dir_of(f)))))
    })
}

/// Setzt ▲ (besser), ▼ (schlechter), = (gleich) je Meilenstein gegen den früheren Lauf.
pub(crate) fn mark_arrows(cur: Option<&mut Progress>, prev: Option<&Progress>) {
    let (Some(cur), Some(prev)) = (cur, prev) else {
        return;
    };
    let before: HashMap<&str, &MilestoneRow> = prev.ladder.iter().map(|r| (r.id.as_str(), r)).collect();
    for row in &mut cur.ladder {
        if let Some(b) = before.get(row.id.as_str()) {
            arrow(b, row).clone_into(&mut row.arrow);
        }
    }
}

fn arrow(before: &MilestoneRow, now: &MilestoneRow) -> &'static str {
    // mehr Seeds am Ziel schlägt den Tag; ein früherer Tag ist besser
    let days = match (before.median_day, now.median_day) {
        (Some(b), Some(n)) => b.cmp(&n),
        _ => std::cmp::Ordering::Equal,
    };
    match now.reached.cmp(&before.reached).then(days) {
        std::cmp::Ordering::Greater => "▲",
        std::cmp::Ordering::Less => "▼",
        std::cmp::Ordering::Equal => "=",
    }
}

/// Baut `result.json` aus Auswertung und Bericht.
fn result_file(insights: &Insights, report: String) -> ResultFile {
    let mut r = ResultFile {
        run: insights.run.clone(),
        generated: insights.generated.clone(),
        goal_reached: insights.goal.is_some_and(|g| g.reached),
        goal: insights.goal,
        quality: insights.quality,
        seeds: insights.seeds,
        previous: insights.previous.clone(),
        report,
        ..ResultFile::default()
    };
    if let Some(p) = &insights.progress {
        r.config = Some(p.config.clone());
        r.test_match.clone_from(&p.test_match);
        r.milestones.clone_from(&p.ladder);
        r.blocker.clone_from(&p.blocker);
    }
    if let Some(s) = insights.summary.iter().rfind(|s| s.starts_with("Größter Hebel")) {
        r.next_lever.clone_from(s);
    }
    r
}

/// Schreibt `result.json` in den Ordner des Berichts.
pub(crate) fn write_result(root: &Path, stem: &Path, insights: &Insights) -> Result<(), String> {
    let file = result_file(insights, format!("{}.html", base_of(stem)));
    let text = go_marshal_indent(&file)? + "\n";
    write_file(root, dir_of(stem).join("result.json"), text.as_bytes()).map_err(|e| e.to_string())
}
