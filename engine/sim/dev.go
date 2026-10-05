package sim

import "math"

// Dev-Aktionen (B-178): nur für den Dev-Mode des Raums (engine/room/dev.go), keine Spielregel.

// DevDropGold lässt amount Münzen beim Spieler index in seiner Stufe fallen. Alle liegen bei seinem X, ohne
// rng-Aufruf, damit der Zufallsstrom unberührt bleibt; sie sind wie normale Münzen aufhebbar. false: kein Spieler.
func DevDropGold(isl *Island, index, amount int) bool {
	stage := isl.StageOf(index)
	if stage < 0 {
		return false
	}
	w := isl.Stages[stage]
	for _, p := range w.Players {
		if p.Index != index {
			continue
		}
		for range amount {
			w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: p.X})
		}
	}
	return true
}

// DevAddStock legt Material in den Vorrat der Insel des Spielers index, höchstens bis zum Lager-Maximum; der Rest
// wird verworfen. ok false: unbekannter Rohstoff oder Spieler.
func DevAddStock(isl *Island, index int, resource string, amount int) (taken int, ok bool) {
	stage := isl.StageOf(index)
	if stage < 0 {
		return 0, false
	}
	return addStockCapped(isl.Stages[stage], resource, amount)
}

// DevStartWave startet sofort eine Welle in der Stufe des Spielers index, wie bei Nachtbeginn (B-232). false: kein Spieler.
func DevStartWave(isl *Island, index int) bool {
	stage := isl.StageOf(index)
	if stage < 0 {
		return false
	}
	startWave(isl.Stages[stage])
	return true
}

// DevSetPhase lässt die Zeit aller Stufen vorwärts zum Beginn der nächsten Phase (day, dusk, night) springen (B-232).
// Der Zyklus steht danach kurz vor dem Wechsel, damit der nächste Schritt ihn samt Ereignissen (Welle, Morgen-Einkommen)
// wie sonst auslöst, auch wenn die Stufe schon in dieser Phase war. false: unbekannte Phase.
func DevSetPhase(isl *Island, phase string) bool {
	day := float64(globalDayNight.DayMinutes * 60)
	starts := map[string]float64{"day": 0, "dusk": day, "night": day + float64(globalDayNight.TwilightMinutes*60)}
	start, ok := starts[phase]
	if !ok {
		return false
	}
	length := day + float64((globalDayNight.TwilightMinutes+globalDayNight.NightMinutes)*60)
	for _, w := range isl.Stages {
		now := w.Time * w.CycleSpeed
		target := math.Floor(now/length)*length + start
		if target <= now {
			target += length
		}
		w.Time = target / w.CycleSpeed
		w.Cycle = cycleAt(globalDayNight, target-0.001)
	}
	return true
}
