package game

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// voySim ist eine Simulation, die je Takt den nächsten Frame der Liste liefert (danach den letzten ohne Ereignisse)
// und die Takte zählt.
type voySim struct {
	*fakeSim
	frames []view.Frame
	steps  int
}

func (s *voySim) Step() view.Frame {
	s.steps++
	f := s.frames[min(s.steps, len(s.frames))-1]
	if s.steps > len(s.frames) {
		f.Events = nil
	}
	f.Clock = float64(s.steps) / TPS
	return f
}

// voyDrawer ist ein Zeichner der Fahrt, der nur zählt.
type voyDrawer struct {
	ready bool
	asked int
	last  view.Voyage
}

func (d *voyDrawer) VoyageReady(_ *view.Frame, v *view.Voyage) bool {
	d.asked, d.last = d.asked+1, *v
	return d.ready
}

func (d *voyDrawer) DrawVoyage(*ebiten.Image, *view.Frame, *view.Voyage) bool { return true }

// fixedSource liefert in jedem Update dieselbe Eingabe.
type fixedSource struct{ in view.Input }

func (s *fixedSource) Poll() view.Input { return s.in }

// harbor ist der Frame am Hafen: Insel a abgeschlossen und aktuell, Ziele b und c, Wahl choice.
func harbor(choice string) view.Frame {
	sw := &sim.SwitchState{Open: true, Choice: choice, Completed: []string{"z", "a"},
		Targets: []sim.SwitchTarget{{ID: "b"}, {ID: "c"}}}
	return view.Frame{Seats: []view.Seat{{Player: 0, Device: keys1}},
		Stages: []view.Stage{{Level: "wald", World: &sim.World{IslandSwitch: sw}}}}
}

// switched ist der Frame des Inselwechsels: Ereignis islandSwitch mit Ziel target, Stufen der neuen Insel.
func switched(target string) view.Frame {
	ev := view.Event{Data: sim.Event{"type": "islandSwitch", "island": 1, "target": target}}
	return view.Frame{Seats: []view.Seat{{Player: 0, Device: keys1}}, Stages: stages("mine"), Events: []view.Event{ev}}
}

// stuck ist der Frame eines gemeldeten, aber nicht vollzogenen Wechsels: Ereignis, die Insel a steht weiter am Hafen.
func stuck(target string) view.Frame {
	f := harbor(target)
	f.Events = switched(target).Events
	return f
}

// voyGame ist eine Schleife im Fenster mit Zeichner der Fahrt; die Simulation liefert frames.
func voyGame(d *voyDrawer, frames ...view.Frame) (*Game, *voySim, *fixedSource) {
	s, src := &voySim{fakeSim: newFakeSim(), frames: frames}, &fixedSource{}
	g := &Game{sim: s, src: src, mode: ModePlay, parts: Parts{Menus: pauseMenus{}}}
	if d != nil {
		g.parts.Voyage = d
	}
	return g, s, src
}

// B-977/AC-01: Das Ereignis islandSwitch beginnt die Fahrt von der aktuellen Insel (letzte der Abgeschlossenen) zum
// Ziel des Wechsels; nennt das Ereignis keins, gilt die Wahl, ohne Wahl das erste Ziel der Liste.
func TestVoyageStartsOnIslandSwitch(t *testing.T) {
	cases := []struct{ name, choice, target, want string }{
		{"Ziel des Ereignisses", "c", "c", "c"},
		{"Wahl ohne Ziel im Ereignis", "c", "", "c"},
		{"ohne Wahl das erste Ziel", "", "", "b"},
		{"ohne Wahl, Ereignis nennt das erste", "", "b", "b"},
	}
	for _, c := range cases {
		d := &voyDrawer{ready: true}
		g, _, _ := voyGame(d, harbor(c.choice), switched(c.target))
		g.advance()
		if g.voy.on || d.asked != 0 {
			t.Fatalf("%s: Fahrt am Hafen ohne Wechsel", c.name)
		}
		g.advance()
		v := d.last
		if !g.voy.on || v.From != "a" || v.To != c.want || !slices.Equal(v.Done, []string{"z", "a"}) ||
			!slices.Equal(v.Open, []string{"b", "c"}) || v.Duration != voyageSeconds {
			t.Errorf("%s: Fahrt an %v, %+v", c.name, g.voy.on, v)
		}
	}
}

