package level

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math"
	"slices"
)

// Geplante Chunks (docs/rules/geplante-chunks.md, B-461): feste Gegenden je Insel und Tiefe. Dieses Paket prüft die
// Planung unabhängig vom Seed gegen die kürzeste mögliche Länge des Bioms und die Verteidigungszone.

// edgeGapUnits ist der Abstand von Ressourcen, Truhen und Adern zur Chunk-Grenze (wie `inside` im Generator).
const edgeGapUnits = 2

// PlannedObject ist ein festes Objekt eines geplanten Chunks; At zählt in Units ab Chunk-Anfang.
type PlannedObject struct {
	Kind string  `json:"kind"`
	At   float64 `json:"at"`
}

// PlannedChunk ist ein geplanter Chunk. Slot zählt relativ zu den Hub-Chunks (−n links, +n rechts, nie 0).
// Buildings trägt höchstens eine Ruine (§ 8, B-468); Enemies sind die Wachen des Gegnerlagers (§ 9, B-517).
type PlannedChunk struct {
	Slot      int               `json:"slot"`
	Kind      string            `json:"kind"`
	Seed      string            `json:"seed,omitempty"`
	Objects   []PlannedObject   `json:"objects,omitempty"`
	Buildings []PlannedBuilding `json:"buildings,omitempty"`
	Enemies   []PlannedEnemy    `json:"enemies,omitempty"`
}

// UnmarshalJSON liest streng: ein unbekanntes Feld (z. B. ein Tippfehler wie `object`) ist ein Fehler (fail closed).
func (c *PlannedChunk) UnmarshalJSON(raw []byte) error {
	type plain PlannedChunk
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields()
	return dec.Decode((*plain)(c))
}

// geometry beschreibt die Länge eines Bioms in Chunks, gegen die eine Planung geprüft wird.
type geometry struct {
	cw      float64 // Chunk-Breite
	hubHalf float64 // halbe Breite der Hub-Chunks
	usable  int     // planbare Plätze je Seite (ohne Ausgang und Rand)
	count   int     // eingestellte Chunk-Anzahl; 0 = kürzeste Länge des Bioms
}

// geometryOf rechnet wie `frame` im Generator: mit der eingestellten Anzahl count, ohne sie (0) mit der kleinsten Länge.
func geometryOf(b Biome, count int) geometry {
	cw := b.ChunkWidthUnits
	hubChunks := hubChunksOf(b)
	n := count
	if n == 0 {
		n = int(math.Round(float64(b.LengthUnits.Min) / cw))
		if n%2 != 0 {
			n++
		}
	}
	return geometry{cw: cw, hubHalf: float64(hubChunks) * cw / 2, usable: (n-hubChunks)/2 - 1, count: count}
}

// beyond ist der Grund für einen Platz jenseits der geprüften Länge.
func (g geometry) beyond() error {
	if g.count > 0 {
		return fmt.Errorf("jenseits der Chunk-Anzahl %d (höchstens ±%d)", g.count, g.usable)
	}
	return fmt.Errorf("jenseits der kürzesten Länge (höchstens ±%d)", g.usable)
}

// dist ist der Abstand einer Stelle `at` Units hinter dem Chunk-Anfang von der Hub-Mitte.
func (g geometry) dist(slot int, at float64) float64 {
	if slot > 0 {
		return g.hubHalf + float64(slot-1)*g.cw + at
	}
	return g.hubHalf + float64(-slot)*g.cw - at
}

// placed ist ein festes oder implizites Objekt mit seinem Abstand zum Hub.
type placed struct {
	kind  string
	slot  int
	at    float64
	dist  float64
	fixed bool // vom Nutzer gesetzt; false = Ereignis-Chunk ohne Objektliste, Objekt in der Chunk-Mitte
}

// spotKey ist `kind@x` mit der Lage x als Abstand zur Hub-Mitte, links negativ (der Spielstand ordnet Objekte so zu).
type spotKey struct {
	kind string
	x    float64
}

