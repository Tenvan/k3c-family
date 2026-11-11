package main

// Messmodus: warten, bis alle Bilder geladen und einmal gezeichnet sind, aufwärmen, dann die echte Zeit je gezeichnetem
// Frame (Hochleistungszähler, siehe clock_windows.go) sammeln und einen JSON-Bericht schreiben.
// GPU-Speicher misst native/measure.ps1 von außen (Windows-Zähler) und ergänzt den Bericht.

import (
	"encoding/json"
	"fmt"
	"math"
	"os"
	"runtime/debug"
	"sort"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"
)

type phase int

const (
	phaseLoading phase = iota
	phaseWarmup
	phaseMeasuring
	phaseDone
)

type Measure struct {
	args        Args
	phase       phase
	t           float64
	last        float64
	loadedAfter float64
	framesMs    []float64
	workMs      float64 // Summe der Rechenzeit der gemessenen Frames
}

// RunInfo: was der Bericht vom laufenden Spiel braucht.
type RunInfo struct {
	sheets    int
	bytes     int64
	size      [2]int
	unmanaged bool
	workMs    float64 // Rechenzeit dieses Frames (CPU: Update und Zeichenbefehle)
	rigImages int
	rigBytes  int64
}

type FrameStats struct {
	P50 float64 `json:"p50"`
	P95 float64 `json:"p95"`
	P99 float64 `json:"p99"`
	Max float64 `json:"max"`
}

type Report struct {
	Modus         string     `json:"modus"`
	Art           string     `json:"art"`
	Ebitengine    string     `json:"ebitengine"`
	Adapter       string     `json:"adapter"`
	Backend       string     `json:"backend"`
	Treiber       string     `json:"treiber"`
	Texturen      string     `json:"texturen"`
	Fenster       [2]int     `json:"fenster"`
	Vsync         bool       `json:"vsync"`
	Sheets        int        `json:"sheets"`
	TexturMibRgba float64    `json:"textur_mib_rgba"`
	RigBilder     int        `json:"rig_bilder"`
	RigMibRgba    float64    `json:"rig_mib_rgba"`
	LadezeitS     float64    `json:"ladezeit_s"`
	MessdauerS    float64    `json:"messdauer_s"`
	Frames        int        `json:"frames"`
	FpsMittel     float64    `json:"fps_mittel"`
	Fps1PctLow    float64    `json:"fps_1pct_low"`
	FrametimeMs   FrameStats `json:"frametime_ms"`
	RechenzeitMs  float64    `json:"rechenzeit_ms"` // Mittel je Frame: Update und Zeichenbefehle (CPU)
}

func NewMeasure(args Args) *Measure { return &Measure{args: args} }

func mib(b int64) float64 { return float64(b) / (1024 * 1024) }

// frame läuft einmal je Draw. Ablauf: Laden → Aufwärmen → Messen → Bericht; meldet true, wenn das Programm enden soll.
func (m *Measure) frame(now float64, loaded bool, info RunInfo) bool {
	dt := 0.0
	if m.last > 0 {
		dt = now - m.last
	}
	m.last = now
	switch m.phase {
	case phaseLoading:
		if loaded {
			m.loadedAfter = now - started
			logf("📂 %d Sheets geladen in %.1f s (%.0f MiB RGBA)", info.sheets, m.loadedAfter, mib(info.bytes))
			m.phase, m.t = phaseWarmup, 0
		}
	case phaseWarmup:
		if m.t += dt; m.t >= m.args.warmupS {
			m.phase, m.t = phaseMeasuring, 0
		}
	case phaseMeasuring:
		m.framesMs = append(m.framesMs, dt*1000)
		m.workMs += info.workMs
		if m.t += dt; m.args.measureS > 0 && m.t >= m.args.measureS {
			m.finish(m.build(info))
			m.phase = phaseDone
			return true
		}
	}
	return false
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

func (m *Measure) build(info RunInfo) Report {
	sorted := append([]float64(nil), m.framesMs...)
	sort.Float64s(sorted)
	n := len(sorted)
	total := sum(sorted)
	// 1-%-Low: mittlere FPS der langsamsten 1 % der Frames
	worst := sorted[n-max(n/100, 1):]
	worstMs := sum(worst) / float64(len(worst))
	textures := "atlas"
	if info.unmanaged {
		textures = "unmanaged"
	}
	return Report{
		Modus: m.args.mode, Art: m.args.art, Ebitengine: ebitenVersion(), Adapter: "?", Backend: backendName(), Treiber: "?", Texturen: textures,
		Fenster: info.size, Vsync: m.args.vsync, Sheets: info.sheets, TexturMibRgba: math.Round(mib(info.bytes)*10) / 10,
		RigBilder: info.rigImages, RigMibRgba: math.Round(mib(info.rigBytes)*10) / 10, RechenzeitMs: m.workMs / float64(n),
		LadezeitS: m.loadedAfter, MessdauerS: total / 1000, Frames: n,
		FpsMittel: float64(n) * 1000 / total, Fps1PctLow: 1000 / worstMs,
		FrametimeMs: FrameStats{P50: percentile(sorted, 0.5), P95: percentile(sorted, 0.95), P99: percentile(sorted, 0.99), Max: sorted[n-1]},
	}
}

func (m *Measure) finish(r Report) {
	text, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		fatal("❌ Bericht nicht serialisiert: %v", err)
	}
	logf("✅ Messung %s, Art %s: %.0f FPS Mittel, %.0f FPS 1-%%-Low, p99 %.1f ms, Rechenzeit %.3f ms je Frame", r.Modus, r.Art, r.FpsMittel,
		r.Fps1PctLow, r.FrametimeMs.P99, r.RechenzeitMs)
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
// die Umgebung verlangt mit EBITENGINE_DIRECTX=version=12 ausdrücklich 12. Den Adapter gibt Ebitengine nicht heraus.
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
