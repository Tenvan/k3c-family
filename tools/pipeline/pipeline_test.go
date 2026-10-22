package main

import (
	"bytes"
	"image"
	"image/color"
	"image/png"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// gradient erzeugt ein Testbild mit vielen Farben und weichem Alpha (ohne Zufall).
func gradient(w, h int) *image.NRGBA {
	img := image.NewNRGBA(image.Rect(0, 0, w, h))
	for y := 0; y < h; y++ {
		for x := 0; x < w; x++ {
			img.SetNRGBA(x, y, color.NRGBA{R: uint8(x * 37), G: uint8(y * 53), B: uint8((x + y) * 29), A: uint8((x*y*17)%256)})
		}
	}
	return img
}

// scaled vergrößert img um k (Blockpixel wie in hochskalierten Packs).
func scaled(img *image.NRGBA, k int) *image.NRGBA {
	b := img.Rect
	out := image.NewNRGBA(image.Rect(0, 0, b.Dx()*k, b.Dy()*k))
	for y := 0; y < out.Rect.Dy(); y++ {
		for x := 0; x < out.Rect.Dx(); x++ {
			out.SetNRGBA(x, y, img.NRGBAAt(x/k, y/k))
		}
	}
	return out
}

func writePNG(t *testing.T, file string, img image.Image) {
	t.Helper()
	var buf bytes.Buffer
	if err := png.Encode(&buf, img); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(filepath.Dir(file), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(file, buf.Bytes(), 0o644); err != nil {
		t.Fatal(err)
	}
}

// assertDB32 prüft, dass jedes Pixel transparent ist oder eine DB32-Farbe mit hartem Alpha hat.
func assertDB32(t *testing.T, name string, img image.Image, hard bool) {
	t.Helper()
	b := img.Bounds()
	for y := b.Min.Y; y < b.Max.Y; y++ {
		for x := b.Min.X; x < b.Max.X; x++ {
			c := color.NRGBAModel.Convert(img.At(x, y)).(color.NRGBA)
			if c.A == 0 {
				continue
			}
			if !inDB32(c.R, c.G, c.B) || (hard && c.A != 0xff) {
				t.Fatalf("%s: Pixel (%d,%d) = %v ist nicht DB32 mit hartem Alpha", name, x, y, c)
			}
		}
	}
}

func TestDB32MapsToItself(t *testing.T) {
	for i, c := range db32 {
		if got := nearest(uint8(c>>16), uint8(c>>8), uint8(c)); got != i {
			t.Errorf("DB32 %06x → Index %d, erwartet %d", c, got, i)
		}
	}
}

func TestConvertPaletteAndHardAlpha(t *testing.T) {
	r := convert(gradient(40, 30), options{})
	if r.factor != 1 || r.img.Bounds().Dx() != 40 {
		t.Fatalf("16-px-Quelle ohne Hochskalierung bleibt gleich groß, Faktor %d", r.factor)
	}
	assertDB32(t, "gradient", r.img, true)
}

func TestConvertDetectsUpscale(t *testing.T) {
	src := gradient(10, 8)
	r := convert(scaled(src, 3), options{})
	if r.upscale != 3 || r.img.Bounds() != image.Rect(0, 0, 10, 8) {
		t.Fatalf("Hochskalierung ×3 erwartet 10×8, bekam ×%d %v", r.upscale, r.img.Bounds())
	}
	flat := image.NewNRGBA(image.Rect(0, 0, 16, 32))
	for i := range flat.Pix {
		flat.Pix[i] = 0x40
	}
	if f := convert(flat, options{}); f.upscale != 1 || f.img.Bounds().Dx() != 16 {
		t.Fatalf("einfarbige Kachel gilt nicht als hochskaliert, bekam ×%d", f.upscale)
	}
	want := convert(src, options{})
	if !bytes.Equal(encode(t, r.img), encode(t, want.img)) {
		t.Fatal("hochskalierte Quelle muss dasselbe Ergebnis liefern wie die Quelle selbst")
	}
}

func TestConvertHalves32px(t *testing.T) {
	r := convert(gradient(9, 6), options{Dichte: 32})
	if r.factor != 2 || r.img.Bounds() != image.Rect(0, 0, 5, 3) {
		t.Fatalf("32-px-Quelle wird halbiert (Rand aufgerundet), bekam ×%d %v", r.factor, r.img.Bounds())
	}
	keep := convert(scaled(gradient(8, 8), 2), options{Dichte: 32, Ausnahme: keepSource})
	if keep.factor != 2 || keep.img.Bounds().Dx() != 8 {
		t.Fatalf("quelldichte nimmt nur die Hochskalierung zurück, bekam ×%d", keep.factor)
	}
}

func TestDownscaleMode(t *testing.T) {
	img := image.NewNRGBA(image.Rect(0, 0, 2, 2))
	red, blue := color.NRGBA{R: 0xff, A: 0xff}, color.NRGBA{B: 0xff, A: 0xff}
	img.SetNRGBA(0, 0, red)
	img.SetNRGBA(1, 0, blue)
	img.SetNRGBA(0, 1, blue)
	if got := downscale(img, 2).NRGBAAt(0, 0); got != blue {
		t.Fatalf("häufigste Farbe Blau erwartet, bekam %v", got)
	}
	// Randblock 2×1: Gleichstand deckend gegen transparent → deckend; zwei deckende → kleinerer RGBA-Wert (Blau).
	edge := image.NewNRGBA(image.Rect(0, 0, 2, 1))
	edge.SetNRGBA(0, 0, red)
	if got := downscale(edge, 2).NRGBAAt(0, 0); got != red {
		t.Fatalf("deckend vor transparent erwartet, bekam %v", got)
	}
	edge.SetNRGBA(1, 0, blue)
	if got := downscale(edge, 2).NRGBAAt(0, 0); got != blue {
		t.Fatalf("Gleichstand zweier Farben: kleinerer Wert erwartet, bekam %v", got)
	}
}

func TestSoftAlphaKeepsAlpha(t *testing.T) {
	r := convert(gradient(16, 16), options{Ausnahme: softAlpha})
	assertDB32(t, "alpha-weich", r.img, false)
	if c := color.NRGBAModel.Convert(r.img.At(3, 3)).(color.NRGBA); c.A != uint8(3*3*17) {
		t.Fatalf("Alpha bleibt weich, bekam %d", c.A)
	}
}

// setup legt Quellen und Liste in einem Temp-Ordner an.
func setup(t *testing.T, list string) (string, string, string) {
	t.Helper()
	dir := t.TempDir()
	src := filepath.Join(dir, "src")
	writePNG(t, filepath.Join(src, "a", "x.png"), scaled(gradient(8, 8), 2))
	writePNG(t, filepath.Join(src, "b", "y.png"), gradient(12, 7))
	listFile := filepath.Join(dir, "liste.json")
	if err := os.WriteFile(listFile, []byte(list), 0o644); err != nil {
		t.Fatal(err)
	}
	return listFile, src, filepath.Join(dir, "out")
}

func TestRunTwiceByteEqual(t *testing.T) {
	list, src, out := setup(t, `{"b/y.png": {"dichte": 32}, "a/x.png": {}}`)
	var runs [2]map[string][]byte
	for i := range runs {
		dir := out + string(rune('1'+i))
		if n, err := run(list, src, dir); err != nil || n != 2 {
			t.Fatalf("Lauf %d: n=%d, err=%v", i, n, err)
		}
		runs[i] = readTree(t, dir)
	}
	if len(runs[0]) != 2 {
		t.Fatalf("zwei Ergebnisse erwartet, bekam %d", len(runs[0]))
	}
	for rel, data := range runs[0] {
		if !bytes.Equal(data, runs[1][rel]) {
			t.Fatalf("%s: zwei Läufe nicht byte-gleich", rel)
		}
		img, err := png.Decode(bytes.NewReader(data))
		if err != nil {
			t.Fatal(err)
		}
		assertDB32(t, rel, img, true)
	}
}

func TestRunErrorsWriteNothing(t *testing.T) {
	cases := map[string]string{
		"fehlt":  `{"a/x.png": {}, "c/fehlt.png": {}}`,
		"kaputt": `{"a/x.png": {}, "c/kaputt.png": {}}`,
	}
	for name, list := range cases {
		t.Run(name, func(t *testing.T) {
			listFile, src, out := setup(t, list)
			if err := os.MkdirAll(filepath.Join(src, "c"), 0o755); err != nil {
				t.Fatal(err)
			}
			if err := os.WriteFile(filepath.Join(src, "c", "kaputt.png"), []byte("kein png"), 0o644); err != nil {
				t.Fatal(err)
			}
			_, err := run(listFile, src, out)
			if err == nil || !strings.Contains(err.Error(), "c/"+name+".png") {
				t.Fatalf("Fehler mit Dateinamen erwartet, bekam %v", err)
			}
			if _, err := os.Stat(out); !os.IsNotExist(err) {
				t.Fatal("bei Fehler darf kein Ergebnis geschrieben werden")
			}
		})
	}
}

func TestListRejectsBadEntries(t *testing.T) {
	for _, list := range []string{
		`{}`, `{"../x.png": {}}`, `{"a/./x.png": {}}`, `{"a/x.gif": {}}`,
		`{"a/x.png": {"dichte": 24}}`, `{"a/x.png": {"ausnahme": "egal"}}`, `{"a/x.png": {"farbe": 1}}`,
	} {
		listFile, src, out := setup(t, list)
		if _, err := run(listFile, src, out); err == nil {
			t.Errorf("%s: Fehler erwartet", list)
		}
	}
}

func readTree(t *testing.T, dir string) map[string][]byte {
	t.Helper()
	files := map[string][]byte{}
	err := filepath.WalkDir(dir, func(p string, d os.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		data, err := os.ReadFile(p)
		rel, _ := filepath.Rel(dir, p)
		files[filepath.ToSlash(rel)] = data
		return err
	})
	if err != nil {
		t.Fatal(err)
	}
	return files
}

func encode(t *testing.T, img image.Image) []byte {
	t.Helper()
	var buf bytes.Buffer
	if err := png.Encode(&buf, img); err != nil {
		t.Fatal(err)
	}
	return buf.Bytes()
}
