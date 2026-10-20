package backend

import (
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"

	"k3c/engine/level"
)

// WZ8.5 (B-461/AC-09, AC-11, AC-12): Editor für geplante Chunks. Alle Tests schreiben nur in Kopien (t.TempDir()),
// nie in data/worlds/ des Repos.

const islandsFixture = `{
  "notes": "Fixture",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "depthScaling": { "hp": 1.5 }, "plannedChunks": {} },
    {"id":"island2","name":"Zwei","depths":[0,1],"plannedChunks":{"1":[{"slot":3,"kind":"cavern"}]}}
  ]
}
`

func planApp(t *testing.T, content string) (*App, string) {
	t.Helper()
	root := t.TempDir()
	path := filepath.Join(root, islandsPath)
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
	return &App{root: root}, path
}

func readFile(t *testing.T, path string) string {
	t.Helper()
	b, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	return string(b)
}

var portalPlan = []level.PlannedChunk{{Slot: 6, Kind: "portal"}}

func TestPlanLoadListetInselnMitBiomen(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	view := app.PlanLoad()
	if view.Error != "" || len(view.Islands) != 2 {
		t.Fatalf("Fehler %q, Inseln %d", view.Error, len(view.Islands))
	}
	i1 := view.Islands[0]
	if !reflect.DeepEqual(i1.Biomes, []string{"forest", "cave", "mine", "ironhold", "crystal"}) || len(i1.Planned) != 0 {
		t.Errorf("Insel 1: Biome %v, Planung %v", i1.Biomes, i1.Planned)
	}
	if got := view.Islands[1].Planned["1"]; len(got) != 1 || got[0].Slot != 3 {
		t.Errorf("Insel 2: Planung %v", view.Islands[1].Planned)
	}
	if view.Stand != planStand([]byte(readFile(t, path))) {
		t.Error("Stand ist nicht der Hash der Datei")
	}
	if bad, _ := planApp(t, "{"); bad.PlanLoad().Error == "" {
		t.Error("ungültige Datei muss einen Fehler melden")
	}
}

func TestPlanCheckNenntDenPlatz(t *testing.T) {
	app, _ := planApp(t, islandsFixture)
	if v := app.PlanCheck("island1", 0, portalPlan, 0); !v.OK {
		t.Fatalf("gültige Planung abgelehnt: %s", v.Error)
	}
	v := app.PlanCheck("island1", 0, []level.PlannedChunk{{Slot: 5, Kind: "volcano"}}, 0)
	if v.OK || v.Slot != 5 || !strings.Contains(v.Error, "Platz +5") {
		t.Errorf("Prüfung: %+v", v)
	}
	if v := app.PlanCheck("island1", 9, portalPlan, 0); v.OK {
		t.Error("Tiefe außerhalb der Insel muss abgelehnt werden")
	}
	if v := app.PlanCheck("nirgends", 0, portalPlan, 0); v.OK {
		t.Error("unbekannte Insel muss abgelehnt werden")
	}
}

// AC-12: Speichern ersetzt nur den Wert von plannedChunks der Insel; alle übrigen Bytes bleiben.
func TestPlanSaveIstFormatStabilUndAtomar(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	before := readFile(t, path)
	stand := app.PlanLoad().Stand
	saved := app.PlanSave("island1", 0, portalPlan, 0, stand)
	if saved.Error != "" || saved.Hint != planRegHint || saved.Stand == stand {
		t.Fatalf("Speichern: %+v", saved)
	}
	after := readFile(t, path)
	want := strings.Replace(before, `"plannedChunks": {} }`, `"plannedChunks": {"0": [{"slot":6,"kind":"portal"}]} }`, 1)
	if after != want {
		t.Errorf("Datei nach dem Speichern:\n%s\nerwartet:\n%s", after, want)
	}
	if saved.Stand != planStand([]byte(after)) {
		t.Error("neuer Stand passt nicht zur Datei")
	}
	files, _ := os.ReadDir(filepath.Dir(path))
	if len(files) != 1 {
		t.Errorf("Temp-Dateien übrig: %d Einträge in data/", len(files))
	}
	if !strings.Contains(planRegHint, "REG-Änderung: Commit mit Begründung") || !strings.Contains(planRegHint, "task golden:update") {
		t.Errorf("REG-Hinweis: %q", planRegHint)
	}
}

