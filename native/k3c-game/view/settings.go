package view

import "math"

// Sprachen der Einstellungen (wie src/core/settings.ts); DefaultLanguage gilt für jeden anderen Wert.
const (
	DefaultLanguage = "de"
	LanguageEnglish = "en"
)

// Settings sind die Einstellungen am Gerät (Optionen-Menü, B-800): Session.SetSettings ändert sie, local speichert
// sie dauerhaft neben den Spielständen, Frame.Settings zeigt den aktuellen Stand. game wendet Vollbild an (nicht im
// Aufnahme-Modus), audio die Lautstärken, hud und menu die Sprache.
type Settings struct {
	Music      float64 // Lautstärke der Musik 0 … 1
	Effects    float64 // Lautstärke der Klang-Effekte 0 … 1
	Fullscreen bool    // Vollbild statt Fenster
	Language   string  // DefaultLanguage oder LanguageEnglish
}

// DefaultSettings sind die Werte ohne gespeicherte Einstellungen (wie der Browser: volle Lautstärke, Deutsch, Fenster).
func DefaultSettings() Settings {
	return Settings{Music: 1, Effects: 1, Language: DefaultLanguage}
}

// Normalized begrenzt die Lautstärken auf 0 … 1 (NaN = Standard) und setzt eine unbekannte Sprache auf
// DefaultLanguage. SetSettings und das Laden wenden es an.
func (s Settings) Normalized() Settings {
	d := DefaultSettings()
	s.Music, s.Effects = volume(s.Music, d.Music), volume(s.Effects, d.Effects)
	if s.Language != LanguageEnglish {
		s.Language = DefaultLanguage
	}
	return s
}

func volume(v, def float64) float64 {
	if math.IsNaN(v) {
		return def
	}
	return math.Max(0, math.Min(1, v))
}
