package e2e

// Treiber des Ende-zu-Ende-Laufs: die echte Schleife (game.New mit allen Fachpaketen, ohne Grafik-Bibliothek) und
// local im Aufnahme-Modus (Speichern im Takt, feste Uhr), getaktet über game.Game.Update wie im Fenster.

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"maps"
	"slices"
	"testing"
	"time"

	"k3c/engine/sim"
	"k3c/native/k3c-game/game"
	"k3c/native/k3c-game/local"
	"k3c/native/k3c-game/view"
)

// fixedNow ist die Uhr der Spielstände (wie main.go › shotTime im Aufnahme-Modus).
func fixedNow() time.Time { return time.Date(2026, 1, 1, 12, 0, 0, 0, time.UTC) }

// station ist ein geprüfter Punkt des Ablaufs: Name, Takt (Zahl der Updates seit dem Start, wie --at) und Welt-Hash.
type station struct {
	Name string
	Tick int
	Hash string
}

// run ist ein Lauf des Ablaufs.
type run struct {
	t        *testing.T
	g        *game.Game
	sim      *local.Session
	dir      string
	tick     int // Zahl der Updates seit dem Start
	stations []station
}

// newRun baut das Spiel wie main.go (ohne Grafik-Bibliothek und Fenster) mit den Schaltern argv.
func newRun(t *testing.T, argv ...string) *run {
	t.Helper()
	dir := t.TempDir()
	args, err := game.ParseArgs(append(argv, "--saves", dir), dir)
	if err != nil {
		t.Fatal(err)
	}
	parts, err := game.Assemble(game.Packages(nil))
	if err != nil {
		t.Fatal(err)
	}
	s := local.Open(local.Config{Dir: dir, Now: fixedNow, Recording: true})
	g, err := game.New(parts, s, args)
	if err != nil {
		t.Fatal(err)
	}
	return &run{t: t, g: g, sim: s, dir: dir}
}

// frame ist der Frame des letzten Updates.
func (r *run) frame() *view.Frame { return r.g.Frame() }

// step rechnet ein Update.
func (r *run) step() {
	r.t.Helper()
	if err := r.g.Update(); err != nil {
		r.t.Fatalf("Takt %d: Update: %v", r.tick, err)
	}
	r.tick++
}

// to rechnet bis zum Takt tick.
func (r *run) to(tick int) {
	r.t.Helper()
	for r.tick < tick {
		r.step()
	}
}

// until rechnet, bis cond gilt; spätestens beim Takt limit bricht der Test mit what ab.
func (r *run) until(what string, limit int, cond func(f *view.Frame) bool) {
	r.t.Helper()
	for !cond(r.frame()) {
		if r.tick >= limit {
			r.t.Fatalf("❌ %s: bis Takt %d nicht erreicht", what, limit)
		}
		r.step()
	}
}

// mark merkt die Station name beim aktuellen Takt mit dem Welt-Hash des Frames.
func (r *run) mark(name string) {
	r.stations = append(r.stations, station{Name: name, Tick: r.tick, Hash: frameHash(r.t, r.frame())})
	r.t.Logf("✅ Station %-12s Takt %6d, Sim-Takt %6d, %s", name, r.tick, r.frame().Tick, screenName(r.g))
}

// screenName ist der Typ des obersten Screens ("" im Spiel).
func screenName(g *game.Game) string {
	if s := g.Screen(); s != nil {
		return fmt.Sprintf("%T", s)
	}
	return ""
}

// expectScreen prüft den obersten Screen.
func (r *run) expectScreen(want string) {
	r.t.Helper()
	if got := screenName(r.g); got != want {
		r.t.Fatalf("Takt %d: Screen %q, erwartet %q", r.tick, got, want)
	}
}

// frameHash ist der Welt-Hash des Frames: SHA-256 über das JSON der Welten nach Stufen-Index ("-" ohne Spiel).
func frameHash(t *testing.T, f *view.Frame) string {
	t.Helper()
	worlds := map[int]*sim.World{}
	for _, st := range f.Stages {
		worlds[st.Index] = st.World
	}
	return hashWorlds(t, worlds)
}

func hashWorlds(t *testing.T, worlds map[int]*sim.World) string {
	t.Helper()
	if len(worlds) == 0 {
		return "-"
	}
	h := sha256.New()
	for _, k := range slices.Sorted(maps.Keys(worlds)) {
		raw, err := json.Marshal(worlds[k])
		if err != nil {
			t.Fatal(err)
		}
		h.Write(raw)
	}
	return hex.EncodeToString(h.Sum(nil))
}

// hasEvent meldet ein Ereignis des Typs typ im Frame; player ≥ 0 verlangt diesen Spieler (Feld player).
func hasEvent(f *view.Frame, typ string, player int) bool {
	for _, e := range f.Events {
		if e.Type() == typ && (player < 0 || fmt.Sprint(e.Data["player"]) == fmt.Sprint(player)) {
			return true
		}
	}
	return false
}

// hasCue meldet einen Cue der Art k im Frame.
func hasCue(f *view.Frame, k view.CueKind) bool {
	return slices.ContainsFunc(f.Cues, func(c view.Cue) bool { return c.Kind == k })
}

// enemyNear meldet einen lebenden Gegner höchstens d Units neben einem Monarchen der Spieler.
func enemyNear(f *view.Frame, d float64) bool {
	for _, s := range f.Seats {
		m, st := f.Monarch(s.Player), f.World(view.Cell{Player: s.Player})
		if m == nil || st == nil {
			continue
		}
		for _, e := range st.Enemies {
			if e.HP > 0 && e.X > m.X-d && e.X < m.X+d {
				return true
			}
		}
	}
	return false
}
