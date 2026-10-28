package balance

import (
	"testing"
)

// WZ28.4/AC-04 (B-596/AC-07): Die neuen Kennzahlen sind in einem kleinen Szenario plausibel gefüllt.
func TestNeueKennzahlenPlausibel(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"1", "2"}, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1})
	f := FiguresOf(r)
	if f == nil || f.Runs != 2 || len(f.Nights) != 1 {
		t.Fatalf("Kennzahlen %+v", f)
	}
	n := f.Nights[0]
	if f.FirstWallTickMedian == nil || *f.FirstWallTickMedian <= 0 {
		t.Error("Zeit bis zur ersten Mauer fehlt")
	}
	if n.Night != 1 || n.Runs != 2 || n.Survival < 0 || n.Survival > 100 { // seit B-652 steht in Nacht 1 oft noch keine Mauer (Freiraum räumen)
		t.Errorf("Überlebensquote %+v", n)
	}
	if n.DayExpense <= 0 { // Einnahmen erst am Morgen von Tag 2, außerhalb des Szenarios
		t.Errorf("Gold-Bilanz je Phase leer: %+v", n)
	}
	if n.ResourceIn.Wood+n.ResourceOut.Wood+n.ResourceIn.Stone+n.ResourceOut.Stone <= 0 {
		t.Errorf("Rohstoff-Fluss leer: %+v / %+v", n.ResourceIn, n.ResourceOut)
	}
	if n.IdleShare <= 0 || n.IdleShare > 100 {
		t.Errorf("Wartezeit-Anteil %v", n.IdleShare)
	}
	checkDays(t, r)
	if FiguresOf(Report{}) != nil {
		t.Error("ohne gültigen Lauf: nil erwartet")
	}
}

// checkDays: Bauern gezählt, Nacht-Anteil des Golds nie größer als der ganze Tag.
func checkDays(t *testing.T, r Report) {
	t.Helper()
	for _, res := range r.Results {
		if d := res.Metrics.Days[0]; d.PeasantTicks == 0 || d.NightIncome > d.Income || d.NightExpense > d.Expense {
			t.Errorf("Seed %s Tag 1: %+v", res.Seed, d)
		}
	}
}

// Ältere Berichte ohne Varianten und Kennzahlen bleiben lesbar.
func TestAlterBerichtLesbar(t *testing.T) {
	s, err := ReadSummary([]byte(`{"version":1,"seeds":3,"pass":true,"targets":[]}`))
	if err != nil || s.Seeds != 3 || s.Varianten != nil || s.Kennzahlen != nil {
		t.Fatalf("%v %+v", err, s)
	}
}
