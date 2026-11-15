package level

import (
	"math"
	"strings"
	"testing"
)

// B-983: Der Generator rechnet mit den Plätzen der Welt (Options.HubSites); nil ist die Basis.

func wallNear(l Layout, x float64) bool {
	for _, s := range l.Sites {
		if math.Abs(s.X-x) < payGapUnits {
			return true
		}
	}
	return false
}

// Ein Platz der Welt hält Mauern und Türme auf Abstand: mit dem Platz weit draußen kommt keine Verteilung in seine Nähe,
// in der Basis (kein Platz dort) geschieht das bei vielen Seeds.
func TestHubSitesHaltenZahlzieleAufAbstand(t *testing.T) {
	b := biome(t, "forest")
	world := []HubSite{{"workshop", -16}, {"smithy", 60}, {"stairsUp", 16}}
	near := 0
	for i := range 200 {
		seed := "plaetze-" + string(rune('a'+i%26)) + string(rune('a'+i/26))
		base, err := GenerateWith(b, seed, Options{})
		if err != nil {
			t.Fatal(err)
		}
		own, err := GenerateWith(b, seed, Options{HubSites: world})
		if err != nil {
			t.Fatal(err)
		}
		if wallNear(own, own.HubCenterUnits+60) {
			t.Fatalf("Seed %s: Mauer oder Turm näher als %v Units am Platz der Welt", seed, payGapUnits)
		}
		if wallNear(base, base.HubCenterUnits+60) {
			near++
		}
	}
	if near == 0 {
		t.Error("kein Seed zeigt den Unterschied: der Test beweist nichts")
	}
}

// Ohne eigene Plätze (nil) und mit den Plätzen der Basis ist das Level gleich.
func TestHubSitesBasisGleichNil(t *testing.T) {
	b := biome(t, "forest")
	own := make([]HubSite, len(hubSites))
	for i, s := range hubSites {
		own[i] = HubSite{s.kind, s.offset}
	}
	for _, seed := range []string{"eins", "zwei", "drei"} {
		a, _ := GenerateWith(b, seed, Options{})
		c, _ := GenerateWith(b, seed, Options{HubSites: own})
		if len(a.Sites) != len(c.Sites) || len(a.Entities) != len(c.Entities) {
			t.Fatalf("Seed %s: Level weicht von der Basis ab", seed)
		}
		for i := range a.Sites {
			if a.Sites[i] != c.Sites[i] {
				t.Fatalf("Seed %s: Platz %d weicht ab: %+v / %+v", seed, i, a.Sites[i], c.Sites[i])
			}
		}
	}
}

// Die Prüfung der Planung nimmt die Plätze der Welt: Eine Ruine auf der Lage eines Platzes gleicher Art wird abgelehnt.
func TestHubSitesPlanPruefungFolgtWelt(t *testing.T) {
	b := biome(t, "forest")
	lane := geometryOf(b, 0).hubHalf + 10
	at := []HubSite{{"smithy", lane}}
	plan := []PlannedChunk{withRuin(p(1, "forest"), "smithy", 10)} // Lage hubHalf + 10 Units vom Hub
	if err := ValidatePlanWith(b, plan, 0, nil); err != nil {
		t.Fatalf("Basis: %v", err)
	}
	err := ValidatePlanWith(b, plan, 0, at)
	if err == nil || !strings.Contains(err.Error(), "Hub-Platzes gleicher Art") {
		t.Errorf("Platz der Welt auf der Ruinen-Lage: Ablehnung erwartet, war %v", err)
	}
	if _, err := GenerateWith(b, "x", Options{Planned: plan, HubSites: at}); err == nil {
		t.Error("GenerateWith nimmt die Plätze der Welt nicht für die Prüfung")
	}
}
