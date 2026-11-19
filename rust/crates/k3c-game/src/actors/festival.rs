//! Das Fest im Hub (RU62.13, B-1002): Solange `World::festival` gilt, tanzen die freien Bürger (Bauern ohne Arbeit,
//! Landstreicher) im Hub und die Truppen jubeln an ihrem Posten. Es gibt keine neuen Animationen: die vorhandenen Posen
//! laufen gespiegelt (Blickrichtung wechselt im Takt) und je Figur versetzt (`anim::shift`). Arbeitende Bauern und
//! Handwerker arbeiten weiter; wer läuft oder kämpft, behält seine Pose.

use std::f64::consts::TAU;

use super::anim::{ANIM_ATTACK, ANIM_IDLE, Want, shift};

/// Wie weit vom Hub entfernt ein freier Bürger noch tanzt, in Units.
pub const HUB_DANCE_RANGE: f64 = 14.0;
/// Takte des Tanzes (Wechsel der Blickrichtung je Sekunde) und des Jubelns der Truppen.
const DANCE_HZ: f64 = 1.1;
const CHEER_HZ: f64 = 0.45;

/// Die Truppe, die feiert (Art, Id, Ort, Hub, Arbeit, Zeit).
#[derive(Clone, Copy, Debug)]
pub struct Guest<'a> {
    pub kind: &'a str,
    pub id: i64,
    pub x: f64,
    pub hub_x: f64,
    /// Arbeit am Ziel
    pub working: bool,
    pub now: f64,
}

/// `festival_want`: Wunsch und Blickrichtung des Gastes im Fest. `want` und `dir` sind das Ergebnis ohne Fest; gibt dasselbe
/// zurück, wenn die Truppe nicht feiert.
pub fn festival_want(g: Guest<'_>, want: Want, dir: i64) -> (Want, i64) {
    if want.anim != ANIM_IDLE || dir != 0 || g.working {
        return (want, dir);
    }
    if g.kind == "peasant" || g.kind == "vagrant" {
        if (g.x - g.hub_x).abs() > HUB_DANCE_RANGE {
            return (want, dir);
        }
        // Tanz: die Schlag-Pose in Schleife, gespiegelt im Takt, je Figur versetzt
        let w = Want {
            anim: ANIM_ATTACK.to_string(),
            ..Want::default()
        };
        return (w, mirror(g.now, DANCE_HZ, g.id));
    }
    // Jubel: die Ruhe-Pose, langsam gespiegelt
    (want, mirror(g.now, CHEER_HZ, g.id))
}

/// `mirror`: Blickrichtung (-1 oder 1) einer Figur im Takt `hz`, versetzt nach der Id.
pub fn mirror(now: f64, hz: f64, id: i64) -> i64 {
    if (TAU * (now * hz + shift(id) * 3.0)).sin() >= 0.0 {
        1
    } else {
        -1
    }
}

#[cfg(test)]
#[path = "festival_tests.rs"]
mod festival_tests;
