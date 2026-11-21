//! RU55.6 (B-1047): Wald-Dichte und Parallax-Satz je Welt im Look-Profil (`data/look.json` › forest, worlds).
//!
//! Die Walddichte (Bäume im Umkreis der Kamera) macht das Bild dunkler und dichter; je Welt (Level-ID) kann ein
//! eigener Parallax-Satz (Tempo, Dunst, Platz des Leitmotivs) stehen. Welten ohne Eintrag nutzen den Standard.

use serde::{Deserialize, Serialize};

use crate::look::{Look, TintColor};

/// Wald nach Rodung (`.profiles.*.forest`): Dichte aus der Zahl der Bäume im Umkreis, weich über die Zeit.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Forest {
    pub enabled: bool,
    /// Umkreis um die Kamera in Units.
    pub radius: f64,
    /// Zahl Bäume im Umkreis, ab der der Wald voll dicht ist (Dichte 1).
    pub full: f64,
    /// Zeitkonstante der Glättung in Sekunden (kein Springen beim Fällen).
    pub smooth: f64,
    /// Deckkraft der Waldwand (mittlere und nahe Schicht) bei Dichte 0, also gerodet.
    pub open_alpha: f64,
    /// Tönung bei Dichte 1 (dunkles Oliv); skaliert mit der Dichte.
    pub tint: TintColor,
    /// Zusatz für die Höfe der Fackeln bei Dichte 1 (Faktor `1 + fire`): im Wald kommt das Licht von der Fackel.
    pub fire: f64,
}

/// Aus: Standardwerte, wie sie `data/look.json` für k2c nennt; nur `enabled` ist false.
pub const FOREST_OFF: Forest = Forest {
    enabled: false,
    radius: 14.0,
    full: 10.0,
    smooth: 1.5,
    open_alpha: 0.35,
    tint: TintColor {
        color: [0.7, 0.85, 0.55],
        strength: 0.55,
    },
    fire: 0.5,
};

impl Default for Forest {
    fn default() -> Self {
        FOREST_OFF
    }
}

/// Leitmotiv einer Welt (RU55.6): Lage als Anteil der Zellenbreite, Größe in M (Monarchenhöhen, 3 … 5).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Motif {
    pub x: f64,
    pub size: f64,
}

const MOTIF_DEFAULT: Motif = Motif { x: 0.7, size: 4.0 };

impl Default for Motif {
    fn default() -> Self {
        MOTIF_DEFAULT
    }
}

/// Parallax-Satz einer Welt (`.profiles.*.worlds.<level>`); fehlende Felder: Standard der Kulisse.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WorldLook {
    /// Wald-Wirkung (Dichte, Waldwand) gilt in dieser Welt.
    pub forest: bool,
    /// Scroll-Faktoren fern, mitte, nah statt der Biom-Werte.
    pub scroll: Option<[f64; 3]>,
    /// Dunst nach fern, mitte, nah statt des Profil-Standards.
    pub haze: Option<[f64; 3]>,
    pub motif: Motif,
}

/// Ohne Eintrag: keine Wald-Wirkung, keine eigenen Werte.
pub static NO_WORLD: WorldLook = WorldLook {
    forest: false,
    scroll: None,
    haze: None,
    motif: MOTIF_DEFAULT,
};

impl Look {
    /// Parallax-Satz der Welt `level`; ohne Eintrag der Standard.
    pub fn world(&self, level: &str) -> &WorldLook {
        self.worlds.get(level).unwrap_or(&NO_WORLD)
    }

    /// Dunst nach Schicht `i` für `level`: der Satz der Welt, sonst der Standard des Profils.
    pub fn haze_for(&self, level: &str, i: usize) -> f64 {
        let set = self.world(level).haze.unwrap_or(self.haze);
        set.get(i).copied().unwrap_or(0.0)
    }

    /// Scroll-Faktor der Schicht `i` für `level`: der Satz der Welt, sonst `default` (aus dem Biom).
    pub fn scroll_for(&self, level: &str, i: usize, default: f64) -> f64 {
        self.world(level)
            .scroll
            .and_then(|s| s.get(i).copied())
            .unwrap_or(default)
    }
}

use std::collections::BTreeMap;

/// Parallax-Sätze des Profils k2c (Werte wie `data/look.json`).
pub fn builtin_worlds() -> BTreeMap<String, crate::look_world::WorldLook> {
    let forest = WorldLook {
        forest: true,
        haze: Some([0.45, 0.3, 0.05]),
        motif: Motif { x: 0.72, size: 4.5 },
        ..WorldLook::default()
    };
    let crystal = WorldLook {
        haze: Some([0.2, 0.1, 0.0]),
        motif: Motif { x: 0.7, size: 4.0 },
        ..WorldLook::default()
    };
    BTreeMap::from([("forest".to_string(), forest), ("crystal".to_string(), crystal)])
}

#[cfg(test)]
#[path = "look_world_tests.rs"]
mod look_world_tests;
