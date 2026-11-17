//! Port von `native/k3c-game/audio/register.go`: Slot-Register der Klänge.
//!
//! data/sounds.json (wie src/audio/events.ts › sfxUrls und music.ts › musicTracks) mit den Skill-Linien aus
//! data/monarch.json (src/audio/skills.ts). Dateien sind Pfade unter public/. Eigene kleine Strukturen lesen die
//! Dateien wie in Go (nur `sprite` fehlt in `k3c_data::sounds`), damit die Fehlermeldungen gleich bleiben.
//!
//! Abweichung von Go: `load_register` bekommt statt `fs.ReadFileFS` eine Lesefunktion (Name → Bytes oder Fehlertext).

use std::collections::BTreeMap;

use serde::Deserialize;

use super::duck::{DEFAULT_DUCK, DuckShape};

/// Mischpegel (B-935, Klangbild docs/rules/klang.md): Dämpfung je Bus vor den Einstellungen und Form des Duckings.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mixing {
    pub effects: f64,
    pub music: f64,
    pub duck: DuckShape,
}

impl Mixing {
    /// Null-Wert (Go: `Mixing{}`): ein Register ohne Pegel.
    pub const ZERO: Mixing = Mixing {
        effects: 0.0,
        music: 0.0,
        duck: DuckShape {
            level: 0.0,
            attack: 0.0,
            hold: 0.0,
            release: 0.0,
        },
    };
}

/// Ohne Abschnitt mischung: Busse ungedämpft, Ducking wie vor B-935.
pub const DEFAULT_MIXING: Mixing = Mixing {
    effects: 1.0,
    music: 1.0,
    duck: DEFAULT_DUCK,
};

/// Slot-Linie eines Skills aus monarch.json.
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(default)]
pub struct SkillInfo {
    pub kind: String,
    pub line: String,
}

/// Das Slot-Register der Klänge. `Default` ist der Null-Wert von Go (leer, Null-Pegel).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Register {
    /// Effekt-Slot (Bus sfx) → erste Präferenz ohne Atlas-Sprite
    pub effects: BTreeMap<String, String>,
    /// Musik-Slot (Bus music) → erste Präferenz
    pub music: BTreeMap<String, String>,
    /// Mischpegel aus sounds.json › mischung; Null-Wert = `DEFAULT_MIXING`
    pub mix: Mixing,
    pub skills: BTreeMap<String, SkillInfo>,
}

impl Register {
    /// Die Mischpegel; ein Register ohne Pegel (Null-Wert, z. B. in Tests) nimmt `DEFAULT_MIXING`.
    pub fn mixing(&self) -> Mixing {
        if self.mix == Mixing::ZERO {
            DEFAULT_MIXING
        } else {
            self.mix
        }
    }

    /// Slot zum Wirken eines Skills wie skills.ts › skillSlot: eigener Slot, bei aktiven Skills der Slot der Linie,
    /// sonst der gemeinsame; ein passiver Skill ohne eigenen Slot bleibt still (`None`).
    pub fn skill_slot(&self, id: &str) -> Option<String> {
        let own = format!("sfx:skill:{id}");
        if self.effects.contains_key(&own) {
            return Some(own);
        }
        let s = self.skills.get(id)?;
        if s.kind != "active" {
            return None;
        }
        let line = format!("sfx:skill:{}", s.line);
        if self.effects.contains_key(&line) {
            return Some(line);
        }
        Some(SKILL_FALLBACK.to_string())
    }
}

/// Klingt für aktive Skills ohne eigenen Slot und ohne Slot der Linie (skills.ts › SKILL_FALLBACK).
pub const SKILL_FALLBACK: &str = "sfx:skill";

