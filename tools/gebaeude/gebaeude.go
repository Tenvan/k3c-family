package main

import (
	"encoding/json"
	"fmt"
	"image"
	"image/png"
	"io"
	"math"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// master ist die Master-Dichte in px je Kachel (= UNIT_PX); Koordinaten sind Vielfache von 1/master Kachel.
const master = 32

// dichten sind die erlaubten Render-Dichten in px je Kachel (½, 1, 2, 4 × Master).
var dichten = []int{16, 32, 64, 128}

type kacheln struct {
	B int `json:"b"`
	H int `json:"h"`
}

// baustein ist ein Rechteck [x, y, b, h] in Kachel-Einheiten; fein = entfällt bei gröberer Dichte als Master.
type baustein struct {
	Art      string     `json:"art"`
	Rechteck [4]float64 `json:"rechteck"`
	Farbe    string     `json:"farbe"`
	Fein     bool       `json:"fein,omitempty"`
}

type building struct {
	ID        string     `json:"id"`
	Kacheln   kacheln    `json:"kacheln"`
	Bausteine []baustein `json:"bausteine"`
}

type eintrag struct {
	ID      string            `json:"id"`
	Kacheln kacheln           `json:"kacheln"`
	Dateien map[string]string `json:"dateien"`
}

func erlaubt(d int) bool {
	for _, v := range dichten {
		if v == d {
			return true
		}
	}
	return false
}

// masterPx rechnet [x, y, b, h] in ganze Master-Pixel um; nur Vielfache von 1/32 Kachel sind erlaubt.
func masterPx(r [4]float64) ([4]int, error) {
	var out [4]int
	for i, v := range r {
		m := v * master
		if math.Abs(m-math.Round(m)) > 1e-9 {
			return out, fmt.Errorf("wert %g liegt nicht im 1/%d-Kachel-Raster", v, master)
		}
		out[i] = int(math.Round(m))
	}
	return out, nil
}

// render zeichnet das Gebäude in Dichte d (px je Kachel) als Bild mit DB32-Palette, ohne Glättung.
func render(b building, d int) (*image.Paletted, error) {
	if !erlaubt(d) {
		return nil, fmt.Errorf("dichte %d nicht erlaubt (16, 32, 64, 128 px je Kachel)", d)
	}
	pal, idx := palette()
	img := image.NewPaletted(image.Rect(0, 0, b.Kacheln.B*d, b.Kacheln.H*d), pal)
	for i, s := range b.Bausteine {
		c, m, err := check(b, i, idx, d)
		if err != nil {
			return nil, err
		}
		if d < master && s.Fein {
			continue
		}
		// Ganzzahlige Abbildung Master-Pixel → Ziel-Pixel; bei d ≥ Master exakt, bei 16 abgerundet.
		x0, y0 := m[0]*d/master, m[1]*d/master
		x1, y1 := (m[0]+m[2])*d/master, (m[1]+m[3])*d/master
		for y := y0; y < y1; y++ {
			for x := x0; x < x1; x++ {
				img.SetColorIndex(x, y, c)
			}
		}
	}
	return img, nil
}

// check prüft Baustein i (Farbe, Raster, Maße, Mindestbreite bei Dichte d) und liefert Farbindex und Master-Pixel.
func check(b building, i int, idx map[string]uint8, d int) (uint8, [4]int, error) {
	s := b.Bausteine[i]
	where := fmt.Sprintf("%s: Baustein %d (%s)", b.ID, i, s.Art)
	c, ok := idx[s.Farbe]
	if !ok {
		return 0, [4]int{}, fmt.Errorf("%s: Farbe %q nicht in DB32", where, s.Farbe)
	}
	m, err := masterPx(s.Rechteck)
	if err != nil {
		return 0, m, fmt.Errorf("%s: %w", where, err)
	}
	if m[0] < 0 || m[1] < 0 || m[2] <= 0 || m[3] <= 0 || m[0]+m[2] > b.Kacheln.B*master || m[1]+m[3] > b.Kacheln.H*master {
		return 0, m, fmt.Errorf("%s: Rechteck %v außerhalb von %d×%d Kacheln", where, s.Rechteck, b.Kacheln.B, b.Kacheln.H)
	}
	if d < master && !s.Fein && (m[2]*d < master || m[3]*d < master) {
		return 0, m, fmt.Errorf("%s: dünner als 1 px bei %d px je Kachel, als fein markieren", where, d)
	}
	return c, m, nil
}

// run baut jedes *.json (außer index.json) in dir zu PNGs je Dichte und schreibt index.json; liefert die Zahl der Gebäude.
func run(dir, prefix string) (int, error) {
	files, err := filepath.Glob(filepath.Join(dir, "*.json"))
	if err != nil {
		return 0, err
	}
	sort.Strings(files)
	// Alte PNGs weg, damit gelöschte oder umbenannte Gebäude keine Waisen hinterlassen; der Ordner hält nur Ausgaben.
	old, err := filepath.Glob(filepath.Join(dir, "*.png"))
	if err != nil {
		return 0, err
	}
	for _, f := range old {
		if err := os.Remove(f); err != nil {
			return 0, err
		}
	}
	var index []eintrag
	for _, f := range files {
		if filepath.Base(f) == "index.json" {
			continue
		}
		e, err := build(f, dir, prefix)
		if err != nil {
			return 0, err
		}
		index = append(index, e)
	}
	data, err := json.MarshalIndent(index, "", "  ")
	if err != nil {
		return 0, err
	}
	return len(index), os.WriteFile(filepath.Join(dir, "index.json"), append(data, '\n'), 0o644)
}

func build(file, dir, prefix string) (eintrag, error) {
	var b building
	raw, err := os.ReadFile(file)
	if err != nil {
		return eintrag{}, err
	}
	if err := json.Unmarshal(raw, &b); err != nil {
		return eintrag{}, fmt.Errorf("%s: %w", file, err)
	}
	if want := strings.TrimSuffix(filepath.Base(file), ".json"); b.ID != want {
		return eintrag{}, fmt.Errorf("%s: id %q passt nicht zum Dateinamen", file, b.ID)
	}
	e := eintrag{ID: b.ID, Kacheln: b.Kacheln, Dateien: map[string]string{}}
	for _, d := range dichten {
		img, err := render(b, d)
		if err != nil {
			return eintrag{}, err
		}
		name := fmt.Sprintf("%s-%d.png", b.ID, d)
		if err := writePNG(filepath.Join(dir, name), img); err != nil {
			return eintrag{}, err
		}
		e.Dateien[fmt.Sprint(d)] = prefix + name
	}
	return e, nil
}

func writePNG(path string, img image.Image) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	if err := encode(f, img); err != nil {
		_ = f.Close()
		return err
	}
	return f.Close()
}

// encode schreibt PNG ohne Zeitstempel; gleiche Pixel ergeben gleiche Bytes.
func encode(w io.Writer, img image.Image) error {
	return png.Encode(w, img)
}
