//! Layout als JSON in der Form von `json.Marshal(level.Layout)`: gleiche Feldnamen und Reihenfolge, ganze Zahlen ohne
//! `.0`, Strings mit den Escapes von Go (`<`, `>`, `&`, U+2028/2029). Mit `with_sim` kommen die Felder dazu, die Go
//! nicht in die level-Nachricht schreibt (`sites`, `freeGround`, `ruins`, `guards`), für den Vergleich mit Go.

use super::Layout;
use super::ruins::Spot;
use std::fmt::Write;

/// Schreibt eine Zahl wie Go (`strconv.FormatFloat(v, 'f', -1, 64)` für 1e-6 ≤ |v| < 1e21): kürzeste Ziffernfolge, die
/// zurück denselben Wert ergibt, ohne Exponent und ganze Werte ohne Nachkomma. Beides leistet `Display` von `f64`.
fn num(out: &mut String, v: f64) {
    let _ = write!(out, "{v}");
}

/// Schreibt einen String mit den Escapes von `encoding/json` (HTML-sicher).
fn text(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' | '>' | '&' | '\u{2028}' | '\u{2029}' => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn key(out: &mut String, k: &str) {
    out.push('"');
    out.push_str(k);
    out.push_str("\":");
}

fn spots(out: &mut String, name: &str, list: &[Spot]) {
    out.push(',');
    key(out, name);
    out.push('[');
    for (i, s) in list.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"kind\":");
        text(out, s.kind);
        out.push_str(",\"x\":");
        num(out, s.x);
        out.push('}');
    }
    out.push(']');
}

impl Layout<'_> {
    /// JSON des Layouts; `with_sim` hängt Plätze, Baugrund, Ruinen und Wachen an.
    pub fn to_json(&self, with_sim: bool) -> String {
        let mut out = String::with_capacity(64 * (self.entities.len() + self.chunks.len()) + 256);
        out.push_str("{\"seed\":");
        text(&mut out, self.seed);
        out.push_str(",\"biomeId\":");
        text(&mut out, self.biome_id);
        for (k, v) in [
            ("widthUnits", self.width),
            ("chunkWidthUnits", self.cw),
            ("hubCenterUnits", self.hub),
        ] {
            out.push(',');
            key(&mut out, k);
            num(&mut out, v);
        }
        out.push_str(",\"chunks\":[");
        for (i, c) in self.chunks.iter().enumerate() {
            out.push_str(if i > 0 { ",{\"index\":" } else { "{\"index\":" });
            let _ = write!(out, "{}", c.index);
            out.push_str(",\"kind\":");
            text(&mut out, c.kind);
            out.push_str(",\"startUnits\":");
            num(&mut out, c.start);
            out.push('}');
        }
        out.push(']');
        spots(&mut out, "entities", &self.entities);
        if !self.backdrops.is_empty() {
            out.push_str(",\"backdrops\":[");
            for (i, v) in self.backdrops.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                let _ = write!(out, "{v}");
            }
            out.push(']');
        }
        if with_sim {
            self.sim_fields(&mut out);
        }
        out.push('}');
        out
    }

    fn sim_fields(&self, out: &mut String) {
        out.push_str(",\"sites\":[");
        for (i, s) in self.sites.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            let _ = write!(out, "{{\"side\":{},\"kind\":", s.side);
            text(out, s.kind);
            out.push_str(",\"x\":");
            num(out, s.x);
            let _ = write!(out, ",\"zone\":{}}}", s.zone);
        }
        out.push_str("],\"freeGround\":[");
        for (i, x) in self.free_ground.iter().enumerate() {
            if i > 0 {
                out.push(',');
            }
            num(out, *x);
        }
        out.push(']');
        spots(out, "ruins", &self.ruins);
        spots(out, "guards", &self.guards);
    }
}
