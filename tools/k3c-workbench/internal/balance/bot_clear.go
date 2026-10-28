package balance

import (
	"math"

	"k3c/engine/sim"
)

// Freiraum (B-652): Ein Mauer- oder Turm-Platz wird erst bezahlbar, wenn im Radius keine Bäume und Felsen mehr stehen. Die
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

// blockedSite: der Mauer- oder Turm-Platz mit freigeschalteter Hub-Stufe, den noch Bäume oder Felsen sperren, mit dem
// niedrigsten rank; nil, wenn keiner gesperrt ist.
func blockedSite(w *sim.World) *sim.Site {
	var site *sim.Site
	for _, s := range w.Sites {
		if (s.Kind != "wall" && s.Kind != "tower") || s.State != "unpaid" || sim.SiteHubLevel(w, s) > w.HubLevel {
			continue
		}
		if len(sim.SiteBlockers(w, s)) > 0 && (site == nil || rank(w, s) < rank(w, site)) {
			site = s
		}
	}
	return site
}

// rank sortiert Plätze: Mauern vor Türmen, dann nach der Zahl der Blocker, dann nach Abstand zur Burg.
func rank(w *sim.World, s *sim.Site) float64 {
	d := math.Abs(s.X - w.HubX)
	d += 1000 * float64(len(sim.SiteBlockers(w, s)))
	if s.Kind == "tower" {
		d += 1e6
	}
	return d
}
