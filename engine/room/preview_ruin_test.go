package room

import (
	"encoding/json"
	"os"
	"reflect"
	"testing"

	"k3c/engine/level"
)

// WZ14.5 (B-468/AC-05): Die Vorschau liefert die Ruinen eines Chunks (PreviewChunkRuins) in der Form von
// plannedChunks.buildings; Grundlage ist testdata/planned/forest-ruin.json.

func ruinPreviewOpts(t *testing.T) (PreviewOptions, []level.PlannedChunk) {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/planned/forest-ruin.json")
	if err != nil {
		t.Fatal(err)
	}
	var f struct{ Planned []level.PlannedChunk }
	if err := json.Unmarshal(raw, &f); err != nil {
		t.Fatal(err)
	}
	return PreviewOptions{Seed: "ruine-1", Source: PreviewSource{Biome: "forest"}, Planned: f.Planned}, f.Planned
}

func TestVorschauLiefertRuinenJeChunk(t *testing.T) {
	o, plan := ruinPreviewOpts(t)
	for _, c := range plan {
		got, err := PreviewChunkRuins(o, c.Slot)
		if err != nil {
			t.Fatal(err)
		}
		if !reflect.DeepEqual(got, c.Buildings) {
			t.Errorf("Platz %+d: Ruinen %+v, geplant %+v", c.Slot, got, c.Buildings)
		}
	}
	if got, err := PreviewChunkRuins(o, 1); err != nil || len(got) != 0 {
		t.Errorf("Platz +1 ohne Ruine: %+v, %v", got, err)
	}
	o.Planned = nil
	if got, err := PreviewChunkRuins(o, 2); err != nil || len(got) != 0 {
		t.Errorf("ohne Planung: %+v, %v", got, err)
	}
	if _, err := PreviewChunkRuins(o, 0); err == nil {
		t.Error("Platz 0 (Hub) angenommen")
	}
}

// Die Vorschau-Welt trägt die Ruinen als Plätze mit Kennung (Zustand wie snap, Feld sites[].ruin).
func TestVorschauWeltHatRuinenPlaetze(t *testing.T) {
	o, _ := ruinPreviewOpts(t)
	w := mustPreview(t, o)
	n := 0
	for _, s := range w.Sites {
		if s.Ruin {
			n++
		}
	}
	if n != len(w.Level.Ruins) || n != 3 {
		t.Errorf("%d Ruinen-Plätze, Level hat %d, erwartet 3", n, len(w.Level.Ruins))
	}
}
