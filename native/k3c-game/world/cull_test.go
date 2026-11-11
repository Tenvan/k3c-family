package world

import (
	"image"
	"testing"

	"k3c/native/k3c-game/view"
)

// cam ist eine Kamera mit 1 px je Unit über 2000 px: Ausschnitt X ± 1000 Units.
func cam(x float64) view.Camera { return view.NewCamera(image.Rect(0, 0, 2000, 100), x, 1) }

// Fälle aus cameraCull.test.ts (PF1.2): Ausschnitte 1000 … 3000 und 8000 … 10000, Rand CullMargin.
func TestCullMask(t *testing.T) {
	views := []view.Camera{cam(2000), cam(9000)}
	cases := []struct {
		name string
		x    float64
		want uint
	}{
		{"im Ausschnitt der ersten", 2000, 2},
		{"rechter Rand der ersten", 3000 + CullMargin, 2},
		{"linker Rand der ersten", 1000 - CullMargin, 2},
		{"nur in der zweiten", 9000, 1},
		{"außerhalb beider", 5500, 3},
	}
	for _, tc := range cases {
		if got := CullMask(tc.x, views, CullMargin); got != tc.want {
			t.Errorf("%s: Maske %b, erwartet %b", tc.name, got, tc.want)
		}
	}
	if got := CullMask(5500, nil, CullMargin); got != 0 {
		t.Errorf("ohne Kamera: %b", got)
	}
}

// Ausschnitt je Ebene: fern (Faktor 0) hängt nicht an der Kamera, mitte und nah verschieben sich um X · Faktor.
func TestSpanParallax(t *testing.T) {
	for _, x := range []float64{0, 5000} {
		if l, r := Span(cam(x), 0); l != -1000 || r != 1000 {
			t.Errorf("fern bei X=%v: %v … %v", x, l, r)
		}
	}
	if l, r := Span(cam(4000), 0.5); l != 1000 || r != 3000 {
		t.Errorf("nah: %v … %v", l, r)
	}
	if l, r := Span(view.Camera{X: 3}, 1); l <= r {
		t.Errorf("ohne Maßstab nicht leer: %v … %v", l, r)
	}
}

// Kacheln im Ausschnitt: genau die, die hineinragen; ohne Maßstab keine.
func TestTileRange(t *testing.T) {
	k := view.NewCamera(image.Rect(0, 0, 480, 100), 30, 10) // 6 … 54 Units
	if a, b := tileRange(k, 1, 24); a != 0 || b != 2 {
		t.Errorf("Boden: %d … %d", a, b)
	}
	if a, b := tileRange(k, 0, 24); a != -1 || b != 1 {
		t.Errorf("fern: %d … %d", a, b)
	}
	if a, b := tileRange(view.Camera{}, 1, 24); a <= b {
		t.Errorf("ohne Maßstab: %d … %d", a, b)
	}
}
