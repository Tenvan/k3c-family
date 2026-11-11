package main

// Messmodus (`--measure <s> --figuren <n> --art rig|sheet`, B-778/AC-05): n Reiter mit Pferd im Trab, als Rig oder aus
// dem Sheet. Aufwärmen, dann die echte Zeit je Bild sammeln und einen JSON-Bericht schreiben (Felder wie B-777).
// GPU-Speicher und Arbeitsspeicher misst native/measure.ps1 von außen und ergänzt den Bericht.

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"runtime/debug"
	"sort"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/paint"
)

const measureState = "trab"

type FrameStats struct {
	P50 float64 `json:"p50"`
	P95 float64 `json:"p95"`
	P99 float64 `json:"p99"`
	Max float64 `json:"max"`
}

type Report struct {
	Art           string     `json:"art"`
	Figuren       int        `json:"figuren"`
	Zustand       string     `json:"zustand"`
	Ebitengine    string     `json:"ebitengine"`
	Backend       string     `json:"backend"`
	Fenster       [2]int     `json:"fenster"`
	Vsync         bool       `json:"vsync"`
	Bilder        int        `json:"bilder"`
	TexturMibRgba float64    `json:"textur_mib_rgba"`
	MessdauerS    float64    `json:"messdauer_s"`
	Frames        int        `json:"frames"`
	FpsMittel     float64    `json:"fps_mittel"`
	Fps1PctLow    float64    `json:"fps_1pct_low"`
	FrametimeMs   FrameStats `json:"frametime_ms"`
	RechenzeitMs  float64    `json:"rechenzeit_ms"` // mittlere Zeit für Pose, Umhang und Zeichenaufrufe je Bild (CPU)
}

type Measure struct {
	args     Args
	actors   []*paint.Actor
	sheet    *Sheet
	t        float64
	warm     bool
	last     float64
	framesMs []float64
	workMs   float64
}

func NewMeasure(g *Game) (*Measure, error) {
	m := &Measure{args: g.args}
	if g.args.kind == "sheet" {
		if g.sheets == nil {
			return nil, fmt.Errorf("--art sheet braucht die Sheets des Monarchen (erst `task gen:paper`)")
		}
	}
	for i := range g.args.figures {
		a, err := NewActor(g.assets, i%len(variants))
		if err != nil {
			return nil, err
		}
		a.State = states[stateIndex(measureState)]
		a.T = float64(i) * 0.137 // versetzte Phasen, fest statt gewürfelt
		m.actors = append(m.actors, a)
	}
	return m, nil
}

// Update dreht die Figuren weiter; die Rechenzeit zählt nur für die Art, die gemessen wird.
func (m *Measure) Update(g *Game, dt float64) error {
	start := clock()
	for _, a := range m.actors {
		if m.args.kind == "sheet" {
			a.T += dt
			continue
		}
		if err := a.Update(dt); err != nil {
			return err
		}
	}
	if m.warm {
		m.workMs += (clock() - start) * 1000
	}
	return nil
}

// place verteilt die Figuren in einem Raster über das Fenster: Fußpunkt je Figur.
func (m *Measure) place(g *Game, i int) (x, y float64) {
	n := len(m.actors)
	cols := int(math.Ceil(math.Sqrt(float64(n) * float64(g.w) / float64(g.h))))
	rows := int(math.Ceil(float64(n) / float64(cols)))
	cw, ch := float64(g.w)/float64(cols), float64(g.h)/float64(rows)
	return (float64(i%cols) + 0.5) * cw, float64(i/cols+1)*ch - 8
}

func (m *Measure) Draw(g *Game, screen *ebiten.Image) {
	start := clock()
	scale := g.assets.monarch.Density / 2 // Pixeldichte der Sheets: beide Arten zeichnen gleich groß
	for i, a := range m.actors {
		x, y := m.place(g, i)
		if m.args.kind == "sheet" {
			sh, err := g.sheets.sheet(measureState, a.Variant)
			if err != nil {
				fatal("💥 %v", err)
			}
			freq, _ := a.Freq()
			phase := math.Mod(a.T*freq, 1)
			sh.draw(screen, int(phase*float64(len(sh.frames))), x, y, scale, false)
			continue
		}
		g.painter.Dst, g.painter.View = screen, viewAt(x, y, scale, false)
		a.Draw(g.painter)
	}
	now := clock()
	if m.warm {
		m.workMs += (now - start) * 1000
	}
	if m.frame(now) {
		m.finish(m.build(g))
		g.quit = true
	}
}

