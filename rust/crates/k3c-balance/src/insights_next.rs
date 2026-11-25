//! Tuning-Hinweise und Vorschlag für den nächsten Lauf (Port von `engine/balance/insights_next.go`, B-632). Die Befehle
//! passen zu den Werkzeugen: sim_test (voller Lauf), task balance:targets (Replays nach reports/balance-replays/), replay_run,
//! Lauf-Spec der Balancing-Seite und CLI für --tune. Dazu die Hilfen aus `insights_text.go` und `insights_plain.go`, die
//! `worst` und die Hinweise brauchen (`label`, `worst`); `above`, `below`, `word` stehen in `insights_plain`, RU31.4 übernimmt den Rest.

use std::cmp::Ordering;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::hints::Lever;
use crate::insights::{Insights, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, TargetInsight};
use crate::insights_levers::recommendations;
use crate::insights_plain::LeverPlan;
use crate::pending::Hint;
use crate::summary::Summary;
use crate::summary_text::num;
use crate::tuneparam::{MAX_TUNE_PARAMS, TUNE_SEARCH, parse_tune_param};

/// Befehl für den Chat an Claude.
pub const WHERE_CHAT: &str = "chat";
/// Befehl für das Terminal des Checkouts.
pub const WHERE_SHELL: &str = "shell";
/// Befehl für die Balancing-Seite der Workbench (Lauf-Spec).
pub const WHERE_WORKBENCH: &str = "workbench";

/// Höchstens drei Schritte des Hebels in seine Richtung, ab dem Wert in data/.
pub(crate) const TUNE_RANGE: f64 = 3.0;

/// Ein kopierbarer Aufruf. `where`: chat (Auftrag an Claude im Chat, Claude ruft das MCP-Werkzeug), shell (Terminal im
/// Checkout) oder workbench (Balancing-Seite › Lauf-Spec).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Command {
    pub label: String,
    pub r#where: String,
    pub text: String,
}

/// Der Vorschlag: Schritte als Text, Seeds zum Nachspielen und kopierbare Befehle.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct NextRun {
    pub steps: Vec<String>,
    pub seeds: Vec<String>,
    pub commands: Vec<Command>,
}

impl TargetInsight {
    /// Name des Ziels mit Grad, wenn nicht normal (Go: `label`).
    pub fn label(&self) -> String {
        if self.grade == "normal" {
            self.name.clone()
        } else {
            format!("{} ({})", self.name, self.grade)
        }
    }
}

/// Verletzte Ziele, am weitesten daneben zuerst (Go: `worst`).
pub fn worst(inn: &Insights) -> Vec<&TargetInsight> {
    let mut out: Vec<&TargetInsight> = inn.targets.iter().filter(|t| t.light == LIGHT_RED).collect();
    out.sort_by(|a, b| {
        let (x, y) = (a.distance.unwrap_or(0.0), b.distance.unwrap_or(0.0));
        x.partial_cmp(&y).unwrap_or(Ordering::Equal)
    });
    out
}

/// Die Tuning-Hinweise: verletzte Ziele zuerst, dann knappe und ungültige (Go: `hints`).
pub fn hints(inn: &Insights) -> Vec<Hint> {
    let mut out: Vec<Hint> = worst(inn)
        .into_iter()
        .map(|t| Hint {
            target: t.label(),
            light: LIGHT_RED.into(),
            text: red_text(t),
            why: why_text(t),
            levers: t.levers.clone(),
            plans: t.plans.clone(),
        })
        .collect();
    for t in &inn.targets {
        if t.light == LIGHT_YELLOW {
            out.push(Hint {
                target: t.label(),
                light: LIGHT_YELLOW.into(),
                levers: t.levers.clone(),
                plans: t.plans.clone(),
                why: why_text(t),
                text: "Knapp an der Grenze: beim Tunen anderer Ziele im Blick behalten, erst bei Rot drehen.".into(),
            });
        } else if t.light == LIGHT_GREY {
            out.push(Hint {
                target: t.label(),
                light: LIGHT_GREY.into(),
                levers: Vec::<Lever>::new(),
                plans: Vec::<LeverPlan>::new(),
                text: format!(
                    "Läufe abgebrochen oder ohne Messwert: Seeds {} nachspielen, Fehler mit logs_errors suchen.",
                    seed_list(&t.invalid_seeds, 5)
                ),
                why: format!(
                    "{} ungültige Seeds; der Wert ist nicht belastbar.",
                    t.invalid_seeds.len()
                ),
            });
        }
    }
    out
}

