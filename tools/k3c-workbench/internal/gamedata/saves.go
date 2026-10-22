package gamedata

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

// EnvSavesDir verlegt den Spielstand-Ordner wie in server/saves.mjs; ein relativer Pfad gilt ab der Repo-Wurzel.
const EnvSavesDir = "K3C_SAVES_DIR"

// save ist der Teil von SaveGame (src/world/sim/campaign.ts), den saves_list zeigt.
type save struct {
	SavedAt       string            `json:"savedAt"`
	Depth         int               `json:"depth"`
	UnlockedDepth int               `json:"unlockedDepth"`
	Time          float64           `json:"time"` // Sekunden der Kampagne
	Players       []json.RawMessage `json:"players"`
}

// SavesDir ist der Spielstand-Ordner: K3C_SAVES_DIR (Wert von env) oder saves/ in der Repo-Wurzel.
func SavesDir(root, env string) string {
	switch {
	case env == "":
		return filepath.Join(root, "saves")
	case filepath.IsAbs(env):
		return env
	default:
		return filepath.Join(root, env)
	}
}

// SavesList ist saves_list: eine Zeile je Datei im Spielstand-Ordner, Sicherungen eingeschlossen.
func SavesList(root, dir string) (string, error) {
	files, err := filepath.Glob(filepath.Join(dir, "*.json"))
	if err != nil || len(files) == 0 {
		return "keine Spielstände", err
	}
	cycle, cycleErr := cycleSeconds(root)
	var lines []string
	if cycleErr != nil {
		lines = append(lines, "Tag unbekannt: "+cycleErr.Error())
	}
	for _, f := range files {
		lines = append(lines, saveLine(f, cycle))
	}
	return strings.Join(lines, "\n"), nil
}

func saveLine(path string, cycle float64) string {
	slot := strings.TrimSuffix(filepath.Base(path), ".json")
	data, err := os.ReadFile(path)
	var s save
	if err == nil {
		err = json.Unmarshal(data, &s)
	}
	if err != nil {
		return slot + " · nicht lesbar"
	}
	day := "?"
	if cycle > 0 {
		day = strconv.Itoa(int(s.Time/cycle) + 1)
	}
	when := s.SavedAt
	if t, err := time.Parse(time.RFC3339, s.SavedAt); err == nil {
		when = t.Local().Format("2006-01-02 15:04")
	}
	return fmt.Sprintf("%s · Stufe %d/%d · Tag %s · %d Spieler · %s", slot, s.Depth, s.UnlockedDepth, day, len(s.Players), when)
}

// cycleSeconds ist die Länge eines Tag/Nacht-Zyklus aus data/biomes/forest.json, dem Biom der Oberwelt (wie
// globalDayNight in src/world/sim/cycle.ts). Der Wert steht nur in data/, nicht im Code.
func cycleSeconds(root string) (float64, error) {
	data, err := os.ReadFile(filepath.Join(root, "data", "biomes", "forest.json"))
	if err != nil {
		return 0, err
	}
	var biome struct {
		Cycle struct {
			DayMinutes      float64 `json:"dayMinutes"`
			TwilightMinutes float64 `json:"twilightMinutes"`
			NightMinutes    float64 `json:"nightMinutes"`
		} `json:"cycle"`
	}
	if err := json.Unmarshal(data, &biome); err != nil {
		return 0, fmt.Errorf("forest.json: %w", err)
	}
	c := biome.Cycle
	length := (c.DayMinutes + c.TwilightMinutes + c.NightMinutes) * 60
	if length <= 0 {
		return 0, fmt.Errorf("forest.json: Zyklus ohne Länge")
	}
	return length, nil
}
