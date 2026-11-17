//! Bilder der Welten (B-937/B-938): `data/paper.json › weltbilder` ordnet je Welt-ID ein großes Titelbild
//! (Weltauswahl) und ein kleines Bild (Spielstand-Liste, Pause-Menü) zu, je als Asset `gruppe/id`. Der Weltname liegt
//! nicht im Bild, das Spiel schreibt ihn darüber. Hat eine Welt keinen Eintrag, gilt der von `standard`.
//! Vorlage: `native/k3c-game/art/worldimages.go`.

use std::collections::BTreeMap;

use serde::Deserialize;

/// Die Welt, deren Bilder (und Seekarten-Miniatur) als Ersatz gelten.
pub const STANDARD_WORLD: &str = "standard";

/// Die beiden Assets einer Welt.
#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct WorldImage {
    #[serde(default, rename = "gross")]
    pub big: String,
    #[serde(default, rename = "klein")]
    pub small: String,
}

/// Zuordnung Welt-ID → Bilder.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldImages(pub BTreeMap<String, WorldImage>);

impl WorldImages {
    /// Liest den Abschnitt `weltbilder` aus dem Inhalt von paper.json; er muss `standard` mit beiden Bildern enthalten.
    pub fn parse(raw: &[u8]) -> Result<WorldImages, String> {
        #[derive(Deserialize)]
        struct P {
            #[serde(default, rename = "weltbilder")]
            images: BTreeMap<String, WorldImage>,
        }
        let p: P = serde_json::from_slice(raw).map_err(|e| format!("paper.json › weltbilder: {e}"))?;
        for (id, w) in &p.images {
            if w.big.is_empty() || w.small.is_empty() {
                return Err(format!("paper.json › weltbilder.{id}: gross und klein nötig"));
            }
        }
        if !p.images.contains_key(STANDARD_WORLD) {
            return Err(format!("paper.json › weltbilder: Eintrag {STANDARD_WORLD:?} fehlt"));
        }
        Ok(WorldImages(p.images))
    }

    /// Liest die Zuordnung aus data/paper.json; ein Fehler steht im Log und liefert eine leere Zuordnung (Menüs ohne
    /// Weltbilder).
    pub fn from_data() -> WorldImages {
        super::datafile::read("paper.json")
            .and_then(|raw| Self::parse(&raw))
            .unwrap_or_else(|e| {
                k3c_platform::log::error("❌ Weltbilder nicht lesbar", &[("err", &e)]);
                WorldImages::default()
            })
    }

    /// Bilder der Welt `id`, ersatzweise die der Standard-Welt (leer, wenn es auch die nicht gibt).
    pub fn of(&self, id: &str) -> WorldImage {
        self.0
            .get(id)
            .or_else(|| self.0.get(STANDARD_WORLD))
            .cloned()
            .unwrap_or_default()
    }

    /// Bilder aller Welten: die kleinen, mit `big` auch die großen (evtl. doppelt).
    pub fn assets(&self, big: bool) -> Vec<String> {
        let mut out = Vec::with_capacity(2 * self.0.len());
        for i in self.0.values() {
            out.push(i.small.clone());
            if big {
                out.push(i.big.clone());
            }
        }
        out
    }
}
