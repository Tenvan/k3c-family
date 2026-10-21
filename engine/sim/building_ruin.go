package sim

import "k3c/engine/level"

// Ruinen als Plätze (docs/rules/geplante-chunks.md § 8, B-468): Jede Ruine der Planung (Layout.Ruins) ist ein
// zusätzlicher Platz der Stufe, gebaut mit dem HP-Anteil ihrer Art (level.RuinHP), und wirkt wie ein gebautes Gebäude
// dieser Art. Sie ist Hub-neutral (Hub-Stufe und Mauerlinien zählen sie nicht; für „alles gebaut“ zählt sie wie jeder
// Platz), wird wie jedes Gebäude repariert und zerstört, ist danach erst ab der Hub-Stufe ihrer Art bezahlbar und
// kehrt beim Burgfall (castleLosses) in den Ausgangszustand zurück. Die Herkunft kommt immer aus dem Level (kind@x wie siteLine), damit
// emptySite und destroySite die Kennung Site.Ruin neu setzen; der Spielstand ordnet den Platz über kind@x zu.

// isRuin: Ist kind@x eine Ruine der Planung dieser Stufe?
func isRuin(w *World, kind string, x float64) bool {
	for _, r := range w.Level.Ruins {
		if r.Kind == kind && r.X == x {
			return true
		}
	}
	return false
}

// ruinSite ist der Ausgangszustand eines Ruinen-Platzes: gebaut, Stufe 1, Start-HP der Ruine. Zieht wie emptySite
// eine ID.
func ruinSite(w *World, kind string, x float64) *Site {
	s := emptySite(w, kind, x)
	s.State, s.BuildProgress, s.HP = "built", 1, level.RuinHP(kind)
	return s
}

// addRuinSites legt die Ruinen der Stufe als Plätze an; ohne Planung gibt es keine (Goldens bleiben gleich).
func addRuinSites(w *World) {
	for _, r := range w.Level.Ruins {
		w.Sites = append(w.Sites, ruinSite(w, r.Kind, r.X))
	}
}

// kindHubLevel ist die Hub-Stufe der Art kind: die kleinste der Hub-Plätze dieser Art in hub.json (sites,
// islandSites); 1, wenn die Art dort keinen Platz hat.
func kindHubLevel(w *World, kind string) int {
	h := hubOf(w)
	best := 0
	for _, list := range [][]HubSite{h.Sites, h.IslandSites} {
		for _, s := range list {
			if s.Kind == kind && (best == 0 || s.HubLevel < best) {
				best = s.HubLevel
			}
		}
	}
	return max(1, best)
}

// lineBeforeRuin: Ist das Zahlziel s eine Ruine und steht ein bezahlbarer, unbezahlter Bauplatz der Mauerlinien in
// Zahl-Reichweite von x, geht die Münze an ihn (der nächste), nicht an Wiederaufbau, Produkt oder Ausbau der Ruine.
func lineBeforeRuin(w *World, s *Site, x float64) *Site {
	if s == nil || !s.Ruin {
		return s
	}
	line := func(o *Site) bool {
		_, ok := siteLine(w, o)
		return ok && o.State == "unpaid" && sitePayable(w, o)
	}
	if l := nearest(w.Sites, func(o *Site) float64 { return o.X }, x, economy.PayRangeUnits, line); l != nil {
		return l
	}
	return s
}
