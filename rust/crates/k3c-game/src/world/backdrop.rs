//! Port von `native/k3c-game/world/backdrop.go`.
//!
//! Kulisse je Biom (B-795/AC-02): drei Schichten aus Papier-Kacheln `hintergruende/hg-<teil>-<fern|mitte|nah>-v<n>`
//! (Browser: src/scenes/render/worldSprites.ts › BACKGROUND_LAYERS, paperAssets.ts › paperBackgroundFile), Farben aus
//! data/biomes/*.json › palette. Seit NV3 steht `fern` fest (Faktor 0, native/k3c-ebiten/level.go); `mitte` und `nah`
//! scrollen mit den Faktoren des Browsers. Variante je Kachel aus World.Level.Backdrops (Server, nach Seed).

use std::collections::BTreeMap;

use k3c_data::{Fehler, Source, data};
use serde::Deserialize;

/// Farbe RGBA (Go: `color.RGBA`), gerade, Alpha 255.
pub type Rgba = [u8; 4];

/// TILE_UNITS ist die Breite einer Kulissen-Kachel in Units (engine/level › BackdropTileUnits, Regeldatei § 10).
pub const TILE_UNITS: f64 = 24.0;
/// FRAME_PER_UNIT sind Rahmen-Einheiten der Papier-Manifeste je Unit (32 px je Unit, BACKGROUND_DENSITY).
pub const FRAME_PER_UNIT: f64 = 32.0;
/// LAYER_COUNT ist die Zahl der Kulissen-Schichten fern, mitte, nah.
pub const LAYER_COUNT: usize = 3;

/// LAYER_NAMES sind die Namensteile der Schichten von hinten nach vorn.
pub const LAYER_NAMES: [&str; LAYER_COUNT] = ["fern", "mitte", "nah"];

/// BIOME_LAYERS: je Biom (ID, Namensteil der Assets, Scroll-Faktoren fern → nah; fern fest wie NV3).
pub const BIOME_LAYERS: [(&str, &str, [f64; LAYER_COUNT]); 5] = [
    ("forest", "wald", [0.0, 0.35, 0.5]),
    ("cave", "hoehle", [0.0, 0.2, 0.4]),
    ("mine", "mine", [0.0, 0.25, 0.45]),
    ("ironhold", "eisenstollen", [0.0, 0.25, 0.45]),
    ("crystal", "kristall", [0.0, 0.25, 0.45]),
];

/// Rückfall-Farben ohne Palette (wie NV3).
pub const DEFAULT_SKY: Rgba = [158, 199, 219, 255];
pub const DEFAULT_GROUND: Rgba = [74, 56, 36, 255];

/// Look ist das Aussehen eines Bioms: Farben, Scroll-Faktoren und die Asset-Namen je Schicht und Variante (fest
/// angelegt, damit das Zeichnen keine Namen baut).
#[derive(Clone, Debug, PartialEq)]
pub struct Look {
    pub sky: Rgba,
    pub ground: Rgba,
    pub scroll: [f64; LAYER_COUNT],
    /// `[Schicht][Variante − 1]` = `gruppe/id`
    pub assets: [Vec<String>; LAYER_COUNT],
}

impl Look {
    /// Ein leeres Aussehen mit Rückfall-Farben (Go: `fallbackLook`).
    pub fn fallback() -> Look {
        Look {
            sky: DEFAULT_SKY,
            ground: DEFAULT_GROUND,
            scroll: [0.0; LAYER_COUNT],
            assets: [Vec::new(), Vec::new(), Vec::new()],
        }
    }
}

/// Looks sind die Aussehen je Biom-ID (= Level).
#[derive(Clone, Debug)]
pub struct Looks {
    map: BTreeMap<String, Look>,
    fallback: Look,
}

/// Palette ist der Ausschnitt aus data/biomes/*.json, den die Welt braucht.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct Palette {
    #[serde(rename = "sky")]
    pub sky: String,
    #[serde(rename = "ground")]
    pub ground: String,
}

/// LoadLooks liest Paletten (data/biomes) und Varianten (data/paper.json › hintergruende); Fehler stehen im Log, die
/// Welt zeichnet dann mit Rückfall-Farben bzw. ohne Kulisse.
pub fn load_looks() -> Looks {
    let src = Source::from_env();
    let (group, variants, err) = backdrop_group(&src);
    if let Some(e) = err {
        k3c_platform::log::error("❌ Kulissen-Daten nicht lesbar", &[("err", &e)]);
    }
    let (palettes, err) = read_palettes(&src);
    if let Some(e) = err {
        k3c_platform::log::error("❌ Paletten nicht lesbar", &[("err", &e)]);
    }
    build_looks(&group, variants, &palettes)
}

