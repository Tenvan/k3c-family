package sim

import (
	"fmt"
	"maps"
	"slices"
	"strconv"

	"k3c/engine/level"
)

// Natur-Werte der Insel (B-891, docs/rules/welten.md § 2.1): `nature` ist die Vorgabe der Insel für alle Stufen, deren
// Biom die Eigenschaft hat; `natureByDepth` setzt je Tiefe fest und gewinnt. Was keine Stufe festsetzt, würfelt der
// Level-Generator aus dem Seed (level.ResolveNature).

// NatureAt liefert die fest gesetzten Natur-Werte der Tiefe: Vorgabe der Insel ohne die Schlüssel, die das Biom nicht
// kennt, darüber die Werte der Tiefe.
func (d IslandDef) NatureAt(depth int) level.NatureValues {
	b := biomeForDepth(depth)
	base := level.NatureValues{}
	for _, k := range slices.Sorted(maps.Keys(d.Nature.Traits)) {
		if _, ok := b.Traits.Get(k); ok {
			base.SetTrait(k, d.Nature.Traits[k])
		}
	}
	for _, gk := range slices.Sorted(maps.Keys(d.Nature.Weights)) {
		g, ok := b.Weights.Get(gk)
		for _, k := range slices.Sorted(maps.Keys(d.Nature.Weights[gk])) {
			if _, known := g.Options.Get(k); ok && known {
				base.SetWeight(gk, k, d.Nature.Weights[gk][k])
			}
		}
	}
	return level.MergeNature(base, d.NatureByDepth[strconv.Itoa(depth)])
}

// checkNature prüft die Natur-Werte der Insel: jeder Schlüssel der Vorgabe gilt in mindestens einem Biom der Insel und
// liegt dort im Bereich; je Tiefe nur bekannte Tiefen und Schlüssel ihres Bioms; die zusammengelegten Werte setzen nicht
// alle Baumarten auf 0.
func checkNature(d IslandDef) error {
	for _, k := range slices.Sorted(maps.Keys(d.Nature.Traits)) {
		if !slices.ContainsFunc(d.Depths, func(depth int) bool { _, ok := biomeForDepth(depth).Traits.Get(k); return ok }) {
			return fmt.Errorf("nature: kein Biom der Insel hat die Eigenschaft %q", k)
		}
	}
	for _, gk := range slices.Sorted(maps.Keys(d.Nature.Weights)) {
		for _, k := range slices.Sorted(maps.Keys(d.Nature.Weights[gk])) {
			if !slices.ContainsFunc(d.Depths, func(depth int) bool {
				g, ok := biomeForDepth(depth).Weights.Get(gk)
				_, known := g.Options.Get(k)
				return ok && known
			}) {
				return fmt.Errorf("nature: kein Biom der Insel hat die Variante %s.%s", gk, k)
			}
		}
	}
	for _, key := range slices.Sorted(maps.Keys(d.NatureByDepth)) {
		depth, err := strconv.Atoi(key)
		if err != nil || strconv.Itoa(depth) != key || !slices.Contains(d.Depths, depth) {
			return fmt.Errorf("natureByDepth: Tiefe %q ist keine Tiefe der Insel", key)
		}
		if err := biomeForDepth(depth).CheckNatureValues(d.NatureByDepth[key], true); err != nil {
			return fmt.Errorf("natureByDepth Tiefe %d: %w", depth, err)
		}
	}
	for _, depth := range d.Depths {
		if err := biomeForDepth(depth).CheckNatureValues(d.NatureAt(depth), false); err != nil {
			return fmt.Errorf("nature Tiefe %d: %w", depth, err)
		}
	}
	return nil
}

// natureFor liefert die fest gesetzten Natur-Werte der Tiefe; ohne Nature die der Insel def.
func (o CreateOptions) natureFor(depth int, def IslandDef) level.NatureValues {
	if o.Nature != nil {
		return o.Nature[depth]
	}
	return def.NatureAt(depth)
}
