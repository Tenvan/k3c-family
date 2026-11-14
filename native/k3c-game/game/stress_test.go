package game

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
	"time"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// stressSim ist devSim, der neue Spiele mitschreibt.
type stressSim struct {
	*devSim
	games []view.NewGame
}

func (s *stressSim) New(g view.NewGame) error { s.games = append(s.games, g); return nil }

// stressGameOf baut eine Schleife mit Schein-Simulation, fester Uhr, Bildrate 60 und Berichtsordner dir.
func stressGameOf(t *testing.T, dir string) (*Game, *stressSim, *time.Time) {
	t.Helper()
	s := &stressSim{devSim: &devSim{fakeSim: newFakeSim()}}
	g := playGame(s.fakeSim)
	g.sim = s
	now := time.Date(2026, 11, 14, 12, 0, 0, 0, time.UTC)
	g.clock, g.fpsOf, g.reports = func() time.Time { return now }, func() float64 { return 60 }, dir
	return g, s, &now
}

// stressFrame ist ein Frame mit Spieler 1 an der Tastatur und einer Stufe mit n Gegnern.
func stressFrame(tick uint64, enemies int, events ...string) view.Frame {
	w := &sim.World{}
	for range enemies {
		w.Enemies = append(w.Enemies, &sim.Enemy{})
	}
	f := view.Frame{Tick: tick, StepMs: float64(tick % 7), Stages: []view.Stage{{World: w}},
		Seats: []view.Seat{{Player: 0, Device: view.DeviceID{Kind: view.DeviceKeyboard}}}}
	for _, e := range events {
		f.Events = append(f.Events, view.Event{Data: sim.Event{"type": e}})
	}
	return f
}

var stressCfg = view.StressTest{Players: 2, Depth: 1, Seed: 42, Enemies: 10, Growth: view.GrowthLinear, Factor: 5,
	Build: 3, Troops: 20, Timescale: 4, Waves: 2, Enemy: "goblin", Troop: "archer"}

// B-896/AC-02: Ein gestarteter Test läuft mit den eingestellten Werten: Spiel mit Seed, Tiefe und Geräten, F2 und F10
// an, im ersten Takt Ausbau, Truppen und Zeitraffer, zu jeder Welle zusätzliche Gegner nach der Kurve.
func TestStressStartsWithConfig(t *testing.T) {
	g, s, _ := stressGameOf(t, t.TempDir())
	by := view.DeviceID{Kind: view.DeviceKeyboard}
	if err := (menuSession{Sim: g.sim, g: g}).StartStress(stressCfg, by); err != nil {
		t.Fatal(err)
	}
	if len(s.games) != 1 {
		t.Fatalf("%d Spiele", len(s.games))
	}
	ng := s.games[0]
	if ng.Seed != "42" || ng.Depth != 1 || ng.Name != stressSave || len(ng.Devices) != 2 || ng.Devices[0] != by {
		t.Errorf("Spiel %+v", ng)
	}
	if len(s.set) != 1 || !s.set[0].Stats || g.dev.Debug != view.DebugValues {
		t.Errorf("F2 %v, F10 %d", s.set, g.dev.Debug)
	}
	g.frame = stressFrame(10, 0)
	g.stressStep()
	g.frame = stressFrame(11, 0, "wave")
	g.stressStep()
	g.frame = stressFrame(12, 10, "wave")
	g.stressStep()
	want := []view.DevAction{
		{Action: "buildAll", Level: 3},
		{Action: "spawn", Kind: "troop", Unit: "archer", Amount: 20},
		{Action: "timescale", Factor: 4},
		{Action: "spawn", Kind: "enemy", Unit: "goblin", Amount: 10},
		{Action: "spawn", Kind: "enemy", Unit: "goblin", Amount: 15},
	}
	if len(s.actions) != len(want) {
		t.Fatalf("Aktionen %+v", s.actions)
	}
	for i, a := range want {
		if s.actions[i] != a {
			t.Errorf("Aktion %d: %+v, erwartet %+v", i, s.actions[i], a)
		}
	}
	if g.stress == nil {
		t.Error("Test endet, solange Gegner der letzten Welle leben")
	}
}

