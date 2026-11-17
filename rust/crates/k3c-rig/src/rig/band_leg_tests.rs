//! Port der Gangart-Tests aus native/k3c-rig/rig/band_test.go (`TestLegStripNeverFolds`, `TestLegStripContinuous`).

use super::band::Vertex;
use super::band_tests::fold_at;
use super::gait::{GaitPose, LEG_NAMES};
use super::load::Rig;
use super::matrix::Point;
use super::testutil::{animation_names, load_rig, rig_dirs};

/// Vierbeiner, deren Bein-Band bekannt umklappt. Seit B-791 leer: Die Tangenten-Grenze hält die Kurve am Band-Ende
/// (Elefant), unfold die innere Kante an engen Beugungen (Wölfe im Galopp).
const KNOWN_FOLDS: [&str; 0] = [];

fn leg_strip(r: &Rig, leg: usize, angles: &[f64]) -> (Vec<Point>, Vec<Vertex>) {
    let (mut pts, mut strip) = (Vec::new(), Vec::new());
    r.leg_points(leg, angles, &mut pts);
    r.bands[leg].strip(&pts, r.density, &mut strip);
    (pts, strip)
}

fn check_leg_folds(dir: &str, r: &Rig, gait_name: &str) {
    let mut pose = GaitPose::default();
    for step in 0..64 {
        r.gait_pose(gait_name, f64::from(step) / 64.0, &mut pose)
            .expect("Gangart");
        for (leg, name) in LEG_NAMES.iter().enumerate() {
            let i = fold_at(&leg_strip(r, leg, &pose.angles).1);
            assert!(
                i < 0,
                "{dir} {gait_name} Phase {step}/64, Bein {name}: Band klappt bei Querschnitt {i} um"
            );
        }
    }
}

/// Ein Bein-Band klappt in keiner Gangart um: Alle Dreiecke behalten ihren Umlaufsinn (sonst zeigt das Bild Streifen).
#[test]
fn leg_strip_never_folds() {
    for dir in rig_dirs() {
        let r = load_rig(&dir);
        if r.kind != "vierbeiner" || KNOWN_FOLDS.contains(&dir.as_str()) {
            continue;
        }
        for name in animation_names(&r) {
            check_leg_folds(&dir, &r, &r.animations[&name].gait);
        }
    }
}

fn max_move(a: &[Point], b: &[Point]) -> f64 {
    a.iter()
        .zip(b)
        .fold(0.0, |m, (p, q)| f64::max(m, (q[0] - p[0]).hypot(q[1] - p[1])))
}

fn max_vertex_move(a: &[Vertex], b: &[Vertex]) -> f64 {
    a.iter()
        .zip(b)
        .fold(0.0, |m, (p, q)| f64::max(m, (q.x - p.x).hypot(q.y - p.y)))
}

fn check_leg_jumps(dir: &str, r: &Rig, gait_name: &str, steps: i64, leg_jump: f64, min_jump: f64) {
    let mut pose = GaitPose::default();
    let mut prev: Vec<(Vec<Point>, Vec<Vertex>)> = vec![(Vec::new(), Vec::new()); LEG_NAMES.len()];
    for step in 0..=steps {
        r.gait_pose(gait_name, (step % steps) as f64 / steps as f64, &mut pose)
            .expect("Gangart");
        for (leg, name) in LEG_NAMES.iter().enumerate() {
            let (pts, strip) = leg_strip(r, leg, &pose.angles);
            if step > 0 && strip.len() == prev[leg].1.len() {
                let limit = f64::max(leg_jump * max_move(&prev[leg].0, &pts), min_jump);
                let j = max_vertex_move(&prev[leg].1, &strip);
                assert!(
                    j <= limit,
                    "{dir} {gait_name} Phase {step}/{steps}, Bein {name}: Ecke springt {j:.3} (Grenze {limit:.3})"
                );
            }
            prev[leg] = (pts, strip);
        }
    }
}

/// Zwischen benachbarten Phasen (1/512) bewegt sich keine Ecke eines Bein-Bands sprunghaft: höchstens leg_jump mal so
/// weit wie der am weitesten bewegte Stützpunkt des Beins (plus eine kleine Grenze für Phasen fast ohne Bewegung).
#[test]
fn leg_strip_continuous() {
    const STEPS: i64 = 512;
    for dir in rig_dirs() {
        let r = load_rig(&dir);
        if r.kind != "vierbeiner" {
            continue;
        }
        for name in animation_names(&r) {
            check_leg_jumps(&dir, &r, &r.animations[&name].gait, STEPS, 4.0, 0.05);
        }
    }
}