/// Richtung als Wort und die ersten zwei Stellschrauben; Zahlen zum Eintragen stehen in `plans` (Go: `redText`).
fn red_text(t: &TargetInsight) -> String {
    let mut dir = "Außerhalb des Korridors".to_string();
    let w = t.word();
    if !w.is_empty() {
        let side = if t.above() { "über" } else { "unter" };
        let mut c = w.chars();
        let head: String = c.next().map(|f| f.to_uppercase().collect()).unwrap_or_default();
        dir = format!("{head}{} (Wert {side} dem Korridor)", c.as_str());
    }
    if t.levers.is_empty() {
        return format!("{dir}: keine Stellschraube hinterlegt (data/balance-hints.json), Ursache im Replay suchen.");
    }
    let parts: Vec<&str> = t.levers.iter().take(2).map(|l| l.label.as_str()).collect();
    format!(
        "{dir}. Eine dieser Stellschrauben probieren: {} (Zahlen darunter).",
        parts.join(" oder ")
    )
}

/// Messwert, Sollwert und Lage in Klartext, dazu der Vergleich mit der Baseline (Go: `whyText`).
fn why_text(t: &TargetInsight) -> String {
    if t.trend_text.is_empty() {
        t.plain.clone()
    } else {
        format!("{} {}", t.plain, t.trend_text)
    }
}

/// Eine Zahl mit Vorzeichen bei Plus (Go: `signed`).
pub fn signed(x: f64) -> String {
    if x > 0.0 { format!("+{}", num(x)) } else { num(x) }
}

fn seed_list(seeds: &[String], n: usize) -> String {
    if seeds.len() <= n {
        seeds.join(", ")
    } else {
        format!("{} …", seeds[..n].join(", "))
    }
}

/// Der Vorschlag für den nächsten Lauf (Go: `nextRun`).
pub fn next_run(inn: &Insights, s: &Summary) -> NextRun {
    let mut n = NextRun {
        seeds: top_seeds(inn, 3),
        ..NextRun::default()
    };
    let w = worst(inn);
    if let (false, Some(first)) = (n.seeds.is_empty(), w.first()) {
        n.steps.push(format!(
            "Seeds {} im Replay nachspielen: {} verletzt.",
            n.seeds.join(", "),
            first.label()
        ));
        n.commands.extend(replay_commands(first, &n.seeds[0], s.seeds));
    }
    let params = tune_params(&w);
    if !params.is_empty() {
        n.steps.push(format!(
            "Hebel abstimmen: {} (Suche über höchstens drei Schritte).",
            params.join(", ")
        ));
        n.commands.extend(tune_commands(&params, s.seeds));
    }
    if let Some(c) = recommendations(&inn.lever_groups) {
        n.commands.insert(0, c);
    }
    if inn.pass && inn.counts.get(LIGHT_YELLOW).copied().unwrap_or(0) == 0 {
        n.steps.push(
            "Alle Ziele grün: Baseline neu schreiben (task balance:baseline), damit spätere Läufe dagegen vergleichen."
                .into(),
        );
    } else {
        n.steps.push(
            "Nach jeder Änderung in data/ den vollen Lauf wiederholen und mit diesem Bericht vergleichen.".into(),
        );
    }
    n.commands.push(Command {
        label: "Voller Lauf zum Vergleich".into(),
        r#where: WHERE_CHAT.into(),
        text: format!(
            r#"Starte sim_test {{"action":"start","mode":"offline","seeds":{}}} und vergleiche den neuen Bericht mit {}."#,
            s.seeds,
            run_name(inn)
        ),
    });
    n
}

