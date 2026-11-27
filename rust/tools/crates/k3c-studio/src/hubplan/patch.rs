//! Format-stabiles Schreiben der hub.json (Port von `patch.go`, B-462/AC-06): Der Planer liefert den Kandidaten als JSON
//! beliebiger Form; geschrieben wird die alte Datei, in der nur die geänderten Werte ersetzt sind. Schlüsselreihenfolge,
//! Einrückung, Zahlenformat und Zeilenenden bleiben, eine geänderte Zahl ist ein Ein-Zeilen-Diff. Hinzugefügte und
//! entfernte Listen-Einträge folgen dem Layout der Liste (eine Zeile je Eintrag).

use std::collections::BTreeMap;

use serde::de::IgnoredAny;

use crate::kitx::go_quote;

#[cfg(test)]
#[path = "patch_tests.rs"]
pub(super) mod tests;

/// Tiefste Schachtelung: der Text wird rekursiv verarbeitet.
const MAX_DEPTH: usize = 64;

/// Ein JSON-Wert mit seiner Lage im Text: `kind` `{`, `[` oder 0 (Zahl, Text, true, false, null).
struct Node {
    start: usize,
    end: usize,
    kind: u8,
    /// Objekt: Schlüssel in Dateireihenfolge, parallel zu `kids`.
    keys: Vec<String>,
    kids: Vec<Node>,
}

/// Die Bedeutung eines Werts: Zahlenformat, Leerraum und Schlüsselreihenfolge zählen nicht.
#[derive(PartialEq)]
enum Meaning {
    Null,
    Bool(bool),
    Number(f64),
    Text(String),
    List(Vec<Meaning>),
    Object(BTreeMap<String, Meaning>),
}

/// Liest einen Text, den `serde_json` schon als gültig erkannt hat, und merkt die Lagen aller Teilwerte.
struct Scanner<'a> {
    raw: &'a str,
    at: usize,
}

impl Scanner<'_> {
    /// Überspringt Leerraum und die Trenner `,` und `:`.
    fn skip(&mut self) {
        let bytes = self.raw.as_bytes();
        while bytes.get(self.at).is_some_and(|b| b" \t\r\n,:".contains(b)) {
            self.at += 1;
        }
    }

    /// Hinter das Ende des Strings, der bei `at` beginnt.
    fn string_end(&mut self) {
        let bytes = self.raw.as_bytes();
        self.at += 1;
        while let Some(&b) = bytes.get(self.at) {
            self.at += if b == b'\\' { 2 } else { 1 };
            if b == b'"' {
                break;
            }
        }
    }

    fn text(&mut self) -> Result<String, String> {
        let start = self.at;
        self.string_end();
        let token = self.raw.get(start..self.at).unwrap_or_default();
        serde_json::from_str(token).map_err(|e| e.to_string())
    }

    fn value(&mut self, depth: usize) -> Result<Node, String> {
        self.skip();
        let bytes = self.raw.as_bytes();
        let start = self.at;
        let mut node = Node {
            start,
            end: start,
            kind: 0,
            keys: Vec::new(),
            kids: Vec::new(),
        };
        match bytes.get(start) {
            Some(&open @ (b'{' | b'[')) => {
                if depth >= MAX_DEPTH {
                    return Err(format!("mehr als {MAX_DEPTH} Ebenen"));
                }
                node.kind = open;
                self.at += 1;
                self.skip();
                while !matches!(bytes.get(self.at), None | Some(b'}' | b']')) {
                    if open == b'{' {
                        node.keys.push(self.text()?);
                    }
                    node.kids.push(self.value(depth + 1)?);
                    self.skip();
                }
                self.at += 1;
            }
            Some(b'"') => self.string_end(),
            _ => {
                while bytes.get(self.at).is_some_and(|b| !b" \t\r\n,:]}".contains(b)) {
                    self.at += 1;
                }
            }
        }
        node.end = self.at;
        Ok(node)
    }
}

/// Liest `raw` als genau einen JSON-Wert mit den Lagen aller Teilwerte.
fn scan(raw: &str) -> Result<Node, String> {
    serde_json::from_str::<IgnoredAny>(raw).map_err(|e| e.to_string())?;
    Scanner { raw, at: 0 }.value(0)
}

