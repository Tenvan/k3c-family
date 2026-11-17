//! Port von native/k3c-rig/rig/expected_test.go: Tests gegen die Sollwerte aus tools/paper (B-778/AC-02, B-789/AC-03):
//! Teil-Matrizen und Stützpunkte je Frame, Toleranz 0,01, für alle exportierten Figuren. Die Sollwerte schreibt
//! `task native:rig:export` nach assets/soll/<gruppe>/<id>/<animation>.json, beim Reittier dazu reiten-<gangart>.json
//! (Reiter: der Monarch).

use serde_json::Value;

use super::FigurePose;
use super::frames::{Ride, frame_time, ride_cloak_frames};
use super::gait::{GaitPose, LEG_NAMES, ride_matrix};
use super::load::Rig;
use super::matrix::{IDENTITY, M, Point, apply, invert, mul, translate};
use super::ride::rein_points;
use super::testutil::{animation_names, assets, load_rig, rig_dirs};

const TOLERANCE: f64 = 0.01;
const RIDER_DIR: &str = "figuren/monarch";

fn read_expected(dir: &str, name: &str) -> Vec<Value> {
    let path = assets().join("soll").join(dir).join(format!("{name}.json"));
    let text = std::fs::read(&path).unwrap_or_else(|e| panic!("Sollwerte fehlen (erst `task native:rig:export`): {e}"));
    let doc: Value = serde_json::from_slice(&text).expect("gültiges JSON");
    let frames = doc["frames"].as_array().cloned().unwrap_or_default();
    assert!(!frames.is_empty(), "{dir}/{name}: keine Frames");
    frames
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() <= TOLERANCE
}

fn num(v: &Value) -> f64 {
    v.as_f64().expect("Zahl")
}

fn point(v: &Value) -> Point {
    [num(&v[0]), num(&v[1])]
}

fn check_point(what: &str, got: Point, want: Point) {
    assert!(
        near(got[0], want[0]) && near(got[1], want[1]),
        "{what}: {got:?}, Soll {want:?}"
    );
}

fn check_points(what: &str, got: &[Point], want: &Value) {
    let want: Vec<Point> = want
        .as_array()
        .map(|a| a.iter().map(point).collect())
        .unwrap_or_default();
    assert_eq!(
        got.len(),
        want.len(),
        "{what}: {} Punkte, Soll {}",
        got.len(),
        want.len()
    );
    for (g, w) in got.iter().zip(&want) {
        check_point(what, *g, *w);
    }
}

/// checkWorld vergleicht die Matrizen aller Teile des Rigs; das Schwert des Monarchen ist im Rig ein Element am Platz
/// `waffe`.
fn check_world(what: &str, r: &Rig, got: &[M], want: &Value) {
    let map = want.as_object().cloned().unwrap_or_default();
    let mut seen = 0;
    for (id, m) in &map {
        let m: Vec<f64> = m.as_array().expect("Matrix").iter().map(num).collect();
        let i = r.index_of(id);
        let g: M = if i >= 0 {
            got[i as usize]
        } else if id == "schwert" && r.slot("waffe").is_some() {
            let s = r.slot("waffe").expect("Platz");
            mul(r.slot_world(s, got), translate(-s.point[0], -s.point[1]))
        } else {
            panic!("{what}: Teil {id} fehlt im Rig {}", r.id);
        };
        seen += 1;
        assert!(
            m.iter().enumerate().all(|(k, w)| near(g[k], *w)),
            "{what} {id}: {g:?}, Soll {m:?}"
        );
    }
    assert!(
        seen >= r.parts.len(),
        "{what}: Soll nennt {seen} Teile, das Rig hat {}",
        r.parts.len()
    );
}

/// checkAngles vergleicht die Winkel; die Posen idle und lauf nennen auch Teile, die eine Figur nicht hat (krone), die
/// zählen nicht (die Matrizen prüft checkWorld für alle Teile).
fn check_angles(what: &str, r: &Rig, got: &[f64], want: &Value) {
    for (id, w) in want.as_object().cloned().unwrap_or_default() {
        let i = r.index_of(&id);
        assert!(
            i < 0 || near(got[i as usize], num(&w)),
            "{what} Winkel {id}: Soll {w} (Index {i}, Ist {got:?})"
        );
    }
}

/// checkHidden vergleicht die ausgeblendeten Teile bei Phase p (Keyframes mit `sichtbar`).
fn check_hidden(what: &str, r: &Rig, name: &str, p: f64, want: &Value) {
    let mut got: Vec<&str> = r
        .parts
        .iter()
        .filter(|part| r.keyframe_hidden(name, &part.id, p))
        .map(|part| part.id.as_str())
        .collect();
    got.sort_unstable();
    let want: Vec<&str> = want
        .as_array()
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    assert_eq!(got, want, "{what} ausgeblendet");
}

