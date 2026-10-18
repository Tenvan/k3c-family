package main

import (
	"reflect"
	"strings"
	"testing"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// WZ17.5 (B-485/AC-04, geplante-chunks.md § 7): Chunk-Anzahl je Stufe im Editor. Alle Tests schreiben nur in Kopien
// (t.TempDir()), nie in data/islands.json des Repos.

const countsFixture = `{
  "notes": "Fixture",
  "islands": [
    { "id": "island1", "name": "Insel 1", "depths": [0, 1, 2, 3, 4], "plannedChunks": {} },
    {"id":"island2","name":"Zwei","depths":[0,1],"plannedChunks":{},"chunkCounts":{"1": 20}}
  ]
}
`

func TestPlanLoadLiefertAnzahlenUndInselgroessen(t *testing.T) {
	app, _ := planApp(t, countsFixture)
	view := app.PlanLoad()
	if view.Error != "" {
		t.Fatal(view.Error)
	}
	if got := view.Islands[1].ChunkCounts; !reflect.DeepEqual(got, map[string]int{"1": 20}) {
		t.Errorf("Anzahlen Insel 2: %v", got)
	}
	if got := view.Islands[0].ChunkCounts; got == nil || len(got) != 0 {
		t.Errorf("Insel 1 ohne Anzahlen braucht eine leere Tabelle, nicht %v", got)
	}
	if !reflect.DeepEqual(view.Sizes, sim.IslandSizes()) || len(view.Sizes) != 6 || view.Sizes[1] != (sim.IslandSize{Name: "klein", ChunkCount: 20}) {
		t.Errorf("Inselgrößen: %+v", view.Sizes)
	}
}

// AC-04: Ein Platz jenseits der Anzahl steht als Fehler am Platz; eine Anzahl außerhalb der Tabelle am Entwurf (Platz 0).
func TestPlanCheckPrueftGegenDieAnzahl(t *testing.T) {
	app, _ := planApp(t, countsFixture)
	if v := app.PlanCheck("island1", 0, portalPlan, 20); !v.OK {
		t.Fatalf("Portal +6 bei 20 Chunks abgelehnt: %s", v.Error)
	}
	far := []level.PlannedChunk{{Slot: 11, Kind: "clearing"}}
	if v := app.PlanCheck("island1", 0, far, 20); v.OK || v.Slot != 11 || !strings.Contains(v.Error, "Platz +11") {
		t.Errorf("Platz +11 bei 20 Chunks: %+v", v)
	}
	if v := app.PlanCheck("island1", 0, far, 48); !v.OK {
		t.Errorf("Platz +11 bei 48 Chunks abgelehnt: %s", v.Error)
	}
	v := app.PlanCheck("island1", 0, nil, 18)
	if v.OK || v.Slot != 0 || !strings.Contains(v.Error, "keine Inselgröße") || !strings.Contains(v.Error, "16, 20, 24, 32, 40, 48") {
		t.Errorf("Anzahl 18: %+v", v)
	}
}

// AC-04: Speichern ersetzt nur den Wert von chunkCounts der Insel (fehlt das Feld, wird es angehängt); „Biom-Länge“
// (0) entfernt den Eintrag der Tiefe; alle übrigen Bytes bleiben.
func TestPlanSaveSchreibtNurDieAnzahlDerStufe(t *testing.T) {
	app, path := planApp(t, countsFixture)
	before := readFile(t, path)
	s := app.PlanSave("island1", 0, nil, 20, app.PlanLoad().Stand)
	if s.Error != "" || s.Hint != planRegHint {
		t.Fatalf("Speichern: %+v", s)
	}
	want := strings.Replace(before, `"plannedChunks": {} },`, `"plannedChunks": {}, "chunkCounts": {"0": 20} },`, 1)
	if got := readFile(t, path); got != want {
		t.Fatalf("Datei:\n%s\nerwartet:\n%s", got, want)
	}
	if s = app.PlanSave("island1", 2, nil, 32, s.Stand); s.Error != "" {
		t.Fatal(s.Error)
	}
	want = strings.Replace(want, `{"0": 20}`, `{"0": 20, "2": 32}`, 1)
	if got := readFile(t, path); got != want {
		t.Fatalf("zweite Tiefe:\n%s\nerwartet:\n%s", got, want)
	}
	if s = app.PlanSave("island1", 0, nil, 0, s.Stand); s.Error != "" {
		t.Fatal(s.Error)
	}
	want = strings.Replace(want, `{"0": 20, "2": 32}`, `{"2": 32}`, 1)
	if got := readFile(t, path); got != want {
		t.Fatalf("Biom-Länge:\n%s\nerwartet:\n%s", got, want)
	}
	if got := app.PlanLoad().Islands[0].ChunkCounts; !reflect.DeepEqual(got, map[string]int{"2": 32}) {
		t.Errorf("PlanLoad nach dem Speichern: %v", got)
	}
}

func TestPlanSaveAnzahlOhneAenderungIstByteGleich(t *testing.T) {
	app, path := planApp(t, countsFixture)
	before := readFile(t, path)
	stand := app.PlanLoad().Stand
	if s := app.PlanSave("island2", 1, nil, 20, stand); s.Error != "" || s.Stand != stand || readFile(t, path) != before {
		t.Errorf("gleiche Anzahl: %+v", s)
	}
	if s := app.PlanSave("island1", 3, nil, 0, stand); s.Error != "" || readFile(t, path) != before {
		t.Errorf("Biom-Länge ohne Eintrag: %+v", s)
	}
}

// AC-04: Ungültiges wird nicht geschrieben, ein veralteter Stand auch dann nicht, wenn sich nur die Anzahl ändert.
func TestPlanSaveAnzahlNurGueltigUndAufDemStand(t *testing.T) {
	app, path := planApp(t, countsFixture)
	before := readFile(t, path)
	stand := app.PlanLoad().Stand
	if s := app.PlanSave("island1", 0, nil, 18, stand); s.Error == "" || s.Stale || readFile(t, path) != before {
		t.Errorf("Anzahl 18: %+v", s)
	}
	far := []level.PlannedChunk{{Slot: 11, Kind: "clearing"}}
	if s := app.PlanSave("island1", 0, far, 20, stand); s.Error == "" || readFile(t, path) != before {
		t.Errorf("Platz jenseits der Anzahl: %+v", s)
	}
	if s := app.PlanSave("island1", 0, nil, 24, "alt"); !s.Stale || readFile(t, path) != before {
		t.Errorf("veralteter Stand: %+v", s)
	}
}

// AC-04: Chunk- und Live-Vorschau zeigen die Länge des Entwurfs aus derselben Quelle; ohne Entwurf gilt die gespeicherte
// Anzahl, der Entwurf 0 (Biom-Länge) überdeckt sie.
func TestVorschauenZeigenDieAnzahlDesEntwurfs(t *testing.T) {
	app, _ := planApp(t, countsFixture)
	count := 20
	src := WorldSource{Island: "island1", Stage: 0, ChunkCount: &count}
	chunks := app.WorldLevel("abc", src)
	live := app.WorldPreview(WorldPreviewArgs{Seed: "abc", Source: src})
	if chunks.Error != "" || live.Error != "" {
		t.Fatalf("Fehler: %q / %q", chunks.Error, live.Error)
	}
	if len(chunks.Chunks) != 20 || !reflect.DeepEqual(chunks.Layout, live.Level) || len(chunks.Warnings) != 0 {
		t.Errorf("Chunk-Vorschau %d Chunks, Warnungen %v, gleich zur Live-Vorschau: %v", len(chunks.Chunks), chunks.Warnings, reflect.DeepEqual(chunks.Layout, live.Level))
	}
	if saved := app.WorldLevel("abc", WorldSource{Island: "island2", Stage: 1}); len(saved.Chunks) != 20 {
		t.Errorf("gespeicherte Anzahl: %d Chunks, %q", len(saved.Chunks), saved.Error)
	}
	zero := 0
	cave, _ := level.LoadBiome("cave")
	plain, _ := level.Generate(cave, "abc")
	if biome := app.WorldLevel("abc", WorldSource{Island: "island2", Stage: 1, ChunkCount: &zero}); !reflect.DeepEqual(biome.Layout, plain) {
		t.Errorf("Entwurf Biom-Länge: %d Chunks statt %d", len(biome.Chunks), len(plain.Chunks))
	}
	bad := 18
	if v := app.WorldLevel("abc", WorldSource{Island: "island1", ChunkCount: &bad}); v.Error == "" {
		t.Error("Anzahl 18 muss abgelehnt werden")
	}
	if v := app.WorldPreview(WorldPreviewArgs{Seed: "abc", Source: WorldSource{Island: "island1", ChunkCount: &bad}}); v.Error == "" {
		t.Error("Live-Vorschau: Anzahl 18 muss abgelehnt werden")
	}
}

// Planung und Anzahl in einem Schreibvorgang: fehlen beide Felder, werden beide angehängt, sonst nur ihre Werte ersetzt.
func TestPlanSaveSchreibtPlanungUndAnzahlZusammen(t *testing.T) {
	const bare = "{\n  \"islands\": [\n    { \"id\": \"island1\", \"depths\": [0, 1, 2, 3, 4] }\n  ]\n}\n"
	app, path := planApp(t, bare)
	s := app.PlanSave("island1", 0, portalPlan, 20, app.PlanLoad().Stand)
	if s.Error != "" {
		t.Fatal(s.Error)
	}
	want := strings.Replace(bare, `[0, 1, 2, 3, 4] }`, `[0, 1, 2, 3, 4], "plannedChunks": {"0": [{"slot":6,"kind":"portal"}]}, "chunkCounts": {"0": 20} }`, 1)
	if got := readFile(t, path); got != want {
		t.Fatalf("Datei:\n%s\nerwartet:\n%s", got, want)
	}
	if s = app.PlanSave("island1", 0, nil, 32, s.Stand); s.Error != "" {
		t.Fatal(s.Error)
	}
	want = strings.Replace(want, `{"0": [{"slot":6,"kind":"portal"}]}, "chunkCounts": {"0": 20}`, `{}, "chunkCounts": {"0": 32}`, 1)
	if got := readFile(t, path); got != want {
		t.Errorf("zweites Speichern:\n%s\nerwartet:\n%s", got, want)
	}
}
