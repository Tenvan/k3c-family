package main

import (
	"runtime"
	"testing"

	"k3c/tools/workkit/winicon"
)

// Das eingebettete PNG wird zu einem gültigen Icon-Handle (Fenster-Icon im Dev-Lauf, ohne Ressource der EXE).
func TestFensterIconAusEingebettetemPNG(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("Fenster-Icon nur unter Windows")
	}
	for _, px := range []int{16, 32} {
		if winicon.Handle(windowIconPNG, px) == 0 {
			t.Errorf("kein Icon der Größe %d aus dem eingebetteten PNG", px)
		}
	}
}
