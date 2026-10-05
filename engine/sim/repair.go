package sim

import "math"

// Reparatur (B-112, materialien-gebaeude.md § 4): Bauern reparieren beschädigte, gebaute Gebäude zwischen den Wellen
// kostenlos. Lesart „Anteil der Bauzeit“: Die Reparatur dauert den fehlenden HP-Anteil mal die Bauzeit der Stufe
// (halb kaputt = halbe Bauzeit). Ein zerstörtes Gebäude (destroySite) ist leer und wird neu bezahlt, nicht repariert.

// levelSeconds ist die Bauzeit der aktuellen Stufe eines gebauten Platzes.
func levelSeconds(w *World, s *Site) float64 {
	if levels := buildings[s.Kind].Levels; len(levels) > 0 {
		return levels[levelOf(w, s)-1].BuildSeconds
	}
	return buildings[s.Kind].BuildSeconds
}

// damaged: gebaut, beschädigt, nicht im Ausbau und ohne lebenden Bauer.
func damaged(w *World, s *Site) bool {
	return s.State == "built" && s.HP > 0 && s.HP < s.MaxHP && s.Upgrade == "" && !isWorker(w, s.WorkerID)
}

// repairJob: nächstes beschädigtes Gebäude, nur ohne Gefahr; nil, wenn es keins gibt.
func repairJob(w *World, t *Troop) *Job {
	if isDangerous(w) {
		return nil
	}
	var site *Site
	for _, s := range w.Sites {
		if damaged(w, s) && (site == nil || math.Abs(s.X-t.X) < math.Abs(site.X-t.X)) {
			site = s
		}
	}
	if site == nil {
		return nil
	}
	site.WorkerID = intPtr(t.ID)
	return &Job{Type: "repair", SiteID: site.ID}
}

// repair stellt HP her, ohne Gold und Material, bis MaxHP.
func repair(w *World, t *Troop, dt float64) {
	site := siteByID(w, t.Job.SiteID)
	if site == nil || site.State != "built" || site.Upgrade != "" || site.HP >= site.MaxHP {
		releaseJob(w, t)
		return
	}
	if !walkTo(t, site.X, dt) {
		return
	}
	site.HP = math.Min(site.MaxHP, site.HP+float64(site.MaxHP*dt*workFactor(t, "builder"))/math.Max(0.1, levelSeconds(w, site)))
	if site.HP >= site.MaxHP {
		releaseJob(w, t)
		w.Events = append(w.Events, Event{"type": "repaired", "kind": site.Kind})
	}
}
