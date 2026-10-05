package sim

// Taverne (B-116, Q30): Bei jedem dawn entstehen an jeder gebauten Taverne Landstreicher, solange dort weniger als
// das Maximum stehen; sie sind an die Taverne gebunden (`AnchorX`) und wandern in ihrem Radius
// (data/buildings.json › tavern.vagrants).

func tavernDawn(w *World) {
	v := buildings["tavern"].Vagrants
	for _, s := range w.Sites {
		if s.Kind != "tavern" || s.State != "built" {
			continue
		}
		for range v.PerDawn {
			if vagrantsAt(w, s.X) >= v.Max {
				break
			}
			spawnVagrant(w, s.X, s.X+float64((w.rng.Next()-0.5)*v.WanderUnits))
		}
	}
}

// vagrantsAt zählt die Landstreicher, die an x gebunden sind (Camp oder Taverne).
func vagrantsAt(w *World, x float64) int {
	n := 0
	for _, t := range w.Troops {
		if t.Kind == "vagrant" && t.AnchorX == x {
			n++
		}
	}
	return n
}

// wanderRadius: Landstreicher einer Taverne wandern im Radius der Taverne, alle anderen um ihr Camp.
func wanderRadius(w *World, t *Troop) float64 {
	for _, s := range w.Sites {
		if s.Kind == "tavern" && s.X == t.AnchorX {
			return buildings["tavern"].Vagrants.WanderUnits
		}
	}
	return economy.RecruitCamp.WanderUnits
}
