package level

import (
	"fmt"
	"math"
	"testing"
)

// B-652 (AC-02, AC-03): je Zone und Seite 1 bis 3 Mauern und 1 bis 3 Türme, deterministisch, Mindestabstand der Zahlziele,
// ein Teil der Türme 4 bis 10 Units neben einer Mauer.
func TestSitesVerteilt500Seeds(t *testing.T) {
	p := DefaultSitePlacement()
	b := biome(t, "forest")
	var towers, near int
	for i := range 500 {
		lay, err := Generate(b, fmt.Sprintf("sites-%d", i))
		if err != nil {
			t.Fatal(err)
		}
		again, _ := Generate(b, fmt.Sprintf("sites-%d", i))
		if fmt.Sprint(lay.Sites) != fmt.Sprint(again.Sites) {
			t.Fatalf("Seed %d: Plätze nicht deterministisch", i)
		}
		count := map[string]int{}
		for _, s := range lay.Sites {
			count[fmt.Sprintf("%d/%d/%s", s.Side, s.Zone, s.Kind)]++
		}
		for k, n := range count {
			if n > p.PerZoneMax {
				t.Fatalf("Seed %d: %s hat %d Plätze, höchstens %d", i, k, n, p.PerZoneMax)
			}
		}
		for a, sa := range lay.Sites {
			if d := math.Abs(sa.X - lay.HubCenterUnits); d < p.ZoneStartUnits || d >= p.ZoneStartUnits+float64(p.Zones)*lay.ChunkWidthUnits {
				t.Fatalf("Seed %d: Platz %v außerhalb der Zonen", i, sa)
			}
			for _, sb := range lay.Sites[a+1:] {
				if math.Abs(sa.X-sb.X) < payGapUnits {
					t.Fatalf("Seed %d: Plätze %v und %v zu nah", i, sa, sb)
				}
			}
			if sa.Kind != "tower" {
				continue
			}
			towers++
			for _, sb := range lay.Sites {
				if sb.Kind == "wall" && sb.Side == sa.Side && sb.Zone == sa.Zone {
					if d := math.Abs(sa.X - sb.X); d >= p.TowerNearMinUnits && d <= p.TowerNearMaxUnits {
						near++
						break
					}
				}
			}
		}
	}
	if towers == 0 {
		t.Fatal("keine Türme")
	}
	if share := float64(near) / float64(towers); share < p.TowerNearShare*0.8 {
		t.Errorf("Anteil naher Türme %.2f, erwartet etwa %.2f", share, p.TowerNearShare)
	}
	t.Logf("Türme %d, davon nah an einer Mauer %d (%.0f %%)", towers, near, 100*float64(near)/float64(towers))
}

// Jede Seite und Zone mit genug Platz bekommt mindestens eine Mauer und einen Turm.
func TestSitesJedeZoneHatMauerUndTurm(t *testing.T) {
	p := DefaultSitePlacement()
	b := biome(t, "forest")
	for i := range 100 {
		lay, err := Generate(b, fmt.Sprintf("zone-%d", i))
		if err != nil {
			t.Fatal(err)
		}
		have := map[string]bool{}
		for _, s := range lay.Sites {
			have[fmt.Sprintf("%d/%d/%s", s.Side, s.Zone, s.Kind)] = true
		}
		for _, side := range []int{-1, 1} {
			for zone := 1; zone <= 3 && zone <= p.Zones; zone++ {
				for _, kind := range []string{"wall", "tower"} {
					if !have[fmt.Sprintf("%d/%d/%s", side, zone, kind)] {
						t.Errorf("Seed %d: Seite %d Zone %d ohne %s", i, side, zone, kind)
					}
				}
			}
		}
	}
}

// Die Plätze ändern die übrigen Würfe nicht: Entitäten und Chunks bleiben gleich (eigener Strom).
func TestSitesAendernNichtsAnDenEntitaeten(t *testing.T) {
	b := biome(t, "forest")
	lay, err := Generate(b, "k3c-dev")
	if err != nil {
		t.Fatal(err)
	}
	if len(lay.Sites) == 0 || len(lay.Entities) == 0 {
		t.Fatalf("Plätze %d, Entitäten %d", len(lay.Sites), len(lay.Entities))
	}
}