// B-977/AC-02, Laden schneller: Die Fahrt endet genau nach der Fahrzeit; solange taktet die Simulation nicht, und die
// Uhr des Frames steht (Banner laufen nicht hinter der Karte ab); danach läuft sie ohne Sprung zurück weiter.
func TestVoyageEndsAfterTravelTime(t *testing.T) {
	g, s, _ := voyGame(&voyDrawer{ready: true}, harbor("b"), switched("b"))
	g.advance()
	g.advance()
	steps, clock := s.steps, g.frame.Clock
	n := int(voyageSeconds * TPS)
	for i := 1; i < n; i++ {
		g.advance()
		if !g.voy.on || s.steps != steps || g.frame.Clock != clock || len(g.frame.Events) != 0 {
			t.Fatalf("Takt %d: Fahrt %v, Takte der Simulation %d, Uhr %v, Ereignisse %d", i, g.voy.on, s.steps-steps, g.frame.Clock, len(g.frame.Events))
		}
	}
	if g.advance(); g.voy.on || s.steps != steps {
		t.Fatalf("nach der Fahrzeit: Fahrt %v, Takte der Simulation %d", g.voy.on, s.steps-steps)
	}
	if g.advance(); s.steps != steps+1 || g.frame.Clock <= clock {
		t.Errorf("nach der Fahrt: Takte %d, Uhr %v nach %v", s.steps-steps, g.frame.Clock, clock)
	}
}

// B-977/AC-02, Laden langsamer: Nach der Fahrzeit wartet die Fahrt, bis die Grafik der Zielinsel geladen ist.
func TestVoyageWaitsForLoading(t *testing.T) {
	g, s, _ := voyGame(&voyDrawer{ready: true}, harbor("b"), harbor("b"), switched("b"))
	g.parts.Art, g.parts.Assets = &loader{}, []view.AssetSource{assetsOf{"wald": {"a/b"}, "mine": {"c/d"}}}
	g.advance() // Hafen: lädt „wald“ nach dem ersten Bild
	g.shown = true
	g.advance()
	g.advance() // Inselwechsel: Fahrt beginnt, „mine“ wartet auf ein Bild
	if !g.voy.on || !g.Loading() {
		t.Fatalf("Wechsel: Fahrt %v, lädt %v", g.voy.on, g.Loading())
	}
	steps := s.steps
	for range int(voyageSeconds*TPS) + 30 { // kein Bild gezeichnet: es lädt noch
		g.advance()
	}
	if !g.voy.on || !g.Loading() || s.steps != steps {
		t.Fatalf("lädt noch: Fahrt %v, lädt %v, Takte der Simulation %d", g.voy.on, g.Loading(), s.steps-steps)
	}
	g.drawVoyage(nil) // das Bild der Fahrt gibt das Laden frei (wie die Ladeanzeige)
	if g.advance(); g.voy.on || g.Loading() || !slices.Equal(g.levels, []string{"mine"}) || s.steps != steps {
		t.Errorf("geladen: Fahrt %v, lädt %v, Level %v, Takte %d", g.voy.on, g.Loading(), g.levels, s.steps-steps)
	}
}

