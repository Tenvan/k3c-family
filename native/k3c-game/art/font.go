package art

// Schrift für HUD und Menüs: eine freie OFL-Schrift aus festem Pfad (Datei kommt über die MCP-Tools `assets_*` ins Repo).
// Fehlt die Datei, fällt die Schrift nicht still zurück, sondern meldet es mit Pfad: im strengen Modus als Fehler,
// sonst mit der eingebauten Ebitengine-Debug-Schrift (feste Größe, keine Farbe) und Warnung im Log.

import (
	"bytes"
	"fmt"
	"image/color"
	"log/slog"
	"os"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
	"github.com/hajimehoshi/ebiten/v2/text/v2"
)

// Font zeichnet Text in einer Schrift; Fallback = Debug-Schrift (Größe und Farbe wirken dann nicht).
type Font struct {
	Path     string
	Fallback bool

	src   *text.GoTextFaceSource
	faces map[float64]*text.GoTextFace
	op    text.DrawOptions
}

// debug-Schrift von ebitenutil: 6 × 16 Pixel je Zeichen (Zeilenabstand 16, `\n` bricht dort selbst um)
const (
	debugCharW = 6
	debugCharH = 16
)

// LineSpacing ist der Abstand zweier Grundlinien als Vielfaches der Schriftgröße (Text mit `\n`).
const LineSpacing = 1.25

// LoadFont lädt die Schrift aus path. Fehlt die Datei: strict → Fehler (ErrMissing, mit Pfad), sonst Debug-Schrift.
// Eine vorhandene, aber kaputte Datei ist immer ein Fehler.
func LoadFont(path string, strict bool) (*Font, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		if strict {
			return nil, fmt.Errorf("%w: Schrift %s (%v)", ErrMissing, path, err)
		}
		slog.Warn("📄 Schrift fehlt, Debug-Schrift als Ersatz", "pfad", path, "fehler", err)
		return &Font{Path: path, Fallback: true}, nil
	}
	src, err := text.NewGoTextFaceSource(bytes.NewReader(data))
	if err != nil {
		return nil, fmt.Errorf("%s: Schrift kaputt (%w)", path, err)
	}
	return &Font{Path: path, src: src, faces: map[float64]*text.GoTextFace{}}, nil
}

// face liefert die Schrift in Größe size (Pixel), je Größe einmal angelegt.
func (f *Font) face(size float64) *text.GoTextFace {
	if g, ok := f.faces[size]; ok {
		return g
	}
	g := &text.GoTextFace{Source: f.src, Size: size}
	f.faces[size] = g
	return g
}

// Measure liefert Breite (längste Zeile) und Höhe von s in Pixeln bei Größe size; `\n` beginnt eine neue Zeile im
// Abstand LineSpacing × size.
func (f *Font) Measure(s string, size float64) (w, h float64) {
	if f.Fallback {
		lines, widest := debugLines(s)
		return float64(widest * debugCharW), float64(lines * debugCharH)
	}
	return text.Measure(s, f.face(size), LineSpacing*size)
}

// debugLines zählt die Zeilen von s und die Zeichen der längsten (Debug-Schrift, ein Byte je Zeichen wie DebugPrint).
func debugLines(s string) (lines, widest int) {
	n := 0
	lines = 1
	for i := range len(s) {
		if s[i] == '\n' {
			lines, widest, n = lines+1, max(widest, n), 0
			continue
		}
		n++
	}
	return lines, max(widest, n)
}

// Draw zeichnet s mit der linken oberen Ecke bei (x, y); `\n` beginnt eine neue Zeile im Abstand LineSpacing × size.
// clr ist vormultipliziert (color.RGBA). Die Größen sollten aus einer kleinen Menge kommen (Rollen), jede neue Größe
// legt einmal ein Face an. Font selbst legt je Aufruf nichts an (Optionen und Faces werden wiederverwendet); was
// text.Draw intern braucht (Glyph-Puffer aus sync.Pool, Glyph-Cache), liegt bei Ebitengine und ist nicht abschaltbar.
func (f *Font) Draw(dst *ebiten.Image, s string, x, y, size float64, clr color.RGBA) {
	if f.Fallback {
		ebitenutil.DebugPrintAt(dst, s, int(x), int(y))
		return
	}
	f.op.GeoM.Reset()
	f.op.GeoM.Translate(x, y)
	f.op.ColorScale.Reset()
	f.op.ColorScale.SetR(float32(clr.R) / 255)
	f.op.ColorScale.SetG(float32(clr.G) / 255)
	f.op.ColorScale.SetB(float32(clr.B) / 255)
	f.op.ColorScale.SetA(float32(clr.A) / 255)
	f.op.LineSpacing = LineSpacing * size
	text.Draw(dst, s, f.face(size), &f.op)
}
