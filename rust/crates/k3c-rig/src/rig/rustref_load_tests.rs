//! Vergleich mit den Go-Referenzwerten für Laden, Zeichenreihenfolge und Knochen-Matrizen (`load.json`, `world.json`),
//! dazu Fehlerfälle von `Rig::load`.

use std::path::Path;

use serde_json::Value;

use super::fehler::Fehler;
use super::figure::FigurePose;
use super::gait::{GaitPose, ride_matrix};
use super::load::Rig;
use super::matrix::{M, invert, mul, rotation, translate};
use super::rustref_tests::{f, fs, near_all, read};
use super::testutil::load_rig;

fn s(v: &Value) -> &str {
    v.as_str().expect("Text")
}

fn ints(v: &Value) -> Vec<i64> {
    fs(v).iter().map(|x| *x as i64).collect()
}

#[test]
fn load_gegen_go() {
    let r = read("load");
    let mut n = 0;
    for l in r["rigs"].as_array().expect("rigs") {
        let rig = load_rig(s(&l["rig"]));
        let what = s(&l["rig"]).to_string();
        near_all(&format!("{what} Maßstab"), &[rig.scale], &[f(&l["scale"])]);
        assert_eq!(rig.parent, ints(&l["parent"]), "{what}: Eltern");
        let topo: Vec<i64> = rig.topo.iter().map(|&i| i as i64).collect();
        assert_eq!(topo, ints(&l["topo"]), "{what}: Topo-Reihenfolge");
        near_all(&format!("{what} Faktoren"), &rig.factor, &fs(&l["factor"]));
        let hide: Vec<bool> = l["hide"]
            .as_array()
            .expect("hide")
            .iter()
            .map(|b| b.as_bool().expect("bool"))
            .collect();
        assert_eq!(rig.hide, hide, "{what}: ausgeblendet");
        let order: Vec<(i64, i64, f64)> = rig
            .order
            .iter()
            .map(|i| (i.kind as i64, i.index as i64, i.layer))
            .collect();
        let want: Vec<(i64, i64, f64)> = l["order"]
            .as_array()
            .expect("order")
            .iter()
            .map(|i| (f(&i["kind"]) as i64, f(&i["index"]) as i64, f(&i["layer"])))
            .collect();
        assert_eq!(order, want, "{what}: Zeichenreihenfolge");
        assert_eq!(rig.rider.to_vec(), ints(&l["rider"]), "{what}: Reiter-Teile");
        assert_eq!(rig.legs.upper.to_vec(), ints(&l["upper"]), "{what}: Oberbeine");
        assert_eq!(rig.legs.lower.to_vec(), ints(&l["lower"]), "{what}: Unterbeine");
        assert_eq!(rig.legs.hoof.to_vec(), ints(&l["hoof"]), "{what}: Hufe");
        assert_eq!(
            vec![rig.legs.neck, rig.legs.tail, rig.legs.back],
            ints(&l["neck_tail_back"]),
            "{what}: Hals"
        );
        near_all(&format!("{what} Beinlängen"), &[rig.legs.l1, rig.legs.l2], &fs(&l["l"]));
        let names: Vec<&String> = rig.animations.keys().collect();
        let want_names: Vec<&str> = l["anim_names"]
            .as_array()
            .map(|a| a.iter().map(s).collect())
            .unwrap_or_default();
        assert_eq!(names, want_names, "{what}: Animationen");
        for (name, ids) in want_names.iter().zip(l["anim_ids"].as_array().expect("anim_ids")) {
            let want_ids: Vec<&str> = ids.as_array().map(|a| a.iter().map(s).collect()).unwrap_or_default();
            assert_eq!(rig.animations[*name].ids, want_ids, "{what} {name}: Keyframe-Teile");
        }
        for (p, files) in rig.parts.iter().zip(l["files"].as_array().expect("files")) {
            let got = format!(
                "{}|{}",
                rig.image_file(&p.id, &p.image.file, "x"),
                rig.image_file(&p.id, &p.image.file, "")
            );
            assert_eq!(got, s(files), "{what} {}: Bilddatei", p.id);
        }
        n += 1;
    }
    assert!(n > 0);
}

