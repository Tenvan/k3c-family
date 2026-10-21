package sim

// Vorposten (docs/rules/geplante-chunks.md § 8, B-468): Ein Platz jenseits des äußersten Tors seiner Seite ist ein
// Vorposten. Maßgeblich ist die Lage laut Layout der Stufe (Tor der äußersten Mauerlinie), nicht die äußerste gebaute
// Mauer (outerWall): sonst griffe die Regel auch für Tore und Türme vor der Mauer. Bauern bauen und reparieren dort nur
// ohne Gefahr, Bürger holen dort bei Gefahr keine Waffen; bei Gefahr lassen sie den Auftrag liegen (leaveOnDanger) und
// gehen in die Burg.

// outermostGate ist das X des Tors der äußersten Linie auf Seite side (-1 links, +1 rechts); false ohne Linie dort.
func outermostGate(w *World, side int) (float64, bool) {
	gate, index := 0.0, 0
	for _, l := range w.Level.Lines {
		if l.Side == side && l.Index > index {
			gate, index = l.Gate, l.Index
		}
	}
	return gate, index > 0
}

// outpost: Liegt x jenseits des äußersten Tors seiner Seite?
func outpost(w *World, x float64) bool {
	side := 1
	if x < w.HubX {
		side = -1
	}
	gate, ok := outermostGate(w, side)
	return ok && float64(side)*(x-gate) > 0
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
