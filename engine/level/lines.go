package level

import (
	"encoding/json"
	"fmt"
	"math"
	"slices"

	"k3c/data"
)

// payGapUnits ist der Mindestabstand zweier Zahlziele: 2 × payRangeUnits (data/economy.json, Glossar › Zahlziel).
var payGapUnits = 2 * loadPayRange()

func readData(name string, v any) {
	raw, err := data.Files.ReadFile(name)
	if err != nil {
		panic(err)
	}
	if err := json.Unmarshal(raw, v); err != nil {
		panic(fmt.Errorf("%s: %w", name, err))
	}
}

func loadPayRange() float64 {
	var economy struct {
		PayRangeUnits float64 `json:"payRangeUnits"`
	}
	readData("economy.json", &economy)
	return economy.PayRangeUnits
}

// hubSite ist ein Hub- oder Insel-Platz aus data/hub.json (sites, islandSites): Art und Lage als Abstand zur Hub-Mitte,
// links negativ.
type hubSite struct {
	kind   string
	offset float64
}

// hubSites sind alle Hub- und Insel-Plätze aus data/hub.json.
var hubSites = loadHubSites()

// farmOffsets sind die Abstände der Farm-Bauplätze zur Hub-Mitte aus data/hub.json › sites (beide Seiten, Q57).
var farmOffsets = loadFarmOffsets()

func loadHubSites() []hubSite {
	type entry struct {
		Kind        string  `json:"kind"`
		OffsetUnits float64 `json:"offsetUnits"`
	}
	var hub struct {
		Sites       []entry `json:"sites"`
		IslandSites []entry `json:"islandSites"`
	}
	readData("hub.json", &hub)
	var out []hubSite
	for _, s := range slices.Concat(hub.Sites, hub.IslandSites) {
		out = append(out, hubSite{s.Kind, s.OffsetUnits})
	}
	return out
}

func loadFarmOffsets() []float64 {
	var out []float64
	for _, s := range hubSites {
		if s.kind == "farm" {
			out = append(out, math.Abs(s.offset))
		}
	}
	return out
}
