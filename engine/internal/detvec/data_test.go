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

// shipFile ist eine Kopie von sim.shipFile (nicht exportiert, engine/sim/ship.go).
type shipFile struct {
	Stages []struct {
		Cost         sim.Cost
		HP           float64
		BuildSeconds float64
	}
}

// rawCombat und worldFile sind Kopien von sim.rawCombat und sim.rawWorld (nicht exportiert, engine/sim/combat_data.go,
// island_worlds.go): der Kopf einer Welt-Datei, wie ihn decodeWorld liest; die Inseln bleiben roh.
type rawCombat struct {
	Mode      *string `json:"mode"`
	GroupSize *int    `json:"groupSize"`
}

type worldFile struct {
	ID        *string                    `json:"id"`
	Name      *string                    `json:"name"`
	Notes     string                     `json:"notes"`
	Extends   *string                    `json:"extends"`
	Kind      *string                    `json:"kind"`
	Gewaesser string                     `json:"gewaesser"`
	Overrides map[string]json.RawMessage `json:"overrides"`
	Combat    *rawCombat                 `json:"combat"`
	Islands   *[]json.RawMessage         `json:"islands"`
	Story     json.RawMessage            `json:"story"`
	Skins     json.RawMessage            `json:"skins"`
}

// raw ist der Baum der Datei selbst: für Dateien, die nur das native Spiel in Teilen liest (eigenes Go-Modul) oder
// die kein Go-Code liest (render.json).
func raw(b []byte) (any, error) { return into[any](b) }

// dataFiles sind die Spiel-Daten mit ihrem Lese-Typ (RU3.4: balance-hints bis hub und die Biome; RU3.5: der Rest und
// die Welten).
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
		{"island-sizes.json", into[struct{ Sizes []sim.IslandSize }]},
		{"light.json", raw},
		{"monarch.json", intoType(reflect.TypeOf(sim.WorldValues{}.Monarch))},
		{"native.json", raw},
		{"paper.json", raw},
		{"render.json", raw},
		{"ruins.json", into[level.RuinRules]},
		{"ship.json", into[shipFile]},
		{"sizes.json", raw},
		{"sounds.json", raw},
		{"story.json", into[sim.StoryLimits]},
		{"stresstests.json", raw},
		{"troops.json", into[map[string]sim.TroopData]},
		{"waves.json", intoType(reflect.TypeOf(sim.WorldValues{}.Waves))},
	}
	biomes, _ := filepath.Glob(filepath.Join(dataDir, "biomes", "*.json"))
	for _, p := range biomes {
		id := strings.TrimSuffix(filepath.Base(p), ".json")
		files = append(files, dataFile{"biomes/" + id + ".json", func([]byte) (any, error) { return level.LoadBiome(id) }})
	}
	// Nur die eingebettete Standard-Welt: eigene Welten sind lokal und nicht eingecheckt (welten.md § 6).
	for _, id := range []string{sim.StandardWorldID} {
		files = append(files, dataFile{"worlds/" + id + ".json", func(b []byte) (any, error) {
			if _, err := sim.ParseWorld(b, id); err != nil {
				return nil, err
			}
			return into[worldFile](b)
		}})
	}
	return files
}

// rustOnly sind Daten-Dateien, die nur das Rust-Spiel liest (Look-Profile RU55, Welt-Features RU62).
var rustOnly = map[string]bool{
	"look.json": true, "cargo.json": true, "coop.json": true, "festival.json": true,
	"hermits.json": true, "mounts.json": true, "weather.json": true,
}

// TestDataFilesComplete: Jede Spiel-Daten-Datei steht in dataFiles (ohne die Workbench-Dateien asset-*, assets-* und
// ohne data/.omc).
func TestDataFilesComplete(t *testing.T) {
	known := map[string]bool{}
	for _, f := range dataFiles() {
		known[f.name] = true
	}
	for _, pattern := range []string{"*.json", "biomes/*.json", "worlds/" + sim.StandardWorldID + ".json"} {
		files, _ := filepath.Glob(filepath.Join(dataDir, pattern))
		for _, p := range files {
			rel, _ := filepath.Rel(dataDir, p)
			rel = filepath.ToSlash(rel)
			// look.json (RU55) und die Welt-Feature-Dateien (RU62) liest nur das Rust-Spiel; der Go-Stand hängt nicht daran.
			if strings.HasPrefix(rel, "asset-") || strings.HasPrefix(rel, "assets-") || rustOnly[rel] {
				continue
			}
			if !known[rel] {
				t.Errorf("%s fehlt in dataFiles", rel)
			}
		}
	}
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
	// Der Go-Baum ist eingefroren; die Asset-Sprints (BB1, #772) ändern Maße in data/paper.json weiter (wie im Rust-Test
	// `same_as_go_zahlen_aus_daten`): Zahlen der paper.json kommen aus den Daten, Schlüssel und Texte bleiben Vorgabe.
	if w, ok := want["paper.json"]; ok {
		takeNumbers(w, got["paper.json"], func(v any) { want["paper.json"] = v })
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

// takeNumbers ersetzt in want jede Zahl durch die Zahl an derselben Stelle in have (Schlüssel und Listenlängen bleiben
// Vorgabe); set legt eine ersetzte Wurzel ab.
func takeNumbers(want, have any, set func(any)) {
	switch w := want.(type) {
	case float64:
		if h, ok := have.(float64); ok {
			set(h)
		}
	case map[string]any:
		h, _ := have.(map[string]any)
		for k, v := range w {
			if hv, ok := h[k]; ok {
				takeNumbers(v, hv, func(n any) { w[k] = n })
			}
		}
		for k, hv := range h { // neue Schlüssel in den Daten sind erlaubt
			if _, ok := w[k]; !ok {
				w[k] = hv
			}
		}
	case []any:
		h, _ := have.([]any)
		if len(h) == len(w) {
			for i, v := range w {
				takeNumbers(v, h[i], func(n any) { w[i] = n })
			}
		}
	}
}
