package game

import (
	"slices"
	"testing"

	"k3c/native/k3c-game/view"
)

// Menü-Aktionen der Eingabe-Folge: genau im genannten Takt, in Reihenfolge der Folge, der gehaltene Befehl bleibt.
func TestScriptActions(t *testing.T) {
	script, err := ParseScript("0@0=R,0@2=a:skillmenu,0@2=a:right,1@2=a:menu,0@3=a:confirm")
	if err != nil {
		t.Fatal(err)
	}
	src := newScripted(Scene{Players: 2, Script: script})
	want := map[int][2][]view.Action{
		2: {{view.ActionSkillMenu, view.ActionRight}, {view.ActionMenu}},
		3: {{view.ActionConfirm}, nil},
	}
	for tick := range 5 {
		in := src.Poll()
		for p := range 2 {
			if got := in.Devices[p].Actions; !slices.Equal(got, want[tick][p]) {
				t.Errorf("Takt %d, Spieler %d: %v, erwartet %v", tick, p, got, want[tick][p])
			}
		}
		if in.Devices[0].Command.Move != 1 {
			t.Errorf("Takt %d: Befehl %+v verloren", tick, in.Devices[0].Command)
		}
	}
	for _, bad := range []string{"0@1=a:", "0@1=a:sprung", "0@1=a:Down"} {
		if _, err := ParseScript(bad); err == nil {
			t.Errorf("angenommen: %s", bad)
		}
	}
}

// --menu: main, saves und lobby ohne Spiel (Aufnahme ohne Standard-Seed), pause mit Spiel (Seed k3c), options mit
// Szene über der Pause.
func TestParseArgsMenu(t *testing.T) {
	cases := []struct {
		argv   []string
		seed   string
		inGame bool
	}{
		{[]string{"--shots", "o", "--menu", "main"}, "", false},
		{[]string{"--shots", "o", "--menu", "saves", "--seed", "wald"}, "wald", false},
		{[]string{"--shots", "o", "--menu", "pause"}, "k3c", true},
		{[]string{"--menu", "options"}, "", false},
		{[]string{"--shots", "o", "--menu", "options", "--seed", "x"}, "x", true},
	}
	for _, c := range cases {
		a, err := ParseArgs(c.argv, "s")
		if err != nil {
			t.Fatalf("%v: %v", c.argv, err)
		}
		if a.Scene.Seed != c.seed || a.Scene.InGame() != c.inGame || (a.Shots != "" && len(a.Scene.Script) != 0) {
			t.Errorf("%v: Seed %q, im Spiel %v, Folge %v", c.argv, a.Scene.Seed, a.Scene.InGame(), a.Scene.Script)
		}
	}
	if _, err := ParseArgs([]string{"--menu", "credits"}, "s"); err == nil {
		t.Error("unbekanntes Menü angenommen")
	}
}
