package sim

import "testing"

func findNode(w *World, kind string, x float64) *ResourceNode {
	for _, n := range w.Nodes {
		if n.Kind == kind && n.X == x {
			return n
		}
	}
	return nil
}

// B-532: Der Knoten nennt den Spieler der letzten Markier-Münze; Speichern und Laden behalten ihn.
func TestKnotenMarkiererInSnapshotUndSpielstand(t *testing.T) {
	c := CreateCampaign("markedby", "t", 1e-6)
	c.JoinPlayer()
	p1 := c.JoinPlayer()
	w := c.CurrentWorld()
	var tree *ResourceNode
	for _, n := range w.Nodes {
		if n.Kind == "tree" && (tree == nil || n.X > tree.X) {
			tree = n // der äußerste Baum liegt abseits anderer Zahlziele
		}
	}
	if tree.MarkedBy != nil {
		t.Fatal("unmarkierter Knoten hat einen Markierer")
	}
	payAt(w, []*Player{p1}, tree.X, 10, func() bool { return tree.Marked })
	if !tree.Marked || tree.MarkedBy == nil || *tree.MarkedBy != p1.Index {
		t.Fatalf("markiert %v, markedBy %v, erwartet Spieler %d", tree.Marked, tree.MarkedBy, p1.Index)
	}
	s := c.ToSave("2026-10-23T00:00:00Z")
	got := findNode(FromSave(s, 1e-6).CurrentWorld(), "tree", tree.X)
	if got == nil || !got.Marked || got.MarkedBy == nil || *got.MarkedBy != p1.Index {
		t.Fatalf("nach dem Laden: %+v", got)
	}
}

// Ein Stand ohne `nodesMarkedBy` (älter) lädt: der Knoten bleibt markiert, der Markierer unbekannt.
func TestKnotenMarkiererFehltImAltenStand(t *testing.T) {
	c := CreateCampaign("markedby", "t", 1e-6)
	p := c.JoinPlayer()
	w := c.CurrentWorld()
	tree := firstNode(t, w, "tree")
	markNode(tree, p)
	s := c.ToSave("2026-10-23T00:00:00Z")
	s.Hubs[0].NodesMarkedBy = nil
	got := findNode(FromSave(s, 1e-6).CurrentWorld(), "tree", tree.X)
	if got == nil || !got.Marked || got.MarkedBy != nil {
		t.Fatalf("älterer Stand: %+v", got)
	}
}
