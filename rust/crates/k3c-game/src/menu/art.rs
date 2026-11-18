//! Port von `native/k3c-game/menu/art.go`: Illustrationen der Menü-Karten (B-818, Vorbild `src/scenes/hubArtData.ts`):
//! je Eintrag von Haupt- und Pause-Menü eine Papierfigur aus der vorhandenen Spielgrafik (Entscheidung 005, keine eigenen
//! Assets). Fehlt ein Sheet, zeichnet die Karte nur Text.
//!
//! Abweichung: `Art(lib)` ist [`art`] mit einer `usable`-Abfrage; `menus.Assets` heißt [`Menus::assets`] (der Rust-Client
//! entscheidet in RU26 selbst, wie er es ruft).

use k3c_platform::{Canvas, Rect, TexId};

use super::Menus;
use super::draw::Painter;
use super::pending::{MAIN_COUNT, PAUSE_ROWS};
use crate::art::{STANDARD_WORLD, WorldImage, WorldImages};

/// Die Animation, deren erstes Bild die Karte zeigt; die Weltbilder (Rig hintergrund) heißen "stand".
pub const ART_ANIM: &str = "idle";
pub const WORLD_ANIM: &str = "stand";

/// Bilder der Einträge des Hauptmenüs (`gruppe/id`).
pub const MAIN_ART: [&str; MAIN_COUNT as usize] = [
    "truppen/krieger",
    "objekte/portal",
    "objekte/truhe-offen",
    "objekte/truhe",
    "truppen/bauer",
    "objekte/stern",
    "objekte/ausgang",
];
/// Bilder der Zeilen des Pause-Menüs.
pub const PAUSE_ART: [&str; PAUSE_ROWS as usize] = [
    "truppen/krieger",
    "objekte/truhe",
    "objekte/stern",
    "truppen/bauer",
    "objekte/ausgang",
];

/// Nennt die Assets aller Menü-Illustrationen samt der großen und kleinen Weltbilder, sortiert und ohne Doppelte (game
/// lädt sie im Hauptmenü). Weltbilder sind weich: nur was `usable` laden kann (fehlende lassen das Laden nie scheitern).
pub fn art(usable: Option<&dyn Fn(&str) -> bool>) -> Vec<String> {
    art_list(&WorldImages::from_data(), true, usable)
}

/// Nennt die Karten-Illustrationen und die Weltbilder von `worlds`, die `usable` laden kann (große nur mit `big`),
/// sortiert, ohne Doppelte.
pub fn art_list(worlds: &WorldImages, big: bool, usable: Option<&dyn Fn(&str) -> bool>) -> Vec<String> {
    let soft: Vec<String> = worlds
        .assets(big)
        .into_iter()
        .filter(|a| usable.is_some_and(|u| u(a)))
        .collect();
    let mut out: Vec<String> = Vec::new();
    let fixed = MAIN_ART.iter().chain(PAUSE_ART.iter()).map(|a| (*a).to_string());
    for a in fixed.chain(soft) {
        if !out.contains(&a) {
            out.push(a);
        }
    }
    out.sort_unstable();
    out
}

impl Menus {
    /// Meldet die Illustrationen in jedem Level: das Pause-Menü zeigt sie im laufenden Spiel, dazu die kleinen
    /// Weltbilder (die großen nur im Hauptmenü).
    pub fn assets(&self) -> Vec<String> {
        art_list(&self.worlds, false, self.usable.as_deref())
    }

    /// Liefert das Bild (groß oder klein) der Welt `id`: das der Welt, fehlt dessen Sheet das der Standard-Welt, fehlt
    /// auch das "" (Text-Karte). Läuft nur beim Bauen der Seite, nie je Frame.
    pub fn world_asset(&self, id: &str, big: bool) -> String {
        let pick = |w: WorldImage| if big { w.big } else { w.small };
        let a = pick(self.worlds.of(id));
        let Some(has) = self.has.as_deref() else {
            return a;
        };
        if a.is_empty() || has(&a) {
            return a;
        }
        let std = pick(self.worlds.of(STANDARD_WORLD));
        if !std.is_empty() && has(&std) {
            return std;
        }
        String::new()
    }
}

impl Painter {
    /// Das erste Bild des Assets (Karten-Illustration "idle", Weltbild "stand"); `None` ohne Sheet.
    pub fn frame_of(&self, asset: &str) -> Option<(TexId, Rect)> {
        let lib = self.lib.as_ref()?.borrow();
        let s = lib.sheet(asset, ART_ANIM).or_else(|| lib.sheet(asset, WORLD_ANIM))?;
        s.frame(0).map(|f| (f.tex, f.src))
    }

    /// Meldet, ob die Bibliothek das Bild des Assets geladen hat.
    pub fn has_sheet(&self, asset: &str) -> bool {
        self.frame_of(asset).is_some()
    }

    /// Zeichnet das erste Bild der Illustration `asset` eingepasst in das Feld `(x, y, w, h)`, unten mittig; ohne Sheet
    /// nichts (Text-Karte, `false`). Linearer Filter wie die Spielgrafik.
    #[allow(clippy::cast_possible_truncation)]
    pub fn art_box(&mut self, dst: &mut dyn Canvas, asset: &str, x: f64, y: f64, w: f64, h: f64) -> bool {
        if asset.is_empty() {
            return false;
        }
        let Some((tex, src)) = self.frame_of(asset) else {
            return false;
        };
        let (iw, ih) = (f64::from(src.w), f64::from(src.h));
        let k = (w / iw).min(h / ih);
        let dst_rect = Rect::new(
            (x + (w - iw * k) / 2.0) as f32,
            (y + h - ih * k) as f32,
            (iw * k) as f32,
            (ih * k) as f32,
        );
        dst.sprite(tex, src, dst_rect, [1.0; 4]);
        true
    }
}
