package balance

import (
	"testing"

	"k3c/engine/sim"
)

func TestCheckHelps(t *testing.T) {
	for _, c := range []struct {
		helps, world string
		ok           bool
	}{{"", "", true}, {"", "standard", true}, {"voll", "standard", true}, {"boss", "standard", true}, {"voll", "", false}, {"gibtsnicht", "standard", false}} {
		if err := checkHelps(c.helps, c.world); (err == nil) != c.ok {
			t.Errorf("checkHelps(%q, %q) = %v, ok erwartet: %v", c.helps, c.world, err, c.ok)
		}
	}
}

// Die Hilfen der Seite sind die des Stufen-Tests: Grad dev, Dev-Bau Stufe 5 samt Vorrat; ohne Hilfen bleibt die Insel unberührt.
func TestNewIslandHelps(t *testing.T) {
	base := Scenario{Seed: "k3c-dev", Players: 1, Days: 1, World: "standard"}
	plain, err := newIsland(base)
	if err != nil {
		t.Fatal(err)
	}
	base.Helps = HelpsFull
	aided, err := newIsland(base)
	if err != nil {
		t.Fatal(err)
	}
	if hub := sim.EconomyOf(aided.Stages[0]).HubLevel; hub != 5 {
		t.Errorf("Hub mit Hilfen voll = %d, erwartet 5", hub)
	}
	if hub := sim.EconomyOf(plain.Stages[0]).HubLevel; hub != 1 {
		t.Errorf("Hub ohne Hilfen = %d, erwartet 1", hub)
	}
	if gradeOf(base) != "dev" || gradeOf(Scenario{World: "standard"}) != "" {
		t.Errorf("Grad: mit Hilfen dev, ohne unverändert")
	}
}

func TestMatrixHelpsBrauchtWelt(t *testing.T) {
	m := Matrix{Seeds: []string{"1"}, Players: []int{1}, Bots: []string{"wirtschaft"}, Depths: []int{0}, Days: 1, Helps: HelpsAids}
	if _, err := m.Scenarios(); err == nil {
		t.Error("Hilfen ohne Welt müssen abgelehnt werden")
	}
	m.World = "standard"
	sc, err := m.Scenarios()
	if err != nil || sc[0].Helps != HelpsAids {
		t.Errorf("Szenario = %+v, %v", sc, err)
	}
}
