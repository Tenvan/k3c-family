package sim

import (
	"math"
	"slices"
	"testing"

	"k3c/engine/level"
)

// W2.3 (B-012/AC-01, AC-02, Sprint W2 AC-05): Die Mine (Tiefe 2) hat Kupfer aus Erz und Adern und eigene Gegner.

// mineWorld ist eine Mine ohne Landstreicher, Truhen und Nacht.
func mineWorld(t *testing.T) *World {
	t.Helper()
	w := newWorld(2, "mine", Options{})
	w.Troops, w.Camps, w.Pickups, w.CycleSpeed = []*Troop{}, []*Camp{}, []*Pickup{}, 1e-6
	return w
}

func firstNode(t *testing.T, w *World, kind string) *ResourceNode {
	t.Helper()
	for _, n := range w.Nodes {
		if n.Kind == kind {
			return n
		}
	}
	t.Fatalf("kein %s in der Mine", kind)
	return nil
}

func TestMineLevelHatKupfer(t *testing.T) {
	b := biomeForDepth(2)
	l, err := level.Generate(b, "mine")
	if err != nil {
		t.Fatal(err)
	}
	count := map[string]int{}
	for _, e := range l.Entities {
		count[e.Kind]++
	}
	if b.PrimaryResource != "copper" || count["copperOre"] == 0 || count["copperVein"] != 2 {
		t.Fatalf("Mine: Rohstoff %s, %d Kupfererz, %d Kupfer-Adern", b.PrimaryResource, count["copperOre"], count["copperVein"])
	}
	w := mineWorld(t)
	firstNode(t, w, "copperOre")
	firstNode(t, w, "copperVein")
}

// Kupfererz und Kupfer-Ader: zwei Spieler markieren, ein Bauer liefert in den Insel-Vorrat; das Erz ist danach weg,
// die Ader liefert weiter.
func TestMineKupferBisInDenVorrat(t *testing.T) {
	isl := mustIsland(t, "mine", []int{0, 1, 2})
	isl.DefeatedBosses = []string{"goblinLeader", "trollKing", "ratQueen"} // Lieferkette ohne Minibosse prüfen (B-338)
	w := isl.Stages[2]
	w.Troops, w.CycleSpeed = []*Troop{}, 1e-6
	p0, p1 := AddIslandPlayer(isl, 2), AddIslandPlayer(isl, 2)
	ore, vein := firstNode(t, w, "copperOre"), firstNode(t, w, "copperVein")
	ore.X, vein.X = w.HubX-30, w.HubX+30 // nah an der Burg, außer Reichweite anderer Zahlziele
	for _, n := range []*ResourceNode{ore, vein} {
		gold := p0.Gold + p1.Gold
		g, _ := gatherOf(n.Kind)
		payAt(w, []*Player{p0, p1}, n.X, 10, func() bool { return n.Marked })
		if !n.Marked || gold-p0.Gold-p1.Gold != g.MarkCost {
			t.Fatalf("%s: markiert %v, bezahlt %d, markCost %d", n.Kind, n.Marked, gold-p0.Gold-p1.Gold, g.MarkCost)
		}
	}
	peasant := spawnVagrant(w, w.HubX, w.HubX)
	peasant.Kind = "peasant"
	runIsland(isl, []PlayerCommand{{}, {}}, 300)
	oreAmount := economy.Gatherables["copperOre"].Amount
	if nodeByID(w, ore.ID) != nil || nodeByID(w, vein.ID) == nil || !vein.Marked {
		t.Fatalf("Erz noch da %v, Ader weg %v", nodeByID(w, ore.ID) != nil, nodeByID(w, vein.ID) == nil)
	}
	if isl.Stock.Copper < oreAmount+2*economy.Veins["copperVein"].Amount {
		t.Errorf("Kupfer im Vorrat %d, erwartet Erz (%d) und mehrere Ader-Lieferungen", isl.Stock.Copper, oreAmount)
	}
}

// Bei vollem Aggressionspool spawnt die Welle nur Zombie, Rattenschwarm und Minengeist, skaliert nach Tiefe 2.
func TestMineWelleNurMinenGegner(t *testing.T) {
	allowed := biomeForDepth(2).Enemies.Portal
	if !slices.Equal(allowed, []string{"zombie", "ratSwarm", "mineGhost"}) {
		t.Fatalf("Pool der Mine %v", allowed)
	}
	seen := map[string]bool{}
	for wave := range 6 {
		w := mineWorld(t)
		w.Wave = wave * 3
		*w.Aggression = 100
		run(w, 30)
		for _, e := range w.Enemies {
			seen[e.Kind] = true
			d, s := enemyData[e.Kind], waves.DepthScaling
			if !slices.Contains(allowed, e.Kind) || e.MaxHP != math.Round(d.HP*s.HP*s.HP) || e.Damage != math.Round(d.Damage*s.Damage*s.Damage) {
				t.Fatalf("Welle %d: %s mit HP %v, Schaden %v", w.Wave, e.Kind, e.MaxHP, e.Damage)
			}
		}
	}
	if len(seen) == 0 {
		t.Fatal("keine Gegner gespawnt")
	}
	t.Logf("gespawnt: %v", seen)
}
