//! Port von `claude_test.go`; `open` läuft hier nie mit gültigem Prompt (es würde Claude Desktop öffnen).

use super::*;

/// Dekodiert `%XX`, wie es Claude Desktop tut.
fn decode(value: &str) -> String {
    let mut bytes = Vec::new();
    let mut rest = value.bytes();
    while let Some(byte) = rest.next() {
        if byte == b'%' {
            let hex = [rest.next().unwrap(), rest.next().unwrap()];
            bytes.push(u8::from_str_radix(std::str::from_utf8(&hex).unwrap(), 16).unwrap());
        } else {
            bytes.push(byte);
        }
    }
    String::from_utf8(bytes).unwrap()
}

#[test]
fn link_kodiert_und_dekodiert() {
    let prompt = "Fix #12 & prüfe a+b=c\nzweite Zeile";
    let folder = r"C:\WORKSPACE\k3c";
    let built = link(prompt, folder).unwrap();
    assert!(!built.contains('+') && !built.contains(' '), "{built}");
    let query = built.strip_prefix("claude://code/new?").unwrap();
    let (first, second) = query.split_once('&').unwrap();
    assert_eq!(decode(first.strip_prefix("folder=").unwrap()), folder);
    assert_eq!(decode(second.strip_prefix("q=").unwrap()), prompt);
}

/// Zeichen für Zeichen wie Go `url.Values.Encode` mit `+` → `%20`.
#[test]
fn link_wie_go() {
    let built = link("a b+ü%\"'|<>^!*()-_.~/?:", "").unwrap();
    assert_eq!(
        built,
        "claude://code/new?q=a%20b%2B%C3%BC%25%22%27%7C%3C%3E%5E%21%2A%28%29-_.~%2F%3F%3A"
    );
}

#[test]
fn link_grenzen() {
    assert_eq!(link("  ", ""), Err("prompt ist leer".to_string()));
    assert!(link(&"x".repeat(MAX_PROMPT), "").is_ok());
    assert_eq!(
        link(&"x".repeat(MAX_PROMPT + 1), ""),
        Err("prompt hat 14001 Zeichen, Claude Desktop übernimmt höchstens 14000".to_string())
    );
    // Kodiert zu lang: 6000 Umlaute sind 36.000 Zeichen.
    assert_eq!(
        link(&"ü".repeat(6000), ""),
        Err("prompt ist kodiert zu lang für einen Link (36020 Zeichen)".to_string())
    );
    // Auch der Ordner zählt zur Länge.
    assert!(link("x", &"y".repeat(MAX_LINK)).is_err());
}

#[test]
fn open_ohne_prompt_startet_nichts() {
    assert_eq!(open(" ", ""), Err("prompt ist leer".to_string()));
}
