package sim

// Plantage (B-114, Q25, Q51): Eine gebaute Farm lässt auf jedem ihrer Plätze (data/buildings.json › farm.plantation)
// GrowSeconds nach dem Leerwerden einen Baum wachsen, gleichmäßig im Streifen ±StripUnits um die Farm. Plantage-Bäume
// brauchen keine Münze: Sie entstehen markiert und sind damit kein Zahlziel. Sie haben keine Level-Position; der
// Spielstand speichert sie nicht, nach dem Laden wachsen sie neu.
// ponytail: Eine zerstörte und neu gebaute Farm vergisst ihre alten Bäume (bis zu 6 weitere), Grenze bei Bedarf über
// die Bäume im Streifen zählen.

// Plantation sind die Plantage-Werte einer Farm.
type Plantation struct {
	Slots                   int
	GrowSeconds, StripUnits float64
}

// plot ist ein Platz der Plantage: der gewachsene Baum oder die Zeit bis zum nächsten.
type plot struct {
	tree   *ResourceNode
	growIn float64
}

// plotX ist die Position von Platz i im Streifen um die Farm.
func plotX(s *Site, p Plantation, i int) float64 {
	if p.Slots < 2 {
		return s.X
	}
	return s.X - p.StripUnits + float64(float64(i)*2*p.StripUnits)/float64(p.Slots-1)
}

// stepPlantations lässt auf den Plätzen gebauter Farmen Bäume nachwachsen.
func stepPlantations(w *World, dt float64) {
	for _, s := range w.Sites {
		p := buildings[s.Kind].Plantation
		if p.Slots == 0 || s.State != "built" {
			continue
		}
		if s.plots == nil {
			s.plots = make([]plot, p.Slots)
			for i := range s.plots {
				s.plots[i].growIn = p.GrowSeconds
			}
		}
		for i := range s.plots {
			growPlot(w, s, p, i, dt)
		}
	}
}

func growPlot(w *World, s *Site, p Plantation, i int, dt float64) {
	pl := &s.plots[i]
	if pl.tree != nil && pl.tree.gone {
		pl.tree, pl.growIn = nil, p.GrowSeconds
	}
	if pl.tree != nil {
		return
	}
	pl.growIn -= dt
	if pl.growIn <= 0 {
		pl.tree = &ResourceNode{ID: w.newID(), Kind: "tree", X: plotX(s, p, i), Marked: true}
		w.Nodes = append(w.Nodes, pl.tree)
	}
}
