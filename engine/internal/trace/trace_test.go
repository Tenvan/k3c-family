package trace

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/engine/store"
)

var write = flag.Bool("write", false, "Aufzeichnungen schreiben: lange Läufe nach .work/trace/, die kurze nach testdata/rust/trace/")

const (
	longTicks  = 3600 // Takte je Szene in .work/trace/ (2 Minuten bei 30 Hz)
	shortTicks = 300  // Takte der eingecheckten Aufzeichnung
	shortScene = "nacht"
	longDir    = "../../../.work/trace"
	shortPath  = "../../../testdata/rust/trace/nacht-300.jsonl.gz"
)

// scene beschreibt eine Szene: Spielerzahl, Aufbau mit Dev-Aktionen vor dem ersten Takt und feste Eingaben je Takt.
type scene struct {
	name    string
	players int
	setup   func(do func(room.DevAction))
}

func slot(i int) *int { return &i }

var scenes = []scene{
	{name: "tag", players: 1}, // ruhiger Tag, nur Bewegung und Münzen
	{name: "nacht", players: 2, setup: func(do func(room.DevAction)) {
		do(room.DevAction{Action: "night"})
		do(room.DevAction{Action: "spawnCastle", Slot: slot(0), Kind: "enemy", Unit: "goblin", Amount: 6})
		do(room.DevAction{Action: "spawnCastle", Slot: slot(0), Kind: "enemy", Unit: "skeleton", Amount: 4})
		do(room.DevAction{Action: "spawn", Slot: slot(0), Kind: "troop", Unit: "warrior", Amount: 3})
		do(room.DevAction{Action: "spawn", Slot: slot(0), Kind: "troop", Unit: "archer", Amount: 3})
	}},
	{name: "basis", players: 2, setup: func(do func(room.DevAction)) { // wie die Last-Szene des Spiels (measure_load.go)
		do(room.DevAction{Action: "buildAll", Level: 3})
		for _, s := range []struct {
			unit string
			n    int
		}{{"warrior", 7}, {"archer", 5}, {"peasant", 5}, {"vagrant", 3}} {
			do(room.DevAction{Action: "spawn", Slot: slot(0), Kind: "troop", Unit: s.unit, Amount: s.n})
		}
	}},
}

// input ist die feste Eingabe von Spieler p im Takt tick: abschnittsweise rechts und links, Münzen und Schläge im Wechsel.
func input(p, tick int) sim.PlayerCommand {
	dir := 1.0
	if (tick/120+p)%2 == 1 {
		dir = -1
	}
	return sim.PlayerCommand{MoveX: dir, Sprint: tick%240 < 60, Pay: tick%40 < 6, Attack: (tick+p*10)%50 < 4}
}

// memStore ist ein Spielstand-Speicher im Speicher.
type memStore map[string][]byte

func (s memStore) Load(n string) ([]byte, error) {
	if d, ok := s[n]; ok {
		return d, nil
	}
	return nil, store.ErrNotFound
}
func (s memStore) Store(n string, d []byte) (string, error) { s[n] = d; return "", nil }
func (s memStore) Delete(n string) error                    { delete(s, n); return nil }

// recorder ist der Peer des Geräts: er merkt sich Level und serialisiert jeden Zustand sofort.
type recorder struct {
	layout level.Layout
	scene  string
	w      *Schreiber
	err    error
}

func (r *recorder) Joined(string, string, string, []room.Seat) {}
func (r *recorder) Seats([]room.Seat, []string)                {}
func (r *recorder) Replaced()                                  {}
func (r *recorder) Closed(bool)                                {}
func (r *recorder) Level(_, _ int, l level.Layout)             { r.layout = l }
func (r *recorder) State(tick, _ int, state any) {
	w, ok := state.(*sim.World)
	if !ok {
		r.err = fmt.Errorf("Zustand ist %T, erwartet *sim.World", state)
		return
	}
	if r.w.Lines() == nil { // erster Zustand: davor kam das Level
		if err := r.w.Level(r.scene, w.Seed, len(w.Players), r.layout); err != nil {
			r.err = err
			return
		}
	}
	events := w.Events
	if events == nil {
		events = []sim.Event{}
	}
	if err := r.w.Takt(tick, w, events); err != nil {
		r.err = err
	}
}

