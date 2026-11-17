//! Fackeln (Port von `engine/sim/torches.go`): Sie gehören zu gebauter Mauer und gebautem Turm (kein Kauf, keine
//! Bauoption) und leuchten nachts. Die Versätze stehen in `data/light.json › torches` (`k3c_data::light::Light::torches`);
//! Gos Paket-Variable `torchOffsets` gibt es nicht, die Versätze kommen als Parameter. Kein Zustand, nichts im Protokoll.

use std::collections::BTreeMap;

use super::common::sign;
use super::{Site, World};

/// Go: `TorchXs`: die Orte (Units) aller Fackeln der Welt, in der Reihenfolge der Plätze.
pub fn torch_xs_of(w: &World, offsets: &BTreeMap<String, Vec<f64>>) -> Vec<f64> {
    torch_xs(&w.sites, w.hub_x, offsets)
}

/// Go: `torchXs`: Nur gebaute Plätze mit Versätzen tragen Fackeln; links der Hub-Mitte werden die Versätze gespiegelt, an
/// einer zerstörten Mauer (wieder unbezahlt) entfallen sie.
pub fn torch_xs(sites: &[Site], hub_x: f64, offsets: &BTreeMap<String, Vec<f64>>) -> Vec<f64> {
    let mut out = Vec::new();
    for s in sites {
        if s.state != "built" {
            continue;
        }
        let mut side = sign(s.x - hub_x);
        if side == 0.0 {
            side = 1.0;
        }
        for o in offsets.get(&s.kind).into_iter().flatten() {
            out.push(s.x + side * o);
        }
    }
    out
}
