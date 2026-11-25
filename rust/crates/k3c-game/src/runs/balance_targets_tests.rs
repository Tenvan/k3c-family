//! Tests der Ziel- und Abstimmungs-Läufe von `sim balance` (RU40.1): Port von `TestVerletztesZielIstKeinFehler`,
//! `TestWerkzeugFehlerIstFehler`, `TestFortschrittVarianteImBericht` und `TestTuneSchreibtErgebnis` aus
//! `engine/simcli/balance_test.go`. Abweichung: Die langen Go-Tests rechnen ohne `--days` fünf Tage je Lauf; hier steht
//! `--days 1` dabei (Sim-Sperre: nur kurze Läufe mit festen Eingaben), geprüft wird dasselbe (Exit-Code, Dateien).

use serde_json::Value;

use super::balance_tests::require_json_lines;
use super::testutil::{WorkDir, call};
use super::*;

/// Verletzte Ziele sind kein Fehler (Exit 0, BAL2.3): Der Bericht entsteht (.json, .md und .html).
#[test]
fn verletztes_ziel_ist_kein_fehler() {
    let dir = WorkDir::new("ziele");
    let (code, out, err) = call(
        "",
        &[
            "balance",
            "--targets",
            "--seeds",
            "1",
            "--days",
            "1",
            "--report-dir",
            &dir.path(),
        ],
    );
    assert_eq!(code, EXIT_OK, "{err}");
    assert_eq!(dir.files("balance-", ".md").len(), 1, "erwartet einen Markdown-Bericht");
    assert_eq!(dir.files("balance-", ".html").len(), 1);
    let lines = require_json_lines(&out);
    assert_eq!(lines.last().and_then(|l| l["event"].as_str()), Some("report"));
    assert_eq!(lines.last().and_then(|l| l["files"].as_array()).map(Vec::len), Some(3));
}

/// Ein Fehler des Werkzeugs selbst ist Exit 1, ein falscher Aufruf Exit 2.
#[test]
fn werkzeug_fehler_ist_fehler() {
    let (code, _, _) = call("", &["balance", "--targets", "--seeds", "1", "--write-baseline"]);
    assert_eq!(code, EXIT_USAGE, "--write-baseline ohne --baseline");
    let dir = WorkDir::new("datei");
    let file = format!("{}/datei", dir.path());
    std::fs::write(&file, "").expect("Datei");
    let (code, _, _) = call(
        "",
        &[
            "balance",
            "--targets",
            "--seeds",
            "1",
            "--days",
            "1",
            "--report-dir",
            &file,
        ],
    );
    assert_eq!(code, EXIT_FAILED, "Berichtsordner, der eine Datei ist");
}

/// WZ28.4: `--progress` schreibt Zeilen `progress`, `--vary` und die Kennzahlen stehen im Bericht, die Berichtsdateien meldet
/// eine Zeile `report`; Fehlbedienung ist ein Fehler.
#[test]
fn fortschritt_variante_im_bericht() {
    let dir = WorkDir::new("variante");
    let report_dir = dir.path();
    let (code, out, err) = call(
        "",
        &[
            "balance",
            "--targets",
            "--seeds",
            "1",
            "--days",
            "1",
            "--grade",
            "normal",
            "--progress",
            "--vary",
            "economy.json:purse.startGold=120",
            "--report-dir",
            &report_dir,
        ],
    );
    assert_eq!(code, EXIT_OK, "{err}");
    assert!(
        out.contains("{\"event\":\"progress\",\"done\":1,\"total\":1}\n"),
        "Fortschritt fehlt: {out:?}"
    );
    require_json_lines(&out);
    let json = dir.files("balance-", ".json");
    assert!(
        json.len() == 1 && out.contains("\"event\":\"report\""),
        "Bericht: {json:?}, stdout {out:?}"
    );
    let s: Value =
        serde_json::from_str(&std::fs::read_to_string(format!("{report_dir}/{}", json[0])).expect("Bericht"))
            .expect("JSON");
    assert_eq!(s["varianten"].as_array().map(Vec::len), Some(1));
    assert_eq!(s["kennzahlen"]["nights"].as_array().map(Vec::len), Some(1));
    for a in [
        vec![
            "balance",
            "--targets",
            "--seeds",
            "1",
            "--vary",
            "economy.json:purse.gibtsnicht=1",
            "--report-dir",
            &report_dir,
        ],
        vec!["balance", "--vary", "economy.json:purse.startGold=1"],
        vec!["balance", "--targets", "--players", "1,2", "--report-dir", &report_dir],
    ] {
        let (code, _, _) = call("", &a);
        assert_ne!(code, EXIT_OK, "{a:?}: Fehler erwartet");
    }
}

/// WZ28.6: `--tune sweep` schreibt `tuning-<Zeit>.json` mit Punkten, Begründung und Fortschritt; Fehlbedienung ist ein Fehler.
#[test]
fn tune_schreibt_ergebnis() {
    let dir = WorkDir::new("tune");
    let report_dir = dir.path();
    let param = "economy.json:purse.startGold=8..20";
    let (code, out, err) = call(
        "",
        &[
            "balance",
            "--targets",
            "--seeds",
            "1",
            "--days",
            "1",
            "--grade",
            "normal",
            "--progress",
            "--tune",
            "sweep",
            "--steps",
            "2",
            "--param",
            param,
            "--report-dir",
            &report_dir,
        ],
    );
    assert_eq!(code, EXIT_OK, "{err}");
    assert!(
        out.contains("{\"event\":\"progress\",\"done\":2,\"total\":2}\n"),
        "Fortschritt: {out:?}"
    );
    let files = dir.files("tuning-", ".json");
    assert_eq!(files.len(), 1, "ergebnis: {files:?}");
    let tn: Value =
        serde_json::from_str(&std::fs::read_to_string(format!("{report_dir}/{}", files[0])).expect("Ergebnis"))
            .expect("JSON");
    assert_eq!(tn["mode"], "sweep");
    assert_eq!(tn["points"].as_array().map(Vec::len), Some(2));
    assert!(tn["reason"].as_str().is_some_and(|r| !r.is_empty()));
    assert_eq!(tn["params"][0]["base"], 14.0);
}

/// WZ28.6: Fehlbedienung von `--tune` und `--param` ist ein Fehler (kein Lauf, kein Bericht).
#[test]
fn tune_fehlbedienung() {
    let dir = WorkDir::new("tune-fehler");
    let report_dir = dir.path();
    for a in [
        vec![
            "balance",
            "--tune",
            "sweep",
            "--param",
            "economy.json:purse.startGold=1..2",
        ],
        vec![
            "balance",
            "--targets",
            "--param",
            "economy.json:purse.startGold=1..2",
            "--report-dir",
            &report_dir,
        ],
        vec![
            "balance",
            "--targets",
            "--tune",
            "raten",
            "--param",
            "economy.json:purse.startGold=1..2",
            "--report-dir",
            &report_dir,
        ],
        vec!["balance", "--targets", "--tune", "sweep", "--report-dir", &report_dir],
    ] {
        let (code, _, _) = call("", &a);
        assert_ne!(code, EXIT_OK, "{a:?}: Fehler erwartet");
    }
}
