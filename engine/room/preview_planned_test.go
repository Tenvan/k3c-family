package room

import (
	"slices"
	"strings"
	"testing"

	"k3c/engine/level"
)

// WZ8.4 (B-461/AC-08): Die Vorschau nimmt die Planung als Option „geplante Chunks“, ohne globalen Zustand.

func plannedOpts(plan ...level.PlannedChunk) PreviewOptions {
	o := previewOpts()
	o.Planned = plan
	return o
}

var (
	planA = []level.PlannedChunk{{Slot: 6, Kind: "portal"}}
	planB = []level.PlannedChunk{{Slot: -3, Kind: "clearing", Seed: "lichtung"}, {Slot: 2, Kind: "rocks", Objects: []level.PlannedObject{{Kind: "rock", At: 10}}}}
)

func portalsOf(t *testing.T, o PreviewOptions) []float64 {
	t.Helper()
	w := mustPreview(t, o)
	out := slices.Clone(w.Portals)
	slices.Sort(out)
	for i := range out {
		out[i] -= w.HubX
	}
	return out
}

func TestVorschauMitPlanungIstDeterministisch(t *testing.T) {
	a, b := mustPreview(t, plannedOpts(planB...)), mustPreview(t, plannedOpts(planB...))
	if jsonOf(t, a) != jsonOf(t, b) {
		t.Error("gleiche Planung, verschiedene Welt")
	}
}

func TestVorschauOhnePlanungBleibtZufall(t *testing.T) {
	biome, _ := level.LoadBiome("forest")
	want, err := level.Generate(biome, "abc")
	if err != nil {
		t.Fatal(err)
	}
	for name, o := range map[string]PreviewOptions{"nil": plannedOpts(), "leer": plannedOpts([]level.PlannedChunk{}...)} {
		if got := mustPreview(t, o).Level; !slices.Equal(got.Entities, want.Entities) {
			t.Errorf("%s: das Level weicht von level.Generate ab", name)
		}
	}
}

func TestVorschauLehntUngueltigePlanungAb(t *testing.T) {
	_, err := BuildPreview(plannedOpts(level.PlannedChunk{Slot: 0, Kind: "forest"}))
	if err == nil || !strings.Contains(err.Error(), "Platz 0") {
		t.Fatalf("Fehler %v, erwartet mit dem Grund der Prüfung (Platz 0)", err)
	}
}

func TestVorschauAufeinanderfolgendeAufrufeMitVerschiedenerPlanung(t *testing.T) {
	first := portalsOf(t, plannedOpts(planA...))
	other := portalsOf(t, plannedOpts(planB...))
	again := portalsOf(t, plannedOpts(planA...))
	if !slices.Equal(first, again) || !slices.Contains(first, 325) || slices.Contains(other, 325) {
		t.Errorf("Portale: erst %v, andere Planung %v, dann wieder %v", first, other, again)
	}
	snapB1 := jsonOf(t, mustPreview(t, plannedOpts(planB...)))
	_ = mustPreview(t, plannedOpts(planA...)) // dazwischen eine andere Planung
	if snapB2 := jsonOf(t, mustPreview(t, plannedOpts(planB...))); snapB1 != snapB2 {
		t.Error("zweiter Aufruf mit Planung B liefert einen anderen Snapshot")
	}
}
