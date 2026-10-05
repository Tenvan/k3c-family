package sim

// Kaserne und Kämpfer-Limit (B-116, Q29): Es zählen nur Kämpfer, Basis je Hub plus Zuschlag je gebauter Kaserne
// (data/buildings.json › barracks.troopLimit). Geprüft wird nur beim Waffe-Holen (`weaponToFetch`, warrior.go).

// fighters zählt die Kämpfer der Welt: Bogenschützen, Krieger und Elite (Q40); Bauern, Berufe, Landstreicher nicht.
func fighters(w *World) int {
	n := 0
	for _, t := range w.Troops {
		if isFighter(t.Kind) {
			n++
		}
	}
	return n
}

// weaponsInFlight zählt die Bauern außer t, die gerade eine Kämpfer-Waffe holen (Regal oder Boden); sie werden gleich
// Kämpfer.
func weaponsInFlight(w *World, t *Troop) int {
	n := 0
	for _, o := range w.Troops {
		if o != t && o.Job != nil && fetchesFighterWeapon(w, o.Job) {
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