fn flat(m: &[M]) -> Vec<f64> {
    m.iter().flatten().copied().collect()
}

fn mats(v: &Value) -> Vec<f64> {
    v.as_array()
        .map(|a| a.iter().flat_map(fs).collect())
        .unwrap_or_default()
}

#[test]
fn world_gegen_go() {
    let r = read("world");
    let mut n = 0;
    for w in r["worlds"].as_array().expect("worlds") {
        let rig = load_rig(s(&w["rig"]));
        let (name, t) = (s(&w["pose"]), f(&w["t"]));
        let what = format!("{} {name} t={t}", w["rig"]);
        let (angles, base): (Vec<f64>, M) = if rig.kind == "figur" {
            let mut fp = FigurePose::default();
            rig.figure_pose(name, t, &mut fp).expect("Pose");
            (fp.angles, translate(fp.offset[0], fp.offset[1]))
        } else {
            let mut gp = GaitPose::default();
            rig.gait_pose(&rig.animations[name].gait, t, &mut gp).expect("Gangart");
            (gp.angles, ride_matrix(gp.dy, gp.pitch, rig.scale))
        };
        let mut world = Vec::new();
        rig.world(&angles, base, &mut world);
        near_all(&format!("{what} Matrizen"), &flat(&world), &mats(&w["world"]));
        let slots: Vec<M> = rig.slots.iter().map(|sl| rig.slot_world(sl, &world)).collect();
        near_all(&format!("{what} Plätze"), &flat(&slots), &mats(&w["slots"]));
        near_all(
            &format!("{what} Inverse"),
            &invert(&mul(base, rotation(33.0, 4.0, -7.0))),
            &fs(&w["inv"]),
        );
        let hidden: Vec<bool> = (0..rig.parts.len()).map(|i| rig.hidden(i)).collect();
        let want: Vec<bool> = w["hidden"]
            .as_array()
            .expect("hidden")
            .iter()
            .map(|b| b.as_bool().expect("bool"))
            .collect();
        assert_eq!(hidden, want, "{what}: hidden");
        n += 1;
    }
    assert!(n > 0);
}

fn temp_rig(name: &str, json: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../.work/tmp").join(format!("k3c-rig-test-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("Ordner");
    std::fs::write(dir.join("rig.json"), json).expect("Datei");
    dir
}

/// Fehlerfälle von Load: fehlende Datei, kaputtes JSON, unbekannte Eltern, Kreis, Band und Platz an unbekanntem Teil.
#[test]
fn load_fehler() {
    let missing = Rig::load(Path::new("gibt-es-nicht")).expect_err("fehlt");
    assert!(
        matches!(&missing, Fehler::Fehlt(m) if m.starts_with("Rig-Datei fehlt: ") && m.contains("task native:rig:export"))
    );
    let broken = Rig::load(&temp_rig("kaputt", "{")).expect_err("kaputt");
    assert!(matches!(&broken, Fehler::Kaputt(m) if m.starts_with("Rig-Datei kaputt: ")));
    let cases = [
        (
            r#"{"id":"a","teile":[{"id":"x","eltern":"y"}]}"#,
            "Rig a: Teil x nennt unbekannte Eltern y",
        ),
        (
            r#"{"id":"a","teile":[{"id":"x","eltern":"y"},{"id":"y","eltern":"x"}]}"#,
            "Rig a: Eltern der Teile bilden einen Kreis",
        ),
        (
            r#"{"id":"a","teile":[{"id":"x"}],"baender":[{"id":"b","teil":"q"}]}"#,
            "Rig a: Band b hängt an unbekanntem Teil q",
        ),
        (
            r#"{"id":"a","teile":[{"id":"x"}],"plaetze":[{"name":"p","teil":"q"}]}"#,
            "Rig a: Platz p hängt an unbekanntem Teil q",
        ),
    ];
    for (i, (json, want)) in cases.iter().enumerate() {
        let e = Rig::load(&temp_rig(&format!("f{i}"), json)).expect_err("Fehler");
        assert_eq!(e.to_string(), *want);
    }
}