/// checkGait vergleicht Ride, Wind, Winkel, Welt-Matrizen und Beine eines Vierbeiners mit einem Soll-Frame.
fn check_gait(what: &str, r: &Rig, p: &GaitPose, world: &[M], f: &Value) {
    let ride = &f["ride"];
    assert!(
        near(p.dy, num(&ride["dy"])) && near(p.pitch, num(&ride["pitch"])) && near(p.wind, num(&f["wind"])),
        "{what} Ride {}/{} Wind {}, Soll {ride} Wind {}",
        p.dy,
        p.pitch,
        p.wind,
        f["wind"]
    );
    check_angles(what, r, &p.angles, &f["winkel"]);
    check_world(what, r, world, &f["matrizen"]);
    for (leg, name) in LEG_NAMES.iter().enumerate() {
        let mut pts = Vec::new();
        r.leg_points(leg, &p.angles, &mut pts);
        check_points(&format!("{what} Bein {name}"), &pts[..6], &f["beine"][*name]);
    }
}

fn check_figure(r: &Rig, dir: &str, name: &str) {
    let frames = read_expected(dir, name);
    let freq = r.figure_freq(name).expect("Frequenz");
    let cloak = r.cloak.as_ref().map(|_| r.figure_cloak_frames(name).expect("Umhang"));
    let mut pose = FigurePose::default();
    for (n, f) in frames.iter().enumerate() {
        r.figure_pose(name, frame_time(n as i64, frames.len() as i64, freq), &mut pose)
            .expect("Pose");
        check_angles(name, r, &pose.angles, &f["winkel"]);
        check_point(&format!("{name} Versatz"), pose.offset, point(&f["versatz"]));
        assert!(
            near(pose.wind, num(&f["wind"])),
            "{name} Wind {}, Soll {}",
            pose.wind,
            f["wind"]
        );
        let mut world = Vec::new();
        r.world(&pose.angles, translate(pose.offset[0], pose.offset[1]), &mut world);
        check_world(name, r, &world, &f["matrizen"]);
        check_hidden(name, r, name, n as f64 / frames.len() as f64, &f["ausgeblendet"]);
        if let Some(cloak) = &cloak {
            check_points(&format!("{name} Umhang"), &cloak[n], &f["umhang"]);
        }
    }
}

fn check_gait_animation(r: &Rig, dir: &str, name: &str) {
    let frames = read_expected(dir, name);
    let mut pose = GaitPose::default();
    for (n, f) in frames.iter().enumerate() {
        r.gait_pose(&r.animations[name].gait, n as f64 / frames.len() as f64, &mut pose)
            .expect("Gangart");
        let mut world = Vec::new();
        r.world(&pose.angles, ride_matrix(pose.dy, pose.pitch, r.scale), &mut world);
        check_gait(name, r, &pose, &world, f);
    }
}

/// Jede Animation jeder Figur rechnet Rust wie tools/paper: figur mit Posen, Keyframes, Sichtbarkeit und Umhang,
/// vierbeiner mit seiner Gangart.
#[test]
fn animations_match_paper() {
    for dir in rig_dirs() {
        let r = load_rig(&dir);
        for name in animation_names(&r) {
            match r.kind.as_str() {
                "figur" => check_figure(&r, &dir, &name),
                "vierbeiner" => check_gait_animation(&r, &dir, &name),
                kind => panic!("Rig-Typ {kind} unbekannt"),
            }
        }
    }
}

fn ride_gaits(dir: &str) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(assets().join("soll").join(dir))
        .map(|d| {
            d.filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
        .iter()
        .filter_map(|n| n.strip_prefix("reiten-").and_then(|n| n.strip_suffix(".json")))
        .map(str::to_string)
        .collect()
}

fn check_ride(mount: &Rig, rider: &Rig, dir: &str, gait_name: &str) {
    let frames = read_expected(dir, &format!("reiten-{gait_name}"));
    let cloak = ride_cloak_frames(mount, rider, gait_name, frames.len() as i64).expect("Umhang");
    let mut s = Ride::default();
    for (n, f) in frames.iter().enumerate() {
        s.set(mount, rider, gait_name, n as f64 / frames.len() as f64, IDENTITY)
            .expect("Ride");
        check_gait(gait_name, mount, &s.pose, &s.mount_world, f);
        check_world(
            &format!("{gait_name} Reiter"),
            rider,
            &s.rider_world,
            &f["reiterMatrizen"],
        );
        check_points(&format!("{gait_name} Umhang"), &cloak[n], &f["umhang"]);
        let (hand, _, mouth) = rein_points(mount, &s.mount_world, rider, &s.rider_world, &s.base)
            .expect("Zügel fehlen (Plätze hand und maul)");
        let inv = invert(&s.base);
        check_point("Zügel Hand", apply(&inv, hand), point(&f["zuegel"]["hand"]));
        check_point("Zügel Maul", apply(&inv, mouth), point(&f["zuegel"]["maul"]));
    }
}

/// Jedes Reittier (Platz `sattel`) trägt den Monarchen in allen Gangarten, für die es Sollwerte gibt.
#[test]
fn ride_matches_paper() {
    let (rider, mut mounts) = (load_rig(RIDER_DIR), 0);
    for dir in rig_dirs() {
        let mount = load_rig(&dir);
        if mount.kind != "vierbeiner" || mount.slot("sattel").is_none() {
            continue;
        }
        let gaits = ride_gaits(&dir);
        assert!(!gaits.is_empty(), "{dir}: keine Reit-Sollwerte");
        for gait_name in gaits {
            check_ride(&mount, &rider, &dir, &gait_name);
        }
        mounts += 1;
    }
    assert!(mounts > 0, "kein Reittier mit Platz sattel");
}
