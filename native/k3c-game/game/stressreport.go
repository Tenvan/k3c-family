package game

// Messwerte und Bericht des Stresstests (B-896): je Takt Rechenzeit der Simulation (Frame.StepMs), Bildrate und
// Objektzahlen; am Ende reports/native-stress-<Zeit>.json (lesbar über report_read der Workbench) mit Takten je
// Sekunde, Taktdauer p50/p95/max, FPS, Objektzahlen und Speicher.

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"slices"
	"strings"
	"time"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// stressRun ist ein laufender Stresstest.
type stressRun struct {
	cfg       view.StressTest
	start     time.Time
	prelude   bool   // Vorspann (Ausbau, Truppen, Zeitraffer) ausgeführt
	startTick uint64 // Takt beim Vorspann
	waves     int    // gezählte Wellen (Ereignis wave)
	steps     []float64
	fpsSum    float64
	fpsMin    float64
	fpsN      int
	peak      stressObjects
}

// stressObjects sind die Objektzahlen aller Stufen des Frames.
type stressObjects struct {
	Enemies     int `json:"gegner"`
	Troops      int `json:"truppen"`
	Projectiles int `json:"geschosse"`
	Coins       int `json:"muenzen"`
}

// StressReport ist der Bericht eines Stresstests.
type StressReport struct {
	Kind           string          `json:"kind"` // immer "native-stress"
	Config         view.StressTest `json:"einstellungen"`
	Start          string          `json:"start"`
	End            string          `json:"end"`
	Seconds        float64         `json:"dauerSekunden"`
	Cause          string          `json:"endeUrsache"` // wellen, minuten, gewonnen, verloren, verlassen
	Waves          int             `json:"wellen"`
	Ticks          uint64          `json:"takte"`
	TicksPerSecond float64         `json:"takteJeSekunde"`
	StepMs         FrameStats      `json:"taktdauerMs"`
	FpsMean        float64         `json:"fpsMittel"`
	FpsMin         float64         `json:"fpsMin"`
	Peak           stressObjects   `json:"hoechstwerte"`
	End0           stressObjects   `json:"amEnde"`
	HeapMib        float64         `json:"goHeapMib"`
	SysMib         float64         `json:"goSysMib"`
}

func newStressRun(t view.StressTest, now time.Time) *stressRun {
	return &stressRun{cfg: t, start: now, steps: make([]float64, 0, 4096)}
}

// objectsOf zählt die Objekte aller Stufen des Frames.
func objectsOf(f *view.Frame) stressObjects {
	var o stressObjects
	for _, st := range f.Stages {
		if w := st.World; w != nil {
			o.Enemies += len(w.Enemies)
			o.Troops += len(w.Troops)
			o.Projectiles += len(w.Projectiles)
			o.Coins += len(w.Coins)
		}
	}
	return o
}

// sample nimmt die Messwerte eines Takts auf (fps ≤ 0: keine Bildrate, z. B. Tests).
func (r *stressRun) sample(f *view.Frame, fps float64) {
	if f.StepMs > 0 {
		r.steps = append(r.steps, f.StepMs)
	}
	if fps > 0 {
		r.fpsSum, r.fpsN = r.fpsSum+fps, r.fpsN+1
		if r.fpsMin == 0 || fps < r.fpsMin {
			r.fpsMin = fps
		}
	}
	o := objectsOf(f)
	r.peak = stressObjects{max(r.peak.Enemies, o.Enemies), max(r.peak.Troops, o.Troops),
		max(r.peak.Projectiles, o.Projectiles), max(r.peak.Coins, o.Coins)}
}

// over liefert die Ursache, aus der der Test endet ("" = läuft weiter): Spielende, Minuten, oder die letzte
// eingestellte Welle ist geschlagen (keine Gegner mehr).
func (r *stressRun) over(f *view.Frame, now time.Time) string {
	switch {
	case f.Outcome == view.OutcomeWon:
		return "gewonnen"
	case f.Outcome == view.OutcomeLost:
		return "verloren"
	case r.cfg.Minutes > 0 && now.Sub(r.start) >= time.Duration(r.cfg.Minutes)*time.Minute:
		return "minuten"
	case r.cfg.Waves > 0 && r.waves >= r.cfg.Waves && objectsOf(f).Enemies == 0:
		return "wellen"
	}
	return ""
}

// report baut den Bericht zum Frame f am Ende.
func (r *stressRun) report(f *view.Frame, now time.Time, cause string) StressReport {
	secs := now.Sub(r.start).Seconds()
	rep := StressReport{Kind: "native-stress", Config: r.cfg, Start: r.start.Format("2006-01-02T15:04:05"),
		End: now.Format("2006-01-02T15:04:05"), Seconds: round1(secs), Cause: cause, Waves: r.waves, Peak: r.peak,
		End0: objectsOf(f)}
	if f.Tick > r.startTick {
		rep.Ticks = f.Tick - r.startTick
	}
	if secs > 0 {
		rep.TicksPerSecond = round1(float64(rep.Ticks) / secs)
	}
	sorted := slices.Sorted(slices.Values(r.steps))
	if len(sorted) > 0 {
		rep.StepMs = FrameStats{P50: round2(percentile(sorted, 0.50)), P95: round2(percentile(sorted, 0.95)),
			P99: round2(percentile(sorted, 0.99)), Max: round2(sorted[len(sorted)-1])}
	}
	if r.fpsN > 0 {
		rep.FpsMean, rep.FpsMin = round1(r.fpsSum/float64(r.fpsN)), round1(r.fpsMin)
	}
	var ms runtime.MemStats
	runtime.ReadMemStats(&ms)
	rep.HeapMib, rep.SysMib = round1(float64(ms.HeapAlloc)/(1<<20)), round1(float64(ms.Sys)/(1<<20))
	return rep
}

func round2(v float64) float64 { return float64(int64(v*100+0.5)) / 100 }

// writeStressReport schreibt den Bericht nach dir und liefert den Pfad (Doppelpunkte der Uhrzeit als Bindestrich).
func writeStressReport(dir string, r StressReport) (string, error) {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return "", err
	}
	raw, err := json.MarshalIndent(r, "", "  ")
	if err != nil {
		return "", err
	}
	name := strings.ReplaceAll(fmt.Sprintf("native-stress-%s.json", r.End), ":", "-")
	path := filepath.Join(dir, name)
	return path, os.WriteFile(path, raw, 0o644)
}

// fps ist die gemessene Bildrate (Tests setzen fpsOf; im Aufnahme-Modus 0).
func (g *Game) fps() float64 {
	if g.fpsOf != nil {
		return g.fpsOf()
	}
	if g.args.Shots != "" {
		return 0
	}
	return ebiten.ActualFPS()
}
