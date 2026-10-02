package sim

import (
	"math"
	"reflect"
	"testing"
)

// Tests SP13.3: Träger bringen Material in Inseln zum nächsten Lager oder zur Burg; bei vollem Maximum wartet der Bauer.

// carrier stellt einen tagsüber wartenden Träger mit Holz bei x in die Welt.
func carrier(w *World, x float64, amount int) *Troop {
	w.Cycle.Phase, w.Enemies = "day", nil
	t := &Troop{ID: 900, Kind: "peasant", X: x, HP: 1, MaxHP: 1, Job: &Job{Type: "carry", Resource: "wood", Amount: amount}}
	w.Troops = append(w.Troops, t)
	return t
}

func runCarry(w *World, t *Troop, ticks int) {
	for range ticks {
		if t.Job == nil {
			return
		}
		w.Events = nil
		carry(w, t, dt)
	}
}

func TestIslandCarryNearestDeposit(t *testing.T) {
	isl := mustIsland(t, "trag-a", []int{0})
	w := isl.Stages[0]
	s := storageSite(w)
	s.State, s.X = "built", w.HubX+30
	p := carrier(w, w.HubX+25, 40) // Lager näher als die Burg
	runCarry(w, p, 2000)
	if p.Job != nil || math.Abs(p.X-s.X) > 1 || isl.Stock.Wood != 40 {
		t.Errorf("Lager: Job %v, X %v (Lager %v), Holz %d", p.Job, p.X, s.X, isl.Stock.Wood)
	}
	p = carrier(w, w.HubX+5, 10) // Burg näher
	runCarry(w, p, 2000)
	if math.Abs(p.X-w.HubX) > 1 || isl.Stock.Wood != 50 {
		t.Errorf("Burg: X %v (Hub %v), Holz %d", p.X, w.HubX, isl.Stock.Wood)
	}
	s.X = w.HubX + 10 // Gleichstand: Burg vor Lager
	if got := depositPoint(w, w.HubX+5); got != w.HubX {
		t.Errorf("Gleichstand: %v, erwartet Burg %v", got, w.HubX)
	}
}

func TestIslandCarryWaitsWhenFull(t *testing.T) {
	isl := mustIsland(t, "trag-b", []int{0})
	w := isl.Stages[0]
	isl.Stock.Wood = 290 // Maximum 300: Teilaufnahme
	p := carrier(w, w.HubX, 25)
	runCarry(w, p, 5)
	if p.Job == nil || p.Job.Amount != 15 || isl.Stock.Wood != 300 {
		t.Fatalf("Teilaufnahme: Job %+v, Holz %d, erwartet Rest 15 und 300", p.Job, isl.Stock.Wood)
	}
	var gathered []Event
	for range 200 { // voll: warten, nichts geht verloren, kein Ereignis
		w.Events = nil
		carry(w, p, dt)
		gathered = append(gathered, w.Events...)
	}
	if p.Job == nil || p.Job.Amount != 15 || isl.Stock.Wood != 300 || count(gathered, "gathered") != 0 {
		t.Fatalf("Warten: Job %+v, Holz %d, Ereignisse %d", p.Job, isl.Stock.Wood, count(gathered, "gathered"))
	}
	isl.Stock.Wood = 100 // Platz frei: Rest wird abgegeben
	runCarry(w, p, 1)
	if p.Job != nil || isl.Stock.Wood != 115 || count(w.Events, "gathered") != 1 || w.Events[0]["amount"] != 15 {
		t.Errorf("Abgabe: Job %+v, Holz %d, Ereignisse %v", p.Job, isl.Stock.Wood, w.Events)
	}
}

func TestIslandCarryDanger(t *testing.T) {
	isl := mustIsland(t, "trag-c", []int{0})
	w := isl.Stages[0]
	s := storageSite(w)
	s.State, s.X = "built", w.HubX+30
	p := carrier(w, w.HubX+29, 10)
	w.Cycle.Phase = "night"
	runCarry(w, p, 2000)
	if math.Abs(p.X-w.HubX) > 1 || isl.Stock.Wood != 10 {
		t.Errorf("Gefahr: X %v (Hub %v), Holz %d", p.X, w.HubX, isl.Stock.Wood)
	}
}

