package game

import (
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// AC-07 (B-816): Mit Leistungsanzeige stehen Bildrate und Rechenzeit (Update, Draw) im Text; im Aufnahme-Modus feste
// Striche statt Zeiten, damit das Bild vom Takt abhängt.
func TestStatsText(t *testing.T) {
	l1, l2 := statsText(false, true, 59.6, 7, 0.42, 3.25)
	if !strings.Contains(l1, "60 FPS") || !strings.HasPrefix(l2, "U 0.4 D 3.") {
		t.Errorf("Fenster: %q %q", l1, l2)
	}
	if l1, l2 = statsText(false, false, 60, 7, 1, 2); l2 != "" || !strings.Contains(l1, "60 FPS") {
		t.Errorf("aus: %q %q", l1, l2)
	}
	r1, r2 := statsText(true, true, 12, 345, 1, 2)
	if !strings.Contains(r1, "345") || strings.Contains(r1, "FPS") || r2 != "U --- D ---" {
		t.Errorf("Aufnahme: %q %q", r1, r2)
	}
	if a, b := statsText(true, true, 99, 345, 5, 5); a != r1 || b != r2 {
		t.Errorf("Aufnahme hängt von Zeiten ab: %q %q", a, b)
	}
	w, _, _ := view.StatsTextSizeLines(2)
	for _, s := range []string{l1, r1, r2, "U12.3 D45.6"} {
		if len(s)*6+4 > w {
			t.Errorf("%q passt nicht in %d px", s, w)
		}
	}
}

func TestAvg(t *testing.T) {
	var a avg
	if a.take() != 0 {
		t.Error("leer")
	}
	a.add(0.002)
	a.add(0.004)
	if got := a.take(); got < 2.999 || got > 3.001 {
		t.Errorf("Mittel %v", got)
	}
	if a.n != 0 || a.take() != 0 {
		t.Error("nicht zurückgesetzt")
	}
}

// emptySource ist eine Eingabe ohne Geräte.
type emptySource struct{}

func (emptySource) Poll() view.Input { return view.Input{} }

// AC-06 (B-816): Ohne Leistungsanzeige (und im Aufnahme-Modus) fragt die Schleife die Wanduhr nie ab; mit Anzeige misst
// Update zweimal je Takt.
func TestNoClockWhenStatsOff(t *testing.T) {
	calls := 0
	o := &overlay{now: func() float64 { calls++; return float64(calls) * 0.001 }}
	g := playGame(newFakeSim())
	g.stats, g.src = o, emptySource{}
	g.frame.Settings = view.DefaultSettings()
	g.advanceTimed()
	if calls != 0 || o.upd.n != 0 {
		t.Fatalf("Anzeige aus: %d Zeitnahmen", calls)
	}
	g.frame.Settings.Stats, g.frame.Recording = true, true
	g.advanceTimed()
	if calls != 0 {
		t.Fatalf("Aufnahme-Modus: %d Zeitnahmen", calls)
	}
	g.frame.Settings.Stats, g.frame.Recording = true, false
	g.advanceTimed()
	if calls != 2 || o.upd.n != 1 {
		t.Errorf("Anzeige an: %d Zeitnahmen, %d Werte", calls, o.upd.n)
	}
}
