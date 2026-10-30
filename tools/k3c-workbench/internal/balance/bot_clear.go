package balance

import (
	"math"

	"k3c/engine/sim"
)

// Freiraum (B-652, B-636): Ein Mauer-, Turm- oder Baumschul-Platz wird erst bezahlbar, wenn im Radius keine Bäume und Felsen mehr stehen. Die
// Profile markieren die Blocker des nächsten gesperrten Platzes zum Abbau (Bauern holen sie), bevor sie dort bauen.

// clearNode ist der nächste Baum oder Fels, der einen schon freigeschalteten Platz sperrt und noch nicht markiert ist; nil,
// wenn nichts zu räumen ist oder das Gold für die Markierung fehlt. Gewählt wird der Platz mit den wenigsten Blockern (der
// billigste Freiraum), Mauern vor Türmen, bei Gleichstand der innere.
func clearNode(w *sim.World, p *sim.Player) *sim.ResourceNode {
	site := blockedSite(w)
	if site == nil {
		return nil
	}
	var best *sim.ResourceNode
	for _, n := range sim.SiteBlockers(w, site) {
		np, ok := prices.nodes[n.Kind]
		if !ok || n.Marked || p.Gold < np.MarkCost-n.PaidGold {
			continue
		}
		if best == nil || closer(p.X, n.X, best.X) {
			best = n
		}
	}
	return best
}

// blockedSite: der Mauer-, Turm- oder Baumschul-Platz mit freigeschalteter Hub-Stufe, den noch Bäume oder Felsen sperren, mit dem
// niedrigsten rank; nil, wenn keiner gesperrt ist.
func blockedSite(w *sim.World) *sim.Site {
	var site *sim.Site
	for _, s := range w.Sites {
		if (s.Kind != "wall" && s.Kind != "tower" && s.Kind != "nursery") || s.State != "unpaid" || sim.SiteHubLevel(w, s) > w.HubLevel {
			continue
		}
		if len(sim.SiteBlockers(w, s)) > 0 && wanted(w, s) && (site == nil || rank(w, s) < rank(w, site)) {
			site = s
		}
	}
	return site
}

// rank sortiert Plätze: Mauern vor Türmen vor Baumschulen, dann nach der Zahl der Blocker, dann nach Abstand zur Burg.
func rank(w *sim.World, s *sim.Site) float64 {
	d := math.Abs(s.X - w.HubX)
	d += 1000 * float64(len(sim.SiteBlockers(w, s)))
	switch s.Kind {
	case "tower":
		d += 1e6
	case "nursery":
		d += 2e6
	}
	return d
}

// wanted: Der Bot baut je Seite, Zone und Art (Mauer, Turm) nur einen Platz, den billigsten (innersten); weitere Plätze der
// Zone lässt er stehen, sonst verbrennt er das Startgold an verstreuten Mauern statt an Bauern und Bögen (B-652). Plätze,
// die nicht aus den Zonen stammen, sind immer gewollt.
func wanted(w *sim.World, s *sim.Site) bool {
	z, ok := sim.SiteZone(w, s)
	if !ok {
		return true
	}
	for _, o := range w.Sites {
		if o != s && o.Kind == s.Kind && o.State != "unpaid" && (o.X < w.HubX) == (s.X < w.HubX) {
			if oz, ok := sim.SiteZone(w, o); ok && oz == z {
				return false
			}
		}
	}
	return true
}

// lairSafeUnits ist der Abstand zum Bau des Endbosses, den der Bot beim Markieren und Sammeln hält: Wer dort hinläuft, weckt den
// Endboss (lair.triggerUnits 15) und seine Beschwörung räumt das Heer ab (B-652: Kristall-Knoten am Bau, Tag 27).
const lairSafeUnits = 60.0

// nearLair: x liegt nahe am Bau des Endbosses dieser Stufe, solange das Heer ihn nicht tragen kann (< endbossWant Kämpfer).
func nearLair(w *sim.World, x float64) bool {
	if sim.EconomyOf(w).Fighters >= endbossWant {
		return false
	}
	for _, e := range w.Level.Entities {
		if e.Kind == "lair" && math.Abs(e.X-x) < lairSafeUnits {
			return true
		}
	}
	return false
}
