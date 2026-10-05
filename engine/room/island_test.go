package room

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"time"

	"k3c/engine/sim"
	"k3c/engine/store"
)

// moveToExit stellt Monarch i an den Tiefen-Eingang seiner Stufe (Ausgang nach unten).
func moveToExit(t testing.TB, r *Room, i int) {
	t.Helper()
	w := r.isl.Stages[r.isl.StageOf(i)]
	for _, e := range w.Level.Entities {
		if e.Kind != "exit" {
			continue
		}
		for _, p := range w.Players {
			if p.Index == i {
				p.X = e.X
				return
			}
		}
	}
	t.Fatalf("kein Ausgang für Monarch %d", i)
}

// AC-01: Zwei Geräte, zwei Spieler: Nur der Wechsler wechselt die Stufe, nur sein Gerät bekommt `level` (vor `state`).
func TestEinzelwechselSchicktLevelAnRichtigesGeraet(t *testing.T) {
	f := newFixture()
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "zwei", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	if r.isl.StageOf(0) != 0 || r.isl.StageOf(1) != 0 {
		t.Fatalf("Start: Stufen %d, %d", r.isl.StageOf(0), r.isl.StageOf(1))
	}
	before := len(h.log)
	moveToExit(t, r, 0)
	ticks(r, 70)
	if r.isl.StageOf(0) != 1 || r.isl.StageOf(1) != 0 {
		t.Fatalf("nach Wechsel: Stufen %d, %d", r.isl.StageOf(0), r.isl.StageOf(1))
	}
	i := slices.Index(x.log, "level 1")
	if i < 0 || i+1 >= len(x.log) || !strings.HasPrefix(x.log[i+1], "state") || x.world != r.isl.Stages[1] {
		t.Fatalf("Xbox: Level vor Zustand der Stufe 1 fehlt: %v", x.log[len(x.log)-4:])
	}
	if h.has("level 1") || h.world != r.isl.Stages[0] || len(h.log) <= before {
		t.Fatalf("Handy bekam das Level der anderen Stufe: %v", h.log[before:])
	}
	// Das Gerät folgt dem Monarchen im kleinsten Slot.
	moveToExit(t, r, 1)
	ticks(r, 70)
	if !h.has("level 1") || h.world != r.isl.Stages[1] {
		t.Fatalf("Handy folgt nicht: %v", h.log[len(h.log)-3:])
	}
}

// Ein neuer Monarch tritt in der Stufe seines Geräts bei, ein Gerät ohne Spieler in der Startstufe.
func TestBeitrittInStufeDesGeraets(t *testing.T) {
	f := newFixture()
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "beitritt", true, 1, []int{0}, Options{}))(t)
	if r.isl.StageOf(0) != 1 || !x.has("level 1") {
		t.Fatalf("Startstufe 1: Stufe %d, %v", r.isl.StageOf(0), x.log)
	}
	moveToExit(t, r, 0)
	ticks(r, 70) // Monarch 0 steht jetzt in Stufe 2
	ok(t, r.AddSlot("xbox", r.peerOf("xbox"), 1))
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	if r.isl.StageOf(1) != 2 || r.isl.StageOf(2) != 1 || !h.has("level 1") {
		t.Fatalf("Stufen: Slot 1 → %d, Handy → %d, %v", r.isl.StageOf(1), r.isl.StageOf(2), h.log)
	}
}

func fileStore(t *testing.T) (*fixture, *store.Saves) {
	t.Helper()
	f := newFixture()
	s := &store.Saves{Dir: filepath.Join(t.TempDir(), "saves")}
	f.m.Store = s
	return f, s
}

