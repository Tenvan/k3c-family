package balance

import (
	"bytes"
	"strings"
	"testing"
)

// BAL3/AC-01: Jedes neue Profil liefert mit gleichem Seed und gleichen Daten byte-gleiche, gültige Kennzahlen.
func TestNeueProfileGleicheBytes(t *testing.T) {
	for _, tc := range []struct {
		bot     string
		players int
	}{{"walls", 2}, {"economy", 2}, {"coop2", 2}, {"coop4", 4}} {
		t.Run(tc.bot, func(t *testing.T) {
			m := Matrix{Seeds: []string{"1"}, Players: []int{tc.players}, Bots: []string{tc.bot}, Depths: []int{0}, Days: 1}
			r := mustRun(t, m)
			if res := r.Results[0]; !res.Valid {
				t.Fatalf("ungültig: %s", res.Error)
			}
			a, err := r.JSON()
			if err != nil {
				t.Fatal(err)
			}
			b, _ := mustRun(t, m).JSON()
			if !bytes.Equal(a, b) {
				t.Fatal("zwei Läufe mit gleichem Seed liefern verschiedenes JSON")
			}
		})
	}
}

// Mauern zuerst baut an Tag 1 eine Mauer; Wirtschaft zuerst wirbt an Tag 1 an und baut keine Mauer.
func TestMauernUndWirtschaftZuerst(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"walls", "economy"}, Depths: []int{0}, Days: 1})
	walls, eco := r.Results[0].Metrics, r.Results[1].Metrics
	if walls.FirstWallTick == nil {
		t.Error("walls: keine Mauer an Tag 1")
	}
	if eco.FirstWallTick != nil && eco.Days[0].DuskTick != nil && *eco.FirstWallTick <= *eco.Days[0].DuskTick {
		t.Errorf("economy: Mauer an Tag 1 (Tick %d)", *eco.FirstWallTick)
	}
	if eco.Days[0].Expense == 0 {
		t.Error("economy: an Tag 1 nichts bezahlt (kein Anwerben)")
	}
}

// B-158 › Ausnahmefälle: Ein Profil mit mehr Spielern als das Szenario ist ein Fehler beim Start.
func TestProfilZuWenigeSpieler(t *testing.T) {
	for _, tc := range []struct {
		bot     string
		players int
	}{{"coop2", 1}, {"coop4", 2}, {"coop4", 3}} {
		_, err := Run(Matrix{Seeds: []string{"1"}, Players: []int{tc.players}, Bots: []string{tc.bot}, Depths: []int{0}, Days: 1})
		if err == nil || !strings.Contains(err.Error(), "braucht") {
			t.Errorf("%s mit %d Spielern: Fehler erwartet, war %v", tc.bot, tc.players, err)
		}
		g := Target{Measure: "castleHeld", Name: "x", Rule: "x", Kind: KindShare, Players: tc.players, Bot: tc.bot, Days: 1, Lower: new(float64)}
		if err := g.validate(); err == nil || !strings.Contains(err.Error(), "braucht") {
			t.Errorf("Ziel %s mit %d Spielern: Fehler erwartet, war %v", tc.bot, tc.players, err)
		}
	}
}