func (p placed) key() spotKey {
	x := p.dist
	if p.slot < 0 {
		x = -x
	}
	return spotKey{p.kind, x}
}

// hasChunkKind: Chunk-Arten des Bioms (chunkWeights) und die Ereignis-Arten.
func hasChunkKind(b Biome, kind string) bool {
	if kind == "portal" || isEvent(kind) {
		return true
	}
	_, ok := b.ChunkWeights.Get(kind)
	return ok
}

// hasObjectKind: Ressourcen laut resourcesPerChunk, die Ader des Bioms und die Ereignis-Arten.
func hasObjectKind(b Biome, kind string) bool {
	if kind == "portal" || isEvent(kind) || (b.Veins.Count > 0 && kind == b.Veins.Kind) {
		return true
	}
	for _, chunkKind := range b.ResourcesPerChunk {
		if _, ok := chunkKind.Value.Get(kind); ok {
			return true
		}
	}
	return false
}

// ValidatePlan prüft die geplanten Chunks einer Tiefe unabhängig vom Seed. Ist die Planung gültig, scheitert kein Seed
// zur Laufzeit. lines sind die Mauerlinien (DefaultWallLines für die eingebetteten). Der Fehler nennt Platz und Grund.
// Gerechnet wird mit der kürzesten Länge des Bioms; ValidatePlanFor rechnet mit einer eingestellten Chunk-Anzahl.
func ValidatePlan(b Biome, plan []PlannedChunk) error {
	return ValidatePlanFor(b, plan, 0)
}

// ValidatePlanFor prüft wie ValidatePlan, aber gegen die Chunk-Anzahl count (Options.ChunkCount, geplante-chunks.md
// § 7) statt gegen die kürzeste Länge; count 0 ist ValidatePlan.
func ValidatePlanFor(b Biome, plan []PlannedChunk, count int) error {
	if err := checkCount(b, count); err != nil {
		return err
	}
	if len(plan) == 0 {
		return nil
	}
	g := geometryOf(b, count)
	seen, spots := make(map[int]bool, len(plan)), make(map[spotKey]bool, len(plan))
	var all, allRuins []placed
	for _, c := range plan {
		if err := checkChunk(c, b, g, seen); err != nil {
			return fmt.Errorf("auf Platz %+d: %w", c.Slot, err)
		}
		seen[c.Slot] = true
		items, err := checkObjects(c, b, g)
		if err != nil {
			return fmt.Errorf("auf Platz %+d: %w", c.Slot, err)
		}
		ruins, err := checkBuildings(c, g, items)
		if err != nil {
			return fmt.Errorf("auf Platz %+d: %w", c.Slot, err)
		}
		if err := checkGuards(c, b, g); err != nil {
			return fmt.Errorf("auf Platz %+d: %w", c.Slot, err)
		}
		if err := markSpots(spots, items, ruins); err != nil {
			return fmt.Errorf("auf Platz %+d: %w", c.Slot, err)
		}
		all, allRuins = append(all, items...), append(allRuins, ruins...)
	}
	if err := checkRuinsAcross(allRuins, all); err != nil {
		return err
	}
	return checkTotals(b, g, plan, all)
}

// markSpots trägt Objekte und Ruinen eines Chunks in spots ein; kind@x muss je Stufe eindeutig sein.
func markSpots(spots map[spotKey]bool, lists ...[]placed) error {
	for _, list := range lists {
		for _, it := range list {
			key := it.key()
			if spots[key] {
				return fmt.Errorf("%s@%v doppelt (kind@x muss je Stufe eindeutig sein)", key.kind, key.x)
			}
			spots[key] = true
		}
	}
	return nil
}

func checkChunk(c PlannedChunk, b Biome, g geometry, seen map[int]bool) error {
	switch {
	case c.Slot == 0:
		return fmt.Errorf("es gibt keinen Platz 0 (Hub)")
	case c.Kind == "hub" || c.Kind == "exit" || c.Kind == "edge":
		return fmt.Errorf("die Art %q ist nicht planbar", c.Kind)
	case c.Slot > g.usable || -c.Slot > g.usable:
		return g.beyond()
	case seen[c.Slot]:
		return fmt.Errorf("doppelt geplant")
	case !hasChunkKind(b, c.Kind):
		return fmt.Errorf("unbekannte Chunk-Art %q", c.Kind)
	}
	return nil
}

