package world

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func deleteRoot(t *testing.T) Editor {
	t.Helper()
	root := t.TempDir()
	put := func(rel, s string) {
		p := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(p, []byte(s), 0o600); err != nil {
			t.Fatal(err)
		}
	}
	put("data/paper.json", `{"weltbilder":{"standard":{},"sengoku":{}}}`)
	put("data/worlds/standard.json", `{"id":"standard","name":"Standard"}`)
	put("data/worlds/sengoku.json", `{"id":"sengoku","name":"Sengoku"}`)
	put("data/worlds/eltern.json", `{"id":"eltern","name":"Eltern"}`)
	put("data/worlds/kind.json", `{"id":"kind","name":"Kind","extends":"eltern"}`)
	put("data/worlds/frei.json", `{"id":"frei","name":"Frei"}`)
	put("saves/alt.json", `{"world":"frei"}`)
	put("saves/neu.json", `{"world":"standard"}`)
	return Editor{Root: root}
}

func TestDeleteLoeschtFreieWeltUndWarntVorSpielstaenden(t *testing.T) {
	e := deleteRoot(t)
	warnings, err := e.Delete("frei")
	if err != nil {
		t.Fatal(err)
	}
	if _, err := os.Stat(e.file("frei")); !os.IsNotExist(err) {
		t.Fatalf("Datei noch da: %v", err)
	}
	if len(warnings) != 1 || !strings.Contains(warnings[0], "alt") || strings.Contains(warnings[0], "neu") {
		t.Errorf("Warnung: %v", warnings)
	}
	if _, err := os.Stat(filepath.Join(e.Root, "saves", "alt.json")); err != nil {
		t.Errorf("Spielstand angefasst: %v", err)
	}
}

func TestDeleteLehntFesteWeltenErbenUndUnbekanntesAb(t *testing.T) {
	e := deleteRoot(t)
	cases := map[string]string{
		"standard": "feste Welt", "sengoku": "feste Welt", "eltern": "Erben (kind)", "gibt-es-nicht": "gibt es nicht", "../x": "ungültige Welt-ID",
	}
	for id, want := range cases {
		_, err := e.Delete(id)
		if err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("Delete(%q): %v, will %q", id, err, want)
		}
	}
	for _, id := range []string{"standard", "sengoku", "eltern"} {
		if _, err := os.Stat(e.file(id)); err != nil {
			t.Errorf("%s gelöscht trotz Ablehnung", id)
		}
	}
}

func TestDeleteErbeKannNachDemKindGeloeschtWerden(t *testing.T) {
	e := deleteRoot(t)
	if _, err := e.Delete("kind"); err != nil {
		t.Fatal(err)
	}
	if _, err := e.Delete("eltern"); err != nil {
		t.Fatal(err)
	}
}

func TestDeleteOhnePaperJSONLoeschtNichts(t *testing.T) {
	e := deleteRoot(t)
	if err := os.Remove(filepath.Join(e.Root, "data", "paper.json")); err != nil {
		t.Fatal(err)
	}
	if _, err := e.Delete("frei"); err == nil || !strings.Contains(err.Error(), "paper.json") {
		t.Fatalf("Delete: %v", err)
	}
	if _, err := os.Stat(e.file("frei")); err != nil {
		t.Fatal("Datei trotz Ablehnung gelöscht")
	}
}
