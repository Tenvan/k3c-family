//! Port von `native/k3c-game/hud/layout.go`.
//!
//! Lage der HUD-Elemente je Zelle (Spiegel von src/scenes/hud/hudLayout.ts, B-337): jedes Element hat einen Anker in
//! der Zelle und einen Rang; `layout` legt sie ohne Überschneidung untereinander und mit den Freiflächen ab. Was nicht
//! passt, fällt nach Rang weg, Rang 0 (Pflicht) nie. Reine Rechnung ohne Zeichnen und ohne Allokation.
//!
//! Abweichung vom Browser: Es gibt keine Leiste über die ganze Breite; die Zeilen der Leiste (Uhr, Kampf, Vorrat,
//! Spielerkarte, Meldung) stehen oben in jeder Zelle, jede Zelle zeigt die Werte ihrer eigenen Stufe.

/// Rect ist ein Rechteck in Fenster-Pixeln (Go: `rect`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub const fn new(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect { x, y, w, h }
    }

    pub fn overlaps(&self, b: Rect) -> bool {
        let a = self;
        a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h
    }

    pub fn inside(&self, b: Rect) -> bool {
        let a = self;
        a.x >= b.x && a.y >= b.y && a.x + a.w <= b.x + b.w && a.y + a.h <= b.y + b.h
    }
}

/// Align ist die Bündigkeit eines Ankers (waagrecht: Start = links, senkrecht: Start = oben).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
}

/// ItemId benennt ein HUD-Element einer Zelle; die Reihenfolge gilt bei gleichem Rang (Index in `Items`).
pub type ItemId = usize;
pub const ITEM_PLAYER: ItemId = 0;
pub const ITEM_BANNER: ItemId = 1;
pub const ITEM_SKILLS: ItemId = 2;
pub const ITEM_CLOCK: ItemId = 3;
pub const ITEM_FIGHT: ItemId = 4;
pub const ITEM_SHARED: ItemId = 5;
pub const ITEM_PLACE: ItemId = 6;
pub const ITEM_CITIZENS: ItemId = 7;
pub const ITEM_STORY: ItemId = 8;
pub const ITEM_BOSS: ItemId = 9;
pub const ITEM_EVENTS: ItemId = 10;
pub const ITEM_TRAVEL: ItemId = 11;
pub const ITEM_JOIN: ItemId = 12;
pub const ITEM_RADAR: ItemId = 13;
pub const ITEM_COUNT: usize = 14;

// Ränge laut 🧑 (2026-10-10): Pflicht Spielerkarte, Meldung, Skill-Leiste; dann die Zeilen der Leiste, Reise,
// Beitritt, Radar.
pub const RANK_MUST: i64 = 0;
pub const RANK_SHARED: i64 = 1;
pub const RANK_TRAVEL: i64 = 2;
pub const RANK_JOIN: i64 = 3;
pub const RANK_RADAR: i64 = 4;
pub const RANK_MAX: i64 = RANK_RADAR;

/// GAP ist der Abstand zwischen Elementen und zu Freiflächen (px bei 1080, mit dem Maßstab skaliert).
pub const GAP: f64 = 8.0;

/// Anchor ist Anker und Rang eines Elements (Abstände in px bei 1080).
#[derive(Clone, Copy, Debug)]
pub struct Anchor {
    pub h: Align,
    pub v: Align,
    pub dx: f64,
    pub dy: f64,
    pub rank: i64,
}

const fn anchor(h: Align, v: Align, dx: f64, dy: f64, rank: i64) -> Anchor {
    Anchor { h, v, dx, dy, rank }
}

/// ROW_H ist der Zeilenabstand der oberen Zeilen (Browser: 42 px je Zeile der Leiste).
pub const ROW_H: f64 = 42.0;

/// ANCHORS der Elemente: drei Zeilen oben wie die Leiste des Browsers (Zeile 0 Spielerkarte, Uhr, Kampf; Zeile 1
/// Vorrat, Meldung, Ort; Zeile 2 Hub und Bürger, mittig die Geschichtszeile), darunter Boss und Events, unten
/// Skill-Leiste, Reise, Beitritt und Radar (B-915: Unterkante im Fluss).
pub const ANCHORS: [Anchor; ITEM_COUNT] = {
    use Align::{Center, End, Start};
    let mut a = [anchor(Start, Start, 0.0, 0.0, 0); ITEM_COUNT];
    a[ITEM_PLAYER] = anchor(Start, Start, 24.0, 4.0, RANK_MUST);
    a[ITEM_BANNER] = anchor(Center, Start, 0.0, 4.0 + ROW_H, RANK_MUST);
    a[ITEM_SKILLS] = anchor(Start, End, 24.0, 24.0, RANK_MUST);
    a[ITEM_CLOCK] = anchor(Center, Start, 0.0, 4.0, RANK_SHARED);
    a[ITEM_FIGHT] = anchor(End, Start, 24.0, 4.0, RANK_SHARED);
    a[ITEM_SHARED] = anchor(Start, Start, 24.0, 4.0 + ROW_H, RANK_SHARED);
    a[ITEM_PLACE] = anchor(End, Start, 24.0, 4.0 + ROW_H, RANK_SHARED);
    a[ITEM_CITIZENS] = anchor(End, Start, 24.0, 4.0 + 2.0 * ROW_H, RANK_SHARED);
    // Geschichtszeile (B-974): vor Boss und Events
    a[ITEM_STORY] = anchor(Center, Start, 0.0, 4.0 + 2.0 * ROW_H, RANK_SHARED);
    a[ITEM_BOSS] = anchor(Center, Start, 0.0, 4.0 + 3.0 * ROW_H, RANK_SHARED);
    a[ITEM_EVENTS] = anchor(Center, Start, 0.0, 4.0 + 4.0 * ROW_H, RANK_SHARED);
    a[ITEM_TRAVEL] = anchor(Center, End, 0.0, 150.0, RANK_TRAVEL);
    a[ITEM_JOIN] = anchor(Center, End, 0.0, 85.0, RANK_JOIN);
    a[ITEM_RADAR] = anchor(End, End, 24.0, 24.0, RANK_RADAR);
    a
};

