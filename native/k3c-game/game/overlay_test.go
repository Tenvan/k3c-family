package game

import (
	"image"
	"strings"
	"testing"

	"k3c/native/k3c-game/art"
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

// Takt- und Leistungsanzeige mit der Schrift des Spiels (NG13): eine und zwei Zeilen (F2) liegen samt Fläche ganz in
// view.StatsRect, die das HUD frei hält; der Text steht rechtsbündig.
func TestStatsBoxInStatsRect(t *testing.T) {
	font, err := art.LoadFont("../../../public/schrift/k3c.ttf", true)
	if err != nil {
		t.Skipf("Schrift fehlt: %v", err)
	}
	texts := [][2]string{{"T 9999999", "U --- D ---"}, {"9999 FPS", "U99.9 D99.9"}}
	for _, win := range []image.Rectangle{image.Rect(0, 0, 1280, 720), image.Rect(0, 0, 1920, 1080), image.Rect(0, 0, 800, 600)} {
		sr := view.StatsRect(win)
		r := rectF{float64(sr.Min.X), float64(sr.Min.Y), float64(sr.Dx()), float64(sr.Dy())}
		px := statsPx(r)
		for _, tx := range texts {
			var tw [2]float64
			var th float64
			tw[0], th = font.Measure(tx[0], px)
			tw[1], _ = font.Measure(tx[1], px)
			for lines := 1; lines <= 2; lines++ {
				b := statsBox(r, tw, th, lines, px)
				if b.x < r.x || b.y < r.y || b.x+b.w > r.x+r.w+1e-9 || b.y+b.h > r.y+r.h+1e-9 {
					t.Errorf("%v %q, %d Zeilen: Fläche %+v nicht in StatsRect %+v", win, tx, lines, b, r)
				}
			}
		}
	}
}
