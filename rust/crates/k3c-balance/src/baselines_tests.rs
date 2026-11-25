//! Tests zu `baselines.rs` (Port von `engine/balance/baselines_test.go`, B-640/AC-02). Go liest den gesicherten Bericht mit
//! `ReadBaseline` (RU31); hier mit `read_summary`, dem Kern davon.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use k3c_det::golden;
use serde_json::json;

use crate::baselines::{
    BASELINE_DIR, baseline_file, has_baseline, list_baselines, rfc3339_utc, save_baseline, valid_baseline_name,
};
use crate::summary::read_summary;

/// Ein frischer Ordner unter dem Temp-Ordner (Go: `t.TempDir`); der Aufrufer räumt ihn nicht auf, das tut `Dir::drop`.
struct Dir(PathBuf);

impl Dir {
    fn new() -> Dir {
        static N: AtomicU32 = AtomicU32::new(0);
        let p = std::env::temp_dir().join(format!(
            "k3c-baselines-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).expect("Ordner");
        Dir(p)
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn write_report(root: &Path, run: &str, raw: &str) -> PathBuf {
    let dir = root.join("reports").join(run);
    std::fs::create_dir_all(&dir).expect("Ordner");
    let path = dir.join("balance-20261028-100000.json");
    std::fs::write(&path, raw).expect("Bericht");
    path
}

const PASS_REPORT: &str = concat!(
    r#"{"version":1,"seeds":1,"seedMode":"dev","pass":true,"targets":["#,
    r#"{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"world":"standard","value":100,"lower":100,"upper":100,"status":"ok","pass":true}]}"#
);

fn failing() -> String {
    PASS_REPORT.replacen(r#""pass":true,"targets""#, r#""pass":false,"targets""#, 1)
}

fn err_text<T>(r: Result<T, crate::Fehler>) -> String {
    r.err().map_or_else(|| "kein Fehler".to_string(), |e| e.to_string())
}

#[test]
fn baseline_sichern_auflisten_und_lesen() {
    let d = Dir::new();
    let path = write_report(&d.0, "simtest-run-3", PASS_REPORT);
    let spec = json!({"seeds": 1, "bot": "coop2", "world": "standard", "days": 40});
    let info = save_baseline(&d.0, "M1", &path, Some(spec), false).expect("sichern");
    assert_eq!(
        (
            info.name.as_str(),
            info.run.as_str(),
            info.seeds,
            info.seed_mode.as_str(),
            &info.bots,
            &info.worlds
        ),
        (
            "M1",
            "simtest-run-3",
            1,
            "dev",
            &vec!["coop2".to_string()],
            &vec!["standard".to_string()]
        ),
        "Konfiguration"
    );
    assert!(has_baseline(&d.0, "M1") && !has_baseline(&d.0, "M2"), "has_baseline");
    let list = list_baselines(&d.0).expect("Liste");
    assert!(list.len() == 1 && list[0].name == "M1", "Liste: {list:?}");
    assert_eq!(
        list[0].spec.as_ref().expect("Spec")["world"],
        "standard",
        "Spec des Laufs fehlt in der Liste"
    );
    let raw = std::fs::read_to_string(d.0.join(baseline_file("M1"))).expect("balance.json");
    let base = read_summary(&raw).expect("Baseline");
    assert_eq!(
        base.targets.len(),
        1,
        "gesicherter Bericht lässt sich als Baseline lesen"
    );
}

#[test]
fn baseline_lehnt_ab() {
    let d = Dir::new();
    let pass = write_report(&d.0, "simtest-run-1", PASS_REPORT);
    let fail = write_report(&d.0, "simtest-run-2", &failing());
    save_baseline(&d.0, "M1", &pass, None, false).expect("sichern");
    let missing = d.0.join("fehlt.json");
    for (name, path, want) in [
        ("M1", &pass, "gibt es schon"),
        ("M2", &fail, "nicht erreicht"),
        ("../x", &pass, "Buchstaben"),
        ("", &pass, "Buchstaben"),
        ("M3", &missing, ""),
    ] {
        let e = err_text(save_baseline(&d.0, name, path, None, false));
        assert!(
            e != "kein Fehler" && e.contains(want),
            "{name:?}: Fehler {e:?}, erwartet {want:?}"
        );
    }
    assert_eq!(
        list_baselines(&d.0).expect("Liste").len(),
        1,
        "abgelehnte Namen legen nichts an"
    );
}

#[test]
fn list_baselines_ohne_ordner_leer() {
    let d = Dir::new();
    assert_eq!(list_baselines(&d.0).expect("Liste"), Vec::new());
}

/// Offener Meilenstein: ein gültiger Lauf ohne Pass wird nur mit `open` gesichert.
#[test]
fn baseline_offener_meilenstein() {
    let d = Dir::new();
    let fail = write_report(&d.0, "simtest-run-2", &failing());
    assert!(
        save_baseline(&d.0, "M1", &fail, None, false).is_err(),
        "ohne open gesichert"
    );
    let info = save_baseline(&d.0, "M1", &fail, None, true).expect("mit open");
    assert!(!info.pass && has_baseline(&d.0, "M1"), "{info:?}");
}

/// Ein Bericht ohne gültige Messung (Lauf abgestürzt) wird auch mit `open` nicht gesichert; parallele Aufrufe legen einen Namen
/// nur einmal an.
#[test]
fn baseline_ungueltig_und_parallel() {
    let d = Dir::new();
    let bad = failing().replacen(r#""status":"ok""#, r#""status":"ungültig""#, 1);
    let bad = write_report(&d.0, "simtest-run-3", &bad);
    let e = err_text(save_baseline(&d.0, "M1", &bad, None, true));
    assert!(e.contains("gültige Messung"), "ungültiger Bericht: {e}");
    let ok = write_report(&d.0, "simtest-run-4", PASS_REPORT);
    let wins = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(|| {
                if save_baseline(&d.0, "P1", &ok, None, false).is_ok() {
                    wins.fetch_add(1, Ordering::SeqCst);
                }
            });
        }
    });
    assert_eq!(wins.load(Ordering::SeqCst), 1, "Aufrufe, die P1 angelegt haben");
}

#[test]
fn baseline_namen() {
    for ok in ["M1", "a_b-c", &"x".repeat(40)] {
        assert!(valid_baseline_name(ok).is_ok(), "{ok}");
    }
    for bad in ["", "../x", "a b", "ä", &"x".repeat(41)] {
        assert!(valid_baseline_name(bad).is_err(), "{bad:?}");
    }
    assert_eq!(baseline_file("M1"), format!("{BASELINE_DIR}/M1/balance.json"));
}

#[test]
fn zeit_als_rfc3339_utc() {
    let at = |s| rfc3339_utc(UNIX_EPOCH + Duration::from_secs(s));
    assert_eq!(at(0), "1970-01-01T00:00:00Z");
    assert_eq!(at(1_794_909_600), "2026-11-17T10:00:00Z");
    assert_eq!(at(951_782_399), "2000-02-28T23:59:59Z");
    assert_eq!(at(951_782_400), "2000-02-29T00:00:00Z");
    assert_eq!(at(951_868_800), "2000-03-01T00:00:00Z");
}

/// `testdata/balance/baseline.json` liest Rust in dieselben Werte wie Go: Der JSON-Baum der gelesenen Zusammenfassung gleicht
/// dem der Datei (Zahlen als `12.0` oder `12` zählen gleich).
#[test]
fn baseline_datei_im_repo() {
    let path = golden::testdata().join("balance/baseline.json");
    let raw = std::fs::read_to_string(&path).expect("baseline.json");
    let s = read_summary(&raw).expect("Baseline");
    assert!(
        s.seeds == 100 && s.targets.len() == 3,
        "{} Seeds, {} Ziele",
        s.seeds,
        s.targets.len()
    );
    let at = golden::diff("", &golden::read(&path), &golden::tree(&s));
    assert!(at.is_empty(), "baseline.json: Abweichung bei {at}");
}
