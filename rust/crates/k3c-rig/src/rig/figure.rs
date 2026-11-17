//! Port von native/k3c-rig/rig/figure.go: Posen der Figuren (idle, lauf, Keyframes).

use super::fehler::Fehler;
use super::load::{Animation, Rig};
use super::matrix::Point;

pub const TAU: f64 = std::f64::consts::PI * 2.0;

/// FigurePose ist die Pose einer figur: Winkel je Teil (Grad), Versatz der ganzen Figur und Fahrtwind für den Umhang.
#[derive(Debug, Clone, Default)]
pub struct FigurePose {
    pub angles: Vec<f64>,
    pub offset: Point,
    pub wind: f64,
}

/// Frequenz der Posen des Rigs (Zyklen je Sekunde), wie tools/paper/rig/figur.ts › FIGUR_POSEN.
fn pose_freq(name: &str) -> Option<f64> {
    match name {
        "idle" => Some(0.5),
        "lauf" => Some(1.6),
        _ => None,
    }
}

impl Rig {
    /// `pose_factor`: Faktor der Figur auf den Winkel eines Teils in den Posen idle und lauf (arme, beine, kopf); 1 ohne
    /// Eintrag. Die regulären Ausdrücke von Go (`^(ober|unter)arm-`, `^(ober|unter)schenkel-`, `^(kopf|krone)$`) sind
    /// als Präfix- und Gleichheitsprüfung nachgebaut.
    pub(super) fn pose_factor(&self, id: &str) -> f64 {
        let Some(p) = &self.proportions else {
            return 1.0;
        };
        let pick = |f: Option<f64>| f.unwrap_or(1.0);
        if id.starts_with("oberarm-") || id.starts_with("unterarm-") {
            return pick(p.arms);
        }
        if id.starts_with("oberschenkel-") || id.starts_with("unterschenkel-") {
            return pick(p.legs);
        }
        if id == "kopf" || id == "krone" {
            return pick(p.head);
        }
        1.0
    }

    /// `animation` liefert die Animation oder den Fehler animation-unbekannt (wie tools/paper).
    pub(super) fn animation(&self, name: &str) -> Result<&Animation, Fehler> {
        self.animations.get(name).ok_or_else(|| {
            let names: Vec<&str> = self.animations.keys().map(String::as_str).collect();
            Fehler::Rig(format!(
                "animation-unbekannt: Animation „{name}“ fehlt in {} ({})",
                self.id,
                names.join(", ")
            ))
        })
    }

    /// `figure_freq`: Frequenz einer Animation: Pose des Rigs, sonst ein Zyklus über alle Frames bei fps (Standard 8).
    pub fn figure_freq(&self, name: &str) -> Result<f64, Fehler> {
        if let Some(f) = pose_freq(name) {
            return Ok(f);
        }
        let a = self.animation(name)?;
        let fps = if a.fps == 0.0 { 8.0 } else { a.fps };
        Ok(fps / a.frames as f64)
    }

    /// `set` schreibt den Winkel eines Teils mit dem Faktor der Figur; unbekannte Teile zählen nicht.
    fn set(&self, angles: &mut [f64], id: &str, deg: f64) {
        if let Some(&i) = self.index.get(id) {
            angles[i] = deg * self.factor[i];
        }
    }

    fn idle(&self, p: f64, out: &mut FigurePose) {
        let s = p.sin();
        self.set(&mut out.angles, "oberarm-v", 5.0 + 2.0 * s);
        self.set(&mut out.angles, "unterarm-v", -6.0 - 3.0 * s);
        self.set(&mut out.angles, "oberarm-h", -3.0 - 2.0 * s);
        self.set(&mut out.angles, "unterarm-h", -4.0);
        self.set(&mut out.angles, "kopf", 2.0 * s);
        self.set(&mut out.angles, "krone", -s);
    }

    fn walk(&self, p: f64, out: &mut FigurePose) {
        let (s, c, s2) = (p.sin(), p.cos(), (2.0 * p).sin());
        let a = &mut out.angles;
        self.set(a, "oberschenkel-v", -30.0 * s);
        self.set(a, "oberschenkel-h", 30.0 * s);
        self.set(a, "unterschenkel-v", 45.0 * f64::max(0.0, c));
        self.set(a, "unterschenkel-h", 45.0 * f64::max(0.0, -c));
        self.set(a, "oberarm-v", 25.0 * s);
        self.set(a, "unterarm-v", -15.0 - 12.0 * f64::max(0.0, -s));
        self.set(a, "oberarm-h", -25.0 * s);
        self.set(a, "unterarm-h", -15.0 - 12.0 * f64::max(0.0, s));
        self.set(a, "kopf", 2.0 * s2);
        self.set(a, "krone", -2.0 * s2);
        out.offset = [0.0, 2.5 * s.abs() - 2.0];
        out.wind = 30.0;
    }

    /// `figure_pose` rechnet die Pose der Figur zur Zeit t (Sekunden): idle und lauf als Funktionen, Keyframes addieren
    /// sich; andere Animationen brauchen Keyframes. `out.angles` wird wiederverwendet.
    pub fn figure_pose(&self, name: &str, t: f64, out: &mut FigurePose) -> Result<(), Fehler> {
        let anim = self.animation(name)?;
        let is_pose = pose_freq(name).is_some();
        if !is_pose && anim.keyframes.is_empty() {
            return Err(Fehler::Rig(format!(
                "animation-unbekannt: figur kennt ohne Keyframes nur idle und lauf, nicht „{name}“"
            )));
        }
        let freq = self.figure_freq(name).unwrap_or(0.0);
        let phase = ((t * freq) % 1.0 + 1.0) % 1.0;
        zero(&mut out.angles, self.parts.len());
        (out.offset, out.wind) = ([0.0, 0.0], 0.0);
        match name {
            "idle" => self.idle(phase * TAU, out),
            "lauf" => self.walk(phase * TAU, out),
            _ => {}
        }
        if let Some(torso) = self.proportions.as_ref().and_then(|p| p.torso) {
            out.offset[1] *= torso;
        }
        self.add_keyframes(anim, phase, &mut out.angles);
        Ok(())
    }
}

/// `zero` liefert einen genullten Slice der Länge n und nutzt den vorhandenen Speicher.
pub fn zero(v: &mut Vec<f64>, n: usize) {
    v.clear();
    v.resize(n, 0.0);
}