/// Size ist die Größe eines Elements in px; `w == 0` heißt: nicht gezeigt (Go: Eintrag von `items`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

/// Items sind die Größen der Elemente einer Zelle in px.
pub type Items = [Size; ITEM_COUNT];

/// Placed ist das Ergebnis: Lage je Element, `ok` false = ausgeblendet oder nicht gezeigt.
#[derive(Clone, Copy, Debug, Default)]
pub struct Placed {
    pub at: [Rect; ITEM_COUNT],
    pub ok: [bool; ITEM_COUNT],
}

/// MAX_FREE ist die Höchstzahl der Freiflächen.
pub const MAX_FREE: usize = 4;

/// anchor_rect ist die Wunschlage des Elements id mit Größe (w, h) in der Fläche area beim Maßstab k.
pub fn anchor_rect(id: ItemId, area: Rect, w: f64, h: f64, k: f64) -> Rect {
    let a = ANCHORS[id];
    Rect {
        x: pos(a.h, area.x, area.w, w, a.dx * k),
        y: pos(a.v, area.y, area.h, h, a.dy * k),
        w,
        h,
    }
}

fn pos(al: Align, start: f64, size: f64, w: f64, d: f64) -> f64 {
    match al {
        Align::End => start + size - w - d,
        Align::Center => start + (size - w) / 2.0 + d,
        Align::Start => start + d,
    }
}

/// layout legt die Elemente in area ab: höherer Rang (kleinere Zahl) zuerst, bei Gleichstand nach ItemId; free sind
/// Flächen, die frei bleiben (höchstens MAX_FREE). Ein Pflicht-Element, das nirgends passt, steht an seinem Anker.
pub fn layout(items: &Items, area: Rect, free: &[Rect], k: f64) -> Placed {
    let mut out = Placed::default();
    let mut taken = [Rect::default(); ITEM_COUNT + MAX_FREE];
    let mut n = free.len().min(MAX_FREE);
    taken[..n].copy_from_slice(&free[..n]);
    for rank in 0..=RANK_MAX {
        for id in 0..ITEM_COUNT {
            if ANCHORS[id].rank != rank || items[id].w <= 0.0 {
                continue;
            }
            let mut found = place(id, area, items[id].w, items[id].h, k, &taken[..n]);
            if found.is_none() && rank == RANK_MUST {
                found = Some(anchor_rect(id, area, items[id].w, items[id].h, k));
            }
            if let Some(r) = found {
                out.at[id] = r;
                out.ok[id] = true;
                taken[n] = r;
                n += 1;
            }
        }
    }
    out
}

/// place sucht die nächste freie Lage zur Wunschlage: Kandidaten sind die Wunschlage und die Lagen direkt neben jedem
/// Hindernis (rechts, links, darunter, darüber); gewählt wird die mit dem kürzesten Weg.
pub fn place(id: ItemId, area: Rect, w: f64, h: f64, k: f64, obstacles: &[Rect]) -> Option<Rect> {
    let base = anchor_rect(id, area, w, h, k);
    let g = GAP * k;
    let (mut best, mut best_d) = (None, f64::INFINITY);
    let span = 2 * obstacles.len() as i64;
    for i in -1..span {
        let x = candidate(i, base.x, w, g, obstacles, true);
        for j in -1..span {
            let y = candidate(j, base.y, h, g, obstacles, false);
            let r = Rect { x, y, w, h };
            let d = (x - base.x).abs() + (y - base.y).abs();
            if d >= best_d || !r.inside(area) || hits(r, obstacles) {
                continue;
            }
            best = Some(r);
            best_d = d;
        }
    }
    best
}

/// candidate ist der Kandidat i einer Achse: -1 = Wunschlage, 2o = hinter Hindernis o, 2o+1 = davor.
pub fn candidate(i: i64, base: f64, size: f64, g: f64, obstacles: &[Rect], horizontal: bool) -> f64 {
    if i < 0 {
        return base;
    }
    let o = obstacles[(i / 2) as usize];
    let (start, extent) = if horizontal { (o.x, o.w) } else { (o.y, o.h) };
    if i % 2 == 0 {
        return start + extent + g;
    }
    start - g - size
}

pub fn hits(r: Rect, obstacles: &[Rect]) -> bool {
    obstacles.iter().any(|o| r.overlaps(*o))
}
