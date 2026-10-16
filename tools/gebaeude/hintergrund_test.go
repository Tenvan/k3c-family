package main

import (
	"fmt"
	"image"
	"image/png"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"testing"
)

// Regeldatei § 10: Mindestabstand der mittleren Helligkeit (Luma 0–255) von Schicht zu Schicht und Obergrenze für „nah“.
const (
	hgMinStufe = 8.0
	hgNahMax   = 75.0
)

var gebaeudeDir = filepath.Join("..", "..", "public", "grafik", "gebaeude")

func decodePNG(t *testing.T, path string) *image.Paletted {
	t.Helper()
	f, err := os.Open(path)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = f.Close() }()
	img, err := png.Decode(f)
	if err != nil {
		t.Fatal(err)
	}
	p, ok := img.(*image.Paletted)
	if !ok {
		t.Fatalf("%s: kein Paletten-Bild", path)
	}
	return p
}

// ladeKlasse liest alle Daten-JSONs der Klassen kachel und hintergrund und rendert sie nach out; liefert id → Klasse.
func ladeKlasse(t *testing.T, out map[string]*image.Paletted) map[string]string {
	t.Helper()
	files, err := filepath.Glob(filepath.Join(gebaeudeDir, "*.json"))
	if err != nil {
		t.Fatal(err)
	}
	klassen := map[string]string{}
	for _, f := range files {
		if filepath.Base(f) == "index.json" {
			continue
		}
		e, err := build(f, "", out)
		if err != nil {
			t.Fatal(err)
		}
		if e.Klasse == "kachel" || e.Klasse == "hintergrund" {
			klassen[e.ID] = e.Klasse
		}
	}
	return klassen
}

// lumaMittel ist die mittlere Helligkeit (0,299 R + 0,587 G + 0,114 B) über die deckenden Pixel.
func lumaMittel(img *image.Paletted) float64 {
	var sum, n float64
	r := img.Bounds()
	for y := r.Min.Y; y < r.Max.Y; y++ {
		for x := r.Min.X; x < r.Max.X; x++ {
			i := img.ColorIndexAt(x, y)
			if i == 0 {
				continue
			}
			cr, cg, cb, _ := img.Palette[i].RGBA()
			sum += 0.299*float64(cr>>8) + 0.587*float64(cg>>8) + 0.114*float64(cb>>8)
			n++
		}
	}
	if n == 0 {
		return 0
	}
	return sum / n
}

// TestKachelKlassen: Klasse kachel und hintergrund sind in jeder Dichte nahtlos (erste = letzte Pixelspalte);
// eine Hintergrund-Schicht ist oben transparent und in der untersten Zeile deckend (GEB7 AC-02).
func TestKachelKlassen(t *testing.T) {
	out := map[string]*image.Paletted{}
	klassen := ladeKlasse(t, out)
	names := make([]string, 0, len(out))
	for name := range out {
		names = append(names, name)
	}
	sort.Strings(names)
	for _, name := range names {
		id := idVonDatei(name)
		klasse, ok := klassen[id]
		if !ok {
			continue
		}
		img := out[name]
		r := img.Bounds()
		for y := r.Min.Y; y < r.Max.Y; y++ {
			if img.ColorIndexAt(r.Min.X, y) != img.ColorIndexAt(r.Max.X-1, y) {
				t.Errorf("%s: Zeile %d, Randspalten ungleich (sichtbare Naht)", name, y)
				break
			}
		}
		if klasse != "hintergrund" {
			continue
		}
		for x := r.Min.X; x < r.Max.X; x++ {
			if img.ColorIndexAt(x, r.Min.Y) != 0 {
				t.Errorf("%s: oberste Zeile nicht transparent (x=%d), der Himmel muss durchscheinen", name, x)
				break
			}
		}
		for x := r.Min.X; x < r.Max.X; x++ {
			if img.ColorIndexAt(x, r.Max.Y-1) == 0 {
				t.Errorf("%s: unterste Zeile hat eine Lücke (x=%d), Bodenlinie und Fluss-Spiegelung brauchen sie deckend", name, x)
				break
			}
		}
	}
}

var dichteSuffix = regexp.MustCompile(`-(16|32|64|128)\.png$`)

// idVonDatei: `hg-wald-fern-32.png` → `hg-wald-fern`.
func idVonDatei(name string) string { return dichteSuffix.ReplaceAllString(name, "") }

// TestHintergrundStufen: je Biom drei Schichten fern, mitte, nah; mittlere Helligkeit steigt um mindestens hgMinStufe
// und „nah“ bleibt unter hgNahMax, also dunkler als die Gebäude davor (GEB7 AC-04, Regeldatei § 10).
func TestHintergrundStufen(t *testing.T) {
	out := map[string]*image.Paletted{}
	klassen := ladeKlasse(t, out)
	biome := map[string]bool{}
	for id, k := range klassen {
		if k == "hintergrund" {
			biome[strings.Split(id, "-")[1]] = true
		}
	}
	if len(biome) == 0 {
		t.Skip("noch keine Hintergrund-Schichten")
	}
	for biom := range biome {
		var l [3]float64
		for i, s := range []string{"fern", "mitte", "nah"} {
			img, ok := out[fmt.Sprintf("hg-%s-%s-32.png", biom, s)]
			if !ok {
				t.Fatalf("hg-%s-%s fehlt (je Biom fern, mitte, nah)", biom, s)
			}
			l[i] = lumaMittel(img)
		}
		if l[1]-l[0] < hgMinStufe || l[2]-l[1] < hgMinStufe {
			t.Errorf("%s: Helligkeit fern %.1f, mitte %.1f, nah %.1f; Abstand mindestens %.0f je Schicht", biom, l[0], l[1], l[2], hgMinStufe)
		}
		if l[2] > hgNahMax {
			t.Errorf("%s: nah %.1f heller als die Obergrenze %.0f", biom, l[2], hgNahMax)
		}
	}
}