func TestPlanSaveOhneAenderungIstByteGleich(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	before := readFile(t, path)
	view := app.PlanLoad()
	same := view.Islands[1].Planned["1"]
	saved := app.PlanSave("island2", 1, same, 0, view.Stand)
	if saved.Error != "" || saved.Stand != view.Stand || readFile(t, path) != before {
		t.Fatalf("Speichern ohne Änderung: %+v, Datei geändert: %v", saved, readFile(t, path) != before)
	}
	// Planung leer lassen, wo keine war: ebenfalls byte-gleich
	if s := app.PlanSave("island1", 0, nil, 0, view.Stand); s.Error != "" || readFile(t, path) != before {
		t.Errorf("leere Planung ohne vorherige: %+v", s)
	}
}

func TestPlanSaveBeruehrtNurDenBetroffenenEintrag(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	stand := app.PlanLoad().Stand
	next := []level.PlannedChunk{{Slot: -2, Kind: "tunnel", Seed: "x"}}
	if s := app.PlanSave("island2", 1, next, 0, stand); s.Error != "" {
		t.Fatal(s.Error)
	}
	after := readFile(t, path)
	if !strings.HasPrefix(after, `{
  "notes": "Fixture",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "depthScaling": { "hp": 1.5 }, "plannedChunks": {} },
    {"id":"island2","name":"Zwei","depths":[0,1],"plannedChunks":`) || !strings.HasSuffix(after, "}\n  ]\n}\n") {
		t.Errorf("Datei außerhalb des Werts verändert:\n%s", after)
	}
	view := app.PlanLoad()
	if got := view.Islands[1].Planned["1"]; len(got) != 1 || got[0].Slot != -2 || got[0].Seed != "x" || len(view.Islands[0].Planned) != 0 {
		t.Errorf("Planung nach dem Speichern: %+v", view.Islands)
	}
	// Planung entfernen bringt die Insel auf {} zurück
	if s := app.PlanSave("island2", 1, nil, 0, view.Stand); s.Error != "" || !strings.Contains(readFile(t, path), `"plannedChunks":{}`) {
		t.Errorf("Planung entfernen: %+v\n%s", s, readFile(t, path))
	}
}

func TestPlanSaveLegtFehlendesFeldAn(t *testing.T) {
	const bare = "{\n  \"islands\": [\n    { \"id\": \"island1\", \"depths\": [0, 1, 2, 3, 4] }\n  ]\n}\n"
	app, path := planApp(t, bare)
	if s := app.PlanSave("island1", 0, portalPlan, 0, app.PlanLoad().Stand); s.Error != "" {
		t.Fatal(s.Error)
	}
	want := "{\n  \"islands\": [\n    { \"id\": \"island1\", \"depths\": [0, 1, 2, 3, 4], \"plannedChunks\": {\"0\": [{\"slot\":6,\"kind\":\"portal\"}]} }\n  ]\n}\n"
	if got := readFile(t, path); got != want {
		t.Errorf("Datei:\n%s\nerwartet:\n%s", got, want)
	}
}

func TestPlanSaveSchreibtNurGueltiges(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	before := readFile(t, path)
	s := app.PlanSave("island1", 0, []level.PlannedChunk{{Slot: 0, Kind: "forest"}}, 0, app.PlanLoad().Stand)
	if s.Error == "" || s.Stale || readFile(t, path) != before {
		t.Errorf("ungültige Planung: %+v, Datei geändert: %v", s, readFile(t, path) != before)
	}
}

// AC-12: Wurde die Datei seit dem Laden geändert, wird nicht überschrieben; die Meldung nennt den Grund.
func TestPlanSaveLehntVeralteteDateiAb(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	stand := app.PlanLoad().Stand
	external := strings.Replace(islandsFixture, `"notes": "Fixture"`, `"notes": "extern geändert"`, 1)
	if err := os.WriteFile(path, []byte(external), 0o644); err != nil {
		t.Fatal(err)
	}
	s := app.PlanSave("island1", 0, portalPlan, 0, stand)
	if !s.Stale || s.Error != "data/worlds/standard.json geändert, bitte neu laden" || readFile(t, path) != external {
		t.Errorf("veralteter Stand: %+v, Datei: %q", s, readFile(t, path))
	}
}

