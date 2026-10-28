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
	for _, site := range l.Sites {
		if math.Abs(x-site.X) < payGapUnits {
			t.Errorf("Biom %s, Seed %d: Camp %v, Platz %s bei %v", id, seed, x, site.Kind, site.X)
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
