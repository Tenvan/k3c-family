//! Platzhalter für Teile des HUD, die spätere Sprints liefern (Leitfaden § 7, Plan RU20.4). RU23 hat seine Platzhalter (Skill-Menü,
//! Seekarte, Kampf-Anzeige, Geschichte, Schiffsfahrt, Welt-Texte) gelöscht; RU53 (B-1106) hat die Shims für Sim-Werte
//! (Skills, Beutel, Zahlziele) durch `k3c_sim` ersetzt. RU22 prüft, dass keiner übrig ist.
//!
//! Übrig sind nur die Shims für RU19 (`WorldDef`, Geschichte); alles, was RU23 lieferte, steht in
//! `kampf.rs`, `voyage.rs`, `drawvoyage.rs` und `drawworld.rs`.
#![allow(clippy::unimplemented)] // Platzhalter: wartet auf den genannten Sprint (RU19)

use crate::view::frame::Frame;

/// WorldDef ist die Welt-Definition der Kampagne (Go: `sim.WorldDef`, RU19): Platzhalter ohne Inhalt; `Campaign::world`
/// gibt es in Rust erst mit RU19, `campaign_world` liefert bis dahin `None`.
#[derive(Clone, Debug, Default)]
pub struct WorldDef;

impl WorldDef {
    /// Go: `StoryStart` (RU19).
    pub fn story_start(&self) -> Vec<String> {
        unimplemented!("RU19: story_start")
    }
    /// Go: `len(w.Story.Start)` (RU19).
    pub fn story_start_len(&self) -> usize {
        unimplemented!("RU19: story_start_len")
    }
    /// Go: `StoryArrival` (RU19).
    pub fn story_arrival(&self, _island: &str) -> Vec<String> {
        unimplemented!("RU19: story_arrival")
    }
    /// Go: `StoryBoss` (RU19).
    pub fn story_boss(&self, _island: &str) -> Vec<String> {
        unimplemented!("RU19: story_boss")
    }
    /// Go: `StoryDefeated` (RU19).
    pub fn story_defeated(&self, _island: &str, _completed: &[String]) -> Vec<String> {
        unimplemented!("RU19: story_defeated")
    }
    /// Go: `StoryVictory` (RU19).
    pub fn story_victory(&self, _island: &str, _completed: &[String]) -> Vec<String> {
        unimplemented!("RU19: story_victory")
    }
}

/// Go: `f.Campaign.World` (RU19): `None`, solange `Campaign` die Welt nicht trägt.
pub fn campaign_world(_f: &Frame) -> Option<&WorldDef> {
    None
}

/// Go: `storyDoneLines` (story.go, RU19): Zahl der Zeilen der Abschlusszeile der Insel.
pub fn story_done_lines(_w: &WorldDef, _island: &str) -> usize {
    unimplemented!("RU19: story_done_lines")
}
