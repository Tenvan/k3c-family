//! Port von `native/k3c-game/hud/paint.go`: Zeichnen mit der Plattform-Schicht. Flächen sind einfarbige Rechtecke,
//! Scheibe, Ring und Dreieck kommen aus weißen Vorlagen (eingefärbt über die Tönung), Text über `art::Font` mit schwarzer
//! Kontur. Ohne Schriftdatei zeichnet `art::Font` die Pixel-Schrift (fest, weiß); Maße kommen immer aus `Font::measure`
//! und werden je Element gemerkt (nur neu gemessen, wenn sich Text oder Größe ändern).
//!
//! Abweichung von Go: Statt `*ebiten.Image` zeichnet `Painter` über `&mut dyn Canvas`. Die Vorlagen entstehen beim ersten
//! Zeichen als vormultiplizierte RGBA-Pixel (geglättet, wie `props::paint`) und werden hochgeladen. `fill` zeichnet mit
//! `Canvas::rect` (ein einfarbiges Rechteck sieht wie der gestreckte Bildpunkt aus), `frame` wie Go aus vier Rechtecken
//! innen am Rand (`Canvas::stroke_rect` läge mittig auf dem Rand), `circle`, `ring_at` und das Dreieck mit den Vorlagen.

use k3c_platform::{Affine, Canvas, Rect as PRect, TexId};

use super::fonts::Rgba;
use super::layout::Rect;
use crate::art::{Font, OUTLINE8, OutlineCache};

/// DISC_PX: Kantenlänge der Vorlagen für Scheibe, Ring und Dreieck.
pub const DISC_PX: u32 = 64;

/// Z_BOX und Z_CONTENT: Ebenen des HUD in der Plattform-Schicht. Die Plattform ordnet je Ebene nach Aufruf (B-1058),
/// wie Go; Flächen der Boxen auf `Z_BOX`, alles Übrige (Icons, Text, Scheiben, Balken) auf `Z_CONTENT`, beide über der
/// Welt (`Layer::HUD` mal 100), hält die Trennung ausdrücklich.
pub const Z_BOX: u32 = 800;
pub const Z_CONTENT: u32 = 801;

/// MAX_LABELS: höchstens so viele Welt-Labels je Zelle werden gemessen und gezeichnet.
pub const MAX_LABELS: usize = 24;

/// MENU_WINDOW: Zahl der sichtbaren Zeilen des Skill-Menüs (Go: `menuWindow`, skilltext.go; RU23).
pub const MENU_WINDOW: usize = 5;

/// Zahl der Spieler, für die Texte gemessen werden (Go: `[2][slotCount]measure`).
const MEASURED_PLAYERS: usize = 2;

/// TextSlot benennt einen gemessenen Text je Spieler (Go: `textSlot`, die `iota`-Reihenfolge bleibt). Abweichung: keine
/// `enum`, weil Zeilen, Labels und Vorrats-Teile Bereiche sind (`menu_row(i)`, `label(i)`, `stock(i)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextSlot(pub usize);

impl TextSlot {
    pub const GOLD: TextSlot = TextSlot(0);
    pub const CLOCK: TextSlot = TextSlot(1);
    pub const FIGHT: TextSlot = TextSlot(2);
    pub const PLACE: TextSlot = TextSlot(3);
    pub const CIT: TextSlot = TextSlot(4);
    pub const BANNER: TextSlot = TextSlot(5);
    pub const BOSS: TextSlot = TextSlot(6);
    pub const EVENTS: TextSlot = TextSlot(7);
    pub const TRAVEL: TextSlot = TextSlot(8);
    pub const JOIN: TextSlot = TextSlot(9);
    pub const SKILL_BAR: TextSlot = TextSlot(10);
    pub const HINT: TextSlot = TextSlot(11);
    pub const GUIDE: TextSlot = TextSlot(12);
    /// Geschichtszeile, erste und zweite Zeile (story.go)
    pub const STORY_A: TextSlot = TextSlot(13);
    pub const STORY_B: TextSlot = TextSlot(14);
    pub const MENU_TITLE: TextSlot = TextSlot(15);
    pub const MENU_FOOTER: TextSlot = TextSlot(16);
    const MENU_ROW: usize = 17;
    const MENU_GLYPH: usize = Self::MENU_ROW + MENU_WINDOW;
    const MENU_CAP: usize = Self::MENU_GLYPH + MENU_WINDOW;
    const LABEL: usize = Self::MENU_CAP + MENU_WINDOW;
    const STOCK: usize = Self::LABEL + MAX_LABELS;
    /// Zahl aller Slots je Spieler.
    pub const COUNT: usize = Self::STOCK + super::icons::STOCK_SLOTS;

