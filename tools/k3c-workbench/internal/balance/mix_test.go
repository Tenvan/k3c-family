package balance

import (
	"strings"
	"testing"
)

// WZ28.12/AC-13 (B-611): Eine Mischung „a+b“ gibt Spieler 1 das Profil a, Spieler 2 das Profil b.
func TestMischungJeSpieler(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 2, Days: 1})
	if err != nil {
		t.Fatal(err)
	}
	mix, ok := BotByName("walls+passive")
	if !ok {
		t.Fatal("walls+passive unbekannt")
	}
	w := isl.Stages[0]
	p0, p1 := w.Players[0], w.Players[1]
	if mix(w, p0) != bots["walls"](w, p0) || mix(w, p1) != bots["passive"](w, p1) {
		t.Errorf("Mischung steuert nicht je Spieler: %+v %+v", mix(w, p0), mix(w, p1))
	}
}

func TestMischungUngueltig(t *testing.T) {
	for _, bad := range []string{"saver+", "+saver", "saver+raten", "saver+coop2", "coop4+saver", "saver,passive"} {
		if _, ok := BotByName(bad); ok {
			t.Errorf("BotByName(%q): abgelehnt erwartet", bad)
		}
	}
	if err := checkPlayers("saver+passive", 3); err == nil || !strings.Contains(err.Error(), "2 Profile") {
		t.Errorf("Anzahl ≠ Spieler: %v", err)
	}
	if err := checkPlayers("saver+passive+walls", 3); err != nil {
		t.Errorf("drei Profile, drei Spieler: %v", err)
	}
}

// Lauf und Ziele nehmen die Mischung an; ein Lauf mit Mischung ist gültig.
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
	m := Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"saver+passive"}, Depths: []int{0}, Days: 1}
	if err := m.checkBots(); err != nil {
		t.Fatal(err)
	}
	if res := runOne(Scenario{Seed: "1", Players: 2, Bot: "saver+passive", Days: 1}, 0); !res.Valid {
		t.Errorf("Lauf mit Mischung ungültig: %s", res.Error)
	}
}
