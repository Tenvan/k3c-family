package local

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"slices"
	"testing"
	"time"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/engine/store"
	"k3c/native/k3c-game/view"
)

// fixedNow ist die Uhr der Tests (Insel-ID und Zeitstempel der Spielstände).
func fixedNow() time.Time { return time.Date(2026, 11, 11, 12, 0, 0, 0, time.UTC) }

func testSession(t *testing.T, dir string) *Session {
	t.Helper()
	return Open(Config{Dir: dir, Now: fixedNow})
}

var twoDevices = []view.DeviceID{{Kind: view.DeviceKeyboard, Index: 0}, {Kind: view.DeviceGamepad, Index: 0}}

// commandAt sind die festen Befehle des Determinismus-Tests: beide halten rechts, mit Sprint, Bezahlen und Angriff im
// Wechsel, Spieler 2 dreht alle 100 Takte um.
func commandAt(player, i int) view.Command {
	if player == 0 {
		return view.Command{Move: 1, Sprint: i%90 < 30, Pay: i%50 == 0}
	}
	move := 1.0
	if (i/100)%2 == 1 {
		move = -1
	}
	return view.Command{Move: move, Attack: i%70 == 0}
}

// hashWorlds ist der Welt-Hash: SHA-256 über das JSON der Welten nach Stufen-Index.
func hashWorlds(t *testing.T, worlds map[int]*sim.World) string {
	t.Helper()
	keys := make([]int, 0, len(worlds))
	for k := range worlds {
		keys = append(keys, k)
	}
	slices.Sort(keys)
	h := sha256.New()
	for _, k := range keys {
		raw, err := json.Marshal(worlds[k])
		if err != nil {
			t.Fatal(err)
		}
		h.Write(raw)
	}
	return hex.EncodeToString(h.Sum(nil))
}

func frameWorlds(f view.Frame) map[int]*sim.World {
	out := map[int]*sim.World{}
	for _, st := range f.Stages {
		out[st.Index] = st.World
	}
	return out
}

// refPeer ist ein Gerät direkt am Raum, ohne local.
type refPeer struct{ worlds map[int]*sim.World }

func (p *refPeer) Joined(string, string, string, []room.Seat) {}
func (p *refPeer) Level(int, int, level.Layout)               {}
func (p *refPeer) State(_, stage int, state any)              { p.worlds[stage] = state.(*sim.World) }
func (p *refPeer) Seats([]room.Seat, []string)                {}
func (p *refPeer) Replaced()                                  {}
func (p *refPeer) Closed(bool)                                {}

const determinismTicks = 300

// referenceHash taktet einen Raum direkt über engine/room mit denselben Befehlen.
func referenceHash(t *testing.T) (string, []float64) {
	t.Helper()
	m := room.NewManager(&store.Saves{Dir: t.TempDir(), Now: fixedNow})
	m.Now = fixedNow
	m.NewCode = func() string { return "REFR" }
	p := &refPeer{worlds: map[int]*sim.World{}}
	r, err := m.Create("ref", p, "wald-1", true, 0, []int{0, 1}, room.Options{})
	if err != nil {
		t.Fatal(err)
	}
	for i := range determinismTicks {
		in := map[int]sim.PlayerCommand{0: commandAt(0, i).Sim(), 1: commandAt(1, i).Sim()}
		if err := r.Input("ref", p, in); err != nil {
			t.Fatal(err)
		}
		r.Tick()
	}
	return hashWorlds(t, p.worlds), monarchXs(p.worlds)
}

func monarchXs(worlds map[int]*sim.World) []float64 {
	xs := []float64{}
	for _, w := range worlds {
		for _, pl := range w.Players {
			xs = append(xs, pl.X)
		}
	}
	slices.Sort(xs)
	return xs
}

// AC-03 (NG1.2), B-793/AC-01: 2 Spieler, 300 Takte über local ergeben denselben Welt-Hash wie ein direkt über
// engine/room getakteter Raum mit denselben Befehlen.
func TestDeterminismLikeRoom(t *testing.T) {
	s := testSession(t, t.TempDir())
	if err := s.New(view.NewGame{Name: "wald-1", Devices: twoDevices}); err != nil {
		t.Fatal(err)
	}
	var f view.Frame
	for i := range determinismTicks {
		for p := range 2 {
			if err := s.Command(p, commandAt(p, i)); err != nil {
				t.Fatal(err)
			}
		}
		f = s.Step()
	}
	want, wantXs := referenceHash(t)
	if got := hashWorlds(t, frameWorlds(f)); got != want {
		t.Errorf("Welt-Hash %s, Raum direkt %s", got, want)
	}
	if got := monarchXs(frameWorlds(f)); !slices.Equal(got, wantXs) {
		t.Errorf("Monarchen bei %v, Raum direkt %v", got, wantXs)
	}
	if f.Tick != determinismTicks || len(f.Seats) != 2 || f.Monarch(0) == nil || f.Monarch(1) == nil {
		t.Errorf("Frame: Tick %d, Plätze %+v", f.Tick, f.Seats)
	}
	if f.Seats[1].Device != twoDevices[1] {
		t.Errorf("Gerät von Spieler 2 = %v", f.Seats[1].Device)
	}
}

