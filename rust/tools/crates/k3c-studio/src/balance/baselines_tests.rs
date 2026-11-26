//! Go `baselines_test.go` (B-640/AC-02): Baselines sichern, auflisten, nie überschreiben, nur durchgelaufene Läufe.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::value::RawValue;

use super::super::summary::read_summary;
use super::super::test_helpers::temp_root;
use super::{BASELINE_DIR, BaselineInfo, baseline_file, has_baseline, list_baselines, save_baseline};

const PASS_REPORT: &str = concat!(
    r#"{"version":1,"seeds":1,"seedMode":"dev","pass":true,"targets":["#,
    r#"{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"#,
    r#""world":"standard","value":100,"lower":100,"upper":100,"status":"ok","pass":true}]}"#
);

fn write_report(root: &Path, run: &str, raw: &str) -> PathBuf {
    let dir = root.join("reports").join(run);
    fs::create_dir_all(&dir).expect("Ordner");
    let path = dir.join("balance-20261028-100000.json");
    fs::write(&path, raw).expect("Bericht");
    path
}

fn failed_report() -> String {
    PASS_REPORT.replacen(r#""pass":true,"targets""#, r#""pass":false,"targets""#, 1)
}

fn raw(text: &str) -> Option<Box<RawValue>> {
    Some(RawValue::from_string(text.to_owned()).expect("JSON"))
}

fn names(list: &[BaselineInfo]) -> Vec<&str> {
    list.iter().map(|i| i.name.as_str()).collect()
}

#[test]
fn sichern_auflisten_und_lesen() {
    let (_dir, root) = temp_root();
    let path = write_report(&root, "simtest-run-3", PASS_REPORT);
    let spec = raw(r#"{"seeds":1,"bot":"coop2","world":"standard","days":40}"#);
    let info = save_baseline(&root, "M1", &path, spec, false).expect("sichern");
    assert_eq!(
        (
            info.name.as_str(),
            info.run.as_str(),
            info.seeds,
            info.seed_mode.as_str()
        ),
        ("M1", "simtest-run-3", 1, "dev")
    );
    assert_eq!(
        (info.bots, info.worlds),
        (vec!["coop2".to_owned()], vec!["standard".to_owned()])
    );
    assert!(has_baseline(&root, "M1") && !has_baseline(&root, "M2"), "has_baseline");
    let list = list_baselines(&root).expect("Liste");
    assert_eq!(names(&list), ["M1"]);
    let spec = list[0].spec.as_ref().expect("Spec fehlt").get();
    let compact: String = spec.split_whitespace().collect();
    assert!(
        compact.contains(r#""world":"standard""#),
        "Spec des Laufs fehlt in der Liste: {spec}"
    );
    let saved = fs::read(root.join(baseline_file("M1"))).expect("Bericht der Baseline");
    assert_eq!(saved, PASS_REPORT.as_bytes());
    assert_eq!(read_summary(&saved).expect("als Baseline lesbar").targets.len(), 1);
}

/// `run.json` wie Go `json.MarshalIndent`: der rohe Auftrag wird mit eingerückt und `<`, `>`, `&` maskiert.
#[test]
fn run_json_wie_go() {
    let (_dir, root) = temp_root();
    let path = write_report(&root, "simtest-run-3", PASS_REPORT);
    let spec = raw("{ \"bot\": \"a<b\",\n      \"seeds\":1.50, \"list\": [ ] , \"l\":[1,2]}");
    let info = save_baseline(&root, "M1", &path, spec, false).expect("sichern");
    let written = fs::read_to_string(root.join(BASELINE_DIR).join("M1").join("run.json")).expect("run.json");
    let want = r#"{
  "name": "M1",
  "saved": "SAVED",
  "run": "simtest-run-3",
  "seeds": 1,
  "seedMode": "dev",
  "bots": [
    "coop2"
  ],
  "worlds": [
    "standard"
  ],
  "pass": true,
  "frozen": [],
  "spec": {
    "bot": "aLTb",
    "seeds": 1.50,
    "list": [],
    "l": [
      1,
      2
    ]
  }
}"#;
    // So maskiert Go das `<` (Backslash, u003c).
    let want = want.replace("LT", &format!("{}u003c", '\\'));
    assert_eq!(written, want.replace("SAVED", &info.saved));
    assert!(
        chrono::DateTime::parse_from_rfc3339(&info.saved).is_ok(),
        "{}",
        info.saved
    );
    let entries = fs::read_dir(root.join(BASELINE_DIR).join("M1"))
        .expect("Ordner")
        .count();
    assert_eq!(entries, 2, "keine Temp-Reste");
}

/// Ohne Auftrag fehlt `spec`; ein `run.json` mit `null`-Feldern (Go nil) bleibt lesbar, fremde Ordner fehlen.
#[test]
fn liste_liest_wie_go() {
    let (_dir, root) = temp_root();
    let base = root.join(BASELINE_DIR);
    let put = |dir: &str, text: &str| {
        fs::create_dir_all(base.join(dir)).expect("Ordner");
        fs::write(base.join(dir).join("run.json"), text).expect("run.json");
    };
    put("b", r#"{"name":"b","bots":null,"frozen":null,"seeds":null}"#);
    put("a", r#"{"name":"a","frozen":["holz"],"neu":1}"#);
    put("falsch", r#"{"name":"anders"}"#);
    put("kaputt", "{");
    fs::create_dir_all(base.join("leer")).expect("Ordner ohne run.json");
    let list = list_baselines(&root).expect("Liste");
    assert_eq!(names(&list), ["a", "b"]);
    assert_eq!(list[1].frozen, None);
    let json = list[1].json().expect("JSON");
    assert!(json.contains(r#""frozen": null"#) && !json.contains("spec"), "{json}");
}

#[test]
fn lehnt_ab() {
    let (_dir, root) = temp_root();
    let pass = write_report(&root, "simtest-run-1", PASS_REPORT);
    let fail = write_report(&root, "simtest-run-2", &failed_report());
    save_baseline(&root, "M1", &pass, None, false).expect("sichern");
    let saved = root.join(BASELINE_DIR).join("M1").join("run.json");
    let before = fs::read(&saved).expect("run.json");
    let long = "x".repeat(41);
    let missing = root.join("fehlt.json");
    for (name, path, want) in [
        ("M1", &pass, "gibt es schon"),
        ("M2", &fail, "nicht erreicht"),
        ("../x", &pass, "Buchstaben"),
        ("", &pass, "Buchstaben"),
        ("a/b", &pass, "Buchstaben"),
        (r"a\b", &pass, "Buchstaben"),
        ("..", &pass, "Buchstaben"),
        ("C:", &pass, "Buchstaben"),
        ("ä", &pass, "Buchstaben"),
        (long.as_str(), &pass, "Buchstaben"),
        ("M3", &missing, ""),
    ] {
        let err = save_baseline(&root, name, path, None, false).expect_err(name);
        assert!(err.contains(want), "{name:?}: Fehler {err}, erwartet {want}");
    }
    assert_eq!(fs::read(&saved).expect("run.json"), before, "M1 überschrieben");
    assert_eq!(
        names(&list_baselines(&root).expect("Liste")),
        ["M1"],
        "abgelehnte Namen legen nichts an"
    );
    assert_eq!(fs::read_dir(root.join(BASELINE_DIR)).expect("Basis").count(), 1);
}

#[test]
fn liste_ohne_ordner_leer() {
    let (_dir, root) = temp_root();
    assert!(list_baselines(&root).expect("Liste").is_empty());
}

/// Offener Meilenstein: ein gültiger Lauf ohne Pass wird nur mit `open` gesichert.
#[test]
fn offener_meilenstein() {
    let (_dir, root) = temp_root();
    let fail = write_report(&root, "simtest-run-2", &failed_report());
    assert!(
        save_baseline(&root, "M1", &fail, None, false).is_err(),
        "ohne open gesichert"
    );
    let info = save_baseline(&root, "M1", &fail, None, true).expect("mit open");
    assert!(!info.pass && has_baseline(&root, "M1"));
}

/// Ein Bericht ohne gültige Messung (Lauf abgestürzt) wird auch mit `open` nicht gesichert; parallele Aufrufe legen
/// einen Namen nur einmal an.
#[test]
fn ungueltig_und_parallel() {
    let (_dir, root) = temp_root();
    let invalid = failed_report().replacen(r#""status":"ok""#, r#""status":"ungültig""#, 1);
    let bad = write_report(&root, "simtest-run-3", &invalid);
    let err = save_baseline(&root, "M1", &bad, None, true).expect_err("ungültiger Bericht");
    assert!(err.contains("gültige Messung"), "{err}");
    let ok = write_report(&root, "simtest-run-4", PASS_REPORT);
    let wins = std::thread::scope(|scope| {
        let runs: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| save_baseline(&root, "P1", &ok, None, false).is_ok()))
            .collect();
        runs.into_iter()
            .filter_map(|r| r.join().ok())
            .filter(|won| *won)
            .count()
    });
    assert_eq!(wins, 1, "Aufrufe, die P1 angelegt haben");
}

/// Eine Junction auf dem Weg (`reports/baseline`) führt nicht aus `reports/` heraus.
#[cfg(windows)]
#[test]
fn junction_auf_dem_weg_abgelehnt() {
    use super::super::test_helpers::junction;

    let (_dir, root) = temp_root();
    let pass = write_report(&root, "simtest-run-1", PASS_REPORT);
    let outside = root.parent().expect("Eltern").join("draussen");
    fs::create_dir_all(&outside).expect("draußen");
    assert!(junction(&root.join("reports").join("baseline"), &outside), "mklink /J");
    let err = save_baseline(&root, "M1", &pass, None, false).expect_err("über die Junction");
    assert!(err.contains("Balancing darf"), "{err}");
    assert_eq!(fs::read_dir(&outside).expect("draußen").count(), 0);
}
