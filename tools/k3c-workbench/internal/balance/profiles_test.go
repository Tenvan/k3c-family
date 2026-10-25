package balance

import (
	"bytes"
	"strings"
	"testing"

	"k3c/engine/sim"
	"k3c/tools/k3c-workbench/internal/enginetools"
)

// BAL3/AC-01: Jedes neue Profil liefert mit gleichem Seed und gleichen Daten byte-gleiche, gültige Kennzahlen.
func TestNeueProfileGleicheBytes(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	for _, tc := range []struct {
		bot     string
		players int
	}{{"walls", 2}, {"economy", 2}, {"wirtschaft", 2}, {"coop2", 2}, {"coop4", 4}} {
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

// BAL6/AC-02 (B-300/AC-01): „Mauern zuerst“ spielt messbar anders als „sparsam“: Es baut an Tag 1 einen Turm,
// „sparsam“ nie.
func TestMauernZuerstAndersAlsSparsam(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"walls", "saver"}, Depths: []int{0}, Days: 1})
	walls, saver := r.Results[0].Metrics, r.Results[1].Metrics
	if walls.FirstTowerTick == nil {
		t.Error("walls: kein Turm an Tag 1")
	}
	if saver.FirstTowerTick != nil {
		t.Errorf("saver: Turm bei Tick %d", *saver.FirstTowerTick)
	}
}

// B-300/AC-01 (Beschluss BAL6.3, autonom): „Mauern zuerst“ zahlt den Hub-Ausbau und baut danach eine Mauer auf Stufe 2
// aus. Holz und Stein für Mauern, Türme, Hub-Stufe 2 und den Mauer-Ausbau liegen im Vorrat (Dev-Vorrat).
func TestMauernZuerstBautMauerAus(t *testing.T) {
	isl, err := newIsland(Scenario{Seed: "1", Players: 2, Days: 1})
	if err != nil {
		t.Fatal(err)
	}
	for _, r := range []string{"wood", "stone", "copper"} { // Kupfer: Material-Tor des Hub-Ausbaus 1 → 2 (GEB17)
		if _, ok := sim.DevAddStock(isl, 0, r, 200); !ok {
			t.Fatalf("Vorrat %s", r)
		}
	}
	w, cmds := isl.Stages[0], make([]sim.PlayerCommand, 2)
	upgraded, tick := false, 0
	for ; tick < 10*60*enginetools.TickHz && w.Cycle.Phase == "day" && !upgraded; tick++ {
		for _, p := range w.Players {
			cmds[p.Index] = bots["walls"](w, p)
		}
		sim.StepIsland(isl, cmds, 1.0/enginetools.TickHz)
		for _, ev := range w.Events {
			upgraded = upgraded || (ev["type"] == "upgraded" && ev["kind"] == "wall" && ev["level"] == 2)
		}
	}
	if w.HubLevel < 2 || !upgraded {
		t.Errorf("Hub-Stufe %d, Mauer auf Stufe 2 ausgebaut: %v (nach %d Ticks)", w.HubLevel, upgraded, tick)
	}
	t.Logf("Mauer auf Stufe 2 nach %d Ticks", tick)
}
