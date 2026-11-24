//! Vorschau des markierten Spielstands in der Spielstand-Auswahl (GS3.8, B-966): ein kleines Diorama der Seekarte: die
//! Inseln der Welt als Kacheln nach Etappen von links nach rechts, das Schiff an der aktuellen Insel, auf den
//! abgeschlossenen die Trophäe samt Fahne (gezeichnet, bis die Assets aus GS3.2 da sind). Darunter die Abschlusszeile
//! (`done`) und klein „<Tage> Tage · <n> Monarchen“ der zuletzt abgeschlossenen Insel. Ohne Zeiger, ohne Wahl: Auf/Ab der
//! Liste wechselt die Vorschau sofort. Ein nicht ladbarer Stand (keine Route) hat keine; ohne Abschluss keine Trophäen.
//! Die Seite baut die Vorschau nur bei neuer Version oder Sprache (`SavesScreen::fill`), gezeichnet wird ohne Rechnung.

use k3c_platform::Canvas;

use super::draw::Painter;
use super::layout::{COL_TEXT, Geom, Rgba, fade, rgb};
use super::texts::{TextKey, tr1};
use crate::view::session::SaveInfo;
use crate::view::trophy::{Part, parts};

const COL_BOARD: Rgba = rgb(0x0c1024);
const COL_DONE: Rgba = rgb(0x1f3d2b);
const COL_OPEN: Rgba = rgb(0x2b3a67);
const COL_SHIP: Rgba = rgb(0x8f6d4a);
const COL_SAIL: Rgba = rgb(0xffffff);
/// Anteil der Fenstergröße, der rechts und unten frei bleibt (TV-Overscan).
const SAFE: f64 = 0.05;

/// Eine Kachel der Vorschau: abgeschlossen (Trophäe) und/oder Liegeplatz des Schiffs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PreviewNode {
    pub done: bool,
    pub ship: bool,
}

/// Die Vorschau eines Stands: Spalten je Etappe (aufsteigend), Zeilen darunter; `on` = es gibt eine Vorschau.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Preview {
    pub on: bool,
    pub cols: Vec<Vec<PreviewNode>>,
    /// `done`-Zeile der zuletzt abgeschlossenen Insel; leer = keine
    pub line: String,
    /// „<Tage> Tage · <n> Monarchen“ derselben Insel; leer ohne Chronik-Eintrag
    pub numbers: String,
}

/// preview_of baut die Vorschau des Stands `e`; ein nicht ladbarer Stand oder einer ohne Route hat keine.
pub fn preview_of(lang: &str, e: &SaveInfo) -> Preview {
    if !e.loadable || e.route.is_empty() {
        return Preview::default();
    }
    let mut etappen: Vec<i64> = e.route.iter().map(|i| i.etappe).collect();
    etappen.sort_unstable();
    etappen.dedup();
    let cols = etappen
        .iter()
        .map(|et| {
            e.route
                .iter()
                .filter(|i| i.etappe == *et)
                .map(|i| PreviewNode {
                    done: e.completed.contains(&i.id),
                    ship: !e.island.is_empty() && e.island == i.id,
                })
                .collect()
        })
        .collect();
    let mut p = Preview {
        on: true,
        cols,
        ..Preview::default()
    };
    if let Some(last) = e.completed.last() {
        p.line = e
            .route
            .iter()
            .find(|i| i.id == *last)
            .map(|i| i.done.clone())
            .unwrap_or_default();
        p.numbers = e
            .chronicle
            .iter()
            .find(|c| c.island == *last)
            .map(|c| {
                let part = |one, many, n: i64| tr1(lang, if n == 1 { one } else { many }, &n.to_string());
                format!(
                    "{} · {}",
                    part(TextKey::SavesDay, TextKey::SavesDays, c.days),
                    part(TextKey::SavesMonarch, TextKey::SavesMonarchs, c.monarchs)
                )
            })
            .unwrap_or_default();
    }
    p
}

