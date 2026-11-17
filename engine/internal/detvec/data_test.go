package detvec

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"strings"
	"testing"

	"k3c/engine/balance"
	"k3c/engine/internal/golden"
	"k3c/engine/level"
	"k3c/engine/sim"
)

// JSON-Bäume der Spiel-Daten für den Rust-Port (RU3/AC-04): je Datei unter data/ der Baum der Go-Struktur, in die
// Go sie liest. `k3c-data` vergleicht seine Strukturen damit.

const (
	dataDir      = "../../../data"
	dataTreePath = "../../../testdata/rust/data-tree.json.gz"
)

// dataFile ist eine Datei mit ihrem Lese-Typ in Go.
type dataFile struct {
	name string
	load func(raw []byte) (any, error)
}

// into liest raw in einen frischen Wert vom Typ T, wie `loadFrom` in engine/sim.
func into[T any](raw []byte) (any, error) {
	var v T
	err := json.Unmarshal(raw, &v)
	return v, err
}

// intoType wie into für einen nicht exportierten Typ (über ein exportiertes Feld per reflect geholt).
func intoType(t reflect.Type) func([]byte) (any, error) {
	return func(raw []byte) (any, error) {
		p := reflect.New(t)
		err := json.Unmarshal(raw, p.Interface())
		return p.Elem().Interface(), err
	}
}

// bossesFile vereint die beiden Lese-Typen von bosses.json in engine/sim (loadBosses, loadEndBosses).
type bossesFile struct {
	Bosses    []sim.BossData
	EndBosses []sim.BossData `json:"endBosses"`
}

// dataFiles sind die Spiel-Daten mit ihrem Lese-Typ (RU3.4: balance-hints bis hub und die Biome).
func dataFiles() []dataFile {
	files := []dataFile{
		{"balance-hints.json", func(raw []byte) (any, error) { return balance.LoadHints(raw) }},
		{"balance-targets.json", func(raw []byte) (any, error) { return balance.LoadTargets(raw) }},
		{"bosses.json", into[bossesFile]},
		{"buildings.json", into[map[string]sim.BuildingData]},
		{"difficulty.json", into[struct{ Grades map[string]sim.Grade }]},
		{"economy.json", intoType(reflect.TypeOf(sim.WorldValues{}.Economy))},
		{"enemies.json", into[map[string]sim.EnemyData]},
		{"events.json", into[struct{ Events []sim.NightEvent }]},
		{"goals.json", into[sim.Goals]},
		{"guards.json", into[level.GuardRules]},
		{"hub.json", into[sim.HubData]},
	}
	biomes, _ := filepath.Glob(filepath.Join(dataDir, "biomes", "*.json"))
	for _, p := range biomes {
		id := strings.TrimSuffix(filepath.Base(p), ".json")
		files = append(files, dataFile{"biomes/" + id + ".json", func([]byte) (any, error) { return level.LoadBiome(id) }})
	}
	return files
}

func buildDataTree(t *testing.T) map[string]any {
	t.Helper()
	out := map[string]any{}
	for _, f := range dataFiles() {
		raw, err := os.ReadFile(filepath.Join(dataDir, f.name))
		if err != nil {
			t.Fatal(err)
		}
		v, err := f.load(raw)
		if err != nil {
			t.Fatalf("%s: %v", f.name, err)
		}
		out[f.name] = golden.Tree(t, v)
	}
	return out
}

// TestDataTree schreibt mit -write testdata/rust/data-tree.json.gz; ohne -write vergleicht er Go mit der Datei.
func TestDataTree(t *testing.T) {
	got := buildDataTree(t)
	if *write {
		writeDataTree(t, got)
		return
	}
	var want map[string]any
	if err := json.Unmarshal(golden.Read(t, dataTreePath), &want); err != nil {
		t.Fatal(err)
	}
	if d := golden.Diff("data", any(want), any(got)); d != "" {
		t.Fatalf("Baum weicht ab (neu schreiben mit -write): %s", d)
	}
}

// writeDataTree schreibt ein Objekt mit einer Datei je Zeile, Schlüssel sortiert, gzip ohne Zeitstempel.
func writeDataTree(t *testing.T, tree map[string]any) {
	t.Helper()
	keys := make([]string, 0, len(tree))
	for k := range tree {
		keys = append(keys, k)
	}
	slices.Sort(keys)
	var text bytes.Buffer
	text.WriteString("{\n")
	for i, k := range keys {
		name, _ := json.Marshal(k)
		val, err := json.Marshal(tree[k])
		if err != nil {
			t.Fatal(err)
		}
		text.Write(name)
		text.WriteString(": ")
		text.Write(val)
		if i < len(keys)-1 {
			text.WriteString(",")
		}
		text.WriteString("\n")
	}
	text.WriteString("}\n")
	var buf bytes.Buffer
	zw := gzip.NewWriter(&buf)
	if _, err := zw.Write(text.Bytes()); err != nil {
		t.Fatal(err)
	}
	if err := zw.Close(); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(dataTreePath, buf.Bytes(), 0o644); err != nil {
		t.Fatal(err)
	}
	t.Logf("💾 %s: %d Dateien", dataTreePath, len(keys))
}
