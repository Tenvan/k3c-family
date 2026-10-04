package sim

import (
	"sort"

	"k3c/engine/level"
)

// Mauerlinien in der Sim (B-206, Q47, Q48, Q54, Q58, Q59): Mauer, Turm und Tor jeder Linie aus dem Layout der Stufe.
// Linie und Seite eines Platzes ergeben sich aus Art und Position im Layout (`siteLine`), nicht aus Feldern am Site:
// destroySite und castleFallen setzen den Platz neu auf und würden sie sonst verlieren.

// siteSpec ist ein anzulegender Platz.
type siteSpec struct {
	kind string
	x    float64
}

// worldSiteSpecs: die Hub-Plätze dieser Tiefe und Mauer, Turm und Tor jeder Linie, fest nach X sortiert (jeder Platz
// zieht beim Anlegen eine ID).
func worldSiteSpecs(w *World) []siteSpec {
	out := []siteSpec{}
	for _, s := range hub.Sites {
		if w.Biome.Depth < s.FromDepth || (s.NeedsDeeper && !hasDepth(w.Biome.Depth+1)) {
			continue
		}
		out = append(out, siteSpec{s.Kind, w.HubX + s.OffsetUnits})
	}
	for _, l := range w.Level.Lines {
		out = append(out, siteSpec{"wall", l.Wall}, siteSpec{"tower", l.Tower}, siteSpec{"gate", l.Gate})
	}
	sort.SliceStable(out, func(i, j int) bool { return out[i].x < out[j].x })
	return out
}

// siteLine ist die Linie eines Platzes; false für Plätze, die zu keiner Linie gehören.
func siteLine(w *World, s *Site) (level.WallLine, bool) {
	for _, l := range w.Level.Lines {
		if (s.Kind == "wall" && s.X == l.Wall) || (s.Kind == "tower" && s.X == l.Tower) || (s.Kind == "gate" && s.X == l.Gate) {
			return l, true
		}
	}
	return level.WallLine{}, false
}

// wallBuilt: Hat die Mauer der Linie k auf Seite side den Zustand built? (Q58)
func wallBuilt(w *World, side, k int) bool {
	for _, s := range w.Sites {
		if l, ok := siteLine(w, s); ok && s.Kind == "wall" && l.Side == side && l.Index == k {
			return s.State == "built"
		}
	}
	return false
}

// lineOpen: Darf ein unbezahlter Platz bezahlt werden? Mauer und Turm der Linie k ab Hub-Stufe k, wenn alle Mauern
// innen auf derselben Seite gebaut sind (Q48; eine zerstörte Mauer k−1 sperrt also nur neue Linien, Q58). Das Tor
// der Linie k ab Hub-Stufe max(gateHubLevel, k) nur an der äußersten gebauten Mauer seiner Seite (Q47).
func lineOpen(w *World, s *Site) bool {
	l, ok := siteLine(w, s)
	if !ok {
		return true
	}
	if s.Kind == "gate" {
		return w.HubLevel >= max(hub.WallLines.GateHubLevel, l.Index) &&
			wallBuilt(w, l.Side, l.Index) && !wallBuilt(w, l.Side, l.Index+1)
	}
	if w.HubLevel < l.Index {
		return false
	}
	for k := 1; k < l.Index; k++ {
		if !wallBuilt(w, l.Side, k) {
			return false
		}
	}
	return true
}
