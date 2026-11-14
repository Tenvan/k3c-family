package level

import (
	"maps"
	"slices"
	"strconv"

	"k3c/engine/rng"
)

// Natur-Eigenschaften der Biome (B-891, docs/rules/welten.md § 2.1): Ein Biom nennt in `traits` seine Eigenschaften
// (z. B. Walddichte) mit Bereich und neutralem Wert und in `weights` Gewichts-Gruppen (z. B. Baumarten) mit dem Bereich
// des Gewichts je Variante. Je Stufe gilt ein Wert: fest gesetzt in der Welt-Datei oder aus dem Seed gewürfelt. Die
// Würfe kommen aus eigenen Strömen `<biom>:<seed>:nature` und `<biom>:<seed>:variant:<gruppe>:<x>`; der Hauptstrom des
// Generators bleibt unberührt. Geplante Chunks würfeln ihren Inhalt ohne Natur-Faktoren (Chunk-Seed = gleicher Inhalt).

// Trait ist eine Eigenschaft eines Bioms: ganzzahliger Wert in Min..Max. Default ist der neutrale Wert (Faktor 1, Level
// wie ohne Eigenschaft). Wirkung, genau eine davon: Scales skaliert die Anzahl der Ressourcen-Arten (Schlüssel aus
// resourcesPerChunk oder die Adern-Art) mit Wert/Default, Chunks das Gewicht der Chunk-Arten in chunkWeights; Display
// heißt: nur Darstellung, der Generator liest den Wert nicht.
type Trait struct {
	Name    string   `json:"name"`
	Min     int      `json:"min"`
	Max     int      `json:"max"`
	Default int      `json:"default"`
	Scales  []string `json:"scales,omitempty"`
	Chunks  []string `json:"chunks,omitempty"`
	Display bool     `json:"display,omitempty"`
}

// WeightGroup ist eine Gewichts-Eigenschaft (z. B. Baumarten): Jede Entität der Art Kind bekommt eine Variante aus
// Options, mit der Wahrscheinlichkeit Gewicht / Summe aller Gewichte. Gewicht 0 = kommt nicht vor.
type WeightGroup struct {
	Name    string         `json:"name"`
	Kind    string         `json:"kind"`
	Options Ordered[Range] `json:"options"`
}

// MaxWeight ist das größte Gewicht einer Variante (0 = kommt nicht vor, 5 = sehr häufig).
const MaxWeight = 5

// NatureValues sind Werte der Natur-Eigenschaften: Traits je Schlüssel aus Biome.Traits, Weights je Gruppe und
// Variante aus Biome.Weights. In der Welt-Datei sind es die fest gesetzten Werte (fehlende würfelt der Generator), im
// Layout die geltenden Werte der Stufe.
type NatureValues struct {
	Traits  map[string]int            `json:"traits,omitempty"`
	Weights map[string]map[string]int `json:"weights,omitempty"`
}

// Empty meldet, ob kein Wert gesetzt ist.
func (v NatureValues) Empty() bool { return len(v.Traits) == 0 && len(v.Weights) == 0 }

// SetTrait setzt einen Eigenschafts-Wert (legt die Map bei Bedarf an).
func (v *NatureValues) SetTrait(key string, val int) {
	if v.Traits == nil {
		v.Traits = map[string]int{}
	}
	v.Traits[key] = val
}

// SetWeight setzt das Gewicht einer Variante (legt die Maps bei Bedarf an).
func (v *NatureValues) SetWeight(group, option string, val int) {
	if v.Weights == nil {
		v.Weights = map[string]map[string]int{}
	}
	if v.Weights[group] == nil {
		v.Weights[group] = map[string]int{}
	}
	v.Weights[group][option] = val
}

// MergeNature legt over über base (over gewinnt je Schlüssel); die Eingaben bleiben unverändert.
func MergeNature(base, over NatureValues) NatureValues {
	out := NatureValues{Traits: maps.Clone(base.Traits)}
	for _, v := range []NatureValues{base, over} {
		for _, g := range slices.Sorted(maps.Keys(v.Weights)) {
			for _, k := range slices.Sorted(maps.Keys(v.Weights[g])) {
				out.SetWeight(g, k, v.Weights[g][k])
			}
		}
	}
	for _, k := range slices.Sorted(maps.Keys(over.Traits)) {
		out.SetTrait(k, over.Traits[k])
	}
	return out
}

// NeutralNature sind die neutralen Werte des Bioms: jede Eigenschaft auf Default, jede Variante auf ihr größtes Gewicht.
func NeutralNature(b Biome) NatureValues {
	out := NatureValues{}
	for _, e := range b.Traits {
		out.SetTrait(e.Key, e.Value.Default)
	}
	for _, g := range b.Weights {
		for _, o := range g.Value.Options {
			out.SetWeight(g.Key, o.Key, o.Value.Max)
		}
	}
	return out
}