/// Die Bedeutung des Werts `node` im Text `raw`; `None`: nicht lesbar (etwa eine zu große Zahl).
fn meaning(raw: &str, node: &Node) -> Option<Meaning> {
    let token = &raw[node.start..node.end];
    Some(match node.kind {
        b'{' => {
            let mut map = BTreeMap::new();
            for (key, kid) in node.keys.iter().zip(&node.kids) {
                map.insert(key.clone(), meaning(raw, kid)?);
            }
            Meaning::Object(map)
        }
        b'[' => Meaning::List(node.kids.iter().map(|k| meaning(raw, k)).collect::<Option<_>>()?),
        _ => match token {
            "null" => Meaning::Null,
            "true" => Meaning::Bool(true),
            "false" => Meaning::Bool(false),
            _ if token.starts_with('"') => Meaning::Text(serde_json::from_str(token).ok()?),
            _ => Meaning::Number(token.parse().ok().filter(|n: &f64| n.is_finite())?),
        },
    })
}

/// `a` und `b` bedeuten denselben Wert.
fn same(a: &str, x: &Node, b: &str, y: &Node) -> bool {
    matches!((meaning(a, x), meaning(b, y)), (Some(x), Some(y)) if x == y)
}

/// Zwei JSON-Texte bedeuten denselben Wert.
pub(super) fn same_json(a: &str, b: &str) -> bool {
    matches!((scan(a), scan(b)), (Ok(x), Ok(y)) if same(a, &x, b, &y))
}

/// Schreibt `cand` in das Format von `old`: unveränderte Werte bleiben byte-gleich, geänderte werden ersetzt. Das
/// Ergebnis bedeutet garantiert dasselbe wie `cand`, sonst ist es ein Fehler.
pub(super) fn patch_json(old: &str, cand: &str) -> Result<String, String> {
    let o = scan(old)?;
    let c = scan(cand)?;
    let nl = if old.contains("\r\n") { "\r\n" } else { "\n" };
    let patcher = Patcher { old, cand, nl };
    let out = format!("{}{}{}", &old[..o.start], patcher.value(&o, &c), &old[o.end..]);
    if !same_json(&out, cand) {
        return Err("kandidat lässt sich nicht format-stabil schreiben".to_owned());
    }
    Ok(out)
}

/// Dieselben Schlüssel, Reihenfolge egal.
fn same_keys(a: &[String], b: &[String]) -> bool {
    a.len() == b.len() && a.iter().all(|k| b.contains(k))
}

/// Der führende Leerraum der Zeile, in der `at` steht.
fn line_indent(raw: &str, at: usize) -> &str {
    let from = raw.as_bytes()[..at]
        .iter()
        .rposition(|b| *b == b'\n')
        .map_or(0, |i| i + 1);
    let line = &raw[from..];
    let len = line.len() - line.trim_start_matches([' ', '\t']).len();
    &line[..len]
}

/// Ein Schlüssel mit seinem Wert.
fn member(key: &str, value: &str) -> String {
    format!("{}: {value}", go_quote(key))
}

struct Patcher<'a> {
    old: &'a str,
    cand: &'a str,
    nl: &'a str,
}

