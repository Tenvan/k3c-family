package art

import (
	"image/color"
	"testing"
)

// Sweep gibt Zwischenbilder frei, die outlineKeep Durchläufe nicht gezeichnet wurden; gezeichnete bleiben.
func TestOutlineSweep(t *testing.T) {
	var c OutlineCache
	c.entries = map[outlineKey]*outlineEntry{{s: "alt"}: {}, {s: "neu"}: {}}
	for range outlineKeep {
		c.Sweep()
		c.entries[outlineKey{s: "neu"}].used = c.gen
	}
	if c.Len() != 2 {
		t.Fatalf("zu früh freigegeben: %d", c.Len())
	}
	c.Sweep()
	if _, ok := c.entries[outlineKey{s: "neu"}]; !ok || c.Len() != 1 {
		t.Errorf("nach %d Durchläufen: %d Bilder", outlineKeep+1, c.Len())
	}
}

// Ohne Schrift, Text oder Deckkraft zeichnet Draw nichts und legt nichts an; Debug-Schrift ohne Zwischenbild.
func TestOutlineDrawSkips(t *testing.T) {
	var c OutlineCache
	c.Draw(nil, nil, "x", 0, 0, 20, color.RGBA{255, 255, 255, 255}, Outline8, 1)
	c.Draw(nil, &Font{}, "", 0, 0, 20, color.RGBA{}, Outline8, 1)
	c.Draw(nil, &Font{}, "x", 0, 0, 20, color.RGBA{}, Outline8, 0)
	if c.Len() != 0 {
		t.Errorf("%d Bilder", c.Len())
	}
	if got := scaleAlpha(color.RGBA{200, 100, 50, 255}, 0.5); got != (color.RGBA{100, 50, 25, 127}) {
		t.Errorf("Deckkraft %v", got)
	}
	if OutlineStroke(7) != 1 || OutlineStroke(28) != 2 || outlinePad(28) != 2+9 {
		t.Errorf("Kontur %v, Rand %v", OutlineStroke(28), outlinePad(28))
	}
}
