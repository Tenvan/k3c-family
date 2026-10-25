package sim

// nodeMarkable: Nimmt der Knoten gerade eine Markier-Münze? Sammelbar (Baum, Fels, Erz, Ader) und noch nicht markiert.
func nodeMarkable(n *ResourceNode) bool {
	_, ok := gatherOf(n.Kind)
	return !n.Marked && ok
}

// markNode markiert einen Knoten und merkt den Spieler der letzten Markier-Münze (B-532).
func markNode(n *ResourceNode, p *Player) {
	n.Marked = true
	by := p.Index
	n.MarkedBy = &by
}

// markedByOf liefert den Markierer je markiertem Knoten (`kind@x`) für den Spielstand; nil, wenn keiner bekannt ist.
func markedByOf(w *World) map[string]int {
	var out map[string]int
	for _, n := range w.Nodes {
		if n.Marked && n.MarkedBy != nil {
			if out == nil {
				out = map[string]int{}
			}
			out[key(n.Kind, n.X)] = *n.MarkedBy
		}
	}
	return out
}

// applyMarkedBy setzt den gespeicherten Markierer eines markierten Knotens; fehlt er, bleibt er unbekannt.
func applyMarkedBy(n *ResourceNode, saved map[string]int) {
	if by, ok := saved[key(n.Kind, n.X)]; ok && n.Marked {
		n.MarkedBy = &by
	}
}
