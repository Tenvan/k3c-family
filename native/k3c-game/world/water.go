package world

// Fluss mit Kage-Shader (B-790, NG11.1): ein DrawRectShader je Zelle über den Fluss unter dem Uferstreifen. Lage,
// Kachelbreite und Fließen der Reihen rechnet riverLayout wie für die Kacheln; der Shader (water/river.kage) bekommt
// sie mit den Phasen der Animation als Uniforms, die Phasen folgen nur aus Frame.Clock. Die Fassung wählt die
// Umgebungsvariable K3C_RIVER (tiles oder shader, Standard shader) als Startwert; im Spiel schaltet F3 um
// (view.Settings.River, B-816). Übersetzt der Shader nicht, bleibt der Fluss bei den Kacheln.

import (
	_ "embed"
	"image/color"
	"log/slog"
	"math"
	"os"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

//go:embed water/river.kage
var riverKage []byte

// riverEnv wählt die Fassung des Flusses (tiles oder shader).
const riverEnv = "K3C_RIVER"

// Tempo der Shader-Animation in Umläufen je Sekunde: Verzerrung, Heben des Bands, Glanz.
const (
	waterWobbleHz = 0.35
	waterLiftHz   = 0.5
	waterGlintHz  = 0.8
)

var (
	riverShade = color.RGBA{50, 104, 150, 255}
	riverGlint = color.RGBA{150, 196, 226, 255}
)

// riverMode ist die Fassung des Flusses.
type riverMode int

const (
	riverShader riverMode = iota
	riverTiles
)

func (m riverMode) String() string {
	if m == riverTiles {
		return "tiles"
	}
	return "shader"
}

// riverModeOf liest den Wert von K3C_RIVER; alles außer "tiles" ist die Shader-Fassung.
func riverModeOf(v string) riverMode {
	if strings.EqualFold(strings.TrimSpace(v), "tiles") {
		return riverTiles
	}
	return riverShader
}

// RiverStart ist der Startwert der Fluss-Fassung aus K3C_RIVER als view.RiverShader oder view.RiverTiles (game schaltet
// mit view.Settings.NextRiver von diesem Wert aus um, solange keine Fassung gewählt ist).
func RiverStart() string { return riverModeOf(os.Getenv(riverEnv)).String() }

// riverModeFor ist die Fassung nach den Einstellungen: "" (nichts gewählt) gilt der Startwert start.
func riverModeFor(setting string, start riverMode) riverMode {
	switch setting {
	case view.RiverTiles:
		return riverTiles
	case view.RiverShader:
		return riverShader
	}
	return start
}

// waterParams sind die Uniforms einer Zelle zur Zeit t: Rechteck (Fenster-px), Reihen und Phasen (0 … 2π).
type waterParams struct {
	x, top, w, h int
	rows         [riverRows]riverRow
	phase        [3]float64
}

// phaseAt ist die Phase einer Schwingung mit hz Umläufen je Sekunde zur Zeit t, auf 0 … 2π gefaltet (float32 im
// Shader bleibt so auch nach Stunden genau).
func phaseAt(t, hz float64) float64 {
	p := math.Mod(t*hz, 1)
	if p < 0 {
		p++
	}
	return p * 2 * math.Pi
}

// waterLayout rechnet die Uniforms der Zelle zur Zeit t (Sekunden aus Frame.Clock); reine Rechnung.
func waterLayout(c view.Cell, t float64) waterParams {
	p := waterParams{rows: riverLayout(c, t), x: c.Rect.Min.X, w: c.Rect.Dx()}
	p.top = int(math.Floor(p.rows[0].y))
	p.h = c.Rect.Max.Y - p.top
	p.phase = [3]float64{phaseAt(t, waterWobbleHz), phaseAt(t, waterLiftHz), phaseAt(t, waterGlintHz)}
	return p
}

// riverWater hält Shader, Optionen und Uniform-Puffer; die Map zeigt einmal angelegt auf die Arrays, je Zelle werden
// nur deren Werte überschrieben (keine Allokation je Bild). Nicht kopieren.
type riverWater struct {
	src    []byte         // Shader-Quelle; übersetzt wird beim ersten Bedarf (F3 zur Laufzeit)
	tried  bool           // Übersetzen versucht (gelungen oder nicht)
	shader *ebiten.Shader // nil: Kachel-Fassung
	op     ebiten.DrawRectShaderOptions
	origin [2]float32
	rows   [4 * riverRows]float32
	phase  [3]float32
}

// init merkt die Quelle und übersetzt sie sofort, wenn die Shader-Fassung gewählt ist; sonst erst ready beim ersten
// Umschalten.
func (r *riverWater) init(mode riverMode, src []byte) {
	r.src = src
	if mode == riverShader {
		r.build()
	}
	used := riverTiles
	if r.shader != nil {
		used = riverShader
	}
	slog.Info("🚦 Fluss-Fassung", "gewählt", mode.String(), "aktiv", used.String())
}

// build übersetzt die Quelle (einmal) und legt bei Erfolg die Uniforms an.
func (r *riverWater) build() {
	r.tried = true
	if r.shader = compileRiver(r.src); r.shader == nil {
		return
	}
	r.op.Uniforms = map[string]any{
		"Origin": r.origin[:],
		"Rows":   r.rows[:],
		"Phase":  r.phase[:],
		"Base":   rgb(riverBase),
		"Wave":   rgb(riverWave),
		"Shade":  rgb(riverShade),
		"Glint":  rgb(riverGlint),
	}
}

// ready meldet, ob die Shader-Fassung zur Verfügung steht; übersetzt beim ersten Aufruf, falls noch nicht geschehen
// (eine kaputte Quelle meldet compileRiver im Log, der Fluss bleibt bei den Kacheln).
func (r *riverWater) ready() bool {
	if !r.tried {
		r.build()
	}
	return r.shader != nil
}

// compileRiver übersetzt die Shader-Quelle; bei einem Fehler meldet es ihn im Log und liefert nil (Kachel-Fassung).
func compileRiver(src []byte) *ebiten.Shader {
	s, err := ebiten.NewShader(src)
	if err != nil {
		slog.Error("❌ Fluss-Shader übersetzt nicht, Fluss als Kacheln", "err", err)
		return nil
	}
	return s
}

func rgb(c color.RGBA) []float32 {
	return []float32{float32(c.R) / 255, float32(c.G) / 255, float32(c.B) / 255}
}

// set schreibt die Parameter p in die Uniform-Puffer.
func (r *riverWater) set(p *waterParams) {
	r.origin = [2]float32{float32(p.x), float32(p.top)}
	for i, row := range p.rows {
		r.rows[4*i], r.rows[4*i+1], r.rows[4*i+2], r.rows[4*i+3] = float32(row.y), float32(row.h), float32(row.tile), float32(row.x0)
	}
	for i, v := range p.phase {
		r.phase[i] = float32(v)
	}
}

// draw zeichnet den Fluss der Zelle zur Zeit t mit einem Shader-Aufruf.
func (r *riverWater) draw(dst *ebiten.Image, c view.Cell, t float64) {
	p := waterLayout(c, t)
	if p.w <= 0 || p.h <= 0 || p.rows[0].tile <= 0 {
		return
	}
	r.set(&p)
	r.op.GeoM.Reset()
	r.op.GeoM.Translate(float64(p.x), float64(p.top))
	dst.DrawRectShader(p.w, p.h, r.shader, &r.op)
}
