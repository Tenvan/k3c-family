package sim

// Lager-Maximum der Insel (B-113, materialien-gebaeude.md § 1): je Rohstoff 300 je Hub (Stufe) plus 300 je gebautem
// Lager. Gilt nur, wenn die Welt zu einer Insel gehört; Campaign und einzelne Welten sind unbegrenzt.

// stockField ist der Zähler im Vorrat zu einem Rohstoff, nil bei unbekanntem Namen.
func stockField(s *Stock, resource string) *int {
	switch resource {
	case "wood":
		return &s.Wood
	case "stone":
		return &s.Stone
	case "copper":
		return &s.Copper
	case "iron":
		return &s.Iron
	case "crystal":
		return &s.Crystal
	}
	return nil
}

// capacity ist das Maximum je Rohstoff; ok false = unbegrenzt (keine Insel).
func capacity(w *World) (limit int, ok bool) {
	isl := w.island
	if isl == nil {
		return 0, false
	}
	limit = economy.Storage.BasePerHub * len(isl.Stages)
	for _, st := range isl.Stages {
		for _, s := range st.Sites {
			if s.Kind == "storage" && s.State == "built" {
				limit += economy.Storage.PerStorage
			}
		}
	}
	return limit, true
}

// addStockCapped legt höchstens bis zur Kapazität ab und liefert die aufgenommene Menge. Liegt der Vorrat schon
// darüber (Lager zerstört), wird nichts aufgenommen, der Überschuss bleibt. ok false: kein Baumaterial.
func addStockCapped(w *World, resource string, amount int) (taken int, ok bool) {
	field := stockField(w.Stock, resource)
	if field == nil {
		return 0, false
	}
	taken = amount
	if limit, capped := capacity(w); capped {
		taken = max(0, min(amount, limit-*field))
	}
	*field += taken
	return taken, true
}
