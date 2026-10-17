package level

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math"
	"slices"
)

// Geplante Chunks (docs/rules/geplante-chunks.md, B-461): feste Gegenden je Insel und Tiefe. Dieses Paket prüft die
// Planung unabhängig vom Seed gegen die kürzeste mögliche Länge des Bioms und jede Streuung der Mauerlinien.
// Die Linien sind ein Parameter (kein globaler Zustand); DefaultWallLines liefert die eingebetteten Daten.

// veinGapUnits ist der Mindestabstand einer festen Ader zu jedem Bauplatz der Mauerlinien und zur Farm
// (docs/rules/geplante-chunks.md § 4, Q57).
const veinGapUnits = 4

// edgeGapUnits ist der Abstand von Ressourcen, Truhen und Adern zur Chunk-Grenze (wie `inside` im Generator).
const edgeGapUnits = 2

// PlannedObject ist ein festes Objekt eines geplanten Chunks; At zählt in Units ab Chunk-Anfang.
type PlannedObject struct {
	Kind string  `json:"kind"`
	At   float64 `json:"at"`
}

// PlannedChunk ist ein geplanter Chunk. Slot zählt relativ zu den Hub-Chunks (−n links, +n rechts, nie 0).
// Buildings und Enemies sind für Teil 2 (B-468) im Format reserviert und werden noch abgelehnt.
type PlannedChunk struct {
	Slot      int             `json:"slot"`
	Kind      string          `json:"kind"`
	Seed      string          `json:"seed,omitempty"`
	Objects   []PlannedObject `json:"objects,omitempty"`
	Buildings json.RawMessage `json:"buildings,omitempty"`
	Enemies   json.RawMessage `json:"enemies,omitempty"`
}

// UnmarshalJSON liest streng: ein unbekanntes Feld (z. B. ein Tippfehler wie `object`) ist ein Fehler (fail closed).
func (c *PlannedChunk) UnmarshalJSON(raw []byte) error {
	type plain PlannedChunk
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields()
	return dec.Decode((*plain)(c))
}

// geometry beschreibt die kürzeste mögliche Länge eines Bioms in Chunks.
type geometry struct {
	cw      float64 // Chunk-Breite
	hubHalf float64 // halbe Breite der Hub-Chunks
	usable  int     // planbare Plätze je Seite bei kürzester Länge (ohne Ausgang und Rand)
}

