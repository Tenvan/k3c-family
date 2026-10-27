package level

import (
	"math"
	"strconv"
	"testing"
)

// TestCampAbstandZuLinien500Seeds: Jedes Camp liegt ≥ payGapUnits von Mauer, Turm und Tor jeder Linie entfernt,
// im Level selbst und bei jeder möglichen Streuung (B-261/AC-01, W0 › AC-08).
func TestCampAbstandZuLinien500Seeds(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		for seed := range 500 {
			l, err := Generate(b, strconv.Itoa(seed))
			if err != nil {
				t.Fatal(err)
			}
			for _, e := range l.Entities {
				if e.Kind == "recruitCamp" {
					checkCamp(t, l, e.X, id, seed)
				}
			}
		}
	}
}

func checkCamp(t *testing.T, l Layout, x float64, id string, seed int) {
	t.Helper()
	if !wallLines.ClearOf(x - l.HubCenterUnits) {
		t.Errorf("Biom %s, Seed %d: Camp bei Hub %+v zu nah an einer möglichen Linie", id, seed, x-l.HubCenterUnits)
	}
	for _, line := range l.Lines {
		for _, p := range []float64{line.Tower, line.Wall, line.Gate} {
			if math.Abs(x-p) < payGapUnits {
				t.Errorf("Biom %s, Seed %d: Camp %v, Linien-Platz %v", id, seed, x, p)
			}
		}
	}
}

// TestCampClearOfLinesGrenzen: Außerhalb der äußersten Tor-Position bei voller Streuung plus payGapUnits ist frei,
// die alten Camp-Orte ±75 und ±125 nicht.
func TestCampClearOfLinesGrenzen(t *testing.T) {
	d := wallLines
	outer := d.WallUnits[len(d.WallUnits)-1] + float64(d.JitterOutwardUnits) + d.GateOutsetUnits
	cases := []struct {
		dx   float64
		want bool
	}{{75, false}, {-75, false}, {125, false}, {-125, false}, {outer + payGapUnits - 0.5, false}, {outer + payGapUnits, true}, {-175, true}}
	for _, c := range cases {
		if got := wallLines.ClearOf(c.dx); got != c.want {
			t.Errorf("ClearOf(%v) = %v, erwartet %v", c.dx, got, c.want)
		}
	}
}

// B-645: In Biomen mit Bau liegt das Camp nie näher als lairCampGapUnits am Bau und höchstens in 5 % der Seeds hinter einem
// Portal (wo der Abstand zu den Linien keinen anderen Platz lässt, hat er Vorrang, B-261).
func TestCampFernVomBau500Seeds(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		if !b.Lair {
			continue
		}
		behind := 0
		for seed := range 500 {
			l, err := Generate(b, strconv.Itoa(seed))
			if err != nil {
				t.Fatal(err)
			}
			if campBehindPortal(t, l, id, seed) {
				behind++
			}
		}
		if behind > 25 {
			t.Errorf("Biom %s: Camp in %d von 500 Seeds hinter einem Portal, erlaubt 25", id, behind)
		}
	}
}

func campBehindPortal(t *testing.T, l Layout, id string, seed int) bool {
	t.Helper()
	var camp, lair float64
	var portals []float64
	for _, e := range l.Entities {
		switch e.Kind {
		case "recruitCamp":
			camp = e.X
		case "lair":
			lair = e.X
		case "portal":
			portals = append(portals, e.X)
		}
	}
	if math.Abs(camp-lair) < lairCampGapUnits {
		t.Errorf("Biom %s, Seed %d: Camp %v nur %v vom Bau", id, seed, camp, math.Abs(camp-lair))
	}
	for _, p := range portals {
		if (p-l.HubCenterUnits)*(camp-l.HubCenterUnits) > 0 && math.Abs(p-l.HubCenterUnits) < math.Abs(camp-l.HubCenterUnits) {
			return true
		}
	}
	return false
}
