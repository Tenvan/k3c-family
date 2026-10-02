package sim

import "math"

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

// depositPoint ist die Abgabestelle für einen Träger bei x: die Burg oder das nächste gebaute Lager. Bei Gleichstand
// gewinnt die Burg, dann das frühere Lager in w.Sites.
func depositPoint(w *World, x float64) float64 {
	best := w.HubX
	for _, s := range w.Sites {
		if s.Kind == "storage" && s.State == "built" && math.Abs(s.X-x) < math.Abs(best-x) {
			best = s.X
		}
	}
	return best
}

// carryToStock bringt das Material zur Abgabestelle und legt höchstens bis zum Maximum ab. Der Rest bleibt im Job,
// der Bauer wartet dort und versucht es jeden Tick erneut. Bei Gefahr geht er zur Burg (wie ohne Insel).
func carryToStock(w *World, t *Troop, dt float64) {
	x := w.HubX
	if !isDangerous(w) {
		x = depositPoint(w, t.X)
	}
	if !walkTo(t, x, dt) {
		return
	}
	taken, ok := addStockCapped(w, t.Job.Resource, t.Job.Amount)
	if ok && taken > 0 {
		w.Events = append(w.Events, Event{"type": "gathered", "resource": t.Job.Resource, "amount": taken})
		t.Job.Amount -= taken
	}
	if !ok || t.Job.Amount <= 0 {
		gatherPressure(w)
		t.Job = nil
		return
	}
	// Maximum voll: Das Material bleibt beim Träger, aber ein wartender Bauplatz oder Bogen geht vor, sonst blockiert
	// ein wartender Träger den Bau, der Platz schaffen würde.
	if j := siteJob(w, t); j != nil {
		t.carried, t.Job = t.Job, j
	}
}

// resourceFull sagt, ob der Rohstoff in einer Insel am Maximum steht (ohne Insel nie).
func resourceFull(w *World, resource string) bool {
	field := stockField(w.Stock, resource)
	limit, capped := capacity(w)
	return field != nil && capped && *field >= limit
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
