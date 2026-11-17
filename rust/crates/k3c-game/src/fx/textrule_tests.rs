//! Port von `native/k3c-game/fx/textrule_test.go`.
//!
//! Textregel (textRule.test.ts): Spieltexte stehen nur in `texts.rs`; kein anderes Zeichenketten-Literal mit Umlaut oder
//! zwei Wörtern.
//!
//! Abweichung von Go: Es gibt keinen Parser (`go/parser`) im Test. Ein Zeichenscan sucht die Literale `"…"` der Quelle
//! (ohne Kommentare, Zeichen-Literale und Raw-Strings); die Regel ist dieselbe, Ausnahme wie in Go nur Texte mit 💥.

use std::fs;
use std::path::Path;

/// `literals` liefert die Zeichenketten-Literale einer Quelle ohne Kommentare (`//`, `/* */`) und ohne Zeichen-Literale.
fn literals(src: &str) -> Vec<String> {
    let c: Vec<char> = src.chars().collect();
    let (mut out, mut i) = (Vec::new(), 0);
    while i < c.len() {
        match c[i] {
            '/' if c.get(i + 1) == Some(&'/') => {
                while i < c.len() && c[i] != '\n' {
                    i += 1;
                }
            }
            '/' if c.get(i + 1) == Some(&'*') => {
                i += 2;
                while i + 1 < c.len() && !(c[i] == '*' && c[i + 1] == '/') {
                    i += 1;
                }
                i += 2;
            }
            '\'' => {
                // Zeichen-Literal ('"', '\n') oder Lebenszeit ('a)
                if c.get(i + 1) == Some(&'\\') {
                    i += 2;
                    while i < c.len() && c[i] != '\'' {
                        i += 1;
                    }
                    i += 1;
                } else if c.get(i + 2) == Some(&'\'') {
                    i += 3;
                } else {
                    i += 1;
                }
            }
            '"' => {
                let mut s = String::new();
                i += 1;
                while i < c.len() && c[i] != '"' {
                    if c[i] == '\\' && i + 1 < c.len() {
                        i += 1;
                    }
                    s.push(c[i]);
                    i += 1;
                }
                i += 1;
                out.push(s);
            }
            _ => i += 1,
        }
    }
    out
}

/// `is_text`: Umlaut oder zwei Buchstabenfolgen (je mindestens 2) mit Leerraum dazwischen.
fn is_text(s: &str) -> bool {
    if s.chars().any(|c| "äöüßÄÖÜ".contains(c)) {
        return true;
    }
    let c: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < c.len() {
        if !c[i].is_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        while i < c.len() && c[i].is_whitespace() {
            i += 1;
        }
        let before = c[..start].iter().rev().take_while(|x| x.is_ascii_alphabetic()).count();
        let after = c[i..].iter().take_while(|x| x.is_ascii_alphabetic()).count();
        if before >= 2 && after >= 2 {
            return true;
        }
    }
    false
}

#[test]
fn keine_texte_ausserhalb_der_textdatei() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/fx");
    let mut checked = 0;
    for entry in fs::read_dir(&dir).expect("Ordner fx") {
        let path = entry.expect("Eintrag").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        if !name.ends_with(".rs") || name.ends_with("_tests.rs") || name == "texts.rs" || name == "testutil.rs" {
            continue;
        }
        checked += 1;
        let src = fs::read_to_string(&path).expect("Quelle");
        for s in literals(&src) {
            assert!(
                !is_text(&s) || s.contains('💥'),
                "{name}: Text-Literal {s:?} gehört nach texts.rs"
            );
        }
    }
    assert!(checked >= 8, "zu wenige Dateien geprüft: {checked}");
}

#[test]
fn textregel_erkennt_literale_und_texte() {
    let src =
        "let a = \"kein Ziel\"; // \"Kommentar mit Worten\"\nlet b = '\"'; let c = \"hit\"; /* \"noch mehr Text\" */";
    assert_eq!(literals(src), vec!["kein Ziel".to_string(), "hit".to_string()]);
    assert!(is_text("kein Ziel") && is_text("Größe") && !is_text("hit") && !is_text("RU18: auras_of"));
}
