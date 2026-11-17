//! Rig-Bibliothek: Figuren aus Teilen (`k3c-rig`, Vorlage `native/k3c-game/art/rigs.go`). Fachmodule dürfen
//! `k3c_rig` nicht importieren, deshalb reicht art die nötigen Typen durch und baut Actor und Painter über die
//! Library. Nur dieses Modul und `png.rs` nennen `k3c_rig`.
//!
//! Abweichung von Go: Das Hochladen der Teil-Bilder braucht die Zeichenfläche. `Library::new` prüft den Rig-Ordner
//! (Elemente-Datei) ohne Canvas, `Library::load_rigs` lädt die Bilder hoch; Actor, Painter und `rig_assets` setzen
//! das voraus (sonst `Missing` wie Gos „Rig-Bibliothek nicht geladen“).

use std::path::Path;
use std::rc::Rc;

use k3c_platform::Canvas;
use k3c_rig::paint::{Actor as RigActor, Assets, Painter as RigPainter, State as RigState};
use k3c_rig::rig::{Fehler, M as RigM, Point as RigPoint, Rig, load_elements};

use super::{ArtError, Library};
use crate::view::cell::Camera;

/// Painter zeichnet Rig-Figuren in ein Ziel (Canvas) mit einer Sicht (View).
pub type Painter = RigPainter;
/// Actor ist eine Figur aus Rig-Teilen mit Zustand (Animation, Variante, Waffe).
pub type Actor = RigActor;
/// State ist Animation bzw. Gangart einer Figur.
pub type State = RigState;
/// M ist eine 2D-Affin-Matrix wie SVG matrix(a b c d e f) (Painter.View, RigView).
pub type M = RigM;
/// Point ist ein Punkt in Rig-Einheiten (Actor.SetFrame: Kette des Umhangs).
pub type Point = RigPoint;

/// Meldet, ob der Rig-Ordner lesbar ist (Elemente-Datei); die Bilder lädt `Library::load_rigs`.
pub(super) fn check_rigs(dir: &Path) -> Result<(), ArtError> {
    load_elements(&dir.join("elemente"))
        .map(|_| ())
        .map_err(|e| missing_rigs(dir, &e))
}

fn missing_rigs(dir: &Path, e: &Fehler) -> ArtError {
    ArtError::Missing(format!("Rig-Bibliothek {}: {e}", dir.display()))
}

fn not_loaded() -> ArtError {
    ArtError::Missing("Rig-Bibliothek nicht geladen (RigDir leer)".to_string())
}

/// ActorFrame liefert den Rahmen einer Figur (x, y, Breite, Höhe in Rig-Einheiten, Fußpunkt bei 0,0): beritten bzw.
/// Vierbeiner der des Reittiers, zu Fuß der des Reiters (wie native/k3c-ebiten/rigs.go).
pub fn actor_frame(a: &Actor) -> [f64; 4] {
    frame_of(a.rider.as_deref(), a.mount.as_deref())
}

/// Rahmen aus Reiter und Reittier (Kern von `actor_frame`; `Actor` hat private Felder, so bleibt er testbar).
pub(super) fn frame_of(rider: Option<&Rig>, mount: Option<&Rig>) -> [f64; 4] {
    if let Some(m) = mount {
        return m.frame;
    }
    if let Some(r) = rider {
        return r.frame;
    }
    [0.0; 4]
}

/// RigView liefert die Sicht-Matrix (Painter.View) einer Rig-Figur: Fußpunkt bei Welt-x wx (Units) auf der
/// Boden-Linie ground_y (Fenster-y in px, `Cell.ground_y`), k ist die Kamera der Zelle. size sind Welt-Units je
/// Rig-Einheit (Spielhöhe der Figur / Rahmenhöhe), frame ihr Rahmen (`actor_frame`). left spiegelt wie das Sheet im
/// eigenen Rahmen (nicht um den Fußpunkt), damit Rig und Sheet an derselben Stelle stehen. Reine Rechnung.
pub fn rig_view(k: &Camera, wx: f64, ground_y: f64, size: f64, frame: [f64; 4], left: bool) -> M {
    let u = size * k.scale; // Bildpunkte je Rig-Einheit
    let (mut fx, mut sx) = (k.screen_x(wx), u);
    if left {
        fx += (frame[2] + 2.0 * frame[0]) * u;
        sx = -u;
    }
    [sx, 0.0, 0.0, u, fx, ground_y]
}

impl Library {
    /// Lädt die Rig-Bilder hoch (Go: `paint.LoadAssets` in `New`); ohne `rig_dir` bleibt es leer. Fehlt etwas:
    /// `Missing` mit dem Pfad.
    pub fn load_rigs(&mut self, canvas: &mut dyn Canvas) -> Result<(), ArtError> {
        let dir = self.cfg.rig_dir.clone();
        if dir.as_os_str().is_empty() {
            return Ok(());
        }
        self.rigs = Some(Assets::load(&dir, canvas).map_err(|e| missing_rigs(&dir, &e))?);
        Ok(())
    }

    /// Die geladenen Rig-Bilder (für `Actor::draw`), `None` vor `load_rigs`.
    pub fn rig_assets(&self) -> Option<&Assets> {
        self.rigs.as_ref()
    }

    /// Legt einen Painter an (Puffer werden wiederverwendet; einer je Zeichner genügt).
    pub fn new_painter(&self) -> Result<Painter, ArtError> {
        let a = self.rigs.as_ref().ok_or_else(not_loaded)?;
        Ok(Painter::new(a.white))
    }

    /// Legt eine Figur an: rider und mount sind Rig-Ordner `gruppe/id` ("" = keiner), variant z. B. "spieler2".
    /// Fehlende Teil-Bilder melden den Pfad.
    pub fn new_actor(
        &mut self,
        canvas: &mut dyn Canvas,
        rider: &str,
        mount: &str,
        variant: &str,
    ) -> Result<Actor, ArtError> {
        let a = self.rigs.as_mut().ok_or_else(not_loaded)?;
        let r = rig(a, rider)?;
        let m = rig(a, mount)?;
        Actor::new(a, canvas, r, m, variant).map_err(|e| ArtError::Broken(e.to_string()))
    }
}

/// Lädt das Rig `gruppe/id` ("" = keins).
fn rig(a: &mut Assets, rel: &str) -> Result<Option<Rc<Rig>>, ArtError> {
    if rel.is_empty() {
        return Ok(None);
    }
    a.rig(rel).map(Some).map_err(|e| ArtError::Broken(e.to_string()))
}
