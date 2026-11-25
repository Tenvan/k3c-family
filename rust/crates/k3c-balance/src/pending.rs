//! Platzhalter für Funktionen späterer Sessions (Leitfaden § 7). Die Session, die die Funktion liefert, löscht den
//! Platzhalter. Alle tragen dieselbe Signatur wie die spätere Funktion. Die Dinge hier haben einen Rumpf mit den echten
//! Werten, damit `hints.rs` schon läuft (Ziel-Kennzahlen, Gruppen, Ampel); die liefernde Session ersetzt sie durch ihr Original.

/// RU31.1 (`insights.go`): die Ampel eines Ziels.
pub const LIGHT_GREEN: &str = "green";
/// RU31.1 (`insights.go`).
pub const LIGHT_YELLOW: &str = "yellow";
/// RU31.1 (`insights.go`).
pub const LIGHT_RED: &str = "red";
/// RU31.1 (`insights.go`): grau, ungültig bzw. ohne Messwert; hier, weil `ship_need.rs` und `delta.rs` (RU28.4) sie brauchen.
pub const LIGHT_GREY: &str = "grey";

/// RU31.1 (`insights_markdown.go`, Go: `lightIcon`): das Zeichen der Ampel.
pub fn light_icon(light: &str) -> &'static str {
    match light {
        LIGHT_GREEN => "🟢",
        LIGHT_YELLOW => "🟡",
        LIGHT_RED => "🔴",
        LIGHT_GREY => "⚪",
        _ => "",
    }
}
