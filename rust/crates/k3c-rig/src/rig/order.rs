//! Port von native/k3c-rig/rig/order.go: Zeichenreihenfolge und Welt-Matrizen der Teile.

use super::fehler::Fehler;
use super::load::{Rig, Slot};
use super::matrix::{M, mul, rotation, translate};

/// ItemKind sagt, was an einer Stelle der Zeichenreihenfolge steht.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemKind {
    #[default]
    Part,
    Band,
    Slot,
    Reins,
}

/// Item ist ein Eintrag der Zeichenreihenfolge; `index` zeigt in `parts`, `bands` oder `slots`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Item {
    pub kind: ItemKind,
    pub index: usize,
    pub layer: f64,
}

impl Rig {
    /// `build_order` sortiert Teile mit Bild, Bänder, Plätze und Zügel nach Ebene (bei Gleichstand in dieser Reihenfolge).
    pub(super) fn build_order(&mut self) -> Result<(), Fehler> {
        for (i, p) in self.parts.iter().enumerate() {
            if !p.image.file.is_empty() {
                self.order.push(Item {
                    kind: ItemKind::Part,
                    index: i,
                    layer: p.layer,
                });
            }
        }
        for (i, b) in self.bands.iter().enumerate() {
            if !b.part.is_empty() && self.index_of(&b.part) < 0 {
                return Err(Fehler::Rig(format!(
                    "Rig {}: Band {} hängt an unbekanntem Teil {}",
                    self.id, b.id, b.part
                )));
            }
            self.order.push(Item {
                kind: ItemKind::Band,
                index: i,
                layer: b.layer,
            });
        }
        for (i, s) in self.slots.iter().enumerate() {
            if self.index_of(&s.part) < 0 {
                return Err(Fehler::Rig(format!(
                    "Rig {}: Platz {} hängt an unbekanntem Teil {}",
                    self.id, s.name, s.part
                )));
            }
            self.order.push(Item {
                kind: ItemKind::Slot,
                index: i,
                layer: s.layer,
            });
        }
        if let Some(reins) = &self.reins {
            self.order.push(Item {
                kind: ItemKind::Reins,
                index: 0,
                layer: reins.layer,
            });
        }
        self.order
            .sort_by(|a, b| a.layer.partial_cmp(&b.layer).unwrap_or(std::cmp::Ordering::Equal));
        Ok(())
    }

    /// `world` rechnet die Welt-Matrix je Teil: Ein Teil dreht um seinen Pivot, Kinder erben die Drehung; base wirkt auf
    /// alle. `angles` und `out` haben einen Eintrag je Teil (Reihenfolge von `parts`); `out` wird wiederverwendet.
    pub fn world(&self, angles: &[f64], base: M, out: &mut Vec<M>) {
        out.resize(self.parts.len(), [0.0; 6]);
        for &i in &self.topo {
            let parent = if self.parent[i] >= 0 {
                out[self.parent[i] as usize]
            } else {
                base
            };
            let p = &self.parts[i];
            out[i] = mul(parent, rotation(angles[i], p.pivot[0], p.pivot[1]));
        }
    }

    /// `slot_world` liefert die Welt-Matrix eines Platzes: Teil · Verschiebung zum Punkt · Drehung des Platzes.
    pub fn slot_world(&self, s: &Slot, world: &[M]) -> M {
        let part = world[self.index_of(&s.part) as usize];
        mul(
            part,
            mul(translate(s.point[0], s.point[1]), rotation(s.rotation, 0.0, 0.0)),
        )
    }
}
