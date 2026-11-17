//! Port von native/k3c-rig/rig/frames.go: Zustände wie in den Sheets (tools/paper/rig/animation.ts, reiten.ts): Pose je
//! Frame und der eingeschwungene Umhang. Für den Test gegen die Sollwerte und den Bildvergleich; im Spiel läuft der Umhang
//! mit der echten Zeit.

use super::cloak::{cloak_run, frame_steps, step_time};
use super::fehler::Fehler;
use super::figure::FigurePose;
use super::gait::{GaitPose, gait_freq, ride_matrix};
use super::load::Rig;
use super::matrix::{IDENTITY, M, Point, apply, invert, mul, translate};
use super::ride::rider_base;

/// RIDE_FLOOR: tiefster y-Wert des Umhangs beim Reiten (Rücken des Reittiers, Ride-Gruppe).
const RIDE_FLOOR: f64 = -103.0;

/// `frame_time`: Pose-Zeit von Frame n einer Animation mit `frames` Frames und Frequenz freq.
pub fn frame_time(n: i64, frames: i64, freq: f64) -> f64 {
    n as f64 / (freq * frames as f64)
}

impl Rig {
    /// `cloak_anchor`: Anker des Umhangs in Welt-Einheiten (Punkt am Anker-Teil).
    pub fn cloak_anchor(&self, world: &[M]) -> Point {
        let cloak = self.cloak.as_ref().expect("Rig hat einen Umhang");
        apply(&world[self.index_of(&cloak.anchor) as usize], cloak.point)
    }

    /// `figure_cloak_frames` liefert die Umhang-Kette je Frame einer figur-Animation zu Fuß (Boden y = 0).
    pub fn figure_cloak_frames(&self, name: &str) -> Result<Vec<Vec<Point>>, Fehler> {
        let freq = self.figure_freq(name)?;
        let mut pose = FigurePose::default();
        let mut world: Vec<M> = Vec::new();
        let mut err: Option<Fehler> = None;
        let mut state = |k: i64| {
            err = self.figure_pose(name, step_time(k), &mut pose).err();
            if err.is_some() {
                return ([0.0, 0.0], 0.0);
            }
            self.world(&pose.angles, translate(pose.offset[0], pose.offset[1]), &mut world);
            (self.cloak_anchor(&world), pose.wind)
        };
        let frames = self.animations[name].frames;
        let cloak = self.cloak.as_ref().expect("Rig hat einen Umhang");
        let out = cloak_run(cloak, &frame_steps(freq, frames), &mut state, 0.0);
        match err {
            Some(e) => Err(e),
            None => Ok(out),
        }
    }
}

/// Ride hält den Zustand eines Reiters auf seinem Reittier bei einer Phase: Posen, Welt-Matrizen und Ride-Matrix.
#[derive(Debug, Clone)]
pub struct Ride {
    pub pose: GaitPose,
    pub mount_world: Vec<M>,
    pub rider_world: Vec<M>,
    /// Ride-Matrix ohne Maßstab (Hub und Neigung); darin liegen Umhang und Durchhang der Zügel
    pub base: M,
    angles: Vec<f64>,
}

impl Default for Ride {
    fn default() -> Ride {
        Ride {
            pose: GaitPose::default(),
            mount_world: Vec::new(),
            rider_world: Vec::new(),
            base: [0.0; 6],
            angles: Vec::new(),
        }
    }
}

impl Ride {
    /// `set` rechnet Reittier und Reiter bei Phase ph der Gangart; at verschiebt beide (Ort in der Welt).
    pub fn set(&mut self, mount: &Rig, rider: &Rig, gait_name: &str, ph: f64, at: M) -> Result<(), Fehler> {
        mount.gait_pose(gait_name, ph, &mut self.pose)?;
        self.base = mul(at, ride_matrix(self.pose.dy, self.pose.pitch, 1.0));
        let mount_base = mul(at, ride_matrix(self.pose.dy, self.pose.pitch, mount.scale));
        mount.world(&self.pose.angles, mount_base, &mut self.mount_world);
        rider.rider_angles(&self.pose, &mut self.angles);
        let rider_base = rider_base(mount, &self.mount_world, rider, &self.pose);
        rider.world(&self.angles, rider_base, &mut self.rider_world);
        Ok(())
    }

    /// `cloak_anchor`: Anker des Umhangs in der Ride-Gruppe (dort rechnet die Physik beim Reiten).
    pub fn cloak_anchor(&self, rider: &Rig) -> Point {
        apply(&invert(&self.base), rider.cloak_anchor(&self.rider_world))
    }
}

/// `ride_cloak_frames` liefert die Umhang-Kette je Frame beim Reiten (Koordinaten der Ride-Gruppe, Boden −103).
pub fn ride_cloak_frames(mount: &Rig, rider: &Rig, gait_name: &str, frames: i64) -> Result<Vec<Vec<Point>>, Fehler> {
    let freq = gait_freq(gait_name)?;
    let mut s = Ride::default();
    let mut err: Option<Fehler> = None;
    let mut state = |k: i64| {
        let ph = ((step_time(k) * freq) % 1.0 + 1.0) % 1.0;
        err = s.set(mount, rider, gait_name, ph, IDENTITY).err();
        if err.is_some() {
            return ([0.0, 0.0], 0.0);
        }
        (s.cloak_anchor(rider), s.pose.wind)
    };
    let cloak = rider.cloak.as_ref().expect("Reiter hat einen Umhang");
    let out = cloak_run(cloak, &frame_steps(freq, frames), &mut state, RIDE_FLOOR);
    match err {
        Some(e) => Err(e),
        None => Ok(out),
    }
}

/// `ride_floor`: Boden des Umhangs beim Reiten.
pub fn ride_floor() -> f64 {
    RIDE_FLOOR
}
