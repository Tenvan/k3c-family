package sim

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