/// Abschnitt mischung in sounds.json.
#[derive(Default, Deserialize)]
#[serde(default)]
struct MixingFile {
    #[serde(rename = "effekte")]
    effects: f64,
    #[serde(rename = "musik")]
    music: f64,
    #[serde(rename = "duckPegel")]
    duck_level: f64,
    #[serde(rename = "duckAnstieg")]
    duck_attack: f64,
    #[serde(rename = "duckHalten")]
    duck_hold: f64,
    #[serde(rename = "duckAbklingen")]
    duck_release: f64,
}

impl MixingFile {
    /// Prüft die Werte: Pegel in (0, 1], Ducking-Pegel in (0, 1), Rampen > 0, Halten ≥ 0.
    fn parse(&self) -> Result<Mixing, String> {
        let unit = |v: f64| v > 0.0 && v <= 1.0;
        if !unit(self.effects) || !unit(self.music) {
            return Err(format!(
                "❌ data/sounds.json › mischung: effekte {} / musik {} nicht in (0, 1]",
                self.effects, self.music
            ));
        }
        if self.duck_level <= 0.0 || self.duck_level >= 1.0 {
            return Err(format!(
                "❌ data/sounds.json › mischung: duckPegel {} nicht in (0, 1)",
                self.duck_level
            ));
        }
        if self.duck_attack <= 0.0 || self.duck_release <= 0.0 || self.duck_hold < 0.0 {
            return Err(format!(
                "❌ data/sounds.json › mischung: Ducking-Zeiten {} / {} / {} ungültig",
                self.duck_attack, self.duck_hold, self.duck_release
            ));
        }
        Ok(Mixing {
            effects: self.effects,
            music: self.music,
            duck: DuckShape {
                level: self.duck_level,
                attack: self.duck_attack,
                hold: self.duck_hold,
                release: self.duck_release,
            },
        })
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct ChoiceFile {
    #[serde(rename = "datei")]
    file: String,
    sprite: String,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct SlotFile {
    id: String,
    #[serde(rename = "art")]
    kind: String,
    bus: String,
    #[serde(rename = "wahl")]
    choices: Vec<ChoiceFile>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct SoundsFile {
    #[serde(rename = "mischung")]
    mix: Option<MixingFile>,
    slots: Vec<SlotFile>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct MonarchFile {
    skills: BTreeMap<String, SkillInfo>,
}

/// Liest sounds.json und monarch.json über `read` (Name unter data/ → Bytes oder Fehlertext).
pub fn load_register(read: &dyn Fn(&str) -> Result<Vec<u8>, String>) -> Result<Register, String> {
    let sounds = read("sounds.json").map_err(|e| format!("❌ Slot-Register data/sounds.json: {e}"))?;
    let monarch = read("monarch.json").map_err(|e| format!("❌ Skills data/monarch.json: {e}"))?;
    parse_register(&sounds, &monarch)
}

/// Baut das Register aus den Inhalten von sounds.json und monarch.json.
pub fn parse_register(sounds: &[u8], monarch: &[u8]) -> Result<Register, String> {
    let sf: SoundsFile = serde_json::from_slice(sounds).map_err(|e| format!("❌ data/sounds.json: {e}"))?;
    let mf: MonarchFile = serde_json::from_slice(monarch).map_err(|e| format!("❌ data/monarch.json: {e}"))?;
    let mix = match &sf.mix {
        Some(m) => m.parse()?,
        None => DEFAULT_MIXING,
    };
    let mut r = Register {
        mix,
        skills: mf.skills,
        ..Register::default()
    };
    for s in sf.slots.iter().filter(|s| s.kind == "sound") {
        match s.bus.as_str() {
            "sfx" => {
                if let Some(w) = s.choices.iter().find(|w| w.sprite.is_empty() && !w.file.is_empty()) {
                    r.effects.insert(s.id.clone(), w.file.clone());
                }
            }
            "music" => {
                if let Some(w) = s.choices.first().filter(|w| !w.file.is_empty()) {
                    r.music.insert(s.id.clone(), w.file.clone());
                }
            }
            _ => {}
        }
    }
    Ok(r)
}
