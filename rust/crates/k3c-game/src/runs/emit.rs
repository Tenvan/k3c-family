//! JSON-Zeilen auf stdout, byte-gleich zu Gos `json.Marshal` (RU33.1): Felder in fester Folge, Map-Schlüssel sortiert,
//! HTML-Escape (`<`, `>`, `&`) und U+2028/U+2029 wie Go. Die Zeilen tragen nur Text und ganze Zahlen; Zahlen mit
//! Nachkommastellen kommen nicht vor (Texte schreiben sie selbst über `k3c_det::golden::go_float`).

use std::io::Write;

/// Schreibt `s` als JSON-Text wie Go (`encoding/json`, mit HTML-Escape).
pub fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '<' | '>' | '&' | '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c if u32::from(c) < 0x20 => out.push_str(&format!("\\u{:04x}", u32::from(c))),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Schreibt JSON-Zeilen; ein Fehler beim Schreiben (Empfänger weg) wird verschluckt, der Lauf endet über den Code.
pub struct Emitter<'a> {
    out: &'a mut dyn Write,
}

impl<'a> Emitter<'a> {
    pub fn new(out: &'a mut dyn Write) -> Self {
        Emitter { out }
    }

    /// Eine Zeile aus Text-Feldern; die Schlüssel stehen sortiert wie bei Gos `map[string]string`.
    pub fn text_line(&mut self, fields: &[(&str, &str)]) {
        let mut sorted: Vec<&(&str, &str)> = fields.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));
        let body: Vec<String> = sorted
            .iter()
            .map(|(k, v)| format!("{}:{}", json_string(k), json_string(v)))
            .collect();
        self.raw(&format!("{{{}}}", body.join(",")));
    }

    /// `{"event":"result","text":"…"}`.
    pub fn result_text(&mut self, text: &str) {
        self.text_line(&[("event", "result"), ("text", text)]);
    }

    /// `{"event":"result","report":{…}}`; `report` ist fertiger JSON-Text.
    pub fn result_report(&mut self, report: &str) {
        self.raw(&format!("{{\"event\":\"result\",\"report\":{report}}}"));
    }

    /// `{"event":"error","message":"…"}`.
    pub fn error(&mut self, message: &str) {
        self.text_line(&[("event", "error"), ("message", message)]);
    }

    /// `{"event":"aborted"}`.
    pub fn aborted(&mut self) {
        self.text_line(&[("event", "aborted")]);
    }

    /// `{"event":"progress","done":N,"total":N}` (Feldfolge fest, wie Gos `progressEvent`).
    pub fn progress(&mut self, done: i64, total: i64) {
        self.raw(&format!("{{\"event\":\"progress\",\"done\":{done},\"total\":{total}}}"));
    }

    fn raw(&mut self, line: &str) {
        let _ = writeln!(self.out, "{line}");
        let _ = self.out.flush();
    }
}
