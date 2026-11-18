//! Port von `native/k3c-game/fx/texts.go`.
//!
//! Spieltexte der Effekte (Spiegel von src/core/texts.*.ts › cast.noTarget); einzige Stelle mit Text im Paket.

use crate::view::settings::LANGUAGE_ENGLISH;

/// `no_target_text` ist „kein Ziel“ in der Sprache lang (Deutsch für jede andere).
pub fn no_target_text(lang: &str) -> &'static str {
    if lang == LANGUAGE_ENGLISH {
        return "no target";
    }
    "kein Ziel"
}
