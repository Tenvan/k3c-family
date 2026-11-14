package world

// Messung Fluss Shader gegen Kacheln im Fenster (NG11.2, B-790/AC-02): 1920 × 1080 px, VSync aus, zwei Zellen wie bei
// zwei Spielern, je Zelle Himmel, Uferstreifen und Fluss. Kein Lasttest und keine Simulation: eine feste Szene, die
// Uhr läuft je Bild 1/60 s weiter, je Fassung ein Lauf. Ebitengine braucht den Hauptfaden, deshalb läuft die Messung
// in TestMain statt der Tests:
//
//	K3C_MEASURE_WORLD=<bericht.json> go test ./world/   (im Ordner native/k3c-game)
//
// Neben dem Bericht liegt je Fassung ein Bild der Szene (<bericht>-<fassung>.png) für die Sichtprüfung.

import (
	"encoding/json"
	"fmt"
	"image"
	"image/color"
	"image/png"
	"math"
	"os"
	"runtime"
	"slices"
	"strings"
	"testing"
	"time"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

const (
	measureEnv    = "K3C_MEASURE_WORLD"
	measureW      = 1920
	measureH      = 1080
	measureWarm   = 120  // Bilder zum Aufwärmen je Fassung
	measureFrames = 3000 // gemessene Bilder je Fassung
)

var measureSky = color.RGBA{142, 202, 230, 255}

func TestMain(m *testing.M) {
	if out := os.Getenv(measureEnv); out != "" {
		os.Exit(runMeasure(out))
	}
	os.Exit(m.Run())
}

type measureRow struct {
	Fassung    string  `json:"fassung"`
	Bilder     int     `json:"bilder"`
	FpsMittel  float64 `json:"fps_mittel"`
	FpsLow1    float64 `json:"fps_1prozent_low"` // Mittel der langsamsten 1 % der Bilder
	FrameP99Ms float64 `json:"frametime_p99_ms"`
	FlussMs    float64 `json:"fluss_draw_ms"`  // Draw des Flusses beider Zellen je Bild (CPU, ohne GPU-Zeit)
	AllocsBild float64 `json:"allocs_je_bild"` // ganzer Prozess je Bild (Ebitengine eingeschlossen)
}

type measureReport struct {
	Fenster [2]int       `json:"fenster"`
	Zellen  int          `json:"zellen"`
	Vsync   bool         `json:"vsync"`
	Zeilen  []measureRow `json:"zeilen"`
}

type measureGame struct {
	modes  []riverMode
	i      int
	w      *World
	f      *view.Frame
	cells  [2]view.Cell
	frame  int
	last   time.Time
	start  time.Time
	times  []float64
	work   time.Duration
	allocs uint64
	report measureReport
	err    error
	out    string
	shot   *ebiten.Image // Bild der Szene je Fassung, geschrieben im nächsten Update
	shotOf string
}

// runMeasure misst beide Fassungen nacheinander und schreibt den Bericht nach out; Rückgabe ist der Exit-Code.
func runMeasure(out string) int {
	g := &measureGame{
		modes: []riverMode{riverTiles, riverShader}, out: out, f: measureFrame(),
		cells:  [2]view.Cell{measureCell(0, 40), measureCell(1, 95.5)},
		times:  make([]float64, 0, measureFrames),
		shot:   ebiten.NewImage(measureW, measureH),
		report: measureReport{Fenster: [2]int{measureW, measureH}, Zellen: 2},
	}
	ebiten.SetVsyncEnabled(false)
	ebiten.SetWindowSize(measureW/2, measureH/2)
	ebiten.SetWindowTitle("K3C Messung Fluss")
	if err := ebiten.RunGame(g); err != nil || g.err != nil {
		fmt.Println("❌ Messung:", err, g.err)
		return 1
	}
	raw, _ := json.MarshalIndent(g.report, "", "  ")
	if err := os.WriteFile(out, append(raw, '\n'), 0o644); err != nil {
		fmt.Println("❌ Bericht:", err)
		return 1
	}
	fmt.Println("💾 Bericht", out)
	return 0
}

// measureFrame ist ein Frame mit zwei Spielern in derselben Stufe (Wald, leere Welt).
func measureFrame() *view.Frame {
	return &view.Frame{
		Stages: []view.Stage{{Index: 0, Level: "forest", World: &sim.World{}}},
		Seats:  []view.Seat{{Player: 0, Monarch: 0, Stage: 0}, {Player: 1, Monarch: 1, Stage: 0}},
	}
}

// measureCell ist Zelle i (oben, unten) des Fensters, 32 px je Unit, Kamera bei x.
func measureCell(i int, x float64) view.Cell {
	c := view.Cell{Index: i, Player: i, Rect: image.Rect(0, i*measureH/2, measureW, (i+1)*measureH/2)}
	c.Camera = view.NewCamera(c.Rect, x, 32)
	return c
}

// measureWorld legt die Welt für eine Fassung an (wie New, ohne Umgebungsvariable).
func measureWorld(mode riverMode) (*World, error) {
	w := &World{looks: loadLooks()}
	w.op.Filter = ebiten.FilterLinear
	w.op.DisableMipmaps = true
	w.water.init(mode, riverKage)
	if mode == riverShader && w.water.shader == nil {
		return nil, fmt.Errorf("fluss-shader übersetzt nicht")
	}
	return w, nil
}

func (g *measureGame) Update() error {
	if g.err != nil {
		return g.err
	}
	if g.shotOf != "" {
		g.err, g.shotOf = writeShot(g.shot, g.shotOf), ""
	}
	if g.i >= len(g.modes) {
		return ebiten.Termination
	}
	if g.w == nil {
		g.w, g.err = measureWorld(g.modes[g.i])
		g.frame, g.times, g.work, g.f.Clock = 0, g.times[:0], 0, 0
	}
	return g.err
}

func (g *measureGame) Draw(screen *ebiten.Image) {
	if g.w == nil {
		return
	}
	now := time.Now()
	if g.frame > measureWarm {
		g.times = append(g.times, float64(now.Sub(g.last).Microseconds())/1000)
	}
	g.last = now
	if g.frame == measureWarm-1 {
		g.drawScene(g.shot)
		g.shotOf = fmt.Sprintf("%s-%s.png", strings.TrimSuffix(g.out, ".json"), g.modes[g.i])
	}
	if g.frame == measureWarm {
		g.start, g.allocs = now, mallocs()
	}
	g.f.Clock += 1.0 / 60
	work := g.drawScene(screen)
	if g.frame >= measureWarm {
		g.work += work
	}
	g.frame++
	if g.frame == measureWarm+measureFrames {
		g.finish(now)
	}
}

// drawScene zeichnet Himmel, Ufer und Fluss beider Zellen; Rückgabe ist die Zeit der Fluss-Draws.
func (g *measureGame) drawScene(dst *ebiten.Image) time.Duration {
	dst.Fill(measureSky)
	var work time.Duration
	for _, c := range g.cells {
		g.w.drawGround(dst, g.f, c)
		t0 := time.Now()
		g.w.drawRiver(dst, g.f, c)
		work += time.Since(t0)
	}
	return work
}

// finish schreibt die Zeile der laufenden Fassung und schaltet zur nächsten.
func (g *measureGame) finish(now time.Time) {
	slices.Sort(g.times)
	n := len(g.times)
	slow := g.times[n-max(1, n/100):]
	sum := 0.0
	for _, v := range slow {
		sum += v
	}
	row := measureRow{
		Fassung: g.modes[g.i].String(), Bilder: measureFrames,
		FpsMittel:  round2(float64(measureFrames-1) / now.Sub(g.start).Seconds()),
		FpsLow1:    round2(1000 / (sum / float64(len(slow)))),
		FrameP99Ms: round2(g.times[n*99/100]),
		FlussMs:    round3(float64(g.work.Nanoseconds()) / 1e6 / measureFrames),
		AllocsBild: round2(float64(mallocs()-g.allocs) / measureFrames),
	}
	g.report.Zeilen = append(g.report.Zeilen, row)
	fmt.Printf("📄 %-6s: %7.1f FPS, 1%%-Low %7.1f FPS, p99 %5.2f ms, Fluss-Draw %6.3f ms, %5.1f Allokationen je Bild\n",
		row.Fassung, row.FpsMittel, row.FpsLow1, row.FrameP99Ms, row.FlussMs, row.AllocsBild)
	g.w, g.i = nil, g.i+1
}

func (*measureGame) Layout(int, int) (int, int) { return measureW, measureH }

// writeShot schreibt das Bild img als PNG nach path.
func writeShot(img *ebiten.Image, path string) error {
	rgba := image.NewRGBA(img.Bounds())
	img.ReadPixels(rgba.Pix)
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	if err := png.Encode(f, rgba); err != nil {
		_ = f.Close()
		return err
	}
	return f.Close()
}

func mallocs() uint64 {
	var m runtime.MemStats
	runtime.ReadMemStats(&m)
	return m.Mallocs
}

func round2(v float64) float64 { return math.Round(v*100) / 100 }

func round3(v float64) float64 { return math.Round(v*1000) / 1000 }
