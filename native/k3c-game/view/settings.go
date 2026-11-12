package view

import "math"

// Sprachen der Einstellungen (wie src/core/settings.ts); DefaultLanguage gilt für jeden anderen Wert.
const (
	DefaultLanguage = "de"
	LanguageEnglish = "en"
)

// Fassungen des Flusses (Settings.River, B-816); world kennt keine weiteren.
const (
	RiverShader = "shader" // Fluss mit Kage-Shader
	RiverTiles  = "tiles"  // Fluss aus Wellen-Kacheln
)

// Settings sind die Einstellungen am Gerät (Optionen-Menü, B-800): Session.SetSettings ändert sie, local speichert
// sie dauerhaft neben den Spielständen, Frame.Settings zeigt den aktuellen Stand. game wendet Vollbild an (nicht im
// Aufnahme-Modus), audio die Lautstärken, hud und menu die Sprache.
type Settings struct {
	Music      float64 // Lautstärke der Musik 0 … 1
	Effects    float64 // Lautstärke der Klang-Effekte 0 … 1
	Fullscreen bool    // Vollbild statt Fenster
	Language   string  // DefaultLanguage oder LanguageEnglish
	// River ist die Fassung des Flusses (F3, B-816): "" = Startwert (K3C_RIVER, sonst Shader), sonst RiverShader oder
	// RiverTiles. Kein Eintrag im Optionen-Menü; local speichert nur einen gewählten Wert.
	River string
	// Stats zeigt Bildrate und Rechenzeit (Update/Draw in ms) in StatsRectLines (F2, B-816).
	Stats bool
}

// DefaultSettings sind die Werte ohne gespeicherte Einstellungen (wie der Browser: volle Lautstärke, Deutsch, Fenster).
func DefaultSettings() Settings {
	return Settings{Music: 1, Effects: 1, Language: DefaultLanguage}
}

// Normalized begrenzt die Lautstärken auf 0 … 1 (NaN = Standard), setzt eine unbekannte Sprache auf
// DefaultLanguage und eine unbekannte Fluss-Fassung auf "". SetSettings und das Laden wenden es an.
func (s Settings) Normalized() Settings {
	d := DefaultSettings()
	s.Music, s.Effects = volume(s.Music, d.Music), volume(s.Effects, d.Effects)
	if s.Language != LanguageEnglish {
		s.Language = DefaultLanguage
	}
	if s.River != RiverShader && s.River != RiverTiles {
		s.River = ""
	}
	return s
}

// NextRiver liefert die andere Fassung des Flusses: ausgehend von River bzw. bei "" von start (der Startwert der
// Welt, RiverShader oder RiverTiles).
func (s Settings) NextRiver(start string) string {
	cur := s.River
	if cur == "" {
		cur = start
	}
	if cur == RiverTiles {
		return RiverShader
	}
	return RiverTiles
}

func volume(v, def float64) float64 {
	if math.IsNaN(v) {
		return def
	}
	return math.Max(0, math.Min(1, v))
}
