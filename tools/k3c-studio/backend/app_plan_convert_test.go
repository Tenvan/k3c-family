package backend

import (
	"reflect"
	"strings"
	"testing"

	"k3c/engine/level"
)

// WZ17.5 (B-485/AC-06, geplante-chunks.md § 7): Umwandlung eines Seed-Chunks in feste Objekte ändert das Bild nicht.

// converted ersetzt im Entwurf den Chunk am Platz slot durch seine festen Objekte ohne Seed (wie das Frontend).
func converted(draft []level.PlannedChunk, slot int, objects []level.PlannedObject) []level.PlannedChunk {
	out := make([]level.PlannedChunk, len(draft))
	for i, c := range draft {
		if c.Slot == slot {
			c = level.PlannedChunk{Slot: c.Slot, Kind: c.Kind, Objects: objects}
		}
		out[i] = c
	}
	return out
}

// AC-06: gleiche Objekte und Orte vorher und nachher, in Chunk- und Live-Vorschau, mit und ohne Chunk-Anzahl.
func TestUmwandlungAendertDasBildNicht(t *testing.T) {
	app, _ := planApp(t, islandsFixture)
	draft := []level.PlannedChunk{{Slot: 6, Kind: "portal"}, {Slot: -3, Kind: "clearing", Seed: "lichtung"}, {Slot: 2, Kind: "forest", Seed: "wald"}}
	for _, count := range []int{0, 24, 160} {
		for _, seed := range []string{"abc", "k3c", "x7"} {
			n := count
			src := WorldSource{Island: "island1", Stage: 0, Planned: &draft, ChunkCount: &n}
			before := app.WorldLevel(seed, src)
			got := app.PlanChunkObjects(seed, src, -3)
			if before.Error != "" || got.Error != "" || len(got.Objects) == 0 {
				t.Fatalf("Anzahl %d, Seed %s: Fehler %q / %q, %d Objekte", count, seed, before.Error, got.Error, len(got.Objects))
			}
			next := converted(draft, -3, got.Objects)
			if v := app.PlanCheck("island1", 0, next, count); !v.OK {
				t.Fatalf("umgewandelter Entwurf ungültig: %s", v.Error)
			}
			after := WorldSource{Island: "island1", Stage: 0, Planned: &next, ChunkCount: &n}
			if lv := app.WorldLevel(seed, after); !reflect.DeepEqual(lv.Layout, before.Layout) {
				t.Errorf("Anzahl %d, Seed %s: Chunk-Vorschau nach der Umwandlung anders", count, seed)
			}
			liveBefore := app.WorldPreview(WorldPreviewArgs{Seed: seed, Source: src})
			liveAfter := app.WorldPreview(WorldPreviewArgs{Seed: seed, Source: after})
			if liveBefore.Error != "" || !reflect.DeepEqual(liveBefore.PreviewResult, liveAfter.PreviewResult) {
				t.Errorf("Anzahl %d, Seed %s: Live-Vorschau nach der Umwandlung anders (%q)", count, seed, liveBefore.Error)
			}
		}
	}
}

// Ohne Planung am Platz, ohne Chunk-Seed oder mit festen Objekten gibt es keine Umwandlung; der Fehler nennt den Grund.
func TestUmwandlungBrauchtEinenSeedChunk(t *testing.T) {
	app, _ := planApp(t, islandsFixture)
	draft := []level.PlannedChunk{{Slot: 6, Kind: "portal"}, {Slot: -3, Kind: "clearing", Objects: []level.PlannedObject{{Kind: "tree", At: 10}}}}
	src := WorldSource{Island: "island1", Stage: 0, Planned: &draft}
	cases := map[int]string{4: "nicht geplant", 6: "keinen Chunk-Seed", -3: "keinen Chunk-Seed"}
	for slot, want := range cases {
		if v := app.PlanChunkObjects("abc", src, slot); v.Error == "" || !strings.Contains(v.Error, want) || len(v.Objects) != 0 {
			t.Errorf("Platz %+d: %+v, erwartet %q", slot, v, want)
		}
	}
	draft[1].Seed = "lichtung"
	if v := app.PlanChunkObjects("abc", src, -3); !strings.Contains(v.Error, "feste Objekte") {
		t.Errorf("Seed und feste Objekte: %+v", v)
	}
	if v := app.PlanChunkObjects("", src, -3); v.Error == "" {
		t.Error("leerer Seed muss abgelehnt werden")
	}
}
