package game

import (
	"errors"
	"fmt"
	"strings"
	"testing"

	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// AC-01 (B-810): Fehlende Grafik liefert einen Fehlerzustand mit Text und Pfad, kein stilles Ende: g.err bleibt leer,
// Update beendet nicht, Fault() nennt den Fehler.
func TestMissingGraphicsIsFault(t *testing.T) {
	path := "public/grafik/natur/baum/manifest.json"
	ld := &loader{fail: fmt.Errorf("%w: Manifest %s", art.ErrMissing, path)}
	g := &Game{sim: newFakeSim(), parts: Parts{Art: ld, Assets: []view.AssetSource{assetsOf{"wald": {"natur/baum"}}}}}
	g.frame.Stages = stages("wald")
	g.syncLevels()
	if g.Loading() {
		g.shown = true
		g.syncLevels()
	}
	if g.err != nil || g.Fault() == nil || !errors.Is(g.Fault(), art.ErrMissing) {
		t.Fatalf("Fehlerzustand fehlt: err %v, fault %v", g.err, g.Fault())
	}
	text := strings.Join(g.fault.Lines(), "")
	if !strings.Contains(g.fault.Lines()[0], "Grafik fehlt") || !strings.Contains(strings.ReplaceAll(text, " ", ""), path) {
		t.Errorf("Meldung ohne Titel oder Pfad: %q", g.fault.Lines())
	}
	g.advance() // src ist nil: ohne die Sperre im Fehlerzustand stürzte der Poll ab
}

// Im Messmodus beendet der Fehler das Programm über Update (kein Fenster für Fehler); ein erster Fehler bleibt stehen.
func TestFailMeasureKeepsExit(t *testing.T) {
	g := &Game{}
	g.args.Measure.Seconds = 5
	g.fail(errors.New("a"))
	g.fail(errors.New("b"))
	if g.err == nil || g.err.Error() != "a" || g.Fault() != nil {
		t.Errorf("Messmodus: err %v, fault %v", g.err, g.Fault())
	}
	f := &Game{}
	f.fail(errors.New("erster"))
	f.fail(errors.New("zweiter"))
	if f.Fault() == nil || f.Fault().Error() != "erster" {
		t.Errorf("erster Fehler bleibt: %v", f.Fault())
	}
}

func TestFaultTitleAndHint(t *testing.T) {
	other := NewFault(errors.New("Fenster zu klein"))
	if !strings.HasPrefix(other.Lines()[0], "Fehler") {
		t.Errorf("Titel: %q", other.Lines()[0])
	}
	all := strings.Join(other.Lines(), "\n")
	if !strings.Contains(all, "Fenster zu klein") || !strings.Contains(all, "beendet das Spiel") {
		t.Errorf("Text: %q", all)
	}
}
