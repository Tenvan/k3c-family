//! Port von `manage_test.go`, dazu die Härtungen beim Löschen. Alles in einem temporären Checkout unter `.work/`.

use std::fs;
use std::path::Path;

use super::manage::{delete_report, delete_reports, load_ratings, rate_report, rate_reports};
use super::reportlist::reports;
use super::test_helpers::{temp_root, write_report};

const PASS: &str = r#"{"seeds":1,"pass":true}"#;

fn rating(root: &Path, path: &str) -> String {
    let list = reports(root).expect("Liste");
    let found = list.into_iter().find(|r| r.path == path);
    found.unwrap_or_else(|| panic!("{path} fehlt in der Liste")).rating
}

fn texts(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

/// WZ28.13/AC-14 (B-612): Bewertung wird gespeichert und in der Liste geliefert; Löschen entfernt JSON, Markdown,
/// leeren Lauf-Ordner und Bewertung; fremde Pfade und Bewertungen werden abgelehnt.
#[test]
fn berichte_bewerten_und_loeschen() {
    let (_dir, root) = temp_root();
    let (a, b) = ("reports/simtest-run-1/balance-1.json", "reports/balance-2.json");
    write_report(&root, a, PASS);
    write_report(&root, "reports/simtest-run-1/balance-1.md", "# Bericht");
    write_report(&root, b, r#"{"seeds":2,"pass":false}"#);

    rate_report(&root, a, "gut").expect("bewerten");
    rate_report(&root, b, "schlecht").expect("bewerten");
    assert_eq!(
        (rating(&root, a), rating(&root, b)),
        ("gut".to_owned(), "schlecht".to_owned())
    );
    rate_report(&root, b, "").expect("Bewertung löschen");
    assert_eq!(rating(&root, b), "");
    // Format wie Go: beide Studios teilen die Datei
    let file = fs::read_to_string(root.join("reports/ratings.json")).expect("Datei");
    let want =
        "{\n  \"version\": 1,\n  \"ratings\": {\n    \"reports/simtest-run-1/balance-1.json\": \"gut\"\n  }\n}\n";
    assert_eq!(file, want);
    for (rel, value) in [
        (a, "toll"),
        ("reports/ratings.json", "gut"),
        ("../x/balance-1.json", "gut"),
        ("reports/balance-9.json", "gut"),
    ] {
        assert!(rate_report(&root, rel, value).is_err(), "{rel} {value}");
    }

    delete_report(&root, a).expect("löschen");
    assert!(!root.join("reports/simtest-run-1").exists(), "Lauf-Ordner bleibt");
    assert!(!load_ratings(&root).expect("Bewertungen").contains_key(a));
    for bad in [
        a,
        "reports/ratings.json",
        "data/economy.json",
        "reports/../data/balance-1.json",
        "reports",
        "",
    ] {
        assert!(delete_report(&root, bad).is_err(), "delete_report({bad})");
    }
    assert!(root.join(b).exists(), "anderer Bericht weg");
    // der letzte Bericht direkt unter reports/ lässt reports/ stehen
    delete_report(&root, r"reports\balance-2.json").expect("löschen");
    assert!(root.join("reports").is_dir());
}

/// Review WZ28.7: Eine kaputte `ratings.json` kostet nur die Bewertungen, nicht die Liste und nicht das Löschen.
#[test]
fn kaputte_bewertungen() {
    let (_dir, root) = temp_root();
    write_report(&root, "reports/balance-1.json", PASS);
    write_report(&root, "reports/ratings.json", "{kaputt");
    assert_eq!(reports(&root).expect("Liste").len(), 1);
    assert!(rate_report(&root, "reports/balance-1.json", "gut").is_err());
    delete_report(&root, "reports/balance-1.json").expect("löschen");
    let kept = fs::read_to_string(root.join("reports/ratings.json")).expect("Datei");
    assert_eq!(kept, "{kaputt", "nicht überschrieben");
}

/// B-662/AC-02: Sammel-Bewerten und Sammel-Löschen; ein fremder Pfad lehnt den ganzen Aufruf ab.
#[test]
fn sammel_bewerten_und_loeschen() {
    let (_dir, root) = temp_root();
    let a = "reports/simtest-run-1/balance-1.json";
    let b = "reports/simtest-run-2/balance-2.json";
    let c = "reports/tuning-3.json";
    for path in [a, b, c] {
        write_report(&root, path, PASS);
    }
    write_report(&root, "reports/simtest-run-1/balance-1.md", "# Bericht");
    rate_reports(&root, &texts(&[a, b, c]), "gut").expect("bewerten");
    for bad in [
        texts(&[a, "data/economy.json"]),
        texts(&[b, "../x/balance-1.json"]),
        texts(&[a, "reports/balance-9.json"]),
        Vec::new(),
    ] {
        assert!(rate_reports(&root, &bad, "schlecht").is_err(), "{bad:?}");
        assert!(delete_reports(&root, &bad).is_err(), "{bad:?}");
    }
    for path in [a, b, c] {
        assert_eq!(rating(&root, path), "gut", "{path} nach abgelehntem Aufruf");
    }
    delete_reports(&root, &texts(&[a, b, a])).expect("löschen");
    for gone in [
        a,
        "reports/simtest-run-1/balance-1.md",
        "reports/simtest-run-1",
        "reports/simtest-run-2",
    ] {
        assert!(!root.join(gone).exists(), "{gone} bleibt");
    }
    let ratings = load_ratings(&root).expect("Bewertungen");
    assert_eq!(ratings.len(), 1, "{ratings:?}");
    assert_eq!(ratings.get(c).map(String::as_str), Some("gut"));
}

/// Härtung: gesicherte Baselines lassen sich auch mit passendem Dateinamen nicht löschen.
#[test]
fn baselines_bleiben() {
    let (_dir, root) = temp_root();
    let ok = "reports/balance-1.json";
    let direct = "reports/baseline/balance-2.json";
    let saved = "reports/baseline/alt/balance-3.json";
    for path in [ok, direct, saved, "reports/baseline/alt/balance.json"] {
        write_report(&root, path, PASS);
    }
    let mut bad = vec![direct, saved, "reports/baseline/alt/balance.json"];
    if cfg!(windows) {
        // Windows meint damit denselben Ordner
        bad.extend([
            "reports/BASELINE/alt/balance-3.json",
            "reports/baseline./alt/balance-3.json",
        ]);
    }
    for path in bad {
        assert!(delete_reports(&root, &texts(&[ok, path])).is_err(), "{path} gelöscht");
    }
    for path in [ok, direct, saved] {
        assert!(root.join(path).exists(), "{path} weg");
    }
}

/// Härtung: ein Lauf-Ordner, der als Junction aus `reports/` hinausführt, wird nicht angefasst, und der ganze Aufruf
/// löscht nichts.
#[cfg(windows)]
#[test]
fn loeschen_folgt_keiner_junction() {
    let (_dir, root) = temp_root();
    let outside = root.parent().expect("Ordner").join("draussen");
    let ok = "reports/balance-1.json";
    write_report(&root, ok, PASS);
    write_report(&outside, "balance-2.json", PASS);
    let link = root.join("reports").join("raus");
    let mut cmd = std::process::Command::new("cmd");
    cmd.args(["/c", "mklink", "/J"]).arg(&link).arg(&outside);
    assert!(cmd.output().is_ok_and(|o| o.status.success()), "mklink /J");

    let through = "reports/raus/balance-2.json";
    assert!(delete_reports(&root, &texts(&[ok, through])).is_err());
    assert!(outside.join("balance-2.json").exists(), "Ziel gelöscht");
    assert!(root.join(ok).exists(), "trotz Ablehnung gelöscht");
    assert!(link.exists(), "Junction entfernt");
    fs::remove_dir(&link).expect("Junction lösen");
}
