package sim

import (
	"fmt"
	"sort"
	"testing"
)

// W0.1 (B-206/AC-05): Plätze und Zahlziele allein aus data/hub.json und data/buildings.json, ohne Welt.

// dataTarget ist ein Zahlziel mit Offset zur Hub-Mitte.
type dataTarget struct {
	name string
	x    float64
}

// sideX ist die Seite eines Offsets: −1 links, +1 rechts.
func sideX(x float64) int {
	if x < 0 {
		return -1
	}
	return 1
}

// lineWalls liefert die Mauer-Offsets einer Seite für eine Streuung je Linie (Bit i gesetzt = Linie i+1 voll gestreut).
func lineWalls(mask int) []float64 {
	l := hub.WallLines
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
func lineTargets(left, right int) []dataTarget {
	l := hub.WallLines
	var out []dataTarget
	for _, side := range []struct {
		dir  float64
		mask int
	}{{-1, left}, {1, right}} {
		for i, w := range lineWalls(side.mask) {
			k := fmt.Sprintf("L%d@%+.0f", i+1, side.dir)
			out = append(out,
				dataTarget{"wall " + k, side.dir * w},
				dataTarget{"tower " + k, side.dir * (w - l.TowerInsetUnits)},
				dataTarget{"gate " + k, side.dir * (w + l.GateOutsetUnits)})
		}
	}
	return out
}

// isLineKind: Mauer, Turm und Tor gehören zu den Linien, nicht zu den Hub-Plätzen.
func isLineKind(kind string) bool { return kind == "wall" || kind == "tower" || kind == "gate" }

// fixedTargets: Burg, Hub- und Insel-Plätze (ohne Linien), Händler und Angebote.
func fixedTargets() []dataTarget {
	out := []dataTarget{{"castle", 0},
		{"merchant buy", hub.Merchant.BuyOffsetUnits}, {"merchant sell", hub.Merchant.SellOffsetUnits}}
	for _, s := range append(append([]HubSite{}, hub.Sites...), hub.IslandSites...) {
		if isLineKind(s.Kind) {
			continue
		}
		out = append(out, dataTarget{fmt.Sprintf("%s@%+.0f", s.Kind, s.OffsetUnits), s.OffsetUnits})
		for _, o := range buildings[s.Kind].Offers {
			out = append(out, dataTarget{fmt.Sprintf("%s:%s", s.Kind, o.Kind), s.OffsetUnits + o.DX})
		}
	}
	return out
}

func TestSiteDataGenauEinPlatzJeBau(t *testing.T) {
	count := map[string]map[int]int{}
	for _, s := range append(append([]HubSite{}, hub.Sites...), hub.IslandSites...) {
		if count[s.Kind] == nil {
			count[s.Kind] = map[int]int{}
		}
		count[s.Kind][sideX(s.OffsetUnits)]++
	}
	for kind := range buildings {
		c := count[kind]
		switch {
		case kind == "castle" || isLineKind(kind):
		case kind == "farm":
			if c[-1] != 1 || c[1] != 1 {
				t.Errorf("farm: je Seite genau ein Platz erwartet, links %d, rechts %d", c[-1], c[1])
			}
		case c[-1]+c[1] != 1:
			t.Errorf("%s: genau ein Hub-Platz erwartet, gefunden %d", kind, c[-1]+c[1])
		}
	}
	for kind := range count {
		if _, ok := buildings[kind]; !ok {
			t.Errorf("Platz %q ohne Eintrag in buildings.json", kind)
		}
	}
	for _, kind := range []string{"wall", "tower", "gate"} {
		if count[kind] != nil {
			t.Errorf("%s gehört nur auf die Mauerlinien, nicht in sites", kind)
		}
	}
}

func TestSiteDataAbstandBeiVollerStreuung(t *testing.T) {
	minGap := 2 * economy.PayRangeUnits // zwei Reichweiten: kein Zahlziel überdeckt ein anderes
	fixed := fixedTargets()
	combos := 1 << len(hub.WallLines.WallUnits)
	for left := range combos {
		for right := range combos {
			all := append(append([]dataTarget{}, fixed...), lineTargets(left, right)...)
			sort.Slice(all, func(i, j int) bool { return all[i].x < all[j].x })
			for i := 1; i < len(all); i++ {
				if gap := all[i].x - all[i-1].x; gap < minGap {
					t.Fatalf("Streuung links %05b rechts %05b: %s (%v) und %s (%v) nur %v Units auseinander, mindestens %v",
						left, right, all[i-1].name, all[i-1].x, all[i].name, all[i].x, gap, minGap)
				}
			}
		}
	}
}

func TestSiteDataLinien(t *testing.T) {
	l := hub.WallLines
	if len(l.WallUnits) == 0 || l.FixedLines < 1 {
		t.Fatalf("Linie 1 muss fest sein: %d Linien, %d fest", len(l.WallUnits), l.FixedLines)
	}
	if l.JitterOutwardUnits < 0 || l.TowerInsetUnits <= 0 || l.GateOutsetUnits <= 0 {
		t.Errorf("Streuung %v, Turm %v innen, Tor %v außen: alle positiv erwartet",
			l.JitterOutwardUnits, l.TowerInsetUnits, l.GateOutsetUnits)
	}
	for i := 1; i < len(l.WallUnits); i++ {
		if l.WallUnits[i] <= l.WallUnits[i-1] {
			t.Errorf("Mauer-Offsets nicht steigend: Linie %d = %v nach %v", i+1, l.WallUnits[i], l.WallUnits[i-1])
		}
	}
	outer := l.WallUnits[len(l.WallUnits)-1] + l.JitterOutwardUnits + l.GateOutsetUnits
	for _, b := range biomes {
		if outer >= b.Portals.MinDistanceFromHubUnits {
			t.Errorf("%s: äußerstes Tor bei %v nicht unter dem Portal-Mindestabstand %v", b.ID, outer,
				b.Portals.MinDistanceFromHubUnits)
		}
	}
}

// Zuordnung Art → Hub-Stufe aus docs/rules/materialien-gebaeude.md § 3.
var hubLevelRule = map[string]int{
	"wall": 1, "tower": 1, "workshop": 1, "farm": 1,
	"gate": 2, "barracks": 2, "storage": 2, "tavern": 2, "stairsUp": 2, "stairsDown": 2,
	"smithy": 3, "healer": 3, "armory": 4,
}

func TestSiteDataHubStufe(t *testing.T) {
	for _, s := range append(append([]HubSite{}, hub.Sites...), hub.IslandSites...) {
		if want, ok := hubLevelRule[s.Kind]; !ok || s.HubLevel != want {
			t.Errorf("%s@%v: hubLevel %d, laut § 3 %d", s.Kind, s.OffsetUnits, s.HubLevel, want)
		}
	}
	if hub.WallLines.GateHubLevel != hubLevelRule["gate"] {
		t.Errorf("Tor: gateHubLevel %d, laut § 3 %d", hub.WallLines.GateHubLevel, hubLevelRule["gate"])
	}
}
