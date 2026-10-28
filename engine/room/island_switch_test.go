package room

import (
	"fmt"
	"maps"
	"slices"
	"testing"

	"k3c/engine/sim"
)

// Inselwechsel im Raum (B-345, K4.2; WZ18.4, B-488/AC-03): Der Raum tauscht die Insel erst bei SwitchReady, in der Welt
// nordmeer (Test-Welt mit zwei Inseln, worlds_test.go) mit 2 Spielern gemeinsam zur nächsten Insel der Welt.

// nordmeerGate: Raum in der Welt nordmeer, Xbox (Monarch 0) und Handy (Monarch 1) in der tiefsten Stufe an der Burg.
func nordmeerGate(t *testing.T) (*fixture, *Room, *stagePeer, *stagePeer) {
	t.Helper()
	f, _ := worldFixture()
	x, h := &stagePeer{}, &stagePeer{}
	r := need(f.m.Create("xbox", x, "stufen", true, 4, []int{0}, Options{World: "nordmeer"}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	gate := r.isl.Stages[1]
	if len(gate.Players) != 2 || gate.Biome.Depth != 4 {
		t.Fatalf("Spieler in Tiefe %d: %d", gate.Biome.Depth, len(gate.Players))
	}
	boardHarbor(r.isl)
	return f, r, x, h
}

// boardHarbor baut das Schiff fertig und stellt alle Spieler (Stufe 1, tiefste) an den Steg des Hafens (Stufe 0).
func boardHarbor(isl *sim.Island) {
	sim.DevBuildAll(isl, 5)
	harbor, deep := isl.Stages[0], isl.Stages[1]
	for _, s := range harbor.Sites {
		if s.Kind != "ship" {
			continue
		}
		for _, p := range deep.Players {
			p.X = s.X
		}
	}
	harbor.Players, deep.Players = append(harbor.Players, deep.Players...), nil
}

func TestKampfInselwechselErstBeiSwitchReady(t *testing.T) {
	_, r, _, _ := nordmeerGate(t)
	old := r.isl
	ticks(r, 120) // Endboss nicht besiegt: kein Wechselpunkt
	if r.isl != old {
		t.Fatal("Insel getauscht ohne SwitchReady")
	}
}

// AC-03 (Raum): Endboss besiegt, beide Spieler am Wechselpunkt → gemeinsamer Wechsel zu Insel 2 der Welt; der
// gespeicherte Stand nennt Welt und Insel 2.
func TestKampfInselwechselTauschtMitGeraeten(t *testing.T) {
	f, r, x, h := nordmeerGate(t)
	old, xi, hi := r.isl, len(x.msgs), len(h.msgs)
	old.EndbossDefeated = true
	for i := 0; i < 120 && r.isl == old; i++ {
		ticks(r, 1)
	}
	if r.isl == old {
		t.Fatal("Insel nicht getauscht")
	}
	switchedTogether(t, r)
	// Level vor Zustand: Das Gerät beginnt die neue Insel mit einem vollen Zustand (net sendet nach Level immer snap).
	if !beginsWithLevel(x.since(xi), 0) || !beginsWithLevel(h.since(hi), 0) {
		t.Fatalf("Xbox %v, Handy %v", x.since(xi), h.since(hi))
	}
	r.saveNow()
	s := need(sim.ParseIslandSave(f.store.data["stufen"]))(t)
	if s.World != "nordmeer" || s.IslandID != "eisinsel" {
		t.Fatalf("Spielstand: Welt %q, Insel %q", s.World, s.IslandID)
	}
}

// switchedTogether: Der Raum steht auf Insel 2 der Welt nordmeer, beide Monarchen gesteuert in ihrer ersten Stufe, Slots
// unverändert.
func switchedTogether(t *testing.T, r *Room) {
	t.Helper()
	if r.isl.Number != 1 || r.isl.IslandID() != "eisinsel" || r.isl.WorldID() != "nordmeer" || r.isl.SwitchReady {
		t.Fatalf("Nummer %d, Insel %q, Welt %q, bereit %v", r.isl.Number, r.isl.IslandID(), r.isl.WorldID(), r.isl.SwitchReady)
	}
	for i := range 2 {
		if r.isl.StageOf(i) != 0 || r.monarchs[i].state != Taken {
			t.Fatalf("Monarch %d: Stufe %d, Zustand %v", i, r.isl.StageOf(i), r.monarchs[i].state)
		}
	}
	if x, h := r.devices["xbox"].slots, r.devices["handy"].slots; !maps.Equal(x, map[int]int{0: 0}) || !maps.Equal(h, map[int]int{0: 1}) {
		t.Fatalf("Slots Xbox %v, Handy %v", x, h)
	}
}

func TestKampfInselwechselLetzteInselOhneTausch(t *testing.T) {
	_, r, _, _ := zweiStufen(t)
	old := r.isl
	old.SwitchReady = true
	ticks(r, 5)
	if r.isl != old {
		t.Fatal("Insel getauscht, obwohl es keine nächste gibt")
	}
}

// switchPeer merkt sich je Zustand, ob er das Ereignis islandSwitch trägt (der Zustand ist ohne Manager.Snapshot die
// lebende Welt, deshalb beim Senden lesen).
type switchPeer struct{ stagePeer }

func (p *switchPeer) State(tick, stage int, s any) {
	p.stagePeer.State(tick, stage, s)
	if w, ok := s.(*sim.World); ok && slices.ContainsFunc(w.Events, func(e sim.Event) bool { return e["type"] == "islandSwitch" }) {
		p.msgs = append(p.msgs, fmt.Sprintf("islandSwitch %d", stage))
	}
}

// AC-01 (KB2.1, B-385): Beim Inselwechsel bekommt jedes Gerät den letzten Zustand der alten Insel mit dem Ereignis
// islandSwitch, danach seats, Level und Zustand der neuen Insel.
func TestKampfInselwechselEreignisErreichtGeraete(t *testing.T) {
	f, _ := worldFixture()
	x, h := &switchPeer{}, &switchPeer{}
	r := need(f.m.Create("xbox", x, "stufen", true, 4, []int{0}, Options{World: "nordmeer"}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	boardHarbor(r.isl)
	old, xi, hi := r.isl, len(x.msgs), len(h.msgs)
	old.EndbossDefeated = true
	for i := 0; i < 120 && r.isl == old; i++ {
		ticks(r, 1)
	}
	if r.isl == old {
		t.Fatal("Insel nicht getauscht")
	}
	want := []string{"state 0", "islandSwitch 0", "level 0", "state 0"} // der Steg liegt in Stufe 0 (Hafen)
	for name, got := range map[string][]string{"Xbox": x.since(xi), "Handy": h.since(hi)} {
		if i := slices.Index(got, "islandSwitch 0"); i < 1 || !slices.Equal(got[i-1:], want) {
			t.Errorf("%s: %v", name, got)
		}
	}
}
