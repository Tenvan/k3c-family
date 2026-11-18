//! Geplante Chunks einsetzen (Port von `engine/level/planned_gen.go`): Optionen des Generators, Platzbelegung, feste
//! Objekte und Zufalls-Ressourcen aus dem eigenen Strom eines geplanten Chunks.

use std::borrow::Cow;

use k3c_rng::Rng;

use crate::biome::Biome;
use crate::level::{Gen, inside, is_event};
use crate::lines::HubSite;
use crate::planned::EDGE_GAP_UNITS;
use crate::ruins::RUIN_GAP_UNITS;
use crate::types::{Chunk, Entity, NatureValues, PlannedChunk, PlannedObject};

/// Go: `Options`, die optionalen Parameter des Generators. Der Nullwert (`Options::default()`) erzeugt das Level wie
/// `generate`: ohne Planung und mit den Daten der Basis. Es gibt keinen globalen Zustand zum Umschalten.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Options {
    /// Die geplanten Chunks dieser Stufe (docs/rules/geplante-chunks.md).
    pub planned: Vec<PlannedChunk>,
    /// Die eingestellte Chunk-Anzahl der Stufe (Inselgröße, geplante-chunks.md § 7); 0 = gewürfelte Länge aus
    /// lengthUnits wie bisher. Sie ersetzt den Wurf der Länge.
    pub chunk_count: i64,
    /// Die fest gesetzten Natur-Werte der Stufe (nature.rs, B-891); fehlende würfelt der Generator aus dem Seed.
    pub nature: NatureValues,
    /// Ob die Stufe den Bau des Endbosses trägt (docs/rules/bosse.md § 1, B-981): `None` = das Feld `lair` des Bioms
    /// (Level ohne Insel, Golden), sonst gilt der Wert in jedem Biom; die Insel setzt ihn nur für die tiefste Stufe.
    pub lair: Option<bool>,
    /// Die Hub- und Insel-Plätze der Welt (hub.sites und hub.islandSites, B-983); `None` = die der Basis. Sie halten
    /// Zahlziele und Ruinen auf Abstand.
    pub hub_sites: Option<Vec<HubSite>>,
}

impl Options {
    /// Go: `hasLair`: Die Stufe trägt den Bau des Endbosses (`lair`, sonst `Biome::lair`).
    pub fn has_lair(&self, b: &Biome) -> bool {
        self.lair.unwrap_or(b.lair)
    }
}

/// Die Zahl der Neuwürfe, bevor eine Ressource an den Rand der Ruinen-Sperrzone gesetzt wird.
const CLEAR_TRIES: usize = 32;

/// Go: `fixedObjects`, die Objekte eines geplanten Chunks mit `at` ab Chunk-Anfang; `None` bei einer Chunk-Art, die
/// ihre Objekte aus dem eigenen Strom würfelt.
pub(crate) fn fixed_objects(c: &PlannedChunk, cw: f64) -> Option<Cow<'_, [PlannedObject]>> {
    if !c.objects.is_empty() {
        return Some(Cow::Borrowed(&c.objects));
    }
    if is_event(&c.kind) || c.kind == "portal" {
        return Some(Cow::Owned(vec![PlannedObject {
            kind: c.kind.clone(),
            at: cw / 2.0,
        }]));
    }
    None
}

#[allow(dead_code)] // RU6.5: generate ruft die Methoden
impl Gen {
    /// Go: `slotIndex`, der Chunk-Index eines Platzes: +n liegt n Chunks rechts der Hub-Chunks, −n links davon.
    pub(crate) fn slot_index(&self, slot: i64) -> i64 {
        if slot > 0 {
            return self.hub_start + self.hub_chunks + slot - 1;
        }
        self.hub_start + slot
    }

