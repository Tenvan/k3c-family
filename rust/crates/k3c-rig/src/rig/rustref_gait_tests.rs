//! Vergleich mit den Go-Referenzwerten für Gangarten, Frames, Reiten und Keyframes (`gait.json`, `frames.json`), 1e-9.

use serde_json::Value;

use super::frames::{Ride, frame_time, ride_cloak_frames};
use super::gait::{GaitPose, gait_freq};
use super::matrix::{M, translate};
use super::ride::rein_points;
use super::rustref_tests::{f, fs, near, near_all, near_all_tol, pts, read};
use super::testutil::load_rig;

fn flat(p: &[[f64; 2]]) -> Vec<f64> {
    p.iter().flatten().copied().collect()
}

fn frames_flat(v: &Value) -> Vec<Vec<f64>> {
    v.as_array()
        .map(|a| a.iter().map(|fr| flat(&pts(fr))).collect())
        .unwrap_or_default()
}

fn mats(v: &Value) -> Vec<f64> {
    v.as_array().expect("Matrizen").iter().flat_map(fs).collect()
}

fn s(v: &Value) -> &str {
    v.as_str().expect("Text")
}

#[test]
fn gait_gegen_go() {
    let r = read("gait");
    let mut n = 0;
    for g in r["gaits"].as_array().expect("gaits") {
        let rig = load_rig(s(&g["rig"]));
        let (name, ph) = (s(&g["gait"]), f(&g["phase"]));
        let what = format!("{} {name} ph={ph}", g["rig"]);
        if let Ok(q) = gait_freq(name) {
            near(&format!("{what} Frequenz"), q, f(&g["freq"]));
        }
        let mut p = GaitPose::default();
        match rig.gait_pose(name, ph, &mut p) {
            Err(e) => assert_eq!(Some(e.to_string().as_str()), g["err"].as_str(), "{what}: Fehler"),
            Ok(()) => {
                assert!(g["err"].is_null(), "{what}: Go meldet {}", g["err"]);
                near_all(&format!("{what} Winkel"), &p.angles, &fs(&g["angles"]));
                near_all(
                    &format!("{what} dy,pitch,wind,rider_y,lean"),
                    &[p.dy, p.pitch, p.wind, p.rider_y, p.lean],
                    &[
                        f(&g["dy"]),
                        f(&g["pitch"]),
                        f(&g["wind"]),
                        f(&g["rider_y"]),
                        f(&g["lean"]),
                    ],
                );
                near_all(&format!("{what} Reiter"), &p.rider, &fs(&g["rider"]));
                let mut ra = Vec::new();
                rig.rider_angles(&p, &mut ra);
                near_all(&format!("{what} Reiter-Winkel"), &ra, &fs(&g["rider_angles"]));
            }
        }
        n += 1;
    }
    assert!(n > 0);
}

#[test]
fn frames_gegen_go() {
    let r = read("frames");
    for (i, t) in fs(&r["frame_times"]).chunks(2).enumerate() {
        let n = [0, 1, 5][i];
        near("frame_time", frame_time(n, 8, 1.6), t[0]);
        near("frame_time", frame_time(n, 4, 0.5), t[1]);
    }
    for c in r["cloaks"].as_array().expect("cloaks") {
        let rig = load_rig(s(&c["rig"]));
        let what = format!("{} {} Umhang", c["rig"], c["anim"]);
        match rig.figure_cloak_frames(s(&c["anim"])) {
            Err(e) => assert_eq!(Some(e.to_string().as_str()), c["err"].as_str(), "{what}: Fehler"),
            Ok(frames) => {
                let want = frames_flat(&c["frames"]);
                assert_eq!(frames.len(), want.len(), "{what}: Frames");
                for (k, (g, w)) in frames.iter().zip(&want).enumerate() {
                    near_all(&format!("{what} Frame {k}"), &flat(g), w);
                }
            }
        }
    }
}

#[test]
fn ride_gegen_go() {
    let r = read("frames");
    let rider = load_rig("figuren/monarch");
    for c in r["ride_cloaks"].as_array().expect("ride_cloaks") {
        let (mount, gait) = (s(&c["mount"]), s(&c["gait"]));
        let what = format!("{mount} {gait} Reiter-Umhang");
        match ride_cloak_frames(&load_rig(mount), &rider, gait, 6) {
            Err(e) => assert_eq!(Some(e.to_string().as_str()), c["err"].as_str(), "{what}: Fehler"),
            Ok(frames) => {
                let want = frames_flat(&c["frames"]);
                assert_eq!(frames.len(), want.len(), "{what}: Frames");
                for (k, (g, w)) in frames.iter().zip(&want).enumerate() {
                    near_all_tol(&format!("{what} Frame {k}"), &flat(g), w, 1e-6);
                }
            }
        }
    }
    for c in r["rides"].as_array().expect("rides") {
        let mount = load_rig(s(&c["mount"]));
        let what = format!("{} {} ph={}", c["mount"], c["gait"], c["phase"]);
        let mut st = Ride::default();
        st.set(&mount, &rider, s(&c["gait"]), f(&c["phase"]), translate(10.0, -3.0))
            .expect("Ride");
        near_all(&format!("{what} Basis"), &st.base, &fs(&c["base"]));
        let flat_m = |m: &[M]| m.iter().flatten().copied().collect::<Vec<f64>>();
        near_all(
            &format!("{what} Reittier-Matrizen"),
            &flat_m(&st.mount_world),
            &mats(&c["mount_world"]),
        );
        near_all(
            &format!("{what} Reiter-Matrizen"),
            &flat_m(&st.rider_world),
            &mats(&c["rider_world"]),
        );
        if rider.cloak.is_some() {
            near_all(&format!("{what} Anker"), &st.cloak_anchor(&rider), &fs(&c["anchor"]));
        }
        let rein = rein_points(&mount, &st.mount_world, &rider, &st.rider_world, &st.base);
        match rein {
            Some((h, k, m)) => near_all(
                &format!("{what} Zügel"),
                &[h[0], h[1], k[0], k[1], m[0], m[1]],
                &fs(&c["rein"]),
            ),
            None => assert!(c["rein"].is_null(), "{what}: Go hat Zügel"),
        }
    }
}

#[test]
fn keyframe_hidden_gegen_go() {
    let r = read("frames");
    let mut n = 0;
    for h in r["hidden"].as_array().expect("hidden") {
        let rig = load_rig(s(&h["rig"]));
        let (anim, ph) = (s(&h["anim"]), f(&h["phase"]));
        let mut got: Vec<&str> = rig
            .parts
            .iter()
            .filter(|p| rig.keyframe_hidden(anim, &p.id, ph))
            .map(|p| p.id.as_str())
            .collect();
        got.sort_unstable();
        let want: Vec<&str> = h["parts"].as_array().expect("parts").iter().map(s).collect();
        assert_eq!(got, want, "{} {anim} ph={ph}: ausgeblendet", h["rig"]);
        n += 1;
    }
    assert!(n > 0);
}
