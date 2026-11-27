//! Ersatz für `offline_test.go` (WZ28.7/AC-07, B-596/AC-05). Go ersetzt den globalen HTTP-Transport und zählt Anfragen;
//! Rust hat keinen solchen Punkt. Stattdessen: Das Crate hat in `[dependencies]` keinen HTTP-Client (reqwest nur für
//! Tests; die Web-Recherche der Asset-Tools liegt in `k3c-assetkit`), und Bericht lesen, bewerten, löschen läuft über
//! die Aufruf-Tabelle der Balancing-Seite durch. Den Offline-Lauf prüft `app::balance` mit Attrappe.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use k3c_studio::app::App;
use k3c_studio::app::rpc::call;
use serde_json::{Value, json};

#[path = "support/tmp.rs"]
mod tmp;

#[test]
fn ohne_http_client() {
    let manifest = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")).unwrap();
    let mut section = String::new();
    let mut found = Vec::new();
    let mut seen = 0;
    for line in manifest.lines().map(str::trim) {
        if line.starts_with('[') {
            line.clone_into(&mut section);
            continue;
        }
        // auch Abschnitte je Zielsystem, nicht aber dev- und build-dependencies
        let runtime = section == "[dependencies]" || section.ends_with(".dependencies]") && section.contains("target.");
        let Some((name, _)) = line.split_once('=').filter(|_| runtime && !line.starts_with('#')) else {
            continue;
        };
        seen += 1;
        let name = name.trim().split('.').next().unwrap_or_default();
        // hyper nur mit Client-Feature wäre ebenso einer; direkt gehört er hier gar nicht hin
        if [
            "reqwest",
            "ureq",
            "hyper",
            "hyper-util",
            "isahc",
            "surf",
            "attohttpc",
            "curl",
        ]
        .contains(&name)
        {
            found.push(name.to_owned());
        }
    }
    assert!(seen > 10, "nur {seen} Abhängigkeiten gelesen: Abschnitte stimmen nicht");
    assert!(found.is_empty(), "HTTP-Client in den Abhängigkeiten: {found:?}");
}

#[tokio::test(flavor = "multi_thread")]
async fn bericht_lesen_bewerten_loeschen() {
    let dir = tmp::tmp_dir();
    let root = std::path::absolute(dir.path().join("repo")).unwrap();
    let report = root.join("reports/simtest-run-1/balance-1.json");
    std::fs::create_dir_all(report.parent().unwrap()).unwrap();
    std::fs::write(&report, r#"{"version":1,"seeds":1,"pass":true}"#).unwrap();
    let app = App::new(root, 0, None).with_usage_file(dir.path().join("usage.json"));
    app.startup().await;

    let list = call(&app, "BalanceReports", &[]).unwrap();
    let path = list[0]["path"].clone();
    assert_eq!(path, json!("reports/simtest-run-1/balance-1.json"), "{list}");
    let read = call(&app, "BalanceReport", std::slice::from_ref(&path)).unwrap();
    assert_eq!(read["pass"], json!(true));
    assert_eq!(
        call(&app, "BalanceReportRate", &[path.clone(), json!("gut")]),
        Ok(Value::Null)
    );
    assert_eq!(call(&app, "BalanceReportDelete", &[path]), Ok(Value::Null));
    assert!(!report.exists());
    app.shutdown().await;
}
