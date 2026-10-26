package sim

import (
	"encoding/json"
	"strings"
	"testing"

	"k3c/engine/internal/golden"
	"k3c/engine/level"
)

// TestGoldenGzAbweichung (GD1, AC-02): Ein gepackter Golden-Lauf vergleicht so streng wie die JSON-Datei. Der erste
// Snapshot aus sim-forest-tag.json.gz passt zur frisch erzeugten Welt; ein absichtlich geändertes Feld meldet Diff
// mit seinem Pfad.
func TestGoldenGzAbweichung(t *testing.T) {
	goldenPurse(t)
	var run goldenRun
	if err := json.Unmarshal(golden.Read(t, "../../testdata/golden/sim-forest-tag.json.gz"), &run); err != nil {
		t.Fatal(err)
	}
	if len(run.Snapshots) == 0 || run.Snapshots[0].Tick != 0 {
		t.Fatal("sim-forest-tag: kein Snapshot bei Tick 0")
	}
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
	want := run.Snapshots[0].World
	if d := golden.Diff("world", want, golden.Tree(t, w)); d != "" {
		t.Fatalf("Tick 0 weicht schon ohne Änderung ab: %s", d)
	}
	want["stock"].(map[string]any)["wood"] = 1.0
	if d := golden.Diff("world", want, golden.Tree(t, w)); !strings.HasPrefix(d, "world.stock.wood") {
		t.Fatalf("geändertes Feld nicht erkannt: %q", d)
	}
}

// goldenPurse stellt für einen Golden-Lauf den Beutel der Zeit vor der Münzskala (100 Gold, BS3) ein: Die aufgezeichneten
// Eingaben brauchen so viel Gold, und die Läufe prüfen die Simulation, nicht die Werte der Wirtschaft.
func goldenPurse(t *testing.T) {
	t.Helper()
	old := economy.Purse
	economy.Purse.StartGold, economy.Purse.MaxGold = 100, 100
	t.Cleanup(func() { economy.Purse = old })
}
