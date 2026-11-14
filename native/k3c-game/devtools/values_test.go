package devtools

import (
	"strings"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// valueFrame ist ein Frame mit zwei Spielern in derselben Stufe der Welt geoWorld (Monarch 1 bei 50, Monarch 2 bei 20).
func valueFrame() *view.Frame {
	w := geoWorld()
	w.Seed, w.Wave, w.Stock = "wald", 3, &sim.Stock{Wood: 7, Iron: 2}
	w.Cycle = sim.CycleInfo{Phase: "day", Day: 4}
	w.Players[0].Gold = 12
	w.Players = append(w.Players, &sim.Player{Index: 1, X: 20, Gold: 5})
	w.Sites[0].State = "built"
	w.Coins = []*sim.Coin{{ID: 9, X: 80}}
	return &view.Frame{Tick: 120, Timescale: 2, StepMs: 1.25, Dev: view.DevView{Debug: view.DebugValues, TPS: 30},
		Seats:  []view.Seat{{Player: 0, Monarch: 0, Stage: 0}, {Player: 1, Monarch: 1, Stage: 0}},
		Stages: []view.Stage{{Index: 0, Depth: 0, World: w}}}
}

// AC-05 (B-889): Die Seite zeigt Tageszeit mit Tag und Welle, je Spieler Gold, Vorrat, Position und Chunk, die
// Objektanzahl, den Takt und das Objekt am Monarchen (Gegner bei 52, näher als der Bauplatz bei 46).
func TestValueLines(t *testing.T) {
	lines := valueLines(nil, valueFrame(), geoCell())
	text := strings.Join(lines, "\n")
	for _, want := range []string{"Tag 4 · Tag · Welle 3", "Spieler 1: Gold 12 · Holz 7", "Eisen 2", "x 50.0 · Chunk 2",
		"Spieler 2: Gold 5", "Chunk 1", "Truppen 1 · Gegner 1 · Bauten 1 · Münzen am Boden 1",
		"Takt 120 · 30/s · 1.25 ms · Zeitraffer 2× · Seed wald", "Auswahl (am Monarchen): Monarch 1"} {
		if !strings.Contains(text, want) {
			t.Errorf("fehlt %q in\n%s", want, text)
		}
	}
}

// AC-05: Ein Klick wählt das nächste Objekt an der Stelle (Gegner mit Zustand, Ziel, LP und Entscheidung, Truppe mit
// Auftrag); weit weg von allem bleibt die Auswahl leer.
func TestSelectAt(t *testing.T) {
	w := geoWorld()
	w.Enemies[0].HP, w.Enemies[0].MaxHP, w.Enemies[0].Fleeing, w.Enemies[0].HomeX = 3, 10, true, 99
	got := strings.Join(selectAt(w, 52.5, "Klick"), " | ")
	for _, want := range []string{"Auswahl (Klick): Gegner #2", "LP 3/10", "Zustand flieht", "Ziel Portal x 99.0", "KI: zurück zum Portal"} {
		if !strings.Contains(got, want) {
			t.Errorf("fehlt %q in %q", want, got)
		}
	}
	w.Troops[0].Job = &sim.Job{Type: "gather", Resource: "wood"}
	if got = strings.Join(selectAt(w, 95, ""), " "); !strings.Contains(got, "KI: gather wood") {
		t.Errorf("Truppe: %q", got)
	}
	if got := selectAt(w, 30, "Klick"); len(got) != 1 || got[0] != "Auswahl: –" {
		t.Errorf("leer: %v", got)
	}
	f := valueFrame()
	f.Dev.Pick = view.Pick{Picked: true, Player: 0, X: 46}
	if x, how := pickX(f, geoCell()); x != 46 || how != "Klick" {
		t.Errorf("Maus-Wahl: %v %s", x, how)
	}
}

// Die Seite Werte zeichnet ohne Schrift ohne Absturz.
func TestValuesDraw(t *testing.T) {
	o := New(nil)[0].(*overlay)
	dst := ebiten.NewImage(640, 360)
	f := valueFrame()
	f.Cells = []view.Cell{geoCell()}
	o.Draw(dst, f, view.WindowCell(dst.Bounds()))
}
