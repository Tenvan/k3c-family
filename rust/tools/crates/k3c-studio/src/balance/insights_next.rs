//! Tuning-Hinweise und Vorschlag für den nächsten Lauf (B-632). Die Befehle passen zu den Werkzeugen: `sim_test`
//! (voller Lauf), `task balance:targets` (Replays nach `reports/balance-replays/`), `replay_run`, Lauf-Spec der
//! Balancing-Seite und CLI für `--tune`.

use std::cmp::Reverse;
use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::hints::Lever;
use super::insights::{Insights, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, TargetInsight};
use super::insights_levers::recommendations;
use super::insights_text::{Hint, worst};
use super::report_types::nil_empty;
use super::summary::{Summary, go_fixed, num};
use super::tune::{MAX_TUNE_PARAMS, TUNE_SEARCH};
use super::tuneparam::parse_tune_param;
use crate::kitx::{go_quote, marshal_raw};

/// Befehl als Auftrag an Claude im Chat.
pub const WHERE_CHAT: &str = "chat";
/// Befehl für das Terminal im Checkout.
pub const WHERE_SHELL: &str = "shell";
/// Lauf-Spec der Balancing-Seite.
pub const WHERE_WORKBENCH: &str = "workbench";

/// Höchstens so viele Schritte eines Hebels in seine Richtung, ab dem Wert in `data/`.
pub(crate) const TUNE_RANGE: f64 = 3.0;

/// Vorschlag: Schritte als Text, Seeds zum Nachspielen und kopierbare Befehle.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct NextRun {
    pub steps: Vec<String>,
    /// Go schreibt ohne verletzte Seeds `null`.
    #[serde(serialize_with = "nil_empty")]
    pub seeds: Vec<String>,
    pub commands: Vec<Command>,
}

/// Ein kopierbarer Aufruf; `place` (JSON `where`) sagt, wohin er gehört.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Command {
    pub label: String,
    #[serde(rename = "where")]
    pub place: String,
    pub text: String,
}

fn command(label: impl Into<String>, place: &str, text: String) -> Command {
    Command {
        label: label.into(),
        place: place.to_owned(),
        text,
    }
}

/// Rot zuerst (am weitesten daneben vorn), dann knappe und ungültige Ziele in Reihenfolge des Berichts.
pub(crate) fn hints(insights: &Insights) -> Vec<Hint> {
    let mut out: Vec<Hint> = worst(insights)
        .into_iter()
        .map(|t| hint(t, LIGHT_RED, red_text(t)))
        .collect();
    for t in &insights.targets {
        if t.light == LIGHT_YELLOW {
            let text = "Knapp an der Grenze: beim Tunen anderer Ziele im Blick behalten, erst bei Rot drehen.";
            out.push(hint(t, LIGHT_YELLOW, text.to_owned()));
        } else if t.light == LIGHT_GREY {
            out.push(Hint {
                target: t.label(),
                light: LIGHT_GREY.to_owned(),
                text: format!(
                    "Läufe abgebrochen oder ohne Messwert: Seeds {} nachspielen, Fehler mit logs_errors suchen.",
                    seed_list(&t.invalid_seeds, 5)
                ),
                why: format!(
                    "{} ungültige Seeds; der Wert ist nicht belastbar.",
                    t.invalid_seeds.len()
                ),
                ..Hint::default()
            });
        }
    }
    out
}

fn hint(t: &TargetInsight, light: &str, text: String) -> Hint {
    Hint {
        target: t.label(),
        light: light.to_owned(),
        text,
        why: why_text(t),
        levers: t.levers.clone(),
        plans: t.plans.clone(),
    }
}

/// Richtung als Wort und die ersten zwei Stellschrauben; Zahlen zum Eintragen stehen in den Plänen.
fn red_text(t: &TargetInsight) -> String {
    let mut direction = "Außerhalb des Korridors".to_owned();
    let mut word = t.word().chars();
    if let Some(first) = word.next() {
        let side = if t.above() { "über" } else { "unter" };
        direction = format!("{}{} (Wert {side} dem Korridor)", first.to_uppercase(), word.as_str());
    }
    if t.levers.is_empty() {
        return format!(
            "{direction}: keine Stellschraube hinterlegt (data/balance-hints.json), Ursache im Replay suchen."
        );
    }
    let parts: Vec<&str> = t.levers.iter().take(2).map(|l| l.label.as_str()).collect();
    format!(
        "{direction}. Eine dieser Stellschrauben probieren: {} (Zahlen darunter).",
        parts.join(" oder ")
    )
}

/// Messwert, Sollwert und Lage in Klartext, dazu der Vergleich mit der Baseline.
fn why_text(t: &TargetInsight) -> String {
    if t.trend_text.is_empty() {
        return t.plain.clone();
    }
    format!("{} {}", t.plain, t.trend_text)
}

/// Zahl mit Vorzeichen.
pub(crate) fn signed(x: f64) -> String {
    if x > 0.0 {
        return format!("+{}", num(x));
    }
    num(x)
}

fn seed_list(seeds: &[String], n: usize) -> String {
    if seeds.len() <= n {
        return seeds.join(", ");
    }
    format!("{} …", seeds[..n].join(", "))
}

