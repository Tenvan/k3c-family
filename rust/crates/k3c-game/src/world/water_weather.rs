//! Wetter im Bild (RU62.7, B-999): Vorboten und Wirkung von Taifun und Flut. Das Spiel zeichnet nur, Art und Phase kommen
//! aus `World::weather`, die überfluteten Randzonen aus `World::flood_zones` (Sim, RU62.5 und RU62.6). Gezeigt wird nur,
//! wenn die Stufe der Zelle an der Oberfläche liegt (Tiefe 0): jede Split-Screen-Hälfte entscheidet für sich, ein Stufenwechsel
//! wechselt das Bild mit. Regen und Wind sind gezeichnete Linien aus reiner Rechnung (keine Zuteilung, kein Asset); nur die
//! Wolkenbank der Vorboten ist ein Bild (`hintergruende/wetter-wolkenbank`), ohne Sheet ein dunkler Verlauf.

use k3c_platform::Canvas;
use k3c_sim::state::World as SimWorld;
use k3c_sim::state::weather_view::{WeatherKind, WeatherPhase};

use crate::view::cell::Cell;
use crate::view::frame::Frame;
use crate::world::World;
use crate::world::layers::{ANIM_STAND, stage_of};

/// Das Bild der Wolkenbank (Gruppe `hintergruende`, Zuordnung in `data/paper.json › wetter`).
pub const CLOUD_ASSET: &str = "hintergruende/wetter-wolkenbank";
/// Regenlinien je Zelle (Taifun aktiv) und Windstreifen.
pub const RAIN_STREAKS: usize = 90;
pub const WIND_STREAKS: usize = 14;
/// Höhe der überfluteten Zone und der Wolkenbank (Anteil der Welt-Höhe).
const FLOOD_DEPTH_UNITS: f64 = 0.9;
const CLOUD_SHARE: f64 = 0.3;
/// Höchster Anstieg des Wassers am Ufer bei den Vorboten der Flut, in Units.
const RISE_MAX_UNITS: f64 = 0.45;

/// Was die Zelle vom Wetter zeigt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sight {
    None,
    TaifunForecast,
    TaifunActive,
    FloodForecast,
    FloodActive,
}

/// `sight_of`: das Wetter der Stufe `wd`; unter Tage (Tiefe über 0) und ohne Wetter nichts.
pub fn sight_of(wd: &SimWorld) -> Sight {
    if wd.biome.depth != 0 {
        return Sight::None;
    }
    match wd.weather {
        Some(s) => match (s.kind, s.phase) {
            (WeatherKind::Taifun, WeatherPhase::Forecast) => Sight::TaifunForecast,
            (WeatherKind::Taifun, WeatherPhase::Active) => Sight::TaifunActive,
            (WeatherKind::Flut, WeatherPhase::Forecast) => Sight::FloodForecast,
            (WeatherKind::Flut, WeatherPhase::Active) => Sight::FloodActive,
        },
        None => Sight::None,
    }
}

fn frac(v: f64) -> f64 {
    v - v.floor()
}

/// `rain_streak`: Linie `i` des Regens in Fenster-px (`x0, y0, x1, y1`) in einer Fläche `w × h` zur Zeit `clock`; schräg in
/// Windrichtung (nach rechts), die Anfangslage je Linie folgt der Zahl `i` (keine Zufallsquelle).
pub fn rain_streak(i: usize, clock: f64, w: f64, h: f64) -> [f64; 4] {
    let k = i as f64;
    let x = frac(k * 0.618_033_988_7) * (w + 0.2 * h);
    let speed = 1.2 + frac(k * 0.381_966_011) * 0.8;
    let y = frac(k * 0.754_877_666 + clock * speed) * h;
    let len = 0.05 * h + frac(k * 0.569_840_29) * 0.04 * h;
    [x - 0.2 * h, y, x - 0.2 * h + 0.18 * len, y + len]
}

/// `wind_streak`: Windstreifen `i`, lang und flach, wandert nach rechts über die Fläche `w × h`.
pub fn wind_streak(i: usize, clock: f64, w: f64, h: f64) -> [f64; 4] {
    let k = i as f64;
    let speed = 0.25 + frac(k * 0.381_966_011) * 0.2;
    let x = frac(k * 0.618_033_988_7 + clock * speed) * (w + 0.3 * w) - 0.15 * w;
    let y = (0.05 + frac(k * 0.754_877_666) * 0.9) * h;
    let len = 0.08 * w + frac(k * 0.569_840_29) * 0.08 * w;
    [x, y, x + len, y + 0.02 * h * (frac(k * 0.43) - 0.5)]
}

/// `flood_span`: Spanne einer überfluteten Zone in Fenster-x, auf die Zelle `[min, max]` begrenzt; leer = nicht sichtbar.
pub fn flood_span(sx0: f64, sx1: f64, min: f64, max: f64) -> Option<(f64, f64)> {
    let (a, b) = (sx0.min(sx1).max(min), sx0.max(sx1).min(max));
    (b > a).then_some((a, b))
}

/// `rise_units`: Anstieg des Wassers am Ufer bei den Vorboten der Flut nach dem Fortschritt des Tages (0 … 1).
pub fn rise_units(progress: f64) -> f64 {
    RISE_MAX_UNITS * progress.clamp(0.0, 1.0)
}

