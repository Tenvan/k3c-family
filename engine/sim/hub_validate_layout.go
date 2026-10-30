package sim

import (
	"fmt"
	"math"
	"slices"
	"sort"
	"strings"
)

// Abstands-Regeln der Prüfung (hub_validate.go, W0.1 B-206/AC-05, W11.1 B-409): Plätze und Zahlziele allein aus der
// Kandidaten-hub.json und data/buildings.json, ohne Welt, in jeder Tiefe.

// hubTarget ist ein Zahlziel mit Offset zur Hub-Mitte.
type hubTarget struct {
	name string
	x    float64
}

// zoneTargets: der feste Baugrund der Zone 1 (Mauer und Turm) beider Seiten; die übrigen Plätze der Zonen würfelt der Generator
// und hält selbst Abstand (engine/level/sites.go).
func zoneTargets(p SitePlacementData) []hubTarget {
	var out []hubTarget
	for _, dir := range []float64{-1, 1} {
		k := fmt.Sprintf("Z1@%+.0f", dir)
		out = append(out, hubTarget{"wall " + k, dir * p.Zone1WallUnits}, hubTarget{"tower " + k, dir * p.Zone1TowerUnits})
	}
	return out
}

// fixedTargets: Burg, Hub- und Insel-Plätze (ohne Linien), Händler und Angebote, die es in Tiefe depth gibt
// (deeper: es gibt eine tiefere Stufe), wie world_lines.go, island.go und merchant.go sie anlegen.
func fixedTargets(h *HubData, depth int, deeper bool) []hubTarget {
	out := []hubTarget{{"castle", 0}}
	if depth == h.Merchant.OnlyDepth {
		out = append(out, hubTarget{"merchant buy", h.Merchant.BuyOffsetUnits},
			hubTarget{"merchant sell", h.Merchant.SellOffsetUnits})
	}
	for _, s := range allSites(h) {
		if isLineKind(s.Kind) || depth < s.FromDepth || (s.NeedsDeeper && !deeper) {
			continue
		}
		out = append(out, hubTarget{siteName(s), s.OffsetUnits})
		for _, o := range buildings[s.Kind].Offers {
			out = append(out, hubTarget{fmt.Sprintf("%s:%s", s.Kind, o.Kind), s.OffsetUnits + o.DX})
		}
	}
	return out
}

// eachLayout ruft fn für jede Tiefe der Biome (mit und ohne tiefere Stufe) auf.
func eachLayout(h *HubData, fn func(desc string, all []hubTarget)) {
	var depths []int
	for _, b := range biomes {
		if !slices.Contains(depths, b.Depth) {
			depths = append(depths, b.Depth)
		}
	}
	for _, depth := range depths {
		for _, deeper := range []bool{true, false} {
			desc := fmt.Sprintf("Tiefe %d (tiefer %v)", depth, deeper)
			fn(desc, slices.Concat(fixedTargets(h, depth, deeper), zoneTargets(h.SitePlacement)))
		}
	}
}

// checkGaps: Zwei Zahlziele liegen ≥ 2 × payRangeUnits auseinander (kein Zahlziel überdeckt ein anderes), Treppen
// ≥ stairsGapUnits zu jedem Zahlziel derselben Tiefe. Jedes Paar wird nur einmal gemeldet (erste Lage).
func checkGaps(h *HubData) []HubFinding {
	minGap := 2 * economy.PayRangeUnits
	var out []HubFinding
	seen := map[string]bool{}
	report := func(rule, desc string, a, b hubTarget, limit float64) {
		key := rule + "|" + a.name + "|" + b.name
		if seen[key] {
			return
		}
		seen[key] = true
		out = append(out, HubFinding{rule, a.name + " / " + b.name, fmt.Sprintf("%v und %v nur %v Units auseinander, mindestens %v (%s)",
			a.x, b.x, math.Abs(b.x-a.x), limit, desc)})
	}
	eachLayout(h, func(desc string, all []hubTarget) {
		sort.SliceStable(all, func(i, j int) bool { return all[i].x < all[j].x })
		for i := 1; i < len(all); i++ {
			if all[i].x-all[i-1].x < minGap {
				report(RuleGap, desc, all[i-1], all[i], minGap)
			}
		}
		for _, s := range all {
			if !strings.HasPrefix(s.name, "stairs") {
				continue
			}
			for _, o := range all {
				if o.name != s.name && math.Abs(o.x-s.x) < stairsGapUnits {
					report(RuleStairs, desc, s, o, stairsGapUnits)
				}
			}
		}
	})
	return out
}
