package main

import (
	"bytes"
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

// kacheln sind die Gebäudemaße in Kacheln, Vielfache von 1/32 (Baum: 1 × 2,25).
type kacheln struct {
	B float64 `json:"b"`
	H float64 `json:"h"`
}

// baustein ist ein Rechteck [x, y, b, h] in Kachel-Einheiten; fein = entfällt bei gröberer Dichte als Master.
type baustein struct {
	Art      string     `json:"art"`
	Rechteck [4]float64 `json:"rechteck"`
	Farbe    string     `json:"farbe"`
	Fein     bool       `json:"fein,omitempty"`
	// Detail entfällt unter der Dichte des feinen Rasters (nur mit `raster` 128, GEB18): Feinheiten der vierfachen Auflösung.
	Detail bool `json:"detail,omitempty"`
	// Wiederholung kopiert den Baustein entlang x (Regeldatei § 5); `build` löst sie vor dem Zeichnen auf.
	Wiederholung *wiederholung `json:"wiederholung,omitempty"`
}

// vorlage ist die Gebäude-Vorlage (Glossar): Absicht neben den Koordinaten, damit eine Session das Gebäude neu erzeugen kann.
type vorlage struct {
	Motiv       string   `json:"motiv"`
	Teile       []string `json:"teile"`
	Materialien []string `json:"materialien,omitempty"`
	Stimmung    string   `json:"stimmung,omitempty"`
	Fein        string   `json:"fein,omitempty"`
}

type building struct {
	ID        string     `json:"id"`
	Klasse    string     `json:"klasse,omitempty"` // z. B. "baum"; die Vorschau gruppiert danach
	Vorlage   *vorlage   `json:"vorlage"`
	Kacheln   kacheln    `json:"kacheln"`
	// Raster in Teilen je Kachel: ohne Angabe 32, Hintergrund-Schichten in vierfacher Auflösung 128 (GEB18, `hires.go`)
	Raster    int        `json:"raster,omitempty"`
	Bausteine []baustein `json:"bausteine"`
	Stufen    []stufe    `json:"stufen,omitempty"` // Ausbaustufen 2, 3, …; Grundform = Stufe 1 (§ 6)
	// Stufe1 zeichnet nur die Stufe 1 anders als `bausteine`; die Stufen ab 2 bauen weiter auf `bausteine` auf (B-653)
	Stufe1 []baustein `json:"stufe1,omitempty"`
}

type eintrag struct {
	ID      string            `json:"id"`
	Klasse  string            `json:"klasse,omitempty"`
	Kacheln kacheln           `json:"kacheln"`
	Vorlage *vorlage          `json:"vorlage"`
	Dateien map[string]string `json:"dateien"`
	Stufen  []stufenEintrag   `json:"stufen,omitempty"`
}

func erlaubt(d int) bool {
	for _, v := range dichten {
		if v == d {
			return true
		}
	}
	return false
}

// masterPx rechnet [x, y, b, h] in ganze Raster-Pixel um; nur Vielfache von 1/raster Kachel sind erlaubt.
func masterPx(r [4]float64, raster int) ([4]int, error) {
	var out [4]int
	for i, v := range r {
		m := v * float64(raster)
		if math.Abs(m-math.Round(m)) > 1e-9 {
			return out, fmt.Errorf("wert %g liegt nicht im 1/%d-Kachel-Raster", v, raster)
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
	raster := rasterVon(b)
	size, err := masterPx([4]float64{0, 0, b.Kacheln.B, b.Kacheln.H}, raster)
	if err != nil || size[2] <= 0 || size[3] <= 0 {
		return nil, fmt.Errorf("%s: kacheln %v ungültig", b.ID, b.Kacheln)
	}
	pal, idx := palette()
	img := image.NewPaletted(image.Rect(0, 0, size[2]*d/raster, size[3]*d/raster), pal)
	for i, s := range b.Bausteine {
		c, m, err := check(b, i, idx, size, d)
		if err != nil {
			return nil, err
		}
		if entfaellt(s, d, raster) {
			continue
		}
		// Ganzzahlige Abbildung Raster-Pixel → Ziel-Pixel; bei d ≥ Raster exakt, darunter abgerundet.
		x0, y0 := m[0]*d/raster, m[1]*d/raster
		x1, y1 := (m[0]+m[2])*d/raster, (m[1]+m[3])*d/raster
		for y := y0; y < y1; y++ {
			for x := x0; x < x1; x++ {
				img.SetColorIndex(x, y, c)
			}
		}
	}
	return img, nil
}

// check prüft Baustein i (Farbe, Raster, Maße, Mindestbreite bei Dichte d) und liefert Farbindex und Master-Pixel.
func check(b building, i int, idx map[string]uint8, size [4]int, d int) (uint8, [4]int, error) {
	s := b.Bausteine[i]
	where := fmt.Sprintf("%s: Baustein %d (%s)", b.ID, i, s.Art)
	c, ok := idx[s.Farbe]
	if !ok {
		return 0, [4]int{}, fmt.Errorf("%s: Farbe %q nicht in DB32", where, s.Farbe)
	}
	raster := rasterVon(b)
	m, err := masterPx(s.Rechteck, raster)
	if err != nil {
		return 0, m, fmt.Errorf("%s: %w", where, err)
	}
	if m[0] < 0 || m[1] < 0 || m[2] <= 0 || m[3] <= 0 || m[0]+m[2] > size[2] || m[1]+m[3] > size[3] {
		return 0, m, fmt.Errorf("%s: Rechteck %v außerhalb von %g×%g Kacheln", where, s.Rechteck, b.Kacheln.B, b.Kacheln.H)
	}
	if !entfaellt(s, d, raster) && (m[2]*d < raster || m[3]*d < raster) {
		return 0, m, fmt.Errorf("%s: dünner als 1 px bei %d px je Kachel, als fein oder detail markieren", where, d)
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
	// Erst alles rendern: scheitert ein Gebäude, bleibt der Ordner unverändert.
	var index []eintrag
	out := map[string]*image.Paletted{}
	for _, f := range files {
		if filepath.Base(f) == "index.json" {
			continue
		}
		e, err := build(f, prefix, out)
		if err != nil {
			return 0, err
		}
		index = append(index, e)
	}
	// Dann alte PNGs weg, damit gelöschte oder umbenannte Gebäude keine Waisen hinterlassen; der Ordner hält nur Ausgaben.
	old, err := filepath.Glob(filepath.Join(dir, "*.png"))
	if err != nil {
		return 0, err
	}
	for _, f := range old {
		if err := os.Remove(f); err != nil {
			return 0, err
		}
	}
	for name, img := range out {
		if err := writePNG(filepath.Join(dir, name), img); err != nil {
			return 0, err
		}
	}
	data, err := json.MarshalIndent(index, "", "  ")
	if err != nil {
		return 0, err
	}
	return len(index), os.WriteFile(filepath.Join(dir, "index.json"), append(data, '\n'), 0o644)
}

// build liest ein Gebäude, rendert es in allen Dichten nach out (Dateiname → Bild) und liefert seinen Index-Eintrag.
func build(file, prefix string, out map[string]*image.Paletted) (eintrag, error) {
	var b building
	raw, err := os.ReadFile(file)
	if err != nil {
		return eintrag{}, err
	}
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields() // Tippfehler in Vorlage oder Bausteinen fallen auf
	if err := dec.Decode(&b); err != nil {
		return eintrag{}, fmt.Errorf("%s: %w", file, err)
	}
	if err := pruefeVorlage(b.Vorlage); err != nil {
		return eintrag{}, fmt.Errorf("%s: %w", file, err)
	}
	if err := pruefeRaster(b); err != nil {
		return eintrag{}, fmt.Errorf("%s: %w", file, err)
	}
	if err := wiederholungenAufloesen(&b); err != nil {
		return eintrag{}, fmt.Errorf("%s: %w", file, err)
	}
	if want := strings.TrimSuffix(filepath.Base(file), ".json"); b.ID != want {
		return eintrag{}, fmt.Errorf("%s: id %q passt nicht zum Dateinamen", file, b.ID)
	}
	e := eintrag{ID: b.ID, Klasse: b.Klasse, Kacheln: b.Kacheln, Vorlage: b.Vorlage}
	if len(b.Stufen) == 0 {
		e.Dateien, err = renderDichten(b, b.ID, prefix, out)
		return e, err
	}
	if e.Stufen, err = renderStufen(b, prefix, out); err != nil {
		return eintrag{}, err
	}
	e.Dateien = e.Stufen[0].Dateien // Karten der Vorschau zeigen die Grundform
	return e, nil
}

// pruefeVorlage verlangt eine Vorlage mit Motiv und mindestens einem Teil (GEB8).
func pruefeVorlage(v *vorlage) error {
	switch {
	case v == nil:
		return fmt.Errorf("vorlage fehlt")
	case strings.TrimSpace(v.Motiv) == "":
		return fmt.Errorf("vorlage.motiv ist leer")
	case len(v.Teile) == 0:
		return fmt.Errorf("vorlage.teile ist leer")
	}
	return nil
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