    /// Go: `reserve`, belegt die geplanten Plätze direkt nach `frame()` und zählt die festen Objekte, die auf die
    /// Biom-Summen anzurechnen sind. Ein Ereignis-Chunk ohne Objektliste trägt sein Objekt in der Chunk-Mitte.
    pub(crate) fn reserve(&mut self, plan: &[PlannedChunk]) {
        self.planned.clear();
        self.fixed.clear();
        for c in plan {
            let i = self.slot_index(c.slot);
            self.kinds[i as usize] = c.kind.clone();
            self.planned.insert(i, c.clone());
            let side = usize::from(c.slot > 0);
            for o in fixed_objects(c, self.cw).iter().flat_map(|l| l.iter()) {
                *self.fixed.entry(o.kind.clone()).or_default() += 1;
                if o.kind == "portal" {
                    self.side_fixed[side] += 1;
                }
            }
        }
    }

    /// Go: `chunkStream`, der eigene Strom eines geplanten Chunks: ohne Chunk-Seed `<biom>:<seed>:chunk:<platz>` (Art
    /// fest, Inhalt wechselt mit dem Level-Seed), mit Chunk-Seed `<biom>:chunk:<chunk-seed>` (Inhalt in jedem
    /// Spielstand gleich). Der Haupt-Strom bleibt unberührt. Die Namen sind Vertrag.
    pub(crate) fn chunk_stream(&self, c: &PlannedChunk) -> Rng {
        if !c.seed.is_empty() {
            return Rng::new(&format!("{}:chunk:{}", self.b.id, c.seed));
        }
        Rng::new(&format!("{}:{}:chunk:{}", self.b.id, self.seed, c.slot))
    }

    /// Go: `plannedEntities`, setzt die Objekte eines geplanten Chunks: feste Objekte oder die Ressourcen der Chunk-Art
    /// aus dem eigenen Strom.
    pub(crate) fn planned_entities(&self, c: &Chunk, pc: &PlannedChunk) -> Vec<Entity> {
        let mut out = Vec::new();
        if let Some(objects) = fixed_objects(pc, self.cw) {
            out.extend(objects.iter().map(|o| Entity {
                kind: o.kind.clone(),
                x: c.start_units + o.at,
                ..Entity::default()
            }));
            return out;
        }
        let mut r = self.chunk_stream(pc);
        let Some(resources) = self.planned_res.get(&c.kind) else {
            return out;
        };
        for res in &resources.0 {
            for _ in 0..r.int(res.value[0], res.value[1]) {
                let x = self.clear_of_ruin(&mut r, c, pc);
                out.push(Entity {
                    kind: res.key.clone(),
                    x,
                    ..Entity::default()
                });
            }
        }
        out
    }

    /// Go: `clearOfRuin`, eine Position wie `inside`, die ≥ `RUIN_GAP_UNITS` von der Ruine des Chunks entfernt liegt
    /// (B-522). Ohne Ruine ist es genau ein Wurf wie bisher; sonst wird bis zu `CLEAR_TRIES`-mal neu gewürfelt
    /// (gleicher Strom, deterministisch) und zuletzt an den nächsten erlaubten Rand der Ruine gesetzt.
    pub(crate) fn clear_of_ruin(&self, r: &mut Rng, c: &Chunk, pc: &PlannedChunk) -> f64 {
        let mut x = inside(self.cw, r, c);
        let Some(b) = pc.buildings.first() else {
            return x;
        };
        let (gap, edge) = (RUIN_GAP_UNITS as f64, EDGE_GAP_UNITS as f64);
        let ruin = c.start_units + b.at;
        for _ in 0..CLEAR_TRIES {
            if (x - ruin).abs() >= gap {
                return x;
            }
            x = inside(self.cw, r, c);
        }
        if (x - ruin).abs() >= gap {
            return x;
        }
        let lo = ruin - gap;
        if lo >= c.start_units + edge && x < ruin {
            return lo;
        }
        let hi = ruin + gap;
        if hi <= c.start_units + self.cw - edge {
            return hi;
        }
        ruin - gap
    }
}

#[cfg(test)]
#[path = "planned_gen_tests.rs"]
mod planned_gen_tests;
