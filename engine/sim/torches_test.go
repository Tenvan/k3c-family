package sim

import (
	"encoding/json"
	"os"
	"slices"
	"testing"
)

// B-622/AC-01: Fackeln stehen an gebauten Mauern und Türmen, links gespiegelt; an zerstörten oder unfertigen Plätzen
// entfallen sie. Derselbe Fall prüft den Client (src/scenes/render/nightLight.test.ts).
func TestFackelPlaetze(t *testing.T) {
	raw, err := os.ReadFile("../../testdata/torches.json")
	if err != nil {
		t.Fatal(err)
	}
	var c struct {
		HubX    float64
		Offsets map[string][]float64
		Sites   []*Site
		Torches []float64
	}
	if err := json.Unmarshal(raw, &c); err != nil {
		t.Fatal(err)
	}
	if got := torchXs(c.Sites, c.HubX, c.Offsets); !slices.Equal(got, c.Torches) {
		t.Fatalf("Fackeln %v, erwartet %v", got, c.Torches)
	}
}

// B-622/AC-01: Die Welt liest die Versätze aus data/light.json; eine zerstörte Mauer verliert ihre Fackeln.
func TestFackelnDerWelt(t *testing.T) {
	w := quietWorld(t)
	if len(torchOffsets["wall"]) == 0 || len(torchOffsets["tower"]) == 0 {
		t.Fatalf("data/light.json › torches ohne Mauer oder Turm: %v", torchOffsets)
	}
	if got := TorchXs(w); len(got) != 0 {
		t.Fatalf("ohne gebaute Mauer schon Fackeln: %v", got)
	}
	wall := leftWall(t, w)
	wall.State, wall.HP = "built", wall.MaxHP
	if got := TorchXs(w); len(got) != len(torchOffsets["wall"]) {
		t.Fatalf("gebaute Mauer: %d Fackeln, erwartet %d", len(got), len(torchOffsets["wall"]))
	}
	destroySite(w, wall)
	if got := TorchXs(w); len(got) != 0 {
		t.Fatalf("zerstörte Mauer trägt noch Fackeln: %v", got)
	}
}
