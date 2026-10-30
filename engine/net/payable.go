package net

import "k3c/engine/sim"

// Bezahlbar je Bauplatz und abbaubar je Knoten (B-587, B-588): Der Server meldet, wo eine Münze gerade etwas bezahlt;
// der Client zeichnet Bau- und Abbau-Schild nur danach und rechnet nichts nach. Beide Felder fehlen bei false; `clearing` meldet einen Platz, der erst nach dem Roden von Bäumen und Felsen bezahlbar wird.

// addPayable ergänzt `payable` je Bauplatz (sim.SitePayable) und `minable` je Knoten (sim.NodeMarkable); s ist der
// Zustand aus stateOf, `sites` und `nodes` in der Reihenfolge von w.Sites und w.Nodes.
func addPayable(w *sim.World, s map[string]any) {
	sites, _ := s["sites"].([]any)
	for i, raw := range sites {
		if sim.SitePayable(w, w.Sites[i]) {
			raw.(map[string]any)["payable"] = true
		} else if sim.SiteNeedsClearing(w, w.Sites[i]) {
			raw.(map[string]any)["clearing"] = true // erst roden (B-636)
		}
	}
	nodes, _ := s["nodes"].([]any)
	for i, raw := range nodes {
		if sim.NodeMarkable(w.Nodes[i]) {
			raw.(map[string]any)["minable"] = true
		}
	}
}
