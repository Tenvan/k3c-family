package data

import (
	"encoding/json"
	"io/fs"
	"slices"
	"testing"
)

// Erwartete Dateien: fehlt eine, ist das Einbetten oder der Umzug nach data/ kaputt.
var want = []string{
	"biomes/cave.json", "biomes/forest.json", "biomes/mine.json",
	"buildings.json", "difficulty.json", "economy.json", "enemies.json", "hub.json",
	"monarch.json", "sprites.json", "troops.json", "waves.json",
}

func TestEmbeddedFilesAreValidJSON(t *testing.T) {
	var got []string
	err := fs.WalkDir(Files, ".", func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		raw, err := Files.ReadFile(path)
		if err != nil {
			return err
		}
		if !json.Valid(raw) {
			t.Errorf("%s ist kein gültiges JSON", path)
		}
		got = append(got, path)
		return nil
	})
	if err != nil {
		t.Fatal(err)
	}
	for _, name := range want {
		if !slices.Contains(got, name) {
			t.Errorf("%s fehlt in data.Files (eingebettet: %v)", name, got)
		}
	}
}
