package view

import (
	"slices"
	"testing"

	"k3c/engine/sim"
)

func testFrame() (*Frame, *sim.World, *sim.World) {
	top := &sim.World{Players: []*sim.Player{{Index: 0, X: 1}, {Index: 1, X: 2}}}
	deep := &sim.World{Players: []*sim.Player{{Index: 1, X: 7}}}
	f := &Frame{
		Stages: []Stage{{Index: 0, World: top}, {Index: 2, Depth: 1, World: deep}},
		Seats:  []Seat{{Player: 0, Monarch: 0, Stage: 0}, {Player: 1, Monarch: 1, Stage: 2, Device: DeviceID{DeviceGamepad, 1}}},
		Events: []Event{
			{Stage: 0, Data: sim.Event{"type": "night", "day": 1}},
			{Stage: 2, Data: sim.Event{"type": "coin"}},
			{Stage: 0, Data: sim.Event{"day": 2}},
		},
	}
	return f, top, deep
}

func TestFrameResolvesSeats(t *testing.T) {
	f, top, deep := testFrame()
	if w := f.World(Cell{Player: 0}); w != top {
		t.Error("Zelle 0 sieht nicht die Oberfläche")
	}
	if w := f.World(Cell{Player: 1}); w != deep {
		t.Error("Zelle 1 sieht nicht ihre Stufe")
	}
	if f.World(Cell{Player: -1}) != nil {
		t.Error("Fenster-Zelle hat eine Welt")
	}
	if m := f.Monarch(1); m == nil || m.X != 7 {
		t.Errorf("Monarch 1 = %+v, erwartet den aus Stufe 2", m)
	}
	if f.Monarch(5) != nil {
		t.Error("unbekannter Spieler hat einen Monarchen")
	}
	if s, ok := f.Seat(1); !ok || s.Device.String() != "pad2" {
		t.Errorf("Seat 1 = %+v", s)
	}
}

func TestFrameMissingStage(t *testing.T) {
	f := &Frame{Seats: []Seat{{Player: 0, Stage: 3}}}
	if f.World(Cell{Player: 0}) != nil || f.Monarch(0) != nil {
		t.Error("fehlende Stufe liefert Welt")
	}
	f.Stages = []Stage{{Index: 3}}
	if f.Monarch(0) != nil {
		t.Error("Stufe ohne Welt liefert Monarchen")
	}
}

func TestFrameEvents(t *testing.T) {
	f, _, _ := testFrame()
	got := slices.Collect(f.EventsOf(0))
	if len(got) != 2 || got[0].Type() != "night" || got[1].Type() != "" {
		t.Errorf("EventsOf(0) = %+v", got)
	}
	if len(slices.Collect(f.EventsOf(9))) != 0 {
		t.Error("Ereignisse einer fremden Stufe")
	}
	for range f.EventsOf(0) {
		break // Abbruch beim ersten Ereignis
	}
	n := 0
	if a := testing.AllocsPerRun(10, func() {
		for e := range f.EventsOf(2) {
			n += len(e.Type())
		}
	}); a != 0 {
		t.Errorf("EventsOf legt %v Objekte an", a)
	}
}
