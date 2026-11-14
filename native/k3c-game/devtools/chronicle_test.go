package devtools

import (
	"bytes"
	"encoding/json"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// chronicleFor liefert eine Chronik, die in buf loggt und Berichte nach dir schreibt, mit fester Uhr.
func chronicleFor(dir string) (*chronicle, *bytes.Buffer) {
	var buf bytes.Buffer
	log := slog.New(slog.NewTextHandler(&buf, &slog.HandlerOptions{ReplaceAttr: func(_ []string, a slog.Attr) slog.Attr {
		if a.Key == slog.TimeKey {
			return slog.Attr{}
		}
		return a
	}}))
	t0 := time.Date(2026, 11, 14, 20, 0, 0, 0, time.UTC)
	n := 0
	now := func() time.Time { n++; return t0.Add(time.Duration(n) * time.Minute) }
	return newChronicle(log, now, dir), &buf
}

// chronFrame ist ein Frame mit einem Spieler in Stufe 0 einer Tag-Welt.
func chronFrame(tick uint64) *view.Frame {
	w := &sim.World{Seed: "wald", Cycle: sim.CycleInfo{Phase: "day", Day: 1}, Troops: []*sim.Troop{{ID: 1}, {ID: 2}},
		Players: []*sim.Player{{Index: 0, Gold: 10}}}
	return &view.Frame{Tick: tick, Seats: []view.Seat{{Player: 0, Monarch: 0, Stage: 0}},
		Stages: []view.Stage{{Index: 0, World: w}}}
}

func logLines(buf *bytes.Buffer) []string {
	return strings.Split(strings.TrimSpace(buf.String()), "\n")
}

// AC-06 (B-889): Ohne Wechsel bleibt das Log leer, auch über viele Takte; ein Tageszeitwechsel steht genau einmal da.
func TestChronicleQuietWithoutChange(t *testing.T) {
	c, buf := chronicleFor("")
	f := chronFrame(1)
	for i := range 300 {
		f.Tick = uint64(i + 1)
		c.Listen(f)
	}
	if buf.Len() != 0 {
		t.Fatalf("Log ohne Wechsel: %s", buf)
	}
	f.Stages[0].World.Cycle.Phase = "dusk"
	c.Listen(f)
	c.Listen(f)
	if l := logLines(buf); len(l) != 1 || !strings.Contains(l[0], "🚦 Tageszeit") || !strings.Contains(l[0], "phase=dusk") {
		t.Errorf("Wechsel: %v", l)
	}
}

// AC-06: Eine Nacht fasst gespawnt, getötet, Verluste und Schaden je Seite in einer Zeile 🏹 zusammen; Gold-Änderungen
// stehen am Tageswechsel als 🪙 je Spieler; Bau fertig und Burgfall sofort.
func TestChronicleNightAndEconomy(t *testing.T) {
	c, buf := chronicleFor("")
	f := chronFrame(1)
	w := f.Stages[0].World
	c.Listen(f)
	ev := func(data ...sim.Event) {
		f.Events = f.Events[:0]
		for _, d := range data {
			f.Events = append(f.Events, view.Event{Stage: 0, Data: d})
		}
		f.Tick++
		c.Listen(f)
	}
	w.Cycle.Phase, w.Players[0].Gold = "night", 25
	ev(sim.Event{"type": "night"}, sim.Event{"type": "wave", "count": 6}, sim.Event{"type": "built", "kind": "wall"})
	w.Players[0].Gold = 20
	ev(sim.Event{"type": "kill"}, sim.Event{"type": "kill"}, sim.Event{"type": "hit", "target": "enemy", "damage": 4.0},
		sim.Event{"type": "hit", "target": "troop", "damage": 2.5}, sim.Event{"type": "castleFallen"})
	w.Troops = w.Troops[:1]
	w.Cycle.Phase, w.Cycle.Day = "dawn", 2
	ev(sim.Event{"type": "dawn"})
	text := buf.String()
	for _, want := range []string{"🚦 Bau fertig", "art=wall", "🚦 Burgfall", "🏹 Kampf der Nacht", "gespawnt=6", "getötet=2",
		"verluste=1", "schaden_an_gegnern=4", "schaden_eigene_seite=2.5", "🪙 Wirtschaft des Tags", "einnahmen=15", "ausgaben=5"} {
		if !strings.Contains(text, want) {
			t.Errorf("fehlt %q in\n%s", want, text)
		}
	}
	if strings.Count(text, "🏹") != 1 || strings.Count(text, "🪙") != 1 {
		t.Errorf("Zusammenfassungen mehrfach:\n%s", text)
	}
}

// AC-06: Ein Takt über Budget loggt 🐢 mit Objektanzahl, weitere langsame Takte im Fenster nicht.
func TestChronicleSlowTicks(t *testing.T) {
	c, buf := chronicleFor("")
	f := chronFrame(100)
	f.StepMs = c.vals.StepBudgetMs + 1
	for i := range 20 {
		f.Tick = uint64(100 + i)
		c.Listen(f)
	}
	if l := logLines(buf); len(l) != 1 || !strings.Contains(l[0], "🐢 Takt über Budget") || !strings.Contains(l[0], "truppen=2") {
		t.Errorf("langsame Takte: %v", l)
	}
	f.Tick += uint64(c.vals.SlowLogEverySeconds * 30)
	c.Listen(f)
	if !strings.Contains(buf.String(), "langsame_takte=20") {
		t.Errorf("zweite Meldung: %s", buf)
	}
}

// AC-06: Am Spielende liegt einmal der Sitzungsbericht als JSON in reports/; Programmende ohne Spiel schreibt nichts.
func TestChronicleReport(t *testing.T) {
	dir := t.TempDir()
	c, _ := chronicleFor(dir)
	f := chronFrame(10)
	c.Listen(f)
	f.Tick, f.Stages[0].World.Cycle.Day = 70, 3
	c.Listen(f)
	f.Outcome = view.OutcomeLost
	c.Listen(f)
	c.Listen(&view.Frame{Outcome: view.OutcomeLost})
	c.Finish()
	files, _ := filepath.Glob(filepath.Join(dir, "native-session-*.json"))
	if len(files) != 1 {
		t.Fatalf("Berichte: %v", files)
	}
	raw, err := os.ReadFile(files[0])
	if err != nil {
		t.Fatal(err)
	}
	var r Report
	if err := json.Unmarshal(raw, &r); err != nil {
		t.Fatal(err)
	}
	if r.Seed != "wald" || r.Days != 3 || r.Cause != "verloren" || r.Ticks != 60 || r.Seconds <= 0 || r.Kind != "native-session" {
		t.Errorf("Bericht: %+v", r)
	}
}
