package actors

// Messung Rig gegen Sheet im Fenster (NG3.4, B-780/AC-01 Teil Entwicklungs-PC): zwei Zellen wie bei zwei Spielern,
// zwei Monarchen und n Gegner einer Art, alle in beiden Zellen sichtbar und laufend; n = 2, die größte Welle laut
// data/ und 200. Kein Lasttest und keine Simulation: eine feste Szene, je Fassung ein Lauf. Ebitengine braucht den
// Hauptfaden, deshalb läuft die Messung in TestMain statt der Tests:
//
//	K3C_MEASURE_ACTORS=<bericht.json> go test ./actors/   (im Ordner native/k3c-game)
//
// Neben dem Bericht liegt je Fassung ein Bild der Szene (<bericht>-<figur>-<art>-<n>.png) für die Sichtprüfung.

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

	"k3c/data"
	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

const (
	measureEnv    = "K3C_MEASURE_ACTORS"
	measureWarm   = 60  // Bilder zum Aufwärmen je Fassung
	measureFrames = 300 // gemessene Bilder je Fassung
)

func TestMain(m *testing.M) {
	if out := os.Getenv(measureEnv); out != "" {
		os.Exit(runMeasure(out))
	}
	os.Exit(m.Run())
}

type measureRun struct {
	kind  string
	sheet bool
	n     int
}

type measureRow struct {
	Figur        string  `json:"figur"`
	Art          string  `json:"art"`
	Gegner       int     `json:"gegner"`
	Bilder       int     `json:"bilder"`
	FpsMittel    float64 `json:"fps_mittel"`
	FrameP95Ms   float64 `json:"frametime_p95_ms"`
	RechenzeitMs float64 `json:"rechenzeit_ms"`  // Draw der Figuren beider Zellen je Bild (CPU)
	AllocsBild   float64 `json:"allocs_je_bild"` // ganzer Prozess je Bild (Ebitengine eingeschlossen)
}

type measureReport struct {
	GroessteWelle int          `json:"groesste_welle"`
	Fenster       [2]int       `json:"fenster"`
	Zellen        int          `json:"zellen"`
	Vsync         bool         `json:"vsync"`
	Zeilen        []measureRow `json:"zeilen"`
}

type measureGame struct {
	lib    *art.Library
	runs   []measureRun
	i      int
	d      *drawer
	f      *view.Frame
	frame  int
	last   time.Time
	start  time.Time
	times  []float64
	work   time.Duration
	allocs uint64
	report measureReport
	err    error
	out    string
	shot   *ebiten.Image // Bild der Szene je Fassung, gelesen im nächsten Update
	shotOf string
}

