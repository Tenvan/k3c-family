//! Port von `native/k3c-game/props/plan.go` (Teil 1: Typen, Puffer, `plan`). Zeichenliste einer Zelle: aus Welt,
//! abgeleitetem Zustand und Kamera entsteht eine Liste von Bildern und Formen in Welt-Maßen (Units, Höhe über dem
//! Boden), hinten nach vorn. Rein, ohne Zeichen-Stack; `paint` (RU16.5) zeichnet sie.
//! Reihenfolge wie die Tiefen im Browser (worldRenderer.ts): Burg, feste Objekte und Natur (0), Hintergrundmauer (1),
//! Bauplätze (2), Plantage und Truhen (3), Münzen (6); Schilder, Balken und Texte danach (top).
//! `castle` steht in `plan_castle.rs`, `entities` und `nodes` in `plan_entities.rs`.

use std::rc::Rc;

use k3c_sim::state::World;

use super::buildings::{BackWall, Look};
use super::listen::StageInfo;
use super::simview::SimView;
use super::sites::Texts;
use crate::actors::catalog::UNIT_PX;
use crate::art::{Library, Sheet};
use crate::view::cell::Camera;

/// `px` rechnet Spiel-Pixel des Browsers in Units um (Papier-Rahmen sind in Spiel-Pixeln, 32 je Kachel).
pub fn px(v: f64) -> f64 {
    v / UNIT_PX
}

/// `Shape` ist die Art eines Eintrags der Zeichenliste (Reihenfolge wie die `iota`-Konstanten in Go).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Shape {
    /// Bild `look`, Fuß bei (x, y), Startframe `phase`, vertikal `sy`
    #[default]
    Sprite,
    /// Bild `look` gekachelt von x bis x2, Fuß auf y
    Tiles,
    /// Rechteck Mitte x, Unterkante y, Größe w × h
    Rect,
    /// Rechteck-Rahmen wie `Rect`, Linienbreite `size`
    Frame,
    /// Kreis um (x, y) mit Radius w
    Circle,
    /// Ring wie `Circle`, Linienbreite `size`
    Ring,
    /// Linie (x, y) → (x2, y2), Breite `size`
    Line,
    /// Dreieck (Spitze oben) Mitte x, Unterkante y, Größe w × h
    Tri,
    /// Text Mitte x, Unterkante y, Höhe `size`
    Text,
}

/// `Item` ist ein Eintrag der Zeichenliste; Maße in Units, y = Höhe über dem Boden.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Item {
    pub shape: Shape,
    pub look: Look,
    pub phase: i64,
    pub x: f64,
    pub y: f64,
    pub x2: f64,
    pub y2: f64,
    pub w: f64,
    pub h: f64,
    pub size: f64,
    /// vertikaler Maßstab des Bilds (0 = 1)
    pub sy: f64,
    /// Abdunklung 0 … 1 (Schaden)
    pub shade: f32,
    /// Deckkraft (0 = voll)
    pub alpha: f32,
    pub clr: [u8; 4],
    pub text: String,
}

/// `Sheets` liefert die geladenen Sheets (`art::Library`; `None` im Planer = keine Grafik, dann Platzhalter).
pub trait Sheets {
    fn sheet(&self, asset: &str, anim: &str) -> Option<&Sheet>;
}

impl Sheets for Library {
    fn sheet(&self, asset: &str, anim: &str) -> Option<&Sheet> {
        Library::sheet(self, asset, anim)
    }
}

pub const WHITE: [u8; 4] = [255, 255, 255, 255];
pub const GOLD: [u8; 4] = [255, 209, 102, 255];
pub const HP_GREEN: [u8; 4] = [82, 183, 136, 255];
pub const DARK: [u8; 4] = [52, 58, 64, 255];
pub const STONE_GRAY: [u8; 4] = [108, 117, 125, 255];
pub const POOR_RED: [u8; 4] = [255, 143, 163, 255];

