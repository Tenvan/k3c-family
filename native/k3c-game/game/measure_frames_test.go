package game

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// TestFrameReport prüft die Kennwerte der Bilder an einer festen Folge (RU0.1, B-1019/AC-01).
func TestFrameReport(t *testing.T) {
	buf := make([]frameSample, 0, 100)
	for i := range 96 {
		buf = append(buf, frameSample{t: float64(i) / 60, dt: 16.7, ticks: uint32(i % 2)})
	}
	buf = append(buf,
		frameSample{dt: 40, ticks: 2, gc: 1}, // lang, GC, zwei Takte
		frameSample{dt: 30, ticks: 0},        // lang, ohne Takt
		frameSample{dt: 26, ticks: 1},        // lang, sonst nichts
		frameSample{dt: 16.7, ticks: 1, gc: 2},
	)
	r := frameReport(buf)
	if r.Bilder != 100 || r.MedianMs != 16.7 || r.MaxMs != 40 {
		t.Fatalf("Bilder %d, Median %v, Max %v", r.Bilder, r.MedianMs, r.MaxMs)
	}
	if r.Lang != 3 || r.LangMitGc != 1 || r.LangOhneTakt != 1 || r.LangMehrTakt != 1 {
		t.Errorf("lang %d, mit GC %d, ohne Takt %d, mehr Takt %d", r.Lang, r.LangMitGc, r.LangOhneTakt, r.LangMehrTakt)
	}
	if r.MitGc != 2 || r.GcLaeufe != 3 || r.OhneTakt != 49 || r.MehrTakt != 1 {
		t.Errorf("mit GC %d, GC-Läufe %d, ohne Takt %d, mehr Takt %d", r.MitGc, r.GcLaeufe, r.OhneTakt, r.MehrTakt)
	}
	if got := frameReport(nil); got.Bilder != 0 || got.Lang != 0 {
		t.Errorf("leer: %+v", got)
	}
}

// TestFrameLog prüft Puffer, GC-Zähler und Datei: ein voller Puffer verwirft, je Bild entsteht eine Zeile.
func TestFrameLog(t *testing.T) {
	if newFrameLog("", 5) != nil {
		t.Fatal("ohne Datei gibt es keine Messung je Bild")
	}
	var none *frameLog
	none.start()
	none.add(0, 1, 1, 1) // nil bleibt ohne Wirkung
	path := filepath.Join(t.TempDir(), "bilder.jsonl")
	l := newFrameLog(path, 0)
	l.buf = l.buf[:0:2]
	l.start()
	l.alpha = 0.5
	for i := range 3 {
		l.add(float64(i), 16.7, 2, 1)
	}
	r, err := l.report()
	if err != nil || r.Bilder != 2 || r.Verworfen != 1 || r.Datei != path {
		t.Fatalf("Bericht %+v, Fehler %v", r, err)
	}
	text, err := os.ReadFile(path)
	if err != nil {
		t.Fatal(err)
	}
	lines := strings.Split(strings.TrimSpace(string(text)), "\n")
	if len(lines) != 2 || !strings.Contains(lines[0], `"alpha":0.5000`) || !strings.Contains(lines[1], `"takte":1`) {
		t.Errorf("Datei: %q", text)
	}
}