// frame läuft einmal je Bild: aufwärmen, dann messen; true = Messdauer erreicht.
func (m *Measure) frame(now float64) bool {
	dt := 0.0
	if m.last > 0 {
		dt = now - m.last
	}
	m.last = now
	m.t += dt
	if !m.warm {
		if m.t >= m.args.warmupS {
			m.warm, m.t = true, 0
		}
		return false
	}
	m.framesMs = append(m.framesMs, dt*1000)
	return m.t >= m.args.measureS
}

func percentile(sorted []float64, p float64) float64 {
	if len(sorted) == 0 {
		return 0
	}
	return sorted[int(math.Round(float64(len(sorted)-1)*p))]
}

func sum(v []float64) float64 {
	total := 0.0
	for _, x := range v {
		total += x
	}
	return total
}

func (m *Measure) build(g *Game) Report {
	sorted := append([]float64(nil), m.framesMs...)
	sort.Float64s(sorted)
	n := len(sorted)
	total := sum(sorted)
	worst := sorted[n-max(n/100, 1):] // 1-%-Low: mittlere FPS der langsamsten 1 % der Bilder
	images, bytes := len(g.assets.Images), g.assets.Bytes
	if m.args.kind == "sheet" {
		images, bytes = len(g.sheets.sheets), g.sheets.bytes
	}
	return Report{
		Art: m.args.kind, Figuren: len(m.actors), Zustand: measureState, Ebitengine: ebitenVersion(), Backend: backendName(),
		Fenster: [2]int{g.w, g.h}, Vsync: m.args.vsync, Bilder: images, TexturMibRgba: math.Round(mib(bytes)*100) / 100,
		MessdauerS: total / 1000, Frames: n, FpsMittel: float64(n) * 1000 / total, Fps1PctLow: 1000 / (sum(worst) / float64(len(worst))),
		FrametimeMs:  FrameStats{P50: percentile(sorted, 0.5), P95: percentile(sorted, 0.95), P99: percentile(sorted, 0.99), Max: sorted[n-1]},
		RechenzeitMs: m.workMs / float64(n),
	}
}

func (m *Measure) finish(r Report) {
	text, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		fatal("❌ Bericht nicht serialisiert: %v", err)
	}
	logf("✅ Messung %s × %d: %.0f FPS Mittel, %.0f FPS 1-%%-Low, Rechenzeit %.3f ms je Bild", r.Art, r.Figuren, r.FpsMittel, r.Fps1PctLow, r.RechenzeitMs)
	if m.args.report == "" {
		fmt.Println(string(text))
		return
	}
	if err := os.WriteFile(m.args.report, text, 0o644); err != nil {
		fatal("❌ Bericht nicht geschrieben: %s (%v)", m.args.report, err)
	}
	logf("💾 Bericht: %s", m.args.report)
}

// ebitenVersion liest die gebaute Version aus den Modul-Daten der EXE (go.mod legt sie exakt fest).
func ebitenVersion() string {
	if bi, ok := debug.ReadBuildInfo(); ok {
		for _, dep := range bi.Deps {
			if dep.Path == "github.com/hajimehoshi/ebiten/v2" {
				return strings.TrimPrefix(dep.Version, "v")
			}
		}
	}
	return "?"
}

// backendName nennt die Grafik-Bibliothek. Ebitengine meldet nur „DirectX“; die Version folgt seiner Regel: 11, außer
// die Umgebung verlangt mit EBITENGINE_DIRECTX=version=12 ausdrücklich 12.
func backendName() string {
	var d ebiten.DebugInfo
	ebiten.ReadDebugInfo(&d)
	if d.GraphicsLibrary != ebiten.GraphicsLibraryDirectX {
		return d.GraphicsLibrary.String()
	}
	if strings.Contains(os.Getenv("EBITENGINE_DIRECTX"), "version=12") {
		return "DirectX 12"
	}
	return "DirectX 11"
}
