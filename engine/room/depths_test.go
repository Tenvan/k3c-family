package room

import (
	"encoding/json"
	"errors"
	"testing"
	"time"

	"k3c/engine/sim"
)

// B-199, B-290: Ein neuer Raum spielt die ganze Insel, eine Stufe je Biom (heute fünf); geladene Stände behalten ihre Stufen.
func TestNeuerRaumHatAlleStufen(t *testing.T) {
	f := newFixture()
	r := need(f.m.Create("xbox", &peer{}, "insel", true, 4, []int{0}, Options{}))(t)
	if len(r.isl.Stages) != 5 {
		t.Fatalf("Stufen: %d, erwartet 5", len(r.isl.Stages))
	}
	for i, w := range r.isl.Stages {
		if w.Biome.Depth != i {
			t.Fatalf("Stufe %d hat Tiefe %d", i, w.Biome.Depth)
		}
	}
	if got := r.isl.Stages[r.start].Biome.Depth; got != 4 {
		t.Fatalf("Startstufe Tiefe %d, erwartet 4", got)
	}
	if _, err := f.m.Create("handy", &peer{}, "zu-tief", true, 5, []int{0}, Options{}); !errors.Is(err, ErrBadRequest) {
		t.Fatalf("Startstufe 5: %v, erwartet ErrBadRequest", err)
	}
}

func TestGespeicherterStandBehaeltStufen(t *testing.T) {
	f := newFixture()
	isl := need(sim.CreateIsland("drei", []int{0, 1, 2}, 1, sim.CreateOptions{}))(t)
	f.store.data["drei"] = need(json.Marshal(isl.ToSave("2026-10-01T12:00:00.000Z")))(t)
	r := need(f.m.Create("xbox", &peer{}, "drei", false, 0, []int{0}, Options{}))(t)
	if len(r.isl.Stages) != 3 {
		t.Fatalf("Stufen nach Laden: %d, erwartet 3", len(r.isl.Stages))
	}
}

// B-204: Ein leerer Test-Raum ist nach dem nächsten Sweep weg, ein wartender Monarch hält ihn bis WaitFor offen.
func TestLeererTestRaumSchliesstSofort(t *testing.T) {
	f := newFixture()
	x, y := &peer{}, &peer{}
	gone := need(f.m.Create("xbox", x, "test-leer", true, 0, []int{0}, Options{}))(t)
	held := need(f.m.Create("handy", y, "test-wartet", true, 0, []int{0}, Options{}))(t)
	gone.Leave("xbox", x)
	held.Drop("handy", y)
	f.wait(time.Second)
	if codes := roomNames(f.m); len(codes) != 1 || codes[0] != "test-wartet" {
		t.Fatalf("Räume nach erstem Sweep: %v, erwartet nur test-wartet", codes)
	}
	if len(f.store.deleted) != 1 || f.store.deleted[0] != "test-leer" {
		t.Fatalf("gelöscht: %v, erwartet test-leer", f.store.deleted)
	}
	f.wait(WaitFor)
	if len(f.m.Rooms()) != 0 || len(f.store.deleted) != 2 {
		t.Fatalf("nach WaitFor: Räume %v, gelöscht %v", roomNames(f.m), f.store.deleted)
	}
}

func TestNormalerRaumBleibtEmptyFor(t *testing.T) {
	f := newFixture()
	x := &peer{}
	r := need(f.m.Create("xbox", x, "familie", true, 0, []int{0}, Options{}))(t)
	r.Leave("xbox", x)
	f.wait(EmptyFor - time.Second)
	if len(f.m.Rooms()) != 1 {
		t.Fatal("normaler Raum vor EmptyFor aufgeräumt")
	}
	f.wait(time.Second)
	if len(f.m.Rooms()) != 0 || len(f.store.deleted) != 0 {
		t.Fatalf("nach EmptyFor: %d Räume, gelöscht %v", len(f.m.Rooms()), f.store.deleted)
	}
}

func roomNames(m *Manager) []string {
	var out []string
	for _, i := range m.Rooms() {
		out = append(out, i.Name)
	}
	return out
}
