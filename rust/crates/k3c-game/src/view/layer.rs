//! Port von `native/k3c-game/view/layer.go`: die festen Zeichen-Ebenen und der Drawer.

use k3c_platform::Canvas;

use super::cell::Cell;
use super::frame::Frame;

/// Layer ist die feste Zeichen-Ebene eines Drawers. Die Reihenfolge der Konstanten ist die Zeichenreihenfolge (hinten
/// nach vorn); neue Ebenen gibt es nur über die NG1-Nachfolge. Wie in Go eine Zahl, damit ungültige Werte darstellbar
/// bleiben (`valid`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Layer(pub i64);

impl Layer {
    /// ferne Kulisse, steht (Parallaxe 0)
    pub const FAR: Layer = Layer(0);
    /// mittlere Kulisse, scrollt langsam
    pub const MID: Layer = Layer(1);
    /// nahe Kulisse
    pub const NEAR: Layer = Layer(2);
    /// Boden und Natur am Boden
    pub const GROUND: Layer = Layer(3);
    /// Bauten, Bauplätze, Burg (props)
    pub const BUILDINGS: Layer = Layer(4);
    /// Figuren: Monarchen, Truppen, Gegner (actors)
    pub const ACTORS: Layer = Layer(5);
    /// Fluss im unteren Drittel der Zelle, vor allem anderen der Welt (world)
    pub const RIVER: Layer = Layer(6);
    /// Effekte: Geschosse, Treffer, Zauber (fx)
    pub const EFFECTS: Layer = Layer(7);
    /// Anzeigen je Zelle (hud)
    pub const HUD: Layer = Layer(8);
    /// einmal je Fenster über allen Zellen, z. B. FPS, Hinweise (Cell = Fenster)
    pub const OVERLAY: Layer = Layer(9);
}

const LAYER_COUNT: i64 = 10;

const LAYER_NAMES: [&str; 10] = [
    "far",
    "mid",
    "near",
    "ground",
    "buildings",
    "actors",
    "river",
    "effects",
    "hud",
    "overlay",
];

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.valid() {
            return f.write_str("invalid");
        }
        #[allow(clippy::cast_sign_loss)]
        f.write_str(LAYER_NAMES[self.0 as usize])
    }
}

impl Layer {
    /// Valid meldet, ob l eine der festen Ebenen ist.
    pub fn valid(self) -> bool {
        (0..LAYER_COUNT).contains(&self.0)
    }

    /// PerCell meldet, ob die Ebene je Zelle gezeichnet wird (alle außer `OVERLAY`).
    pub fn per_cell(self) -> bool {
        self.valid() && self != Layer::OVERLAY
    }
}

/// Layers liefert alle Ebenen in Zeichenreihenfolge.
pub fn layers() -> Vec<Layer> {
    (0..LAYER_COUNT).map(Layer).collect()
}

/// Drawer zeichnet einen Teil des Frames auf genau einer Ebene. `dst` ist bei Zellen-Ebenen die Zeichenfläche, vom
/// Aufrufer auf die Zelle beschnitten (`Canvas::clip`, Koordinaten bleiben Fenster-Koordinaten), bei `Layer::OVERLAY`
/// das ganze Fenster mit `c` = Fenster-Zelle. Ein Fachpaket mit mehreren Ebenen liefert mehrere Drawer.
pub trait Drawer {
    fn layer(&self) -> Layer;
    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell);
}

/// Listener bekommt jeden Frame genau einmal, bevor er gezeichnet wird (Ton, Effekte mit Zustand, Hinweise des HUD).
/// Ein Drawer mit eigenem Zustand über Bilder hinweg erfüllt zusätzlich Listener; game meldet ihn dann selbst an.
pub trait Listener {
    fn listen(&mut self, f: &Frame);
}

/// Sorted liefert die Drawer stabil nach Ebene sortiert (gleiche Ebene: Reihenfolge der Anmeldung); `None` und
/// ungültige Ebenen fallen heraus.
pub fn sorted(ds: &[Option<Box<dyn Drawer>>]) -> Vec<&dyn Drawer> {
    let mut out: Vec<&dyn Drawer> = ds
        .iter()
        .flatten()
        .map(|d| d.as_ref())
        .filter(|d| d.layer().valid())
        .collect();
    out.sort_by_key(|d| d.layer());
    out
}

#[cfg(test)]
#[path = "layer_tests.rs"]
mod layer_tests;
