//! Umfang der Zielläufe (Port von `engine/balance/scope.go`, WZ28.4, CLI `--depth`, `--players`, `--bot`, `--grade`,
//! `--days`): Jeder gesetzte Wert ersetzt den Wert jedes Ziels, `None` bzw. leer = wie in data/balance-targets.json. Die
//! Korridore bleiben; ein Ziel mit anderem Szenario ist eine Erkundung, keine Prüfung gegen den bestätigten Korridor.

use k3c_data::balance_targets::{Target, Targets};

use crate::bots::{bot_by_name, check_players};
use crate::fehler::Fehler;
use crate::helps::check_helps;
use crate::island_world::{load_world, shared};
use crate::targets::{measure_kind, measure_names};

/// Die Rolle „goal“ (Go: `RoleGoal`).
pub const ROLE_GOAL: &str = "goal";
/// Die Rolle „guard“ (Go: `RoleGuard`).
pub const ROLE_GUARD: &str = "guard";
/// Die Rolle „info“ (Go: `RoleInfo`).
pub const ROLE_INFO: &str = "info";

/// Legt das Szenario der Zielläufe fest.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Scope {
    pub depth: Option<i64>,
    pub players: Option<i64>,
    pub days: Option<i64>,
    pub bot: String,
    pub grade: String,
    /// BS11: jedes Ziel spielt die ganze erste Insel dieser Welt; ohne Welt entfallen bei einer Einschränkung die Ziele mit
    /// Welt (Erkundung der Einzelstufe).
    pub world: String,
    /// Dev-Hilfen der Insel-Läufe (`helps.rs`), nur mit Welt.
    pub helps: String,
}

/// Die Rolle eines Ziels; ohne Angabe gilt es als goal (ältere Dateien und Berichte; Go: `Target.role`).
pub fn role(g: &Target) -> &str {
    if g.role.is_empty() { ROLE_GOAL } else { &g.role }
}

/// Die Prüfung eines Ziels (Go: `Target.validate`), auch Kennzahl und Art gegen die Tabelle `measures` (`targets.rs`).
pub fn validate_target(g: &Target) -> Result<(), Fehler> {
    if !matches!(g.role.as_str(), "" | ROLE_GOAL | ROLE_GUARD | ROLE_INFO) {
        return Err(Fehler::Daten(format!("role {:?}, erlaubt: goal, guard, info", g.role)));
    }
    check_helps(&g.helps, &g.world)?;
    let Some(kind) = measure_kind(&g.measure) else {
        return Err(Fehler::Daten(format!(
            "kennzahl {:?} unbekannt (bekannt: {})",
            g.measure,
            measure_names().join(", ")
        )));
    };
    if g.name.is_empty() || g.rule.is_empty() {
        return Err(Fehler::Daten("name und rule sind Pflicht".into()));
    }
    if g.kind != kind.name() {
        return Err(Fehler::Daten(format!(
            "art {:?} passt nicht zur Kennzahl {} ({})",
            g.kind,
            g.measure,
            kind.name()
        )));
    }
    if bot_by_name(&g.bot).is_none() {
        return Err(Fehler::Profil(format!("unbekannter Bot {:?}", g.bot)));
    }
    if g.players < 1 || g.players > 4 || g.days < 1 {
        return Err(Fehler::Daten(format!(
            "players 1 bis 4 und days ab 1 nötig, war {} und {}",
            g.players, g.days
        )));
    }
    check_players(&g.bot, g.players)?;
    check_grade(&g.grade)?;
    if !g.world.is_empty() {
        load_world(&g.world).map_err(|e| Fehler::Daten(format!("welt {:?}: {e}", g.world)))?;
    }
    validate_bounds(g)
}

fn validate_bounds(g: &Target) -> Result<(), Fehler> {
    let bad = |s: String| Err(Fehler::Daten(s));
    match (g.lower, g.upper) {
        (None, None) => return bad("mindestens eine Grenze nötig".into()),
        (Some(lo), Some(hi)) if lo > hi => {
            return bad(format!(
                "untergrenze {} über Obergrenze {}",
                k3c_det::golden::go_float(lo),
                k3c_det::golden::go_float(hi)
            ));
        }
        _ => {}
    }
    if g.margin.is_some_and(|m| m < 0.0) {
        return bad("margin darf nicht negativ sein".into());
    }
    for b in [g.lower, g.upper].into_iter().flatten() {
        if g.kind == "share" && !(0.0..=100.0).contains(&b) {
            return bad(format!(
                "grenze {} eines Anteils muss in 0 bis 100 liegen",
                k3c_det::golden::go_float(b)
            ));
        }
    }
    Ok(())
}

/// Leer (normal) oder ein Grad aus data/difficulty.json (Go: `checkGrade`).
fn check_grade(grade: &str) -> Result<(), Fehler> {
    if grade.is_empty() {
        return Ok(());
    }
    let grades = &shared()?.data.difficulty.grades;
    if grades.contains_key(grade) {
        return Ok(());
    }
    // Go nennt die Grade nach Wellengröße sortiert (`Grades`), bei Gleichstand nach Namen.
    let mut names: Vec<(&String, f64)> = grades.iter().map(|(n, g)| (n, g.wave_size)).collect();
    names.sort_by(|a, b| a.1.total_cmp(&b.1).then_with(|| a.0.cmp(b.0)));
    let names: Vec<&str> = names.iter().map(|(n, _)| n.as_str()).collect();
    Err(Fehler::Daten(format!(
        "unbekannter Grad {grade:?} (bekannt: [{}])",
        names.join(" ")
    )))
}

/// Die Methode `Restrict` von `Targets`.
pub trait Restrict: Sized {
    /// Eine Kopie der Ziele mit dem Szenario aus `s`; ein ungültiges Szenario oder ein unbekannter Grad ist ein Fehler.
    fn restrict(&self, s: &Scope) -> Result<Self, Fehler>;
}

impl Restrict for Targets {
    fn restrict(&self, s: &Scope) -> Result<Self, Fehler> {
        let mut out = self.clone();
        let narrowed =
            s.depth.is_some() || s.players.is_some() || s.days.is_some() || !s.bot.is_empty() || !s.grade.is_empty();
        if s.world.is_empty() && narrowed {
            // eine Einschränkung erkundet die Einzelstufe: die Ziele des Insel-Laufs (World, BS11) entfallen
            out.targets.retain(|g| g.world.is_empty());
        }
        for g in &mut out.targets {
            apply(g, s);
            validate_target(g).map_err(|e| Fehler::Daten(format!("szenario: {e}")))?;
        }
        Ok(out)
    }
}

/// Setzt die Werte aus `s` in ein Ziel.
fn apply(g: &mut Target, s: &Scope) {
    let was_island = !g.world.is_empty();
    g.depth = s.depth.unwrap_or(g.depth);
    g.players = s.players.unwrap_or(g.players);
    g.days = s.days.unwrap_or(g.days);
    if !s.bot.is_empty() {
        g.bot = s.bot.clone();
    }
    if !s.grade.is_empty() {
        g.grade = s.grade.clone();
    }
    if !s.world.is_empty() {
        g.world = s.world.clone();
        if !was_island && role(g) == ROLE_GOAL {
            // im Insel-Lauf ist nur das Ziel der Insel (Endboss) das Ziel (B-648)
            g.role = ROLE_GUARD.into();
        }
    }
    if !s.helps.is_empty() {
        g.helps = s.helps.clone();
    }
}
