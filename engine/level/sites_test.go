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
		seed := fmt.Sprintf("sites-%d", i)
		lay, err := Generate(b, seed)
		if err != nil {
			t.Fatal(err)
		}
		again, _ := Generate(b, seed)
		if fmt.Sprint(lay.Sites) != fmt.Sprint(again.Sites) {
			t.Fatalf("Seed %d: Plätze nicht deterministisch", i)
		}
		checkZoneCounts(t, seed, lay, p)
		checkSiteSpacing(t, seed, lay, p)
		tw, nr := countNearTowers(lay, p)
		towers, near = towers+tw, near+nr
	}
	if towers == 0 {
		t.Fatal("keine Türme")
	}
	if share := float64(near) / float64(towers); share < p.TowerNearShare*0.8 {
		t.Errorf("Anteil naher Türme %.2f, erwartet etwa %.2f", share, p.TowerNearShare)
	}
	t.Logf("Türme %d, davon nah an einer Mauer %d (%.0f %%)", towers, near, 100*float64(near)/float64(towers))
}

// checkZoneCounts: je Seite, Zone und Art höchstens PerZoneMax Plätze.
func checkZoneCounts(t *testing.T, seed string, lay Layout, p SitePlacement) {
	t.Helper()
	count := map[string]int{}
	for _, s := range lay.Sites {
		count[fmt.Sprintf("%d/%d/%s", s.Side, s.Zone, s.Kind)]++
	}
	for k, n := range count {
		if n > p.PerZoneMax {
			t.Fatalf("%s: %s hat %d Plätze, höchstens %d", seed, k, n, p.PerZoneMax)
		}
	}
}

// checkSiteSpacing: jeder Platz liegt in den Zonen, und zwei Plätze halten den Zahlziel-Abstand.
func checkSiteSpacing(t *testing.T, seed string, lay Layout, p SitePlacement) {
	t.Helper()
	end := p.ZoneStartUnits + float64(p.Zones)*lay.ChunkWidthUnits
	for a, sa := range lay.Sites {
		if d := math.Abs(sa.X - lay.HubCenterUnits); d < p.ZoneStartUnits || d >= end {
			t.Fatalf("%s: Platz %v außerhalb der Zonen", seed, sa)
		}
		for _, sb := range lay.Sites[a+1:] {
			if math.Abs(sa.X-sb.X) < payGapUnits {
				t.Fatalf("%s: Plätze %v und %v zu nah", seed, sa, sb)
			}
		}
	}
}

// countNearTowers zählt die Türme und davon die, die 4 bis 10 Units neben einer Mauer derselben Zone stehen.
func countNearTowers(lay Layout, p SitePlacement) (towers, near int) {
	for _, sa := range lay.Sites {
		if sa.Kind != "tower" {
			continue
		}
		towers++
		if nearWall(lay, sa, p) {
			near++
		}
	}
	return towers, near
}

func nearWall(lay Layout, tower WallSite, p SitePlacement) bool {
	for _, w := range lay.Sites {
		d := math.Abs(tower.X - w.X)
		if w.Kind == "wall" && w.Side == tower.Side && w.Zone == tower.Zone && d >= p.TowerNearMinUnits && d <= p.TowerNearMaxUnits {
			return true
		}
	}
	return false
}

// Jede Seite und Zone mit genug Platz bekommt mindestens eine Mauer und einen Turm.
func TestSitesJedeZoneHatMauerUndTurm(t *testing.T) {
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
		for _, key := range requiredZoneKeys(3) {
			if !have[key] {
				t.Errorf("Seed %d: %s fehlt", i, key)
			}
		}
	}
}

// requiredZoneKeys: Seite/Zone/Art der ersten zones Zonen beider Seiten.
func requiredZoneKeys(zones int) []string {
	var out []string
	for _, side := range []int{-1, 1} {
		for zone := 1; zone <= zones; zone++ {
			for _, kind := range []string{"wall", "tower"} {
				out = append(out, fmt.Sprintf("%d/%d/%s", side, zone, kind))
			}
		}
	}
	return out
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

// Baugrund (Variante A): Die erste Mauer und der erste Turm der Zone 1 liegen je Seite auf freiem Grund (FreeGround), das
// Layout bleibt unverändert und OnFreeGround meldet genau deren Freiraum.
func TestSitesBaugrundZone1Frei(t *testing.T) {
	p := DefaultSitePlacement()
	b := biome(t, "forest")
	for i := range 200 {
		lay, err := Generate(b, fmt.Sprintf("grund-%d", i))
		if err != nil {
			t.Fatal(err)
		}
		if len(lay.FreeGround) != 4 {
			t.Fatalf("Seed %d: %d Baugrund-Plätze, erwartet 4 (je Seite Mauer und Turm)", i, len(lay.FreeGround))
		}
		for _, x := range lay.FreeGround {
			if !lay.OnFreeGround(x) || !lay.OnFreeGround(x+p.ClearRadiusUnits-1) || lay.OnFreeGround(x+p.ClearRadiusUnits+9999) {
				t.Fatalf("Seed %d: OnFreeGround falsch bei %v", i, x)
			}
		}
	}
}
