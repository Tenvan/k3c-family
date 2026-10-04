package sim

import (
	"encoding/json"
	"os"
	"path/filepath"
	"slices"
	"testing"

	"k3c/engine/internal/golden"
	"k3c/engine/level"
)

// goldenRun ist eine Datei testdata/golden/sim-*.json (ursprünglich aus der TS-Simulation; neu schreiben mit
// `task golden:update`, B-137).
type goldenRun struct {
	path          string
	Name          string
	Biome         string
	Seed          string
	CycleSpeed    float64
	Players       int
	Dt            float64
	Ticks         int
	SnapshotEvery int
	Setup         *goldenSetup // nach CreateWorld und AddPlayer
	Inputs        []struct {
		Ticks    int
		Commands []PlayerCommand
	}
	Snapshots []struct {
		Tick  int
		World map[string]any
	}
}

// goldenSetup: Start-Vorrat, Pool-Punkte und je Spieler die zu lernenden Skills (S1.4).
type goldenSetup struct {
	Stock  Stock
	Pool   int
	Skills [][]string
}

func goldenRuns(t *testing.T) []goldenRun {
	t.Helper()
	files, err := filepath.Glob("../../testdata/golden/sim-*.json")
	if err != nil || len(files) == 0 {
		t.Fatalf("keine Golden-Läufe in testdata/golden/: %v", err)
	}
	runs := make([]goldenRun, len(files))
	for i, f := range files {
		raw, err := os.ReadFile(f)
		if err == nil {
			err = json.Unmarshal(raw, &runs[i])
		}
		if err != nil || (len(runs[i].Snapshots) == 0 && !*golden.Update) || runs[i].SnapshotEvery == 0 {
			t.Fatalf("%s unbrauchbar: %v", f, err)
		}
		runs[i].path = f
	}
	return runs
}

// snapshot ist ein Eintrag von snapshots in der Reihenfolge der Datei.
type snapshot struct {
	Tick  int `json:"tick"`
	World any `json:"world"`
}

// compare vergleicht einen Snapshot vollständig; mit -update sammelt es ihn stattdessen in got.
func compare(t *testing.T, run goldenRun, tick int, want map[string]any, w *World, got *[]snapshot) {
	t.Helper()
	if *golden.Update {
		*got = append(*got, snapshot{tick, golden.Raw(t, w)})
		return
	}
	if d := golden.Diff("world", want, golden.Tree(t, w)); d != "" {
		t.Fatalf("sim-%s Tick %d: %s", run.Name, tick, d)
	}
}

// replay rechnet einen Golden-Lauf mit seinen Eingaben nach und vergleicht jeden Snapshot.
func replay(t *testing.T, run goldenRun) {
	b, err := level.LoadBiome(run.Biome)
	if err != nil {
		t.Fatal(err)
	}
	w, err := CreateWorld(b, run.Seed, Options{CycleSpeed: run.CycleSpeed})
	if err != nil {
		t.Fatal(err)
	}
	for range run.Players {
		AddPlayer(w)
	}
	if run.Setup != nil {
		applySetup(t, w, run)
	}
	snap, tick := 0, 0
	var got []snapshot
	compare(t, run, 0, wantAt(t, run, 0, 0), w, &got)
	for _, seg := range run.Inputs {
		for range seg.Ticks {
			Step(w, seg.Commands, run.Dt)
			tick++
			if tick%run.SnapshotEvery != 0 {
				continue
			}
			snap++
			compare(t, run, tick, wantAt(t, run, snap, tick), w, &got)
		}
	}
	if tick != run.Ticks || (snap != len(run.Snapshots)-1 && !*golden.Update) {
		t.Fatalf("sim-%s: %d Ticks und %d Snapshots gerechnet, Datei hat %d und %d", run.Name, tick, snap+1, run.Ticks, len(run.Snapshots))
	}
	if *golden.Update {
		f := golden.ReadFile(t, run.path)
		f.Set(t, "snapshots", got)
		f.Write(t, run.path)
	}
}

// applySetup setzt Vorrat, Pool und Skills (S1.4: Lauf forest-monarch).
func applySetup(t *testing.T, w *World, run goldenRun) {
	*w.Stock = run.Setup.Stock
	addPoolPoints(w, run.Setup.Pool)
	for i, ids := range run.Setup.Skills {
		for _, id := range ids {
			if err := LearnSkill(w, w.Players[i], id); err != nil {
				t.Fatalf("sim-%s: %v", run.Name, err)
			}
		}
	}
}

// wantAt ist der erwartete Snapshot Nummer snap; mit -update gibt es noch keinen (neue Datei ohne snapshots).
func wantAt(t *testing.T, run goldenRun, snap, tick int) map[string]any {
	if *golden.Update {
		return nil
	}
	if snap >= len(run.Snapshots) || run.Snapshots[snap].Tick != tick {
		t.Fatalf("sim-%s: Snapshot %d fehlt oder hat nicht Tick %d", run.Name, snap, tick)
	}
	return run.Snapshots[snap].World
}

func TestGoldenLaeufe(t *testing.T) {
	for _, run := range goldenRuns(t) {
		t.Run(run.Name, func(t *testing.T) { replay(t, run) })
	}
}

func TestGoldenLaeufeVollstaendig(t *testing.T) {
	names := []string{}
	for _, run := range goldenRuns(t) {
		names = append(names, run.Name)
	}
	for _, want := range []string{
		"forest-tag", "forest-nacht", "cave-aggression", "forest-ohne-spieler",
		"forest-aufbau", "cave-aufbau", "forest-raub", "forest-sturm", "cave-belagerung", "mine-welle", "forest-monarch",
	} {
		if !slices.Contains(names, want) {
			t.Errorf("Golden-Lauf sim-%s fehlt", want)
		}
	}
}
