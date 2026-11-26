//! Gemeinsame Konsolen-Senke (Port von `engine/conlog`): eine Zeile je Eintrag,
//!
//! ```text
//! 17:03:11.042 ERROR 🔧 [svc] Dienst fehlgeschlagen port=8080 err="bereits belegt"
//! ```
//!
//! mit ANSI-Farben je Level statt key=value für alles. Die Level-Wörter (WARN, ERROR) bleiben im Text, damit Leser
//! ohne Farben (Workbench `markOf`, `outLevel`) sie weiter finden. Die JSON-Dateien schreibt [`crate::applog`].

use std::io::Write;
use std::sync::{Mutex, PoisonError};

use serde_json::Value;

use crate::logger::{Level, Record, Sink, value_string};

pub(crate) const RESET: &str = "\x1b[0m";
pub(crate) const BOLD: &str = "\x1b[1m";
pub(crate) const RED: &str = "\x1b[31m";
pub(crate) const BLUE: &str = "\x1b[34m";
pub(crate) const PURPLE: &str = "\x1b[35m";
pub(crate) const CYAN: &str = "\x1b[36m";
pub(crate) const GRAY: &str = "\x1b[90m";
/// `bold + red` und `bold + yellow` aus Go.
pub(crate) const BOLD_RED: &str = "\x1b[1m\x1b[31m";
pub(crate) const BOLD_YELLOW: &str = "\x1b[1m\x1b[33m";

/// Emoji je Namespace (`ns`), das die Konsole vor `[ns]` setzt: Themen auf einen Blick. Nur Emojis aus einem Codepunkt
/// (ohne U+FE0F), sonst verrutscht die Breite in manchen Terminals. Neuer `ns` → hier eintragen.
pub const TOPICS: &[(&str, &str)] = &[
    ("main", "🚀"),
    ("http", "📡"),
    ("ws", "🔌"),
    ("room", "🏰"),
    ("client", "🌐"),
    ("save", "💾"),
    ("store", "📦"),
    ("report", "📊"),
    ("diag", "🩺"),
    ("svc", "🔧"),
    ("check", "🧪"),
    ("tasks", "📋"),
    ("mcp", "🤖"),
    ("usage", "📈"),
    ("watch", "👀"),
    ("dev", "🐛"),
];

/// Emoji zum Namespace, falls eingetragen.
pub fn topic(ns: &str) -> Option<&'static str> {
    TOPICS.iter().find(|(k, _)| *k == ns).map(|(_, e)| *e)
}

/// Ob die Konsole Farben bekommt: aus, sobald `NO_COLOR` gesetzt ist (<https://no-color.org>).
pub fn color() -> bool {
    std::env::var_os("NO_COLOR").is_none_or(|v| v.is_empty())
}

/// Schreibt lesbare, farbige Zeilen in einen Writer (Go `conlog.Handler`).
pub struct ConsoleSink {
    w: Mutex<Box<dyn Write + Send>>,
    level: Level,
    color: bool,
}

impl ConsoleSink {
    /// Senke ab Stufe `level`; `color` schaltet die ANSI-Farben.
    pub fn new(w: Box<dyn Write + Send>, level: Level, color: bool) -> ConsoleSink {
        ConsoleSink {
            w: Mutex::new(w),
            level,
            color,
        }
    }

    /// Die Zeile eines Eintrags samt Zeilenende, genau wie Go `conlog.Handler.Handle`.
    pub fn format(&self, r: &Record) -> String {
        let mut b = String::new();
        self.paint(&mut b, GRAY, &r.time.format("%H:%M:%S%.3f").to_string());
        b.push(' ');
        let (lc, name) = level_style(r.level);
        self.paint(&mut b, lc, &name);
        let mut ns = String::new();
        let mut tail = String::new();
        for (k, v) in &r.attrs {
            if k == "ns" {
                ns = value_string(v);
            } else {
                self.attr(&mut tail, k, v);
            }
        }
        if !ns.is_empty() {
            b.push(' ');
            if let Some(e) = topic(&ns) {
                b.push_str(e);
                b.push(' ');
            }
            self.paint(&mut b, CYAN, &format!("[{ns}]"));
        }
        b.push(' ');
        let msg_color = if r.level >= Level::Error { BOLD_RED } else { BOLD };
        self.paint(&mut b, msg_color, &r.msg);
        b.push_str(&tail); // jedes Attribut bringt sein Leerzeichen mit
        b.push('\n');
        b
    }