/// Die Seeds, die bei den meisten verletzten Zielen genannt werden (bei Gleichstand in Reihenfolge der Datei).
fn top_seeds(inn: &Insights, n: usize) -> Vec<String> {
    let mut count: HashMap<&str, usize> = HashMap::new();
    let mut order: Vec<&str> = Vec::new();
    for t in worst(inn) {
        for s in &t.fail_seeds {
            let c = count.entry(s).or_insert(0);
            if *c == 0 {
                order.push(s);
            }
            *c += 1;
        }
    }
    order.sort_by(|a, b| count[b].cmp(&count[a])); // stabil wie Gos SortStableFunc
    order.into_iter().take(n).map(String::from).collect()
}

fn replay_commands(t: &TargetInsight, seed: &str, seeds: i64) -> Vec<Command> {
    let g = &t.entry.verdict.target;
    let file = format!(
        "reports/balance-replays/{seed}-p{}-{}-d{}.replay.json",
        g.players, g.bot, g.depth
    );
    let path = serde_json::json!({ "path": file }).to_string();
    vec![
        Command {
            label: "1. Replays schreiben".into(),
            r#where: WHERE_SHELL.into(),
            text: format!("task balance:targets -- --seeds {seeds}"),
        },
        Command {
            label: format!("2. Seed {seed} nachspielen"),
            r#where: WHERE_CHAT.into(),
            text: format!(
                "Spiele das Replay mit replay_run {path} ab und erkläre, warum {} verletzt ist.",
                t.label()
            ),
        },
    ]
}

/// Ordner des Laufs für Texte, sonst „diesem Bericht“.
fn run_name(inn: &Insights) -> &str {
    if inn.run.is_empty() || inn.run == "." {
        "diesem Bericht"
    } else {
        &inn.run
    }
}

/// Je verletztem Ziel der erste Hebel mit Wert in data/ als `datei.json:pfad=min..max`, höchstens zwei.
fn tune_params(w: &[&TargetInsight]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for t in w {
        if let Some(p) = t.levers.iter().find_map(|l| tune_param(l).filter(|p| !out.contains(p))) {
            out.push(p);
        }
        if out.len() == MAX_TUNE_PARAMS {
            break;
        }
    }
    out
}

/// Bereich ab dem Wert in data/ um höchstens drei Schritte in Richtung des Hebels; `None` ohne Wert (Go: `tuneParam`).
pub fn tune_param(l: &Lever) -> Option<String> {
    let p = parse_tune_param(&format!("{}=0..1", l.path)).ok()?;
    if p.base == 0.0 {
        return None;
    }
    let far = if l.direction == "-" {
        p.base * (1.0 - TUNE_RANGE * l.step).max(0.0)
    } else {
        p.base * (1.0 + TUNE_RANGE * l.step)
    };
    let (mut lo, mut hi) = (p.base.min(far), p.base.max(far));
    if p.int {
        (lo, hi) = (lo.floor(), hi.ceil());
    }
    if lo >= hi {
        return None;
    }
    Some(format!("{}={lo}..{hi}", l.path))
}

fn tune_commands(params: &[String], seeds: i64) -> Vec<Command> {
    let spec = format!(
        "Abstimmen: Modus {TUNE_SEARCH} · Seeds {seeds} · Parameter {}",
        params.join(" und ")
    );
    let mut cli = format!("task balance:run -- --targets --tune {TUNE_SEARCH} --seeds {seeds}");
    for p in params {
        cli += " --param ";
        cli += p;
    }
    vec![
        Command {
            label: "Abstimmen auf der Balancing-Seite".into(),
            r#where: WHERE_WORKBENCH.into(),
            text: spec,
        },
        Command {
            label: "Abstimmen lassen".into(),
            r#where: WHERE_CHAT.into(),
            text: format!(
                "Stimme mit dem Balancing-Tester (--tune {TUNE_SEARCH}) {} ab und schlage den besten Wert als Änderung in data/ vor.",
                params.join(" und ")
            ),
        },
        Command {
            label: "Abstimmen in der Shell".into(),
            r#where: WHERE_SHELL.into(),
            text: cli,
        },
    ]
}
