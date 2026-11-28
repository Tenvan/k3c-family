//! `TestSetFindingsSchreibtUndBehaeltHaken` und `TestSetFindingsFehler` aus `findings_test.go`.

use super::*;
use crate::playtest::eval_rig_tests::{EvalRig, SESSION};
use crate::playtest::rig_tests::put;

fn proposal(id: &str, quote: &str) -> FindingIn {
    FindingIn {
        id: id.to_owned(),
        quote: quote.to_owned(),
        ..FindingIn::default()
    }
}

fn tick() -> FindingPatch {
    FindingPatch {
        selected: Some(true),
        ..FindingPatch::default()
    }
}

#[test]
fn set_findings_schreibt_und_behaelt_haken() {
    let rig = EvalRig::new(true);
    let store = rig.store();
    let mut first = proposal("f1", "Ich finde den Bauarbeiter nicht");
    (first.ts, first.kind, first.project) = (1_005_000, "Problem".to_owned(), "WZG".to_owned());
    let mut second = proposal("f2", "Zu dunkel");
    (second.ts, second.duplicate_of) = (1_020_500, "B-1100".to_owned());
    let mut input = vec![first, second];
    assert_eq!(store.set_findings(SESSION, &input).unwrap(), (2, 0));
    store.update_finding(SESSION, "f1", &tick()).unwrap();
    "Bauarbeiter fehlt".clone_into(&mut input[0].quote);
    assert_eq!(store.set_findings(SESSION, &input[..1]).unwrap(), (0, 1));
    let list = store.findings(SESSION).unwrap();
    assert_eq!(list.len(), 2);
    assert!(
        list[0].quote == "Bauarbeiter fehlt" && list[0].selected,
        "Haken muss bleiben: {list:?}"
    );
    assert_eq!(store.get(SESSION).unwrap().stage, "ausgewertet");
}

#[test]
fn set_findings_fehler() {
    let rig = EvalRig::new(true);
    let store = rig.store();
    let with = |change: fn(&mut FindingIn)| {
        let mut f = proposal("a", "x");
        change(&mut f);
        vec![f]
    };
    let cases = [
        ("id", vec![proposal("a b", "x")]),
        ("quote", vec![proposal("a", "")]),
        ("type", with(|f| "Bug".clone_into(&mut f.kind))),
        ("prio", with(|f| "dringend".clone_into(&mut f.prio))),
        ("duplicate", with(|f| "Ticket 5".clone_into(&mut f.duplicate_of))),
        ("frame", with(|f| "../x.jpg".clone_into(&mut f.frame))),
        ("doppelt", vec![proposal("a", "x"), proposal("a", "y")]),
        ("leer", Vec::new()),
    ];
    for (name, input) in cases {
        let error = store.set_findings(SESSION, &input).unwrap_err();
        assert!(matches!(error, Error::BadFinding(_)), "{name}: {error:?}");
    }
    let unknown = store.set_findings("2026-01-01-9", &[proposal("a", "x")]);
    assert_eq!(unknown.unwrap_err(), Error::NotFound);
    let mixed = [proposal("ok", "x"), proposal("b", "")];
    let error = store.set_findings(SESSION, &mixed).unwrap_err().to_string();
    assert_eq!(error, "ungültiger Befund b: quote fehlt");
    assert!(
        store.findings(SESSION).unwrap().is_empty(),
        "nichts darf geschrieben sein"
    );
}

#[test]
fn datei_wie_go_und_aenderung_von_der_seite() {
    let rig = EvalRig::new(true);
    let store = rig.store();
    let mut input = proposal("f1", "a<b");
    "Idee".clone_into(&mut input.kind);
    store.set_findings(SESSION, &[input]).unwrap();
    assert_eq!(
        rig.text("findings.json"),
        "{\n  \"findings\": [\n    {\n      \"id\": \"f1\",\n      \"quote\": \"a\\u003cb\",\n      \"ts\": 0,\n      \
         \"type\": \"Idee\",\n      \"selected\": false\n    }\n  ]\n}"
    );
    let patch: FindingPatch = serde_json::from_str(r#"{"type":"Problem","sprint":" WZ1 ","selected":true}"#).unwrap();
    let changed = store.update_finding(SESSION, "f1", &patch).unwrap();
    assert_eq!((changed.kind.as_str(), changed.sprint.as_str()), ("Problem", "WZ1"));
    let bad: FindingPatch = serde_json::from_str(r#"{"prio":"egal"}"#).unwrap();
    let error = store.update_finding(SESSION, "f1", &bad).unwrap_err().to_string();
    assert_eq!(error, "ungültiger Befund f1: Typ oder Prio ungültig");
    let missing = store.update_finding(SESSION, "f9", &tick()).unwrap_err().to_string();
    assert_eq!(missing, "Session oder Datei nicht gefunden: Befund f9 gibt es nicht");
    put(&rig.dir, "findings.json", r#"{"findings":null}"#);
    assert!(store.findings(SESSION).unwrap().is_empty());
    put(&rig.dir, "findings.json", "kaputt");
    let broken = store.findings(SESSION).unwrap_err().to_string();
    assert!(broken.starts_with("findings.json nicht lesbar: "), "{broken}");
}
