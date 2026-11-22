package balance

import (
	"strings"
	"testing"
)

func TestMischungUngueltig(t *testing.T) {
	for _, bad := range []string{"saver+", "+saver", "saver+raten", "saver+coop2", "coop4+saver", "saver,passive"} {
		if KnownBot(bad) {
			t.Errorf("KnownBot(%q): abgelehnt erwartet", bad)
		}
	}
	if err := checkPlayers("saver+passive", 3); err == nil || !strings.Contains(err.Error(), "2 Profile") {
		t.Errorf("Anzahl ≠ Spieler: %v", err)
	}
	if err := checkPlayers("saver+passive+walls", 3); err != nil {
		t.Errorf("drei Profile, drei Spieler: %v", err)
	}
}

// Die Ziele nehmen die Mischung an (den Lauf prüft cmd/k3c-sim mit engine/balance).
func TestMischungImLauf(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	two := 2
	if _, err := ts.Restrict(Scope{Players: &two, Bot: "saver+passive"}); err != nil {
		t.Errorf("Ziele mit Mischung: %v", err)
	}
	three := 3
	if _, err := ts.Restrict(Scope{Players: &three, Bot: "saver+passive"}); err == nil {
		t.Error("Ziele: Mischung mit falscher Spielerzahl angenommen")
	}
	if !KnownBot("walls+passive") {
		t.Error("walls+passive unbekannt")
	}
}
