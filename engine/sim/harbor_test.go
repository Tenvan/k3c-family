package sim

import (
	"testing"

	"k3c/engine/level"
)

// BS5.1/AC-01 (B-637/AC-02): Insel 1, Tiefe 0 hat genau einen Hafen am geplanten Platz (−7), andere Tiefen keinen.
func TestHafenChunkAufInselEins(t *testing.T) {
	isl, err := CreateIsland("hafen", []int{0, 1}, 1, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	for depth, w := range isl.Stages {
		var harbors []level.Entity
		for _, e := range w.Level.Entities {
			if e.Kind == level.HarborKind {
				harbors = append(harbors, e)
			}
		}
		if depth == 1 {
			if len(harbors) != 0 {
				t.Errorf("Tiefe 1: %d Häfen", len(harbors))
			}
			continue
		}
		if len(harbors) != 1 {
			t.Fatalf("Tiefe 0: %d Häfen, erwartet 1", len(harbors))
		}
		cw := float64(w.Level.WidthUnits) / float64(len(w.Level.Chunks))
		start := w.Level.HubCenterUnits - 25 - 7*cw // Anfang von −7 (geplante-chunks.md § 1)
		if x := harbors[0].X; x < start || x > start+cw {
			t.Errorf("Hafen bei %v, Platz −7 liegt in %v…%v", x, start, start+cw)
		}
	}
}