// runMeasure misst alle Fassungen nacheinander und schreibt den Bericht nach out; Rückgabe ist der Exit-Code.
func runMeasure(out string) int {
	lib, err := art.New(art.Config{GrafikDir: repoGrafik, RigDir: repoRigs})
	if err != nil {
		fmt.Println("❌ Grafik:", err)
		return 1
	}
	wave := largestWave()
	g := &measureGame{lib: lib, out: out, shot: ebiten.NewImage(cellW, 2*cellH), report: measureReport{GroessteWelle: wave, Fenster: [2]int{cellW, 2 * cellH}, Zellen: 2}}
	for _, kind := range []string{"goblin", "wolf"} {
		for _, sheet := range []bool{false, true} {
			for _, n := range []int{2, wave, 200} {
				g.runs = append(g.runs, measureRun{kind, sheet, n})
			}
		}
	}
	ebiten.SetVsyncEnabled(false)
	ebiten.SetWindowSize(cellW/2, cellH)
	ebiten.SetWindowTitle("K3C Messung Figuren")
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

func (g *measureGame) Update() error {
	if g.err != nil {
		return g.err
	}
	if g.shotOf != "" {
		g.err, g.shotOf = writeShot(g.shot, g.shotOf), ""
	}
	if g.i >= len(g.runs) {
		return ebiten.Termination
	}
	if g.d == nil {
		g.d, g.f, g.err = measureScene(g.lib, g.runs[g.i])
		g.frame, g.times, g.work = 0, g.times[:0], 0
	}
	return g.err
}

func (g *measureGame) Draw(screen *ebiten.Image) {
	if g.d == nil {
		return
	}
	now := time.Now()
	if g.frame > measureWarm {
		g.times = append(g.times, float64(now.Sub(g.last).Microseconds())/1000)
	}
	g.last = now
	if g.frame == measureWarm-1 {
		g.shot.Fill(color.RGBA{90, 130, 90, 255})
		for _, c := range cells(100) {
			g.d.Draw(g.shot, g.f, c)
		}
		r := g.runs[g.i]
		g.shotOf = fmt.Sprintf("%s-%s-%v-%d.png", strings.TrimSuffix(g.out, ".json"), r.kind, r.sheet, r.n)
	}
	if g.frame == measureWarm {
		g.start, g.allocs = now, mallocs()
	}
	g.d.now += 1.0 / 60
	t0 := time.Now()
	for _, c := range cells(100) {
		g.d.Draw(screen, g.f, c)
	}
	if g.frame >= measureWarm {
		g.work += time.Since(t0)
	}
	g.frame++
	if g.frame == measureWarm+measureFrames {
		g.finish(now)
	}
}

// finish schreibt die Zeile der laufenden Fassung und schaltet zur nächsten.
func (g *measureGame) finish(now time.Time) {
	r := g.runs[g.i]
	way := "rig"
	if r.sheet {
		way = "sheet"
	}
	slices.Sort(g.times)
	row := measureRow{
		Figur: r.kind, Art: way, Gegner: r.n, Bilder: measureFrames,
		FpsMittel:    round2(float64(measureFrames-1) / now.Sub(g.start).Seconds()),
		FrameP95Ms:   round2(g.times[len(g.times)*95/100]),
		RechenzeitMs: round2(float64(g.work.Microseconds()) / 1000 / measureFrames),
		AllocsBild:   round2(float64(mallocs()-g.allocs) / measureFrames),
	}
	g.report.Zeilen = append(g.report.Zeilen, row)
	fmt.Printf("📄 %-6s %-5s %3d Gegner: %7.1f FPS, p95 %5.2f ms, Rechnung %5.2f ms, %5.1f Allokationen je Bild\n",
		row.Figur, row.Art, row.Gegner, row.FpsMittel, row.FrameP95Ms, row.RechenzeitMs, row.AllocsBild)
	g.d, g.i = nil, g.i+1
}

func (*measureGame) Layout(int, int) (int, int) { return cellW, 2 * cellH }

// measureScene baut die feste Szene einer Fassung: zwei Monarchen im Galopp, n Gegner verteilt über die Zelle.
func measureScene(lib *art.Library, r measureRun) (*drawer, *view.Frame, error) {
	d, err := withLib(lib, "gegner/"+r.kind)
	if err != nil {
		return nil, nil, err
	}
	if r.sheet {
		l := d.cat.Enemies[r.kind]
		l.Sheet = true
		d.cat.Enemies[r.kind] = l
	}
	w := &sim.World{Players: []*sim.Player{{ID: 1, Index: 0, X: 100, VX: 5, Facing: 1}, {ID: 2, Index: 1, X: 98, VX: 5, Facing: 1}}}
	for i := range r.n {
		w.Enemies = append(w.Enemies, &sim.Enemy{ID: 10 + i, Kind: r.kind, X: 70 + 60*float64(i)/float64(r.n)})
	}
	f := twoSeats(w)
	d.Listen(f)
	for _, e := range w.Enemies {
		e.X -= 0.1 // alle laufen
	}
	d.Listen(f)
	return d, f, nil
}

// writeShot schreibt das Bild img als PNG nach path.
func writeShot(img *ebiten.Image, path string) error {
	b := img.Bounds()
	rgba := image.NewRGBA(b)
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

// largestWave liest die größte Welle laut data/ bei zwei Spielern: größte Zeile in waves.json (Standard und Elite,
// je Obergrenze), Zuschlag je Zusatzspieler und der größte Faktor der Schwierigkeitsgrade (sim › scaled rundet je
// Gruppe). 0 bei unlesbaren Daten.
func largestWave() int {
	var waves struct {
		Table []struct {
			Standard, Elite [2]int
		} `json:"table"`
		PerExtraPlayer float64 `json:"perExtraPlayer"`
	}
	var diff struct {
		Grades map[string]struct {
			WaveSize float64 `json:"waveSize"`
		} `json:"grades"`
	}
	if readJSON(data.Files, "waves.json", &waves) != nil || readJSON(data.Files, "difficulty.json", &diff) != nil {
		return 0
	}
	grade := 0.0
	for _, g := range diff.Grades {
		grade = math.Max(grade, g.WaveSize)
	}
	size := (1 + waves.PerExtraPlayer) * grade
	most := 0
	for _, r := range waves.Table {
		most = max(most, int(math.Round(float64(r.Standard[1])*size))+int(math.Round(float64(r.Elite[1])*size)))
	}
	return most
}

// Die größte Welle laut data/ bei zwei Spielern (Stand 2026-11: (20 + 5) · 1,25 · 1,6 = 50).
func TestLargestWave(t *testing.T) {
	if n := largestWave(); n < 25 {
		t.Errorf("größte Welle %d", n)
	}
}
