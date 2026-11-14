package menu

// Vorgaben der Seite Testing (B-896) aus data/stresstests.json: Werte wie view.StressTest, dazu der Name je Sprache.
// Fehlt die Datei oder ist sie kaputt, gibt es keine Vorgaben und eine Meldung (die Seite bleibt bedienbar).

import (
	"encoding/json"
	"log/slog"

	"k3c/data"
	"k3c/native/k3c-game/view"
)

// stressPreset ist eine Vorgabe: Werte und Name (deutsch, englisch).
type stressPreset struct {
	view.StressTest
	Name   string `json:"name"`
	NameEn string `json:"nameEn"`
}

// stressPresetsFile ist die Datei der Vorgaben in data/.
const stressPresetsFile = "stresstests.json"

// parseStressPresets liest die Vorgaben aus raw; jede Vorgabe ist normalisiert, ohne Id oder Name verworfen.
func parseStressPresets(raw []byte) ([]stressPreset, error) {
	var v struct {
		Presets []stressPreset `json:"presets"`
	}
	if err := json.Unmarshal(raw, &v); err != nil {
		return nil, err
	}
	out := v.Presets[:0]
	for _, p := range v.Presets {
		if p.Preset == "" || p.Name == "" {
			continue
		}
		p.StressTest = p.Normalized()
		out = append(out, p)
	}
	return out, nil
}

// loadStressPresets liest data/stresstests.json; ein Fehler steht im Log und kommt als Meldung zurück.
func loadStressPresets() ([]stressPreset, error) {
	raw, err := data.Files.ReadFile(stressPresetsFile)
	if err == nil {
		var ps []stressPreset
		if ps, err = parseStressPresets(raw); err == nil {
			return ps, nil
		}
	}
	slog.Warn("💥 Stresstest-Vorgaben nicht lesbar", "datei", stressPresetsFile, "err", err)
	return nil, err
}

// name ist der Name der Vorgabe in der Sprache lang.
func (p stressPreset) name(lang string) string {
	if lang == view.LanguageEnglish && p.NameEn != "" {
		return p.NameEn
	}
	return p.Name
}
