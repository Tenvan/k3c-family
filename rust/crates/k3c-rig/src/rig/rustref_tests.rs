//! Vergleich mit den Go-Referenzwerten (`native/k3c-rig/rig/rustref_test.go` → `testdata/rust/rig/*.json`), Toleranz 1e-9.

use serde_json::Value;

use super::band::{BAND_SUB, Vertex, tangent_limits};
use super::cloak::{cloak_run, frame_steps};
use super::curve::{catmull, hermite, ik, spline};
use super::figure::FigurePose;
use super::testutil::{load_rig, rig_dirs, testdata};

const TOL: f64 = 1e-9;

fn read(name: &str) -> Value {
    let path = testdata().join(format!("{name}.json"));
    let text = std::fs::read(&path).unwrap_or_else(|e| panic!("Referenz {} fehlt: {e}", path.display()));
    serde_json::from_slice(&text).expect("gültiges JSON")
}

fn f(v: &Value) -> f64 {
    v.as_f64().expect("Zahl")
}

fn fs(v: &Value) -> Vec<f64> {
    v.as_array().map(|a| a.iter().map(f).collect()).unwrap_or_default()
}

fn pts(v: &Value) -> Vec<[f64; 2]> {
    v.as_array()
        .expect("Liste")
        .iter()
        .map(|p| [f(&p[0]), f(&p[1])])
        .collect()
}

fn near(what: &str, got: f64, want: f64) {
    assert!((got - want).abs() <= TOL, "{what}: Rust {got}, Go {want}");
}

fn near_all(what: &str, got: &[f64], want: &[f64]) {
    assert_eq!(got.len(), want.len(), "{what}: Länge");
    for (i, (g, w)) in got.iter().zip(want).enumerate() {
        near(&format!("{what}[{i}]"), *g, *w);
    }
}

#[test]
fn curve_gegen_go() {
    let r = read("curve");
    for s in r["catmull"].as_array().expect("catmull") {
        let a = fs(&s["args"]);
        near("catmull", catmull(a[0], a[1], a[2], a[3], a[4]), f(&s["v"][0]));
    }
    for s in r["hermite"].as_array().expect("hermite") {
        let a = fs(&s["args"]);
        near(
            "hermite",
            hermite(a[0], a[1], a[2], a[3], a[4], a[5], a[6]),
            f(&s["v"][0]),
        );
    }
    for s in r["spline"].as_array().expect("spline") {
        let p = spline(&pts(&s["pts"]), f(&s["k"]));
        near_all("spline", &p, &fs(&s["v"]));
    }
    for s in r["ik"].as_array().expect("ik") {
        let a = fs(&s["args"]);
        let (u, v) = ik(a[0], a[1], a[2] == 1.0, a[3], a[4]);
        near_all("ik", &[u, v], &fs(&s["v"]));
    }
}

fn verts(v: &[Vertex]) -> Vec<f64> {
    v.iter().flat_map(|e| [e.x, e.y, e.u, e.v]).collect()
}

#[test]
fn band_gegen_go() {
    let r = read("band");
    let mut n = 0;
    for s in r["strips"].as_array().expect("strips") {
        let (dir, id) = (s["rig"].as_str().expect("rig"), s["band"].as_str().expect("band"));
        let rig = load_rig(dir);
        let b = rig.bands.iter().find(|b| b.id == id).expect("Band");
        let p = pts(&s["points"]);
        let mut out = Vec::new();
        b.strip(&p, f(&s["density"]), &mut out);
        let want: Vec<f64> = s["verts"].as_array().expect("verts").iter().flat_map(fs).collect();
        near_all(&format!("{dir} {id} {}", s["kind"]), &verts(&out), &want);
        let lim = fs(&s["limits"]);
        if !lim.is_empty() {
            let mut got = vec![0.0; lim.len()];
            tangent_limits(&p[..lim.len()], &mut got);
            near_all(&format!("{dir} {id} Grenzen"), &got, &lim);
        }
        n += 1;
    }
    assert!(n > 0 && BAND_SUB == 4);
    for s in r["legs"].as_array().expect("legs") {
        let rig = load_rig(s["rig"].as_str().expect("rig"));
        let mut out = Vec::new();
        rig.leg_points(f(&s["leg"]) as usize, &fs(&s["angles"]), &mut out);
        let want = pts(&s["points"]);
        let flat: Vec<f64> = out.iter().flatten().copied().collect();
        near_all(
            "Bein-Punkte",
            &flat,
            &want.iter().flatten().copied().collect::<Vec<_>>(),
        );
    }
}

#[test]
fn figure_gegen_go() {
    let r = read("figure");
    let mut n = 0;
    for s in r["poses"].as_array().expect("poses") {
        let rig = load_rig(s["rig"].as_str().expect("rig"));
        let (name, t) = (s["anim"].as_str().expect("anim"), f(&s["t"]));
        let what = format!("{} {name} t={t}", s["rig"]);
        let mut fp = FigurePose::default();
        match rig.figure_pose(name, t, &mut fp) {
            Err(e) => assert_eq!(Some(e.to_string().as_str()), s["err"].as_str(), "{what}: Fehler"),
            Ok(()) => {
                assert!(s["err"].is_null(), "{what}: Go meldet {}", s["err"]);
                near_all(&format!("{what} Winkel"), &fp.angles, &fs(&s["angles"]));
                near_all(&format!("{what} Versatz"), &fp.offset, &fs(&s["offset"]));
                near(&format!("{what} Wind"), fp.wind, f(&s["wind"]));
            }
        }
        match rig.figure_freq(name) {
            Err(e) => assert_eq!(
                Some(e.to_string().as_str()),
                s["freq_err"].as_str(),
                "{what}: Frequenz-Fehler"
            ),
            Ok(q) => near(&format!("{what} Frequenz"), q, f(&s["freq"])),
        }
        n += 1;
    }
    assert!(n > 0);
}

#[test]
fn cloak_gegen_go() {
    let r = read("figure");
    let mut n = 0;
    for s in r["cloaks"].as_array().expect("cloaks") {
        let rig = load_rig(s["rig"].as_str().expect("rig"));
        let spec = rig.cloak.as_ref().expect("Umhang");
        let steps = frame_steps(1.3, 6);
        let want_steps: Vec<f64> = fs(&s["steps"]);
        assert_eq!(
            steps.iter().map(|&k| k as f64).collect::<Vec<_>>(),
            want_steps,
            "Schritte"
        );
        let mut state = |k: i64| {
            let wind = if (k + 1000) % 200 >= 100 { 25.0 } else { 0.0 };
            (
                [2.0 * (k as f64 / 60.0).sin(), -50.0 + 3.0 * (k as f64 / 45.0).cos()],
                wind,
            )
        };
        let frames = cloak_run(spec, &steps, &mut state, f(&s["floor"]));
        let want = s["frames"].as_array().expect("frames");
        assert_eq!(frames.len(), want.len());
        for (i, (g, w)) in frames.iter().zip(want).enumerate() {
            let flat: Vec<f64> = g.iter().flatten().copied().collect();
            near_all(
                &format!("{} Umhang Frame {i}", s["rig"]),
                &flat,
                &pts(w).iter().flatten().copied().collect::<Vec<_>>(),
            );
        }
        n += 1;
    }
    assert!(n > 0);
    assert!(!rig_dirs().is_empty());
}
