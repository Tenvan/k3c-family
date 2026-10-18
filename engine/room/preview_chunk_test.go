package room

import (
	"reflect"
	"slices"
	"strings"
	"testing"

	"k3c/engine/level"
)

// WZ17.3 (B-485/AC-06 Teil Server): Seed-Inhalt eines Chunks aus dem Vorschau-Level, für die Umwandlung eines
// Seed-Chunks in feste Objekte (geplante-chunks.md § 7).

// chunkStart ist der Anfang des Platzes slot im Level der Welt, gerechnet wie in geplante-chunks.md § 1 (Hub ± 50,
// alle Biome haben 100 Units Hub-Breite) und damit unabhängig vom Code.
func chunkStart(t *testing.T, o PreviewOptions, slot int) (float64, []level.Entity) {
	t.Helper()
	lv := mustPreview(t, o).Level
	start := lv.HubCenterUnits - 50 + float64(slot)*lv.ChunkWidthUnits
	if slot > 0 {
		start = lv.HubCenterUnits + 50 + float64(slot-1)*lv.ChunkWidthUnits
	}
	var in []level.Entity
	for _, e := range lv.Entities {
		if e.X >= start && e.X < start+lv.ChunkWidthUnits {
			in = append(in, e)
		}
	}
	return start, in
}

func mustChunkObjects(t *testing.T, o PreviewOptions, slot int) []level.PlannedObject {
	t.Helper()
	objs, err := PreviewChunkObjects(o, slot)
	if err != nil {
		t.Fatal(err)
	}
	return objs
}

// Der gelieferte Inhalt sind genau die Objekte des Vorschau-Levels in diesem Chunk, `at` ab Chunk-Anfang.
func TestChunkInhaltEntsprichtDenObjektenImLevel(t *testing.T) {
	cases := map[string]struct {
		o    PreviewOptions
		slot int
	}{
		"Seed-Chunk":            {plannedOpts(planB...), -3},
		"feste Objekte":         {plannedOpts(planB...), 2},
		"ungeplant":             {plannedOpts(planB...), 5},
		"Seed-Chunk, Anzahl 20": {countOpts(20, level.PlannedChunk{Slot: 8, Kind: "clearing", Seed: "lichtung"}), 8},
		"Kristallhöhle links":   {PreviewOptions{Seed: "abc", Source: PreviewSource{Biome: "crystal"}}, -2},
	}
	for name, c := range cases {
		start, want := chunkStart(t, c.o, c.slot)
		got := mustChunkObjects(t, c.o, c.slot)
		if len(want) == 0 || len(got) != len(want) {
			t.Errorf("%s: %d Objekte, im Level %d", name, len(got), len(want))
			continue
		}
		for i, obj := range got {
			if obj.Kind != want[i].Kind || start+obj.At != want[i].X {
				t.Errorf("%s: Objekt %d ist %s@%v ab %v, im Level %s@%v", name, i, obj.Kind, obj.At, start, want[i].Kind, want[i].X)
			}
		}
	}
	if got := mustChunkObjects(t, plannedOpts(planB...), 2); !reflect.DeepEqual(got, planB[1].Objects) {
		t.Errorf("Chunk mit Objektliste liefert %v, geplant %v", got, planB[1].Objects)
	}
}

// Mit Seed liefert ein geplanter Chunk ohne Objektliste den Inhalt seines Seeds: gleich für jeden Level-Seed.
func TestChunkInhaltEinesSeedChunksHaengtNurAmSeed(t *testing.T) {
	a := mustChunkObjects(t, plannedOpts(planB...), -3)
	o := plannedOpts(planB...)
	o.Seed = "anderer"
	if b := mustChunkObjects(t, o, -3); !reflect.DeepEqual(a, b) {
		t.Errorf("Seed-Chunk „lichtung“: %v bei Seed abc, %v bei Seed anderer", a, b)
	}
}

// Umwandlung (AC-06): Der Seed-Chunk mit seinem Inhalt als feste Objekte und ohne Seed ergibt dasselbe Level.
func TestUmgewandelterSeedChunkGibtDasselbeLevel(t *testing.T) {
	for _, n := range []int{0, 24} {
		before := countOpts(n, planB...)
		converted := slices.Clone(planB)
		converted[0] = level.PlannedChunk{Slot: -3, Kind: "clearing", Objects: mustChunkObjects(t, before, -3)}
		after := countOpts(n, converted...)
		if jsonOf(t, mustPreview(t, before).Level) != jsonOf(t, mustPreview(t, after).Level) {
			t.Errorf("Anzahl %d: nach der Umwandlung ein anderes Level", n)
		}
	}
}

func TestChunkInhaltLehntUngueltigenPlatzAb(t *testing.T) {
	lv := mustPreview(t, previewOpts()).Level
	hubChunks := 0
	for _, c := range lv.Chunks {
		if c.Kind == "hub" {
			hubChunks++
		}
	}
	outer := (len(lv.Chunks)-hubChunks)/2 - 1 // Hub-Chunks, Ausgang und Rand abgezogen
	for slot, want := range map[int]string{0: "Platz 0", outer + 1: "Ausgang oder Rand", -outer - 1: "Ausgang oder Rand", 100: "außerhalb", -100: "außerhalb"} {
		_, err := PreviewChunkObjects(previewOpts(), slot)
		if err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("Platz %+d: Fehler %v, erwartet „%s“", slot, err, want)
		}
	}
	if _, err := PreviewChunkObjects(previewOpts(), outer); err != nil {
		t.Errorf("äußerster Platz %+d: %v", outer, err)
	}
	if _, err := PreviewChunkObjects(countOpts(18), 1); err == nil || !strings.Contains(err.Error(), "keine Inselgröße") {
		t.Errorf("ungültige Optionen: Fehler %v, erwartet den Fehler der Vorschau", err)
	}
}
