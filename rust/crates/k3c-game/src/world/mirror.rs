//! Spiegel-Fluss (RU55.3, B-1044, `docs/standards/bildaufbau.md` § 4): Im Look-Profil `k2c` zeigt der Fluss die Welt
//! über der Gehlinie gespiegelt statt eines Wellenmusters. Der Canvas spiegelt das fertig gezeichnete Bild der Zelle
//! (nach Licht und Fackeln): dunkler, leicht in Wasserfarbe getönt, gestaucht, mit schwankendem Rand und hellen
//! waagerechten Glanzstrichen; Lichtquellen bleiben nachts hell (`emissive` = 1 − Tageslicht). Das Zeichnen geht in
//! einem Canvas-Aufruf hinaus (`Canvas::mirror`), das Spiel legt nichts an. Kann der Canvas es nicht, die Fassung
//! `tiles` gewählt ist oder das Profil es nicht vorsieht, zeichnet `draw_river` die Wellen wie bisher.

use k3c_platform::{Canvas, MirrorUniforms, Rect};
use k3c_sim::state::World as SimWorld;

use super::World;
use super::ground::bank_height;
use super::light::daylight;
use super::river::RIVER_BASE;
use super::water::{RIVER_GLINT, RiverMode, WATER_GLINT_HZ, WATER_WOBBLE_HZ, phase_at, rgb, river_mode_for};
use crate::view::cell::Cell;
use crate::view::frame::Frame;

/// Uniforms des Spiegels der Zelle `c` zur Uhr `clock` (Sekunden) mit Lichtquellen-Anteil `emissive` (0 … 1).
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn mirror_uniforms(c: &Cell, clock: f64, emissive: f64) -> MirrorUniforms {
    let m = k3c_data::look::current().mirror;
    let ground = c.ground_y() as f32;
    // Das Wasser unter dem Spiegel ist der Grundton des Flusses, etwas heller zum Glanz hin
    let water = rgb(RIVER_BASE);
    MirrorUniforms {
        src_top: c.rect.min.y as f32,
        ground,
        axis: ground + bank_height(c) as f32,
        squash: m.squash as f32,
        brightness: m.brightness as f32,
        wobble: (m.wobble * c.camera.scale) as f32,
        phase_wobble: phase_at(clock, WATER_WOBBLE_HZ) as f32,
        phase_glint: phase_at(clock, WATER_GLINT_HZ) as f32,
        emissive: emissive.clamp(0.0, 1.0) as f32,
        tint: m.tint as f32,
        depth_fade: m.depth_fade as f32,
        // Nacht (emissive = 1 − Tageslicht): die Glanzstriche gehen auf `glint_night` des Tageswerts zurück
        glint_mix: (m.glint * (1.0 - (1.0 - m.glint_night) * emissive.clamp(0.0, 1.0))) as f32,
        water,
        glint: rgb(RIVER_GLINT),
    }
}

impl World {
    /// Spiegel statt Wellen: Profil sieht es vor, Fluss-Fassung `shader` und der Canvas kann es.
    pub fn mirror_wanted(&self, f: &Frame, dst: &dyn Canvas) -> bool {
        super::fx::on("mirror")
            && k3c_data::look::current().mirror.enabled
            && river_mode_for(&f.settings.river, self.start) == RiverMode::Shader
            && dst.can_mirror()
    }

    /// Zeichnet den Spiegel der Zelle in das Rechteck unter dem Ufer (bis zum unteren Rand der Zelle); `false`, wenn der
    /// Canvas ihn nicht zeichnet. Läuft nach Licht und Fackeln, damit die Spiegelung die beleuchtete Welt zeigt.
    #[allow(clippy::cast_precision_loss)]
    pub fn mirror_cell(&self, dst: &mut dyn Canvas, c: &Cell, clock: f64, emissive: f64) -> bool {
        let u = mirror_uniforms(c, clock, emissive);
        let h = c.rect.max.y as f32 - u.axis;
        if h <= 0.0 || c.rect.dx() <= 0 {
            return true;
        }
        dst.mirror(Rect::new(c.rect.min.x as f32, u.axis, c.rect.dx() as f32, h), &u)
    }

    /// Spiegel der Zelle in der Welt `wd` (Lichtquellen halten nachts ihre Helligkeit).
    pub fn draw_mirror(&self, dst: &mut dyn Canvas, c: &Cell, clock: f64, wd: &SimWorld) {
        let emissive = 1.0 - daylight(&wd.cycle);
        self.mirror_cell(dst, c, clock, emissive);
    }
}

#[cfg(test)]
#[path = "mirror_tests.rs"]
mod mirror_tests;