// B-896/AC-03: Ist die letzte Welle geschlagen, liegt ein Bericht mit den Leistungswerten im Berichtsordner, die
// Simulation steht und der Test ist vorbei.
func TestStressWritesReport(t *testing.T) {
	dir := t.TempDir()
	g, s, now := stressGameOf(t, dir)
	if err := g.startStress(stressCfg, view.DeviceID{Kind: view.DeviceKeyboard}); err != nil {
		t.Fatal(err)
	}
	runWaves(g, now)
	if g.stress != nil || !s.halt {
		t.Fatalf("Test läuft noch (%v) oder Simulation nicht angehalten (%v)", g.stress != nil, s.halt)
	}
	checkReport(t, readReport(t, dir))
}

// runWaves taktet zwei Wellen (Takt 110 und 130); ab Takt 151 sind alle Gegner geschlagen.
func runWaves(g *Game, now *time.Time) {
	for tick := uint64(100); tick < 160 && g.stress != nil; tick++ {
		ev, enemies := []string{}, 5
		if tick == 110 || tick == 130 {
			ev = append(ev, "wave")
		}
		if tick > 150 {
			enemies = 0
		}
		*now = now.Add(time.Second)
		g.frame = stressFrame(tick, enemies, ev...)
		g.stressStep()
	}
}

// readReport liest den einzigen Bericht in dir.
func readReport(t *testing.T, dir string) StressReport {
	t.Helper()
	files, _ := filepath.Glob(filepath.Join(dir, "native-stress-*.json"))
	if len(files) != 1 {
		t.Fatalf("Berichte %v", files)
	}
	raw, err := os.ReadFile(files[0])
	if err != nil {
		t.Fatal(err)
	}
	var r StressReport
	if err := json.Unmarshal(raw, &r); err != nil {
		t.Fatal(err)
	}
	return r
}

// checkReport prüft die Leistungswerte des Berichts nach runWaves.
func checkReport(t *testing.T, r StressReport) {
	t.Helper()
	if r.Kind != "native-stress" || r.Cause != "wellen" || r.Waves != 2 || r.Ticks != 51 || r.TicksPerSecond <= 0 ||
		r.StepMs.Max <= 0 || r.StepMs.P95 < r.StepMs.P50 || r.FpsMean != 60 || r.Peak.Enemies != 5 || r.HeapMib <= 0 ||
		r.Config.Preset != stressCfg.Preset {
		t.Errorf("Bericht %+v", r)
	}
}

// Verlassen oder Spielende beenden den Test mit Bericht; ohne Entwickler-Rolle startet er nicht.
func TestStressEndsOnLeaveAndNeedsDev(t *testing.T) {
	dir := t.TempDir()
	g, s, _ := stressGameOf(t, dir)
	if err := g.startStress(stressCfg, view.DeviceID{}); err != nil {
		t.Fatal(err)
	}
	g.frame = stressFrame(1, 0)
	g.stressStep()
	s.ended = true
	g.stressStep()
	files, _ := filepath.Glob(filepath.Join(dir, "native-stress-*.json"))
	if g.stress != nil || len(files) != 1 || s.halt {
		t.Errorf("Verlassen: läuft %v, Berichte %v, angehalten %v", g.stress != nil, files, s.halt)
	}
	plain := playGame(newFakeSim())
	if err := plain.startStress(stressCfg, view.DeviceID{}); err == nil {
		t.Error("ohne Entwickler-Rolle gestartet")
	}
}

// Gegner je Welle: linear + Faktor, exponentiell × Faktor, gedeckelt bei 200.
func TestStressEnemiesFor(t *testing.T) {
	lin := view.StressTest{Enemies: 10, Growth: view.GrowthLinear, Factor: 5}
	exp := view.StressTest{Enemies: 10, Growth: view.GrowthExp, Factor: 2}
	cases := []struct{ got, want int }{
		{lin.EnemiesFor(0), 0}, {lin.EnemiesFor(1), 10}, {lin.EnemiesFor(3), 20},
		{exp.EnemiesFor(1), 10}, {exp.EnemiesFor(3), 40}, {exp.EnemiesFor(10), view.StressMaxEnemies},
	}
	for i, c := range cases {
		if c.got != c.want {
			t.Errorf("Fall %d: %d, erwartet %d", i, c.got, c.want)
		}
	}
}