// B-486: Nach einem erfolgreichen Speichern (neuer Stand) lehnt auch ein zweites Speichern eine von außen geänderte Datei
// ab und meldet es; die Datei bleibt, wie die externe Änderung sie hinterlassen hat. Danach klappt Speichern mit dem
// frisch geladenen Stand wieder.
func TestPlanSaveNachSpeichernLehntExterneAenderungAb(t *testing.T) {
	app, path := planApp(t, islandsFixture)
	first := app.PlanSave("island1", 0, portalPlan, 0, app.PlanLoad().Stand)
	if first.Error != "" || first.Stale || first.Stand == "" {
		t.Fatalf("erstes Speichern: %+v", first)
	}
	external := strings.Replace(readFile(t, path), `"notes": "Fixture"`, `"notes": "extern geändert"`, 1)
	if err := os.WriteFile(path, []byte(external), 0o644); err != nil {
		t.Fatal(err)
	}
	second := app.PlanSave("island1", 1, []level.PlannedChunk{{Slot: 4, Kind: "cavern"}}, 0, first.Stand)
	if !second.Stale || second.Error != staleError("standard") || readFile(t, path) != external {
		t.Fatalf("zweites Speichern: %+v, Datei unverändert: %v", second, readFile(t, path) == external)
	}
	reloaded := app.PlanLoad()
	if third := app.PlanSave("island1", 1, []level.PlannedChunk{{Slot: 4, Kind: "cavern"}}, 0, reloaded.Stand); third.Error != "" || third.Stale {
		t.Errorf("nach Neuladen: %+v", third)
	}
}

// AC-11: Chunk- und Live-Vorschau lesen dieselbe Quelle: gleicher Entwurf, gleiche Insel und Stufe, gleicher Seed
// ergeben dieselben Chunks und Objekte; der Entwurf (auch ungespeichert) steht im Level.
func TestChunkUndLiveVorschauLesenDieselbeQuelle(t *testing.T) {
	app, _ := planApp(t, islandsFixture)
	draft := []level.PlannedChunk{{Slot: 6, Kind: "portal"}, {Slot: -3, Kind: "clearing", Seed: "lichtung"}}
	src := WorldSource{Island: "island1", Stage: 0, Planned: &draft}
	chunks := app.WorldLevel("abc", src)
	live := app.WorldPreview(WorldPreviewArgs{Seed: "abc", Source: src})
	if chunks.Error != "" || live.Error != "" {
		t.Fatalf("Fehler: %q / %q", chunks.Error, live.Error)
	}
	if !reflect.DeepEqual(chunks.Layout, live.Level) {
		t.Error("Chunk-Vorschau und Live-Vorschau zeigen für denselben Entwurf verschiedene Level")
	}
	var found bool
	for _, e := range chunks.Entities {
		found = found || (e.Kind == "portal" && e.X-chunks.HubCenterUnits == 325)
	}
	if !found {
		t.Error("das geplante Portal bei Hub +325 fehlt im Level")
	}
	if v := app.WorldLevel("abc", WorldSource{Island: "island1", Stage: 9}); v.Error == "" {
		t.Error("Stufe außerhalb der Insel muss abgelehnt werden")
	}
}

func TestWorldQuelleOhneEntwurfNimmtDieGespeichertePlanung(t *testing.T) {
	app, _ := planApp(t, islandsFixture)
	saved := app.WorldLevel("abc", WorldSource{Island: "island2", Stage: 1})
	biome, _ := level.LoadBiome("cave")
	want, _ := level.GenerateWith(biome, "abc", level.Options{Planned: []level.PlannedChunk{{Slot: 3, Kind: "cavern"}}})
	plain, _ := level.Generate(biome, "abc")
	if saved.Error != "" || !reflect.DeepEqual(saved.Layout, want) || reflect.DeepEqual(saved.Layout, plain) {
		t.Errorf("Quelle Insel 2 Stufe 1: Fehler %q, gespeicherte Planung wirkt nicht", saved.Error)
	}
}
