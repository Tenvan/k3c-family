package backend

import (
	"path/filepath"
	"slices"
	"testing"
)

// B-594: Slot-Details eines Slots ohne Zuordnung nennen „keine Grafik“, ein unbekannter Slot ist ein Fehler.
func TestAssetsSlotDetails(t *testing.T) {
	app := &App{root: studioRepo(t), bus: newBus()}
	write(t, filepath.Join(app.root, "data", "biomes.json"), `{"town":{"buildings":["farm"]}}`)
	d, err := app.AssetsSlotDetails("farm")
	if err != nil {
		t.Fatal(err)
	}
	if !slices.Equal(d.Hints, []string{"keine Grafik"}) || d.Sheet != nil {
		t.Errorf("Hinweise %v, Sheet %v", d.Hints, d.Sheet)
	}
	if !slices.Equal(d.Usage, []string{"data/biomes.json › town.buildings"}) {
		t.Errorf("Verwendung %v", d.Usage)
	}
	if _, err := app.AssetsSlotDetails("nix"); err == nil {
		t.Error("unbekannter Slot soll ein Fehler sein")
	}
}
