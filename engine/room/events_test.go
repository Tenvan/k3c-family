package room

import (
	"testing"

	"k3c/engine/sim"
)

// eventPeer merkt sich die Ereignisse jedes Zustands, den das Gerät bekommt (die Welt wird beim nächsten Tick geleert).
type eventPeer struct {
	peer
	events [][]sim.Event
}

func (p *eventPeer) State(tick int, w *sim.World, scale int, paused bool) {
	p.peer.State(tick, w, scale, paused)
	p.events = append(p.events, append([]sim.Event(nil), w.Events...))
}

// count zählt die Ereignisse vom Typ typ im letzten Zustand und liefert das letzte davon.
func (p *eventPeer) count(typ string) (int, sim.Event) {
	n, last := 0, sim.Event(nil)
	if len(p.events) == 0 {
		return 0, nil
	}
	for _, ev := range p.events[len(p.events)-1] {
		if ev["type"] == typ {
			n, last = n+1, ev
		}
	}
	return n, last
}

// F4/AC-01 (B-140/AC-01): Ein Ereignis aus Stufe 1 erreicht nur das Gerät in Stufe 1, mit `stage` 1 und genau einmal.
func TestEventsNurImZustandDerBetroffenenStufe(t *testing.T) {
	f := newFixture()
	x, h := &eventPeer{}, &eventPeer{}
	r := need(f.m.Create("xbox", x, "ereignis", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	moveToExit(t, r, 0)
	ticks(r, 70)
	if r.isl.StageOf(0) != 1 || r.isl.StageOf(1) != 0 {
		t.Fatalf("Stufen: %d, %d", r.isl.StageOf(0), r.isl.StageOf(1))
	}
	cave := r.isl.Stages[1]
	var monarch *sim.Player
	for _, p := range cave.Players {
		if p.Index == 0 {
			monarch = p
		}
	}
	monarch.Gold = 0
	cave.Coins = append(cave.Coins, &sim.Coin{ID: 1 << 30, X: monarch.X})
	xs, hs := len(x.events), len(h.events)
	ticks(r, 1)
	if len(x.events) != xs+1 || len(h.events) != hs+1 {
		t.Fatalf("je Gerät ein Zustand je Tick erwartet: Xbox %d, Handy %d", len(x.events)-xs, len(h.events)-hs)
	}
	n, ev := x.count("coinPickup")
	if n != 1 || ev["stage"] != 1 || ev["player"] != 0 {
		t.Fatalf("Xbox (Stufe 1): %d× coinPickup, %v", n, x.events[len(x.events)-1])
	}
	if n, _ := h.count("coinPickup"); n != 0 {
		t.Fatalf("Handy (Stufe 0) bekam das Ereignis der Stufe 1: %v", h.events[len(h.events)-1])
	}
	ticks(r, 1)
	if n, _ := x.count("coinPickup"); n != 0 {
		t.Fatalf("Ereignis im nächsten Tick wiederholt: %v", x.events[len(x.events)-1])
	}
}
