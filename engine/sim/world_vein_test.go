package sim

import "testing"

// W2.1 (B-114/AC-02, AC-04, Sprint W2 AC-02): Adern liefern mit 2 Bauern die Zielrate, höchstens 2 Bauern, einmal
// markiert. Werte aus data/economy.json › veins.

// veinWorld ist eine ruhige Höhle ohne Nacht mit der ersten Ader direkt neben der Burg (ohne Trageweg) und n Bauern.
func veinWorld(t *testing.T, peasants int) (*World, *ResourceNode) {
	t.Helper()
	w := newWorld(1, "adern", Options{})
	w.Troops, w.Camps, w.Pickups, w.CycleSpeed = []*Troop{}, []*Camp{}, []*Pickup{}, 1e-6
	var vein *ResourceNode
	for _, n := range w.Nodes {
		if isVein(n) && vein == nil {
			vein = n
		}
	}
	if vein == nil {
		t.Fatal("keine Ader in der Höhle")
	}
	vein.X = w.HubX + 1
	for range peasants {
		p := spawnVagrant(w, w.HubX, w.HubX)
		p.Kind, p.HP, p.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
	}
	return w, vein
}

func TestAderZielrateMitZweiBauern(t *testing.T) {
	const minutes = 10
	for kind, v := range economy.Veins {
		w, vein := veinWorld(t, 2)
		vein.Kind, vein.Marked = kind, true
		run(w, minutes*60+2) // 2 s für die Wege zwischen Ader und Burg
		got := float64(*stockField(w.Stock, v.Resource)) / minutes
		want := 2 * float64(v.Amount) / v.WorkSeconds * 60
		if got < 0.9*want || got > 1.1*want {
			t.Errorf("%s: %.1f/min, Zielrate %.1f/min", kind, got, want)
		}
		if nodeByID(w, vein.ID) == nil || !vein.Marked {
			t.Errorf("%s: Ader nach dem Abbau weg oder nicht mehr markiert", kind)
		}
	}
}

func TestAderHoechstensZweiBauern(t *testing.T) {
	w, vein := veinWorld(t, 3)
	vein.Marked = true
	most := 0
	for range 30 * 60 {
		Step(w, nil, dt)
		most = max(most, veinWorkers(w, vein))
	}
	if limit := economy.Veins[vein.Kind].MaxWorkers; most != limit {
		t.Errorf("höchstens %d Bauern gleichzeitig, erwartet %d", most, limit)
	}
}

func TestAderRuhtOhneBauern(t *testing.T) {
	w, vein := veinWorld(t, 0)
	vein.Marked = true
	run(w, 60)
	if *w.Stock != (Stock{}) {
		t.Errorf("ohne Bauern geliefert: %+v", *w.Stock)
	}
}

// Zwei Spieler markieren eine Ader gemeinsam für ihren markCost; sie bleibt danach markiert.
func TestAderEinmalMarkieren(t *testing.T) {
	w, vein := veinWorld(t, 0)
	vein.Kind, vein.X = "copperVein", w.HubX+8 // außer Reichweite der Burg (Hub-Ausbau wäre das nähere Zahlziel)
	cost := economy.Veins[vein.Kind].MarkCost
	p0, p1 := AddPlayer(w), AddPlayer(w)
	gold := p0.Gold + p1.Gold
	payAt(w, []*Player{p0, p1}, vein.X, 10, func() bool { return vein.Marked })
	run(w, 1)
	if !vein.Marked || gold-p0.Gold-p1.Gold != cost {
		t.Errorf("markiert %v, bezahlt %d, markCost %d", vein.Marked, gold-p0.Gold-p1.Gold, cost)
	}
}

// Lager voll → der Bauer nimmt die Ader nicht an.
func TestAderLagerVoll(t *testing.T) {
	isl := mustIsland(t, "adern-voll", []int{0, 1})
	w := isl.Stages[1]
	w.Troops, w.CycleSpeed = []*Troop{}, 1e-6
	limit, _ := capacity(w)
	isl.Stock.Stone = limit
	vein := (*ResourceNode)(nil)
	for _, n := range w.Nodes {
		if n.Kind == "stoneVein" && vein == nil {
			vein = n
		}
	}
	vein.Marked = true
	p := spawnVagrant(w, w.HubX, w.HubX)
	p.Kind = "peasant"
	runIsland(isl, nil, 5)
	if veinWorkers(w, vein) != 0 {
		t.Error("Bauer arbeitet trotz vollem Lager an der Ader")
	}
}

// Adern sind kein endlicher Startvorrat: nicht in gatherables, Level-Objekte bleiben beim Ader-Abbau gleich.
func TestAderNichtImStartvorrat(t *testing.T) {
	for kind := range economy.Veins {
		if _, ok := economy.Gatherables[kind]; ok {
			t.Errorf("%s steht in gatherables", kind)
		}
	}
	w, vein := veinWorld(t, 2)
	vein.Marked = true
	before := len(w.Nodes)
	run(w, 120)
	if len(w.Nodes) != before {
		t.Errorf("%d Knoten statt %d", len(w.Nodes), before)
	}
}