// record spielt eine Szene n Takte und liefert die Aufzeichnung.
func record(t *testing.T, sc scene, n int) *Schreiber {
	t.Helper()
	m := room.NewManager(memStore{})
	m.Dev = sc.setup != nil
	m.Now = func() time.Time { return time.Unix(1_700_000_000, 0) }
	m.NewCode = func() string { return "TRCE" }
	rec := &recorder{w: &Schreiber{}, scene: sc.name}
	slots := make([]int, sc.players)
	for i := range slots {
		slots[i] = i
	}
	r, err := m.Create("trace", rec, "trace-"+sc.name, true, 0, slots, room.Options{Grade: "normal"})
	if err != nil {
		t.Fatal(err)
	}
	if sc.setup != nil {
		sc.setup(func(a room.DevAction) {
			if err := r.Dev("trace", rec, a); err != nil {
				t.Fatalf("Dev-Aktion %s: %v", a.Action, err)
			}
		})
	}
	for tick := range n {
		in := map[int]sim.PlayerCommand{}
		for p := range sc.players {
			in[p] = input(p, tick)
		}
		if err := r.Input("trace", rec, in); err != nil {
			t.Fatal(err)
		}
		r.Tick()
	}
	if rec.err != nil {
		t.Fatal(rec.err)
	}
	return rec.w
}

// TestAufzeichnung schreibt mit -write alle Szenen (lang nach .work/trace/, die kurze nach testdata/rust/trace/);
// ohne -write prüft er, dass ein neuer Lauf der eingecheckten Aufzeichnung gleicht und dass zwei Läufe gleich sind.
func TestAufzeichnung(t *testing.T) {
	if *write {
		schreiben(t)
		return
	}
	var sc scene
	for _, s := range scenes {
		if s.name == shortScene {
			sc = s
		}
	}
	a, b := record(t, sc, shortTicks).Lines(), record(t, sc, shortTicks).Lines()
	a, b = a[:shortTicks+1], b[:shortTicks+1] // Kopf und Takte 0 bis 299 (Takt 0 ist der Startzustand)
	if !equalLines(a, b) {
		t.Fatal("zwei Läufe derselben Szene unterscheiden sich")
	}
	raw, err := os.ReadFile(shortPath)
	if err != nil {
		t.Fatalf("%v (neu schreiben mit -write)", err)
	}
	want, err := Entpacken(raw)
	if err != nil {
		t.Fatal(err)
	}
	if !equalLines(a, want) {
		t.Fatalf("neuer Lauf weicht von %s ab (bei gewollter Änderung neu schreiben mit -write)", shortPath)
	}
	if len(want) != shortTicks+1 {
		t.Fatalf("%d Zeilen, erwartet %d", len(want), shortTicks+1)
	}
}

// TestZeilenformat prüft Kopf und Takt der eingecheckten Aufzeichnung.
func TestZeilenformat(t *testing.T) {
	raw, err := os.ReadFile(shortPath)
	if err != nil {
		t.Skip("noch nicht geschrieben")
	}
	lines, err := Entpacken(raw)
	if err != nil {
		t.Fatal(err)
	}
	var head Kopf
	if err := json.Unmarshal(lines[0], &head); err != nil || head.Art != "level" || head.Szene != shortScene || head.Spieler != 2 {
		t.Fatalf("Kopfzeile: %v %+v", err, head)
	}
	var tk struct {
		Art        string          `json:"art"`
		Takt       int             `json:"takt"`
		Zustand    json.RawMessage `json:"zustand"`
		Ereignisse []any           `json:"ereignisse"`
	}
	for i, l := range lines[1:] {
		tk.Takt = 0
		if err := json.Unmarshal(l, &tk); err != nil || tk.Art != "takt" || tk.Takt != i || len(tk.Zustand) == 0 || tk.Ereignisse == nil {
			t.Fatalf("Taktzeile %d: %v art=%q takt=%d", i, err, tk.Art, tk.Takt)
		}
	}
}

func schreiben(t *testing.T) {
	t.Helper()
	if err := os.MkdirAll(longDir, 0o755); err != nil {
		t.Fatal(err)
	}
	for _, sc := range scenes {
		rec := record(t, sc, longTicks)
		put(t, filepath.Join(longDir, fmt.Sprintf("%s-%d.jsonl.gz", sc.name, longTicks)), rec, -1)
		if sc.name == shortScene {
			if err := os.MkdirAll(filepath.Dir(shortPath), 0o755); err != nil {
				t.Fatal(err)
			}
			put(t, shortPath, rec, shortTicks)
		}
	}
}

func put(t *testing.T, path string, rec *Schreiber, n int) {
	t.Helper()
	gz, err := rec.Gzip(n)
	if err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, gz, 0o644); err != nil {
		t.Fatal(err)
	}
	t.Logf("💾 %s: %d Zeilen, %d Bytes", path, len(rec.Lines()), len(gz))
}

func equalLines(a, b [][]byte) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if !bytes.Equal(a[i], b[i]) {
			return false
		}
	}
	return true
}
