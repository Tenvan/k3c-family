package sim

import "testing"

// CombatSpotState (B-939): Preis, Zahlstand und Zahlbarkeit je Zahlstelle und Monarch, mit 2 Spielern.
func TestCombatSpotStateJeMonarch(t *testing.T) {
	w, _, right := combatIsland(t, CombatGroups)
	spawnSoldiers(w, 1)
	shield := spotFor(w, "shield", 0)
	p0, p1 := w.Players[0], w.Players[1]
	st := shield.StateFor(w, p0)
	if st.Price != combatCost(w, "shield") || st.Paid != 0 || !st.Payable {
		t.Fatalf("Schild vor Zahlung: %+v", st)
	}
	p0.X, p1.X = shield.X, right
	if !shield.StateFor(w, p0).Near || shield.StateFor(w, p1).Near {
		t.Error("Near folgt der Position des Monarchen nicht")
	}
	w.combat.paid = map[string]int{shield.key(p0): 2}
	if got := shield.StateFor(w, p1).Paid; got != 2 {
		t.Errorf("Schild-Zahlstand gilt für alle Monarchen, bekam %d", got)
	}
	trig := spotFor(w, "trigger", 1)
	if trig.StateFor(w, p0).Payable {
		t.Error("Auslöser ohne Gruppe darf nicht zahlbar sein")
	}
}
