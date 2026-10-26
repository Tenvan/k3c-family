package simrun

import (
	"strings"
	"testing"
)

// WZ28.12/AC-13 (B-611): Die Spec nimmt eine Mischung je Spieler an und lehnt unbekannte Profile darin ab.
func TestSpecMischung(t *testing.T) {
	if err := (Spec{Players: 2, Bot: "saver+passive"}).Check(); err != nil {
		t.Errorf("saver+passive: %v", err)
	}
	for _, bad := range []string{"saver+raten", "saver+coop2", "saver+"} {
		if err := (Spec{Bot: bad}).Check(); err == nil {
			t.Errorf("%q: abgelehnt erwartet", bad)
		}
	}
}

// BS11.4/AC-05 (B-642/AC-05): world und days werden Flags der Balance-CLI; eine unbekannte Welt wird abgelehnt.
func TestSpecWeltUndTage(t *testing.T) {
	ok := Spec{Bot: "coop2", World: "standard", Days: 40}
	if err := ok.Check(); err != nil {
		t.Fatal(err)
	}
	args := strings.Join(ok.args("exe", "dir"), " ")
	for _, want := range []string{"--bot coop2", "--days 40", "--world standard"} {
		if !strings.Contains(args, want) {
			t.Errorf("%q fehlt in %s", want, args)
		}
	}
	for _, bad := range []Spec{{World: "nordmeer"}, {World: "../x"}, {Days: 101}} {
		if err := bad.Check(); err == nil {
			t.Errorf("%+v: Fehler erwartet", bad)
		}
	}
}
