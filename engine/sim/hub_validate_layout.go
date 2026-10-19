package sim

import (
	"fmt"
	"math"
	"slices"
	"sort"
	"strings"
)

// Abstands-Regeln der Prüfung (hub_validate.go, W0.1 B-206/AC-05, W11.1 B-409): Plätze und Zahlziele allein aus der
// Kandidaten-hub.json und data/buildings.json, ohne Welt, in jeder Tiefe und bei jeder Streuung der Linien.

// hubTarget ist ein Zahlziel mit Offset zur Hub-Mitte.
type hubTarget struct {
	name string
	x    float64
}

// lineWalls liefert die Mauer-Offsets einer Seite für eine Streuung je Linie (Bit i gesetzt = Linie i+1 voll gestreut).
func lineWalls(l WallLineData, mask int) []float64 {
	out := make([]float64, len(l.WallUnits))
	for i, w := range l.WallUnits {
		out[i] = w
		if i >= l.FixedLines && mask&(1<<i) != 0 {
			out[i] += l.JitterOutwardUnits
		}
	}
	return out
}

// lineTargets: Mauer, Turm und Tor jeder Linie beider Seiten.
func lineTargets(l WallLineData, left, right int) []hubTarget {
	var out []hubTarget
	for _, side := range []struct {
		dir  float64
		mask int
	}{{-1, left}, {1, right}} {
		for i, w := range lineWalls(l, side.mask) {
			k := fmt.Sprintf("L%d@%+.0f", i+1, side.dir)
			out = append(out,
				hubTarget{"wall " + k, side.dir * w},
				hubTarget{"tower " + k, side.dir * (w - l.TowerInsetUnits)},
				hubTarget{"gate " + k, side.dir * (w + l.GateOutsetUnits)})
		}
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

// eachLayout ruft fn für jede Tiefe der Biome (mit und ohne tiefere Stufe) bei jeder Streuung der gestreuten Linien
// (ohne oder voll) auf. Die Burg trennt die Seiten: Jedes benachbarte Paar hängt höchstens von der Streuung einer
// Seite ab, deshalb genügt jede Streuung einer Seite bei ungestreuter anderer Seite.
func eachLayout(h *HubData, fn func(desc string, all []hubTarget)) {
	l := h.WallLines
	n, fixed := len(l.WallUnits), min(max(l.FixedLines, 0), len(l.WallUnits))
	combos := 1 << (n - fixed)
	if l.JitterOutwardUnits == 0 {
		combos = 1
	}
	var depths []int
	for _, b := range biomes {
		if !slices.Contains(depths, b.Depth) {
			depths = append(depths, b.Depth)
		}
	}
	for _, depth := range depths {
		for _, deeper := range []bool{true, false} {
			base := fixedTargets(h, depth, deeper)
			for m := range combos {
				masks := [][2]int{{m << fixed, 0}, {0, m << fixed}}
				if m == 0 {
					masks = masks[:1] // beide Seiten ungestreut: eine Lage
				}
				for _, lr := range masks {
					desc := fmt.Sprintf("Tiefe %d (tiefer %v), Streuung links %0*b rechts %0*b", depth, deeper, n, lr[0], n, lr[1])
					fn(desc, slices.Concat(base, lineTargets(l, lr[0], lr[1])))
				}
			}
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
