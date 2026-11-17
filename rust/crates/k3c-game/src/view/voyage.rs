//! Port von `native/k3c-game/view/voyage.go`: die Schiffsfahrt beim Inselwechsel.

use k3c_platform::Canvas;

use super::frame::Frame;

/// Voyage ist die Schiffsfahrt beim Inselwechsel (B-977): von der verlassenen Insel `from` zur Zielinsel `to`
/// (Insel-IDs aus `Frame::route`). `done` sind die abgeschlossenen Inseln samt `from`, `open` die Ziele, die beim
/// Ablegen wählbar waren (samt `to`; die Karte zeigt sie wie am Hafen). `elapsed` sind die Sekunden seit dem Ablegen
/// aus dem Takt des Spiels, `duration` ist die Fahrzeit; nach `duration` liegt das Schiff am Ziel und wartet auf das
/// Laden. game führt die Fahrt (Beginn mit dem Ereignis islandSwitch, Ende nach Fahrzeit und Laden); nur lesen.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Voyage {
    pub from: String,
    pub to: String,
    pub done: Vec<String>,
    pub open: Vec<String>,
    pub elapsed: f64,
    pub duration: f64,
}

/// VoyageDrawer ist die optionale Rolle des Fachpakets, das die Schiffsfahrt im Vollbild zeichnet (hud, höchstens
/// einmal). `voyage_ready` meldet beim Ablegen, ob die Fahrt zu zeichnen ist (alle Assets da, beide Inseln auf der
/// Karte); sonst zeigt game den Ladebildschirm. `draw_voyage` zeichnet die Fahrt über das ganze Bild `dst`; false =
/// nichts gezeichnet (game zeichnet dann den Ladebildschirm). Beide rechnen keine Regel; `f` ist der letzte Frame.
pub trait VoyageDrawer {
    fn voyage_ready(&self, f: &Frame, v: &Voyage) -> bool;
    fn draw_voyage(&self, dst: &mut dyn Canvas, f: &Frame, v: &Voyage) -> bool;
}
