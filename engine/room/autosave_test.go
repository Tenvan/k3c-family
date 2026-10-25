package room

import (
	"encoding/json"
	"errors"
	"maps"
	"os"
	"slices"
	"testing"
)

// Speichern im Takt und bei Tagesanbruch (B-186, ST1.1): Der Raum speichert alle SaveEvery und beim Tageswechsel; nach
// einem gelungenen Schreiben trägt der nächste Zustand jeder Stufe das Ereignis saved, danach ist es aus der Welt.

// savesOf ist die Zahl der Speicherungen im memStore der Fixture, nachdem der Raum alles geschrieben hat.
func savesOf(f *fixture, r *Room) int {
	waitSaved(r)
	f.store.mu.Lock()
	defer f.store.mu.Unlock()
	return f.store.saves
}

// B-186/AC-01: Nach SaveEvery Ticks liegt ein neuer Stand vor, vorher keiner; der folgende Zustand meldet saved.
func TestSpeichertImTakt(t *testing.T) {
	f := newFixture()
	x := &eventPeer{}
	r := need(f.m.Create("xbox", x, "takt", true, 0, []int{0}, Options{}))(t)
	start := savesOf(f, r)
	ticks(r, saveTicks-1)
	if n := savesOf(f, r); n != start {
		t.Fatalf("vor dem Takt %d Speicherungen", n-start)
	}
	if n, _ := x.count("saved"); n != 0 {
		t.Fatal("saved ohne Speicherung")
	}
	ticks(r, 1)
	if n := savesOf(f, r); n != start+1 {
		t.Fatalf("nach %d Ticks %d Speicherungen, erwartet 1", saveTicks, n-start)
	}
	ticks(r, 1)
	n, ev := x.count("saved")
	if n != 1 || ev["stage"] != 0 {
		t.Fatalf("saved im Zustand nach dem Speichern: %d, %v", n, ev)
	}
	if want := savedExample(t); !slices.Equal(slices.Sorted(maps.Keys(want)), slices.Sorted(maps.Keys(ev))) {
		t.Fatalf("saved %v hat andere Schlüssel als das Beispiel %v", ev, want)
	}
	for _, ev := range r.isl.Stages[0].Events {
		if ev["type"] == "saved" {
			t.Fatal("saved bleibt in der Welt")
		}
	}
	ticks(r, 1)
	if n, _ := x.count("saved"); n != 0 {
		t.Fatal("saved zweimal gemeldet")
	}
}

// savedExample ist das Ereignis aus testdata/protocol/s2c-snapshot-gesichert.json (docs/protocol.md › Ereignisse).
func savedExample(t *testing.T) map[string]any {
	t.Helper()
	var msg struct {
		S struct {
			Events []map[string]any `json:"events"`
		} `json:"s"`
	}
	raw := need(os.ReadFile("../../testdata/protocol/s2c-snapshot-gesichert.json"))(t)
	if err := json.Unmarshal(raw, &msg); err != nil || len(msg.S.Events) != 1 || msg.S.Events[0]["type"] != "saved" {
		t.Fatalf("Beispiel: %v, %v", err, msg.S.Events)
	}
	return msg.S.Events[0]
}

// B-186/AC-01: Beim Tagesanbruch speichert der Raum im selben Tick.
func TestSpeichertBeiTagesanbruch(t *testing.T) {
	f := newFixture()
	r := need(f.m.Create("xbox", &peer{}, "morgen", true, 0, []int{0}, Options{}))(t)
	for _, w := range r.isl.Stages {
		w.CycleSpeed = 1000
	}
	for i := 0; i < saveTicks-1; i++ {
		day, before := r.isl.Stages[0].Cycle.Day, savesOf(f, r)
		ticks(r, 1)
		if r.isl.Stages[0].Cycle.Day > day {
			if n := savesOf(f, r); n != before+1 {
				t.Fatalf("Tagesanbruch Tag %d: %d Speicherungen", day+1, n-before)
			}
			return
		}
	}
	t.Fatal("kein Tagesanbruch vor dem Takt")
}

// B-186: Scheitert das Schreiben, meldet der Raum kein saved.
func TestKeinSavedBeiSchreibfehler(t *testing.T) {
	f, s, _ := logged(t)
	x := &eventPeer{}
	r := need(f.m.Create("xbox", x, "fehler", true, 0, []int{0}, Options{}))(t)
	waitSaved(r)
	s.mu.Lock()
	s.fail = errors.New("gesperrt")
	s.mu.Unlock()
	ticks(r, saveTicks)
	waitSaved(r)
	ticks(r, 1)
	if n, _ := x.count("saved"); n != 0 {
		t.Fatal("saved trotz Schreibfehler")
	}
}

// B-186/AC-02: Der Takt ändert den Spielverlauf nicht. Dieselben Räume mit und ohne Takt (due vor jedem Tick auf 0)
// rechnen nach mehr als zwei Takten dieselbe Welt.
func TestTaktAendertSpielverlaufNicht(t *testing.T) {
	with := setup(t, newFixture())
	without := setup(t, newFixture())
	n := 2*saveTicks + 30
	for i := range with {
		ticks(with[i], n)
		for range n {
			without[i].mu.Lock()
			without[i].saves.due = 0
			without[i].mu.Unlock()
			_ = without[i].Tick()
		}
		waitSaved(with[i])
		if world(t, with[i]) != world(t, without[i]) {
			t.Errorf("Raum %s: Welt mit Takt weicht ab", with[i].Code)
		}
	}
}