// shortest rechnet wie `frame` im Generator mit der kleinsten Länge.
func shortest(b Biome) geometry {
	cw := b.ChunkWidthUnits
	hubChunks := max(2, int(math.Round(b.HubWidthUnits/cw/2))*2)
	count := int(math.Round(float64(b.LengthUnits.Min) / cw))
	if count%2 != 0 {
		count++
	}
	return geometry{cw: cw, hubHalf: float64(hubChunks) * cw / 2, usable: (count-hubChunks)/2 - 1}
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
func ValidatePlan(b Biome, plan []PlannedChunk, lines WallLineData) error {
	if len(plan) == 0 {
		return nil
	}
	g := shortest(b)
	seen, spots := make(map[int]bool, len(plan)), make(map[spotKey]bool, len(plan))
	all := make([]placed, 0, len(plan))
	for _, c := range plan {
		if err := checkChunk(c, b, g, seen); err != nil {
			return fmt.Errorf("auf Platz %+d: %w", c.Slot, err)
		}
		seen[c.Slot] = true
		items, err := checkObjects(c, b, g, lines)
		if err != nil {
			return fmt.Errorf("auf Platz %+d: %w", c.Slot, err)
		}
		for _, it := range items {
			key := it.key()
			if spots[key] {
				return fmt.Errorf("auf Platz %+d: %s@%v doppelt (kind@x muss je Stufe eindeutig sein)", c.Slot, key.kind, key.x)
			}
			spots[key] = true
		}
		all = append(all, items...)
	}
	return checkTotals(b, g, plan, all)
}

func checkChunk(c PlannedChunk, b Biome, g geometry, seen map[int]bool) error {
	switch {
	case c.Slot == 0:
		return fmt.Errorf("es gibt keinen Platz 0 (Hub)")
	case c.Kind == "hub" || c.Kind == "exit" || c.Kind == "edge":
		return fmt.Errorf("die Art %q ist nicht planbar", c.Kind)
	case c.Slot > g.usable || -c.Slot > g.usable:
		return fmt.Errorf("jenseits der kürzesten Länge (höchstens ±%d)", g.usable)
	case seen[c.Slot]:
		return fmt.Errorf("doppelt geplant")
	case !hasChunkKind(b, c.Kind):
		return fmt.Errorf("unbekannte Chunk-Art %q", c.Kind)
	case notEmpty(c.Buildings), notEmpty(c.Enemies):
		return fmt.Errorf("planbar sind nur Art, Seed und Objekte; Gebäude und Gegner sind noch nicht unterstützt (Teil 2, B-468)")
	}
	return nil
}

func notEmpty(raw json.RawMessage) bool {
	t := bytes.TrimSpace(raw)
	return len(t) > 0 && string(t) != "null" && string(t) != "[]" && string(t) != "{}"
}

// checkObjects prüft Art, Lage und Eindeutigkeit der Objekte eines Chunks. Ein Ereignis-Chunk ohne Objektliste trägt
// sein Objekt in der Chunk-Mitte.
func checkObjects(c PlannedChunk, b Biome, g geometry, lines WallLineData) ([]placed, error) {
	if len(c.Objects) == 0 {
		if isEvent(c.Kind) || c.Kind == "portal" {
			p := placed{kind: c.Kind, slot: c.Slot, at: g.cw / 2, dist: g.dist(c.Slot, g.cw/2)}
			return []placed{p}, checkPlaced(p, b, g, lines)
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
		if err := checkPlaced(p, b, g, lines); err != nil {
			return nil, err
		}
		out = append(out, p)
	}
	return out, nil
}

// checkPlaced prüft die Lage-Regeln (docs/rules/geplante-chunks.md § 4) eines Objekts.
func checkPlaced(p placed, b Biome, g geometry, lines WallLineData) error {
	switch {
	case p.kind == "portal" || p.kind == "recruitCamp":
		if p.at < 0 || p.at > g.cw {
			return fmt.Errorf("%s bei %v liegt außerhalb des Chunks (0…%v)", p.kind, p.at, g.cw)
		}
		if p.kind == "portal" {
			return checkPortal(p, b, lines)
		}
		return nil
	case p.at < edgeGapUnits || p.at > g.cw-edgeGapUnits:
		return fmt.Errorf("%s bei %v: mindestens %d Units Abstand zur Chunk-Grenze (%d…%v)", p.kind, p.at, edgeGapUnits, edgeGapUnits, g.cw-edgeGapUnits)
	case b.Veins.Count > 0 && p.kind == b.Veins.Kind:
		return checkVein(p, lines)
	}
	return nil
}

func checkPortal(p placed, b Biome, lines WallLineData) error {
	if p.dist < b.Portals.MinDistanceFromHubUnits {
		return fmt.Errorf("das Portal %v Units vom Hub, mindestens %v", p.dist, b.Portals.MinDistanceFromHubUnits)
	}
	if outer := lines.outermostGate(); p.dist <= outer {
		return fmt.Errorf("das Portal %v Units vom Hub, innerhalb des äußersten Tors (%v)", p.dist, outer)
	}
	return nil
}

func checkVein(p placed, lines WallLineData) error {
	for _, site := range lines.sites() {
		if math.Abs(p.dist-site) < veinGapUnits {
			return fmt.Errorf("die Ader %v Units vom Hub, weniger als %d Units von einem Bauplatz (%v)", p.dist, veinGapUnits, site)
		}
	}
	return nil
}

// sites sind alle Bauplätze der Linien (Mauer, Turm, Tor, jede ganzzahlige Streuung) und die Farm als Abstand zum Hub.
func (d WallLineData) sites() []float64 {
	out := slices.Clone(farmOffsets)
	for k, offset := range d.WallUnits {
		for j := 0; j <= d.jitterOf(k); j++ {
			wall := offset + float64(j)
			out = append(out, wall-d.TowerInsetUnits, wall, wall+d.GateOutsetUnits)
		}
	}
	return out
}

func (d WallLineData) jitterOf(k int) int {
	if k < d.FixedLines {
		return 0
	}
	return d.JitterOutwardUnits
}

// outermostGate ist die größte Torlage aller Linien bei stärkster Streuung.
func (d WallLineData) outermostGate() float64 {
	outer := 0.0
	for k, offset := range d.WallUnits {
		outer = math.Max(outer, offset+float64(d.jitterOf(k))+d.GateOutsetUnits)
	}
	return outer
}
