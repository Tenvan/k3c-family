package main

import (
	"bytes"
	"image/color"
	"image/png"
	"testing"
)

func encode(t *testing.T, draw func(*frame, pose), a anim) []byte {
	t.Helper()
	var buf bytes.Buffer
	if err := png.Encode(&buf, sheet(draw, a)); err != nil {
		t.Fatal(err)
	}
	return buf.Bytes()
}

func TestSheetsSindDeterministisch(t *testing.T) {
	for _, draw := range []func(*frame, pose){drawBeetle, drawSpider} {
		for _, a := range anims {
			if !bytes.Equal(encode(t, draw, a), encode(t, draw, a)) {
				t.Fatalf("%s: zwei Läufe unterscheiden sich", a.name)
			}
		}
	}
}

func TestNurPaletteUndHartesAlpha(t *testing.T) {
	allowed := map[color.NRGBA]bool{
		cOutline: true, cDark: true, cMid: true, cLight: true, cShine: true,
		cEye: true, cBlueD: true, cBlue: true, cBlueL: true, cLimb: true,
	}
	for _, draw := range []func(*frame, pose){drawBeetle, drawSpider} {
		for _, a := range anims {
			img := sheet(draw, a)
			b := img.Bounds()
			for y := b.Min.Y; y < b.Max.Y; y++ {
				for x := b.Min.X; x < b.Max.X; x++ {
					c := img.NRGBAAt(x, y)
					if c.A != 0 && !allowed[c] {
						t.Fatalf("%s (%d,%d): Farbe %v nicht in der Palette", a.name, x, y, c)
					}
				}
			}
		}
	}
}
