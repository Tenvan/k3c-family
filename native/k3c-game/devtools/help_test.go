package devtools

import (
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// AC-01 (B-888): Die Übersicht hat eine Zeile je Taste der Tabelle, mit Taste und Wirkung.
func TestHelpRowsCoverTable(t *testing.T) {
	rows := HelpRows()
	if len(rows) != len(view.DevKeys) {
		t.Fatalf("%d Zeilen, Tabelle %d", len(rows), len(view.DevKeys))
	}
	for i, k := range view.DevKeys {
		if rows[i].Key != k.Label || rows[i].Text == "" {
			t.Errorf("Zeile %d: %+v für %s", i, rows[i], k.Label)
		}
	}
}

// Ohne Schrift und ohne F1 zeichnet die Übersicht ohne Absturz bzw. gar nicht.
func TestHelpDrawWithoutFont(t *testing.T) {
	d := New(nil)[0]
	dst := ebiten.NewImage(320, 180)
	c := view.WindowCell(dst.Bounds())
	f := &view.Frame{}
	d.Draw(dst, f, c)
	f.Dev.Help = true
	d.Draw(dst, f, c)
}
