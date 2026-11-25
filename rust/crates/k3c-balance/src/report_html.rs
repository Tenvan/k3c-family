//! HTML-Bericht (Port von `engine/balance/report_html.go`, B-632): eine Datei ohne externe Ressourcen. Vorlage, Stil und
//! Skripte liegen als Kopie unter `report/` (`include_str!`), die Auswertung kommt als JSON hinein, Spielgrafiken aus
//! `public/` als Daten-URIs (erster Frame, ohne transparenten Rand).
//!
//! Abweichung von Go: Der PNG-Kodierer von Rust schreibt andere Bytes als `image/png`; gleich sind die Pixel, nicht die Datei.

use std::collections::BTreeMap;
use std::path::Path;

use png::{BitDepth, ColorType, Decoder, Encoder, Transformations};

use crate::base64;
use crate::fehler::Fehler;
use crate::insights::Insights;

const PAGE: &str = include_str!("../report/report.html");
const CSS: &str = include_str!("../report/report.css");
/// In Lade-Reihenfolge: core definiert die Hilfen, next startet.
const SCRIPTS: [&str; 4] = [
    include_str!("../report/report-core.js"),
    include_str!("../report/report-charts.js"),
    include_str!("../report/report-view.js"),
    include_str!("../report/report-next.js"),
];

/// Eine Grafik des Berichts: Schlüssel, Datei unter `public/` und Größe eines Frames (0 = ganzes Bild, sonst der erste
/// Frame eines waagrechten Streifens bzw. das Feld oben links).
struct Asset {
    key: &'static str,
    path: &'static str,
    frame: (u32, u32),
}

const ASSETS: [Asset; 10] = [
    Asset {
        key: "castle",
        path: "grafik/gebaeude-paper/burg/stand.png",
        frame: (0, 0),
    },
    Asset {
        key: "wall",
        path: "grafik/gebaeude-paper/mauer/stand.png",
        frame: (0, 0),
    },
    Asset {
        key: "tower",
        path: "grafik/gebaeude-paper/turm/stand.png",
        frame: (0, 0),
    },
    Asset {
        key: "fire",
        path: "grafik/gebaeude-paper/lagerfeuer/stand.png",
        frame: (0, 0),
    },
    Asset {
        key: "axe",
        path: "grafik/objekte/schild-axt/idle.png",
        frame: (36, 51),
    },
    Asset {
        key: "pick",
        path: "grafik/objekte/schild-spitzhacke/idle.png",
        frame: (36, 51),
    },
    Asset {
        key: "ore",
        path: "grafik/objekte/erzbrocken/idle.png",
        frame: (43, 28),
    },
    Asset {
        key: "coin",
        path: "grafik/icons/muenze/idle.png",
        frame: (26, 26),
    },
    Asset {
        key: "enemy",
        path: "grafik/gegner/skelett/idle.png",
        frame: (159, 192),
    },
    Asset {
        key: "enemy2",
        path: "grafik/gegner/goblin/idle.png",
        frame: (143, 192),
    },
];

/// Ein Bild als RGBA-Zeilen (nicht vormultipliziert, 8 Bit).
pub(crate) struct Rgba {
    pub w: u32,
    pub h: u32,
    pub px: Vec<u8>,
}

/// JSON wie Gos `json.Marshal`: `<`, `>`, `&` und U+2028/U+2029 maskiert, damit kein `</script>` im Skript-Block steht.
pub(crate) fn go_json(v: &impl serde::Serialize) -> Result<String, Fehler> {
    let s = serde_json::to_string(v).map_err(|e| Fehler::Daten(format!("balance: {e}")))?;
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            _ => out.push(c),
        }
    }
    Ok(out)
}

/// Schreibt den Bericht; `root` ist der Checkout (Grafiken aus `root/public/`, fehlende zeigt der Bericht als Platzhalter;
/// Go: `Insights.HTML`).
pub fn report_html(inn: &Insights, root: &str) -> Result<Vec<u8>, Fehler> {
    let js: String = SCRIPTS.iter().map(|s| format!("{s}\n")).collect();
    let data = go_json(inn)?;
    let assets = go_json(&load_assets(Path::new(root)))?;
    let title: String = inn.title.chars().filter(|c| !matches!(c, '<' | '>' | '&')).collect();
    let subs = [
        ("__CSS__", CSS),
        ("__JS__", js.as_str()),
        ("__DATA__", data.as_str()),
        ("__ASSETS__", assets.as_str()),
        ("__TITLE__", title.as_str()),
    ];
    Ok(replace_all(PAGE, &subs).into_bytes())
}

