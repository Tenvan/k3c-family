package balance

import (
	"bytes"
	"encoding/json"
	"maps"
	"slices"
	"strings"
	"testing"

	"k3c/data"
	"k3c/engine/sim"
)

// goldTargets: ein Testziel „Median Gold zur Dämmerung von Tag 1“ (1 Spieler, saver, 1 Tag, Seed 1) mit Obergrenze 105
// ohne Randbreite. Mit den Daten von heute liegt der Wert bei 95; Startgold +25 % hebt ihn über 105 (→ verletzt).
func goldTargets(t *testing.T) *Targets {
	t.Helper()
	measures["testGoldAtDusk1"] = measure{KindMedian, func(m *Metrics) []float64 {
		if len(m.Days) == 0 || m.Days[0].GoldAtDusk == nil {
			return nil
		}
		return []float64{float64(*m.Days[0].GoldAtDusk)}
	}}
	t.Cleanup(func() { delete(measures, "testGoldAtDusk1") })
	return &Targets{Version: 1, Seeds: []string{"1"}, Targets: []Target{{Measure: "testGoldAtDusk1", Name: "Gold Tag 1",
		Kind: KindMedian, Players: 1, Bot: "saver", Days: 1, Upper: f(105), Margin: f(0), Rule: "Test"}}}
}

// AC-03: ±10 % und ±25 % auf purse.startGold; +25 % kippt das Testziel und der Bericht nennt es mit vorher/nachher.
func TestSensitivityNenntGekippteZiele(t *testing.T) {
	rep, err := goldTargets(t).Sensitivity(SensitivityOptions{Paths: []string{"economy.json:purse.startGold"}})
	if err != nil {
		t.Fatal(err)
	}
	if got := len(rep.Variations); got != len(Steps) {
		t.Fatalf("%d Variationen, erwartet %d", got, len(Steps))
	}
	v := rep.Variations[3]
	if v.Percent != 25 || v.Before != 100 || v.After != 125 {
		t.Fatalf("Variation %+v", v)
	}
	if len(v.Flips) != 1 || v.Flips[0].Before != StatusOK || v.Flips[0].After != StatusViolated || v.Flips[0].ValueAfter <= 105 {
		t.Fatalf("gekippt: %+v", v.Flips)
	}
	if md := rep.Markdown(); !strings.Contains(md, "Gold Tag 1 (1 Spieler, saver, Tiefe 0, 1 Tage): ok") {
		t.Errorf("Markdown nennt das gekippte Ziel nicht:\n%s", md)
	}
	if len(rep.Variations[0].Flips) != 0 {
		t.Errorf("−25 %% darf die Obergrenze nicht kippen: %+v", rep.Variations[0].Flips)
	}
}

// Ein unbekannter Pfad oder kein Zahlenwert ist ein Fehler mit Pfad, noch vor dem ersten Lauf.
func TestSensitivityUnbekannterPfad(t *testing.T) {
	ts := goldTargets(t)
	for _, p := range []string{"economy.json:purse.gibtsNicht", "gibtsNicht.json:a", "economy.json:purse", "buildings.json:wall.levels.9.cost.gold", "economy.json"} {
		_, err := ts.Sensitivity(SensitivityOptions{Paths: []string{p}})
		if err == nil || !strings.Contains(err.Error(), p) {
			t.Errorf("%s: Fehler mit Pfad erwartet, war %v", p, err)
		}
	}
}

// AC-04: je Grad aus data/difficulty.json eine Kurve über die Nächte; der Grad wirkt (ultra hat mehr Gegner als dev).
func TestKurvenJeGrad(t *testing.T) {
	var d struct{ Grades map[string]json.RawMessage }
	raw, _ := data.Files.ReadFile("difficulty.json")
	if err := json.Unmarshal(raw, &d); err != nil {
		t.Fatal(err)
	}
	curves, err := Curves([]string{"1"}, 1)
	if err != nil {
		t.Fatal(err)
	}
	var got []string
	for _, c := range curves {
		got = append(got, c.Grade)
		if len(c.Nights) != 1 || c.Invalid != 0 {
			t.Errorf("%s: %+v", c.Grade, c)
		}
	}
	if want := slices.Sorted(maps.Keys(d.Grades)); !slices.Equal(slices.Sorted(slices.Values(got)), want) || got[0] != "dev" {
		t.Errorf("Grade %v, erwartet %v (dev zuerst)", got, want)
	}
	enemies := map[string]int{}
	for _, g := range []string{"dev", "ultra"} {
		r, _ := Run(Matrix{Seeds: []string{"1"}, Players: []int{2}, Bots: []string{"saver"}, Depths: []int{0}, Days: 1, Grade: g})
		enemies[g] = r.Results[0].Metrics.Waves[0].Enemies
	}
	if enemies["ultra"] <= enemies["dev"] {
		t.Errorf("Gegner Welle 1: ultra %d, dev %d", enemies["ultra"], enemies["dev"])
	}
}

// Gleiche Eingaben → byte-gleicher Bericht; danach rechnen Sim und Bots wieder mit den Originaldaten.
func TestSensitivityByteGleichUndZurueckgesetzt(t *testing.T) {
	ts := goldTargets(t)
	o := SensitivityOptions{Paths: []string{"economy.json:purse.startGold", "buildings.json:wall.cost.gold"}, Steps: []int{25}, Nights: 1}
	var out [2][]byte
	for i := range out {
		rep, err := ts.Sensitivity(o)
		if err != nil {
			t.Fatal(err)
		}
		js, err := rep.JSON()
		if err != nil {
			t.Fatal(err)
		}
		out[i] = append(js, rep.Markdown()...)
	}
	if !bytes.Equal(out[0], out[1]) {
		t.Error("Bericht nicht byte-gleich")
	}
	isl, err := sim.CreateIsland("1", []int{0}, 1)
	if err != nil {
		t.Fatal(err)
	}
	orig, err := loadPrices(data.Files)
	if err != nil {
		t.Fatal(err)
	}
	_, startGold, _, _ := variedFS("economy.json:purse.startGold", 0)
	if g := sim.AddIslandPlayer(isl, 0).Gold; float64(g) != startGold || prices.wallGold != orig.wallGold {
		t.Errorf("nicht zurückgesetzt: Startgold %d, Mauerpreis %d statt %d", g, prices.wallGold, orig.wallGold)
	}
}
