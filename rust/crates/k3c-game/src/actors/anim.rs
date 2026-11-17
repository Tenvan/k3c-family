//! Port von `native/k3c-game/actors/anim.go`.
//!
//! Zustand → Animation als reine Funktionen (wie src/scenes/render/monarchAnim.ts, mountGait.ts und animate in
//! worldRenderer.ts): welche Animation bzw. Gangart eine Figur zeigt, wann sie beginnt und wie lange eine
//! Einmal-Animation läuft. Ohne Grafik, ohne Zustand außerhalb der Argumente.

use k3c_det::{fmax, fmin};

use super::pending::UNIT_PX;

// Animationen der Rigs und Sheets (Namen wie in den Papier-Specs) und Gangarten der Reittiere.
pub const ANIM_IDLE: &str = "idle";
pub const ANIM_RUN: &str = "lauf";
pub const ANIM_ATTACK: &str = "angriff";
pub const ANIM_DEATH: &str = "death";
pub const ANIM_RESPAWN: &str = "respawn";

pub const GAIT_STAND: &str = "stand";
pub const GAIT_WALK: &str = "schritt";
pub const GAIT_TROT: &str = "trab";
pub const GAIT_GALLOP: &str = "galopp";

// Schwellen der Gangart als Anteil des Referenz-Tempos (mountGait.ts) und Ruhe-Grenze in Units/s.
pub const TROT_FROM: f64 = 0.45;
pub const GALLOP_FROM: f64 = 0.8;
pub const MOVE_EPS: f64 = 0.05;

/// movingEps: Weg je Takt in Units, ab dem eine Figur läuft (MOVING_PX = 0,5 Spiel-Pixel im Browser-Client).
pub const MOVING_EPS: f64 = 0.5 / UNIT_PX;

/// Pose ist die gezeigte Animation einer Figur: Name (zu Fuß) bzw. Gangart (beritten, Vierbeiner), Beginn und Länge.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pose {
    /// Animation der Figur bzw. bei Gait der Name der Gangart
    pub anim: String,
    /// Gangart des Reittiers; leer = zu Fuß bzw. Vierbeiner nach Anim
    pub gait: String,
    /// Zeit (Sekunden der Darstellung) des Beginns
    pub start: f64,
    /// Länge einer Einmal-Animation in Sekunden; 0 = Schleife
    pub dur: f64,
    /// Einmal-Animation hält am Ende (Tod)
    pub hold: bool,
}

/// Want ist der Wunsch des Zustands: Animation, Einmal-Animation und ob sie neu beginnt (neuer Schlag).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Want {
    pub anim: String,
    pub gait: String,
    pub once: bool,
    pub hold: bool,
    pub restart: bool,
}

impl Pose {
    /// Running meldet, ob eine Einmal-Animation (ohne Halten) zur Zeit now noch läuft.
    pub fn running(&self, now: f64) -> bool {
        self.dur > 0.0 && !self.hold && now - self.start < self.dur
    }
}

/// Next liefert die Pose nach dem Wunsch w zur Zeit now und ob sie neu beginnt. Dieselbe Animation läuft ohne
/// Restart weiter; eine laufende Einmal-Animation (Schlag, Respawn) geht einer Schleife vor; Dur setzt der Aufrufer.
pub fn next(cur: &Pose, w: &Want, now: f64) -> (Pose, bool) {
    if cur.anim == w.anim && cur.gait == w.gait && !w.restart {
        return (cur.clone(), false);
    }
    if !w.once && cur.running(now) {
        return (cur.clone(), false);
    }
    (
        Pose {
            anim: w.anim.clone(),
            gait: w.gait.clone(),
            start: now,
            hold: w.hold,
            ..Pose::default()
        },
        true,
    )
}

/// AnimTime liefert die Zeit in der Animation: Schleifen mit Versatz shift (Figuren derselben Art laufen nicht im
/// Gleichschritt), Einmal-Animationen ab 0, haltende bleiben kurz vor dem Ende stehen.
pub fn anim_time(p: &Pose, now: f64, shift: f64) -> f64 {
    let t = fmax(now - p.start, 0.0);
    if p.dur == 0.0 {
        t + shift
    } else if p.hold {
        fmin(t, p.dur * 0.999)
    } else {
        t
    }
}

