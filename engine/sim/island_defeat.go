package sim

// Niederlage-Modi der Raum-Optionen (B-102, docs/rules/stufen.md § 4): Fällt die Burg einer Stufe, wirkt der Modus der
// Insel (Options.Defeat). Welten ohne Insel (Campaign, Golden-Läufe) behalten den Stufenverlust.

// defeatLosses wählt die Verluste eines Burgfalls nach dem Niederlage-Modus.
func defeatLosses(w *World) {
	if w.island == nil {
		castleLosses(w)
		return
	}
	switch w.island.Options.Defeat {
	case "resources":
		resourceLosses(w.island)
	case "lost":
		gameOver(w)
	default: // stage
		castleLosses(w)
	}
}

// resourceLosses: Gold/Material-Verlust. Gold aller Spieler der Insel und der Vorrat je −50 %; Bauten und Truppen bleiben.
func resourceLosses(isl *Island) {
	halveStock(isl.Stock)
	for _, p := range isl.Players() {
		p.Gold /= 2
	}
}

// gameOver: Komplett verloren. Die Insel endet (Island.Over), StepIsland rechnet danach nichts mehr, damit Spieler in
// allen Stufen dasselbe Ende sehen. Dass der letzte Spielstand unverändert bleibt, sichert der Raum (B-344).
func gameOver(w *World) {
	w.island.Over = true
	emit(w, "gameOver", Event{})
}

func halveStock(s *Stock) {
	*s = Stock{Wood: s.Wood / 2, Stone: s.Stone / 2, Copper: s.Copper / 2, Iron: s.Iron / 2, Crystal: s.Crystal / 2}
}
