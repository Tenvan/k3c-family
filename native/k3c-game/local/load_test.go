package local

import (
	"testing"

	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/engine/store"
	"k3c/native/k3c-game/view"
)

const loadTicks = 120

// roomLoadTicked lädt den Spielstand direkt über engine/room und taktet ihn n Mal mit commandAt.
func roomLoadTicked(t *testing.T, dir, name string, n int) string {
	t.Helper()
	m := room.NewManager(&store.Saves{Dir: dir, Now: fixedNow})
	m.Now = fixedNow
	m.NewCode = func() string { return "REFR" }
	p := &refPeer{worlds: map[int]*sim.World{}}
	r, err := m.Create("ref", p, name, false, 0, []int{0, 1}, room.Options{})
	if err != nil {
		t.Fatal(err)
	}
	for i := range n {
		in := map[int]sim.PlayerCommand{0: commandAt(0, i).Sim(), 1: commandAt(1, i).Sim()}
		if err := r.Input("ref", p, in); err != nil {
			t.Fatal(err)
		}
		r.Tick()
	}
	return hashWorlds(t, p.worlds)
}

// Ein geladener Stand läuft über local mit denselben Befehlen genauso weiter wie ein direkt über engine/room
// geladener Raum (Welt-Hash nach 120 Takten).
func TestLoadDeterminismLikeRoom(t *testing.T) {
	dir := t.TempDir()
	s := testSession(t, dir)
	savedGame(t, s)
	if err := s.Load("spiel-1", twoDevices); err != nil {
		t.Fatal(err)
	}
	var f view.Frame
	for i := range loadTicks {
		for p := range 2 {
			if err := s.Command(p, commandAt(p, i)); err != nil {
				t.Fatal(err)
			}
		}
		f = s.Step()
	}
	got := hashWorlds(t, frameWorlds(f))
	if want := roomLoadTicked(t, dir, "spiel-1", loadTicks); got != want {
		t.Errorf("Welt-Hash nach Laden und %d Takten %s, Raum direkt %s", loadTicks, got, want)
	}
}

// Command, Plätze und Stufen legen je Takt nichts an (Puffer werden wiederverwendet).
func TestStepBuffersDoNotAllocate(t *testing.T) {
	s := testSession(t, t.TempDir())
	if err := s.New(view.NewGame{Name: "puffer", Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	s.Step()
	s.Step()
	cases := map[string]func(){
		"Command": func() { _ = s.Command(0, view.Command{Move: 1}) },
		"seats":   func() { s.seats() },
		"stages":  func() { s.p.stages() },
	}
	for name, f := range cases {
		if n := testing.AllocsPerRun(20, f); n != 0 {
			t.Errorf("%s legt %v Objekte an", name, n)
		}
	}
}
