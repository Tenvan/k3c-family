package backend

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// WZ37.1 (B-682): Werte-Overrides im Welten Editor. Basis sind die eingebetteten data/ (Testwurzel ohne Basis-Dateien).

func writeWorld(t *testing.T, a *App, id, content string) string {
	t.Helper()
	path := filepath.Join(a.root, worldsDir, id+".json")
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
	return path
}

func valueOf(v WorldValuesView, file, path string) (WorldValueEntry, bool) {
	for _, e := range v.Entries {
		if e.File == file && strings.Join(e.Path, ".") == path {
			return e, true
		}
	}
	return WorldValueEntry{}, false
}

// B-682: Basiswert aus der Eltern-Kette, Override der Welt, Kind-Welten.
func TestPlanWorldValuesBasisOverrideKinder(t *testing.T) {
	a, _ := worldApp(t)
	writeWorld(t, a, "kind", strings.Replace(strings.Replace(worldFixture, `"id": "standard"`, `"id": "kind", "extends": "standard", "overrides": {"waves": {"perExtraPlayer": 0.37}}`, 1), `"name": "Standard"`, `"name": "Kind"`, 1))
	v := a.PlanWorldValues()
	if v.Error != "" || strings.Join(v.Children, ",") != "kind" {
		t.Fatalf("Standard: Fehler %q, Kinder %v", v.Error, v.Children)
	}
	wall, ok := valueOf(v, "buildings", "wall.cost.wood")
	if !ok || wall.Override != nil || string(wall.Base) == "null" {
		t.Errorf("wall.cost.wood: %+v", wall)
	}
	a.PlanWorldSelect("kind")
	v = a.PlanWorldValues()
	e, ok := valueOf(v, "waves", "perExtraPlayer")
	if v.Extends != "standard" || !ok || string(e.Override) != "0.37" || string(e.Base) == "0.37" {
		t.Errorf("Kind: extends %q, perExtraPlayer %+v", v.Extends, e)
	}
}

// B-682/AC-01: Speichern schreibt nur die Overrides des Entwurfs, gleiche bleiben byte-gleich, leer entfernt den Schlüssel.
func TestPlanWorldSaveOverrides(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	d.Overrides = json.RawMessage(`{"buildings": {"wall": {"cost": {"wood": 7}}}}`)
	res := saveDraft(t, a, d, stand)
	got := readFile(t, path)
	if !strings.Contains(got, `"wood": 7`) || !strings.Contains(got, `"islands"`) || !strings.Contains(got, `"plannedChunks"`) {
		t.Fatalf("overrides nicht geschrieben oder Inseln verloren:\n%s", got)
	}
	if v := a.PlanLoad(); string(v.Overrides["buildings"]) == "" {
		t.Errorf("PlanLoad liest overrides nicht: %+v", v.Overrides)
	}
	res2 := saveDraft(t, a, d, res.Stand)
	if readFile(t, path) != got || res2.Stand != res.Stand {
		t.Error("gleiche Overrides haben die Datei geändert")
	}
	d.Overrides = json.RawMessage(`{}`)
	saveDraft(t, a, d, res.Stand)
	if out := readFile(t, path); out != worldFixture {
		t.Errorf("leere Overrides: Datei nicht wie vorher:\n%s", out)
	}
}

// B-682: Ungültiger Override wird wie im Server abgelehnt, die Meldung nennt Welt, Datei und Pfad; Feld „overrides“.
func TestPlanWorldCheckOverrideUngueltig(t *testing.T) {
	a, path := worldApp(t)
	d, stand := draftOf(t, a)
	d.Overrides = json.RawMessage(`{"buildings": {"wall": {"cost": {"wood": "viel"}}}}`)
	v := a.PlanWorldCheck(d)
	want := "data/worlds/standard.json: overrides › data/buildings.json › wall.cost.wood: Ganzzahl erwartet"
	if v.OK || !strings.Contains(v.Error, want) || v.Field != "overrides" || v.Island != -1 {
		t.Errorf("Prüfung: %+v", v)
	}
	if res := a.PlanWorldSave(d, stand); res.Error == "" || readFile(t, path) != worldFixture {
		t.Errorf("ungültig gespeichert: %+v", res)
	}
}

// B-682: Der Entwurf der Eltern-Welt wird mit allen Welten geprüft, die von ihr erben (die Meldung nennt die Kind-Welt).
func TestPlanWorldCheckPrueftKinder(t *testing.T) {
	a, _ := worldApp(t)
	writeWorld(t, a, "kind", strings.Replace(worldFixture, `"id": "standard"`, `"id": "kind", "extends": "standard", "overrides": {"waves": {"perExtraPlayer": "viel"}}`, 1))
	d, _ := draftOf(t, a)
	d.Overrides = json.RawMessage(`{"waves": {"perExtraPlayer": 0.4}}`)
	v := a.PlanWorldCheck(d)
	if v.OK || !strings.Contains(v.Error, "data/worlds/kind.json: overrides › data/waves.json › perExtraPlayer") {
		t.Errorf("Prüfung: %+v", v)
	}
}
