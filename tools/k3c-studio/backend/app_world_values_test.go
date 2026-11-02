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

// kindWorld ist eine Welt, die von parent erbt, mit den Overrides ov (JSON-Objekt).
func childWorld(id, parent, ov string) string {
	return strings.Replace(strings.Replace(worldFixture, `"id": "standard"`, `"id": "`+id+`", "extends": "`+parent+`", "overrides": `+ov, 1), `"name": "Standard"`, `"name": "`+id+`"`, 1)
}

// B-693: Ein schon ungültiges Kind blockiert das Speichern der Eltern-Welt nicht, die Antwort nennt es als Warnung.
func TestPlanWorldCheckKindSchonUngueltigWarnt(t *testing.T) {
	a, path := worldApp(t)
	writeWorld(t, a, "kind", childWorld("kind", "standard", `{"waves": {"perExtraPlayer": "viel"}}`))
	d, stand := draftOf(t, a)
	d.Overrides = json.RawMessage(`{"waves": {"perExtraPlayer": 0.4}}`)
	v := a.PlanWorldCheck(d)
	if !v.OK || len(v.Warnings) != 1 || !strings.Contains(v.Warnings[0], "Welt kind war schon vorher ungültig") ||
		!strings.Contains(v.Warnings[0], "data/worlds/kind.json: overrides › data/waves.json › perExtraPlayer") {
		t.Fatalf("Prüfung: %+v", v)
	}
	res := a.PlanWorldSave(d, stand)
	if res.Error != "" || len(res.Warnings) != 1 || !strings.Contains(readFile(t, path), `"perExtraPlayer": 0.4`) {
		t.Errorf("Speichern: %+v", res)
	}
}

// B-693: Ein Kind, das nicht lädt, erscheint in Children mit Grund und löst eine Warnung aus; die Prüfung blockiert nicht.
func TestPlanWorldKindOhneParseWarnt(t *testing.T) {
	a, _ := worldApp(t)
	writeWorld(t, a, "kaputt", `{"id": "kaputt", "name": "Kaputt", "extends": "standard", "islands": []}`)
	v := a.PlanWorldValues()
	if strings.Join(v.Children, ",") != "kaputt" || v.ChildErrors["kaputt"] == "" {
		t.Fatalf("Kinder %v, Fehler %v", v.Children, v.ChildErrors)
	}
	d, _ := draftOf(t, a)
	c := a.PlanWorldCheck(d)
	if !c.OK || len(c.Warnings) != 1 || !strings.Contains(c.Warnings[0], "Welt kaputt") {
		t.Errorf("Prüfung: %+v", c)
	}
}

// B-693: Children und Prüfung umfassen auch Enkel (Erben über eine Zwischenwelt).
func TestPlanWorldEnkelInChildrenUndPruefung(t *testing.T) {
	a, _ := worldApp(t)
	writeWorld(t, a, "kind", childWorld("kind", "standard", `{"waves": {"perExtraPlayer": 0.37}}`))
	writeWorld(t, a, "enkel", childWorld("enkel", "kind", `{"waves": {"perExtraPlayer": "viel"}}`))
	if v := a.PlanWorldValues(); strings.Join(v.Children, ",") != "enkel,kind" {
		t.Errorf("Kinder: %v", v.Children)
	}
	d, _ := draftOf(t, a)
	d.Overrides = json.RawMessage(`{"waves": {"perExtraPlayer": 0.4}}`)
	v := a.PlanWorldCheck(d)
	if !v.OK || len(v.Warnings) != 1 || !strings.Contains(v.Warnings[0], "Welt enkel") {
		t.Errorf("Prüfung: %+v", v)
	}
}

// B-693: patchOverrides behält die Zeilenenden der Datei (exakte Ausgabe, LF wie CRLF).
func TestPatchOverridesZeilenenden(t *testing.T) {
	ov := json.RawMessage(`{"waves":{"perExtraPlayer":0.4}}`)
	for _, eol := range []string{"\n", "\r\n"} {
		in := strings.ReplaceAll(worldFixture, "\n", eol)
		got, err := patchOverrides([]byte(in), ov)
		if err != nil {
			t.Fatal(err)
		}
		want := strings.TrimSuffix(in, eol+"}"+eol) + `, "overrides": {` + eol + `    "waves": {` + eol +
			`      "perExtraPlayer": 0.4` + eol + `    }` + eol + `  }` + eol + "}" + eol
		if string(got) != want {
			t.Errorf("eol %q:\n%q\nerwartet\n%q", eol, got, want)
		}
	}
}

func flatPaths(raw string) []string {
	var out []string
	flattenJSON(json.RawMessage(raw), nil, func(p []string, v json.RawMessage) { out = append(out, strings.Join(p, ".")+"="+string(v)) })
	return out
}

// B-693: Gemischte Listen: nur Einträge mit `id` sind ansprechbar (wie listIndex der Engine), die `id` ist kein Feld.
func TestFlattenJSONListen(t *testing.T) {
	cases := []struct{ name, in, want string }{
		{"gemischt", `{"l": [{"id": "a", "x": 1}, {"x": 2}, {"id": "b", "x": 3}]}`, "l.a.x=1 l.b.x=3"},
		{"ohne id", `{"l": [{"x": 1}, {"x": 2}]}`, "l.0.x=1 l.1.x=2"},
		{"id keine Zahl", `{"l": [{"id": 7, "x": 1}, {"id": "b", "x": 2}]}`, "l.b.x=2"},
		{"Werteliste", `{"l": [1, 2]}`, "l=[1,2]"},
	}
	for _, c := range cases {
		if got := strings.Join(flatPaths(c.in), " "); got != c.want {
			t.Errorf("%s: %q, erwartet %q", c.name, got, c.want)
		}
	}
}
