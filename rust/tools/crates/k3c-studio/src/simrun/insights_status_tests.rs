//! Port von `insights_status_test.go` (B-632/AC-05): `sim_test status` nennt die Kurzauswertung und den Pfad des
//! HTML-Berichts vor den Zeilen je Ziel. Dazu das Auswerten und Neuschreiben eines Berichts (`reports.go`).

use std::fs;
use std::time::{Duration, SystemTime};

use chrono::Local;

use super::info::Info;
use super::reports::{read_insights, read_result, write_html};
use super::test_helpers::{repo_root, temp_root, write_report};

fn fixture() -> String {
    let path = repo_root().join("tools/k3c-studio/internal/balance/testdata/insights-run7.json");
    fs::read_to_string(path).expect("Bericht des Go-Pakets")
}

#[test]
fn status_mit_kurzauswertung() {
    let (_dir, root) = temp_root();
    write_report(&root, "reports/simtest-run-3/balance-20261025-062117.json", &fixture());
    write_report(
        &root,
        "reports/simtest-run-3/balance-20261025-062117.html",
        "<html></html>",
    );
    let dir = root.join("reports").join("simtest-run-3");
    let since = SystemTime::now() - Duration::from_secs(60);
    // Hinweise und Ziele kommen aus data/ des Repos (nur gelesen)
    let (verdict, lines) = read_result(&repo_root(), &dir, since, false).expect("Ergebnis");
    assert_eq!(verdict, "Fail");
    assert!(
        lines.len() >= 5 && lines[0].starts_with("4 von 13 Zielen verletzt"),
        "Kurzauswertung fehlt: {lines:?}"
    );
    let html = "HTML-Bericht: reports/simtest-run-3/balance-20261025-062117.html";
    assert!(lines.iter().any(|l| l == html), "{lines:?}");
    let info = Info {
        id: "run-3".to_owned(),
        label: String::new(),
        place: String::new(),
        state: String::new(),
        phase: String::new(),
        done: 0,
        total: 0,
        started: Local::now(),
        ended: None,
        verdict: String::new(),
        report: "reports/simtest-run-3".to_owned(),
        lines,
        spec: None,
    };
    let status = info.status(Local::now(), 10);
    assert!(
        status.matches('\n').count() <= 9 && status.contains("HTML-Bericht:"),
        "status:\n{status}"
    );
}

#[test]
fn bericht_auswerten_und_neu_schreiben() {
    let (_dir, root) = temp_root();
    // Hinweise und Ziele der Auswertung liegen unter data/ des Checkouts: Kopien im temporären Checkout
    for name in ["balance-hints.json", "balance-targets.json"] {
        let text = fs::read_to_string(repo_root().join("data").join(name)).expect("Daten");
        write_report(&root, &format!("data/{name}"), &text);
    }
    let rel = "reports/simtest-run-3/balance-20261025-062117.json";
    write_report(&root, rel, &fixture());
    let insights = read_insights(&root, rel).expect("Auswertung");
    assert!(
        insights
            .summary
            .first()
            .is_some_and(|s| s.starts_with("4 von 13 Zielen verletzt")),
        "{:?}",
        insights.summary
    );
    let html = write_html(&root, rel).expect("HTML");
    assert!(
        html.is_absolute() && html.ends_with("balance-20261025-062117.html"),
        "{html:?}"
    );
    assert!(html.is_file() && html.with_extension("md").is_file());
    write_report(&root, "reports/tuning-1.json", "{}");
    for bad in [
        "reports/tuning-1.json",
        "reports/balance-9.json",
        "data/balance-hints.json",
    ] {
        assert!(read_insights(&root, bad).is_err(), "{bad} ausgewertet");
        assert!(write_html(&root, bad).is_err(), "{bad} geschrieben");
    }
}
