//! Go: `events_location_test.go` (B-217/AC-01, B-260/AC-01): Ereignisse tragen den Ort; der Schutzplatz hängt am Index der
//! Truppe, nicht an ihrer Entity-ID.

use std::collections::BTreeSet;

use crate::unit::{shelter_x, spawn_vagrant};
use crate::world::testutil::quiet_world;

/// Zwei zusätzliche Entities vor den Truppen (z. B. neue Bauplätze) verschieben IDs, aber keine Schutzplätze.
#[test]
fn schutzplatz_ohne_entity_id() {
    let mut w = quiet_world();
    w.troops.clear();
    let hub = w.hub_x;
    for _ in 0..6 {
        let ti = spawn_vagrant(&mut w, hub, hub);
        w.troops[ti].serve();
    }
    let before: Vec<f64> = (0..w.troops.len()).map(|ti| shelter_x(&mut w, ti)).collect();
    for t in &mut w.troops {
        t.id += 3; // wie drei zusätzliche Bauplätze vor den Truppen
    }
    for (ti, want) in before.iter().enumerate() {
        assert_eq!(
            shelter_x(&mut w, ti),
            *want,
            "Truppe {ti}: Schutzplatz nach ID-Verschiebung"
        );
    }
    let seen: BTreeSet<u64> = before.iter().map(|x| x.to_bits()).collect();
    assert_eq!(seen.len(), 5, "Schutzplätze nicht über 5 Plätze gestreut: {before:?}");
}
