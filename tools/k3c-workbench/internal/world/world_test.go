package world

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
)

// Fixture: eine Standard-Welt mit einer Insel, eine Kind-Welt und zwei Biome. Die Engine-Seite (bin/k3c-sim world)
// spielt fakeEngine; ihre echte Rechnung prüfen die Tests von cmd/k3c-sim (world_test.go).
const standardFixture = `{
  "id": "standard",
  "name": "Standard",
  "notes": "Fixture",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }, "plannedChunks": { "0": [ { "slot": -7, "kind": "harbor" } ] } }
  ]
}
`

const childFixture = `{
  "id": "kind",
  "name": "Kind-Welt",
  "extends": "standard",
  "islands": []
}
`

// fakeBase ist die Basis, die fakeEngine für jede Welt liefert.
const fakeBase = `{"economy": {"_c": "x", "purse": {"startGold": 14, "maxGold": 30}, "chestGold": [3, 6]}, "buildings": {"wall": {"cost": 3}}}`

// fakeEngine antwortet wie k3c-sim world: values liest die Overrides aus der Datei, check lehnt einen Text als
// startGold ab, level liefert drei Abschnitte.
func fakeEngine(root string) Engine {
	return func(_ context.Context, input []byte) (json.RawMessage, error) {
		var in struct {
			Op, World string
			Draft     json.RawMessage
		}
		if err := json.Unmarshal(input, &in); err != nil {
			return nil, err
		}
		switch in.Op {
		case "values":
			raw, err := os.ReadFile(filepath.Join(root, worldsDir, in.World+".json"))
			if err != nil {
				return nil, err
			}
			var f struct {
				Extends   string          `json:"extends"`
				Overrides json.RawMessage `json:"overrides"`
			}
			_ = json.Unmarshal(raw, &f)
			if f.Overrides == nil {
				f.Overrides = json.RawMessage("{}")
			}
			kids := []string{}
			if in.World == standardID {
				kids = []string{"kind"}
			}
			return json.Marshal(map[string]any{"extends": f.Extends, "children": kids, "base": json.RawMessage(fakeBase), "overrides": f.Overrides})
		case "check":
			if strings.Contains(string(in.Draft), `"startGold":"`) {
				return nil, errors.New("economy.purse.startGold: Zahl erwartet")
			}
			return json.RawMessage(`{"warnings":[]}`), nil
		case "level":
			return json.RawMessage(`{"seed":"k3c","biomeId":"forest","widthUnits":60,"chunkWidthUnits":20,"chunks":[{"index":0},{"index":1},{"index":2}],"entities":[],"planned":[{"slot":-7,"kind":"harbor"}],"warnings":null}`), nil
		}
		return nil, errors.New("unbekannte op")
	}
}

func fixture(t *testing.T) Editor {
	t.Helper()
	root := t.TempDir()
	files := map[string]string{
		"standard.json": standardFixture, "kind.json": childFixture, "kaputt.txt": "x", "Ungültig.json": "{}",
		"../biomes/forest.json": `{"depth": 0}`, "../biomes/cave.json": `{"depth": 1}`,
	}
	for name, text := range files {
		path := filepath.Join(root, worldsDir, name)
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(path, []byte(text), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	return Editor{Root: root, Engine: fakeEngine(root)}
}

func TestListStandardFirstAndOnlyWorlds(t *testing.T) {
	got := fixture(t).List()
	want := []Info{{ID: "standard", Name: "Standard"}, {ID: "kind", Name: "Kind-Welt"}}
	if !slices.Equal(got, want) {
		t.Fatalf("List = %v, want %v", got, want)
	}
}

func TestLoadIslandsWithBiomes(t *testing.T) {
	d := fixture(t).Load("standard")
	if d.Error != "" || len(d.Islands) != 1 {
		t.Fatalf("Load = %+v", d)
	}
	isl := d.Islands[0]
	if !slices.Equal(isl.Biomes, []string{"forest", "cave"}) || len(isl.Planned["0"]) == 0 || isl.ChunkCounts == nil {
		t.Fatalf("Insel = %+v", isl)
	}
}

func TestLoadRejectsPathOutsidePattern(t *testing.T) {
	e := fixture(t)
	for _, id := range []string{"../secret", "a/b", ""} {
		if d := e.Load(id); !strings.Contains(d.Error, "ungültige Welt-ID") {
			t.Fatalf("%q: Error = %q", id, d.Error)
		}
		if v := e.Level(context.Background(), id, "island1", 0, "k3c"); !strings.Contains(v.Error, "ungültige Welt-ID") {
			t.Fatalf("%q: Level.Error = %q", id, v.Error)
		}
	}
}

func TestLevelArraysNeverNull(t *testing.T) {
	e := fixture(t)
	v := e.Level(context.Background(), "standard", "island1", 0, "k3c")
	if v.Error != "" || len(v.Chunks) != 3 || len(v.Planned) != 1 || v.Warnings == nil {
		t.Fatalf("Level = %+v", v)
	}
	e.Engine = func(context.Context, []byte) (json.RawMessage, error) { return nil, errors.New("die Insel \"x\" gibt es nicht") }
	bad := e.Level(context.Background(), "standard", "x", 0, "k3c")
	if bad.Error == "" || bad.Chunks == nil || bad.Entities == nil || bad.Planned == nil || bad.Warnings == nil {
		t.Fatalf("Fehlerfall = %+v", bad)
	}
}

func TestValuesShowBaseAndChildren(t *testing.T) {
	v := fixture(t).Values(context.Background(), "standard")
	if v.Error != "" || !slices.Equal(v.Children, []string{"kind"}) {
		t.Fatalf("Values = %+v", v)
	}
	if e := entry(t, v, "economy", "purse", "startGold"); string(e.Base) != "14" || e.Override != nil {
		t.Fatalf("startGold = %+v", e)
	}
	entry(t, v, "economy", "chestGold")
}

func entry(t *testing.T, v ValuesView, file string, path ...string) ValueEntry {
	t.Helper()
	at := slices.IndexFunc(v.Entries, func(e ValueEntry) bool { return e.File == file && slices.Equal(e.Path, path) })
	if at < 0 {
		t.Fatalf("Feld %s %v fehlt", file, path)
	}
	return v.Entries[at]
}

func TestFlattenListsByIDAndIndex(t *testing.T) {
	var got []string
	flattenJSON(json.RawMessage(`{"_c":1,"a":[{"id":"x","v":1},{"v":2}],"b":[{"v":3}],"c":[1,2]}`), nil, func(p []string, v json.RawMessage) {
		got = append(got, strings.Join(p, ".")+"="+string(v))
	})
	want := []string{"a.x.v=1", "b.0.v=3", "c=[1,2]"}
	if !slices.Equal(got, want) {
		t.Fatalf("flatten = %v, want %v", got, want)
	}
}
