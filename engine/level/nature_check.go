package level

import (
	"errors"
	"fmt"
	"maps"
	"slices"
)

// checkNature prüft traits und weights des Bioms (Datenprüfung beim Laden, B-891): Bereiche, neutraler Wert > 0,
// genau eine Wirkung je Eigenschaft, skalierte Arten und Chunk-Arten bekannt und je Art höchstens eine Eigenschaft.
func (b Biome) checkNature() error {
	used := map[string]string{}
	for i, e := range b.Traits {
		if e.Key == "" || slices.ContainsFunc(b.Traits[:i], func(o Entry[Trait]) bool { return o.Key == e.Key }) {
			return fmt.Errorf("traits: Schlüssel %q leer oder doppelt", e.Key)
		}
		if err := b.checkTrait(e.Value); err != nil {
			return fmt.Errorf("traits.%s: %w", e.Key, err)
		}
		for _, k := range slices.Concat(prefixed("res:", e.Value.Scales), prefixed("chunk:", e.Value.Chunks)) {
			if other, ok := used[k]; ok {
				return fmt.Errorf("traits.%s: %s wirkt schon über traits.%s", e.Key, k, other)
			}
			used[k] = e.Key
		}
	}
	return b.checkWeights()
}

func prefixed(p string, keys []string) []string {
	out := make([]string, len(keys))
	for i, k := range keys {
		out[i] = p + k
	}
	return out
}

func (b Biome) checkTrait(t Trait) error {
	effects := 0
	for _, has := range []bool{len(t.Scales) > 0, len(t.Chunks) > 0, t.Display} {
		if has {
			effects++
		}
	}
	switch {
	case t.Name == "":
		return errors.New("name fehlt")
	case !validRange(t):
		return fmt.Errorf("braucht 0 ≤ min ≤ default ≤ max und default > 0 (%d, %d, %d)", t.Min, t.Default, t.Max)
	case effects != 1:
		return errors.New("braucht genau eine Wirkung: scales, chunks oder display")
	}
	return b.checkTargets(t)
}

func validRange(t Trait) bool {
	return t.Min >= 0 && t.Min <= t.Default && t.Default <= t.Max && t.Default > 0
}

// checkTargets: Jede skalierte Art steht in resourcesPerChunk oder ist die Adern-Art, jede Chunk-Art in chunkWeights.
func (b Biome) checkTargets(t Trait) error {
	for _, kind := range t.Scales {
		if !b.hasResource(kind) && (b.Veins.Count == 0 || kind != b.Veins.Kind) {
			return fmt.Errorf("%q steht in keinem resourcesPerChunk und ist nicht die Adern-Art", kind)
		}
	}
	for _, kind := range t.Chunks {
		if _, ok := b.ChunkWeights.Get(kind); !ok {
			return fmt.Errorf("chunk-Art %q steht nicht in chunkWeights", kind)
		}
	}
	return nil
}

// checkWeights prüft die Gewichts-Gruppen: Name, Art, je Art höchstens eine Gruppe, Gewichte 0..MaxWeight und
// mindestens eine Variante mit Gewicht > 0.
func (b Biome) checkWeights() error {
	for i, g := range b.Weights {
		switch {
		case g.Key == "" || slices.ContainsFunc(b.Weights[:i], func(o Entry[WeightGroup]) bool { return o.Key == g.Key }):
			return fmt.Errorf("weights: Gruppe %q leer oder doppelt", g.Key)
		case g.Value.Name == "" || g.Value.Kind == "":
			return fmt.Errorf("weights.%s: name oder kind fehlt", g.Key)
		case slices.ContainsFunc(b.Weights[:i], func(o Entry[WeightGroup]) bool { return o.Value.Kind == g.Value.Kind }):
			return fmt.Errorf("weights.%s: Art %q hat schon eine Gruppe", g.Key, g.Value.Kind)
		case !b.hasResource(g.Value.Kind) && g.Value.Kind != b.Veins.Kind:
			return fmt.Errorf("weights.%s: Art %q kommt im Biom nicht vor", g.Key, g.Value.Kind)
		}
		if err := checkOptions(g.Value.Options); err != nil {
			return fmt.Errorf("weights.%s: %w", g.Key, err)
		}
	}
	return nil
}

