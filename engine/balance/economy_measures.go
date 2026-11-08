package balance

import "maps"

// Kennzahlen der Wirtschafts-Korridore (BAL6, B-347; docs/rules/zielkorridore.md › BR1 › Korridore der Wirtschaft).
// Zeitgrenzen hängen an Tag und Phase: „vor Ende der hellen Phase von Tag n“ = bis Dämmerungsbeginn von Tag n,
// „vor Tag n“ = vor dem Tagesbeginn von Tag n (Ereignis dawn). Fehlt ein Tagesbeginn, weil das Szenario kürzer ist,
// zählt bei „vor Tag n“ jeder Zeitpunkt im Lauf, die übrigen Bedingungen gelten dann als nicht erfüllt; die Ziele in
// data/balance-targets.json wählen die Tage passend.

// Schwellen aus der Tabelle der Korridore.
const (
	woodAtDawnMin    = 100 // Holz am Tagesbeginn ≥ 100
	woodFromDay      = 3   // ab Tag 3
	fightersDayEarly = 3   // Kämpfer je Hub zu Beginn von Tag 3 …
	fightersEarly    = 4   // … mindestens 4
	fightersDayLate  = 6   // und zu Beginn von Tag 6 …
	fightersLate     = 8   // … mindestens 8
)

func init() { maps.Copy(measures, economyMeasures) }

// economyMeasures sind die Kennzahlen der Wirtschaft; init nimmt sie in measures (targets.go) auf.
var economyMeasures = map[string]measure{
	// Gold je Monarch zu jedem Tagesbeginn ab Tag 2 innerhalb des Szenarios (Tag 1 ist das Startgold).
	"goldAtDawnPerPlayer": {KindMedian, goldAtDawn},
	// Erster Turm gebaut bis Dämmerungsbeginn von Tag 2.
	"firstTowerBeforeDusk2": {KindShare, func(m *Metrics) []float64 { return []float64{b2f(beforeDusk(m, m.FirstTowerTick, 2))} }},
	// Holz im Vorrat zu jedem Tagesbeginn ab Tag 3 innerhalb des Szenarios mindestens 100.
	"woodAtDawnFrom3": {KindShare, woodAtDawn},
	// Hub-Stufe 2 vor Tag 5, Hub-Stufe 3 vor Tag 10.
	"hubLevel2BeforeDay5":  {KindShare, func(m *Metrics) []float64 { return []float64{b2f(beforeDawn(m, m.HubLevel2Tick, 5))} }},
	"hubLevel3BeforeDay10": {KindShare, func(m *Metrics) []float64 { return []float64{b2f(beforeDawn(m, m.HubLevel3Tick, 10))} }},
	// Kämpfer der Stufe zu Beginn von Tag 3 mindestens 4 und zu Beginn von Tag 6 mindestens 8.
	"fightersAtDawn": {KindShare, func(m *Metrics) []float64 {
		early, late := dawnOf(m, fightersDayEarly), dawnOf(m, fightersDayLate)
		return []float64{b2f(early != nil && late != nil && early.Fighters >= fightersEarly && late.Fighters >= fightersLate)}
	}},
}

func goldAtDawn(m *Metrics) []float64 {
	var v []float64
	for _, d := range m.Dawns {
		if d.Day < 2 || d.Day > lastDay(m) {
			continue
		}
		for _, g := range d.Gold {
			v = append(v, float64(g))
		}
	}
	return v
}

func woodAtDawn(m *Metrics) []float64 {
	seen := false
	for _, d := range m.Dawns {
		if d.Day < woodFromDay || d.Day > lastDay(m) {
			continue
		}
		seen = true
		if d.Stock.Wood < woodAtDawnMin {
			return []float64{0}
		}
	}
	return []float64{b2f(seen)}
}

// lastDay ist der letzte Tag des Szenarios: Ein gültiger Lauf endet mit dem Tagesbeginn danach (letzter Eintrag in
// Dawns); Days hat dafür schon eine Zeile mehr.
func lastDay(m *Metrics) int {
	if len(m.Dawns) == 0 {
		return 0
	}
	return m.Dawns[len(m.Dawns)-1].Day - 1
}

// beforeDusk: Der Zeitpunkt tick liegt spätestens beim Dämmerungsbeginn von Tag day.
func beforeDusk(m *Metrics, tick *int, day int) bool {
	return tick != nil && len(m.Days) >= day && m.Days[day-1].DuskTick != nil && *tick <= *m.Days[day-1].DuskTick
}

// beforeDawn: Der Zeitpunkt tick liegt vor dem Tagesbeginn von Tag day; endet der Lauf vorher, genügt es, dass es ihn
// im Lauf gab (der Lauf endet mit dem Morgen nach dem letzten Tag, also vor Tag day).
func beforeDawn(m *Metrics, tick *int, day int) bool {
	if tick == nil {
		return false
	}
	d := dawnOf(m, day)
	return d == nil || *tick < d.Tick
}

// dawnOf ist der Stand zu Beginn von Tag day, nil ohne diesen Tagesbeginn im Lauf.
func dawnOf(m *Metrics, day int) *DawnMetrics {
	for i := range m.Dawns {
		if m.Dawns[i].Day == day {
			return &m.Dawns[i]
		}
	}
	return nil
}
