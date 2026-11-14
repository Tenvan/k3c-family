package local

import (
	"encoding/json"
	"errors"
	"io/fs"
	"os"
	"path/filepath"

	"k3c/native/k3c-game/view"
)

// SettingsFile ist die Datei der Einstellungen im Ordner der Spielstände. Der Unterstrich hält sie aus der
// Spielstand-Liste (store.Saves.List nimmt nur Namen aus a–z, 0–9 und -).
const SettingsFile = "_settings.json"

// settingsJSON ist das Dateiformat der Einstellungen; fehlende Felder behalten den Standard.
type settingsJSON struct {
	Music      *float64 `json:"music"`
	Effects    *float64 `json:"effects"`
	Window     *string  `json:"window"` // Anzeige-Modus (B-826): "", "rahmenlos" oder "vollbild"
	Language   *string  `json:"language"`
	River      *string  `json:"river,omitempty"` // fehlt: Startwert (K3C_RIVER), B-816
	Stats      *bool    `json:"stats"`
	Flashes    *bool    `json:"flashes"`   // Blitze an (B-809); fehlt: an
	Guide      *bool    `json:"guide"`     // Anleitung an (B-809); fehlt: an
	GuideSeen  *int     `json:"guideSeen"` // erledigte Hinweise der ersten Nacht (Bitmaske, view.Settings.GuideSeen)
}

// loadSettings liest die Einstellungen aus path. Fehlt die Datei oder ist sie kaputt, gelten die Standardwerte (eine
// kaputte Datei meldet der Fehler, das Spiel startet trotzdem).
func loadSettings(path string) (view.Settings, error) {
	s := view.DefaultSettings()
	raw, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return s, nil
	}
	if err != nil {
		return s, err
	}
	var j settingsJSON
	if err := json.Unmarshal(raw, &j); err != nil {
		return s, err
	}
	if j.Music != nil {
		s.Music = *j.Music
	}
	if j.Effects != nil {
		s.Effects = *j.Effects
	}
	if j.Window != nil {
		s.Window = *j.Window
	}
	if j.Language != nil {
		s.Language = *j.Language
	}
	if j.River != nil {
		s.River = *j.River
	}
	if j.Stats != nil {
		s.Stats = *j.Stats
	}
	applyGuide(&s, j)
	return s.Normalized(), nil
}

// applyGuide übernimmt Blitze, Anleitung und „Anleitung gesehen“ aus der Datei (B-809); fehlende Felder bleiben Standard.
func applyGuide(s *view.Settings, j settingsJSON) {
	if j.Flashes != nil {
		s.FlashesOff = !*j.Flashes
	}
	if j.Guide != nil {
		s.GuideOff = !*j.Guide
	}
	if j.GuideSeen != nil {
		s.GuideSeen = *j.GuideSeen
	}
}

// saveSettings schreibt die Einstellungen nach path (erst in eine Temp-Datei, dann umbenennen: nie halb geschrieben).
func saveSettings(path string, s view.Settings) error {
	flashes, guide := !s.FlashesOff, !s.GuideOff
	j := settingsJSON{Music: &s.Music, Effects: &s.Effects, Window: &s.Window, Language: &s.Language, Stats: &s.Stats,
		Flashes: &flashes, Guide: &guide, GuideSeen: &s.GuideSeen}
	if s.River != "" {
		j.River = &s.River
	}
	raw, err := json.MarshalIndent(j, "", "  ")
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	tmp := path + ".tmp"
	if err := os.WriteFile(tmp, raw, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, path)
}

// SetSettings übernimmt die Einstellungen sofort (ab dem nächsten Frame) und speichert sie neben den Spielständen.
func (s *Session) SetSettings(v view.Settings) error {
	s.settings = v.Normalized()
	if err := saveSettings(s.settingsPath, s.settings); err != nil {
		s.log.Error("💥 Einstellungen nicht gespeichert", "datei", s.settingsPath, "err", err)
		return userErr("Einstellungen nicht gespeichert, sie gelten bis zum Beenden", err)
	}
	s.log.Info("💾 Einstellungen gespeichert", "datei", s.settingsPath)
	return nil
}

// Settings liefert die aktuellen Einstellungen (wie Frame.Settings), auch ohne laufendes Spiel.
func (s *Session) Settings() view.Settings { return s.settings }
