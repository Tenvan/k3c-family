package assetsui

import (
	"cmp"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
)

// Größenstufen für das Lineal des Größenvergleichs (B-1170, WZ62.3): liest data/sizes.json (LA1.1, Standard
// docs/standards/bildaufbau.md § 2). Der Monarch auf dem Mount ist 1 M = bezugPx Spiel-Pixel hoch. Nur lesen.

// Stufe ist eine Größenstufe: sichtbare Höhe als Vielfaches von M.
type Stufe struct {
	ID  string  `json:"id"`
	M   float64 `json:"m"`
	Was string  `json:"was"`
}

// StufenAsset ordnet ein Asset einer Stufe zu; Sicht ist die gemessene sichtbare Höhe geteilt durch die Rahmenhöhe.
type StufenAsset struct {
	Stufe string  `json:"stufe"`
	Sicht float64 `json:"sicht"`
}

// Stufen ist der Inhalt von data/sizes.json für die Oberfläche: Stufen nach Höhe sortiert, Assets nach `<gruppe>/<id>`.
type Stufen struct {
	// BezugPx: Höhe des berittenen Monarchen (1 M) in Spiel-Pixeln.
	BezugPx float64                `json:"bezugPx"`
	Stufen  []Stufe                `json:"stufen"`
	Assets  map[string]StufenAsset `json:"assets"`
}

// Stufen liest data/sizes.json; fehlt die Datei oder ist sie leer, kommt eine klare Meldung.
func (d Dienst) Stufen() (Stufen, error) {
	file := filepath.Join(d.Root, "data", "sizes.json")
	b, err := os.ReadFile(file)
	if err != nil {
		return Stufen{}, fmt.Errorf("data/sizes.json: %w", err)
	}
	var raw struct {
		BezugPx float64 `json:"bezugPx"`
		Stufen  map[string]struct {
			M   float64 `json:"m"`
			Was string  `json:"was"`
		} `json:"stufen"`
		Assets map[string]StufenAsset `json:"assets"`
	}
	if err := json.Unmarshal(b, &raw); err != nil {
		return Stufen{}, fmt.Errorf("data/sizes.json: kein gültiges JSON: %w", err)
	}
	if raw.BezugPx <= 0 || len(raw.Stufen) == 0 {
		return Stufen{}, fmt.Errorf("data/sizes.json: bezugPx oder stufen fehlen")
	}
	out := Stufen{BezugPx: raw.BezugPx, Stufen: make([]Stufe, 0, len(raw.Stufen)), Assets: raw.Assets}
	if out.Assets == nil {
		out.Assets = map[string]StufenAsset{}
	}
	for id, s := range raw.Stufen {
		out.Stufen = append(out.Stufen, Stufe{ID: id, M: s.M, Was: s.Was})
	}
	slices.SortFunc(out.Stufen, func(a, b Stufe) int { return cmp.Or(cmp.Compare(a.M, b.M), cmp.Compare(a.ID, b.ID)) })
	return out, nil
}
