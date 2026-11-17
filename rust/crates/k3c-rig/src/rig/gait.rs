//! Port von native/k3c-rig/rig/gait.go: Gangarten des Vierbeiners.
//! Vorgriff in RU5.1: Beine, Reiter-Teile und `LegIndex` (der Rig-Ladevorgang braucht sie); der Rest folgt in RU5.2.

use super::load::Rig;

// Beine in fester Reihenfolge: hinten fern, hinten nah, vorn fern, vorn nah.
pub const LEG_HF: usize = 0;
pub const LEG_HN: usize = 1;
pub const LEG_VF: usize = 2;
pub const LEG_VN: usize = 3;
pub const LEG_COUNT: usize = 4;

/// LEG_NAMES nennt die Beine in der Reihenfolge der Bänder bein-<name>.
pub const LEG_NAMES: [&str; LEG_COUNT] = ["hf", "hn", "vf", "vn"];

/// RIDER_PARTS nennt die Teile des Reiters, die eine Gangart setzt (Reihenfolge von GaitPose.rider).
pub const RIDER_PARTS: [&str; 8] = [
    "oberschenkel-v",
    "unterschenkel-v",
    "oberarm-v",
    "unterarm-v",
    "oberarm-h",
    "unterarm-h",
    "kopf",
    "krone",
];

/// LegIndex: Teile-Index von Oberbein, Unterbein und Huf je Bein, dazu Hals, Schweif und Rückengelenk (-1 = fehlt).
#[derive(Debug, Clone, Default)]
pub struct LegIndex {
    pub upper: [i64; LEG_COUNT],
    pub lower: [i64; LEG_COUNT],
    pub hoof: [i64; LEG_COUNT],
    pub neck: i64,
    pub tail: i64,
    pub back: i64,
    /// Längen der beiden IK-Knochen.
    pub l1: f64,
    pub l2: f64,
}

impl Rig {
    pub(super) fn leg_indices(&self) -> LegIndex {
        let mut x = LegIndex {
            neck: self.index_of("hals"),
            tail: self.index_of("schweif"),
            back: self.index_of("rumpf-hinten"),
            l1: 25.0,
            l2: 31.0,
            ..LegIndex::default()
        };
        for (i, n) in LEG_NAMES.iter().enumerate() {
            x.upper[i] = self.index_of(&format!("oberbein-{n}"));
            x.lower[i] = self.index_of(&format!("unterbein-{n}"));
            x.hoof[i] = self.index_of(&format!("huf-{n}"));
        }
        // Oberbein = Knie − Hüfte, Unterbein bis zum Boden (tools/paper/rig/proportionen.ts › beinLaengen)
        let (h, k) = (x.upper[LEG_HF], x.lower[LEG_HF]);
        if h >= 0 && k >= 0 {
            let (hip, knee) = (self.parts[h as usize].pivot[1], self.parts[k as usize].pivot[1]);
            (x.l1, x.l2) = (knee - hip, -hip - (knee - hip));
        }
        x
    }
}
