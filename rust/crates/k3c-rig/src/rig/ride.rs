//! Port von native/k3c-rig/rig/ride.go: Reiten. Der Reiter sitzt am Platz `sattel` des Reittiers; die Zügel laufen von
//! seiner Hand zum Maul. (Vorgriff in RU5.2, weil `frames` es braucht; RU5.3 ergänzt die Vergleichstests.)

use super::gait::GaitPose;
use super::load::{Rig, Slot};
use super::matrix::{M, Point, apply, invert, mul, rotation, scale, translate};

/// `rider_base` liefert die Basis-Matrix des Reiters: Platz `sattel` des Reittiers (Welt), zurück auf den Maßstab des
/// Reiters, Hub im Sattel, Neigung um den Sitzpunkt. Entspricht tools/paper: Ride-Matrix · translate(versatz) ·
/// rotate(lean um den Sitzpunkt).
pub fn rider_base(mount: &Rig, mount_world: &[M], rider: &Rig, p: &GaitPose) -> M {
    let seat = rider.riding.as_ref().map_or([0.0, 0.0], |r| r.seat);
    let fallback;
    let slot = match mount.slot("sattel") {
        Some(s) => s,
        None => {
            // Reittier ohne Sattel-Platz: Reiter am Ursprung des Reittiers statt Absturz
            fallback = Slot {
                part: mount.parts[0].id.clone(),
                ..Slot::default()
            };
            &fallback
        }
    };
    let mut m = mount.slot_world(slot, mount_world);
    m = mul(m, scale(1.0 / mount.scale));
    m = mul(m, translate(0.0, p.rider_y));
    m = mul(m, rotation(p.lean, 0.0, 0.0));
    mul(m, translate(-seat[0], -seat[1]))
}

/// `rein_points` liefert die Zügel als Bézier-Kurve zweiten Grades in Welt-Einheiten: Hand, Kontrollpunkt, Maul. Der
/// Durchhang zählt in der Ride-Gruppe nach unten (ride = Ride-Matrix ohne Maßstab), wie im Sheet.
pub fn rein_points(
    mount: &Rig,
    mount_world: &[M],
    rider: &Rig,
    rider_world: &[M],
    ride: &M,
) -> Option<(Point, Point, Point)> {
    let (hs, ms) = (rider.slot("hand"), mount.slot("maul"));
    let (Some(hs), Some(ms), Some(reins)) = (hs, ms, mount.reins.as_ref()) else {
        return None;
    };
    let hand = apply(&rider.slot_world(hs, rider_world), [0.0, 0.0]);
    let mouth = apply(&mount.slot_world(ms, mount_world), [0.0, 0.0]);
    let inv = invert(ride);
    let (h, m) = (apply(&inv, hand), apply(&inv, mouth));
    let ctrl = apply(ride, [(h[0] + m[0]) / 2.0, (h[1] + m[1]) / 2.0 + reins.sag]);
    Some((hand, ctrl, mouth))
}

impl Rig {
    /// `hidden` sagt, ob ein Teil des Reiters beim Reiten entfällt (das hintere Bein liegt hinter dem Reittier).
    pub fn hidden(&self, part: usize) -> bool {
        self.hide[part]
    }
}
