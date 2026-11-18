//! Tests für `golden` (RU3/AC-03): alle Golden laden, Selbstvergleich, Pfad der ersten Abweichung.

use std::io::Write;

use serde_json::{Value, json};

use super::{SIM_TOL, diff, diff_tol, go_float, read, testdata, tree};

fn golden_files() -> Vec<std::path::PathBuf> {
    let mut files: Vec<_> = std::fs::read_dir(testdata().join("golden"))
        .expect("testdata/golden lesen")
        .map(|e| e.expect("Eintrag").path())
        .filter(|p| p.to_string_lossy().ends_with(".json") || p.to_string_lossy().ends_with(".json.gz"))
        .collect();
    files.sort();
    files
}

fn rng() -> Value {
    read(testdata().join("golden/rng.json"))
}

#[test]
fn alle_golden_laden_und_gleichen_sich_selbst() {
    let files = golden_files();
    assert_eq!(files.len(), 23, "Golden unter testdata/golden: {files:?}");
    let gz = files
        .iter()
        .filter(|p| p.extension().is_some_and(|e| e == "gz"))
        .count();
    assert_eq!(gz, 15, "davon .json.gz");
    for p in files {
        let v = read(&p);
        assert!(v.is_object(), "{}: oberste Ebene kein Objekt", p.display());
        assert_eq!(diff("golden", &v, &v.clone()), "", "{}", p.display());
    }
}

#[test]
fn geaenderte_zahl() {
    let want = rng();
    let mut got = want.clone();
    got["seeds"][1]["next"][3] = json!(0.25);
    let w = want["seeds"][1]["next"][3].as_f64().expect("Zahl");
    assert_eq!(
        diff("rng", &want, &got),
        format!("rng.seeds[1].next[3] = 0.25, erwartet {}", go_float(w))
    );
}

#[test]
fn kleinste_abweichung_einer_zahl() {
    let want = json!({"x": 0.1});
    let got = json!({"x": f64::from_bits(0.1_f64.to_bits() + 1)});
    assert_eq!(diff("z", &want, &got), "z.x = 0.10000000000000002, erwartet 0.1");
}

#[test]
fn fehlendes_feld() {
    let want = rng();
    let mut got = want.clone();
    got["seeds"][0].as_object_mut().expect("Objekt").remove("hash");
    assert_eq!(
        diff("rng", &want, &got),
        "rng.seeds[0].hash (Schlüssel nur auf einer Seite)"
    );
    assert_eq!(
        diff("rng", &got, &want),
        "rng.seeds[0].hash (Schlüssel nur auf einer Seite)"
    );
}

#[test]
fn vertauschte_liste() {
    let want = rng();
    let mut got = want.clone();
    let list = got["seeds"][0]["weighted"].as_array_mut().expect("Liste");
    let i = (1..list.len())
        .find(|&i| list[i] != list[0])
        .expect("zwei verschiedene");
    list.swap(0, i);
    let (w, g) = (&want["seeds"][0]["weighted"][0], &got["seeds"][0]["weighted"][0]);
    assert_eq!(
        diff("rng", &want, &got),
        format!(
            "rng.seeds[0].weighted[0] = {}, erwartet {}",
            g.as_str().expect("Text"),
            w.as_str().expect("Text")
        )
    );
}

#[test]
fn null_und_leere_liste_sind_gleich() {
    assert_eq!(diff("a", &json!(null), &json!([])), "");
    assert_eq!(diff("a", &json!([]), &json!(null)), "");
    assert_eq!(diff("a", &json!({"l": null}), &json!({"l": []})), "");
    assert_eq!(diff("a", &json!(null), &json!([1])), "a = [1], erwartet <nil>");
    assert_eq!(diff("a", &json!([1]), &json!(null)), "a (Länge)");
    assert_eq!(diff("a", &json!([1]), &json!([1, 2])), "a (Länge)");
    assert_eq!(diff("a", &json!({"k": 1}), &json!(3)), "a");
}

#[test]
fn baum_aus_struktur() {
    #[derive(serde::Serialize)]
    struct S {
        n: i64,
        x: f64,
        l: Vec<String>,
    }
    let got = tree(&S {
        n: 3,
        x: 0.5,
        l: vec![],
    });
    assert_eq!(diff("s", &json!({"n": 3.0, "x": 0.5, "l": null}), &got), "");
}

/// Teil von Gos TestGzRundreise, der liest: `.json.gz` liest sich wie dieselbe `.json`.
#[test]
fn gz_rundreise_lesen() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../.work/tmp")
        .join(format!("k3c-det-golden-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("Ordner");
    let text = "{\n  \"name\": \"x\",\n  \"ticks\": 4\n}\n";
    std::fs::write(dir.join("lauf.json"), text).expect("json schreiben");
    let mut zw = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    zw.write_all(text.as_bytes()).expect("packen");
    std::fs::write(dir.join("lauf.json.gz"), zw.finish().expect("packen")).expect("gz schreiben");
    let a = read(dir.join("lauf.json"));
    let b = read(dir.join("lauf.json.gz"));
    std::fs::remove_dir_all(&dir).expect("aufräumen");
    assert_eq!(diff("lauf", &a, &b), "");
    assert_eq!(a, json!({"name": "x", "ticks": 4}));
}

/// Erwartete Texte aus Go 1.27 (`fmt.Printf("%v", x)`).
#[test]
fn zahlen_wie_go() {
    let cases = [
        (0.1, "0.1"),
        (1e5, "100000"),
        (123456.0, "123456"),
        (1234567.0, "1.234567e+06"),
        (1e20, "1e+20"),
        (1.5e21, "1.5e+21"),
        (1e-4, "0.0001"),
        (1e-5, "1e-05"),
        (0.000123, "0.000123"),
        (-2.5, "-2.5"),
        (3.0, "3"),
        (100.0, "100"),
        (5e-324, "5e-324"),
        (1.7976931348623157e308, "1.7976931348623157e+308"),
    ];
    for (x, want) in cases {
        assert_eq!(go_float(x), want, "{x:e}");
    }
}

#[test]
fn diff_tol_laesst_kleine_fliesskomma_abweichungen_zu() {
    let want = serde_json::json!({"x": 100.0, "t": 1e-12, "n": 5});
    let near = serde_json::json!({"x": 100.005, "t": 3e-12, "n": 5});
    assert_eq!(diff_tol("w", &want, &near, SIM_TOL), "");
    assert_ne!(diff("w", &want, &near), "");
}

#[test]
fn diff_tol_meldet_abweichung_ausserhalb_mit_pfad() {
    let want = serde_json::json!({"a": [1.0, {"x": 100.0}]});
    let far = serde_json::json!({"a": [1.0, {"x": 100.02}]});
    assert_eq!(diff_tol("w", &want, &far, SIM_TOL), "w.a[1].x = 100.02, erwartet 100");
}

#[test]
fn diff_tol_haelt_ganzzahlen_und_laengen_exakt() {
    let want = serde_json::json!({"gold": 100, "ids": [1, 2]});
    let gold = serde_json::json!({"gold": 101, "ids": [1, 2]});
    assert_eq!(diff_tol("w", &want, &gold, SIM_TOL), "w.gold = 101, erwartet 100");
    let ids = serde_json::json!({"gold": 100, "ids": [1]});
    assert_eq!(diff_tol("w", &want, &ids, SIM_TOL), "w.ids (Länge)");
}
