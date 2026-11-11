package rig

import (
	"fmt"
	"sort"
)

// ItemKind sagt, was an einer Stelle der Zeichenreihenfolge steht.
type ItemKind int

const (
	ItemPart ItemKind = iota
	ItemBand
	ItemSlot
	ItemReins
)

// Item ist ein Eintrag der Zeichenreihenfolge; Index zeigt in Parts, Bands oder Slots.
type Item struct {
	Kind  ItemKind
	Index int
	Layer float64
}

// buildOrder sortiert Teile mit Bild, Bänder, Plätze und Zügel nach Ebene (bei Gleichstand in dieser Reihenfolge).
func (r *Rig) buildOrder() error {
	for i, p := range r.Parts {
		if p.File != "" {
			r.Order = append(r.Order, Item{ItemPart, i, p.Layer})
		}
	}
	for i, b := range r.Bands {
		if b.Part != "" && r.Index(b.Part) < 0 {
			return fmt.Errorf("Rig %s: Band %s hängt an unbekanntem Teil %s", r.ID, b.ID, b.Part)
		}
		r.Order = append(r.Order, Item{ItemBand, i, b.Layer})
	}
	for i, s := range r.Slots {
		if r.Index(s.Part) < 0 {
			return fmt.Errorf("Rig %s: Platz %s hängt an unbekanntem Teil %s", r.ID, s.Name, s.Part)
		}
		r.Order = append(r.Order, Item{ItemSlot, i, s.Layer})
	}
	if r.Reins != nil {
		r.Order = append(r.Order, Item{ItemReins, 0, r.Reins.Layer})
	}
	sort.SliceStable(r.Order, func(i, j int) bool { return r.Order[i].Layer < r.Order[j].Layer })
	return nil
}

// World rechnet die Welt-Matrix je Teil: Ein Teil dreht um seinen Pivot, Kinder erben die Drehung; base wirkt auf alle.
// angles und out haben einen Eintrag je Teil (Reihenfolge von Parts); out wird wiederverwendet, wenn es passt.
func (r *Rig) World(angles []float64, base M, out []M) []M {
	n := len(r.Parts)
	if cap(out) < n {
		out = make([]M, n)
	}
	out = out[:n]
	for _, i := range r.topo {
		parent := base
		if r.parent[i] >= 0 {
			parent = out[r.parent[i]]
		}
		p := &r.Parts[i]
		out[i] = Mul(parent, Rotation(angles[i], p.Pivot[0], p.Pivot[1]))
	}
	return out
}

// SlotWorld liefert die Welt-Matrix eines Platzes: Teil · Verschiebung zum Punkt · Drehung des Platzes.
func (r *Rig) SlotWorld(s *Slot, world []M) M {
	return Mul(world[r.Index(s.Part)], Mul(Translate(s.Point[0], s.Point[1]), Rotation(s.Rotation, 0, 0)))
}
