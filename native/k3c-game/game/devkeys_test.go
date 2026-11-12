package game

import (
	"slices"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-game/view"
)

// keysDown liefert eine Tasten-Abfrage, die genau die Tasten in down als frisch gedrückt meldet.
func keysDown(down ...ebiten.Key) func(ebiten.Key) bool {
	return func(k ebiten.Key) bool { return slices.Contains(down, k) }
}

func devGame(s *fakeSim) *Game {
	g := playGame(s)
	g.frame.Settings = view.DefaultSettings()
	return g
}

// AC-05 (B-816): F3 ändert die Fluss-Fassung in den Einstellungen und kehrt beim zweiten Druck zurück; der Rest der
// Einstellungen bleibt, F2 allein ändert die Fluss-Fassung nicht.
func TestDevKeyRiverToggles(t *testing.T) {
	s := newFakeSim()
	g := devGame(s)
	g.pressed = keysDown(ebiten.KeyF3)
	g.devKeys()
	if len(s.set) != 1 || s.set[0].River == "" || s.set[0].Stats || s.set[0].Music != 1 {
		t.Fatalf("F3: %+v", s.set)
	}
	first := s.set[0].River
	g.frame.Settings = s.set[0]
	g.devKeys()
	if len(s.set) != 2 || s.set[1].River == first || s.set[1].River == "" {
		t.Errorf("zweiter Druck: %+v", s.set)
	}
	g.frame.Settings = s.set[1]
	g.pressed = keysDown(ebiten.KeyF2)
	g.devKeys()
	if len(s.set) != 3 || s.set[2].River != s.set[1].River || !s.set[2].Stats {
		t.Errorf("F2: %+v", s.set)
	}
}

// Ohne Taste, im Aufnahme-Modus und vor dem ersten Frame wird nichts gespeichert; beide Tasten zugleich: ein Speichern.
func TestDevKeyIgnored(t *testing.T) {
	s := newFakeSim()
	g := devGame(s)
	g.pressed = keysDown(ebiten.KeyA)
	g.devKeys()
	g.pressed = keysDown(ebiten.KeyF3, ebiten.KeyF2)
	g.args.Shots = "x"
	g.devKeys()
	g.args.Shots = ""
	g.frame.Settings = view.Settings{}
	g.devKeys()
	if len(s.set) != 0 {
		t.Fatalf("gespeichert: %+v", s.set)
	}
	g.frame.Settings = view.DefaultSettings()
	g.devKeys()
	if len(s.set) != 1 || s.set[0].River == "" || !s.set[0].Stats {
		t.Errorf("beide Tasten: %+v", s.set)
	}
}

func TestApplyDevArgs(t *testing.T) {
	s := newFakeSim()
	g := devGame(s)
	g.applyDevArgs()
	if len(s.set) != 0 {
		t.Fatalf("ohne Schalter gespeichert: %+v", s.set)
	}
	g.args.River, g.args.Stats = view.RiverTiles, true
	g.applyDevArgs()
	if len(s.set) != 1 || s.set[0].River != view.RiverTiles || !s.set[0].Stats {
		t.Errorf("--river --stats: %+v", s.set)
	}
}

func TestParseArgsDevFlags(t *testing.T) {
	a, err := ParseArgs([]string{"--shots", "o", "--river", "tiles", "--stats", "--stall", "--grafik", "leer"}, "s")
	if err != nil || a.River != "tiles" || !a.Stats || !a.Stall || a.Grafik != "leer" {
		t.Fatalf("%+v %v", a, err)
	}
	for _, bad := range [][]string{{"--river", "lava"}, {"--stall"}} {
		if _, err := ParseArgs(bad, "s"); err == nil {
			t.Errorf("%v angenommen", bad)
		}
	}
}
