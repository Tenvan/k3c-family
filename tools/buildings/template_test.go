package main

import (
	"image"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// TestVorlagePflicht: ohne Vorlage, mit leerem Motiv oder leeren Teilen, mit unbekanntem Feld bricht run ab und nennt das Feld (GEB8 AC-01).
func TestVorlagePflicht(t *testing.T) {
	const rest = `"kacheln":{"b":1,"h":1},"bausteine":[{"art":"krone","rechteck":[0,0,1,1],"farbe":"dell"}]}`
	cases := map[string]struct{ vorlage, want string }{
		"fehlt":            {``, "vorlage"},
		"motiv leer":       {`"vorlage":{"motiv":"","teile":["Kegel"]},`, "motiv"},
		"teile leer":       {`"vorlage":{"motiv":"Tanne","teile":[]},`, "teile"},
		"unbekanntes Feld": {`"vorlage":{"motiv":"Tanne","teile":["Kegel"],"motv":"x"},`, "motv"},
	}
	for name, c := range cases {
		dir := t.TempDir()
		if err := os.WriteFile(filepath.Join(dir, "tanne.json"), []byte(`{"id":"tanne",`+c.vorlage+rest), 0o644); err != nil {
			t.Fatal(err)
		}
		_, err := run(dir, "")
		if err == nil || !strings.Contains(err.Error(), c.want) || !strings.Contains(err.Error(), "tanne") {
			t.Errorf("%s: %v", name, err)
		}
	}
}

// TestEchteVorlagen: jede Gebäude-JSON im Repo hat eine gültige Vorlage, und ihre Materialien sind Zeilen aus § 3 der Regeldatei.
func TestEchteVorlagen(t *testing.T) {
	regeln, err := os.ReadFile(filepath.Join("..", "..", "docs", "standards", "gebaeude-grafik.md"))
	if err != nil {
		t.Fatal(err)
	}
	files, err := filepath.Glob(filepath.Join("..", "..", "public", "grafik", "gebaeude", "*.json"))
	if err != nil || len(files) == 0 {
		t.Fatalf("keine Gebäude: %v", err)
	}
	for _, f := range files {
		if filepath.Base(f) == "index.json" {
			continue
		}
		e, err := build(f, "", map[string]*image.Paletted{})
		if err != nil {
			t.Fatal(err)
		}
		for _, m := range e.Vorlage.Materialien {
			if !strings.Contains(string(regeln), "\n| "+m+" |") {
				t.Errorf("%s: Material %q steht nicht in § 3", e.ID, m)
			}
		}
	}
}
