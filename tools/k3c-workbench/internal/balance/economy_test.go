package balance

import (
	"slices"
	"testing"

	"k3c/engine/sim"
)

// economyMetrics: ein Lauf über 5 Tage (Dämmerung bei Tick 100·n, Tagesbeginn bei 200·(n−1)) mit festen Werten zu
// Tagesbeginn; wood und fighters gelten je Tag ab Tag 1, gold für beide Monarchen.
func economyMetrics(tower, hub2, hub3 *int, gold int, wood, fighters []int) *Metrics {
	m := &Metrics{FirstTowerTick: tower, HubLevel2Tick: hub2, HubLevel3Tick: hub3}
	for d := 1; d <= 6; d++ {
		m.Days = append(m.Days, DayMetrics{Day: d, DuskTick: ptr(200*(d-1) + 100)})
		m.Dawns = append(m.Dawns, DawnMetrics{Day: d, Tick: 200 * (d - 1), Gold: []int{gold, gold + 10},
			Stock: sim.Stock{Wood: wood[d-1]}, Fighters: fighters[d-1]})
	}
	return m
}

// BAL6/AC-01 (B-347): Die Kennzahlen der Wirtschafts-Korridore lesen Tagesbeginn, Turm und Hub-Stufe richtig ab.
func TestWirtschaftsKennzahlen(t *testing.T) {
	wood := []int{100, 50, 100, 120, 100, 10}
	fighters := []int{2, 3, 4, 6, 7, 8}
	ok := economyMetrics(ptr(300), ptr(799), ptr(900), 40, wood, fighters)
	late := economyMetrics(ptr(301), ptr(800), nil, 40, []int{100, 100, 99, 100, 100, 100}, []int{2, 2, 3, 9, 9, 9})
	for _, c := range []struct {
		measure string
		m       *Metrics
		want    []float64
	}{
		{"firstTowerBeforeDusk2", ok, []float64{1}}, {"firstTowerBeforeDusk2", late, []float64{0}},
		{"woodAtDawnFrom3", ok, []float64{1}}, {"woodAtDawnFrom3", late, []float64{0}}, // Tag 6 liegt nach dem Szenario
		{"hubLevel2BeforeDay5", ok, []float64{1}}, {"hubLevel2BeforeDay5", late, []float64{0}},
		{"hubLevel3BeforeDay10", ok, []float64{1}}, {"hubLevel3BeforeDay10", late, []float64{0}}, // Lauf endet vor Tag 10
		{"fightersAtDawn", ok, []float64{1}}, {"fightersAtDawn", late, []float64{0}},
		{"goldAtDawnPerPlayer", ok, []float64{40, 50, 40, 50, 40, 50, 40, 50}}, // Tag 2 bis 5, je Monarch
	} {
		if got := measures[c.measure].values(c.m); !slices.Equal(got, c.want) {
			t.Errorf("%s: %v, erwartet %v", c.measure, got, c.want)
		}
	}
}

// Ein Ziel mit Grad bewertet nur die Läufe dieses Grads; ohne Grad nur die Läufe mit normalem Grad (leer).
func TestZielMitGrad(t *testing.T) {
	g := shareTarget()
	g.Grade = "hard"
	rs := results(metricsWith(true, 1), metricsWith(false, 1))
	rs[0].Grade = "hard"
	ts := &Targets{Seeds: seeds(2), Targets: []Target{g, shareTarget()}}
	v := ts.Evaluate(Report{Results: rs})
	if v[0].Value != 0 || v[1].Value != 100 {
		t.Errorf("Grad hard %v %%, normal %v %%; erwartet 0 und 100", v[0].Value, v[1].Value)
	}
	if err := (Target{Measure: "castleHeld", Name: "x", Rule: "x", Kind: KindShare, Players: 2, Bot: "saver", Days: 1,
		Grade: "raten", Lower: f(1)}).validate(); err == nil {
		t.Error("unbekannter Grad angenommen")
	}
}

// BAL6.1 (B-347): Der Bot wirtschaft baut Turm, Farm und Werkstatt und schafft Kämpfer; der Lauf legt jeden
// Tagesbeginn ab (Tag 1 = Start, zuletzt der Morgen nach dem letzten Tag) mit Gold je Monarch.
func TestWirtschaftsBotUndTagesbeginn(t *testing.T) {
	r := mustRun(t, Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"wirtschaft"}, Depths: []int{0}, Days: 2})
	res := r.Results[0]
	if !res.Valid {
		t.Fatalf("ungültig: %s", res.Error)
	}
	m := res.Metrics
	var days []int
	for _, d := range m.Dawns {
		days = append(days, d.Day)
		if len(d.Gold) != 2 || d.HubLevel < 1 {
			t.Errorf("Tag %d: %+v", d.Day, d)
		}
	}
	if !slices.Equal(days, []int{1, 2, 3}) || m.Dawns[0].Tick != 0 || m.Dawns[0].Fighters != 2 {
		t.Errorf("Tagesbeginne %v, Start %+v", days, m.Dawns[0])
	}
	if m.FirstTowerTick == nil || m.FirstWorkshopTick == nil {
		t.Errorf("kein Turm (%v) oder keine Werkstatt (%v)", m.FirstTowerTick, m.FirstWorkshopTick)
	}
	if m.Dawns[2].Fighters <= m.Dawns[0].Fighters {
		t.Errorf("keine neuen Kämpfer: %d → %d", m.Dawns[0].Fighters, m.Dawns[2].Fighters)
	}
}

// BAL6/AC-01: Jede Zeile der Korridore der Wirtschaft hat ein Ziel, und jedes Ziel bekommt eine Bewertung (nicht
// „ungültig“). Ein Seed genügt für die Aussage „bewertet“; die Werte stehen im Testlog.
func TestAlleWirtschaftsZeilenBewertet(t *testing.T) {
	if testing.Short() {
		t.Skip("langer Test, läuft ohne -short (B-619)")
	}
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	want := []string{"castleHeld", "goldAtDawnPerPlayer", "firstWallBeforeDusk1", "firstTowerBeforeDusk2",
		"woodAtDawnFrom3", "hubLevel2BeforeDay5", "hubLevel3BeforeDay10", "destroyedPerWave", "fightersAtDawn"}
	grades := map[string]bool{}
	for _, g := range ts.Targets {
		want = slices.DeleteFunc(want, func(m string) bool { return m == g.Measure })
		if g.Measure == "castleHeld" {
			grades[g.Grade] = true
		}
	}
	if len(want) > 0 || len(grades) != 5 {
		t.Fatalf("ohne Ziel: %v; Grade für „Burg hält“: %v", want, grades)
	}
	r, err := ts.RunTargets(1)
	if err != nil {
		t.Fatal(err)
	}
	for _, e := range ts.Summarize(r).Targets {
		t.Logf("%s (%s): %v %s", e.Name, scenarioText(e.Target), e.Value, e.Status)
		if e.Status == StatusInvalid {
			t.Errorf("%s (%s): ungültig %v", e.Name, scenarioText(e.Target), e.InvalidSeeds)
		}
	}
}
