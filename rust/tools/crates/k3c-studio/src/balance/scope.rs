//! Szenario der Zielläufe (WZ28.4): Jeder gesetzte Wert ersetzt den Wert jedes Ziels. Die Korridore bleiben; ein Ziel
//! mit anderem Szenario ist eine Erkundung, keine Prüfung gegen den bestätigten Korridor.

use std::path::Path;

use super::targets::{ROLE_GOAL, ROLE_GUARD, Targets};

/// `None` bzw. leer = wie in `data/balance-targets.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Scope {
    pub depth: Option<i64>,
    pub players: Option<i64>,
    pub days: Option<i64>,
    pub bot: String,
    pub grade: String,
    /// Jedes Ziel spielt die ganze erste Insel dieser Welt; ohne Welt entfallen bei einer Einschränkung die Ziele mit
    /// Welt (Erkundung der Einzelstufe).
    pub world: String,
    /// Dev-Hilfen der Insel-Läufe, nur mit Welt.
    pub helps: String,
}

impl Targets {
    /// Kopie der Ziele mit dem Szenario aus `s`; ein ungültiges Szenario ist ein Fehler.
    pub fn restrict(&self, root: &Path, s: &Scope) -> Result<Targets, String> {
        let mut out = self.clone();
        let narrowed =
            s.depth.is_some() || s.players.is_some() || s.days.is_some() || !s.bot.is_empty() || !s.grade.is_empty();
        if s.world.is_empty() && narrowed {
            // eine Einschränkung erkundet die Einzelstufe: die Ziele des Insel-Laufs (BS11) entfallen
            out.targets.retain(|g| g.world.is_empty());
        }
        for g in &mut out.targets {
            let was_island = !g.world.is_empty();
            g.depth = s.depth.unwrap_or(g.depth);
            g.players = s.players.unwrap_or(g.players);
            g.days = s.days.unwrap_or(g.days);
            if !s.bot.is_empty() {
                g.bot.clone_from(&s.bot);
            }
            if !s.grade.is_empty() {
                g.grade.clone_from(&s.grade);
            }
            if !s.world.is_empty() {
                g.world.clone_from(&s.world);
                if !was_island && g.role() == ROLE_GOAL {
                    // im Insel-Lauf ist nur das Ziel der Insel (Endboss) das Ziel (B-648)
                    ROLE_GUARD.clone_into(&mut g.role);
                }
            }
            if !s.helps.is_empty() {
                g.helps.clone_from(&s.helps);
            }
            g.validate(root).map_err(|e| format!("szenario: {e}"))?;
        }
        Ok(out)
    }
}
