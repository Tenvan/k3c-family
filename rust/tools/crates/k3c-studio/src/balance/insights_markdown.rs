//! Auswertung als Markdown.
//!
//! Bis RU70.17 stehen hier die Ampel-Zeichen, der Stub [`Summary::insights`] und ein Teil von [`Insights::markdown`].

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
    /// Stub RU70.17 mit Fehler statt Absturz: die Tests rufen [`Summary::markdown`], das bei einem Fehler wie Go ohne
    /// Auswertung schreibt.
    pub fn insights(&self, _base: Option<&Summary>, _meta: Meta) -> Result<Insights, String> {
        Err("Auswertung folgt in RU70.17".to_owned())
    }
}

impl Insights {
    /// Stub RU70.17: bisher nur Ergebnis, Leiter und Blocker. Kein `unimplemented!`, weil
    /// [`super::insights_files::write_reports`] ihn erreicht.
    pub fn markdown(&self) -> String {
        let mut b = String::new();
        self.ladder_markdown(&mut b);
        b
    }
}
