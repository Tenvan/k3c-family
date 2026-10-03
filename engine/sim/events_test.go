package sim

import (
	"reflect"
	"testing"
)

// Feedback-Ereignisse im Kampf (F3.1, B-139/AC-01, AC-02).

// eventsOf sammelt alle Ereignisse eines Typs über n Schritte.
func eventsOf(w *World, typ string, steps int, commands []PlayerCommand) []Event {
	var found []Event
	for range steps {
		Step(w, commands, dt)
		for _, ev := range w.Events {
			if ev["type"] == typ {
				found = append(found, ev)
			}
		}
	}
	return found
}

func TestEventsTrefferAufSpielerMitIndex1(t *testing.T) {
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	p0.X, p1.X = w.HubX+40, w.HubX-40
	p1.Gold = 0 // sonst stiehlt der Goblin nur Gold
	spawnEnemy(w, "goblin", p1.X-0.5)
	var onP1 Event
	for _, h := range eventsOf(w, "hit", 60, []PlayerCommand{{}, {}}) {
		if h["target"] == "player" && h["id"] == 0 {
			t.Fatalf("Spieler 0 steht weit weg und wurde getroffen: %v", h)
		}
		if h["target"] == "player" && h["id"] == 1 && onP1 == nil {
			onP1 = h
		}
	}
	if onP1 == nil || onP1["damage"].(float64) <= 0 || onP1["x"] != unitX(p1.X) {
		t.Fatalf("hit auf Spieler 1: %v (Spieler bei %v)", onP1, p1.X)
	}
}

func TestEventsSchlagDesGegners(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	p.Gold = 0
	e := spawnEnemy(w, "goblin", p.X-0.5)
	strikes := eventsOf(w, "strike", 60, []PlayerCommand{{}})
	if len(strikes) == 0 {
		t.Fatal("kein strike")
	}
	if s := strikes[len(strikes)-1]; s["from"] != e.ID || s["x"] != unitX(e.X) { // der Goblin steht beim Zuschlagen
		t.Fatalf("strike: %v, Gegner %d", s, e.ID)
	}
}

func TestEventsPfeilDesGegnersUndTreffer(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	p.Gold = 0
	e := spawnEnemy(w, "goblinArcher", p.X-5)
	var arrows, hits []Event
	for range 90 {
		Step(w, []PlayerCommand{{}}, dt)
		for _, ev := range w.Events {
			switch ev["type"] {
			case "arrow":
				arrows = append(arrows, ev)
			case "hit":
				hits = append(hits, ev)
			}
		}
	}
	if len(arrows) == 0 || len(hits) == 0 {
		t.Fatalf("Pfeile %v, Treffer %v", arrows, hits)
	}
	a := arrows[0]
	if a["from"] != e.ID || a["team"] != "enemy" || a["to"] != p.ID {
		t.Fatalf("arrow: %v (Gegner %d, Spieler-ID %d)", a, e.ID, p.ID)
	}
	if hits[0]["target"] != "player" || hits[0]["id"] != p.Index {
		t.Fatalf("hit: %v", hits[0])
	}
}

func TestEventsPfeilDesBogenschuetzen(t *testing.T) {
	w := quietWorld(t)
	archer := &Troop{ID: 950, Kind: "archer", X: w.HubX, AnchorX: w.HubX, HP: 10, MaxHP: 10}
	w.Troops = []*Troop{archer}
	e := spawnEnemy(w, "goblin", w.HubX-3)
	arrows := eventsOf(w, "arrow", 60, nil)
	if len(arrows) == 0 {
		t.Fatal("kein arrow")
	}
	if a := arrows[0]; a["from"] != archer.ID || a["to"] != e.ID || a["team"] != "player" {
		t.Fatalf("arrow: %v", a)
	}
}

func TestEventsKillMitGold(t *testing.T) {
	w := quietWorld(t)
	e := spawnEnemy(w, "goblin", w.HubX-20)
	e.HP, e.CarriedGold = 0, 3
	e.HomeX = e.X - 50 // sonst verschwindet der „flüchtende“ tote Goblin im Portal (B-189)
	coins := len(w.Coins)
	kills := eventsOf(w, "kill", 1, nil)
	if len(kills) != 1 {
		t.Fatalf("kill: %v", kills)
	}
	k := kills[0]
	gold := enemyData["goblin"].Gold
	if k["kind"] != "goblin" || k["x"] != unitX(e.X) || k["gold"].(int) < gold[0]+3 || k["gold"].(int) > gold[1]+3 {
		t.Fatalf("kill: %v", k)
	}
	if len(w.Coins)-coins != k["gold"].(int) {
		t.Fatalf("Münzen gestreut %d, Ereignis %v", len(w.Coins)-coins, k["gold"])
	}
}

// Zwei Läufe mit gleichem Seed und gleichen Eingaben, Nacht mit Gegnern: gleiche Ereignisfolge (B-139/AC-02).
func TestEventsDeterministisch(t *testing.T) {
	record := func() [][]Event {
		w := newWorld(0, "events", Options{})
		AddPlayer(w)
		AddPlayer(w)
		w.Troops = append(w.Troops, &Troop{ID: 950, Kind: "archer", X: w.HubX, AnchorX: w.HubX - 1, HP: 10, MaxHP: 10})
		for i := range 3 {
			spawnEnemy(w, "goblin", w.HubX-12-float64(i)*3)
		}
		spawnEnemy(w, "goblinArcher", w.HubX+14)
		var ticks [][]Event
		for i := range 600 {
			Step(w, []PlayerCommand{{MoveX: float64(i%3 - 1)}, {Pay: i%50 == 0}}, dt)
			ticks = append(ticks, append([]Event(nil), w.Events...))
		}
		return ticks
	}
	a, b := record(), record()
	kampf := 0
	for _, tick := range a {
		for _, ev := range tick {
			switch ev["type"] {
			case "hit", "kill", "arrow", "strike":
				kampf++
			}
		}
	}
	if kampf == 0 {
		t.Fatal("keine Kampf-Ereignisse in 600 Ticks")
	}
	if !reflect.DeepEqual(a, b) {
		t.Fatal("Ereignisfolgen weichen ab")
	}
}
