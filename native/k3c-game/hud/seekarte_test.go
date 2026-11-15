package hud

import (
	"errors"
	"slices"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// seaSession nimmt die Wahlen der Seekarte entgegen (Spieler, Insel).
type seaSession struct {
	view.Session
	picks []string
	err   error
}

func (s *seaSession) ChooseIsland(p int, id string) error {
	s.picks = append(s.picks, string(rune('0'+p))+":"+id)
	return s.err
}

// Welt mit Etappen: a (1), b und c (2), d (3); Abgelegt wird von a, Ziele sind b und c.
var seaRoute = []view.RouteIsland{{ID: "a", Name: "A", Etappe: 1}, {ID: "b", Name: "B", Etappe: 2},
	{ID: "c", Name: "C", Etappe: 2}, {ID: "d", Name: "D", Etappe: 3}}

func openSwitch(choice string, completed ...string) *sim.SwitchState {
	return &sim.SwitchState{Open: true, Choice: choice, Completed: completed,
		Targets: []sim.SwitchTarget{{ID: "b", Name: "B", Etappe: 2}, {ID: "c", Name: "C", Etappe: 2}}}
}

// seaWorld: zwei Monarchen am Schiff (Spieler 0 Tastatur, 1 Pad), Frame mit Route.
func seaWorld(t *testing.T) (*sim.World, *view.Frame) {
	t.Helper()
	w, f := twoPlayers(t, 0)
	w.Sites = append(w.Sites, &sim.Site{Kind: shipKind, X: 50})
	w.Players[0].X, w.Players[1].X = 50, 51
	f.Route = seaRoute
	return w, f
}

func kindsOf(r seaRow) []seaKind {
	var k []seaKind
	for _, n := range r.nodes {
		k = append(k, n.kind)
	}
	return k
}

// AC-01: Overlay nur bei offenem Wechselpunkt; Ziele, Abgeschlossene und Gesperrte nach Etappen.
func TestSeekarteNurBeiOffenemWechselpunkt(t *testing.T) {
	w, f := seaWorld(t)
	h := New(nil).(*hud)
	h.Listen(f)
	if h.seats[0].sea.open || h.seats[1].sea.open {
		t.Fatal("ohne Wechselpunkt keine Seekarte")
	}
	w.IslandSwitch = openSwitch("b")
	h.Listen(f)
	v := &h.seats[0].sea
	if !v.open || !h.seats[1].sea.open {
		t.Fatal("offener Wechselpunkt: Seekarte in beiden Zellen")
	}
	if len(v.rows) != 3 || v.rows[0].etappe != 1 || v.rows[2].etappe != 3 {
		t.Fatalf("drei Etappen untereinander: %+v", v.rows)
	}
	if !slices.Equal(kindsOf(v.rows[0]), []seaKind{seaDone}) || !slices.Equal(kindsOf(v.rows[1]), []seaKind{seaTarget, seaTarget}) ||
		!slices.Equal(kindsOf(v.rows[2]), []seaKind{seaLocked}) {
		t.Errorf("a abgeschlossen, b und c Ziele, d gesperrt: %+v", v.rows)
	}
	if !v.rows[1].nodes[0].chosen || v.rows[1].nodes[1].chosen || !v.rows[1].nodes[0].cursor {
		t.Error("Wahl b gilt, Cursor beginnt auf der Wahl")
	}
	w.IslandSwitch = nil
	h.Listen(f)
	if h.seats[0].sea.open {
		t.Error("Wechselpunkt geschlossen (Schiff verloren): Seekarte weg")
	}
}

func TestSeekarteAbgeschlosseneUndEinZiel(t *testing.T) {
	w, f := seaWorld(t)
	w.IslandSwitch = &sim.SwitchState{Open: true, Choice: "d", Completed: []string{"a", "b", "c"},
		Targets: []sim.SwitchTarget{{ID: "d", Name: "D", Etappe: 3}}}
	h := New(nil).(*hud)
	h.Listen(f)
	v := &h.seats[0].sea
	for _, r := range v.rows[:2] {
		for _, n := range r.nodes {
			if n.kind != seaDone {
				t.Errorf("%s abgeschlossen", n.name)
			}
		}
	}
	if v.hint != tr(view.DefaultLanguage, "sea.hintOne") {
		t.Errorf("ein Ziel: Hinweis ohne Pfeile, war %q", v.hint)
	}
	if r := v.step(acts(view.ActionRight, view.ActionLeft)); r.moved || r.pick != "" {
		t.Error("ein Ziel: Links/Rechts bewegen nichts")
	}
}

// AC-02: Links/Rechts/A erzeugen die Wahl-Aktion, mit 2 Spielern; die Wahl eines Spielers ist für beide sichtbar.
func TestSeekarteEingabeZweiSpieler(t *testing.T) {
	w, f := seaWorld(t)
	w.IslandSwitch = openSwitch("b")
	h := New(nil).(*hud)
	h.Listen(f)
	s := &seaSession{}
	st := h.Update(input(acts(view.ActionRight, view.ActionConfirm), acts(view.ActionConfirm)), f, s)
	if !slices.Equal(s.picks, []string{"0:c", "1:b"}) {
		t.Errorf("Spieler 1 wählt c, Spieler 2 (eigener Cursor) b: %v", s.picks)
	}
	if st.Consumed[0] || st.Consumed[1] || !hasCue(st.Cues, view.CueMove, 0) || !hasCue(st.Cues, view.CueConfirm, 1) {
		t.Errorf("die Karte verbraucht nichts, meldet Cues: %v %v", st.Consumed, st.Cues)
	}
	w.IslandSwitch = openSwitch("c") // die Simulation hat die Wahl übernommen: beide sehen c
	h.Listen(f)
	for p := range 2 {
		if !h.seats[p].sea.rows[1].nodes[1].chosen || h.seats[p].sea.cursor != 1 {
			t.Errorf("Spieler %d sieht Wahl c", p+1)
		}
	}
}

func TestSeekarteKeineEingabeOhneKroneOderAmSteg(t *testing.T) {
	w, f := seaWorld(t)
	w.IslandSwitch = openSwitch("b")
	w.Players[0].Crown = &sim.Crown{} // ohne Krone: keine Wahl
	w.Players[1].X = 80               // fern vom Steg: Links/Rechts bewegen nur den Monarchen
	h := New(nil).(*hud)
	h.Listen(f)
	s := &seaSession{}
	h.Update(input(acts(view.ActionRight, view.ActionConfirm), acts(view.ActionRight, view.ActionConfirm)), f, s)
	if len(s.picks) != 0 || h.seats[1].sea.cursor != 0 {
		t.Errorf("keine Wahl, Cursor fest: %v", s.picks)
	}
	w.Players[1].X = 50
	h.Update(input(nil, acts(view.ActionBack)), f, s)
	st := h.Update(input(nil, acts(view.ActionBack)), f, s)
	if st.Consumed[1] || len(s.picks) != 0 {
		t.Error("B ist im Spiel unbelegt")
	}
	s.err = errors.New("insel: ohne Krone keine Wahl")
	st = h.Update(input(nil, acts(view.ActionConfirm)), f, s)
	if !hasCue(st.Cues, view.CueDenied, 1) {
		t.Error("abgelehnte Wahl: Cue")
	}
}
