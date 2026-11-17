//! Vergleich der Zeichen-Aufrufe mit den Go-Referenz-Netzen (`native/k3c-rig/paint/rustref_test.go` →
//! `testdata/rust/rig/paint/figures.json`): je Figur und Pose Bilder (Matrix) und Dreiecksnetze (Ecken, Indizes).
//! Toleranz je Koordinate 1e-6 plus 2,5e-7 relativ (Go rundet die Ecken auf `float32`; ein Unterschied im letzten Bit der
//! Rechnung kann ein `float32`-Bit kippen). Indizes müssen gleich sein.

use std::collections::BTreeMap;

use k3c_platform::TexId;
use serde_json::Value;

use super::actor::{Actor, State};
use super::assets::Assets;
use super::painter::Painter;
use super::testutil::{Call, RecCanvas};
use crate::rig::matrix::{mul, scale, translate};
use crate::rig::testutil::{assets, testdata};

fn text(v: &Value) -> &str {
    v.as_str().expect("Text")
}

fn near(what: &str, got: f64, want: f64) {
    let tol = 1e-6 + 2.5e-7 * want.abs();
    assert!((got - want).abs() <= tol, "{what}: Rust {got}, Go {want}");
}

fn near_all(what: &str, got: &[f64], want: &Value) {
    let want: Vec<f64> = want
        .as_array()
        .expect("Liste")
        .iter()
        .map(|x| x.as_f64().expect("Zahl"))
        .collect();
    assert_eq!(got.len(), want.len(), "{what}: Länge");
    for (i, (g, w)) in got.iter().zip(&want).enumerate() {
        near(&format!("{what}[{i}]"), *g, *w);
    }
}

fn check_call(what: &str, got: &Call, want: &Value, names: &BTreeMap<TexId, String>) {
    let tex_name = |t: &TexId| names[t].clone();
    let want_tex = text(&want["tex"]).replace('\\', "/");
    match (got, text(&want["kind"])) {
        (Call::Sprite { tex, src, tf, tint }, "image") => {
            assert_eq!(tex_name(tex), want_tex, "{what}: Bild");
            assert_eq!(
                (src.w as u64, src.h as u64),
                (want["w"].as_u64().unwrap(), want["h"].as_u64().unwrap())
            );
            let g = [tf.a, tf.b, tf.c, tf.d, tf.tx, tf.ty].map(f64::from);
            near_all(&format!("{what} Matrix"), &g, &want["geom"]);
            near(
                &format!("{what} Alpha"),
                f64::from(tint[3]),
                want["alpha"].as_f64().unwrap(),
            );
        }
        (Call::Mesh { tex, verts, idx }, "mesh") => {
            assert_eq!(tex_name(tex), want_tex, "{what}: Netz-Bild");
            let want_verts = want["verts"].as_array().expect("verts");
            assert_eq!(verts.len(), want_verts.len(), "{what}: Ecken");
            for (i, (v, w)) in verts.iter().zip(want_verts).enumerate() {
                let g = [
                    v.pos[0], v.pos[1], v.uv[0], v.uv[1], v.color[0], v.color[1], v.color[2], v.color[3],
                ];
                near_all(&format!("{what} Ecke {i}"), &g.map(f64::from), w);
            }
            let want_idx: Vec<u16> = want["idx"]
                .as_array()
                .expect("idx")
                .iter()
                .map(|x| x.as_u64().unwrap() as u16)
                .collect();
            assert_eq!(idx, &want_idx, "{what}: Indizes");
        }
        (got, kind) => panic!("{what}: Rust {got:?}, Go {kind}"),
    }
}

/// Jede Figur (zu Fuß, Vierbeiner allein, Reiter auf Reittier) zeichnet nach 24 Takten dieselben Bilder und Netze wie Go.
#[test]
fn figuren_gegen_go() {
    let path = testdata().join("paint/figures.json");
    let want: Value = serde_json::from_slice(&std::fs::read(&path).expect("Referenz fehlt")).expect("JSON");
    let mut canvas = RecCanvas::default();
    let mut a = Assets::load(&assets(), &mut canvas).expect("Assets");
    let (mut figures, mut meshes, mut images) = (0, 0, 0);
    for sc in want.as_array().expect("Figuren") {
        let id = text(&sc["id"]);
        let mut rig = |key: &str| (!text(&sc[key]).is_empty()).then(|| a.rig(text(&sc[key])).expect("Rig"));
        let (rider, mount) = (rig("rider"), rig("mount"));
        let state = State {
            name: text(&sc["name"]).to_string(),
            gait: text(&sc["gait"]).to_string(),
        };
        let mut ac = Actor::start(&mut a, &mut canvas, (rider, mount), text(&sc["variant"]), state).expect(id);
        ac.weapon = text(&sc["weapon"]).to_string();
        ac.armor = text(&sc["armor"]).to_string();
        for _ in 0..24 {
            ac.update(1.0 / 60.0).expect(id);
        }
        let mut p = Painter::new(a.white);
        (p.view, p.alpha) = (mul(translate(100.0, 200.0), scale(1.5)), 0.8);
        canvas.calls.clear();
        ac.draw(&mut p, &mut canvas, &a);
        let mut names: BTreeMap<TexId, String> = a.images.iter().map(|(k, v)| (v.tex, k.clone())).collect();
        names.insert(a.white.tex, "white".to_string());
        let calls = sc["calls"].as_array().expect("calls");
        assert_eq!(canvas.calls.len(), calls.len(), "{id}: Zahl der Aufrufe");
        for (i, (g, w)) in canvas.calls.iter().zip(calls).enumerate() {
            check_call(&format!("{id} Aufruf {i}"), g, w, &names);
            match g {
                Call::Sprite { .. } => images += 1,
                Call::Mesh { .. } => meshes += 1,
            }
        }
        figures += 1;
    }
    assert!(
        figures > 100 && meshes > 100 && images > 1000,
        "{figures} Figuren, {meshes} Netze, {images} Bilder"
    );
}
