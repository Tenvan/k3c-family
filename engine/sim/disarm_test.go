package sim

import (
	"math"
	"slices"
	"testing"
)

// W4.3a (B-122/AC-04, Sprint W4 AC-05): Verlust-Kaskade statt Tod (Q67) und Ereignis disarmed (Q69).

// hitUntil lässt einen Skelett-Gegner neben tr los, bis done gilt, und sammelt die disarmed-Ereignisse.
func hitUntil(t *testing.T, w *World, tr *Troop, done func() bool) []Event {
	t.Helper()
	tr.HP = 1
	spawnEnemy(w, "skeleton", tr.X-1)
	var got []Event
	for tm := 0.0; tm < 20; tm += dt {
		Step(w, nil, dt)
		for _, e := range w.Events {
			if e["type"] == "disarmed" {
				got = append(got, e)
			}
		}
		if done() {
			w.Enemies = []*Enemy{}
			return got
		}
	}
	t.Fatalf("Treffer ohne Wirkung: %s mit %v HP", tr.Kind, tr.HP)
	return nil
}

func TestDisarmKriegerWirdBauerUndHebtSchwertAuf(t *testing.T) {
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	tr := w.Troops[0]
	tr.Hammer = false
	makeFighter(w, tr, "warrior")
	tr.X = w.HubX - 5
	events := hitUntil(t, w, tr, func() bool { return tr.Kind != "warrior" })
	if tr.Kind != "peasant" || tr.HP != troops["peasant"].HP || len(w.Drops) != 1 || w.Drops[0].Kind != "warrior" {
		t.Fatalf("nach dem Treffer: %s mit %v HP, Ausrüstung am Boden %v", tr.Kind, tr.HP, w.Drops)
	}
	if len(events) != 1 || events[0]["kind"] != "warrior" || events[0]["cause"] != "skeleton" || events[0]["x"] != unitX(w.Drops[0].X) {
		t.Errorf("disarmed: %v", events)
	}
	if fighters(w) != 0 {
		t.Errorf("entwaffneter Krieger zählt noch zum Limit: %d", fighters(w))
	}
	if runUntil(w, 30, func() bool { return tr.Kind == "warrior" }) < 0 || len(w.Drops) != 0 {
		t.Errorf("Schwert nicht wieder aufgehoben: %s, am Boden %d", tr.Kind, len(w.Drops))
	}
}

func TestDisarmBerufFaelltZuBoden(t *testing.T) {
	w := quietWorld(t)
	tr := addPeasant(w, w.HubX-5, "miner")
	events := hitUntil(t, w, tr, func() bool { return tr.Profession == "" })
	if len(events) != 1 || events[0]["kind"] != "miner" || len(w.Drops) != 1 || tr.Kind != "peasant" {
		t.Fatalf("Bergmann: %s %q, Ereignisse %v, am Boden %d", tr.Kind, tr.Profession, events, len(w.Drops))
	}
	if runUntil(w, 30, func() bool { return tr.Profession == "miner" }) < 0 {
		t.Error("Bergmann-Ausrüstung nicht wieder aufgehoben")
	}
}

// B-928: Ein Treffer macht nie einen Landstreicher; ein getroffener Bauer behält Status und Münze und flieht zur Burg.
func TestTrefferMachtKeinenLandstreicher(t *testing.T) {
	w := quietWorld(t)
	tr := addPeasant(w, w.HubX-5, "")
	coins := len(w.Coins)
	events := hitUntil(t, w, tr, func() bool { return tr.fleeing })
	if len(events) != 0 || tr.Kind != "peasant" {
		t.Fatalf("Treffer: Ereignisse %v, %s", events, tr.Kind)
	}
	tr.HP = 0
	loseLayer(w, tr) // Kampf-HP aufgebraucht: keine Kaskade
	if tr.Kind != "peasant" || len(w.Coins) != coins || tr.HP != troops["peasant"].HP || !tr.fleeing || len(w.Drops) != 0 {
		t.Fatalf("Bauer bei HP 0: %s, Münzen %d → %d, HP %v, flieht %v", tr.Kind, coins, len(w.Coins), tr.HP, tr.fleeing)
	}
	tr.X = w.HubX - 40
	run(w, 30)
	if tr.fleeing || math.Abs(tr.X-w.HubX) > w.values().Hub.HomeRadiusUnits+1 {
		t.Errorf("Bauer nicht an der Burg angekommen: x %.1f, flieht %v", tr.X, tr.fleeing)
	}
}

// Beim Burgfall gibt es kein disarmed.
func TestDisarmNichtBeimBurgfall(t *testing.T) {
	w := dayWorld(t)
	makeFighter(w, w.Troops[0], "warrior")
	w.Castle.HP = 0
	Step(w, nil, dt)
	if !slices.ContainsFunc(w.Events, func(e Event) bool { return e["type"] == "castleFallen" }) {
		t.Fatal("kein Burgfall")
	}
	if slices.ContainsFunc(w.Events, func(e Event) bool { return e["type"] == "disarmed" }) || len(w.Drops) != 0 {
		t.Errorf("disarmed beim Burgfall: %v, am Boden %d", w.Events, len(w.Drops))
	}
}

// B-928: Die Flucht verwirft ein getragenes Material nicht.
func TestFluchtBehaeltDenTrageAuftrag(t *testing.T) {
	w := quietWorld(t)
	tr := addPeasant(w, w.HubX-30, "")
	tr.Job = &Job{Type: "carry", Resource: "wood", Amount: 3}
	tr.fleeing = true
	run(w, 2)
	if tr.Job == nil || tr.Job.Type != "carry" || tr.Job.Amount != 3 {
		t.Fatalf("Trage-Auftrag verloren: %+v", tr.Job)
	}
}
