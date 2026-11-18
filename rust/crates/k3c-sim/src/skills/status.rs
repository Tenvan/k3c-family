//! Status-Effekte für die Anzeige (Port von `engine/sim/skills_status.go`, B-675): Kontrolleffekte der Gegner (Stun, Slow,
//! Taunt), Schild und Last Stand der Spieler und die Stürme stehen schon im Zustand; `auras_of` nennt die wirkenden Passive
//! eines Monarchen mit ihrem Radius. Nur lesen, kein rng: Spielverlauf und Golden bleiben unverändert.

use serde::Serialize;

use k3c_data::monarch::SkillEffect;

use crate::crown::crowned;
use crate::monarch::skill_ref;
use crate::state::{World, WorldValues};
use crate::world::vals;

/// Go: `Aura` ist ein wirkendes Passiv eines Monarchen: Radius in Units um ihn (0 = nur er selbst), `castle` = wirkt im
/// Burgradius statt um ihn (Holy Ground).
#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct Aura {
    #[serde(rename = "skill")]
    pub skill: String,
    #[serde(rename = "radius")]
    pub radius: f64,
    #[serde(rename = "castle", skip_serializing_if = "std::ops::Not::not")]
    pub castle: bool,
}

/// Go: `AurasOf`: die sichtbaren Passive des lebenden Monarchen `pi` in Lernreihenfolge (Auren mit Radius, Guardian,
/// Regeneration, Holy Ground, Frost Armor); reine Werte-Passive ohne Wirkort (Thick Skin, Arcane Power …) fehlen. Leer, wenn
/// keins wirkt oder der Monarch tot ist.
pub fn auras_of(w: &World, pi: usize) -> Vec<Aura> {
    let p = &w.players[pi];
    let mut out = Vec::new();
    if !crowned(p) {
        return out;
    }
    let v = vals(w);
    for id in &p.skills {
        let Some(s) = skill_ref(&v, id) else {
            continue;
        };
        if s.kind != "passive" {
            continue;
        }
        if let Some(mut a) = aura_of(&v, &s.effect) {
            a.skill = id.clone();
            out.push(a);
        }
    }
    out
}

/// Go: `auraOf`: Wirkort eines passiven Effekts; `None` bei Passiven ohne Wirkort.
fn aura_of(v: &WorldValues, e: &SkillEffect) -> Option<Aura> {
    let t = e.r#type.as_str();
    if t == "regen" && e.castle {
        return Some(Aura {
            radius: v.hub.castle_radius_units,
            castle: true,
            ..Aura::default()
        });
    }
    if t == "regen" || (t == "stat" && e.aura > 0.0) {
        return Some(Aura {
            radius: e.aura,
            ..Aura::default()
        });
    }
    if t == "guardian" {
        return Some(Aura {
            radius: e.radius,
            ..Aura::default()
        });
    }
    if t == "frostArmor" {
        return Some(Aura::default());
    }
    None
}
