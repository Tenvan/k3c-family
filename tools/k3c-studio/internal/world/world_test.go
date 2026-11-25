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

	"k3c/tools/k3c-studio/internal/exeengine"
)

// Fixture: eine Standard-Welt mit einer Insel, eine Kind-Welt und zwei Biome. Die Engine-Seite ist die echte Engine
// (exeengine: Spiel-EXE der Repo-Wurzel mit den Fixture-Daten; ohne EXE wird der Test übersprungen).
const standardFixture = `{
  "id": "standard",
  "name": "Standard",
  "notes": "Fixture",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "depthScaling": { "hp": 1.5, "damage": 1.3, "speed": 1.1 }, "plannedChunks": { "0": [ { "slot": -7, "kind": "harbor" } ] } }
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

func fixture(t *testing.T) Editor {
	t.Helper()
	root := t.TempDir()
	// Echte Engine (B-924): sie liest data/ des Checkouts; Kopie der Daten ohne die Welten der Repo-Wurzel.
	if err := os.CopyFS(filepath.Join(root, "data"), os.DirFS(filepath.Join("..", "..", "..", "..", "data"))); err != nil {
		t.Fatal(err)
	}
	if err := os.RemoveAll(filepath.Join(root, worldsDir)); err != nil {
		t.Fatal(err)
	}
	files := map[string]string{
		"standard.json": standardFixture, "kind.json": childFixture, "kaputt.txt": "x", "Ungültig.json": "{}",

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
	return Editor{Root: root, Engine: exeengine.For(t, "world", root)}
}

func TestListStandardFirstAndOnlyWorlds(t *testing.T) {
	got := fixture(t).List()
	want := []Info{
		{ID: "standard", Name: "Standard", Kind: "new"},
		{ID: "sengoku", Name: "sengoku", Missing: true},
		{ID: "nil", Name: "nil", Missing: true},
		{ID: "britannien", Name: "britannien", Missing: true},
		{ID: "kind", Name: "Kind-Welt", Kind: "spinoff"},
	}
	if !slices.Equal(got, want) {
		t.Fatalf("List = %v, want %v", got, want)
	}
}

// AC-01 (B-1168): feste Welten ohne Datei fehlen nicht; eine Datei einer festen Welt zählt als umgesetzt, weitere
// Dateien (z. B. Nordmeer) stehen dahinter; ohne paper.json bleibt es bei den Dateien.
func TestListFixedWorldsAndFiles(t *testing.T) {
	e := fixture(t)
	if err := os.WriteFile(filepath.Join(e.Root, worldsDir, "rom.json"), []byte(`{"id":"rom","name":"Nordmeer","islands":[]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(e.Root, worldsDir, "nil.json"), []byte(`{"id":"nil","name":"Nil","islands":[]}`), 0o644); err != nil {
		t.Fatal(err)
	}
	var ids []string
	for _, w := range e.List() {
		ids = append(ids, w.ID+map[bool]string{true: "!", false: ""}[w.Missing])
	}
	if want := []string{"standard", "sengoku!", "nil", "britannien!", "kind", "rom"}; !slices.Equal(ids, want) {
		t.Fatalf("List = %v, want %v", ids, want)
	}
	if err := os.Remove(filepath.Join(e.Root, "data", "paper.json")); err != nil {
		t.Fatal(err)
	}
	if got := e.List(); len(got) != 4 || got[0].ID != "standard" || got[0].Missing {
		t.Fatalf("ohne paper.json: %v", got)
	}
}

func TestLoadIslandsWithBiomes(t *testing.T) {
	d := fixture(t).Load("standard")
	if d.Error != "" || len(d.Islands) != 1 {
		t.Fatalf("Load = %+v", d)
	}
	isl := d.Islands[0]
	if !slices.Equal(isl.Biomes, []string{"forest", "cave", "mine", "ironhold", "crystal"}) || len(isl.Planned["0"]) == 0 || isl.ChunkCounts == nil {
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
	if v.Error != "" || len(v.Chunks) < 3 || len(v.Planned) != 1 || v.Warnings == nil {
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