    /// Schreibt ` key=value`; leere Attribute (ohne Schlüssel und Wert) fallen weg wie bei slog.
    fn attr(&self, b: &mut String, key: &str, v: &Value) {
        if key.is_empty() && v.is_null() {
            return;
        }
        b.push(' ');
        self.paint(b, GRAY, &format!("{key}="));
        let val = quote(&value_string(v));
        if key == "err" || key == "error" {
            self.paint(b, RED, &val);
        } else {
            b.push_str(&val);
        }
    }

    fn paint(&self, b: &mut String, code: &str, s: &str) {
        if self.color {
            b.push_str(code);
            b.push_str(s);
            b.push_str(RESET);
        } else {
            b.push_str(s);
        }
    }
}

impl Sink for ConsoleSink {
    fn enabled(&self, level: Level) -> bool {
        level >= self.level
    }

    fn handle(&self, r: &Record) -> std::io::Result<()> {
        let line = self.format(r);
        let mut w = self.w.lock().unwrap_or_else(PoisonError::into_inner);
        w.write_all(line.as_bytes())
    }
}

/// Farbe und auf 5 Zeichen aufgefüllter Name (DEBUG, INFO, WARN, ERROR).
fn level_style(l: Level) -> (&'static str, String) {
    let name = format!("{:<5}", l.name());
    let code = match l {
        Level::Error => BOLD_RED,
        Level::Warn => BOLD_YELLOW,
        Level::Info => BLUE,
        Level::Debug => PURPLE,
    };
    (code, name)
}

/// Setzt Werte mit Leer-, Gleich-, Anführungs- oder Steuerzeichen in Anführungszeichen (wie slog.TextHandler).
pub(crate) fn quote(s: &str) -> String {
    if s.is_empty() {
        return "\"\"".to_string();
    }
    if s.chars()
        .any(|r| r == '=' || r == '"' || r.is_whitespace() || !is_print(r))
    {
        return go_quote(s);
    }
    s.to_string()
}

/// Wie Go `strconv.Quote`: Anführungszeichen, Escapes für `\` und `"`, Steuerzeichen als `\n`, `\x1b`, ` ` …
fn go_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for r in s.chars() {
        match r {
            '"' | '\\' => {
                out.push('\\');
                out.push(r);
            }
            _ if is_print(r) => out.push(r),
            '\u{07}' => out.push_str("\\a"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{0b}' => out.push_str("\\v"),
            _ if (r as u32) < 0x20 || r == '\u{7f}' => out.push_str(&format!("\\x{:02x}", r as u32)),
            _ if (r as u32) < 0x10000 => out.push_str(&format!("\\u{:04x}", r as u32)),
            _ => out.push_str(&format!("\\U{:08x}", r as u32)),
        }
    }
    out.push('"');
    out
}

/// Näherung an Go `unicode.IsPrint` (Buchstaben, Zeichen, Zahlen, Satzzeichen, Symbole und das ASCII-Leerzeichen):
/// nicht druckbar sind Steuerzeichen, Leerraum außer `' '`, Formatzeichen und private Bereiche. Nicht vergebene
/// Codepunkte gelten mangels Unicode-Tabelle als druckbar.
fn is_print(r: char) -> bool {
    if r == ' ' {
        return true;
    }
    if r.is_control() || r.is_whitespace() {
        return false;
    }
    !matches!(r as u32,
        0xAD | 0x600..=0x605 | 0x61C | 0x6DD | 0x70F | 0x890..=0x891 | 0x8E2 | 0x180E | 0x200B..=0x200F
        | 0x202A..=0x202E | 0x2060..=0x2064 | 0x2066..=0x206F | 0xFEFF | 0xFFF9..=0xFFFB | 0xFFFE..=0xFFFF
        | 0x110BD | 0x110CD | 0x13430..=0x1343F | 0x1BCA0..=0x1BCA3 | 0x1D173..=0x1D17A | 0xE0001
        | 0xE0020..=0xE007F | 0xE000..=0xF8FF | 0xF0000..=0x10FFFF)
}

#[cfg(test)]
#[path = "conlog_tests.rs"]
mod tests;
