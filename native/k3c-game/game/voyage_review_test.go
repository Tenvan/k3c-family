package game

import (
	"errors"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// deepHarbor ist der Frame am Hafen einer tieferen Stufe: Insel a aktuell, Ziel b; der Spieler steht in Stufe 2, alle
// Stufen der Insel tragen den Wechselpunkt (sim.mirrorSwitch).
func deepHarbor() view.Frame {
	sw := &sim.SwitchState{Open: true, Choice: "b", Completed: []string{"z", "a"}, Targets: []sim.SwitchTarget{{ID: "b"}}}
	return view.Frame{Seats: []view.Seat{{Player: 0, Device: keys1, Stage: 2}}, Stages: []view.Stage{
		{Index: 0, Level: "wald", World: &sim.World{IslandSwitch: sw}},
		{Index: 2, Level: "moor", World: &sim.World{IslandSwitch: sw}}}}
}

// GS2.4: Liegt der Hafen nicht in Stufe 0, trägt der Frame des Wechsels neben der neuen Insel (Spieler in Stufe 0) noch
// den letzten Zustand der verlassenen Stufe 2 der alten Insel samt Wechselpunkt (der Raum schickt erst die alte, dann
// die neue Insel). Die Fahrt beginnt trotzdem; nur ein Spieler, der weiter am Wechselpunkt steht, heißt „nicht
// getauscht“.
func TestVoyageStartsFromDeeperStage(t *testing.T) {
	sw := switched("b")
	sw.Stages = append(sw.Stages, deepHarbor().Stages[1]) // veraltete Stufe 2 der alten Insel
	d := &voyDrawer{ready: true}
	g, _, _ := voyGame(d, deepHarbor(), sw)
	g.advance()
	if g.advance(); !g.voy.on || d.last.From != "a" || d.last.To != "b" {
		t.Errorf("Wechsel aus Stufe 2: Fahrt %v von %q nach %q", g.voy.on, d.last.From, d.last.To)
	}
	stay := deepHarbor() // Fehler im Raum: der Spieler steht weiter in Stufe 2 am Wechselpunkt
	stay.Events = switched("b").Events
	g, _, _ = voyGame(&voyDrawer{ready: true}, deepHarbor(), stay)
	g.advance()
	if g.advance(); g.voy.on {
		t.Error("Insel nicht getauscht (Spieler in Stufe 2): Fahrt beginnt")
	}
}

// GS2.4: Schlägt das Laden der Zielinsel während der Fahrt fehl, meldet die Schleife den Fehler im Fenster (Draw zeigt
// ihn vor der Fahrt); nichts rechnet weiter, die Fahrt hängt nicht in der Ladeanzeige.
func TestVoyageLoadFailureIsFault(t *testing.T) {
	g, s, _ := voyGame(&voyDrawer{ready: true}, harbor("b"), harbor("b"), switched("b"))
	ld := &loader{}
	g.parts.Art, g.parts.Assets = ld, []view.AssetSource{assetsOf{"wald": {"a/b"}, "mine": {"c/d"}}}
	g.advance()
	g.shown = true
	g.advance()
	if g.advance(); !g.voy.on || !g.Loading() {
		t.Fatalf("Wechsel: Fahrt %v, lädt %v", g.voy.on, g.Loading())
	}
	ld.fail = errors.New("Grafik der Zielinsel fehlt")
	g.drawVoyage(nil) // ein Bild der Fahrt gibt das Laden frei
	g.advance()
	if g.Fault() == nil || !errors.Is(g.Fault(), ld.fail) || g.Loading() || g.err != nil {
		t.Fatalf("Ladefehler in der Fahrt: Fehler %v, lädt %v, err %v", g.Fault(), g.Loading(), g.err)
	}
	steps := s.steps
	for range int(voyageSeconds()*TPS) + 5 {
		g.advance()
	}
	if s.steps != steps || g.Fault() == nil {
		t.Errorf("nach dem Fehler: Takte der Simulation %d, Fehler %v", s.steps-steps, g.Fault())
	}
}

// GS2.4: Verliert ein Spieler während der Fahrt sein Pad, meldet die Schleife es einmal (Cue, Seat.Lost), die Fahrt
// läuft ungestört zu Ende, und kein Gerät übernimmt den Platz, solange sie läuft (auch nicht mit Bestätigen). Erst der
// erste Takt auf der Zielinsel ordnet neu zu.
func TestVoyageDeviceLost(t *testing.T) {
	a, b := harbor("b"), switched("b")
	a.Seats, b.Seats = []view.Seat{{Player: 0, Device: pad3}}, []view.Seat{{Player: 0, Device: pad3}}
	g, s, src := voyGame(&voyDrawer{ready: true}, a, b)
	src.in = devicesIn([]view.DeviceID{keys1, pad3})
	g.advance()
	if g.advance(); !g.voy.on {
		t.Fatal("Fahrt beginnt nicht")
	}
	steps := s.steps
	src.in = devicesIn([]view.DeviceID{keys1}, keys1) // Pad weg, die Tastatur drückt Bestätigen
	g.advance()
	if lostCues(g.frame.Cues) != 1 || !g.frame.Seats[0].Lost || !g.voy.on {
		t.Fatalf("Pad getrennt: Cues %+v, Platz %+v, Fahrt %v", g.frame.Cues, g.frame.Seats[0], g.voy.on)
	}
	for i := 2; i < int(voyageSeconds()*TPS); i++ {
		if g.advance(); !g.voy.on || len(g.frame.Cues) != 0 || !g.frame.Seats[0].Lost {
			t.Fatalf("Takt %d: Fahrt %v, Cues %+v, Platz %+v", i, g.voy.on, g.frame.Cues, g.frame.Seats[0])
		}
	}
	if g.advance(); g.voy.on || s.steps != steps || len(s.rebinds)+len(s.joins) != 0 {
		t.Fatalf("Ende der Fahrt: Fahrt %v, Takte %d, Neuzuordnungen %v, Beitritte %v", g.voy.on, s.steps-steps, s.rebinds, s.joins)
	}
	if g.advance(); s.steps != steps+1 || len(s.rebinds) != 1 || s.rebinds[0] != (Rebind{0, keys1}) {
		t.Errorf("erster Takt am Ziel: Takte %d, Neuzuordnungen %v", s.steps-steps, s.rebinds)
	}
}

// lostCues zählt die Meldungen „Pad 3 von Spieler 1 getrennt“.
func lostCues(cues []view.Cue) (n int) {
	for _, c := range cues {
		if c.Kind == view.CueDeviceLost && c.Player == 0 && c.Device == pad3 {
			n++
		}
	}
	return n
}