// checkObjects prüft Art, Lage und Eindeutigkeit der Objekte eines Chunks. Ein Ereignis-Chunk ohne Objektliste trägt
// sein Objekt in der Chunk-Mitte.
func checkObjects(c PlannedChunk, b Biome, g geometry) ([]placed, error) {
	if len(c.Objects) == 0 {
		if isEvent(c.Kind) || c.Kind == "portal" {
			p := placed{kind: c.Kind, slot: c.Slot, at: g.cw / 2, dist: g.dist(c.Slot, g.cw/2)}
			return []placed{p}, checkPlaced(p, b, g)
		}
		return nil, nil
	}
	out := make([]placed, 0, len(c.Objects))
	for i, o := range c.Objects {
		switch {
		case !hasObjectKind(b, o.Kind):
			return nil, fmt.Errorf("unbekannte Objekt-Art %q im Biom %s", o.Kind, b.ID)
		case slices.ContainsFunc(c.Objects[:i], func(p PlannedObject) bool { return p == o }):
			return nil, fmt.Errorf("%s@%v doppelt (kind@x muss eindeutig sein)", o.Kind, o.At)
		}
		p := placed{kind: o.Kind, slot: c.Slot, at: o.At, dist: g.dist(c.Slot, o.At), fixed: true}
		if err := checkPlaced(p, b, g); err != nil {
			return nil, err
		}
		out = append(out, p)
	}
	return out, nil
}

// checkPlaced prüft die Lage-Regeln (docs/rules/geplante-chunks.md § 4) eines Objekts.
func checkPlaced(p placed, b Biome, g geometry) error {
	switch {
	case p.kind == "portal" || p.kind == "recruitCamp":
		if p.at < 0 || p.at > g.cw {
			return fmt.Errorf("%s bei %v liegt außerhalb des Chunks (0…%v)", p.kind, p.at, g.cw)
		}
		if p.kind == "portal" {
			return checkPortal(p, b)
		}
		return nil
	case p.at < edgeGapUnits || p.at > g.cw-edgeGapUnits:
		return fmt.Errorf("%s bei %v: mindestens %d Units Abstand zur Chunk-Grenze (%d…%v)", p.kind, p.at, edgeGapUnits, edgeGapUnits, g.cw-edgeGapUnits)
	case b.Veins.Count > 0 && p.kind == b.Veins.Kind:
		return checkVein(p)
	}
	return nil
}

func checkPortal(p placed, b Biome) error {
	if p.dist < b.Portals.MinDistanceFromHubUnits {
		return fmt.Errorf("das Portal %v Units vom Hub, mindestens %v", p.dist, b.Portals.MinDistanceFromHubUnits)
	}
	if outer := sitePlacement.OutsideUnits; p.dist <= outer {
		return fmt.Errorf("das Portal %v Units vom Hub, innerhalb der Verteidigungszone (%v)", p.dist, outer)
	}
	return nil
}

// veinGapUnits ist der Mindestabstand einer festen Ader zur Farm und zum Baugrund der Zone 1 (docs/rules/geplante-chunks.md § 4, Q57); die Mauer- und
// Turm-Plätze halten ihrerseits Abstand zu Adern (sites.go blockedXs).
const veinGapUnits = 4

func checkVein(p placed) error {
	targets := append([]float64{sitePlacement.Zone1WallUnits, sitePlacement.Zone1TowerUnits}, farmOffsets...) // Baugrund der Zone 1 und Farm
	for _, site := range targets {
		if math.Abs(p.dist-site) < veinGapUnits {
			return fmt.Errorf("die Ader %v Units vom Hub, weniger als %d Units von einem Bauplatz (%v)", p.dist, veinGapUnits, site)
		}
	}
	return nil
}
