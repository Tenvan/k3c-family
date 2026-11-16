package game

// Messung je Bild (RU0.1, B-1019/AC-01): Der Messmodus zählt GC-Läufe bisher nur je Lauf. Mit --measure-frames <datei>
// merkt sich das Spiel in der Messung je gezeichnetem Bild die Zeit, die Bilddauer, die Rechenzeit von Draw, die Takte
// der Simulation seit dem Bild davor, den Anteil alpha der Zwischenbilder und die GC-Läufe seit dem Bild davor. Der
// Puffer wird vor der Messung angelegt, geschrieben wird erst danach (eine JSON-Zeile je Bild); die GC-Zähler kommen aus
// runtime/metrics, denn runtime.ReadMemStats hält alle Goroutinen an und würde die Messung selbst stören.

import (
	"bufio"
	"fmt"
	"os"
	"runtime/metrics"
	"slices"
)

// frameCap ist die Reserve des Puffers je Sekunde Messdauer (Bilder je Sekunde ohne VSync liegen weit darüber; ein voller
// Puffer zählt weiter, merkt sich aber keine Bilder mehr).
const (
	frameCap    = 1000
	frameCapMax = 2_000_000
)

// longFactor: Ein Bild heißt lang, wenn es länger dauert als das 1,5-Fache des Medians.
const longFactor = 1.5

// frameSample ist ein gemessenes Bild.
type frameSample struct {
	t     float64 // Sekunden seit Beginn der Messung
	dt    float64 // Bilddauer in ms
	draw  float64 // Rechenzeit von Draw in ms
	ticks uint32  // Takte der Simulation seit dem Bild davor
	alpha float32 // Anteil der Zwischenbilder (blend.alpha)
	gc    uint32  // GC-Läufe seit dem Bild davor
}

// FrameReport sind die Kennwerte der Bilder (Bericht, Feld bilder).
type FrameReport struct {
	Datei        string  `json:"datei"`
	Bilder       int     `json:"bilder"`
	Verworfen    int     `json:"verworfen"` // Bilder ohne Platz im Puffer
	MedianMs     float64 `json:"median_ms"`
	P99Ms        float64 `json:"p99_ms"`
	P999Ms       float64 `json:"p999_ms"`
	MaxMs        float64 `json:"max_ms"`
	Lang         int     `json:"lang"`           // Bilder über dem 1,5-Fachen des Medians
	LangMitGc    int     `json:"lang_mit_gc"`    // davon mit GC-Lauf im selben Bild
	LangOhneTakt int     `json:"lang_ohne_takt"` // davon ohne neuen Takt
	LangMehrTakt int     `json:"lang_mehr_takt"` // davon mit zwei oder mehr Takten
	OhneTakt     int     `json:"ohne_takt"`      // alle Bilder ohne neuen Takt
	MehrTakt     int     `json:"mehr_takt"`      // alle Bilder mit zwei oder mehr Takten
	MitGc        int     `json:"mit_gc"`         // alle Bilder mit GC-Lauf
	GcLaeufe     int     `json:"gc_laeufe"`      // GC-Läufe in der Messung (runtime/metrics)
}

// frameLog sammelt die Bilder der Messung.
type frameLog struct {
	path    string
	buf     []frameSample
	dropped int
	probe   [1]metrics.Sample
	gcLast  uint64
	alpha   float64 // von drawGame je Bild gesetzt
}

// newFrameLog legt den Puffer für seconds Sekunden an; ohne Datei gibt es keine Messung je Bild.
func newFrameLog(path string, seconds float64) *frameLog {
	if path == "" {
		return nil
	}
	l := &frameLog{path: path, buf: make([]frameSample, 0, min(int(seconds*frameCap)+frameCap, frameCapMax))}
	l.probe[0].Name = "/gc/cycles/total:gc-cycles"
	return l
}

// gcCycles liest die Zahl der GC-Läufe seit Programmstart (ohne Anhalten, ohne Speicher anzufordern).
func (l *frameLog) gcCycles() uint64 {
	metrics.Read(l.probe[:])
	if l.probe[0].Value.Kind() != metrics.KindUint64 {
		return 0
	}
	return l.probe[0].Value.Uint64()
}

// start merkt den GC-Stand zu Beginn der Messung.
func (l *frameLog) start() {
	if l != nil {
		l.gcLast = l.gcCycles()
	}
}

// add merkt ein Bild: Zeit t seit Beginn der Messung, Bilddauer und Draw in ms, Takte seit dem Bild davor.
func (l *frameLog) add(t, dtMs, drawMs float64, ticks uint64) {
	if l == nil {
		return
	}
	gc := l.gcCycles()
	n := gc - l.gcLast
	l.gcLast = gc
	if len(l.buf) == cap(l.buf) {
		l.dropped++
		return
	}
	l.buf = append(l.buf, frameSample{t: t, dt: dtMs, draw: drawMs, ticks: uint32(ticks), alpha: float32(l.alpha), gc: uint32(n)})
}

// frameReport rechnet die Kennwerte aus den Bildern.
func frameReport(buf []frameSample) FrameReport {
	r := FrameReport{Bilder: len(buf)}
	if len(buf) == 0 {
		return r
	}
	sorted := make([]float64, len(buf))
	for i, s := range buf {
		sorted[i] = s.dt
	}
	slices.Sort(sorted)
	r.MedianMs, r.P99Ms, r.P999Ms = percentile(sorted, 0.5), percentile(sorted, 0.99), percentile(sorted, 0.999)
	r.MaxMs = sorted[len(sorted)-1]
	limit := r.MedianMs * longFactor
	for _, s := range buf {
		long := s.dt > limit
		r.GcLaeufe += int(s.gc)
		countIf(s.gc > 0, long, &r.MitGc, &r.LangMitGc)
		countIf(s.ticks == 0, long, &r.OhneTakt, &r.LangOhneTakt)
		countIf(s.ticks >= 2, long, &r.MehrTakt, &r.LangMehrTakt)
		if long {
			r.Lang++
		}
	}
	return r
}

// countIf zählt all, wenn hit gilt, und zusätzlich long, wenn das Bild lang ist.
func countIf(hit, isLong bool, all, long *int) {
	if !hit {
		return
	}
	*all++
	if isLong {
		*long++
	}
}

// write schreibt je Bild eine JSON-Zeile nach l.path.
func (l *frameLog) write() error {
	f, err := os.Create(l.path)
	if err != nil {
		return err
	}
	w := bufio.NewWriter(f)
	for _, s := range l.buf { // Schreibfehler meldet Flush (bufio merkt sich den ersten)
		_, _ = fmt.Fprintf(w, "{\"t\":%.6f,\"dt_ms\":%.4f,\"draw_ms\":%.4f,\"takte\":%d,\"alpha\":%.4f,\"gc\":%d}\n", s.t, s.dt, s.draw, s.ticks, s.alpha, s.gc)
	}
	if err := w.Flush(); err != nil {
		_ = f.Close()
		return err
	}
	return f.Close()
}

// report schreibt die Datei und liefert die Kennwerte für den Bericht (nil ohne --measure-frames).
func (l *frameLog) report() (*FrameReport, error) {
	if l == nil {
		return nil, nil
	}
	r := frameReport(l.buf)
	r.Datei, r.Verworfen = l.path, l.dropped
	return &r, l.write()
}
