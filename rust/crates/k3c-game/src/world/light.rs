//! Port von `native/k3c-game/world/light.go`.
//!
//! Licht bei Tag und Nacht (B-795/AC-03), Regeln wie src/scenes/render/nightLight.ts und viewRules.ts › daylight:
//! Helligkeit aus der Phase des Zyklus (sim.World.Cycle) mit den Untergrenzen aus data/light.json, Lichtquellen
//! (Lagerfeuer an der Burg auf Hub-Stufe 1, Fackeln an gebauter Mauer und gebautem Turm) heben sie um ihre Mitte an.
//! Reine Rechnung, ohne Allokation je Aufruf.

use std::collections::BTreeMap;

use k3c_data::Source;
use k3c_det::{fmax, fmin};
use k3c_sim::state::{CycleInfo, World as SimWorld};
use serde::Deserialize;

use super::backdrop::{Rgba, parse_hex};

/// WHITE ist die Rückfall-Farbe des Lichtscheins.
pub const WHITE: Rgba = [255, 255, 255, 255];

/// LightSource sind die Werte einer Lichtquellen-Art: Reichweite in Units und Stärke in der Mitte.
#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct LightSource {
    #[serde(rename = "radius")]
    pub radius: f64,
    #[serde(rename = "strength")]
    pub strength: f64,
}

/// LightData sind die Werte aus data/light.json.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(default)]
pub struct LightData {
    #[serde(rename = "nightBrightness")]
    pub night: f64,
    #[serde(rename = "backgroundBrightness")]
    pub back: f64,
    #[serde(rename = "glowColor")]
    pub glow_color: String,
    #[serde(rename = "sources")]
    pub sources: BTreeMap<String, LightSource>,
    #[serde(rename = "torches")]
    pub torches: BTreeMap<String, Vec<f64>>,

    /// Farbe des Lichtscheins 0 … 1
    #[serde(skip)]
    pub glow: [f64; 3],
    #[serde(skip)]
    pub campfire: LightSource,
    #[serde(skip)]
    pub torch: LightSource,
}

impl Default for LightData {
    /// Ohne Datei: weder Nacht noch Licht (Go: `lightData{Night: 1, Back: 1}`).
    fn default() -> Self {
        LightData {
            night: 1.0,
            back: 1.0,
            glow_color: String::new(),
            sources: BTreeMap::new(),
            torches: BTreeMap::new(),
            glow: [0.0; 3],
            campfire: LightSource::default(),
            torch: LightSource::default(),
        }
    }
}

/// LoadLight liest data/light.json; ein Fehler steht im Log, dann gibt es weder Nacht noch Licht.
pub fn load_light() -> LightData {
    let res = Source::from_env()
        .read("light.json")
        .map_err(|e| e.to_string())
        .and_then(|raw| serde_json::from_str::<LightData>(&raw).map_err(|e| e.to_string()));
    let mut d = match res {
        Ok(d) => d,
        Err(e) => {
            k3c_platform::log::error("❌ data/light.json nicht lesbar, Nacht bleibt hell", &[("err", &e)]);
            LightData::default()
        }
    };
    d.prepare();
    d
}

/// Daylight ist die Helligkeit der Phase: 1 = Tag, 0 = tiefste Nacht; Dämmerung fällt, Morgengrauen steigt linear,
/// unbekannte Phasen gelten als Nacht.
pub fn daylight(c: &CycleInfo) -> f64 {
    match c.phase.as_str() {
        "day" => 1.0,
        "dusk" => 1.0 - c.progress,
        "dawn" => c.progress,
        _ => 0.0,
    }
}

/// BrightnessOf ist die Helligkeit mit Nacht-Untergrenze night: night + (1 − night)·daylight.
pub fn brightness_of(c: &CycleInfo, night: f64) -> f64 {
    night + (1.0 - night) * daylight(c)
}

/// GlowAlpha ist die Deckkraft eines Lichtscheins der Stärke strength: am Tag 0, nachts die Stärke (0 … 1).
pub fn glow_alpha(strength: f64, c: &CycleInfo) -> f64 {
    fmin(1.0, fmax(0.0, strength)) * (1.0 - daylight(c))
}

