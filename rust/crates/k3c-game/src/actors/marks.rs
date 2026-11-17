//! Port von `native/k3c-game/actors/marks.go`.
//!
//! Anzeigen an den Figuren (NG12, B-806): Lebensbalken, Beute-Gold der Gegner, Last der Bauern und der Punkt unter
//! dem Monarchen. Spiegel von src/scenes/render/worldRenderer.ts (bar, createTroop, createEnemy, createPlayer).
//! Alles hier ist reine Rechnung in Spiel-Pixeln des Browsers (32 je Unit) relativ zum Fußpunkt der Figur, x nach
//! rechts, y nach oben; markdraw.rs zeichnet sie.

use k3c_sim::state::Job;

use crate::view::frame::Seat;

/// barHeightPx ist die Höhe jedes Lebensbalkens (worldRenderer.ts › bar).
pub const BAR_HEIGHT_PX: f64 = 8.0;

/// barLook ist Breite, Abstand der Balken-Mitte über der Figuren-Oberkante und Füllfarbe eines Balkens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BarLook {
    pub width: f64,
    pub above: f64,
    pub color: u32,
}

// Balken je Figurenart (createTroop, createEnemy, createPlayer).
pub const TROOP_BAR: BarLook = BarLook {
    width: 36.0,
    above: 16.0,
    color: 0x52b788,
};
pub const ENEMY_BAR: BarLook = BarLook {
    width: 44.0,
    above: 32.0,
    color: 0xe63946,
};

// Farben und Maße der übrigen Anzeigen.
/// Balken-Hintergrund, Deckkraft `BAR_BACK_ALPHA`
pub const BAR_BACK: u32 = 0x00_0000;
pub const BAR_BACK_ALPHA: f64 = 0.6;

pub const LOOT_RADIUS: f64 = 10.0;
pub const LOOT_FILL: u32 = 0xff_d166;
pub const LOOT_EDGE: u32 = 0x9c_6644;
/// Mitte des Goldkreises über der Oberkante
pub const LOOT_ABOVE: f64 = 12.0;

pub const LOAD_W: f64 = 16.0;
pub const LOAD_H: f64 = 24.0;
pub const LOAD_FILL: u32 = 0x6b_4226;
pub const LOAD_EDGE: u32 = 0x3b_2414;
/// Mitte der Last in Anteilen der Figurenhöhe
pub const LOAD_HEIGHT: f64 = 0.55;
/// seitlicher Versatz, hinter dem Rücken
pub const LOAD_SIDE: f64 = 14.0;
/// Randbreite von Goldkreis und Last
pub const EDGE_WIDTH: f64 = 2.0;

pub const MARKER_W: f64 = 56.0;
pub const MARKER_H: f64 = 12.0;
/// der Punkt sitzt so weit unter dem Boden
pub const MARKER_DROP: f64 = 4.0;
pub const MARKER_ALPHA: f64 = 0.8;

/// playerColors sind die Farben der Spieler (src/core/constants.ts › PLAYER_COLORS).
pub const PLAYER_COLORS: [u32; 4] = [0x3a_86ff, 0xff_006e, 0xfb_5607, 0x83_38ec];

/// PlayerColor liefert die Farbe des Monarchen mit Spieler-Index i (zyklisch).
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
pub fn player_color(i: i64) -> u32 {
    let n = PLAYER_COLORS.len() as i64;
    PLAYER_COLORS[(i % n + n).rem_euclid(n) as usize]
}

/// BarVisible: Der Balken erscheint nur bei verletzter Figur (hp < maxHp); ohne Lebenswert (maxHp ≤ 0) nie.
pub fn bar_visible(hp: f64, max_hp: f64) -> bool {
    max_hp > 0.0 && hp < max_hp
}

/// BarFraction ist der gefüllte Anteil 0 … 1 (Phaser.Math.Clamp(hp / maxHp)).
pub fn bar_fraction(hp: f64, max_hp: f64) -> f64 {
    if max_hp <= 0.0 {
        return 0.0;
    }
    k3c_det::fmin(k3c_det::fmax(hp / max_hp, 0.0), 1.0)
}

/// HasLoot: Der Gegner trägt geklautes Gold (carriedGold > 0) und zeigt den Goldkreis.
pub fn has_loot(carried_gold: i64) -> bool {
    carried_gold > 0
}

/// Carrying: Der Bauer trägt etwas zum Lager (Auftrag carry) und zeigt seine Last.
pub fn carrying(j: Option<&Job>) -> bool {
    j.is_some_and(|j| j.r#type == "carry")
}

/// LoadOffset ist der seitliche Versatz der Last in px: hinter dem Rücken, also bei Blick nach links rechts davon.
pub fn load_offset(left: bool) -> f64 {
    if left { LOAD_SIDE } else { -LOAD_SIDE }
}

/// Seated: Der Monarch mit Spieler-Index monarch gehört einem lokalen Spieler (Frame.Seats); freie Monarchen
/// (niemand steuert sie) bleiben unsichtbar (worldRenderer.ts › sync(hidden), GameScene.ts).
pub fn seated(seats: &[Seat], monarch: i64) -> bool {
    seats.iter().any(|s| s.monarch == monarch)
}

/// box ist ein Rechteck um die Mitte (cx, cy) in px, y nach oben.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoxPx {
    pub cx: f64,
    pub cy: f64,
    pub w: f64,
    pub h: f64,
}

/// barBoxes liefert Hintergrund und Füllung eines Balkens über einer Figur mit Oberkante topPx und gefülltem Anteil
/// frac; die Füllung wächst von der linken Kante des Hintergrunds.
pub fn bar_boxes(l: BarLook, top_px: f64, frac: f64) -> (BoxPx, BoxPx) {
    let cy = top_px + l.above;
    (
        BoxPx {
            cx: 0.0,
            cy,
            w: l.width,
            h: BAR_HEIGHT_PX,
        },
        BoxPx {
            cx: -l.width / 2.0 * (1.0 - frac),
            cy,
            w: l.width * frac,
            h: BAR_HEIGHT_PX,
        },
    )
}

/// lootCenter ist die Mitte des Goldkreises über einer Figur mit Oberkante topPx.
pub fn loot_center(top_px: f64) -> (f64, f64) {
    (0.0, top_px + LOOT_ABOVE)
}

/// loadBox ist das Rechteck der Last einer Figur der Höhe topPx, die nach links (left) schaut.
pub fn load_box(top_px: f64, left: bool) -> BoxPx {
    BoxPx {
        cx: load_offset(left),
        cy: top_px * LOAD_HEIGHT,
        w: LOAD_W,
        h: LOAD_H,
    }
}

/// markerBox ist die Ellipse des Spieler-Punkts unter den Füßen.
pub fn marker_box() -> BoxPx {
    BoxPx {
        cx: 0.0,
        cy: -MARKER_DROP,
        w: MARKER_W,
        h: MARKER_H,
    }
}

#[cfg(test)]
#[path = "marks_tests.rs"]
mod marks_tests;
