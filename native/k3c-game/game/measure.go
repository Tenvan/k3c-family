package game

// Messmodus (--measure, NG9.2, B-802/AC-03; Vorbild native/k3c-ebiten/measure.go): Das Spiel läuft im Fenster in
// Echtzeit (Szene aus --seed, --tick, --input), wärmt --warmup Sekunden auf und misst dann --measure Sekunden die echte
// Zeit je gezeichnetem Bild (Hochleistungszähler, clock_windows.go), die Rechenzeit von Draw (CPU, Zeichenbefehle) und
// von Update (Eingabe, Simulation, Listener; 30 Hz). Danach schreibt es den Bericht als JSON und beendet sich.
// Grafikspeicher und Arbeitsspeicher des Prozesses misst native/measure.ps1 von außen und ergänzt den Bericht.

import (
	"encoding/json"
	"fmt"
	"log/slog"
	"math"
	"os"
	"runtime"
	"runtime/debug"
	"slices"
	"strings"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// FrameStats sind Kennwerte der Bildzeiten in Millisekunden.
type FrameStats struct {
	P50 float64 `json:"p50"`
	P95 float64 `json:"p95"`
	P99 float64 `json:"p99"`
	Max float64 `json:"max"`
}

// Report ist der Messbericht (Feldnamen wie native/k3c-ebiten, damit measure.ps1 beide gleich liest).
type Report struct {
	Szene         string     `json:"szene"`
	Spieler       int        `json:"spieler"`
	Ebitengine    string     `json:"ebitengine"`
	Backend       string     `json:"backend"`
	Fenster       [2]int     `json:"fenster"`
	Vsync         bool       `json:"vsync"`
	Sheets        int        `json:"sheets"`
	TexturMibRgba float64    `json:"textur_mib_rgba"`
	RigMibRgba    float64    `json:"rig_mib_rgba"`
	MessdauerS    float64    `json:"messdauer_s"`
	Frames        int        `json:"frames"`
	FpsMittel     float64    `json:"fps_mittel"`
	Fps1PctLow    float64    `json:"fps_1pct_low"`
	FrametimeMs   FrameStats `json:"frametime_ms"`
	RechenzeitMs  float64    `json:"rechenzeit_ms"` // Mittel je Bild: Draw (CPU, Zeichenbefehle)
	TaktMs        float64    `json:"takt_ms"`       // Mittel je Update: Eingabe, Simulation, Listener (30 Hz)
	TaktMaxMs     float64    `json:"takt_max_ms"`
	Takte         int        `json:"takte"`
	TaktEnde      uint64     `json:"takt_ende"` // Frame.Tick am Ende der Messung
	Phase         string     `json:"phase"`     // Tageszeit der Stufe von Spieler 1 am Ende
	Gegner        int        `json:"gegner"`    // lebende Gegner aller Stufen am Ende
	GoHeapMib     float64    `json:"go_heap_mib"`
	GoSysMib      float64    `json:"go_sys_mib"`
}

// meter sammelt die Zeiten; phase 0 = Aufwärmen, 1 = Messen, 2 = fertig.
type meter struct {
	cfg    Measure
	phase  int
	t      float64 // Sekunden in der Phase
	last   float64 // Zeitpunkt des letzten Bilds
	frames []float64
	drawMs float64
	ticks  []float64
}

func newMeter(cfg Measure) *meter { return &meter{cfg: cfg} }

// done meldet, dass der Bericht geschrieben ist (Update beendet das Programm).
func (m *meter) done() bool { return m != nil && m.phase == 2 }

// tick zählt die Rechenzeit eines Updates in Sekunden (nur in der Messung).
func (m *meter) tick(sec float64) {
	if m != nil && m.phase == 1 {
		m.ticks = append(m.ticks, sec*1000)
	}
}

// frame läuft am Ende jedes Draw mit der Uhrzeit now und der Rechenzeit work dieses Draw in Sekunden.
func (m *meter) frame(now, work float64, g *Game) {
	dt := 0.0
	if m.last > 0 {
		dt = now - m.last
	}
	m.last = now
	switch m.phase {
	case 0:
		if m.t += dt; m.t >= m.cfg.Warmup {
			m.phase, m.t = 1, 0
			slog.Info("⏳ Messung beginnt", "takt", g.frame.Tick, "sekunden", m.cfg.Seconds)
		}
	case 1:
		m.frames = append(m.frames, dt*1000)
		m.drawMs += work * 1000
		if m.t += dt; m.t >= m.cfg.Seconds {
			m.phase = 2
			m.finish(g.report(m))
		}
	}
}

// report baut den Bericht aus den gesammelten Zeiten und dem Stand des Spiels.
func (g *Game) report(m *meter) Report {
	sorted := slices.Sorted(slices.Values(m.frames))
	n := max(len(sorted), 1)
	total := sum(sorted)
	worst := sorted[len(sorted)-max(len(sorted)/100, 1):]
	r := Report{
		Szene: sceneName(g.args.Scene), Spieler: len(g.frame.Seats), Ebitengine: ebitenVersion(), Backend: backendName(),
		Fenster: [2]int{g.win.Dx(), g.win.Dy()}, Vsync: !m.cfg.NoVsync, MessdauerS: total / 1000, Frames: len(sorted),
		FpsMittel: float64(len(sorted)) * 1000 / total, Fps1PctLow: 1000 / (sum(worst) / float64(len(worst))),
		FrametimeMs: FrameStats{P50: percentile(sorted, 0.5), P95: percentile(sorted, 0.95), P99: percentile(sorted, 0.99),
			Max: sorted[len(sorted)-1]},
		RechenzeitMs: m.drawMs / float64(n), Takte: len(m.ticks), TaktEnde: g.frame.Tick,
	}
	if len(m.ticks) > 0 {
		r.TaktMs, r.TaktMaxMs = sum(m.ticks)/float64(len(m.ticks)), slices.Max(m.ticks)
	}
	if b, ok := g.parts.Art.(interface {
		Budget() art.Budget
		RigMiB() float64
	}); ok {
		r.Sheets, r.TexturMibRgba, r.RigMibRgba = b.Budget().Images, round1(b.Budget().MiB()), round1(b.RigMiB())
	}
	r.Phase, r.Gegner = worldState(&g.frame)
	var ms runtime.MemStats
	runtime.ReadMemStats(&ms)
	r.GoHeapMib, r.GoSysMib = round1(float64(ms.HeapAlloc)/(1<<20)), round1(float64(ms.Sys)/(1<<20))
	return r
}

// worldState liefert die Tageszeit der Stufe von Spieler 1 und die lebenden Gegner aller Stufen.
func worldState(f *view.Frame) (string, int) {
	phase, n := "", 0
	for _, st := range f.Stages {
		if st.World == nil {
			continue
		}
		if phase == "" {
			phase = st.World.Cycle.Phase
		}
		for _, e := range st.World.Enemies {
			if e.HP > 0 {
				n++
			}
		}
	}
	return phase, n
}

// sceneName beschreibt die gemessene Szene.
func sceneName(s Scene) string {
	return fmt.Sprintf("seed=%s tick=%d spieler=%d", s.Seed, s.Tick, s.Players)
}

// finish schreibt den Bericht nach cfg.Report bzw. auf die Standardausgabe.
func (m *meter) finish(r Report) {
	slog.Info("✅ Messung fertig", "fps", math.Round(r.FpsMittel), "low1", math.Round(r.Fps1PctLow), "p99_ms", r.FrametimeMs.P99,
		"draw_ms", r.RechenzeitMs, "takt_ms", r.TaktMs, "gegner", r.Gegner)
	text, err := json.MarshalIndent(r, "", "  ")
	if err == nil && m.cfg.Report != "" {
		err = os.WriteFile(m.cfg.Report, text, 0o644)
	} else if err == nil {
		fmt.Println(string(text))
	}
	if err != nil {
		slog.Error("❌ Messbericht nicht geschrieben", "datei", m.cfg.Report, "err", err)
		return
	}
	slog.Info("💾 Messbericht", "datei", m.cfg.Report)
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

func round1(v float64) float64 { return math.Round(v*10) / 10 }

// ebitenVersion liest die gebaute Version aus den Modul-Daten der EXE (go.mod legt sie fest).
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

// backendName nennt die Grafik-Bibliothek (DirectX 11, außer EBITENGINE_DIRECTX verlangt 12).
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