/// `Planner` baut die Zeichenliste; die Puffer bleiben über die Bilder erhalten (`clear()` statt neu anlegen).
/// Go-Zeiger werden Indizes: `walls` verweist in `World::sites`, `viewer` in `World::players`.
pub struct Planner {
    pub lib: Option<Rc<dyn Sheets>>,
    pub body: Vec<Item>,
    pub top: Vec<Item>,
    pub walls: Vec<usize>,
    pub backs: Vec<BackWall>,
    pub roots: Vec<f64>,
    pub texts: Texts,
    /// Monarch der gezeichneten Zelle (Preis der Zahlstellen), `None` = keiner
    pub viewer: Option<usize>,
}

impl Planner {
    pub fn new(lib: Option<Rc<dyn Sheets>>) -> Planner {
        Planner {
            lib,
            body: Vec::new(),
            top: Vec::new(),
            walls: Vec::new(),
            backs: Vec::new(),
            roots: Vec::new(),
            texts: Texts::default(),
            viewer: None,
        }
    }

    /// `size` liefert Breite und Höhe über dem Fuß (Units) des Bilds `l`; `None` = Sheet nicht geladen.
    pub fn size(&self, l: Look) -> Option<(f64, f64)> {
        if !l.ok() {
            return None;
        }
        let s = self.lib.as_ref()?.sheet(l.asset, l.anim)?;
        let k = s.units_per_px / UNIT_PX;
        Some((s.fw as f64 * k, s.fh as f64 * s.origin_y * k))
    }

    /// `sprite` legt ein Bild an; ohne geladenes Sheet `false`.
    pub fn sprite(&mut self, l: Look, x: f64, y: f64, phase: i64) -> bool {
        if self.size(l).is_none() {
            return false;
        }
        self.body.push(Item {
            shape: Shape::Sprite,
            look: l,
            x,
            y,
            phase,
            ..Default::default()
        });
        true
    }

    pub fn rect(buf: &mut Vec<Item>, x: f64, y: f64, w: f64, h: f64, clr: [u8; 4]) {
        buf.push(Item {
            shape: Shape::Rect,
            x,
            y,
            w,
            h,
            clr,
            ..Default::default()
        });
    }

    /// `bar` zeichnet einen Balken (Hintergrund schwarz, Anteil `frac` in `clr`), Unterkante y.
    pub fn bar(&mut self, x: f64, y: f64, w: f64, frac: f64, clr: [u8; 4]) {
        let h = px(8.0);
        Planner::rect(&mut self.top, x, y, w, h, [0, 0, 0, 153]);
        let f = frac.clamp(0.0, 1.0);
        Planner::rect(&mut self.top, x - w / 2.0 + w * f / 2.0, y, w * f, h, clr);
    }

    /// `label` legt einen Text an (Mitte x, Unterkante y; Größe 32 Spiel-Pixel wie fontRules › priceTag).
    pub fn label(&mut self, x: f64, y: f64, s: &str, clr: [u8; 4]) {
        if !s.is_empty() {
            self.top.push(Item {
                shape: Shape::Text,
                x,
                y,
                size: 1.0,
                text: s.to_string(),
                clr,
                ..Default::default()
            });
        }
    }

    /// `plan` baut die Zeichenliste der Welt `w` (`info` = Zustand aus `Listen`, darf fehlen) für Kamera `k`.
    pub fn plan(&mut self, w: Option<&World>, info: Option<&StageInfo>, k: &Camera, clock: f64, view: &dyn SimView) {
        self.body.clear();
        self.top.clear();
        let Some(w) = w else {
            return;
        };
        let biome = w.biome.id.as_str();
        self.decor(w, k, clock);
        self.castle(w, info, k);
        self.entities(w, k, biome);
        self.nodes(w, k, biome);
        self.back_walls(w, k);
        self.sites(w, info, k);
        self.plantation(w, k);
        self.pickups(w, info, k, clock);
        self.coins(w, k);
        self.combat(w, info, k, view);
    }
}

/// `visible`: ragt ein Objekt bei x mit halber Breite `half` in die Zelle (`half` großzügig für Schilder darüber).
pub fn visible(k: &Camera, x: f64, half: f64) -> bool {
    k.scale == 0.0 || k.visible(x, half)
}
