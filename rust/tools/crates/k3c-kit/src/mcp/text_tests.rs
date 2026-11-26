//! Tests der Text-Hilfen (in Go nur indirekt über Server und Zähler geprüft).

use std::collections::HashSet;

use super::*;

#[test]
fn ref_name_eindeutig_und_wie_go() {
    // Go: fmt.Sprintf("%07x", (id*2654435761)&0xfffffff)
    assert_eq!(ref_name(0), "0000000");
    assert_eq!(ref_name(1), "e3779b1");
    assert_eq!(ref_name(2), "c6ef362");
    let names: HashSet<String> = (1..=10_000).map(ref_name).collect();
    assert_eq!(names.len(), 10_000);
    assert!(names.iter().all(|n| n.len() == 7));
}

#[test]
fn clip_zaehlt_zeichen() {
    let short = "ä".repeat(TEXT_RUNES);
    assert_eq!(clip(&short), short);
    let cut = clip(&"ä".repeat(TEXT_RUNES + 1));
    assert_eq!(cut.chars().count(), TEXT_RUNES);
    assert!(cut.ends_with('…'));
}

#[test]
fn clip_args_kuerzt_bei_200() {
    assert_eq!(clip_args(br#"{"a":1}"#), r#"{"a":1}"#);
    assert_eq!(clip_args(&[b'x'; ARGS_RUNES]).len(), ARGS_RUNES);
    let cut = clip_args(&[b'x'; 500]);
    assert_eq!(cut, format!("{}…", "x".repeat(ARGS_RUNES - 1)));
}

#[test]
fn or_dash_fuer_leer() {
    assert_eq!((or_dash(""), or_dash("a")), ("–", "a"));
}

#[test]
fn text_result_und_result_text() {
    let mut r = text_result("eins", false);
    assert_eq!(r.is_error, None);
    r.content.push(ContentBlock::image("AA==", "image/png"));
    r.content.push(ContentBlock::text("zwei"));
    assert_eq!(result_text(&r), "eins\nzwei");
    assert_eq!(text_result("x", true).is_error, Some(true));
}

#[test]
fn outcome_of_erfolg_fehlerergebnis_und_protokollfehler() {
    let ok = outcome_of(&Ok(text_result("erste\nzweite", false)));
    assert_eq!((ok.ok, ok.err.as_str(), ok.summary.as_str()), (true, "", "erste"));
    let bad = outcome_of(&Ok(text_result("kaputt\nmehr", true)));
    assert_eq!(
        (bad.ok, bad.err.as_str(), bad.summary.as_str()),
        (false, "kaputt\nmehr", "kaputt")
    );
    let err = outcome_of(&Err(ErrorData::invalid_params("falsch", None)));
    assert_eq!((err.ok, err.err.as_str(), err.summary.as_str()), (false, "falsch", ""));
}
