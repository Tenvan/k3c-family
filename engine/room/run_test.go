package room

import (
	"context"
	"encoding/json"
	"reflect"
	"testing"
	"time"

	"k3c/engine/sim"
)

// setup baut drei Räume mit 2, 3 und 4 Spielern und festen Eingaben je Slot.
func setup(t *testing.T, f *fixture) []*Room {
	t.Helper()
	r1 := need(f.m.Create("a", &peer{}, "eins", true, 0, []int{0, 1}))(t)
	r2 := need(f.m.Create("b", &peer{}, "zwei", true, 0, []int{0, 1, 2}))(t)
	r3 := need(f.m.Create("c", &peer{}, "drei", true, 0, []int{0, 1}))(t)
	need(f.m.Join("d", &peer{}, r3.Code, []int{0, 1}))(t)
	ok(t, r1.Input("a", r1.peerOf("a"), map[int]sim.PlayerCommand{0: sim.PlayerCommand{MoveX: 1}}))
	ok(t, r1.Input("a", r1.peerOf("a"), map[int]sim.PlayerCommand{1: sim.PlayerCommand{MoveX: -1, Pay: true}}))
	ok(t, r2.Input("b", r2.peerOf("b"), map[int]sim.PlayerCommand{2: sim.PlayerCommand{MoveX: 1, Sprint: true}}))
	ok(t, r3.Input("c", r3.peerOf("c"), map[int]sim.PlayerCommand{0: sim.PlayerCommand{MoveX: -1}}))
	ok(t, r3.Input("d", r3.peerOf("d"), map[int]sim.PlayerCommand{1: sim.PlayerCommand{MoveX: 1, Pay: true}}))
	return []*Room{r1, r2, r3}
}

func world(t *testing.T, r *Room) string {
	t.Helper()
	r.mu.Lock()
	defer r.mu.Unlock()
	return string(need(json.Marshal(r.isl.Stages))(t))
}

// 3 Räume ticken parallel in eigenen Goroutinen. Jeder Raum hängt nur von seinen Eingaben ab: Ein Referenzraum mit
// denselben Eingaben, von Hand gleich oft getaktet, hat denselben Zustand (AC-01, B-036).
func TestDreiRaeumeParallel(t *testing.T) {
	f := newFixture()
	f.m.Now = time.Now
	rooms := setup(t, f)
	ctx, cancel := context.WithCancel(context.Background())
	go f.m.Run(ctx)
	time.Sleep(400 * time.Millisecond)
	cancel()
	time.Sleep(100 * time.Millisecond) // laufende Ticks enden, danach tickt niemand mehr
	ref := setup(t, newFixture())
	for i, r := range rooms {
		r.mu.Lock()
		n := r.tick
		r.mu.Unlock()
		if n < 5 {
			t.Fatalf("Raum %s hat nur %d Ticks", r.Code, n)
		}
		ticks(ref[i], n)
		if world(t, r) != world(t, ref[i]) {
			t.Errorf("Raum %s weicht nach %d Ticks vom Referenzraum ab", r.Code, n)
		}
	}
	status, _ := f.m.Status()
	if len(status) != 3 || status[2].Devices != 2 || len(status[2].Monarchs) != 4 {
		t.Fatalf("Status: %+v", status)
	}
	// Jeder Tick ist gemessen (der Wert selbst kann unter Windows unter der Auflösung der Uhr liegen).
	for _, r := range rooms {
		r.mu.Lock()
		if len(r.durations) != min(r.tick, durations) {
			t.Errorf("Raum %s: %d Messungen bei %d Ticks", r.Code, len(r.durations), r.tick)
		}
		r.mu.Unlock()
	}
}

// Ein Panic im Tick schließt nur diesen Raum; seine Geräte bekommen room_closed, die anderen Räume ticken weiter.
func TestAbsturzTrifftNurDenRaum(t *testing.T) {
	f := newFixture()
	f.m.Now = time.Now
	ok1, broken := &peer{}, &peer{}
	r1 := need(f.m.Create("a", ok1, "heil", true, 0, []int{0}))(t)
	r2 := need(f.m.Create("b", broken, "kaputt", true, 0, []int{0}))(t)
	r2.mu.Lock()
	r2.beforeStep = func() { panic("Absturz im Test") }
	r2.mu.Unlock()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	go f.m.Run(ctx)
	deadline := time.Now().Add(2 * time.Second)
	for f.m.Room(r2.Code) != nil && time.Now().Before(deadline) {
		time.Sleep(10 * time.Millisecond)
	}
	time.Sleep(100 * time.Millisecond)
	_, failures := f.m.Status()
	r2.mu.Lock()
	closed := broken.has("closed")
	r2.mu.Unlock()
	if f.m.Room(r2.Code) != nil || !closed || len(failures) != 1 || failures[0].Error != "Absturz im Test" {
		t.Fatalf("abgestürzter Raum: noch da %v, closed %v, %+v", f.m.Room(r2.Code) != nil, closed, failures)
	}
	r1.mu.Lock()
	n := r1.tick
	r1.mu.Unlock()
	if f.m.Room(r1.Code) == nil || n == 0 {
		t.Fatal("der andere Raum tickt nicht mehr")
	}
}

func TestSummary(t *testing.T) {
	f := newFixture()
	r := need(f.m.Create("a", &peer{}, "kurz", true, 0, []int{0, 1}))(t)
	ticks(r, 3)
	s := r.Summary()
	want := Summary{Code: "KRNZ", Depth: 0, Tick: 3, Phase: "day", Day: 1, Gold: []int{100, 100},
		Troops: s.Troops, Enemies: 0, Castle: 1000, Wave: 0, Devices: []DeviceInfo{{ID: "a", Connected: true, Slots: []int{0, 1}}}, Stages: s.Stages}
	if !reflect.DeepEqual(s, want) || s.Troops["archer"] != 2 {
		t.Fatalf("Summary: %+v", s)
	}
}

// B-088: Die Kennung ist der kürzeste gemeinsame Anfang (mindestens 6 Zeichen), der alle Geräte des Raums unterscheidet.
func TestGeraeteKennungen(t *testing.T) {
	f := newFixture()
	r := need(f.m.Create("b1f4c2e0-5d7a-4c1e-9a33-7e2f0d6c8a15", &peer{}, "kennung", true, 0, []int{0}))(t)
	if err := r.lockedJoin("b1f4c2ff-0000-4c1e-9a33-7e2f0d6c8a15", &peer{}, []int{1}); err != nil {
		t.Fatal(err)
	}
	r.mu.Lock()
	got := r.deviceInfos()
	r.mu.Unlock()
	if len(got) != 2 || got[0].ID != "b1f4c2e" || got[1].ID != "b1f4c2f" {
		t.Errorf("Kennungen: %+v", got)
	}
}
