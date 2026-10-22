//go:build windows

package main

import "testing"

// Das eingebettete PNG wird zu einem gültigen Icon-Handle (Fenster-Icon im Dev-Lauf, ohne Ressource der EXE).
func TestFensterIconAusEingebettetemPNG(t *testing.T) {
	for _, px := range []int{16, 32} {
		if iconOf(px) == 0 {
			t.Errorf("kein Icon der Größe %d aus dem eingebetteten PNG", px)
		}
	}
}
