package main

import (
	"bytes"
	"image"
	"image/color"
	"image/png"
	"path/filepath"
	"testing"
)

// figure: Testfigur aus zwei Frames, ein aufrechter Balken 10x40 über dem Fußpunkt.
func figure(k kingSheet, frames int) *image.NRGBA {
	img := image.NewNRGBA(image.Rect(0, 0, k.fw*frames, k.fh))
	for i := 0; i < frames; i++ {
		fill(img, image.Rect(i*k.fw+k.footX-5, k.footY-39, i*k.fw+k.footX+5, k.footY+1), color.NRGBA{R: 200, G: 30, B: 30, A: 255})
	}
	return img
}

func fill(img *image.NRGBA, r image.Rectangle, c color.NRGBA) {
	for y := r.Min.Y; y < r.Max.Y; y++ {
		for x := r.Min.X; x < r.Max.X; x++ {
			img.SetNRGBA(x, y, c)
		}
	}
}

func pngBytes(t *testing.T, img image.Image) []byte {
	t.Helper()
	var buf bytes.Buffer
	if err := png.Encode(&buf, img); err != nil {
		t.Fatal(err)
	}
	return buf.Bytes()
}

var testKing = kingSheet{name: "test", fw: 64, fh: 48, footX: 30, footY: 44, give: 2}

func TestMonarchDeterministischUndFrameGroessen(t *testing.T) {
	want := map[string]int{"give": 3, "hit": 3, "death": 5, "respawn": 5}
	a, err := deriveMonarch(testKing, figure(testKing, 2), figure(testKing, 3))
	if err != nil {
		t.Fatal(err)
	}
	b, _ := deriveMonarch(testKing, figure(testKing, 2), figure(testKing, 3))
	for _, name := range monarchAnims {
		strip := joinFrames(a[name])
		if !bytes.Equal(pngBytes(t, strip), pngBytes(t, joinFrames(b[name]))) {
			t.Fatalf("%s: zwei Läufe unterscheiden sich", name)
		}
		if got := strip.Rect.Size(); got != image.Pt(want[name]*testKing.fw, testKing.fh) {
			t.Errorf("%s: Streifen %v, erwartet %d Frames à %dx%d", name, got, want[name], testKing.fw, testKing.fh)
		}
	}
}

func TestTodLiegtAmBoden(t *testing.T) {
	f, err := tilt(cutFrame(figure(testKing, 1), 0, testKing.fw, testKing.fh), testKing, 90)
	if err != nil {
		t.Fatal(err)
	}
	bb := opaqueBounds(f)
	if bb.Dx() < 38 || bb.Dy() > 12 {
		t.Errorf("gekippter Balken %v, erwartet liegend (breit ≥38, hoch ≤12)", bb)
	}
	if bb.Max.Y != testKing.footY+1 {
		t.Errorf("Unterkante %d, erwartet Boden %d", bb.Max.Y, testKing.footY+1)
	}
}

func TestZuGrosseFigurBrichtAb(t *testing.T) {
	small := kingSheet{name: "eng", fw: 20, fh: 48, footX: 10, footY: 44}
	src := image.NewNRGBA(image.Rect(0, 0, 20, 48))
	fill(src, image.Rect(5, 4, 15, 45), color.NRGBA{R: 1, A: 255})
	if _, err := tilt(src, small, 90); err == nil {
		t.Fatal("liegende Figur breiter als der Frame muss einen Fehler liefern")
	}
}

func TestTrefferBlitztUndRespawnWaechstVonUnten(t *testing.T) {
	a, _ := deriveMonarch(testKing, figure(testKing, 1), figure(testKing, 2))
	if c := a["hit"][0].NRGBAAt(testKing.footX-5, 20); c.R < 240 || c.G < 180 {
		t.Errorf("erster Treffer-Frame %v, erwartet aufgehellt", c)
	}
	first, last := opaqueBounds(a["respawn"][0]), opaqueBounds(a["respawn"][4])
	if first.Max.Y != last.Max.Y || first.Dy() >= last.Dy()/3 {
		t.Errorf("Respawn: erster Frame %v, letzter %v, erwartet unten bündig und klein", first, last)
	}
}

// Die echten König-Sheets passen in allen Frames in ihre Frame-Größe (nichts abgeschnitten).
func TestKoenigSheetsNichtsAbgeschnitten(t *testing.T) {
	for _, k := range kings {
		idle, err := readPNG(filepath.Join("..", "..", "public", "sprites", k.name, "idle.png"))
		if err != nil {
			t.Skipf("%s fehlt: %v", k.name, err)
		}
		frame := cutFrame(idle, 0, k.fw, k.fh)
		for _, deg := range deathAngles {
			if _, err := tilt(frame, k, deg); err != nil {
				t.Error(err)
			}
		}
	}
}
