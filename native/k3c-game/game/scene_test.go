package game

import (
	"math"
	"strings"
	"testing"

	"k3c/native/k3c-game/view"
)

// Szenen-Schalter des Aufnahme-Modus (AC-11): Standardwerte, gesetzte Werte und Ablehnung.
func TestParseArgsDefaults(t *testing.T) {
	a, err := ParseArgs(nil, "saves-dir")
	if err != nil {
		t.Fatal(err)
	}
	if a.Saves != "saves-dir" || a.Shots != "" || a.Scene.Direct() || a.Script || a.W != 1280 || a.H != 720 {
		t.Errorf("ohne Schalter: %+v", a)
	}
	a, err = ParseArgs([]string{"--shots", "out"}, "s")
	if err != nil {
		t.Fatal(err)
	}
	if a.Scene.Seed != "k3c" || a.Count != 8 || a.Every != 3 || a.Scene.Players != 2 || !a.Script || len(a.Scene.Script) != 2 {
		t.Errorf("Aufnahme-Standard: %+v", a)
	}
}

func TestParseArgsScene(t *testing.T) {
	a, err := ParseArgs([]string{"--shots", "o", "--load", "wald-1", "--players", "1", "--cam", "120.5", "--tick", "900",
		"--count", "4", "--every", "6", "--size", "1920x1080", "--input", "0@0=RS,0@30=-", "--saves", "x"}, "s")
	if err != nil {
		t.Fatal(err)
	}
	s := a.Scene
	if s.Load != "wald-1" || s.Seed != "" || s.Players != 1 || len(s.Cams) != 1 || s.Cams[0] != 120.5 || s.Tick != 900 {
		t.Errorf("Szene: %+v", s)
	}
	if a.Count != 4 || a.Every != 6 || a.W != 1920 || a.H != 1080 || a.Saves != "x" || len(s.Script) != 2 {
		t.Errorf("Schalter: %+v", a)
	}
	if d := s.Devices(); len(d) != 1 || d[0] != (view.DeviceID{Kind: view.DeviceKeyboard}) {
		t.Errorf("Geräte: %v", d)
	}
	a, err = ParseArgs([]string{"--seed", "x", "--depth", "1", "--cam", "-,40"}, "s")
	if err != nil || a.Scene.Depth != 1 || !math.IsNaN(a.Scene.Cams[0]) || a.Scene.Cams[1] != 40 || a.Script {
		t.Errorf("Kamera folgt / fest: %+v %v", a.Scene, err)
	}
}

func TestParseArgsRejects(t *testing.T) {
	bad := [][]string{
		{"--players", "3"},
		{"--seed", "a", "--load", "b"},
		{"--load", "../boese"},
		{"--tick", "-1"},
		{"--depth", "-2"},
		{"--cam", "1,2,3"},
		{"--cam", "abc"},
		{"--count", "0"},
		{"--every", "0"},
		{"--size", "100x50"},
		{"--size", "groß"},
		{"--players", "1", "--input", "1@0=R"},
		{"--input", "0@0=X"},
		{"--input", "0=R"},
		{"--input", "0@0="},
		{"überzählig"},
	}
	for _, argv := range bad {
		if _, err := ParseArgs(argv, "s"); err == nil {
			t.Errorf("angenommen: %s", strings.Join(argv, " "))
		}
	}
}

// Die feste Folge hält je Spieler den letzten Eintrag bis zum Takt.
func TestCommandsAt(t *testing.T) {
	script, err := ParseScript("0@0=R,1@0=LSPA2,0@10=-,0@10=RL,1@5=A")
	if err != nil {
		t.Fatal(err)
	}
	if c := CommandsAt(script, 0); c[0].Move != 1 || c[1].Move != -1 || !c[1].Sprint || !c[1].Pay || c[1].Skill != 2 {
		t.Errorf("Takt 0: %+v", c)
	}
	if c := CommandsAt(script, 7); c[0].Move != 1 || c[1] != (view.Command{Attack: true}) {
		t.Errorf("Takt 7: %+v", c)
	}
	if c := CommandsAt(script, 10); c[0] != (view.Command{}) {
		t.Errorf("Takt 10, späterer Eintrag gewinnt: %+v", c[0])
	}
	if c := CommandsAt(nil, 3); c != ([view.MaxPlayers]view.Command{}) {
		t.Errorf("ohne Folge: %+v", c)
	}
}

// Die Quelle der festen Folge liefert je Poll einen Takt, gleich für gleiche Szenen.
func TestScriptedSource(t *testing.T) {
	sc := Scene{Players: 2, Script: []Hold{{Tick: 2, Player: 1, Cmd: view.Command{Move: -1}}}}
	a, b := newScripted(sc), newScripted(sc)
	for tick := range 4 {
		ia, ib := a.Poll(), b.Poll()
		if len(ia.Devices) != 2 || ia.Devices[1].Command != ib.Devices[1].Command {
			t.Fatalf("Takt %d: %+v / %+v", tick, ia, ib)
		}
		moving := ia.Devices[1].Command.Move == -1
		if moving != (tick >= 2) {
			t.Errorf("Takt %d: Spieler 2 läuft %v", tick, moving)
		}
		if ia.Devices[1].Device != (view.DeviceID{Kind: view.DeviceKeyboard, Index: 1}) {
			t.Errorf("Gerät %v", ia.Devices[1].Device)
		}
	}
}
