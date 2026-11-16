package sim

import (
	"fmt"
	"math"

	"k3c/engine/level"
)

// Hub- und Insel-Plätze im Wertesatz der Welt (docs/rules/welten.md § 1.3, B-983): `hub.sites` und `hub.islandSites` sind
// übersteuerbar. Der Generator bekommt sie als Wert (level.Options.HubSites), die Stufe liest sie aus demselben Wertesatz
// (hubOf); `hub.sitePlacement` bleibt gesperrt.

// RuleBounds: Platz außerhalb der Hub-Breite.
const RuleBounds = "hub-breite"

// LevelSites sind die Hub- und Insel-Plätze für den Generator, in Dateireihenfolge (sites, dann islandSites).
func (h *HubData) LevelSites() []level.HubSite {
	out := make([]level.HubSite, 0, len(h.Sites)+len(h.IslandSites))
	for _, s := range allSites(h) {
		out = append(out, level.HubSite{Kind: s.Kind, OffsetUnits: s.OffsetUnits})
	}
	return out
}

// checkWorldHubSites prüft die Plätze eines Wertesatzes (fail closed): Hub-Stufen, Arten, Pflichtplätze, ein Platz je Bau,
// Lage innerhalb der Hub-Breite und Abstand der Zahlziele (2 × payRangeUnits der Welt). Die Meldung nennt Datei, Regel und
// Platz; der Aufrufer ergänzt die Welt.
func checkWorldHubSites(h *HubData, eco economyData, bs map[string]BuildingData) error {
	out := checkHubLevels(h)
	out = append(out, checkSiteKinds(h, bs)...)
	out = append(out, checkSiteBounds(h)...)
	if len(out) == 0 {
		out = checkGaps(h, bs, eco.PayRangeUnits)
	}
	if len(out) > 0 {
		return fmt.Errorf("data/hub.json: %s", out[0])
	}
	return nil
}

// checkSiteBounds: Jeder Platz liegt innerhalb der halben Hub-Breite des schmalsten Bioms (die Insel-Plätze stehen im Hub).
func checkSiteBounds(h *HubData) []HubFinding {
	half := math.Inf(1)
	for _, b := range biomes {
		half = min(half, b.HubWidthUnits/2)
	}
	var out []HubFinding
	for _, s := range allSites(h) {
		if math.Abs(s.OffsetUnits) >= half {
			out = append(out, HubFinding{RuleBounds, siteName(s), fmt.Sprintf("offsetUnits %v liegt nicht innerhalb der Hub-Breite (±%v)", s.OffsetUnits, half)})
		}
	}
	return out
}

// checkWorldPlans prüft die geplanten Chunks der Inseln einer Welt gegen die Plätze ihres Wertesatzes: Ruinen dürfen nicht
// auf der Lage eines Platzes gleicher Art liegen (level/ruins.go). Die Meldung nennt Welt, Insel, Tiefe und Platz.
func checkWorldPlans(world WorldDef) error {
	sites := world.values.Hub.LevelSites()
	for _, d := range world.Islands {
		if err := checkPlannedChunksWith(d, sites); err != nil {
			return fmt.Errorf("%s: Insel %s: %w", worldPath(world.ID), d.ID, err)
		}
	}
	return nil
}
