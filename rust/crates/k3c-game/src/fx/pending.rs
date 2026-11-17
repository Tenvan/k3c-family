//! Platzhalter für Funktionen späterer Sprints (Leitfaden § 7).

use k3c_sim::state::{Player, World};

/// Go: `sim.Aura` (engine/sim/skills_status.go): ein wirkendes Passiv eines Monarchen. Radius in Units um ihn
/// (0 = nur er selbst), `castle` = wirkt im Burgradius statt um ihn (Holy Ground). RU18 löscht den Typ und nimmt den
/// der Sim.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Aura {
    pub skill: String,
    pub radius: f64,
    pub castle: bool,
}

/// Go: `sim.AurasOf`.
pub fn aura_of(_w: &World, _p: &Player) -> Vec<Aura> {
    unimplemented!("RU18: auras_of")
}
