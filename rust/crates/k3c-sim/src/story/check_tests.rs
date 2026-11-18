//! Go: `story_test.go` (B-965, GS1.2), Teil Prüfung: `TestStoryPruefungen`, `TestStoryWoerterMitSonderzeichen`.

use super::check::{is_real_word, story_words};
use super::mod_tests::{STORY_LEX, story_world_json};
use crate::island::testutil::parse_world_in;

/// Go: `longLine`: 65 Zeichen lang, aber nur Kunst- und echte Wörter im Wortschatz.
fn long_line() -> String {
    format!("Oru KRONE {}SENN", "no ".repeat(17))
}

#[test]
#[allow(clippy::too_many_lines)] // Tabelle der Fälle wie in Go
fn story_pruefungen() {
    let big: Vec<String> = (0..25).map(|i| format!("w{}", "a".repeat(i))).collect();
    let big_json = serde_json::to_string(&big).expect("JSON");
    let long = format!(r#""arrival":["{}"]"#, long_line());
    let lex = STORY_LEX;
    let cases: Vec<(&str, String, String, Vec<&str>)> = vec![
        (
            "unbekanntes Feld Welt",
            r#"story: unbekanntes Feld "intro""#.into(),
            format!(r#"{lex},"intro":["x"]"#),
            vec![],
        ),
        (
            "unbekanntes Feld Insel",
            r#"Insel a: story: unbekanntes Feld "ende""#.into(),
            lex.into(),
            vec![r#""ende":["x"]"#],
        ),
        (
            "Wortschatz zu groß",
            "story.lexicon: 25 Wörter (höchstens 24)".into(),
            format!(r#""lexicon":{big_json}"#),
            vec![],
        ),
        (
            "Wortschatz Wort groß",
            r#"story.lexicon Wort 2: "Kamae" ist nicht klein geschrieben"#.into(),
            r#""lexicon":["oru","Kamae"]"#.into(),
            vec![],
        ),
        (
            "Wortschatz doppelt",
            r#"story.lexicon Wort 2: "oru" doppelt"#.into(),
            r#""lexicon":["oru","oru"]"#.into(),
            vec![],
        ),
        (
            "drei Zeilen",
            "story.start: 3 Zeilen (1 bis 2)".into(),
            format!(r#"{lex},"start":["a","b","c"]"#),
            vec![],
        ),
        (
            "null Zeilen",
            "story.end: 0 Zeilen (1 bis 2)".into(),
            format!(r#"{lex},"end":[]"#),
            vec![],
        ),
        (
            "leere Zeile",
            "story.end Zeile 2: leer".into(),
            format!(r#"{lex},"end":["Oru KRONE no MEER.","  "]"#),
            vec![],
        ),
        (
            "Zeile zu lang",
            "Insel b: story.arrival Zeile 1: 65 Zeichen (höchstens 64)".into(),
            lex.into(),
            vec!["", &long],
        ),
        (
            "Kunstwort fehlt",
            r#"story.start Zeile 1: Wort "kame" fehlt im lexicon"#.into(),
            format!(r#"{lex},"start":["Kame KRONE no MEER."]"#),
            vec![],
        ),
        (
            "Tippfehler",
            r#"Insel c: story.done Zeile 1: Wort "sem" fehlt im lexicon"#.into(),
            lex.into(),
            vec!["", "", r#""done":["Sem KRONE no MEER."]"#],
        ),
        (
            "ein echtes Wort",
            "Insel a: story.boss Zeile 1: 1 echtes Wort (2 bis 3)".into(),
            lex.into(),
            vec![r#""boss":["Oru KRONE no sen."]"#],
        ),
        (
            "vier echte Wörter",
            "story.etappen.1 Zeile 1: 4 echte Wörter (2 bis 3)".into(),
            format!(r#"{lex},"etappen":{{"1":["Oru KRONE MEER SCHIFF WIND."]}}"#),
            vec![],
        ),
        (
            "Flaschenpost",
            "story.bottles Eintrag 2 Zeile 1: 1 echtes Wort (2 bis 3)".into(),
            format!(r#"{lex},"bottles":[["Oru KRONE MEER."],["Oru KRONE."]]"#),
            vec![],
        ),
        (
            "Etappe ohne Insel",
            "story.etappen: Etappe 3 hat keine Insel".into(),
            format!(r#"{lex},"etappen":{{"3":["Oru KRONE MEER."]}}"#),
            vec![],
        ),
        (
            "Etappen-Schlüssel",
            r#"story.etappen: Schlüssel "01" ist keine Etappen-Nummer"#.into(),
            format!(r#"{lex},"etappen":{{"01":["Oru KRONE MEER."]}}"#),
            vec![],
        ),
    ];
    for (name, want, world, islands) in cases {
        let err = parse_world_in(&story_world_json(&world, &islands), "test").expect_err(name);
        assert!(
            err.contains("data/worlds/test.json: ") && err.contains(&want),
            "{name}: erwartet {want:?}, Fehler {err:?}"
        );
    }
}

/// Satzzeichen am Rand, Gedankenstrich, Apostroph und Bindestrich im Kunstwort, Umlaute und Sonderzeichen sind erlaubt.
#[test]
fn story_woerter_mit_sonderzeichen() {
    let ws = r#""lexicon":["akh'et","sen-hotep","hjørn","ma'u"],"start":["Akh'et ra KRONE – sen-hotep, hjørn FLUT!"]"#;
    let err = parse_world_in(&story_world_json(ws, &[]), "test").expect_err("ra fehlt im Wortschatz");
    assert!(err.contains(r#"Wort "ra" fehlt"#), "{err}");
    let ws = ws.replacen(r#""ma'u""#, r#""ma'u","ra""#, 1);
    let ws = format!(r#"{ws},"end":["Ma'u ÄTHER KÜSTE."]"#);
    let w = parse_world_in(&story_world_json(&ws, &[]), "test").expect("Welt");
    assert!(
        w.story.start.as_ref().map(Vec::len) == Some(1) && w.story.end.as_ref().map(Vec::len) == Some(1),
        "Zeilen {:?}",
        w.story
    );
}

#[test]
fn woerter_trennen_und_echte_woerter_erkennen() {
    assert_eq!(
        story_words("Akh'et ra KRONE – sen-hotep, hjørn FLUT!"),
        ["Akh'et", "ra", "KRONE", "sen-hotep", "hjørn", "FLUT"]
    );
    assert!(is_real_word("KRONE") && is_real_word("ÄTHER") && is_real_word("KÜSTE"));
    assert!(!is_real_word("Krone") && !is_real_word("A") && !is_real_word("oru") && !is_real_word("ma'u"));
    assert!(is_real_word("MA'U"), "Apostroph zählt nicht als Buchstabe");
}
