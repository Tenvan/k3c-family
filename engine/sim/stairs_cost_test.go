package sim

import "testing"

// B-631/AC-02: Treppe runter kostet auf Tiefe d 100 Material der Stufe d, Treppe hoch 100 Material der Stufe d−1,
// je plus 50 Gold (data/buildings.json › costByDepth, siteCost).

func TestTreppenKostenJeTiefe(t *testing.T) {
	material := []Cost{{Wood: 100}, {Stone: 100}, {Copper: 100}, {Iron: 100}}
	for d := 0; d <= 4; d++ {
		w := newWorld(d, "b631", Options{})
		if d < 4 {
			want := addCost(material[d], Cost{Gold: 50})
			if got := siteCost(w, siteOfKind(w, "stairsDown")); got != want {
				t.Errorf("Tiefe %d: Treppe runter kostet %+v, erwartet %+v", d, got, want)
			}
		}
		if d > 0 {
			want := addCost(material[d-1], Cost{Gold: 50})
			if got := siteCost(w, siteOfKind(w, "stairsUp")); got != want {
				t.Errorf("Tiefe %d: Treppe hoch kostet %+v, erwartet %+v", d, got, want)
			}
		}
	}
}

// Höhle: Zwei Spieler bezahlen die Treppe hoch, der Bau zieht 100 Holz aus dem Vorrat (kein Stein).
func TestTreppeHochInDerHoehleKostetHolz(t *testing.T) {
	w := newWorld(1, "b631", Options{})
	w.Troops, w.Camps, w.Pickups = []*Troop{}, []*Camp{}, []*Pickup{}
	w.CycleSpeed = 1e-6
	peasant := spawnVagrant(w, w.HubX, w.HubX)
	peasant.Kind, peasant.HP, peasant.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
	players := []*Player{AddPlayer(w), AddPlayer(w)}
	w.HubLevel = 2
	stairs := siteOfKind(w, "stairsUp")
	buildNew(t, w, players, stairs)
	if stairs.State != "built" || *w.Stock != (Stock{}) {
		t.Errorf("Treppe hoch: %s, Vorrat danach %+v", stairs.State, *w.Stock)
	}
}

// Ohne Eintrag für die Tiefe gilt cost der Art.
func TestSiteCostRueckfallAufCost(t *testing.T) {
	w := quietWorld(t)
	if s := siteOfKind(w, "workshop"); siteCost(w, s) != buildings["workshop"].Cost {
		t.Errorf("Werkstatt: %+v, erwartet %+v", siteCost(w, s), buildings["workshop"].Cost)
	}
}
