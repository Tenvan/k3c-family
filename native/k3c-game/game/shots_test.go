package game

import (
	"image"
	"image/color"
	"testing"
)

func filled(w, h int, c color.RGBA) *image.RGBA {
	m := image.NewRGBA(image.Rect(0, 0, w, h))
	for y := range h {
		for x := range w {
			m.SetRGBA(x, y, c)
		}
	}
	return m
}

func TestShrinkAverages(t *testing.T) {
	src := filled(4, 2, color.RGBA{0, 0, 0, 255})
	src.SetRGBA(0, 0, color.RGBA{200, 100, 40, 255})
	got := Shrink(src, 2)
	if got.Bounds() != image.Rect(0, 0, 2, 1) {
		t.Fatalf("Größe %v", got.Bounds())
	}
	if c := got.RGBAAt(0, 0); c != (color.RGBA{50, 25, 10, 255}) {
		t.Errorf("Mittel %v", c)
	}
	if c := got.RGBAAt(1, 0); c != (color.RGBA{0, 0, 0, 255}) {
		t.Errorf("rechter Block %v", c)
	}
	if one := Shrink(src, 1); one.RGBAAt(0, 0) != src.RGBAAt(0, 0) || &one.Pix[0] == &src.Pix[0] {
		t.Error("Faktor 1 kopiert nicht")
	}
}

func TestStripSideBySide(t *testing.T) {
	a, b := filled(3, 2, color.RGBA{255, 0, 0, 255}), filled(2, 3, color.RGBA{0, 0, 255, 255})
	s := Strip([]*image.RGBA{a, b}, 1)
	if s.Bounds() != image.Rect(0, 0, 6, 3) {
		t.Fatalf("Größe %v", s.Bounds())
	}
	checks := []struct {
		x, y int
		want color.RGBA
	}{
		{0, 0, a.RGBAAt(0, 0)}, {2, 1, a.RGBAAt(0, 0)}, {3, 0, stripBack}, {0, 2, stripBack}, {4, 0, b.RGBAAt(0, 0)}, {5, 2, b.RGBAAt(0, 0)},
	}
	for _, c := range checks {
		if got := s.RGBAAt(c.x, c.y); got != c.want {
			t.Errorf("(%d,%d) = %v, erwartet %v", c.x, c.y, got, c.want)
		}
	}
}