// Ohne Spiel liefert Step einen leeren Frame mit Uhr; in der Pause steht der Takt.
func TestStepWithoutGameAndPaused(t *testing.T) {
	s := testSession(t, t.TempDir())
	f := s.Step()
	if s.Active() || len(f.Stages) != 0 || f.Clock <= 0 {
		t.Fatalf("Frame ohne Spiel: %+v", f)
	}
	if err := s.New(view.NewGame{Name: "pause", Devices: twoDevices[:1]}); err != nil {
		t.Fatal(err)
	}
	s.Step()
	s.Pause(true)
	f = s.Step()
	tick := f.Tick
	if !f.Paused || len(f.Cues) != 1 || f.Cues[0].Kind != view.CuePause {
		t.Fatalf("Pause-Frame: %+v", f)
	}
	for range 10 {
		f = s.Step()
	}
	if f.Tick != tick || len(f.Stages) == 0 {
		t.Errorf("Takt %d in der Pause, vorher %d; Stufen %d", f.Tick, tick, len(f.Stages))
	}
	s.Pause(false)
	if f = s.Step(); f.Tick != tick+1 || f.Paused {
		t.Errorf("nach der Pause Takt %d, erwartet %d", f.Tick, tick+1)
	}
}

// Spieler kommen und gehen; der letzte bleibt, höchstens zwei.
func TestJoinLeave(t *testing.T) {
	s := testSession(t, t.TempDir())
	if _, err := s.Join(twoDevices[0]); err == nil {
		t.Fatal("Join ohne Spiel")
	}
	if err := s.New(view.NewGame{Name: "couch", Devices: twoDevices[:1]}); err != nil {
		t.Fatal(err)
	}
	if err := s.Leave(0); err == nil {
		t.Error("letzter Spieler ging")
	}
	if p, err := s.Join(twoDevices[1]); err != nil || p != 1 {
		t.Fatalf("Join = %d, %v", p, err)
	}
	if p, err := s.Join(twoDevices[1]); err != nil || p != 1 {
		t.Errorf("zweites Join desselben Geräts = %d, %v", p, err)
	}
	if _, err := s.Join(view.DeviceID{Kind: view.DeviceGamepad, Index: 3}); err == nil {
		t.Error("dritter Spieler aufgenommen")
	}
	if f := s.Step(); len(f.Seats) != 2 || f.Cues[0].Kind != view.CueJoined || f.Monarch(1) == nil {
		t.Fatalf("Frame nach Join: Plätze %+v, Cues %+v", f.Seats, f.Cues)
	}
	if err := s.Leave(1); err != nil {
		t.Fatal(err)
	}
	if f := s.Step(); len(f.Seats) != 1 || f.Cues[0].Kind != view.CueLeft {
		t.Errorf("Frame nach Leave: Plätze %+v", f.Seats)
	}
	if err := s.Command(1, view.Command{Move: 1}); err == nil {
		t.Error("Befehl für gegangenen Spieler")
	}
}

// Ein Panic im Takt schließt das Spiel ohne Speichern, meldet CueError, der letzte Spielstand bleibt.
func TestPanicInTick(t *testing.T) {
	dir := t.TempDir()
	s := testSession(t, dir)
	if err := s.New(view.NewGame{Name: "absturz", Devices: twoDevices[:1]}); err != nil {
		t.Fatal(err)
	}
	s.Step()
	if err := s.Save(); err != nil {
		t.Fatal(err)
	}
	before, err := s.saves.Load("absturz")
	if err != nil {
		t.Fatal(err)
	}
	s.p.onState = func() { panic("kaputt") }
	f := s.Step()
	if s.Active() || !hasCue(f, view.CueError) {
		t.Fatalf("nach Panic aktiv %v, Cues %+v", s.Active(), f.Cues)
	}
	after, _ := s.saves.Load("absturz")
	if string(after) != string(before) {
		t.Error("Spielstand nach Panic verändert")
	}
	if err := s.Load("absturz"); err != nil {
		t.Errorf("Laden nach Panic: %v", err)
	}
}

// Autosave greift ohne Manager.Run: nach SaveEvery (1800 Takte) liegt der Spielstand im Ordner und ein Frame meldet
// CueSaved.
func TestAutosaveWithoutRun(t *testing.T) {
	dir := t.TempDir()
	s := testSession(t, dir)
	if err := s.New(view.NewGame{Name: "auto", Devices: twoDevices[:1]}); err != nil {
		t.Fatal(err)
	}
	for range int(room.SaveEvery/time.Second) * room.TickHz {
		s.Step()
	}
	// Der Raum schreibt in einer eigenen Goroutine; auf ihr Ergebnis warten, ohne weiter zu takten.
	for deadline := time.Now().Add(5 * time.Second); !s.store.pending(); time.Sleep(10 * time.Millisecond) {
		if time.Now().After(deadline) {
			t.Fatal("kein Autosave ohne Manager.Run")
		}
	}
	if !hasCue(s.Step(), view.CueSaved) {
		t.Error("Autosave ohne CueSaved")
	}
	if list, _ := s.List(); len(list) != 1 || !list[0].Loadable {
		t.Errorf("Liste nach Autosave = %+v", list)
	}
}

// pending meldet, ob ein Ergebnis einer Speicherung auf den nächsten Frame wartet (nur Tests).
func (w *watchedStore) pending() bool {
	w.mu.Lock()
	defer w.mu.Unlock()
	return len(w.results) > 0
}

func hasCue(f view.Frame, k view.CueKind) bool {
	return slices.ContainsFunc(f.Cues, func(c view.Cue) bool { return c.Kind == k })
}
