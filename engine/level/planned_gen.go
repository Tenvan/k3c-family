package level

import (
	"math"
	"strconv"

	"k3c/engine/rng"
)

// Options sind die optionalen Parameter des Generators. Der Nullwert erzeugt das Level wie Generate: ohne Planung und
// mit den eingebetteten Mauerlinien. Es gibt keinen globalen Zustand zum Umschalten.
type Options struct {
	// Lines sind die Mauerlinien (nil = eingebettete aus data/hub.json).
	Lines *WallLines
	// Planned sind die geplanten Chunks dieser Stufe (docs/rules/geplante-chunks.md).
	Planned []PlannedChunk
	// ChunkCount ist die eingestellte Chunk-Anzahl der Stufe (Inselgröße, geplante-chunks.md § 7); 0 = gewürfelte
	// Länge aus lengthUnits wie bisher. Sie ersetzt den Wurf der Länge.
	ChunkCount int
}

func (o Options) lines() WallLines {
	if o.Lines != nil {
		return *o.Lines
	}
	return wallLines
}

// slotIndex ist der Chunk-Index eines Platzes: +n liegt n Chunks rechts der Hub-Chunks, −n links davon.
func (g *gen) slotIndex(slot int) int {
	if slot > 0 {
		return g.hubStart + g.hubChunks + slot - 1
	}
	return g.hubStart + slot
}

// reserve belegt die geplanten Plätze direkt nach frame() und zählt die festen Objekte, die auf die Biom-Summen
// anzurechnen sind. Ein Ereignis-Chunk ohne Objektliste trägt sein Objekt in der Chunk-Mitte.
func (g *gen) reserve(plan []PlannedChunk) {
	g.planned, g.fixed = map[int]PlannedChunk{}, map[string]int{}
	for _, c := range plan {
		i := g.slotIndex(c.Slot)
		g.kinds[i] = c.Kind
		g.planned[i] = c
		side := 0
		if c.Slot > 0 {
			side = 1
		}
		for _, o := range fixedObjects(c, g.cw) {
			g.fixed[o.Kind]++
			if o.Kind == "portal" {
				g.sideFixed[side]++
			}
		}
	}
}

// fixedObjects sind die Objekte eines geplanten Chunks mit At ab Chunk-Anfang; leer bei einer Chunk-Art, die ihre
// Objekte aus dem eigenen Strom würfelt.
func fixedObjects(c PlannedChunk, cw float64) []PlannedObject {
	if len(c.Objects) > 0 {
		return c.Objects
	}
	if isEvent(c.Kind) || c.Kind == "portal" {
		return []PlannedObject{{Kind: c.Kind, At: cw / 2}}
	}
	return nil
}

// chunkStream ist der eigene Strom eines geplanten Chunks: ohne Chunk-Seed `<biom>:<seed>:chunk:<platz>` (Art fest,
// Inhalt wechselt mit dem Level-Seed), mit Chunk-Seed `<biom>:chunk:<chunk-seed>` (Inhalt in jedem Spielstand gleich).
// Der Haupt-Strom bleibt unberührt. Die Namen sind Vertrag.
func (g *gen) chunkStream(c PlannedChunk) *rng.Rng {
	if c.Seed != "" {
		return rng.New(g.b.ID + ":chunk:" + c.Seed)
	}
	return rng.New(g.b.ID + ":" + g.seed + ":chunk:" + strconv.Itoa(c.Slot))
}

// clearTries ist die Zahl der Neuwürfe, bevor eine Ressource an den Rand der Ruinen-Sperrzone gesetzt wird.
const clearTries = 32

// plannedEntities setzt die Objekte eines geplanten Chunks: feste Objekte oder die Ressourcen der Chunk-Art aus dem
// eigenen Strom.
func (g *gen) plannedEntities(c Chunk, pc PlannedChunk) []Entity {
	var out []Entity
	if objects := fixedObjects(pc, g.cw); objects != nil {
		for _, o := range objects {
			out = append(out, Entity{Kind: o.Kind, X: c.StartUnits + o.At})
		}
		return out
	}
	r := g.chunkStream(pc)
	resources, _ := g.b.ResourcesPerChunk.Get(c.Kind)
	for _, res := range resources {
		for range r.Int(res.Value[0], res.Value[1]) {
			out = append(out, Entity{Kind: res.Key, X: g.clearOfRuin(r, c, pc)})
		}
	}
	return out
}

// clearOfRuin ist eine Position wie inside, die ≥ ruinGapUnits von der Ruine des Chunks entfernt liegt (B-522). Ohne
// Ruine ist es genau ein Wurf wie bisher; sonst wird bis zu clearTries-mal neu gewürfelt (gleicher Strom, deterministisch)
// und zuletzt an den nächsten erlaubten Rand der Ruine gesetzt.
func (g *gen) clearOfRuin(r *rng.Rng, c Chunk, pc PlannedChunk) float64 {
	x := g.inside(r, c)
	if len(pc.Buildings) == 0 {
		return x
	}
	ruin := c.StartUnits + pc.Buildings[0].At
	for range clearTries {
		if math.Abs(x-ruin) >= ruinGapUnits {
			return x
		}
		x = g.inside(r, c)
	}
	if math.Abs(x-ruin) >= ruinGapUnits {
		return x
	}
	if lo := ruin - ruinGapUnits; lo >= c.StartUnits+edgeGapUnits && x < ruin {
		return lo
	}
	if hi := ruin + ruinGapUnits; hi <= c.StartUnits+g.cw-edgeGapUnits {
		return hi
	}
	return ruin - ruinGapUnits
}