// checkOptions prüft die Varianten einer Gruppe: vorhanden, eindeutig, Gewichte 0..MaxWeight, eine mit max > 0.
func checkOptions(opts Ordered[Range]) error {
	if len(opts) == 0 {
		return errors.New("ohne Varianten")
	}
	positive := false
	for j, o := range opts {
		r := o.Value
		switch {
		case o.Key == "" || slices.ContainsFunc(opts[:j], func(p Entry[Range]) bool { return p.Key == o.Key }):
			return fmt.Errorf("variante %q leer oder doppelt", o.Key)
		case r.Min < 0 || r.Min > r.Max || r.Max > MaxWeight:
			return fmt.Errorf("%s: braucht 0 ≤ min ≤ max ≤ %d (%d, %d)", o.Key, MaxWeight, r.Min, r.Max)
		}
		positive = positive || r.Max > 0
	}
	if !positive {
		return errors.New("alle Gewichte 0, mindestens eine Variante braucht Gewicht > 0")
	}
	return nil
}

func (b Biome) hasResource(kind string) bool {
	for _, c := range b.ResourcesPerChunk {
		if _, ok := c.Value.Get(kind); ok {
			return true
		}
	}
	return false
}

// CheckNatureValues prüft fest gesetzte Werte gegen das Biom: Wert im Bereich, unbekannte Schlüssel nur mit strict als
// Fehler (die Vorgabe einer Insel gilt nur in den Biomen, die die Eigenschaft haben), und keine Gruppe mit allen
// Varianten fest auf 0.
func (b Biome) CheckNatureValues(v NatureValues, strict bool) error {
	for _, k := range slices.Sorted(maps.Keys(v.Traits)) {
		t, ok := b.Traits.Get(k)
		switch {
		case !ok && strict:
			return fmt.Errorf("biom %s hat keine Eigenschaft %q", b.ID, k)
		case ok && (v.Traits[k] < t.Min || v.Traits[k] > t.Max):
			return fmt.Errorf("%s = %d liegt außerhalb %d–%d (Biom %s)", k, v.Traits[k], t.Min, t.Max, b.ID)
		}
	}
	for _, gk := range slices.Sorted(maps.Keys(v.Weights)) {
		g, ok := b.Weights.Get(gk)
		if !ok {
			if strict {
				return fmt.Errorf("biom %s hat keine Gewichts-Gruppe %q", b.ID, gk)
			}
			continue
		}
		if err := checkGroupValues(b.ID, gk, g, v.Weights[gk], strict); err != nil {
			return err
		}
	}
	return nil
}

func checkGroupValues(biome, gk string, g WeightGroup, fixed map[string]int, strict bool) error {
	for _, k := range slices.Sorted(maps.Keys(fixed)) {
		r, ok := g.Options.Get(k)
		switch {
		case !ok && strict:
			return fmt.Errorf("biom %s hat in %s keine Variante %q", biome, gk, k)
		case ok && (fixed[k] < r.Min || fixed[k] > r.Max):
			return fmt.Errorf("%s.%s = %d liegt außerhalb %d–%d (Biom %s)", gk, k, fixed[k], r.Min, r.Max, biome)
		}
	}
	if !slices.ContainsFunc(g.Options, func(e Entry[Range]) bool {
		w, isFixed := fixed[e.Key]
		return !isFixed && e.Value.Max > 0 || w > 0
	}) {
		return fmt.Errorf("%s: alle Varianten fest auf Gewicht 0 (Biom %s)", gk, biome)
	}
	return nil
}
