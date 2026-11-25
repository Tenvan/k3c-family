//! Tests zu `report_html.rs` und `base64.rs`: Hilfen für den Vergleich mit Go (`rustref_tests.rs`), die Prüfung auf externe
//! Verweise (Go: Regex aus `TestHTMLEigenstaendig`, hier als Zeichenvergleich) und das Zuschneiden der Grafiken.

use std::collections::BTreeMap;

use crate::report_html::{Rgba, decode_png, encode_png, go_json, report_html, trimmed};

/// Eine zerlegte Seite: Text ohne die beiden JSON-Blöcke, Daten als JSON-Baum, Grafiken je Schlüssel als Daten-URI.
pub(crate) struct Zerlegt {
    pub rest: String,
    pub data: serde_json::Value,
    pub assets: BTreeMap<String, String>,
}

/// Schneidet den Inhalt des Skript-Blocks mit dieser `id` heraus.
fn block<'a>(page: &'a str, id: &str) -> (&'a str, &'a str, &'a str) {
    let open = format!("id=\"{id}\">");
    let i = page.find(&open).expect("Block") + open.len();
    let j = i + page[i..].find("</script>").expect("Ende");
    (&page[..i], &page[i..j], &page[j..])
}

pub(crate) fn zerlegt(page: &str) -> Zerlegt {
    let (a, data, rest) = block(page, "report-data");
    let (b, assets, c) = block(rest, "report-assets");
    Zerlegt {
        rest: format!("{a}{b}{c}"),
        data: serde_json::from_str(data).expect("Daten"),
        assets: serde_json::from_str(assets).expect("Grafiken"),
    }
}

/// Base64 (Standard-Alphabet mit `=`) zurück in Bytes, nur für die Tests.
fn unbase64(s: &str) -> Vec<u8> {
    let val = |c: u8| match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'+' => 62,
        _ => 63,
    };
    let mut out = Vec::new();
    for q in s.as_bytes().chunks(4) {
        let n = q
            .iter()
            .take_while(|&&c| c != b'=')
            .fold(0u32, |n, &c| (n << 6) | u32::from(val(c)));
        let k = q.iter().take_while(|&&c| c != b'=').count();
        let n = n << (6 * (4 - k));
        out.extend_from_slice(&n.to_be_bytes()[1..k]);
    }
    out
}

/// Die Pixel einer Daten-URI `data:image/png;base64,…`.
pub(crate) fn pixels(uri: &str) -> Rgba {
    let raw = uri.strip_prefix("data:image/png;base64,").expect("Daten-URI");
    decode_png(&unbase64(raw)).expect("PNG")
}

/// Stimmt die Zeichenfolge ab `at` mit `(https?:)?//` überein? Liefert die Länge, sonst `None`.
fn scheme_slashes(s: &str, at: usize) -> Option<usize> {
    let rest = &s[at..];
    for p in ["https:", "http:", ""] {
        if rest.starts_with(p) && rest[p.len()..].starts_with("//") {
            return Some(p.len() + 2);
        }
    }
    None
}

fn skip_ws(s: &str, at: usize) -> usize {
    at + s[at..].len() - s[at..].trim_start().len()
}

/// Nach `[ws]["']?[ws]` folgt `(https?:)?//`? (Go: Teil der Regex; `s` ist klein geschrieben.)
fn quoted_url(s: &str, at: usize) -> bool {
    let mut i = skip_ws(s, at);
    if s[i..].starts_with(['"', '\'']) {
        i += 1;
    }
    i = skip_ws(s, i);
    scheme_slashes(s, i).is_some()
}

/// Der erste externe Verweis der Seite: `src=`/`href=` mit `//`, `url(//`, `@import`, `<link` (Groß-/Kleinschreibung egal).
pub(crate) fn external_ref(html: &str) -> Option<String> {
    let s = html.to_lowercase();
    for i in 0..s.len() {
        if !s.is_char_boundary(i) {
            continue;
        }
        let rest = &s[i..];
        let hit = if rest.starts_with("src") || rest.starts_with("href") {
            let n = if rest.starts_with("src") { 3 } else { 4 };
            let j = skip_ws(&s, i + n);
            s[j..].starts_with('=') && quoted_url(&s, j + 1)
        } else if rest.starts_with("url(") {
            quoted_url(&s, i + 4)
        } else {
            rest.starts_with("@import")
                || (rest.starts_with("<link") && !rest[5..].starts_with(|c: char| c.is_alphanumeric() || c == '_'))
        };
        if hit {
            return Some(rest.chars().take(30).collect());
        }
    }
    None
}

