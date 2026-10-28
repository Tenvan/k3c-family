package sim

// Vorposten (docs/rules/geplante-chunks.md § 8, B-468): Ein Platz jenseits der äußersten Mauer-Platz-Lage seiner Seite ist
// ein Vorposten. Maßgeblich ist die Lage laut Layout der Stufe (äußerster Mauer-Platz, B-652), nicht die äußerste gebaute
// Mauer (outerWall): sonst griffe die Regel auch für Türme vor der Mauer. Bauern bauen und reparieren dort nur ohne Gefahr,
// Bürger holen dort bei Gefahr keine Waffen; bei Gefahr lassen sie den Auftrag liegen (leaveOnDanger) und gehen in die Burg.

// outermostWall ist das X des äußersten Mauer-Platzes auf Seite side (-1 links, +1 rechts); false ohne Mauer-Platz dort.
func outermostWall(w *World, side int) (float64, bool) {
	best, found := 0.0, false
	for _, l := range w.Level.Sites {
		if l.Kind != "wall" || l.Side != side {
			continue
		}
		if !found || float64(side)*(l.X-best) > 0 {
			best, found = l.X, true
		}
	}
	return best, found
}

// outpost: Liegt x jenseits des äußersten Mauer-Platzes seiner Seite?
func outpost(w *World, x float64) bool {
	side := 1
	if x < w.HubX {
		side = -1
	}
	edge, ok := outermostWall(w, side)
	return ok && float64(side)*(x-edge) > 0
}

// outpostJob: Bau, Reparatur oder Waffen holen an einem Vorposten.
func outpostJob(w *World, j *Job) bool {
	switch j.Type {
	case "build", "repair", "fetchBow", "fetchSword":
		s := siteByID(w, j.SiteID)
		return s != nil && outpost(w, s.X)
	}
	return false
}
