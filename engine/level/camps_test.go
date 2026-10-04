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
	if !clearOfLines(x - l.HubCenterUnits) {
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
		if got := clearOfLines(c.dx); got != c.want {
			t.Errorf("clearOfLines(%v) = %v, erwartet %v", c.dx, got, c.want)
		}
	}
}
