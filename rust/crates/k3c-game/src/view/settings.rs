//! Port von `native/k3c-game/view/settings.go`.
//!
//! Abweichung bis RU7.4: Das Feld `Stress` (`StressTest`, `view/stress.go`) fehlt noch; RU7.4 portiert `stress.go`
//! und ergänzt Feld, Standardwert und Normalisierung hier (Ticket B-1025).

use super::profile::{Controls, default_controls, normalize_controls};

// Sprachen der Einstellungen (wie src/core/settings.ts); DefaultLanguage gilt für jeden anderen Wert.
pub const DEFAULT_LANGUAGE: &str = "de";
pub const LANGUAGE_ENGLISH: &str = "en";

// Fassungen des Flusses (Settings.River, B-816); world kennt keine weiteren.
/// Fluss mit Kage-Shader
pub const RIVER_SHADER: &str = "shader";
/// Fluss aus Wellen-Kacheln
pub const RIVER_TILES: &str = "tiles";

// Anzeige-Modi (Settings.Window, B-826): "" ist das Fenster mit Rahmen, der Null-Wert und Standard.
/// Fenster mit Rahmen
pub const WINDOW_FRAMED: &str = "";
/// Fenster ohne Rahmen in Größe des Monitors
pub const WINDOW_BORDERLESS: &str = "rahmenlos";
/// Vollbild
pub const WINDOW_FULLSCREEN: &str = "vollbild";

/// WindowModes sind die Anzeige-Modi in der Reihenfolge der Optionen.
pub const WINDOW_MODES: [&str; 3] = [WINDOW_FRAMED, WINDOW_BORDERLESS, WINDOW_FULLSCREEN];

/// Settings sind die Einstellungen am Gerät (Optionen-Menü, B-800): Session.SetSettings ändert sie, local speichert
/// sie dauerhaft neben den Spielständen, Frame.Settings zeigt den aktuellen Stand. game wendet den Anzeige-Modus an
/// (nicht im Aufnahme-Modus), audio die Lautstärken, hud und menu die Sprache.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Settings {
    /// Lautstärke der Musik 0 … 1
    pub music: f64,
    /// Lautstärke der Klang-Effekte 0 … 1
    pub effects: f64,
    /// Anzeige-Modus: WindowFramed, WindowBorderless oder WindowFullscreen
    pub window: String,
    /// DefaultLanguage oder LanguageEnglish
    pub language: String,
    /// River ist die Fassung des Flusses (F3, B-816): "" = Startwert (K3C_RIVER, sonst Shader), sonst RiverShader oder
    /// RiverTiles. Kein Eintrag im Optionen-Menü; local speichert nur einen gewählten Wert.
    pub river: String,
    /// Stats zeigt Bildrate und Rechenzeit (Update/Draw in ms) in StatsRectLines (F2, B-816).
    pub stats: bool,
    /// FlashesOff schaltet die Bildschirmblitze ab (Optionen › Blitze, B-809; fx). Der Null-Wert ist der Standard „an“.
    pub flashes_off: bool,
    /// GuideOff schaltet die Hinweise der geführten ersten Nacht ab (Optionen › Anleitung, B-809; hud); Null-Wert = an.
    pub guide_off: bool,
    /// GuideSeen sind die erledigten Hinweise der geführten ersten Nacht als Bitmaske (hud vergibt die Bits und meldet
    /// neue über Session.SetSettings); 0 = nichts gesehen. Das Optionen-Menü setzt sie beim Einschalten der Anleitung
    /// zurück.
    pub guide_seen: i64,
    /// Controls ist das Eingabe-Profil je Spieler (ProfileWASD, ProfileArrows, ProfileGamepad; B-893, profile.go);
    /// höchstens einer hat ein Tastatur-Profil.
    pub controls: Controls,
    /// DevMode zeigt im Hauptmenü die Karte Testing (Optionen › Entwicklermodus, B-896); im Entwickler-Build ist sie
    /// immer da (menu). Null-Wert = aus.
    pub dev_mode: bool,
    // Stress (StressTest) ergänzt RU7.4 (B-1025).
}

/// DefaultSettings sind die Werte ohne gespeicherte Einstellungen (wie der Browser: volle Lautstärke, Deutsch, Fenster).
pub fn default_settings() -> Settings {
    Settings {
        music: 1.0,
        effects: 1.0,
        language: DEFAULT_LANGUAGE.into(),
        controls: default_controls(),
        ..Default::default()
    }
}

impl Settings {
    /// Normalized begrenzt die Lautstärken auf 0 … 1 (NaN = Standard), setzt eine unbekannte Sprache auf
    /// DefaultLanguage, eine unbekannte Fluss-Fassung auf "", negative GuideSeen auf 0 und die Eingabe-Profile auf
    /// gültige (profile.go). SetSettings und das Laden wenden es an.
    pub fn normalized(&self) -> Settings {
        let d = default_settings();
        let mut s = self.clone();
        (s.music, s.effects) = (volume(s.music, d.music), volume(s.effects, d.effects));
        if s.language != LANGUAGE_ENGLISH {
            s.language = DEFAULT_LANGUAGE.into();
        }
        if s.river != RIVER_SHADER && s.river != RIVER_TILES {
            s.river = String::new();
        }
        if s.window != WINDOW_BORDERLESS && s.window != WINDOW_FULLSCREEN {
            s.window = WINDOW_FRAMED.into();
        }
        s.guide_seen = s.guide_seen.max(0);
        s.controls = normalize_controls(s.controls);
        s
    }

    /// NextRiver liefert die andere Fassung des Flusses: ausgehend von River bzw. bei "" von start (der Startwert der
    /// Welt, RiverShader oder RiverTiles).
    pub fn next_river(&self, start: &str) -> &'static str {
        let cur = if self.river.is_empty() { start } else { &self.river };
        if cur == RIVER_TILES {
            return RIVER_SHADER;
        }
        RIVER_TILES
    }
}

fn volume(v: f64, def: f64) -> f64 {
    if v.is_nan() {
        return def;
    }
    v.clamp(0.0, 1.0)
}

#[cfg(test)]
#[path = "settings_tests.rs"]
mod settings_tests;