impl World {
    /// `draw_weather_sky`: Wolkenbank der Vorboten am Himmel und, solange der Taifun wirkt, ein dunkler Schleier über der
    /// Welt; nach der fernen Schicht gerufen.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn draw_weather_sky(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(wd) = stage_of(f, c).and_then(|s| s.world.as_deref()) else {
            return;
        };
        let sight = sight_of(wd);
        if !matches!(sight, Sight::TaifunForecast | Sight::TaifunActive) {
            return;
        }
        let r = c.world_rect();
        let (x, y, w, h) = (r.min.x as f32, r.min.y as f32, r.dx() as f32, r.dy() as f32);
        let band = h * CLOUD_SHARE as f32;
        let cloud = self
            .lib
            .as_ref()
            .and_then(|l| l.sheet(CLOUD_ASSET, ANIM_STAND))
            .and_then(|sh| sh.frame(0).map(|fr| (fr.tex, fr.src)));
        match cloud {
            Some((tex, src)) => dst.sprite(tex, src, k3c_platform::Rect::new(x, y, w, band), [1.0, 1.0, 1.0, 0.95]),
            None => {
                for i in 0..4 {
                    let a = 0.28 * (1.0 - i as f32 / 4.0);
                    dst.rect(
                        k3c_platform::Rect::new(x, y + band * i as f32 / 4.0, w, band / 4.0),
                        [0.12, 0.13, 0.18, a],
                    );
                }
            }
        }
        if sight == Sight::TaifunActive {
            dst.rect(k3c_platform::Rect::new(x, y, w, h), [0.1, 0.12, 0.18, 0.22]);
        }
    }

    /// `draw_weather_front`: Regen und Windstreifen des aktiven Taifuns, steigendes Wasser der Flut-Vorboten und die
    /// überfluteten Randzonen der aktiven Flut; vor dem Licht gerufen.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn draw_weather_front(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let Some(wd) = stage_of(f, c).and_then(|s| s.world.as_deref()) else {
            return;
        };
        let r = c.world_rect();
        let (x, y, w, h) = (r.min.x as f64, r.min.y as f64, r.dx() as f64, r.dy() as f64);
        match sight_of(wd) {
            Sight::TaifunActive => {
                for i in 0..RAIN_STREAKS {
                    let [x0, y0, x1, y1] = rain_streak(i, f.clock, w, h);
                    dst.stroke_line(
                        (x + x0) as f32,
                        (y + y0) as f32,
                        (x + x1) as f32,
                        (y + y1) as f32,
                        1.5,
                        [0.78, 0.84, 0.92, 0.4],
                    );
                }
                for i in 0..WIND_STREAKS {
                    let [x0, y0, x1, y1] = wind_streak(i, f.clock, w, h);
                    dst.stroke_line(
                        (x + x0) as f32,
                        (y + y0) as f32,
                        (x + x1) as f32,
                        (y + y1) as f32,
                        1.0,
                        [1.0, 1.0, 1.0, 0.22],
                    );
                }
            }
            Sight::FloodForecast => {
                let up = rise_units(wd.cycle.progress) * c.camera.scale;
                dst.rect(
                    k3c_platform::Rect::new(x as f32, (c.ground_y() as f64 - up) as f32, w as f32, up as f32),
                    [0.28, 0.5, 0.75, 0.45],
                );
            }
            Sight::FloodActive => self.draw_flood_zones(dst, wd, c),
            Sight::None | Sight::TaifunForecast => {}
        }
    }

    /// Überflutete Zonen: Wasserfläche über dem Boden, helle Wasserlinie oben, deutliche Grenze zur Inselmitte hin.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn draw_flood_zones(&self, dst: &mut dyn Canvas, wd: &SimWorld, c: &Cell) {
        let ground = c.ground_y() as f64;
        let depth = FLOOD_DEPTH_UNITS * c.camera.scale;
        let (min, max) = (c.rect.min.x as f64, c.rect.max.x as f64);
        for z in &wd.flood_zones {
            let (sx0, sx1) = (c.camera.screen_x(z.from), c.camera.screen_x(z.to));
            let Some((a, b)) = flood_span(sx0, sx1, min, max) else {
                continue;
            };
            dst.rect(
                k3c_platform::Rect::new(a as f32, (ground - depth) as f32, (b - a) as f32, depth as f32),
                [0.2, 0.45, 0.72, 0.6],
            );
            dst.stroke_line(
                a as f32,
                (ground - depth) as f32,
                b as f32,
                (ground - depth) as f32,
                2.0,
                [0.85, 0.94, 1.0, 0.9],
            );
            // Grenze: die Kante der Zone, die zur Inselmitte zeigt (nicht der Inselrand), nur wenn sie im Bild liegt
            let edge = if z.from <= wd.width_units / 2.0 { sx1 } else { sx0 };
            if edge > min && edge < max {
                dst.stroke_line(
                    edge as f32,
                    (ground - depth * 1.6) as f32,
                    edge as f32,
                    ground as f32,
                    3.0,
                    [1.0, 0.9, 0.4, 0.95],
                );
            }
        }
    }
}

#[cfg(test)]
#[path = "water_weather_tests.rs"]
mod water_weather_tests;