    /// Zeile i des Skill-Menüs.
    pub const fn menu_row(i: usize) -> TextSlot {
        TextSlot(Self::MENU_ROW + i)
    }
    /// Glyph-Beschriftung der Zeile i beim Zeichnen.
    pub const fn menu_glyph(i: usize) -> TextSlot {
        TextSlot(Self::MENU_GLYPH + i)
    }
    /// Glyph-Beschriftung der Zeile i beim Messen (Grundgröße).
    pub const fn menu_cap(i: usize) -> TextSlot {
        TextSlot(Self::MENU_CAP + i)
    }
    /// Welt-Label i.
    pub const fn label(i: usize) -> TextSlot {
        TextSlot(Self::LABEL + i)
    }
    /// Teil i der Vorrats-Anzeige (drawstock).
    pub const fn stock(i: usize) -> TextSlot {
        TextSlot(Self::STOCK + i)
    }
}

/// Measure ist ein gemessener Text.
#[derive(Clone, Debug, Default)]
struct Measure {
    s: String,
    size: f64,
    w: f64,
    h: f64,
    present: bool,
}

/// Stencils sind die hochgeladenen weißen Vorlagen.
#[derive(Clone, Copy, Debug)]
struct Stencils {
    disc: TexId,
    ring: TexId,
    tri: TexId,
}

/// Painter hält die Vorlagen (beim ersten Zeichnen angelegt) und die gemessenen Texte je Spieler und Element.
#[derive(Debug)]
pub struct Painter {
    st: Option<Stencils>,
    measured: Vec<Measure>,
    /// Texte mit Kontur; `sweep` je Takt in `listen`
    pub outline: OutlineCache,
}

impl Default for Painter {
    fn default() -> Painter {
        Painter {
            st: None,
            measured: vec![Measure::default(); MEASURED_PLAYERS * TextSlot::COUNT],
            outline: OutlineCache::default(),
        }
    }
}

/// straight rechnet die vormultiplizierte Farbe clr (Go: `color.RGBA`) in die gerade Farbe, die `Canvas` für Rechtecke
/// und Tönungen erwartet (Go: `ColorScale.ScaleWithColor`).
pub fn straight(clr: Rgba) -> [f32; 4] {
    let a0 = f32::from(clr[3]) / 255.0;
    if a0 <= 0.0 {
        return [0.0; 4];
    }
    let [r, g, b, _] = clr.map(|v| (f32::from(v) / 255.0 / a0).min(1.0));
    [r, g, b, a0]
}

/// coverage macht aus einem Abstand zur Kante (Pixel, innen positiv) die Deckung eines Pixels (geglättet).
fn coverage(d: f64) -> f64 {
    (d + 0.5).clamp(0.0, 1.0)
}

/// stencil_pixels rechnet eine weiße Vorlage (vormultipliziert) aus der Deckung `cov(x, y)` je Pixelmitte.
fn stencil_pixels(cov: impl Fn(f64, f64) -> f64) -> Vec<u8> {
    let n = DISC_PX as usize;
    let mut out = Vec::with_capacity(n * n * 4);
    for y in 0..n {
        for x in 0..n {
            let a = (cov(x as f64 + 0.5, y as f64 + 0.5) * 255.0).round() as u8;
            out.extend_from_slice(&[a, a, a, a]);
        }
    }
    out
}

/// ring_cov: Scheibe mit Radius 32; mit `inner` > 0 ein Ring von dort bis zum Rand (Go: `StrokeCircle(r, r, r-2, 4)`
/// = innen 28).
fn ring_cov(inner: f64) -> impl Fn(f64, f64) -> f64 {
    move |x, y| {
        let d = (x - 32.0).hypot(y - 32.0);
        coverage(32.0 - d).min(coverage(d - inner))
    }
}

/// tri_cov: Dreieck mit der Spitze oben mittig und der Basis unten über die ganze Breite.
fn tri_cov(x: f64, y: f64) -> f64 {
    let n = f64::from(DISC_PX);
    let slope = (n / 2.0).hypot(n);
    let d = ((x - n / 2.0).abs() * n - (n / 2.0) * y) / slope;
    coverage(-d).min(coverage(n - y))
}

impl Painter {
    /// stencils legt die Vorlagen einmal an.
    pub fn stencils(&mut self, dst: &mut dyn Canvas) {
        if self.st.is_some() {
            return;
        }
        let n = DISC_PX;
        self.st = Some(Stencils {
            disc: dst.texture(&stencil_pixels(ring_cov(-1.0e9)), n, n),
            ring: dst.texture(&stencil_pixels(ring_cov(28.0)), n, n),
            tri: dst.texture(&stencil_pixels(tri_cov), n, n),
        });
    }

    /// ready: Die Vorlagen stehen (Go: `paint.disc != nil`).
    pub fn ready(&self) -> bool {
        self.st.is_some()
    }