/// SheetFrame liefert das Bild eines Sheets mit frames Bildern bei fps zur Animationszeit t; eine Einmal-Animation
/// (once) bleibt auf dem letzten Bild stehen.
#[allow(clippy::cast_possible_truncation)]
pub fn sheet_frame(t: f64, fps: f64, frames: i64, once: bool) -> i64 {
    if frames <= 0 || fps <= 0.0 {
        return 0;
    }
    let i = (t * fps) as i64;
    if once {
        return i.min(frames - 1);
    }
    i % frames
}

/// MountGait liefert die Gangart beim Tempo vx und Referenz-Tempo ref (mountGait.ts) und ob das Reittier läuft;
/// im Stand steht es (Gangart stand).
pub fn mount_gait(vx: f64, r#ref: f64) -> (&'static str, bool) {
    let v = vx.abs();
    if v <= MOVE_EPS {
        return (GAIT_STAND, false);
    }
    let r = if r#ref > 0.0 { v / r#ref } else { 1.0 };
    if r >= GALLOP_FROM {
        (GAIT_GALLOP, true)
    } else if r >= TROT_FROM {
        (GAIT_TROT, true)
    } else {
        (GAIT_WALK, true)
    }
}

/// FallbackGait liefert die Gangart, die das Reittier hat: fehlt sie (Elefant ohne Galopp), die nächst langsamere.
pub fn fallback_gait(g: &str, has: impl Fn(&str) -> bool) -> String {
    let order = [GAIT_GALLOP, GAIT_TROT, GAIT_WALK, GAIT_STAND];
    for (i, o) in order.iter().enumerate() {
        if *o != g {
            continue;
        }
        for f in &order[i..] {
            if has(f) {
                return (*f).to_string();
            }
        }
    }
    g.to_string()
}

/// MonarchFrame ist der Teil des Monarchen-Zustands, den die Animation braucht (wie monarchAnim.ts).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MonarchFrame {
    /// es gibt einen Vor-Zustand
    pub known: bool,
    pub vx: f64,
}

/// MonarchWant wählt die Animation des Monarchen: beritten in der Gangart nach Tempo (ohne Tod und Respawn, B-930).
/// Treffer und Münze-Geben zeigt er beritten nicht (wie mountView.ts).
pub fn monarch_want(_prev: MonarchFrame, cur: MonarchFrame, r#ref: f64, has: impl Fn(&str) -> bool) -> Want {
    let (g, _) = mount_gait(cur.vx, r#ref);
    let g = fallback_gait(g, has);
    Want {
        anim: g.clone(),
        gait: g,
        ..Want::default()
    }
}

/// Motion ist die Bewegung einer Truppe oder eines Gegners zwischen zwei Takten.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Motion {
    /// es gibt einen Vor-Zustand
    pub known: bool,
    pub prev_x: f64,
    pub x: f64,
    pub prev_cd: f64,
    pub cd: f64,
    /// Cooldown zählt Schläge (Kämpfer, Gegner), nicht die Wartezeit beim Umherwandern
    pub attack_cooldown: bool,
}

/// FigureWant wählt die Animation einer Truppe oder eines Gegners (animate in worldRenderer.ts) und die Richtung
/// (-1 links, 1 rechts, 0 unverändert): Ein steigender Cooldown ist ein neuer Schlag.
pub fn figure_want(m: &Motion) -> (Want, i64) {
    let want = |anim: &str| Want {
        anim: anim.to_string(),
        ..Want::default()
    };
    if !m.known {
        return (want(ANIM_IDLE), 0);
    }
    let dx = m.x - m.prev_x;
    let mut dir = 0;
    if dx.abs() > MOVING_EPS {
        dir = 1.0_f64.copysign(dx) as i64;
    }
    if m.attack_cooldown && m.cd > m.prev_cd + 1e-6 {
        return (
            Want {
                anim: ANIM_ATTACK.to_string(),
                once: true,
                restart: true,
                ..Want::default()
            },
            dir,
        );
    }
    if dir != 0 {
        return (want(ANIM_RUN), dir);
    }
    (want(ANIM_IDLE), 0)
}

/// Shift liefert den Zeitversatz der Schleifen einer Figur aus ihrer ID (fest, ohne Zufall).
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::manual_rem_euclid)]
pub fn shift(id: i64) -> f64 {
    (((id % 17) + 17) % 17) as f64 * 0.113
}

#[cfg(test)]
#[path = "anim_tests.rs"]
mod anim_tests;
