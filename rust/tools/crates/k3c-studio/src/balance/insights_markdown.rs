//! Auswertung als Markdown.
//!
//! Bis RU70.17 stehen hier die Ampel-Zeichen und zwei Stubs für [`Summary::markdown`].

use super::insights::{Insights, LIGHT_GREEN, LIGHT_GREY, LIGHT_RED, LIGHT_YELLOW, Meta};
use super::summary::Summary;

/// Das Zeichen zur Ampel, leer bei unbekanntem Wert.
pub(crate) fn light_icon(light: &str) -> &'static str {
    match light {
        LIGHT_GREEN => "🟢",
        LIGHT_YELLOW => "🟡",
        LIGHT_RED => "🔴",
        LIGHT_GREY => "⚪",
        _ => "",
    }
}

impl Summary {
    /// Auswertung mit den Richtwerten.
    ///
    /// Stub statt `unimplemented!("RU70.17")`: die Tests dieser Session rufen [`Summary::markdown`], das bei einem
    /// Fehler wie Go ohne Auswertung schreibt. RU70.17 ersetzt ihn.
    pub fn insights(&self, _base: Option<&Summary>, _meta: Meta) -> Result<Insights, String> {
        Err("Auswertung folgt in RU70.17".to_owned())
    }
}

impl Insights {
    /// Nur erreichbar, wenn [`Summary::insights`] eine Auswertung liefert.
    #[allow(clippy::unimplemented)] // V11: Stub bis RU70.17, kein Test dieser Session ruft ihn auf.
    pub fn markdown(&self) -> String {
        unimplemented!("RU70.17")
    }
}
