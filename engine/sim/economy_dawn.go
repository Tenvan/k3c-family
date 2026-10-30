package sim

// dawnGold ist das Morgengold je Monarch der Stufe: Grundeinkommen plus Zuschlag je Tiefe (tiefere Stufen müssen nach jeder Welle
// wieder aufbauen und liefern selbst keine Münzen, B-645).
func dawnGold(w *World) int {
	return economy.DawnGoldPerPlayer + economy.DawnGoldPerDepth*w.Biome.Depth
}

// payDawnIncome: morgens Steuern für jeden lebenden Monarchen.
func payDawnIncome(w *World) {
	for _, p := range w.Players {
		if isAlive(p) {
			giveGold(w, p, dawnGold(w), p.X)
		}
	}
}