/// BuildLooks baut die Aussehen aller Biome mit Palette oder Kulisse.
pub fn build_looks(group: &str, variants: i64, palettes: &BTreeMap<String, Palette>) -> Looks {
    let mut out = Looks {
        map: BTreeMap::new(),
        fallback: Look::fallback(),
    };
    for (id, p) in palettes {
        let l = out.map.entry(id.clone()).or_insert_with(Look::fallback);
        l.sky = parse_hex(&p.sky, DEFAULT_SKY);
        l.ground = parse_hex(&p.ground, DEFAULT_GROUND);
    }
    for (id, part, scroll) in BIOME_LAYERS {
        let l = out.map.entry(id.to_string()).or_insert_with(Look::fallback);
        l.scroll = scroll;
        for (i, name) in LAYER_NAMES.iter().enumerate() {
            for v in 1..=variants {
                l.assets[i].push(format!("{group}/hg-{part}-{name}-v{v}"));
            }
        }
    }
    out
}

impl Looks {
    /// Assets nennt alle Kulissen-Assets des Levels (alle Varianten aller Schichten); unbekanntes Level: keine
    /// (Go: `nil`, hier `None`).
    pub fn assets(&self, level: &str) -> Option<Vec<String>> {
        let l = self.map.get(level)?;
        let mut out: Vec<String> = Vec::new();
        for a in &l.assets {
            out.extend(a.iter().cloned());
        }
        Some(out)
    }

    /// Of liefert das Aussehen des Levels, sonst ein leeres mit Rückfall-Farben.
    pub fn of(&self, level: &str) -> &Look {
        self.map.get(level).unwrap_or(&self.fallback)
    }
}

/// Variant liefert die Variante (1 …) der Kachel k aus der Folge backdrops (wiederholt, auch für negative k); ohne
/// Folge 1. Mehr Varianten, als es Dateien gibt (n), laufen reihum (wie paperBackgroundFile).
pub fn variant(backdrops: &[i64], k: i64, n: i64) -> i64 {
    if backdrops.is_empty() || n <= 0 {
        return 1;
    }
    let m = backdrops.len() as i64;
    let v = backdrops[(((k % m) + m) % m) as usize].max(1);
    (v - 1) % n + 1
}

/// Datei-Aufbau von data/paper.json, soweit die Kulisse ihn braucht.
#[derive(Default, Deserialize)]
#[serde(default)]
struct PaperBackdrops {
    #[serde(rename = "hintergruende")]
    backdrops: Backdrops,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Backdrops {
    #[serde(rename = "gruppe")]
    group: String,
    #[serde(rename = "varianten")]
    variants: i64,
}

/// BackdropGroup liest Gruppe und Zahl der Varianten der Kulissen aus data/paper.json; wie in Go kommt neben dem
/// Fehler immer ein Wert zurück (Rückfall `hintergruende`, 1).
pub fn backdrop_group(src: &Source) -> (String, i64, Option<Fehler>) {
    let fallback = "hintergruende".to_string();
    let raw = match src.read("paper.json") {
        Ok(r) => r,
        Err(e) => return (fallback, 1, Some(e)),
    };
    match serde_json::from_str::<PaperBackdrops>(&raw) {
        Ok(p) if !p.backdrops.group.is_empty() && p.backdrops.variants >= 1 => {
            (p.backdrops.group, p.backdrops.variants, None)
        }
        Ok(_) => (
            fallback,
            1,
            Some(Fehler::Pruefung("paper.json › hintergruende: <nil>".into())),
        ),
        Err(e) => (
            fallback,
            1,
            Some(Fehler::Pruefung(format!("paper.json › hintergruende: {e}"))),
        ),
    }
}

/// Datei-Aufbau von data/biomes/*.json, soweit die Welt ihn braucht.
#[derive(Default, Deserialize)]
#[serde(default)]
struct BiomePalette {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "palette")]
    palette: Palette,
}

/// ReadPalettes liest die Paletten aller Biome aus data/biomes/*.json; bei einem Fehler kommt wie in Go das bisher
/// Gelesene mit dem Fehler zurück.
pub fn read_palettes(src: &Source) -> (BTreeMap<String, Palette>, Option<Fehler>) {
    let mut out = BTreeMap::new();
    let ids = match data::biome_ids(src) {
        Ok(ids) => ids,
        Err(e) => return (out, Some(e)),
    };
    for id in ids {
        let name = format!("biomes/{id}.json");
        let raw = match src.read(&name) {
            Ok(r) => r,
            Err(e) => return (out, Some(e)),
        };
        match serde_json::from_str::<BiomePalette>(&raw) {
            Ok(b) => {
                out.insert(b.id, b.palette);
            }
            Err(e) => return (out, Some(Fehler::Json(format!("{name}: {e}")))),
        }
    }
    (out, None)
}

/// ParseHex liest eine Farbe `#rrggbb`; sonst def.
pub fn parse_hex(s: &str, def: Rgba) -> Rgba {
    let b = s.as_bytes();
    if b.len() != 7 || b[0] != b'#' {
        return def;
    }
    // Go: strconv.ParseUint(s[1:], 16, 32) lässt kein Vorzeichen zu; from_str_radix nähme ein führendes `+`.
    if !s[1..].bytes().all(|c| c.is_ascii_hexdigit()) {
        return def;
    }
    let Ok(v) = u32::from_str_radix(&s[1..], 16) else {
        return def;
    };
    [(v >> 16) as u8, (v >> 8) as u8, v as u8, 255]
}