impl Patcher<'_> {
    fn old_text(&self, n: &Node) -> &str {
        &self.old[n.start..n.end]
    }

    /// Der Text für `c` an der Stelle von `o`.
    fn value(&self, o: &Node, c: &Node) -> String {
        if same(self.old, o, self.cand, c) {
            return self.old_text(o).to_owned();
        }
        match (o.kind, c.kind) {
            (b'{', b'{') if same_keys(&o.keys, &c.keys) => {
                let kids: Vec<&Node> = o
                    .keys
                    .iter()
                    .filter_map(|key| c.keys.iter().position(|k| k == key).map(|i| &c.kids[i]))
                    .collect();
                self.splice(o, &kids)
            }
            (b'{', b'{') => self.object(o, c),
            (b'[', b'[') => self.array(o, c),
            _ => self.inline(c),
        }
    }

    /// Ersetzt die Teilwerte von `o` paarweise durch die von `kids`; der Text dazwischen bleibt.
    fn splice(&self, o: &Node, kids: &[&Node]) -> String {
        let mut out = String::new();
        let mut at = o.start;
        for (old, new) in o.kids.iter().zip(kids) {
            out.push_str(&self.old[at..old.start]);
            out.push_str(&self.value(old, new));
            at = old.end;
        }
        out.push_str(&self.old[at..o.end]);
        out
    }

    /// Schlüssel hinzugekommen oder entfallen: bleibende in der alten Reihenfolge, neue dahinter in der des
    /// Kandidaten, Layout von `o`.
    fn object(&self, o: &Node, c: &Node) -> String {
        let mut parts = Vec::new();
        for (key, old) in o.keys.iter().zip(&o.kids) {
            if let Some(i) = c.keys.iter().position(|k| k == key) {
                parts.push(member(key, &self.value(old, &c.kids[i])));
            }
        }
        for (key, new) in c.keys.iter().zip(&c.kids) {
            if !o.keys.contains(key) {
                parts.push(member(key, &self.inline(new)));
            }
        }
        self.layout(o, ("{", "}"), &parts)
    }

    /// Einträge paarweise, solange die Anzahl gleich ist; sonst neu gelegt, unveränderte Einträge byte-gleich.
    fn array(&self, o: &Node, c: &Node) -> String {
        if o.kids.len() == c.kids.len() {
            return self.splice(o, &c.kids.iter().collect::<Vec<_>>());
        }
        let mut parts = Vec::with_capacity(c.kids.len());
        let mut j = 0;
        for (i, new) in c.kids.iter().enumerate() {
            let rest = &o.kids[j..];
            if let Some(at) = rest.iter().position(|n| same(self.old, n, self.cand, new)) {
                parts.push(self.old_text(&rest[at]).to_owned());
                j += at + 1;
                continue;
            }
            // Geänderter Eintrag: nur, wenn der alte im Kandidaten nicht mehr vorkommt.
            let changed = rest
                .first()
                .filter(|old| !c.kids[i..].iter().any(|n| same(self.cand, n, self.old, old)));
            match changed {
                Some(old) => {
                    parts.push(self.value(old, new));
                    j += 1;
                }
                None => parts.push(self.fresh(o, new)),
            }
        }
        self.layout(o, ("[", "]"), &parts)
    }

    /// Ein neuer Eintrag der Liste `o` wie ihr erster: ein mehrzeiliges Objekt mehrzeilig, sonst einzeilig.
    fn fresh(&self, o: &Node, new: &Node) -> String {
        let sibling = o
            .kids
            .first()
            .filter(|s| s.kind == b'{' && !s.kids.is_empty() && self.old_text(s).contains('\n'));
        let Some(sibling) = sibling.filter(|_| new.kind == b'{' && !new.kids.is_empty()) else {
            return self.inline(new);
        };
        let inner = line_indent(self.old, sibling.kids[0].start);
        let outer = line_indent(self.old, sibling.end - 1);
        let parts: Vec<String> = new
            .keys
            .iter()
            .zip(&new.kids)
            .map(|(key, v)| member(key, &self.inline(v)))
            .collect();
        let nl = self.nl;
        let body = parts.join(&format!(",{nl}{inner}"));
        format!("{{{nl}{inner}{body}{nl}{outer}}}")
    }

    /// Legt `parts` wie `o`: einzeilig `{ a, b }` bzw. `[a, b]`, mehrzeilig ein Teil je Zeile mit der Einrückung von `o`.
    fn layout(&self, o: &Node, (open, close): (&str, &str), parts: &[String]) -> String {
        if !self.old_text(o).contains('\n') || parts.is_empty() {
            if open == "{" && !parts.is_empty() {
                return format!("{{ {} }}", parts.join(", "));
            }
            return format!("{open}{}{close}", parts.join(", "));
        }
        let outer = line_indent(self.old, o.end - 1);
        let inner = o
            .kids
            .first()
            .map_or_else(|| format!("{outer}  "), |k| line_indent(self.old, k.start).to_owned());
        let nl = self.nl;
        let body = parts.join(&format!(",{nl}{inner}"));
        format!("{open}{nl}{inner}{body}{nl}{outer}{close}")
    }

    /// Ein neuer Wert des Kandidaten, einzeilig im Stil der Datei: `{ "a": 1 }`, `[1, 2]`.
    fn inline(&self, c: &Node) -> String {
        match c.kind {
            b'{' => {
                let parts: Vec<String> = c
                    .keys
                    .iter()
                    .zip(&c.kids)
                    .map(|(key, v)| member(key, &self.inline(v)))
                    .collect();
                if parts.is_empty() {
                    return "{}".to_owned();
                }
                format!("{{ {} }}", parts.join(", "))
            }
            b'[' => {
                let parts: Vec<String> = c.kids.iter().map(|k| self.inline(k)).collect();
                format!("[{}]", parts.join(", "))
            }
            _ => self.cand[c.start..c.end].to_owned(),
        }
    }
}
