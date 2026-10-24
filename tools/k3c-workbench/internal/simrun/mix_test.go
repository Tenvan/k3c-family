package simrun

import "testing"

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