// AC-02: Speichern und Laden einer Insel über den Store (Rundlauf).
func TestInselRundlauf(t *testing.T) {
	f, s := fileStore(t)
	x := &peer{}
	r := need(f.m.Create("xbox", x, "insel", true, 0, []int{0, 1}, Options{}))(t)
	r.isl.Players()[1].Gold = 77
	moveToExit(t, r, 0)
	ticks(r, 70)
	r.Leave("xbox", x)
	waitSaved(r)
	got := need(sim.ParseIslandSave(need(s.Load("insel"))(t)))(t)
	if got.Version != sim.IslandSaveVersion || len(got.Stages) != 3 || len(got.Players) != 2 {
		t.Fatalf("gespeichert: Version %d, %d Stufen, %d Spieler", got.Version, len(got.Stages), len(got.Players))
	}
	f.wait(11 * time.Minute) // leerer Raum wird aufgeräumt
	y := &peer{}
	r2 := need(f.m.Create("xbox", y, "insel", false, 0, []int{0}, Options{}))(t)
	ps := r2.isl.Players()
	if len(ps) != 2 || ps[1].Gold != 77 || r2.isl.StageOf(0) != 1 || r2.isl.StageOf(1) != 0 || fmt.Sprint(y.monarchs) != "[taken free]" {
		t.Fatalf("geladen: %d Spieler, Stufen %d/%d, Plätze %v", len(ps), r2.isl.StageOf(0), r2.isl.StageOf(1), y.monarchs)
	}
}

// AC-02: Ein Stand Version 1 wird geladen; die Datei bleibt als Sicherung, der neue Stand ist Version 2.
func TestVersion1WirdUeberfuehrtUndGesichert(t *testing.T) {
	f, s := fileStore(t)
	c := sim.CreateCampaign("alt", "alt-id", 1)
	c.JoinPlayer().Gold = 12
	v1 := need(json.Marshal(c.ToSave("2026-01-01T00:00:00.000Z")))(t)
	_ = need(s.Store("alt", v1))(t)
	x := &peer{}
	r := need(f.m.Create("xbox", x, "alt", false, 0, []int{0}, Options{}))(t)
	if r.isl.Players()[0].Gold != 12 {
		t.Fatal("Gold aus Version 1 fehlt")
	}
	r.Leave("xbox", x) // speichert als Version 2
	waitSaved(r)
	if got := need(sim.ParseIslandSave(need(s.Load("alt"))(t)))(t); got.Version != sim.IslandSaveVersion {
		t.Fatalf("Version nach Speichern: %d", got.Version)
	}
	if len(r.isl.Stages) != 3 {
		t.Fatalf("Stufen: %d, erwartet 3 (sonst kein Weiterreisen)", len(r.isl.Stages))
	}
	// Die Datei Version 1 liegt dauerhaft neben dem Slot (nicht in der rotierenden Sicherung, die nach 5 Speicherungen verfällt).
	list := need(filepath.Glob(filepath.Join(s.Dir, "alt-*.json")))(t)
	if len(list) != 1 {
		t.Fatalf("dauerhafte Sicherung: %v", list)
	}
	old := need(os.ReadFile(list[0]))(t)
	if string(old) != string(v1) {
		t.Fatal("Sicherung ist nicht der Stand Version 1")
	}
}

// AC-03: Der Status nennt die Stufen mit Tiefe, Phase, Tag und Spielern.
func TestStatusZeigtStufen(t *testing.T) {
	f := newFixture()
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "status", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	moveToExit(t, r, 0)
	ticks(r, 70)
	st, _ := f.m.Status()
	if len(st) != 1 || fmt.Sprint(st[0].Stages) != "[{0 day 1 [1]} {1 day 1 [0]} {2 day 1 []}]" {
		t.Fatalf("Status.Stages: %+v", st)
	}
	if got := fmt.Sprint(r.Summary().Stages); got != fmt.Sprint(st[0].Stages) {
		t.Fatalf("Summary.Stages: %s", got)
	}
}

// N1/AC-01 (B-276/AC-01): Zwei Geräte auf derselben Stufe teilen einen Zustand je Tick; auf zwei Stufen sind es zwei.
func TestEinZustandJeStufeUndTick(t *testing.T) {
	f := newFixture()
	built := 0
	f.m.Snapshot = func(w *sim.World, _ int, _ bool) any { built++; return w }
	x, h := &peer{}, &peer{}
	r := need(f.m.Create("xbox", x, "geteilt", true, 0, []int{0}, Options{}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	built = 0
	ticks(r, 1)
	if built != 1 || x.world != h.world {
		t.Fatalf("eine Stufe, 2 Geräte: %d Zustände je Tick, geteilt %v", built, x.world == h.world)
	}
	moveToExit(t, r, 0)
	ticks(r, 70)
	if r.isl.StageOf(0) == r.isl.StageOf(1) {
		t.Fatal("Stufenwechsel nicht erreicht")
	}
	built = 0
	ticks(r, 1)
	if built != 2 {
		t.Fatalf("zwei Stufen: %d Zustände je Tick", built)
	}
}