// B-977/AC-02 (R4): Während der Fahrt löst keine Spiel-Eingabe etwas aus: kein Befehl, keine Pause, kein Beitritt. Nur
// der Vollbild-Wechsel (kein Spielzug) wirkt weiter.
func TestVoyageIgnoresInput(t *testing.T) {
	g, s, src := voyGame(&voyDrawer{ready: true}, harbor("b"), switched("b"))
	g.advance()
	g.advance()
	g.frame.Settings.Language = "de" // toggleWindow wertet nur mit Einstellungen aus
	clear(s.cmds)
	src.in = view.Input{Devices: []view.DeviceInput{
		press(keys1, view.Command{Move: 1}, view.ActionMenu, view.ActionHome, view.ActionConfirm, view.ActionBack),
		press(pad3, view.Command{}, view.ActionConfirm)}}
	for range 10 {
		g.advance()
	}
	if !g.voy.on || g.mode != ModePlay || g.stack.Len() != 0 || s.paused || len(s.cmds) != 0 || len(s.joins) != 0 || len(s.set) != 0 {
		t.Errorf("Eingabe wirkt: Fahrt %v, Modus %v, Screens %d, Pause %v, Befehle %v, Beitritte %v, Einstellungen %d",
			g.voy.on, g.mode, g.stack.Len(), s.paused, s.cmds, s.joins, len(s.set))
	}
	src.in = view.Input{Devices: []view.DeviceInput{press(keys1, view.Command{}, view.ActionFullscreen)}}
	if g.advance(); len(s.set) != 1 || !g.voy.on {
		t.Errorf("Vollbild-Wechsel in der Fahrt: Einstellungen %d, Fahrt %v", len(s.set), g.voy.on)
	}
}

// B-977/AC-03: Ohne Zeichner, ohne Assets (Zeichner nicht bereit), im Aufnahme- und im Messmodus bleibt beim
// Inselwechsel der Ladebildschirm; neues Spiel und Laden eines Spielstands (Stufen ohne islandSwitch) zeigen ihn ohnehin.
func TestVoyageFallsBackToLoadScreen(t *testing.T) {
	withArt := func(g *Game) {
		g.parts.Art, g.parts.Assets = &loader{}, []view.AssetSource{assetsOf{"wald": {"a/b"}, "mine": {"c/d"}}}
	}
	cases := []struct {
		name   string
		d      *voyDrawer
		frames []view.Frame
	}{
		{"ohne Zeichner", nil, []view.Frame{harbor("b"), harbor("b"), switched("b")}},
		{"ohne Assets", &voyDrawer{}, []view.Frame{harbor("b"), harbor("b"), switched("b")}},
		{"neues Spiel oder Spielstand", &voyDrawer{ready: true}, []view.Frame{{}, {}, {Stages: stages("mine")}}},
		{"Insel nicht getauscht (Fehler im Raum)", &voyDrawer{ready: true}, []view.Frame{harbor("b"), harbor("b"), stuck("b")}},
		{"ohne Wechselpunkt davor", &voyDrawer{ready: true}, []view.Frame{{Stages: stages("wald")}, {Stages: stages("wald")}, switched("b")}},
	}
	for _, c := range cases {
		g, _, _ := voyGame(c.d, c.frames...)
		withArt(g)
		g.advance()
		g.shown = true
		g.advance()
		g.advance()
		if stay := c.frames[2].Stages[0].Level == "wald"; g.voy.on || g.Loading() == stay {
			t.Errorf("%s: Fahrt %v, Ladeanzeige %v", c.name, g.voy.on, g.Loading())
		}
	}
	for _, mode := range []string{"Aufnahme", "Messmodus"} {
		g, _, _ := voyGame(&voyDrawer{ready: true}, harbor("b"), switched("b"))
		if mode == "Aufnahme" {
			g.args.Shots = "x"
		} else {
			g.args.Measure.Seconds = 5
		}
		g.advance()
		if g.advance(); g.voy.on {
			t.Errorf("%s: Fahrt beginnt", mode)
		}
	}
}

// B-977/AC-04: Ein Takt der Fahrt legt nichts an.
func TestVoyageTickAllocatesNothing(t *testing.T) {
	g, _, _ := voyGame(&voyDrawer{ready: true}, harbor("b"), switched("b"))
	g.advance()
	g.advance()
	g.voy.v.Duration = 1e9 // die Messung bleibt in der Fahrt
	for range 5 {
		g.advance()
	}
	if n := testing.AllocsPerRun(50, func() { g.sail(view.Input{}); g.arrive() }); n != 0 || !g.voy.on {
		t.Errorf("Takt der Fahrt: %v Allokationen, Fahrt %v", n, g.voy.on)
	}
}
