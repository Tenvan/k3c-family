//! data/story.json: Go `sim.StoryLimits` (engine/sim/story.go), Obergrenzen der Geschichte.

use serde::{Deserialize, Serialize};

use crate::{Fehler, decode};

/// Go: `sim.StoryLimits`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StoryLimits {
    #[serde(rename = "maxLineChars")]
    pub max_line_chars: i64,
    #[serde(rename = "maxLexicon")]
    pub max_lexicon: i64,
}

/// `FILE` ist der Name unter data/.
pub const FILE: &str = "story.json";

/// `load` liest story.json. Go überliest dort jeden Fehler (`loadStoryLimits`: das Spiel startet ohne Grenzen);
/// hier meldet `load` ihn, der Aufrufer entscheidet.
pub fn load(raw: &str) -> Result<StoryLimits, Fehler> {
    decode("data/story.json", raw)
}

impl StoryLimits {
    /// `usable`: beide Obergrenzen sind gesetzt.
    pub fn usable(&self) -> bool {
        self.max_line_chars > 0 && self.max_lexicon > 0
    }
}
