package sim

import (
	"math"
	"sort"

	"k3c/engine/level"
)

// Verteilte Mauer- und Turm-Plätze in der Sim (B-652, BS12.2): Mauer und Turm kommen aus `Layout.Sites` der Stufe (Zone n von
// innen je Seite), ein Tor-Platz gibt es nicht mehr: Die äußerste gebaute Mauer je Seite ist das Tor (outerWall) und wird
// wie jede Mauer ausgebaut. Seite und Zone eines Platzes ergeben sich aus Art und Position im Layout (`levelSite`), nicht
// aus Feldern am Site: destroySite und castleFallen setzen den Platz neu auf und würden sie sonst verlieren.

// siteSpec ist ein anzulegender Platz.
type siteSpec struct {
	kind string
	x    float64
}

// worldSiteSpecs: die Hub-Plätze dieser Tiefe und alle Mauern und Türme der Stufe, fest nach X sortiert (jeder Platz
// zieht beim Anlegen eine ID).
func worldSiteSpecs(w *World) []siteSpec {
	out := []siteSpec{}
	for _, s := range hubOf(w).Sites {
		if w.Biome.Depth < s.FromDepth || (s.NeedsDeeper && !hasDepth(w.Biome.Depth+1)) {
			continue
		}
		out = append(out, siteSpec{s.Kind, w.HubX + s.OffsetUnits})
	}
	for _, l := range w.Level.Sites {
		out = append(out, siteSpec{l.Kind, l.X})
	}
	sort.SliceStable(out, func(i, j int) bool { return out[i].x < out[j].x })
	return out
}

// wallLookup merkt das Ergebnis von levelSite am Platz: Art und X eines Platzes ändern sich nie (destroySite setzt ihn
// mit denselben Werten neu auf), das Layout einer Welt ebenso.
type wallLookup struct {
	w    *World // Welt, für die der Eintrag gilt
	ok   bool
	site level.WallSite
}

// levelSite ist der verteilte Platz zu s; false für Plätze, die nicht aus Layout.Sites stammen.
func levelSite(w *World, s *Site) (level.WallSite, bool) {
	if s.wall.w != w {
		s.wall = wallLookup{w: w}
		for _, l := range w.Level.Sites {
			if l.Kind == s.Kind && l.X == s.X {
				s.wall.site, s.wall.ok = l, true
				break
			}
		}
	}
	return s.wall.site, s.wall.ok
}

// lineOpen: Darf ein unbezahlter Mauer- oder Turm-Platz bezahlt werden? Zone n ab Hub-Stufe n, und im Freiraum
// (`sitePlacement.clearRadiusUnits`) steht kein Baum und kein Fels mehr (B-652, 🧑 2026-10-28). Innere Mauern müssen
// nicht gebaut sein.
func lineOpen(w *World, s *Site) bool {
	l, ok := levelSite(w, s)
	if !ok {
		return true
	}
	return w.HubLevel >= l.Zone && siteCleared(w, s.X)
}

// siteCleared: Im Freiraum um x steht kein Baum und kein Fels (abgebaute Knoten sind aus Nodes entfernt).
func siteCleared(w *World, x float64) bool {
	radius := level.DefaultSitePlacement().ClearRadiusUnits
	for _, n := range w.Nodes {
		if (n.Kind == "tree" || n.Kind == "rock") && !n.gone && math.Abs(n.X-x) < radius {
			return false
		}
	}
	return true
}

// blockersAt sind die Bäume und Felsen im Freiraum um x.
func blockersAt(w *World, x float64) []*ResourceNode {
	radius := level.DefaultSitePlacement().ClearRadiusUnits
	var out []*ResourceNode
	for _, n := range w.Nodes {
		if (n.Kind == "tree" || n.Kind == "rock") && !n.gone && math.Abs(n.X-x) < radius {
			out = append(out, n)
		}
	}
	return out
}

// SiteBlockers sind die Bäume und Felsen, die einen Mauer- oder Turm-Platz noch sperren (leer für alle anderen Plätze);
// Bots markieren sie zum Abbau (B-652).
func SiteBlockers(w *World, s *Site) []*ResourceNode {
	if _, ok := levelSite(w, s); !ok {
		return nil
	}
	return blockersAt(w, s.X)
}

// SiteNeedsClearing sagt, ob ein Platz erst nach dem Roden bezahlbar wird (Baumschule, Mauer, Turm): Seine Hub-Stufe ist erreicht, aber
// im Freiraum stehen noch Bäume oder Felsen. Das Protokoll meldet es als `clearing`, der Client zeigt „erst roden“ (B-636, B-652).
func SiteNeedsClearing(w *World, s *Site) bool {
	l, ok := levelSite(w, s)
	return ok && s.State == "unpaid" && w.HubLevel >= l.Zone && !siteCleared(w, s.X)
}