    /// measure_text misst s in Größe size mit der Schrift f, gemerkt je Spieler und Element.
    pub fn measure_text(&mut self, f: Option<&Font>, player: i64, slot: TextSlot, s: &str, size: f64) -> (f64, f64) {
        let Some(f) = f else { return (0.0, 0.0) };
        let Ok(p) = usize::try_from(player) else {
            return (0.0, 0.0);
        };
        if s.is_empty() || p >= MEASURED_PLAYERS || slot.0 >= TextSlot::COUNT {
            return (0.0, 0.0);
        }
        let m = &mut self.measured[p * TextSlot::COUNT + slot.0];
        if !m.present || m.s != s || m.size != size {
            (m.w, m.h) = f.measure(s, size);
            m.s.clear();
            m.s.push_str(s);
            m.size = size;
            m.present = true;
        }
        (m.w, m.h)
    }

    /// fill zeichnet die Fläche einer Box r in Farbe clr (Ebene `Z_BOX`, unter dem Inhalt).
    pub fn fill(&self, dst: &mut dyn Canvas, r: Rect, clr: Rgba) {
        dst.layer(Z_BOX);
        self.rect(dst, r, clr);
    }

    /// fill_top zeichnet eine Fläche im Inhalt einer Box (Ebene `Z_CONTENT`, über der Fläche der Box).
    pub fn fill_top(&self, dst: &mut dyn Canvas, r: Rect, clr: Rgba) {
        dst.layer(Z_CONTENT);
        self.rect(dst, r, clr);
    }

    fn rect(&self, dst: &mut dyn Canvas, r: Rect, clr: Rgba) {
        if r.w <= 0.0 || r.h <= 0.0 || clr[3] == 0 {
            return;
        }
        dst.rect(
            PRect::new(r.x as f32, r.y as f32, r.w as f32, r.h as f32),
            straight(clr),
        );
    }

    /// frame zeichnet vier Kanten der Breite lw innen an r.
    pub fn frame(&self, dst: &mut dyn Canvas, r: Rect, lw: f64, clr: Rgba) {
        self.fill_top(dst, Rect::new(r.x, r.y, r.w, lw), clr);
        self.fill_top(dst, Rect::new(r.x, r.y + r.h - lw, r.w, lw), clr);
        self.fill_top(dst, Rect::new(r.x, r.y, lw, r.h), clr);
        self.fill_top(dst, Rect::new(r.x + r.w - lw, r.y, lw, r.h), clr);
    }

    /// stamp zeichnet das Teilbild src der Textur tex gestreckt auf (x, y, w, h) in Farbe clr.
    pub fn stamp(&self, dst: &mut dyn Canvas, tex: TexId, src: PRect, r: Rect, clr: Rgba) {
        if r.w <= 0.0 || r.h <= 0.0 || clr[3] == 0 {
            return;
        }
        dst.layer(Z_CONTENT);
        let mut g = Affine::IDENTITY;
        g.scale((r.w as f32) / src.w, (r.h as f32) / src.h)
            .translate(r.x as f32, r.y as f32);
        dst.sprite_tf(tex, src, g, straight(clr));
    }

    fn stencil(&self, dst: &mut dyn Canvas, pick: fn(&Stencils) -> TexId, r: Rect, clr: Rgba) {
        if let Some(st) = &self.st {
            let n = DISC_PX as f32;
            self.stamp(dst, pick(st), PRect::new(0.0, 0.0, n, n), r, clr);
        }
    }

    /// circle zeichnet eine Scheibe um (x, y) mit Radius r.
    pub fn circle(&self, dst: &mut dyn Canvas, x: f64, y: f64, r: f64, clr: Rgba) {
        self.stencil(dst, |s| s.disc, Rect::new(x - r, y - r, 2.0 * r, 2.0 * r), clr);
    }

    /// ring_at zeichnet einen Ring um (x, y) mit Außenradius r.
    pub fn ring_at(&self, dst: &mut dyn Canvas, x: f64, y: f64, r: f64, clr: Rgba) {
        self.stencil(dst, |s| s.ring, Rect::new(x - r, y - r, 2.0 * r, 2.0 * r), clr);
    }

    /// triangle zeichnet ein Dreieck mit Spitze oben in die Fläche r.
    pub fn triangle(&self, dst: &mut dyn Canvas, r: Rect, clr: Rgba) {
        self.stencil(dst, |s| s.tri, r, clr);
    }

    /// text zeichnet s mit der linken oberen Ecke bei (x, y): echte Schrift mit schwarzer Kontur, Pixel-Schrift ohne.
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn text(&mut self, dst: &mut dyn Canvas, f: Option<&mut Font>, s: &str, x: f64, y: f64, size: f64, clr: Rgba) {
        dst.layer(Z_CONTENT);
        self.outline.draw(dst, f, s, x, y, size, clr, &OUTLINE8, 1.0);
    }

    /// label zeichnet s ohne Kontur mit der linken oberen Ecke bei (x, y): Beschriftung auf heller Fläche (Tastenkappe).
    #[allow(clippy::too_many_arguments)] // Signatur wie Go
    pub fn label(&mut self, dst: &mut dyn Canvas, f: Option<&mut Font>, s: &str, x: f64, y: f64, size: f64, clr: Rgba) {
        dst.layer(Z_CONTENT);
        self.outline.draw(dst, f, s, x, y, size, clr, &[], 1.0);
    }
}