/// Falloff ist der Anteil eines Lichts mit Mitte sx und Reichweite radius am Ort x: 1 in der Mitte, linear 0 am Rand.
pub fn falloff(x: f64, sx: f64, radius: f64) -> f64 {
    if radius <= 0.0 {
        return 0.0;
    }
    fmax(0.0, 1.0 - (x - sx).abs() / radius)
}

impl LightSource {
    /// Valid meldet, ob die Quelle Licht gibt (Werte in data/ vorhanden).
    pub fn valid(&self) -> bool {
        self.radius > 0.0 && self.strength > 0.0
    }
}

/// HasCampfire meldet, ob die Burg der Welt das Lagerfeuer ist (Hub-Stufe 1, src/scenes/render/hubGrowth.ts).
pub fn has_campfire(w: &SimWorld) -> bool {
    w.hub_level <= 1
}

/// TorchSide ist die Richtung der Fackel-Versätze: von der Hub-Mitte weg, in der Mitte nach rechts (engine/sim/torches.go).
pub fn torch_side(x: f64, hub_x: f64) -> f64 {
    if x < hub_x { -1.0 } else { 1.0 }
}

/// SkyLift ist die Aufhellung der tiefsten Nacht aus dem Look-Profil (LA1.5); ohne Himmel-Profil 0.
fn sky_lift(back: bool) -> f64 {
    let s = k3c_data::look::current().sky;
    match (s.enabled, back) {
        (false, _) => 0.0,
        (true, false) => s.night_lift,
        (true, true) => s.back_lift,
    }
}

impl LightData {
    /// Prepare legt die abgeleiteten Werte an (Farbe, Quellen-Arten).
    pub fn prepare(&mut self) {
        let g = parse_hex(&self.glow_color, WHITE);
        self.glow = [
            f64::from(g[0]) / 255.0,
            f64::from(g[1]) / 255.0,
            f64::from(g[2]) / 255.0,
        ];
        self.campfire = self.sources.get("campfire").copied().unwrap_or_default();
        self.torch = self.sources.get("torch").copied().unwrap_or_default();
    }

    /// Front ist die Helligkeit des Vordergrunds außerhalb des Lichts.
    pub fn front(&self, c: &CycleInfo) -> f64 {
        brightness_of(c, (self.night + sky_lift(false)).min(1.0))
    }

    /// Backdrop ist die Helligkeit von Himmel und Kulisse (nachts dunkler als der Vordergrund).
    pub fn backdrop(&self, c: &CycleInfo) -> f64 {
        brightness_of(c, (self.back + sky_lift(true)).min(1.0))
    }

    /// Lift ist die Summe der Lichtscheine am Ort x (vor der Begrenzung auf 1): je Quelle glowAlpha × falloff.
    pub fn lift(&self, x: f64, w: &SimWorld) -> f64 {
        let mut add = 0.0;
        if self.campfire.valid() && has_campfire(w) {
            add += glow_alpha(self.campfire.strength, &w.cycle) * falloff(x, w.castle.x, self.campfire.radius);
        }
        if !self.torch.valid() {
            return add;
        }
        let a = glow_alpha(self.torch.strength, &w.cycle);
        if a <= 0.0 {
            return add;
        }
        for s in &w.sites {
            if s.state != "built" {
                continue;
            }
            let side = torch_side(s.x, w.hub_x);
            for o in self.torches.get(&s.kind).into_iter().flatten() {
                add += a * falloff(x, s.x + side * o, self.torch.radius);
            }
        }
        add
    }

    /// LevelAt ist die Helligkeit des Vordergrunds am Ort x (grau, 0 … 1) wie lightLevelAt im Browser.
    pub fn level_at(&self, x: f64, w: &SimWorld) -> f64 {
        let base = self.front(&w.cycle);
        fmin(1.0, base + (1.0 - base) * self.lift(x, w))
    }

    /// ColorAt ist die Lichtfarbe am Ort x je Kanal (Multiplikator 0 … 1): Grundhelligkeit plus Schein in glowColor.
    pub fn color_at(&self, x: f64, w: &SimWorld) -> (f64, f64, f64) {
        let base = self.front(&w.cycle);
        let l = (1.0 - base) * self.lift(x, w);
        (
            fmin(1.0, base + l * self.glow[0]),
            fmin(1.0, base + l * self.glow[1]),
            fmin(1.0, base + l * self.glow[2]),
        )
    }
}

#[cfg(test)]
#[path = "light_tests.rs"]
mod light_tests;
