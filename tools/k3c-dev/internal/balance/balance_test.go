package balance

import (
	"bytes"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// oneDay ist eine kleine Matrix: zwei Seeds, 2 Spieler, beide Profile, Wald, ein Tag (bis zum zweiten Morgen).
var oneDay = Matrix{Seeds: []string{"1", "2"}, Players: []int{2}, Bots: []string{"passive", "saver"}, Depths: []int{0}, Days: 1, GoldThreshold: 150}

func mustRun(t *testing.T, m Matrix) Report {
	t.Helper()
	r, err := Run(m)
	if err != nil {
		t.Fatal(err)
	}
	return r
}

// BAL1/AC-01: gleiche Seeds, Parameter und Daten → byte-gleiche Kennzahlen.
func TestGleicheMatrixGleicheBytes(t *testing.T) {
	a, err := mustRun(t, oneDay).JSON()
	if err != nil {
		t.Fatal(err)
	}
	b, _ := mustRun(t, oneDay).JSON()
	if !bytes.Equal(a, b) {
		t.Fatal("zwei Läufe mit gleicher Matrix liefern verschiedenes JSON")
	}
}

// BAL1/AC-02: passiv zahlt nichts, sparsam zahlt Mauern; beide Monarchen spielen gleichzeitig.
func TestProfileZahlenOderNicht(t *testing.T) {
	r := mustRun(t, oneDay)
	if len(r.Results) != 4 {
		t.Fatalf("%d Läufe, erwartet 4", len(r.Results))
	}
	for _, res := range r.Results {
		if !res.Valid || res.Metrics == nil {
			t.Fatalf("%+v ungültig: %s", res.Scenario, res.Error)
		}
		m := res.Metrics
		switch res.Bot {
		case "passive":
			if m.FirstWallTick != nil {
				t.Errorf("Seed %s: passiv hat eine Mauer gebaut (Tick %d)", res.Seed, *m.FirstWallTick)
			}
		case "saver":
			if m.FirstWallTick == nil {
				t.Errorf("Seed %s: sparsam hat bis Tag 2 keine Mauer gebaut", res.Seed)
			}
		}
	}
}

// Beide Monarchen bezahlen: Mit „sparsam“ geben beide Spieler Gold aus.
func TestSparsamZweiSpielerZahlenBeide(t *testing.T) {
	isl, err := sim.CreateIsland("1", []int{0}, 1)
	if err != nil {
		t.Fatal(err)
	}
	p0, p1 := sim.AddIslandPlayer(isl, 0), sim.AddIslandPlayer(isl, 0)
	start0, start1 := p0.Gold, p1.Gold
	w, cmds := isl.Stages[0], make([]sim.PlayerCommand, 2)
	for range 10 * 60 * 30 { // zehn Minuten: der helle Teil von Tag 1
		cmds[0], cmds[1] = saver(w, p0), saver(w, p1)
		sim.StepIsland(isl, cmds, 1.0/30)
	}
	if p0.Gold >= start0 || p1.Gold >= start1 {
		t.Fatalf("Gold vorher %d/%d, nachher %d/%d: nicht beide haben gezahlt", start0, start1, p0.Gold, p1.Gold)
	}
}

// BAL1/AC-02: Die Kennzahlen aus B-099 sind gefüllt (Tag, Dämmerung, Welle, Fluss, Tiefe, Gold-Schwelle).
func TestKennzahlenGefuellt(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1, GoldThreshold: 150})
	m := r.Results[0].Metrics
	if len(m.Days) != 2 || m.Days[0].DuskTick == nil || m.Days[0].GoldAtDusk == nil || m.Days[0].StockAtDusk == nil {
		t.Fatalf("Tag 1 ohne Dämmerungsstand: %+v", m.Days)
	}
	if m.Days[0].Expense == 0 || m.Days[1].Income == 0 { // Einnahmen: Steuern am Morgen von Tag 2
		t.Errorf("Gold-Fluss leer: Ausgaben Tag 1 %d, Einnahmen Tag 2 %d", m.Days[0].Expense, m.Days[1].Income)
	}
	if len(m.Waves) != 1 || m.Waves[0].EndTick == nil || m.Waves[0].Enemies == 0 || m.Waves[0].TroopsAtStart == 0 {
		t.Errorf("Welle 1 unvollständig: %+v", m.Waves)
	}
	if m.GoldThresholdTick == nil || m.ReachedDepth != 0 {
		t.Errorf("Gold-Schwelle %v, Tiefe %d", m.GoldThresholdTick, m.ReachedDepth)
	}
}

// Ein abgebrochener Lauf erscheint als ungültig mit Seed, der Rest läuft weiter.
func TestUngueltigerLaufMitSeed(t *testing.T) {
	bots["boom"] = func(*sim.World, *sim.Player) sim.PlayerCommand { panic("kaputt") }
	defer delete(bots, "boom")
	r := mustRun(t, Matrix{Seeds: []string{"s1"}, Players: []int{1}, Bots: []string{"boom", "passive"}, Depths: []int{0, 99}, Days: 1})
	if len(r.Results) != 4 {
		t.Fatalf("%d Läufe, erwartet 4", len(r.Results))
	}
	for i, want := range []bool{false, false, true, false} { // boom×0, boom×99, passive×0, passive×99 (Tiefe unbekannt)
		res := r.Results[i]
		if res.Valid != want || res.Seed != "s1" || (!want && res.Error == "") {
			t.Errorf("Lauf %d (%s, Tiefe %d): valid %v, Fehler %q", i, res.Bot, res.Depth, res.Valid, res.Error)
		}
	}
	b, _ := r.JSON()
	if !strings.Contains(string(b), `"error": "abgebrochen: kaputt"`) {
		t.Errorf("Abbruchgrund fehlt im JSON")
	}
}

func TestMatrixPrueft(t *testing.T) {
	for _, m := range []Matrix{
		{Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1},
		{Seeds: []string{"1"}, Players: []int{5}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1},
		{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"gibtsnicht"}, Depths: []int{0}, Days: 1},
		{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}},
	} {
		if _, err := Run(m); err == nil {
			t.Errorf("%+v: Fehler erwartet", m)
		}
	}
}
