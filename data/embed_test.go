package data

import (
	"encoding/json"
	"io/fs"
	"os"
	"path/filepath"
	"slices"
	"testing"
)

// Erwartete Dateien: fehlt eine, ist das Einbetten oder der Umzug nach data/ kaputt.
var want = []string{
	"biomes/cave.json", "biomes/forest.json", "biomes/mine.json",
	"buildings.json", "difficulty.json", "economy.json", "enemies.json", "hub.json",
	"monarch.json", "sprites.json", "troops.json", "waves.json", "worlds/standard.json",
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

// Mit K3C_DATA_DIR liest der Server die Daten von der Platte (B-507); leer oder ein fehlender Ordner heißt eingebettet.
func TestPlatteStattEinbettung(t *testing.T) {
	dir := t.TempDir()
	if err := os.WriteFile(filepath.Join(dir, "economy.json"), []byte(`{"platte":true}`), 0o644); err != nil {
		t.Fatal(err)
	}
	raw, err := choose(dir).ReadFile("economy.json")
	if err != nil || string(raw) != `{"platte":true}` {
		t.Fatalf("Platte: %q, %v", raw, err)
	}
	for _, d := range []string{"", filepath.Join(dir, "fehlt")} {
		if _, err := choose(d).ReadFile("worlds/standard.json"); err != nil {
			t.Errorf("%q: nicht eingebettet: %v", d, err)
		}
	}
}
