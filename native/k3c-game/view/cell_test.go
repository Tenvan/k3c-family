package view

import (
	"image"
	"math"
	"testing"
)

func near(a, b float64) bool { return math.Abs(a-b) < 1e-9 }

func TestCellSplitsWorldAndRiver(t *testing.T) {
	c := Cell{Index: 1, Player: 1, Rect: image.Rect(0, 540, 1920, 1080)}
	if got := c.GroundY(); got != 900 {
		t.Fatalf("GroundY = %d, erwartet 900", got)
	}
	if w := c.WorldRect(); w != image.Rect(0, 540, 1920, 900) {
		t.Errorf("WorldRect = %v", w)
	}
	if r := c.RiverRect(); r != image.Rect(0, 900, 1920, 1080) {
		t.Errorf("RiverRect = %v", r)
	}
	c.Camera = NewCamera(c.Rect, 10, 32)
	if y := c.ScreenY(2); !near(y, 900-64) {
		t.Errorf("ScreenY(2) = %v", y)
	}
}

func TestCameraMapsWorldToScreen(t *testing.T) {
	k := NewCamera(image.Rect(960, 0, 1920, 540), 100, 20)
	if !near(k.Center, 1440) || !near(k.Half, 480) {
		t.Fatalf("Center/Half = %v/%v", k.Center, k.Half)
	}
	if x := k.ScreenX(100); !near(x, 1440) {
		t.Errorf("Mitte: %v", x)
	}
	if x := k.ScreenX(101); !near(x, 1460) {
		t.Errorf("eine Unit rechts: %v", x)
	}
	if wx := k.WorldX(k.ScreenX(87.5)); !near(wx, 87.5) {
		t.Errorf("WorldX(ScreenX) = %v", wx)
	}
	if x := k.ParallaxX(5, 0); !near(x, 5*20+1440) {
		t.Errorf("Faktor 0 steht: %v", x)
	}
	if x := k.ParallaxX(100, 1); !near(x, k.ScreenX(100)) {
		t.Errorf("Faktor 1 wie Boden: %v", x)
	}
	l, r := k.Span()
	if !near(l, 76) || !near(r, 124) {
		t.Errorf("Span = %v … %v", l, r)
	}
	if !k.Visible(125, 1.5) || k.Visible(126, 1.5) || !k.Visible(75, 1) {
		t.Error("Visible falsch am Rand")
	}
}

func TestCameraWithoutScale(t *testing.T) {
	k := Camera{X: 3}
	if k.WorldX(500) != 3 {
		t.Error("WorldX ohne Maßstab")
	}
	if l, r := k.Span(); l != 3 || r != 3 {
		t.Error("Span ohne Maßstab")
	}
}

func TestWindowCell(t *testing.T) {
	c := WindowCell(image.Rect(0, 0, 1280, 720))
	if c.Index != -1 || c.Player != -1 || !near(c.Camera.Center, 640) {
		t.Errorf("WindowCell = %+v", c)
	}
}

// Die Leistungsanzeige hat zwei Zeilen: gleiche Oberkante und Breite, weiter unten zu Ende; eine Zeile ist StatsRect.
func TestStatsRectLines(t *testing.T) {
	win := image.Rect(0, 0, 1280, 720)
	one, two := StatsRectLines(win, 1), StatsRectLines(win, 2)
	if one != StatsRect(win) || StatsRectLines(win, 0) != one {
		t.Errorf("eine Zeile: %v", one)
	}
	if two.Min != one.Min || two.Dx() != one.Dx() || two.Dy() <= one.Dy() {
		t.Errorf("zwei Zeilen: %v gegen %v", two, one)
	}
	w, h, k := StatsTextSizeLines(2)
	if r := StatsRectLines(win, 2); w*k != r.Dx() || h*k != r.Dy() {
		t.Errorf("Textbild %dx%d·%d passt nicht zu %v", w, h, k, r)
	}
}
