package game

import "testing"

// TestParseMeasureLoad prüft die Angabe der Last-Szene (RU0.1): drei Zahlen ab 0, leer = keine.
func TestParseMeasureLoad(t *testing.T) {
	if l, err := parseMeasureLoad(""); err != nil || l != (MeasureLoad{}) {
		t.Errorf("leer: %+v, %v", l, err)
	}
	if l, err := parseMeasureLoad("5,160,120"); err != nil || l != (MeasureLoad{Build: 5, Troops: 160, Enemies: 120}) {
		t.Errorf("5,160,120: %+v, %v", l, err)
	}
	for _, bad := range []string{"5", "5,1", "5,1,2,3", "a,1,2", "5,-1,2"} {
		if _, err := parseMeasureLoad(bad); err == nil {
			t.Errorf("%q: Fehler erwartet", bad)
		}
	}
	total := 0
	for _, s := range loadTroops {
		total += s.percent
	}
	if total != 100 {
		t.Errorf("Anteile der Truppen: %d %%", total)
	}
}
