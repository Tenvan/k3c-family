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
	// FlashesOff schaltet die Bildschirmblitze ab (Optionen › Blitze, B-809; fx). Der Null-Wert ist der Standard „an“.
	FlashesOff bool
	// GuideOff schaltet die Hinweise der geführten ersten Nacht ab (Optionen › Anleitung, B-809; hud); Null-Wert = an.
	GuideOff bool
	// GuideSeen sind die erledigten Hinweise der geführten ersten Nacht als Bitmaske (hud vergibt die Bits und meldet
	// neue über Session.SetSettings); 0 = nichts gesehen. Das Optionen-Menü setzt sie beim Einschalten der Anleitung zurück.
	GuideSeen int
}

// DefaultSettings sind die Werte ohne gespeicherte Einstellungen (wie der Browser: volle Lautstärke, Deutsch, Fenster).
func DefaultSettings() Settings {
	return Settings{Music: 1, Effects: 1, Language: DefaultLanguage}
}

// Normalized begrenzt die Lautstärken auf 0 … 1 (NaN = Standard), setzt eine unbekannte Sprache auf
// DefaultLanguage, eine unbekannte Fluss-Fassung auf "" und negative GuideSeen auf 0. SetSettings und das Laden wenden
// es an.
func (s Settings) Normalized() Settings {
	d := DefaultSettings()
	s.Music, s.Effects = volume(s.Music, d.Music), volume(s.Effects, d.Effects)
	if s.Language != LanguageEnglish {
		s.Language = DefaultLanguage
	}
	if s.River != RiverShader && s.River != RiverTiles {
		s.River = ""
	}
	s.GuideSeen = max(0, s.GuideSeen)
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
