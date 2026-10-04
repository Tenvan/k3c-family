package store

import (
	"encoding/json"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// SaveInfo ist ein Eintrag der Spielstand-Liste (B-147, S2.2, docs/protocol.md › HTTP: Spielstände). Fehlende Felder
// fehlen auch in der Antwort; eine unlesbare Datei hat nur Name und Error.
type SaveInfo struct {
	Name    string `json:"name"`
	SavedAt string `json:"savedAt,omitempty"`
	Version int    `json:"version,omitempty"`
	Day     int    `json:"day,omitempty"`
	Phase   string `json:"phase,omitempty"`
	Depths  []int  `json:"depths,omitempty"` // Tiefen der Spieler, sortiert, jede einmal
	Error   string `json:"error,omitempty"`
}

// List liefert alle Spielstände (ohne Sicherungen) nach Name sortiert. Gelesen wird generisch, ohne engine/sim.
func (s *Saves) List() []SaveInfo {
	s.mu.Lock()
	defer s.mu.Unlock()
	files, _ := filepath.Glob(filepath.Join(s.Dir, "*.json"))
	list := []SaveInfo{}
	for _, f := range files {
		name := strings.TrimSuffix(filepath.Base(f), ".json")
		if slotName.MatchString(name) {
			list = append(list, saveInfo(name, f))
		}
	}
	slices.SortFunc(list, func(a, b SaveInfo) int { return strings.Compare(a.Name, b.Name) })
	return list
}

func saveInfo(name, path string) SaveInfo {
	bad := SaveInfo{Name: name, Error: "ungültig"}
	data, err := os.ReadFile(path)
	if err != nil {
		return bad
	}
	h, err := parseSave(data)
	if err != nil {
		return bad
	}
	var extra struct {
		Day     int    `json:"day"`
		Phase   string `json:"phase"`
		Players []struct {
			Depth *int `json:"depth"`
		} `json:"players"`
	}
	if json.Unmarshal(data, &extra) != nil {
		return bad
	}
	info := SaveInfo{Name: name, SavedAt: h.savedAt, Version: int(h.version), Day: extra.Day, Phase: extra.Phase}
	for _, p := range extra.Players {
		if p.Depth != nil {
			info.Depths = append(info.Depths, *p.Depth)
		}
	}
	slices.Sort(info.Depths)
	info.Depths = slices.Compact(info.Depths)
	return info
}