/// Ersetzt die Platzhalter in einem Durchlauf; eingesetzter Text wird nicht erneut durchsucht (Go: `strings.Replacer`).
fn replace_all(page: &str, subs: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(page.len() + 4096);
    let mut rest = page;
    while let Some((at, (key, val))) = subs
        .iter()
        .filter_map(|s| rest.find(s.0).map(|i| (i, *s)))
        .min_by_key(|(i, _)| *i)
    {
        out.push_str(&rest[..at]);
        out.push_str(val);
        rest = &rest[at + key.len()..];
    }
    out.push_str(rest);
    out
}

/// Liest die Grafiken; unlesbare fehlen in der Liste.
fn load_assets(root: &Path) -> BTreeMap<&'static str, String> {
    ASSETS
        .iter()
        .filter_map(|a| data_uri(root, a).map(|u| (a.key, u)))
        .collect()
}

fn data_uri(root: &Path, a: &Asset) -> Option<String> {
    let file = std::fs::read(root.join("public").join(a.path)).ok()?;
    let img = decode_png(&file)?;
    let (mut w, mut h) = (img.w, img.h);
    if a.frame.0 > 0 {
        w = w.min(a.frame.0);
        h = h.min(a.frame.1);
    }
    let out = trimmed(&img, (0, 0, w, h));
    Some(format!("data:image/png;base64,{}", base64::encode(&encode_png(&out)?)))
}

/// Liest ein PNG als RGBA (8 Bit), Paletten und Graustufen aufgeweitet.
pub(crate) fn decode_png(file: &[u8]) -> Option<Rgba> {
    let mut dec = Decoder::new(std::io::Cursor::new(file));
    dec.set_transformations(Transformations::EXPAND | Transformations::STRIP_16);
    let mut r = dec.read_info().ok()?;
    let mut buf = vec![0; r.output_buffer_size()?];
    let info = r.next_frame(&mut buf).ok()?;
    let (w, h) = (info.width, info.height);
    let src = &buf[..info.buffer_size()];
    let mut px = Vec::with_capacity(w as usize * h as usize * 4);
    match info.color_type {
        ColorType::Rgba => px.extend_from_slice(src),
        ColorType::Rgb => src
            .as_chunks::<3>()
            .0
            .iter()
            .for_each(|[r, g, b]| px.extend_from_slice(&[*r, *g, *b, 255])),
        ColorType::GrayscaleAlpha => src
            .as_chunks::<2>()
            .0
            .iter()
            .for_each(|[g, a]| px.extend_from_slice(&[*g, *g, *g, *a])),
        ColorType::Grayscale => src.iter().for_each(|&g| px.extend_from_slice(&[g, g, g, 255])),
        ColorType::Indexed => return None,
    }
    (info.bit_depth == BitDepth::Eight).then_some(Rgba { w, h, px })
}

/// Schneidet `rect` (x, y, Breite, Höhe) auf die nicht transparenten Pixel zu (ganz transparent: `rect` unverändert;
/// Go: `trimmed`).
pub(crate) fn trimmed(img: &Rgba, rect: (u32, u32, u32, u32)) -> Rgba {
    let (rx, ry, rw, rh) = rect;
    let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0, 0);
    for y in ry..ry + rh {
        for x in rx..rx + rw {
            if img.px[(y * img.w + x) as usize * 4 + 3] > 0 {
                (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x + 1), y1.max(y + 1));
            }
        }
    }
    let (bx, by, bw, bh) = if x0 == u32::MAX {
        rect
    } else {
        (x0, y0, x1 - x0, y1 - y0)
    };
    let mut px = Vec::with_capacity(bw as usize * bh as usize * 4);
    for y in by..by + bh {
        let i = (y * img.w + bx) as usize * 4;
        px.extend_from_slice(&img.px[i..i + bw as usize * 4]);
    }
    Rgba { w: bw, h: bh, px }
}

pub(crate) fn encode_png(img: &Rgba) -> Option<Vec<u8>> {
    if img.w == 0 || img.h == 0 {
        return None; // Go: png.Encode lehnt 0x0 ab, das Bild fehlt dann im Bericht
    }
    let mut buf = Vec::new();
    let mut enc = Encoder::new(&mut buf, img.w, img.h);
    enc.set_color(ColorType::Rgba);
    enc.set_depth(BitDepth::Eight);
    let mut w = enc.write_header().ok()?;
    w.write_image_data(&img.px).ok()?;
    w.finish().ok()?;
    Some(buf)
}
