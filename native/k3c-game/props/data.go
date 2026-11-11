package props

// Werte aus data/ für die Anzeige (Spiegel der Regeln in src/scenes/hud/economyLabels.ts): Namen, Baukosten,
// Ausbau-Stufen, Bogen-Regal und Plantage der Gebäude, Bogen-Preis, Wurzeln je Biom. Einmal
// beim Laden des Pakets gelesen; ein Lesefehler lässt die Werte leer (die Anzeige zeigt dann weniger, nie falsch).

import (
	"encoding/json"
	"log/slog"
	"strings"

	"k3c/data"
)

// cost sind Kosten in Gold und Material (Schlüssel wie in data/: gold, wood, stone, copper, iron, crystal).
type cost map[string]int

// buildingData ist der Teil eines Eintrags aus data/buildings.json, den die Anzeige braucht.
type buildingData struct {
	Name        string
	Cost        cost
	CostByDepth []cost
	Levels      []struct{ Cost cost }
	BowRack     int
	Plantation  struct {
		Slots      int
		StripUnits float64
	}
}

var (
	buildings           = readJSON[map[string]buildingData]("buildings.json")
	archerCost          = readJSON[map[string]struct{ Cost cost }]("troops.json")["archer"].Cost
	rootsByBiom, depths = readBiomes()
)

// resourceOrder ist die feste Reihenfolge der Rohstoffe; resourceNames ihre deutschen Namen (texts.de.ts › res.*).
var (
	resourceOrder = [...]string{"wood", "stone", "copper", "iron", "crystal"}
	resourceNames = map[string]string{"wood": "Holz", "stone": "Stein", "copper": "Kupfer", "iron": "Eisen", "crystal": "Kristall"}
)

// readJSON liest eine Datei aus data/; Fehler landen im Log, das Ergebnis bleibt dann leer.
func readJSON[T any](name string) T {
	var out T
	raw, err := data.Files.ReadFile(name)
	if err == nil {
		err = json.Unmarshal(raw, &out)
	}
	if err != nil {
		slog.Error("💥 Anzeige-Werte nicht lesbar", "datei", name, "err", err)
	}
	return out
}

// readBiomes liest rootsPerNode je Biom (data/biomes/*.json, fehlt = 1) und die Tiefen, die es gibt (Ausgang:
// „Tiefe n“ oder verschüttet).
func readBiomes() (roots map[string]int, deep map[int]bool) {
	roots, deep = map[string]int{}, map[int]bool{}
	entries, err := data.Files.ReadDir("biomes")
	if err != nil {
		slog.Error("💥 Biome nicht lesbar", "err", err)
		return roots, deep
	}
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		b := readJSON[struct {
			ID           string
			Depth        int
			RootsPerNode int
		}]("biomes/" + e.Name())
		if b.ID != "" {
			roots[b.ID] = max(1, b.RootsPerNode)
			deep[b.Depth] = true
		}
	}
	return roots, deep
}

// siteName ist der Anzeigename einer Platzart (data/buildings.json › name; das Schiff steht dort nicht).
func siteName(kind string) string {
	if b, ok := buildings[kind]; ok && b.Name != "" {
		return b.Name
	}
	if kind == "ship" {
		return "Schiff"
	}
	return kind
}

// buildCost sind die Baukosten eines unbezahlten Platzes laut Daten: der Eintrag der Biom-Tiefe in costByDepth
// (Treppen), fehlt er oder ist er leer, cost der Art (Spiegel von siteCost in engine/sim/hub_level.go).
func buildCost(kind string, depth int) cost {
	b := buildings[kind]
	if depth >= 0 && depth < len(b.CostByDepth) && len(b.CostByDepth[depth]) > 0 {
		return b.CostByDepth[depth]
	}
	return b.Cost
}

// upgradeCost sind die Kosten des Ausbaus eines gebauten Platzes der Stufe level auf level+1 (nil ohne Stufe).
func upgradeCost(kind string, level int) cost {
	lv := buildings[kind].Levels
	if level < 1 {
		level = 1
	}
	if level >= len(lv) {
		return nil
	}
	return lv[level].Cost
}

// materials nennt die Rohstoffe (ohne Gold) einer Kostenliste in fester Reihenfolge.
func materials(c cost) []string {
	var out []string
	for _, r := range resourceOrder {
		if c[r] > 0 {
			out = append(out, r)
		}
	}
	return out
}
