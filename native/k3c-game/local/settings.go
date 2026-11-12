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
	Fullscreen *bool    `json:"fullscreen"`
	Language   *string  `json:"language"`
	River      *string  `json:"river,omitempty"` // fehlt: Startwert (K3C_RIVER), B-816
	Stats      *bool    `json:"stats"`
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
	if j.Fullscreen != nil {
		s.Fullscreen = *j.Fullscreen
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
	return s.Normalized(), nil
}

// saveSettings schreibt die Einstellungen nach path (erst in eine Temp-Datei, dann umbenennen: nie halb geschrieben).
func saveSettings(path string, s view.Settings) error {
	j := settingsJSON{Music: &s.Music, Effects: &s.Effects, Fullscreen: &s.Fullscreen, Language: &s.Language, Stats: &s.Stats}
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