/// `root` liefert die Werte der Hebel.
pub(crate) fn next_run(root: &Path, insights: &Insights, s: &Summary) -> NextRun {
    let mut n = NextRun {
        seeds: top_seeds(insights, 3),
        ..NextRun::default()
    };
    let w = worst(insights);
    if let (Some(seed), Some(first)) = (n.seeds.first(), w.first()) {
        n.steps.push(format!(
            "Seeds {} im Replay nachspielen: {} verletzt.",
            n.seeds.join(", "),
            first.label()
        ));
        n.commands.extend(replay_commands(first, seed, s.seeds));
    }
    let params = tune_params(root, &w);
    if !params.is_empty() {
        n.steps.push(format!(
            "Hebel abstimmen: {} (Suche über höchstens drei Schritte).",
            params.join(", ")
        ));
        n.commands.extend(tune_commands(&params, s.seeds));
    }
    if let Some(c) = recommendations(&insights.lever_groups) {
        n.commands.insert(0, c);
    }
    let yellow = insights.counts.get(LIGHT_YELLOW).copied().unwrap_or_default();
    let step = if insights.pass && yellow == 0 {
        "Alle Ziele grün: Baseline neu schreiben (task balance:baseline), damit spätere Läufe dagegen vergleichen."
    } else {
        "Nach jeder Änderung in data/ den vollen Lauf wiederholen und mit diesem Bericht vergleichen."
    };
    n.steps.push(step.to_owned());
    let text = format!(
        r#"Starte sim_test {{"action":"start","mode":"offline","seeds":{}}} und vergleiche den neuen Bericht mit {}."#,
        s.seeds,
        run_name(insights)
    );
    n.commands.push(command("Voller Lauf zum Vergleich", WHERE_CHAT, text));
    n
}

/// Die Seeds, die bei den meisten verletzten Zielen genannt werden (bei Gleichstand in Reihenfolge der Datei).
fn top_seeds(insights: &Insights, n: usize) -> Vec<String> {
    let mut count: HashMap<&str, usize> = HashMap::new();
    let mut order: Vec<&str> = Vec::new();
    for t in worst(insights) {
        for s in &t.fail_seeds {
            let seen = count.entry(s).or_default();
            if *seen == 0 {
                order.push(s);
            }
            *seen += 1;
        }
    }
    order.sort_by_key(|s| Reverse(count.get(s).copied()));
    order.into_iter().take(n).map(str::to_owned).collect()
}

fn replay_commands(t: &TargetInsight, seed: &str, seeds: i64) -> [Command; 2] {
    let g = &t.entry.verdict.target;
    let file = format!(
        "reports/balance-replays/{seed}-p{}-{}-d{}.replay.json",
        g.players, g.bot, g.depth
    );
    let path = format!(r#"{{"path":{}}}"#, marshal_raw(&go_quote(&file)));
    [
        command(
            "1. Replays schreiben",
            WHERE_SHELL,
            format!("task balance:targets -- --seeds {seeds}"),
        ),
        command(
            format!("2. Seed {seed} nachspielen"),
            WHERE_CHAT,
            format!(
                "Spiele das Replay mit replay_run {path} ab und erkläre, warum {} verletzt ist.",
                t.label()
            ),
        ),
    ]
}

/// Ordner des Laufs für Texte, sonst „diesem Bericht“.
fn run_name(insights: &Insights) -> &str {
    if insights.run.is_empty() || insights.run == "." {
        return "diesem Bericht";
    }
    &insights.run
}

/// Je verletztem Ziel der erste Hebel mit Wert in `data/` als `datei.json:pfad=min..max`, höchstens zwei.
fn tune_params(root: &Path, worst: &[&TargetInsight]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for t in worst {
        let next = t
            .levers
            .iter()
            .find_map(|l| l.tune_param(root).filter(|p| !out.contains(p)));
        out.extend(next);
        if out.len() == MAX_TUNE_PARAMS {
            break;
        }
    }
    out
}

impl Lever {
    /// Bereich ab dem Wert in `data/` um höchstens [`TUNE_RANGE`] Schritte in Richtung des Hebels; `None` ohne Wert.
    pub(crate) fn tune_param(&self, root: &Path) -> Option<String> {
        let p = parse_tune_param(root, &format!("{}=0..1", self.path)).ok()?;
        if p.base == 0.0 {
            return None;
        }
        let far = if self.direction == "-" {
            p.base * (1.0 - TUNE_RANGE * self.step).max(0.0)
        } else {
            p.base * (1.0 + TUNE_RANGE * self.step)
        };
        let (mut lo, mut hi) = (p.base.min(far), p.base.max(far));
        if p.int {
            (lo, hi) = (lo.floor(), hi.ceil());
        }
        (lo < hi).then(|| format!("{}={}..{}", self.path, go_fixed(lo, None), go_fixed(hi, None)))
    }
}

fn tune_commands(params: &[String], seeds: i64) -> [Command; 3] {
    let list = params.join(" und ");
    let spec = format!("Abstimmen: Modus {TUNE_SEARCH} · Seeds {seeds} · Parameter {list}");
    let mut cli = format!("task balance:run -- --targets --tune {TUNE_SEARCH} --seeds {seeds}");
    for p in params {
        cli.push_str(" --param ");
        cli.push_str(p);
    }
    [
        command("Abstimmen auf der Balancing-Seite", WHERE_WORKBENCH, spec),
        command(
            "Abstimmen lassen",
            WHERE_CHAT,
            format!(
                "Stimme mit dem Balancing-Tester (--tune {TUNE_SEARCH}) {list} ab und schlage den besten Wert als \
                 Änderung in data/ vor."
            ),
        ),
        command("Abstimmen in der Shell", WHERE_SHELL, cli),
    ]
}
