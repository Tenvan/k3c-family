package sim

// Kaserne und Kämpfer-Limit (B-116, Q29): Es zählen nur Kämpfer, Basis je Hub plus Zuschlag je gebauter Kaserne
// (data/buildings.json › barracks.troopLimit). Geprüft wird nur beim Waffe-Holen (`bowToFetch`).

// fighters zählt die Kämpfer der Welt; heute nur Bogenschützen, W4.3a ergänzt Krieger und Elite (Q40).
func fighters(w *World) int {
	n := 0
	for _, t := range w.Troops {
		if t.Kind == "archer" {
			n++
		}
	}
	return n
}

// bowsInFlight zählt die Bauern außer t, die gerade einen Bogen holen; sie werden gleich Kämpfer.
func bowsInFlight(w *World, t *Troop) int {
	n := 0
	for _, o := range w.Troops {
		if o != t && o.Job != nil && o.Job.Type == "fetchBow" {
			n++
		}
	}
	return n
}

// troopLimit ist das Kämpfer-Limit der Welt: Basis plus Zuschlag je gebauter Kaserne; eine zerstörte zählt nicht.
func troopLimit(w *World) int {
	l := buildings["barracks"].TroopLimit
	n := l.Base
	for _, s := range w.Sites {
		if s.Kind == "barracks" && s.State == "built" {
			n += l.PerBuilding
		}
	}
	return n
}