impl Painter {
    /// Zeichnet die Vorschau rechts neben die Liste (in den Rand der Karten). Rechts und unten bleiben mindestens 5 % der
    /// Fenstergröße frei (TV-Overscan); ist dort zu wenig Platz, entfällt sie.
    #[allow(clippy::cast_precision_loss)]
    pub fn preview(&mut self, dst: &mut dyn Canvas, g: &Geom, p: &Preview) {
        let (ww, wh) = (g.win.dx() as f64, g.win.dy() as f64);
        let x = g.row_x + g.row_w + 0.02 * ww;
        let w = g.win.max.x as f64 - SAFE * ww - x;
        let bh = 0.3 * wh;
        let limit = g.win.max.y as f64 - SAFE * wh;
        if !p.on || w < 120.0 * g.k || g.row_top + bh > limit {
            return;
        }
        let size = g.small_px;
        let (line, numbers) = (self.wrap(&p.line, w, size, 2), self.wrap(&p.numbers, w, size, 1));
        let lines = (line.len() + numbers.len()) as f64;
        if g.row_top + bh + 0.012 * wh + lines * 1.25 * size > limit {
            return; // Text käme in den Overscan: keine Vorschau
        }
        self.rect(dst, x, g.row_top, w, bh, COL_BOARD);
        let n = p.cols.len().max(1) as f64;
        let rows = p.cols.iter().map(Vec::len).max().unwrap_or(1).max(1) as f64;
        let d = (w / n * 0.62).min(bh / rows * 0.7);
        for (i, col) in p.cols.iter().enumerate() {
            let cx = x + (i as f64 + 0.5) * w / n;
            for (j, nd) in col.iter().enumerate() {
                let cy = g.row_top + (j as f64 + 0.5) * bh / col.len() as f64;
                self.tile(dst, cx, cy, d, nd);
            }
        }
        let (cx, mut y) = (x + w / 2.0, g.row_top + bh + 0.012 * wh);
        for l in line.iter().chain(&numbers) {
            self.centered(dst, l, cx, y, size, COL_TEXT);
            y += 1.25 * size;
        }
    }

    /// wrap legt `s` in höchstens `max_lines` Zeilen der Breite `w` bei Schriftgröße `size` (nie kleiner): bricht an
    /// Leerzeichen um, kürzt, was dann noch zu breit ist, mit „...“.
    pub fn wrap(&self, s: &str, w: f64, size: f64, max_lines: usize) -> Vec<String> {
        let mut out = Vec::new();
        let mut rest = s.trim();
        while !rest.is_empty() && out.len() < max_lines {
            if self.measure(rest, size).0 <= w {
                out.push(rest.to_string());
                break;
            }
            let last = out.len() + 1 == max_lines;
            let cut = if last { None } else { self.break_at(rest, w, size) };
            let (head, tail) = match cut {
                Some(i) => (&rest[..i], rest[i..].trim_start()),
                None => (rest, ""),
            };
            out.push(if tail.is_empty() {
                self.ellipsis(head, w, size)
            } else {
                head.to_string()
            });
            rest = tail;
        }
        out
    }

    /// Das letzte Leerzeichen, vor dem der Anfang noch in `w` passt.
    fn break_at(&self, s: &str, w: f64, size: f64) -> Option<usize> {
        s.match_indices(' ')
            .map(|(i, _)| i)
            .rfind(|i| *i > 0 && self.measure(&s[..*i], size).0 <= w)
    }

    /// `s`, hinten so gekürzt und mit „...“ versehen, dass es in `w` passt (passt es schon, unverändert).
    fn ellipsis(&self, s: &str, w: f64, size: f64) -> String {
        if self.measure(s, size).0 <= w {
            return s.to_string();
        }
        let mut t: String = s.to_string();
        while t.pop().is_some() {
            let c = format!("{}...", t.trim_end());
            if self.measure(&c, size).0 <= w {
                return c;
            }
        }
        "...".to_string()
    }

    /// Eine Kachel mit Mitte (cx, cy) und Kantenlänge d: Insel, Trophäe samt Fahne (abgeschlossen), Schiff links davor.
    fn tile(&mut self, dst: &mut dyn Canvas, cx: f64, cy: f64, d: f64, nd: &PreviewNode) {
        let clr = if nd.done { COL_DONE } else { COL_OPEN };
        self.rect(dst, cx - d / 2.0, cy - d / 2.0, d, d, clr);
        if nd.done {
            let me = &*self;
            parts(cx - 0.12 * d, cy + 0.45 * d, 0.8 * d, true, &mut |p| match p {
                Part::Rect(x, y, w, h, c) => me.rect(dst, x, y, w, h, rgb(c)),
                Part::Disc(x, y, r, c) => me.rect(dst, x - r, y - r, 2.0 * r, 2.0 * r, rgb(c)),
                Part::Ring(x, y, r, c) => me.rect(dst, x - r, y - r, 2.0 * r, 2.0 * r, fade(rgb(c), 0.6)),
            });
        }
        if nd.ship {
            let (sx, sy) = (cx - 0.62 * d, cy + 0.5 * d);
            self.rect(dst, sx - 0.3 * d, sy - 0.1 * d, 0.6 * d, 0.1 * d, COL_SHIP); // Rumpf
            self.rect(dst, sx - 0.04 * d, sy - 0.55 * d, 0.08 * d, 0.45 * d, COL_SHIP); // Mast
            self.rect(dst, sx + 0.04 * d, sy - 0.5 * d, 0.22 * d, 0.34 * d, COL_SAIL); // Segel
        }
    }
}
