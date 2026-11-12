package game

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

// loadGame ist eine Schleife im Fenster mit einem Loader, der die Aufrufe zählt.
func loadGame() (*Game, *loader) {
	ld := &loader{}
	g := &Game{sim: newFakeSim(), parts: Parts{Art: ld, Assets: []view.AssetSource{assetsOf{"wald": {"a/b"}, "mine": {"c/d"}}}}}
	return g, ld
}

// AC-02 (B-810): Ein Stufenwechsel setzt zuerst „lädt“ und lädt nicht; erst nach einem gezeichneten Bild mit der
// Anzeige lädt das folgende Update, danach ist „lädt“ vorbei.
func TestLoadingBeforeLoad(t *testing.T) {
	g, ld := loadGame()
	g.frame.Stages = stages("wald")
	g.syncLevels()
	if !g.Loading() || len(ld.calls) != 0 {
		t.Fatalf("1. Update: lädt %v, Ladeaufrufe %d", g.Loading(), len(ld.calls))
	}
	g.syncLevels() // Draw hat noch nichts gezeichnet: weiter warten
	if !g.Loading() || len(ld.calls) != 0 {
		t.Fatalf("ohne Bild geladen: %v %d", g.Loading(), len(ld.calls))
	}
	if g.loadMsg == nil || !slices.Contains(g.loadMsg.lines, "Stufe: wald") {
		t.Errorf("Text der Anzeige: %+v", g.loadMsg)
	}
	g.shown = true // Draw hat die Anzeige gezeichnet (drawLoading)
	g.syncLevels()
	if g.Loading() || len(ld.calls) != 1 || !slices.Equal(g.levels, []string{"wald"}) {
		t.Errorf("nach dem Bild: lädt %v, Aufrufe %d, Level %v", g.Loading(), len(ld.calls), g.levels)
	}
	g.syncLevels() // nichts mehr zu tun
	if g.Loading() || len(ld.calls) != 1 {
		t.Errorf("ohne Wechsel: %v %d", g.Loading(), len(ld.calls))
	}
}

// Ein weiterer Stufenwechsel (andere Level) durchläuft die Anzeige erneut.
func TestLoadingAgainOnNextChange(t *testing.T) {
	g, ld := loadGame()
	g.frame.Stages = stages("wald")
	g.syncLevels()
	g.shown = true
	g.syncLevels()
	g.frame.Stages = stages("mine")
	g.syncLevels()
	if !g.Loading() || g.shown || len(ld.calls) != 1 {
		t.Errorf("zweiter Wechsel: lädt %v, gezeichnet %v, Aufrufe %d", g.Loading(), g.shown, len(ld.calls))
	}
}

// Aufnahme- und Messmodus laden sofort (bildgleich wie vor B-810); --stall hält die Anzeige dauerhaft.
func TestLoadingRecordingSynchronous(t *testing.T) {
	g, ld := loadGame()
	g.args.Shots = "x"
	g.frame.Stages = stages("wald")
	g.syncLevels()
	if g.Loading() || len(ld.calls) != 1 {
		t.Errorf("Aufnahme: lädt %v, Aufrufe %d", g.Loading(), len(ld.calls))
	}
	s, sl := loadGame()
	s.args.Shots, s.args.Stall = "x", true
	s.frame.Stages = stages("wald")
	s.syncLevels()
	s.syncLevels()
	if !s.Loading() || len(sl.calls) != 0 {
		t.Errorf("--stall: lädt %v, Aufrufe %d", s.Loading(), len(sl.calls))
	}
	m, ml := loadGame()
	m.args.Measure.Seconds = 5
	m.frame.Stages = stages("wald")
	if m.syncLevels(); m.Loading() || len(ml.calls) != 1 {
		t.Errorf("Messmodus: lädt %v, Aufrufe %d", m.Loading(), len(ml.calls))
	}
}

// Ohne Grafik-Bibliothek (Tests, Platzhalter) gibt es nichts zu laden und keine Anzeige.
func TestLoadingWithoutArt(t *testing.T) {
	g := &Game{}
	g.frame.Stages = stages("wald")
	if g.syncLevels(); g.Loading() {
		t.Error("lädt ohne Bibliothek")
	}
}
