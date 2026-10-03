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

// F3.2: Münzen, Bau, Wiederbeleben, die auf bestehende Typen abgebildeten Ereignisse und die Obergrenze.

func firstEvent(t *testing.T, events []Event) Event {
	t.Helper()
	if len(events) == 0 {
		t.Fatal("kein Ereignis")
	}
	return events[0]
}

func TestEventsMuenzeAufgehobenVonSpieler1(t *testing.T) {
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	p0.X, p1.X, p1.Gold = w.HubX-30, w.HubX+30, 0
	w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: p1.X})
	ev := firstEvent(t, eventsOf(w, "coinPickup", 1, []PlayerCommand{{}, {}}))
	if ev["player"] != 1 || ev["x"] != unitX(p1.X) || p1.Gold != 1 {
		t.Fatalf("coinPickup: %v, Gold %d", ev, p1.Gold)
	}
}

func TestEventsMuenzeGegebenAnBauplatz(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	wall := leftWall(t, w)
	wall.State, wall.HP = "unpaid", 0
	p.X, p.Gold = wall.X, 10
	ev := firstEvent(t, eventsOf(w, "coinGive", 30, []PlayerCommand{{Pay: true}}))
	if ev["player"] != 0 || ev["to"] != "site" || ev["x"] != unitX(wall.X) {
		t.Fatalf("coinGive: %v", ev)
	}
}

func TestEventsBaufortschrittInViertelnUndFertig(t *testing.T) {
	w := quietWorld(t)
	wall := leftWall(t, w)
	wall.State, wall.HP, wall.BuildProgress = "waitingWorker", 0, 0
	w.Troops = []*Troop{{ID: 960, Kind: "peasant", X: wall.X, AnchorX: wall.X, HP: 1, MaxHP: 1, Job: &Job{Type: "build", SiteID: wall.ID}}}
	wall.WorkerID = intPtr(960)
	var percents []any
	built := false
	for range 30 * 60 {
		Step(w, nil, dt)
		for _, ev := range w.Events {
			switch ev["type"] {
			case "buildProgress":
				if ev["site"] != wall.ID || ev["kind"] != "wall" || ev["x"] != unitX(wall.X) {
					t.Fatalf("buildProgress: %v", ev)
				}
				percents = append(percents, ev["percent"])
			case "built":
				built = true
			}
		}
		if built {
			break
		}
	}
	if !built || !reflect.DeepEqual(percents, []any{25, 50, 75}) {
		t.Fatalf("fertig %v, Fortschritt %v", built, percents)
	}
}

func TestEventsTodUndWiederbelebenSpieler1(t *testing.T) {
	w := quietWorld(t)
	AddPlayer(w)
	p1 := AddPlayer(w)
	applyDamage(w, p1.ID, 1e6)
	down := firstEvent(t, w.Events[len(w.Events)-1:])
	if down["type"] != "playerDown" || down["player"] != 1 {
		t.Fatalf("Tod: %v", down)
	}
	ev := firstEvent(t, eventsOf(w, "revive", 30*10, []PlayerCommand{{}, {}}))
	if ev["player"] != 1 || ev["x"] != unitX(p1.X) {
		t.Fatalf("revive: %v (Spieler bei %v)", ev, p1.X)
	}
}

func TestEventsSkillPunktSpieler1(t *testing.T) {
	w := quietWorld(t)
	p0, p1 := AddPlayer(w), AddPlayer(w)
	p0.X, p1.X = w.HubX-30, w.HubX+30
	w.Pickups = []*Pickup{{ID: w.newID(), Kind: "skillPoint", X: p1.X}}
	if ev := firstEvent(t, eventsOf(w, "skillPoint", 1, []PlayerCommand{{}, {}})); ev["player"] != 1 {
		t.Fatalf("skillPoint: %v", ev)
	}
}

func TestEventsNachtNaht(t *testing.T) {
	w := newWorld(0, "abend", Options{CycleSpeed: 50})
	ev := firstEvent(t, eventsOf(w, "dusk", 30*60, nil))
	if len(ev) != 1 { // dusk trägt außer dem Typ nichts; Ort und Beteiligte gibt es nicht (ganze Stufe)
		t.Fatalf("dusk: %v", ev)
	}
}

// B-139/AC-03: Mehr als K Ereignisse in einem echten Tick (viele Gegner sterben, ein Bau wird fertig, ein Spieler fällt).
func TestEventsObergrenzeImTick(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	p.Gold, p.HP = 0, 1
	wall := leftWall(t, w)
	wall.State, wall.HP, wall.BuildProgress = "waitingWorker", 0, 0.9999
	w.Troops = []*Troop{{ID: 960, Kind: "peasant", X: wall.X, AnchorX: wall.X, HP: 1, MaxHP: 1, Job: &Job{Type: "build", SiteID: wall.ID}}}
	wall.WorkerID = intPtr(960)
	p.X = wall.X - 20 // vor der Mauer, damit der Goblin den Spieler angreift und nicht die Burg
	spawnEnemy(w, "goblin", p.X-0.5)
	for i := range maxEventsPerTick + 8 {
		e := spawnEnemy(w, "goblin", w.HubX+20+float64(i))
		e.HP, e.HomeX = 0, e.X+50
	}
	Step(w, []PlayerCommand{{}}, dt)
	types := map[any]bool{}
	for _, ev := range w.Events {
		types[ev["type"]] = true
	}
	if len(w.Events) != maxEventsPerTick || w.EventsDropped == 0 || !types["playerDown"] || !types["built"] {
		t.Fatalf("%d Ereignisse, verworfen %d, Typen %v", len(w.Events), w.EventsDropped, types)
	}
}

// Priorität auch dann, wenn Tod und Bau am Ende der Liste stehen; Reihenfolge der behaltenen bleibt.
func TestEventsObergrenzePrioritaet(t *testing.T) {
	w := quietWorld(t)
	for i := range maxEventsPerTick + 5 {
		w.Events = append(w.Events, Event{"type": "hit", "id": i})
	}
	w.Events = append(w.Events, Event{"type": "built"}, Event{"type": "playerDown"}, Event{"type": "buildProgress"})
	total := len(w.Events)
	capEvents(w)
	n := len(w.Events)
	if n != maxEventsPerTick || w.EventsDropped != total-n {
		t.Fatalf("%d behalten, %d verworfen von %d", n, w.EventsDropped, total)
	}
	tail := []any{w.Events[n-3]["type"], w.Events[n-2]["type"], w.Events[n-1]["type"]}
	if !reflect.DeepEqual(tail, []any{"built", "playerDown", "buildProgress"}) || w.Events[0]["id"] != 0 {
		t.Fatalf("Reihenfolge: %v, erstes %v", tail, w.Events[0])
	}
	w.Events = []Event{{"type": "hit"}}
	capEvents(w)
	if w.EventsDropped != 0 || len(w.Events) != 1 {
		t.Fatalf("unter K: %d verworfen", w.EventsDropped)
	}
}