func TestIslandCarryNoIslandUnchanged(t *testing.T) {
	w, err := CreateWorld(biomeForDepth(0), "trag-d", Options{CycleSpeed: islandSpeed})
	if err != nil {
		t.Fatal(err)
	}
	w.Stock.Wood = 5000 // kein Maximum
	p := carrier(w, w.HubX+3, 40)
	runCarry(w, p, 500)
	if p.Job != nil || w.Stock.Wood != 5040 {
		t.Errorf("ohne Insel: Job %v, Holz %d", p.Job, w.Stock.Wood)
	}
}

func TestIslandCarryDeterministic(t *testing.T) {
	run := func() []Event {
		isl := mustIsland(t, "trag-e", []int{0, 1})
		isl.Stock.Wood = 280
		p := carrier(isl.Stages[0], isl.Stages[0].HubX+8, 50)
		var ev []Event
		for range 300 {
			if p.Job == nil {
				break
			}
			isl.Stages[0].Events = nil
			carry(isl.Stages[0], p, dt)
			ev = append(ev, isl.Stages[0].Events...)
		}
		return ev
	}
	if a, b := run(), run(); !reflect.DeepEqual(a, b) {
		t.Errorf("nicht deterministisch: %v vs %v", a, b)
	}
}

// Review SP13.4: Ein wartender Träger darf den Bau nicht blockieren und behält sein Material.
func TestIslandFullCarrierStillBuildsAndKeepsMaterial(t *testing.T) {
	isl := mustIsland(t, "trag-c", []int{0})
	w := isl.Stages[0]
	isl.Stock.Wood = 300 // voll
	p := carrier(w, w.HubX, 10)
	w.Troops = []*Troop{p}
	var site *Site
	for _, s := range w.Sites {
		if s.Kind == "wall" {
			site = s
			break
		}
	}
	site.State = "waitingWorker"
	for range 2000 {
		w.Cycle.Phase, w.Enemies = "day", nil
		stepPeasant(w, p, dt)
		if site.State == "built" {
			break
		}
	}
	if site.State != "built" {
		t.Fatal("der wartende Träger muss den Bauplatz bauen")
	}
	stepPeasant(w, p, dt)
	if p.Job == nil || p.Job.Type != "carry" || p.Job.Amount != 10 {
		t.Fatalf("danach trägt er das behaltene Material weiter: %+v", p.Job)
	}
	isl.Stock.Wood = 0 // Platz
	runCarry(w, p, 2000)
	if p.Job != nil || isl.Stock.Wood != 10 {
		t.Errorf("Abgabe nach dem Bau: Job %+v, Holz %d", p.Job, isl.Stock.Wood)
	}
}

func TestIslandFindJobSkipsFullResource(t *testing.T) {
	isl := mustIsland(t, "trag-d", []int{0})
	w := isl.Stages[0]
	w.Cycle.Phase, w.Enemies = "day", nil
	for _, n := range w.Nodes {
		n.Marked = true
	}
	peasant := &Troop{ID: 901, Kind: "peasant", X: w.HubX, HP: 1, MaxHP: 1}
	isl.Stock.Wood = 300 // Bäume liefern Holz: voll
	for _, s := range w.Sites {
		s.State = "unpaid"
	}
	if j := findJob(w, peasant); j != nil && j.Type == "gather" && economy.Gatherables[nodeByID(w, j.NodeID).Kind].Resource == "wood" {
		t.Error("ein volles Holzlager darf keine Sammelaufträge für Holz vergeben")
	}
}

// Ein wartender Träger holt keinen Bogen (er würde zum Bogenschützen und verlöre das behaltene Material).
func TestIslandFullCarrierDoesNotFetchBow(t *testing.T) {
	isl := mustIsland(t, "trag-e", []int{0})
	w := isl.Stages[0]
	isl.Stock.Wood = 300
	p := carrier(w, w.HubX, 10)
	w.Troops = []*Troop{p}
	for _, s := range w.Sites {
		if s.Kind == "workshop" {
			s.State, s.Bows = "built", 1
		}
	}
	for range 30 {
		stepPeasant(w, p, dt)
	}
	if p.Job == nil || p.Job.Type != "carry" || p.Kind != "peasant" || p.carried != nil {
		t.Errorf("wartender Träger bleibt Träger: Kind %s, Job %+v, carried %v", p.Kind, p.Job, p.carried)
	}
}
