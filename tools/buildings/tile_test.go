package main

import (
	"image"
	"path/filepath"
	"testing"
)

// TestHintergrundmauerKachelbar: Die Hintergrundmauer hat in jeder Stufe und Dichte gleiche erste und letzte
// Pixelspalte, damit Kacheln nebeneinander nahtlos anschließen (GEB3 AC-07).
func TestHintergrundmauerKachelbar(t *testing.T) {
	out := map[string]*image.Paletted{}
	e, err := build(filepath.Join("..", "..", "public", "grafik", "gebaeude", "hintergrundmauer.json"), "", out)
	if err != nil {
		t.Fatal(err)
	}
	if len(e.Stufen) != 5 || len(out) != 5*len(dichten) {
		t.Fatalf("Stufen: %d, Bilder: %d", len(e.Stufen), len(out))
	}
	for name, img := range out {
		r := img.Bounds()
		for y := r.Min.Y; y < r.Max.Y; y++ {
			if img.ColorIndexAt(r.Min.X, y) != img.ColorIndexAt(r.Max.X-1, y) {
				t.Errorf("%s: Zeile %d, Randspalten ungleich", name, y)
				break
			}
		}
	}
}
