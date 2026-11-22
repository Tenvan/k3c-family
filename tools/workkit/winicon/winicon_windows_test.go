//go:build windows

package winicon

import (
	"bytes"
	"image"
	"image/color"
	"image/png"
	"testing"
)

// Ein PNG wird zu einem gültigen Icon-Handle (Fenster-Icon im Dev-Lauf, ohne Ressource der EXE); ohne PNG gibt es keins.
// Die Apps prüfen ihr eingebettetes PNG selbst (winicon_test.go).
func TestHandleAusPNG(t *testing.T) {
	img := image.NewRGBA(image.Rect(0, 0, 32, 32))
	img.Set(3, 3, color.RGBA{R: 255, A: 255})
	var buf bytes.Buffer
	if err := png.Encode(&buf, img); err != nil {
		t.Fatal(err)
	}
	for _, px := range []int{16, 32} {
		if Handle(buf.Bytes(), px) == 0 {
			t.Errorf("kein Icon der Größe %d aus dem PNG", px)
		}
	}
	if Handle(nil, 32) != 0 {
		t.Error("Icon aus leerem PNG")
	}
}
