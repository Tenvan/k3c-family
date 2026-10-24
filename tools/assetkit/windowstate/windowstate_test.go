package windowstate

import (
	"os"
	"path/filepath"
	"testing"
)

func TestLoad(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "sub", "k3c-test.json")
	def := State{Width: DefaultWidth, Height: DefaultHeight}
	if w, ok := Load(path); ok || w != def {
		t.Errorf("fehlende Datei: %+v, %v", w, ok)
	}
	want := State{X: 40, Y: 60, Width: 1400, Height: 900}
	if err := Save(path, want); err != nil {
		t.Fatal(err)
	}
	if w, ok := Load(path); !ok || w != want {
		t.Errorf("Rundlauf: %+v, %v", w, ok)
	}
	neg := State{X: -1200, Y: 40, Width: 1500, Height: 900}
	if err := Save(path, neg); err != nil {
		t.Fatal(err)
	}
	if w, ok := Load(path); !ok || w != neg {
		t.Errorf("negative Position: %+v, %v", w, ok)
	}
	if err := os.WriteFile(path, []byte("{kaputt"), 0o644); err != nil {
		t.Fatal(err)
	}
	if w, ok := Load(path); ok || w != def {
		t.Errorf("kaputte Datei: %+v, %v", w, ok)
	}
	if err := Save(path, State{X: -8, Y: -3000, Width: 10, Height: 20}); err != nil {
		t.Fatal(err)
	}
	if w, _ := Load(path); w != (State{X: -8, Y: -3000, Width: MinWidth, Height: MinHeight}) {
		t.Errorf("Minimum: %+v", w)
	}
}

func TestPlaceRechnetVersatzAus(t *testing.T) {
	px, py := 30, 50 // Position, die Wails nach set(0, 0) meldet (Arbeitsbereich-Versatz)
	var calls [][2]int
	set := func(x, y int) { calls = append(calls, [2]int{x, y}) }
	get := func() (int, int) { return px, py }
	Place(get, set, 200, 300)
	if len(calls) != 2 || calls[0] != [2]int{0, 0} || calls[1] != [2]int{170, 250} {
		t.Errorf("Aufrufe: %v", calls)
	}
}