// ResolveNature liefert die geltenden Werte der Stufe: je Eigenschaft und Variante ein Wurf in min..max aus dem Strom
// `<biom>:<seed>:nature` (immer, auch bei festem Wert, damit ein fester Wert die übrigen nicht verschiebt), dann der
// feste Wert aus fixed. Ergibt eine Gruppe zusammen 0, bekommt ihre erste gewürfelte Variante mit max > 0 das Gewicht 1.
func ResolveNature(b Biome, seed string, fixed NatureValues) (NatureValues, error) {
	if err := b.CheckNatureValues(fixed, true); err != nil {
		return NatureValues{}, err
	}
	if len(b.Traits) == 0 && len(b.Weights) == 0 {
		return NatureValues{}, nil
	}
	r := rng.New(b.ID + ":" + seed + ":nature")
	out := NatureValues{}
	for _, e := range b.Traits {
		v := r.Int(e.Value.Min, e.Value.Max)
		if f, ok := fixed.Traits[e.Key]; ok {
			v = f
		}
		out.SetTrait(e.Key, v)
	}
	for _, g := range b.Weights {
		sum := 0
		for _, o := range g.Value.Options {
			w := r.Int(o.Value.Min, o.Value.Max)
			if f, ok := fixed.Weights[g.Key][o.Key]; ok {
				w = f
			}
			out.SetWeight(g.Key, o.Key, w)
			sum += w
		}
		for _, o := range g.Value.Options {
			if _, isFixed := fixed.Weights[g.Key][o.Key]; sum == 0 && !isFixed && o.Value.Max > 0 {
				out.SetWeight(g.Key, o.Key, 1)
				break
			}
		}
	}
	return out, nil
}

// traitValue ist der Wert der Eigenschaft in v, ohne Eintrag ihr Default.
func traitValue(v NatureValues, key string, t Trait) int {
	if val, ok := v.Traits[key]; ok {
		return val
	}
	return t.Default
}

// withNature ist das Biom nach den Werten v (Kopie): Bereiche in resourcesPerChunk und die Zahl der Adern skaliert,
// Chunk-Gewichte skaliert. Default ergibt dieselben Zahlen und damit dieselben Würfe wie ohne Eigenschaft. Würden alle
// Chunk-Gewichte 0, bleiben die des Bioms.
func (b Biome) withNature(v NatureValues) Biome {
	if len(b.Traits) == 0 {
		return b
	}
	scales := func(t Trait) []string { return t.Scales }
	res := make(Ordered[Ordered[[2]int]], len(b.ResourcesPerChunk))
	for i, c := range b.ResourcesPerChunk {
		kinds := make(Ordered[[2]int], len(c.Value))
		for j, k := range c.Value {
			kinds[j] = k
			if key, t, ok := b.traitOf(k.Key, scales); ok {
				val := traitValue(v, key, t)
				kinds[j].Value = [2]int{scaleCount(k.Value[0], val, t.Default), scaleCount(k.Value[1], val, t.Default)}
			}
		}
		res[i] = Entry[Ordered[[2]int]]{Key: c.Key, Value: kinds}
	}
	b.ResourcesPerChunk = res
	if key, t, ok := b.traitOf(b.Veins.Kind, scales); ok && b.Veins.Count > 0 {
		b.Veins.Count = scaleCount(b.Veins.Count, traitValue(v, key, t), t.Default)
	}
	weights, sum := make(Ordered[float64], len(b.ChunkWeights)), 0.0
	for i, c := range b.ChunkWeights {
		weights[i] = c
		if key, t, ok := b.traitOf(c.Key, func(t Trait) []string { return t.Chunks }); ok && traitValue(v, key, t) != t.Default {
			weights[i].Value = float64(c.Value*float64(traitValue(v, key, t))) / float64(t.Default)
		}
		sum += weights[i].Value
	}
	if sum > 0 {
		b.ChunkWeights = weights
	}
	return b
}

// traitOf liefert die Eigenschaft, die kind in der Liste of nennt.
func (b Biome) traitOf(kind string, of func(Trait) []string) (string, Trait, bool) {
	for _, e := range b.Traits {
		for _, k := range of(e.Value) {
			if k == kind {
				return e.Key, e.Value, true
			}
		}
	}
	return "", Trait{}, false
}

// scaleCount ist n × v / def, kaufmännisch gerundet, in ganzen Zahlen (kein Gleitkomma, deterministisch).
func scaleCount(n, v, def int) int { return (2*n*v + def) / (2 * def) }

// assignVariants gibt jeder Entität der Art einer Gewichts-Gruppe eine Variante nach den Gewichten aus v, gewürfelt
// je Entität im Strom `<biom>:<seed>:variant:<gruppe>:<x>`: Die Variante hängt nur vom Ort ab, ein Baum mehr oder
// weniger verschiebt keine andere.
func assignVariants(b Biome, seed string, v NatureValues, es []Entity) {
	for _, g := range b.Weights {
		w := make([]rng.Weight, len(g.Value.Options))
		for i, o := range g.Value.Options {
			w[i] = rng.Weight{Key: o.Key, W: float64(v.Weights[g.Key][o.Key])}
		}
		prefix := b.ID + ":" + seed + ":variant:" + g.Key + ":"
		for i := range es {
			if es[i].Kind == g.Value.Kind {
				es[i].Variant = rng.New(prefix + strconv.FormatFloat(es[i].X, 'g', -1, 64)).Weighted(w)
			}
		}
	}
}