fn img(w: u32, h: u32, set: &[(u32, u32, [u8; 4])]) -> Rgba {
    let mut px = vec![0; (w * h * 4) as usize];
    for &(x, y, c) in set {
        let i = ((y * w + x) * 4) as usize;
        px[i..i + 4].copy_from_slice(&c);
    }
    Rgba { w, h, px }
}

#[test]
fn externe_verweise_erkannt() {
    for bad in [
        r#"<img src="https://x.de/a.png">"#,
        "<IMG SRC = '//x.de/a.png'>",
        r#"<a href=//x.de>"#,
        "a { background: url( 'http://x.de/a.png') }",
        "a { background: url(//x.de/a.png) }",
        "@import 'x.css';",
        "<LINK rel=stylesheet>",
    ] {
        assert!(external_ref(bad).is_some(), "{bad}");
    }
    for good in [
        r#"<img src="data:image/png;base64,AAAA">"#,
        "<a href=\"#x\">",
        "url(\"data:image/png;base64,AAAA\")",
        "<linked>",
        "x.src = 'a/b'",
    ] {
        assert_eq!(external_ref(good), None, "{good}");
    }
}

/// Der Rand wird abgeschnitten, ganz transparent bleibt das Rechteck, Bild und Zuschnitt behalten die Pixel.
#[test]
fn zuschnitt() {
    let a = img(10, 8, &[(2, 1, [200, 10, 10, 255]), (7, 5, [10, 200, 10, 128])]);
    let t = trimmed(&a, (0, 0, 10, 8));
    assert_eq!((t.w, t.h), (6, 5));
    assert_eq!(&t.px[..4], &[200, 10, 10, 255]);
    assert_eq!(&t.px[((4 * 6 + 5) * 4) as usize..][..4], &[10, 200, 10, 128]);
    let leer = trimmed(&a, (0, 6, 3, 2));
    assert_eq!((leer.w, leer.h), (3, 2), "ganz transparent: Rechteck unverändert");
    assert!(leer.px.iter().all(|&b| b == 0));
}

#[test]
fn png_hin_und_zurueck() {
    let a = img(3, 2, &[(0, 0, [1, 2, 3, 4]), (2, 1, [255, 0, 128, 255])]);
    let b = decode_png(&encode_png(&a).expect("PNG")).expect("lesen");
    assert_eq!((b.w, b.h, b.px), (a.w, a.h, a.px));
    assert!(encode_png(&img(0, 0, &[])).is_none(), "0x0 wie Go: Fehler");
    assert!(decode_png(b"kein png").is_none());
}

#[test]
fn json_wie_go() {
    let b = '\\';
    let want = format!("\"a{b}u003cb{b}u003e{b}u0026c{b}u2028\"");
    assert_eq!(go_json(&"a<b>&c\u{2028}").expect("JSON"), want);
}

#[test]
fn base64_zurueck() {
    let raw: Vec<u8> = (0..=255).collect();
    for n in [0, 1, 2, 3, 4, 100, 256] {
        assert_eq!(unbase64(&crate::base64::encode(&raw[..n])), raw[..n], "{n}");
    }
}

/// Titel ohne `<`, `>`, `&`; Platzhalter bleiben nicht stehen, auch wenn Daten sie enthalten (kein zweiter Durchlauf).
#[test]
fn titel_und_platzhalter() {
    let inn = crate::insights::Insights {
        title: "a<b>&c __DATA__".into(),
        ..crate::insights::Insights::default()
    };
    let page = String::from_utf8(report_html(&inn, "gibt-es-nicht").expect("Bericht")).expect("UTF-8");
    assert!(page.contains("<title>K3C · Balancing-Bericht abc __DATA__</title>"));
    assert!(page.contains(r#"id="report-assets">{}</script>"#));
    assert!(!page.contains("__CSS__") && !page.contains("__JS__") && !page.contains("__ASSETS__"));
}
