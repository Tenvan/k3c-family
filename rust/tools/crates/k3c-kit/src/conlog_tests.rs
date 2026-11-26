//! Port von `engine/conlog/conlog_test.go`. Gruppen (`WithGroup`, `slog.Group`) gibt es im Rust-Logger nicht; der
//! Gruppenpfad aus Go erscheint hier als Schlüssel mit Punkt, die Zeile bleibt zeichengleich.

use std::io::Write;
use std::sync::{Arc, Mutex};

use serde_json::json;

use super::*;
use crate::logger::Logger;

/// Geteilter Puffer als Writer (Gegenstück zu `bytes.Buffer`).
#[derive(Clone, Default)]
struct SharedBuf(Arc<Mutex<Vec<u8>>>);

impl Write for SharedBuf {
    fn write(&mut self, p: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(p);
        Ok(p.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl SharedBuf {
    fn text(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }
}

#[test]
fn zeile_ohne_farben() {
    let buf = SharedBuf::default();
    let log = Logger::new(vec![Box::new(ConsoleSink::new(
        Box::new(buf.clone()),
        Level::Debug,
        false,
    ))])
    .with(&[("ns", json!("room"))]);
    log.error(
        "Raum zu",
        &[
            ("req.code", json!("FAMILIE")),
            ("req.err", json!("weg da")),
            ("req.p.x", json!(1)),
            ("req.leer", json!("")),
        ],
    );
    let out = buf.text();
    let got = &out["15:04:05.000 ".len()..];
    let want = "ERROR 🏰 [room] Raum zu req.code=FAMILIE req.err=\"weg da\" req.p.x=1 req.leer=\"\"\n";
    assert_eq!(got, want);
}

#[test]
fn farben_und_level() {
    let buf = SharedBuf::default();
    let log = Logger::new(vec![Box::new(ConsoleSink::new(
        Box::new(buf.clone()),
        Level::Info,
        true,
    ))]);
    log.debug("versteckt", &[]);
    log.warn("achtung", &[("ns", json!("svc"))]);
    let out = buf.text();
    assert!(
        !out.contains("versteckt")
            && out.contains(&format!("{BOLD_YELLOW}WARN "))
            && out.contains(&format!("{CYAN}[svc]")),
        "{out:?}"
    );
}

#[test]
fn quote_wie_go() {
    let cases = [
        ("a", "a"),
        ("", "\"\""),
        ("a=b", "\"a=b\""),
        ("tab\tx", "\"tab\\tx\""),
        ("\x1b[0m", "\"\\x1b[0m\""),
        ("sag \"hi\"", "\"sag \\\"hi\\\"\""),
        ("C:\\pfad", "C:\\pfad"),
        ("nb\u{a0}sp", "\"nb\\u00a0sp\""),
        ("äöü👑", "äöü👑"),
    ];
    for (input, want) in cases {
        assert_eq!(quote(input), want, "{input:?}");
    }
}

#[test]
fn topics_wortgleich_aus_go() {
    assert_eq!(TOPICS.len(), 16);
    assert_eq!(topic("mcp"), Some("🤖"));
    assert_eq!(topic("res"), None);
    assert!(
        TOPICS.iter().all(|(_, e)| e.chars().count() == 1),
        "nur Emojis aus einem Codepunkt"
    );
}
