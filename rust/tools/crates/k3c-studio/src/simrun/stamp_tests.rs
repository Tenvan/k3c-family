//! Port von `stamp_test.go`.

use super::*;
use crate::simrun::reportlist::{ReportInfo, reports};
use crate::simrun::run::{DONE, RUNNING, WAITING};
use crate::simrun::test_helpers::{temp_root, wait_for, write_report, writing_registry};

fn find_report(root: &Path, path: &str) -> ReportInfo {
    let list = reports(root).expect("Liste");
    let found = list.iter().find(|r| r.path == path);
    found
        .unwrap_or_else(|| panic!("{path} fehlt in der Liste: {list:?}"))
        .clone()
}

/// Label, Lauf-ID, Ziele und Auftrag eines Eintrags der Liste in einer Zeile.
fn brief(r: &ReportInfo) -> String {
    let spec = serde_json::to_string(&r.spec).expect("JSON");
    format!("{}|{}|{}/{}|{spec}", r.label, r.run_id, r.targets_pass, r.targets_total)
}

/// B-662/AC-01: Ein Lauf schreibt Label, Auftrag und Lauf-ID in den Bericht; die Liste liefert sie und x/y Ziele.
#[test]
fn bericht_traegt_auftrag() {
    let (_dir, root) = temp_root();
    let target = |name: &str, pass: bool| {
        format!(r#"{{"name":"{name}","kind":"share","value":80,"lower":75,"upper":90,"pass":{pass}}}"#)
    };
    let body = format!(
        r#"{{"version":1,"seeds":2,"pass":false,"targets":[{},{},{}],"neuesFeld":{{"x":[1,2]}}}}"#,
        target("A", true),
        target("B", false),
        target("C", true)
    );
    let g = writing_registry(&body);
    let spec = Spec {
        seeds: 2,
        bot: "coop2".to_owned(),
        days: 40,
        depth: Some(0),
        label: "Insel 1 solo · ohne Hilfen".to_owned(),
        ..Spec::default()
    };
    let info = g.start_offline(&root, "Repo-Wurzel", &spec).expect("start");
    let done = wait_for(&g, &root, &info.id, |i| {
        i.state != RUNNING && i.state != WAITING && !i.report.is_empty()
    });
    assert!(done.state == DONE && done.verdict == "Fail", "{done:?}");
    let rel = format!("reports/simtest-{}/balance-1.json", info.id);
    let raw = std::fs::read_to_string(root.join(&rel)).expect("Bericht");
    let head = format!(
        "{{\n  \"label\": \"Insel 1 solo · ohne Hilfen\",\n  \"runId\": \"{}\",\n  \"spec\": {{",
        info.id
    );
    let valid = serde_json::from_str::<serde_json::Value>(&raw).is_ok();
    assert!(
        valid && raw.starts_with(&head) && raw.contains("\"neuesFeld\": {"),
        "Bericht nicht vorn ergänzt oder Felder verloren:\n{raw}"
    );
    let want = serde_json::to_string(&spec).expect("JSON");
    assert_eq!(
        brief(&find_report(&root, &rel)),
        format!("{}|{}|2/3|{want}", spec.label, info.id)
    );
}

/// B-662/AC-01: Ein alter Bericht ohne Auftrag liefert leere Felder ohne Fehler, ein `tuning-*.json` 0/0 Ziele.
#[test]
fn alter_bericht_ohne_auftrag() {
    let (_dir, root) = temp_root();
    let old = "reports/simtest-run-9/balance-1.json";
    write_report(
        &root,
        old,
        r#"{"seeds":1,"pass":true,"targets":[{"name":"A","pass":true}]}"#,
    );
    write_report(
        &root,
        "reports/tuning-1.json",
        r#"{"version":1,"mode":"sweep","best":{"pass":true}}"#,
    );
    write_report(
        &root,
        "reports/balance-2.json",
        r#"{"seeds":1,"pass":true,"spec":"kaputt","label":"x"}"#,
    );
    for (path, want) in [
        (old, "||1/1|null"),
        ("reports/tuning-1.json", "||0/0|null"),
        ("reports/balance-2.json", "x||0/0|null"),
    ] {
        assert_eq!(brief(&find_report(&root, path)), want, "{path}");
    }
}

/// Stempeln: ein schon gestempelter Bericht bleibt, ein leeres Objekt bekommt die Felder, Kaputtes wird abgelehnt.
#[test]
fn stempel() {
    let (_dir, root) = temp_root();
    // der Wächter erlaubt Laufausgaben nur unter reports/ (WZ63.7)
    let write = |name: &str, body: &str| {
        let rel = format!("reports/{name}");
        write_report(&root, &rel, body);
        root.join("reports").join(name)
    };
    let st = Stamp {
        label: "x".to_owned(),
        run_id: "run-1".to_owned(),
        spec: Some(Spec {
            seeds: 1,
            ..Spec::default()
        }),
    };
    let empty = write("leer.json", "{}");
    stamp(&root, &empty, &st).expect("stempeln");
    let raw = std::fs::read(&empty).expect("lesen");
    let back = StampRaw::read(&raw).expect("Stempel");
    assert_eq!(back.run_id, "run-1");
    assert_eq!(back.spec().expect("Auftrag").seeds, 1);
    let other = Stamp {
        label: "y".to_owned(),
        run_id: "run-2".to_owned(),
        spec: None,
    };
    stamp(&root, &empty, &other).expect("zweiter Stempel");
    assert_eq!(std::fs::read(&empty).expect("lesen"), raw);
    for bad in ["[1]", "kaputt", ""] {
        let file = write("bad.json", bad);
        assert!(stamp(&root, &file, &st).is_err(), "{bad:?} gestempelt");
    }
    // außerhalb von reports/ und data/worlds schreibt der Wächter nicht
    write_report(&root, "data/x.json", "{}");
    assert!(stamp(&root, &root.join("data/x.json"), &st).is_err());
}

/// Wie Go `json.Marshal`: HTML-Zeichen im Label sind maskiert, Zahlen des Berichts bleiben in ihrer Schreibweise.
#[test]
fn stempel_maskiert_wie_go_und_behaelt_zahlen() {
    let (_dir, root) = temp_root();
    write_report(&root, "reports/balance-1.json", "\n{ \"v\": 1.50e1, \"leer\": {} }\n");
    let file = root.join("reports").join("balance-1.json");
    let st = Stamp {
        label: "<a&b>".to_owned(),
        ..Stamp::default()
    };
    stamp(&root, &file, &st).expect("stempeln");
    let want = "{\n  \"label\": \"\\u003ca\\u0026b\\u003e\",\n  \"v\": 1.50e1,\n  \"leer\": {}\n}\n";
    assert_eq!(std::fs::read_to_string(&file).expect("lesen"), want);
}
