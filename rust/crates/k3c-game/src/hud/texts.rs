//! Port von `native/k3c-game/hud/texts.go`.
//!
//! Spieltexte des HUD (Spiegel von src/core/texts.de.ts und texts.en.ts, nur die Schlüssel, die das HUD zeigt). Einzige
//! Stelle mit deutschen und englischen Texten im Paket. Deutsch ist Standard und Rückfall.
//!
//! Abweichung von Go (Leitfaden § 3): Die Tabellen `textsDE` und `textsEN` stehen als nach Schlüssel sortierte Slices in
//! `texts_de.rs` und `texts_en.rs` (Datei ≤ 400 Zeilen); die Suche ist eine Binärsuche. Go-`kv ...any` wird
//! `&[(&str, Arg)]`.

use super::texts_de::TEXTS_DE;
use super::texts_en::TEXTS_EN;
use crate::view::settings::LANGUAGE_ENGLISH;

/// Arg ist ein Wert für einen Platzhalter (Go: `string`, `int` oder `float64` in `kv ...any`).
#[derive(Clone, Debug, PartialEq)]
pub enum Arg {
    Str(String),
    Int(i64),
    Float(f64),
}

impl From<&str> for Arg {
    fn from(s: &str) -> Self {
        Arg::Str(s.to_string())
    }
}

impl From<String> for Arg {
    fn from(s: String) -> Self {
        Arg::Str(s)
    }
}

impl From<i64> for Arg {
    fn from(n: i64) -> Self {
        Arg::Int(n)
    }
}

impl From<f64> for Arg {
    fn from(x: f64) -> Self {
        Arg::Float(x)
    }
}

/// find sucht den Schlüssel in einer nach Schlüssel sortierten Tabelle.
fn find(table: &'static [(&'static str, &'static str)], key: &str) -> Option<&'static str> {
    table.binary_search_by(|(k, _)| (*k).cmp(key)).ok().map(|i| table[i].1)
}

/// tr liefert den Text zum Schlüssel in der Sprache lang (fehlt er: Deutsch, fehlt auch der: der Schlüssel selbst).
pub fn tr<'a>(lang: &str, key: &'a str) -> &'a str {
    if lang == LANGUAGE_ENGLISH
        && let Some(s) = find(TEXTS_EN, key)
    {
        return s;
    }
    if let Some(s) = find(TEXTS_DE, key) {
        return s;
    }
    key
}

/// has meldet, ob es den Schlüssel gibt (wie `key in de` im Browser).
pub fn has(key: &str) -> bool {
    find(TEXTS_DE, key).is_some()
}

/// name_of ist der Name aus den Texten (`group.id`), sonst fallback (wie texts.ts › nameOf).
pub fn name_of(lang: &str, group: &str, id: &str, fallback: &str) -> String {
    let key = format!("{group}.{id}");
    if has(&key) {
        return tr(lang, &key).to_string();
    }
    fallback.to_string()
}

/// tf liefert den Text zum Schlüssel mit ersetzten Platzhaltern; kv sind Paare Name, Wert. Ein unbekannter Platzhalter
/// bleibt stehen.
pub fn tf(lang: &str, key: &str, kv: &[(&str, Arg)]) -> String {
    fill(tr(lang, key), kv)
}

/// fill ersetzt `{name}` in tpl durch die Werte aus kv (Paare Name, Wert).
pub fn fill(tpl: &str, kv: &[(&str, Arg)]) -> String {
    if !tpl.contains('{') {
        return tpl.to_string();
    }
    let mut b = String::new();
    let mut tpl = tpl;
    loop {
        let i = tpl.find('{');
        // wie Go: j zählt ab der Stelle von `{` (ohne `{`: ab 0)
        let j = tpl[i.unwrap_or(0)..].find('}');
        let (Some(i), Some(j)) = (i, j) else {
            b.push_str(tpl);
            return b;
        };
        let name = &tpl[i + 1..i + j];
        b.push_str(&tpl[..i]);
        match lookup(kv, name) {
            Some(v) => b.push_str(&v),
            None => b.push_str(&tpl[i..=i + j]),
        }
        tpl = &tpl[i + j + 1..];
    }
}

/// lookup sucht den Wert zu name in den Paaren kv.
fn lookup(kv: &[(&str, Arg)], name: &str) -> Option<String> {
    kv.iter().find(|(k, _)| *k == name).map(|(_, v)| str_of(v))
}

/// str_of schreibt einen Wert als Text (ganze Zahlen ohne Nachkommastellen; Go: `str`).
fn str_of(v: &Arg) -> String {
    match v {
        Arg::Str(s) => s.clone(),
        Arg::Int(n) => n.to_string(),
        // Go: strconv.FormatFloat(x, 'f', -1, 64); Rust-Display ist ebenfalls die kürzeste Form ohne Exponent
        Arg::Float(x) if x.is_infinite() => (if *x > 0.0 { "+Inf" } else { "-Inf" }).to_string(),
        Arg::Float(x) => x.to_string(),
    }
}
