//! Ruinen (Port von `engine/level/ruins.go`, docs/rules/geplante-chunks.md § 8, B-468): ein gebautes, stark
//! beschädigtes Nutzgebäude an fester Lage eines geplanten Chunks. Dieses Modul prüft Art und Lage und gibt die Ruinen
//! je Stufe weiter (`Layout::ruins`); als Platz in der Sim entstehen sie in `k3c-sim`. Werte aus `data/ruins.json`
//! (`Rules::ruin`, vorher `ruinRules`); die Werteprüfung (`checkRuinRules`) liegt in `k3c_data::ruins`.

use k3c_det::{fmax, golden::go_float};

use crate::level::Gen;
use crate::planned::{EDGE_GAP_UNITS, Geometry, Placed, go_list, signed};
use crate::rules::Rules;
use crate::types::{Chunk, PlannedChunk, Ruin};

/// Mindestabstand einer Ruine zu Adern, Portalen und festen Objekten. Zu den Bauplätzen der Mauerlinien gilt keiner:
/// Ruinen dürfen überall liegen, die Lage wählt die Planung (§ 8).
pub(crate) const RUIN_GAP_UNITS: i64 = 4;

/// Go: `RuinHP`, die Start-HP einer Ruine, deren Art `hp` HP hat: `hpShare` × `hp`, aufgerundet. `hp` kommt aus dem
/// Wertesatz der Welt (sim), damit Overrides auf die HP eines Gebäudes auch seine Ruine treffen (B-692).
pub fn ruin_hp(rules: &Rules, hp: f64) -> f64 {
    (rules.ruin.hp_share * hp).ceil()
}

/// Go: `checkBuildings`, prüft die Ruine eines Chunks (höchstens eine) gegen die Lage-Regeln aus § 8. `near` sind die
/// festen Objekte des Chunks. Das Ergebnis geht in die Prüfung von `kind@x` ein.
pub(crate) fn check_buildings(
    c: &PlannedChunk,
    g: &Geometry,
    near: &[Placed],
    rules: &Rules,
) -> Result<Vec<Placed>, String> {
    let Some(b) = c.buildings.first() else {
        return Ok(Vec::new());
    };
    if c.buildings.len() > 1 {
        return Err(format!(
            "höchstens eine Ruine je Chunk, geplant sind {}",
            c.buildings.len()
        ));
    }
    if !rules.ruin.kinds.contains(&b.kind) {
        return Err(format!(
            "{:?} ist als Ruine nicht erlaubt (erlaubt: {})",
            b.kind,
            go_list(&rules.ruin.kinds)
        ));
    }
    let p = Placed {
        kind: b.kind.clone(),
        slot: c.slot,
        at: b.at,
        dist: g.dist(c.slot, b.at),
        fixed: true,
    };
    check_ruin_place(&p, g, near, rules)?;
    Ok(vec![p])
}

pub(crate) fn check_ruin_place(p: &Placed, g: &Geometry, near: &[Placed], rules: &Rules) -> Result<(), String> {
    let mut edge = EDGE_GAP_UNITS as f64;
    if let Some(info) = rules.ruin_buildings.get(&p.kind) {
        edge = fmax(edge, info.strip_units);
    }
    if p.at < edge || p.at > g.cw - edge {
        return Err(format!(
            "die Ruine {} bei {}: mindestens {} Units Abstand zur Chunk-Grenze ({}…{})",
            p.kind,
            go_float(p.at),
            go_float(edge),
            go_float(edge),
            go_float(g.cw - edge)
        ));
    }
    for h in &g.sites {
        // isRuin (sim) erkennt eine Ruine an Art und Lage: ein Hub-Platz gleicher Art auf derselben Lage wäre eine (B-523).
        if h.kind == p.kind && h.offset == p.x() {
            return Err(format!(
                "die Ruine {} liegt auf der Lage des Hub-Platzes gleicher Art ({} Units vom Hub)",
                p.kind,
                go_float(h.offset)
            ));
        }
    }
    for o in near {
        if (p.at - o.at).abs() < RUIN_GAP_UNITS as f64 {
            return Err(format!(
                "die Ruine {} bei {}, weniger als {RUIN_GAP_UNITS} Units von {} bei {}",
                p.kind,
                go_float(p.at),
                o.kind,
                go_float(o.at)
            ));
        }
    }
    Ok(())
}

/// Go: `checkRuinsAcross`, prüft den Abstand jeder Ruine zu festen Objekten anderer Chunks: Portale und Camps dürfen
/// bis an die Chunk-Grenze liegen, Ressourcen, Truhen und Adern halten dort schon den Rand-Abstand.
pub(crate) fn check_ruins_across(ruins: &[Placed], objects: &[Placed]) -> Result<(), String> {
    for r in ruins {
        for o in objects {
            if o.slot != r.slot && (r.x() - o.x()).abs() < RUIN_GAP_UNITS as f64 {
                return Err(format!(
                    "auf Platz {}: Ruine {} bei {}, weniger als {RUIN_GAP_UNITS} Units von {} auf Platz {}",
                    signed(r.slot),
                    r.kind,
                    go_float(r.at),
                    o.kind,
                    signed(o.slot)
                ));
            }
        }
    }
    Ok(())
}

#[allow(dead_code)] // RU6.5: generate ruft ruins_of
impl Gen {
    /// Go: `ruinsOf`, die Ruinen der geplanten Chunks mit ihrer X-Position; ohne Würfe, die Ströme bleiben unberührt.
    pub(crate) fn ruins_of(&self, chunks: &[Chunk]) -> Vec<Ruin> {
        let mut out = Vec::new();
        for c in chunks {
            if let Some(pc) = self.planned.get(&c.index) {
                out.extend(pc.buildings.iter().map(|b| Ruin {
                    kind: b.kind.clone(),
                    x: c.start_units + b.at,
                }));
            }
        }
        out
    }
}

#[cfg(test)]
#[path = "ruins_tests.rs"]
mod ruins_tests;
