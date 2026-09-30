// Package level ist der prozedurale Level-Generator (Port von src/world/levelGenerator.ts).
//
// Aufbau: [Rand/Ausgang] ... [Portal] ... [Chunks] [HUB] [Chunks] ... [Portal] ... [Ausgang/Rand].
// Gleicher Seed => gleiches Level wie in TypeScript; geprüft gegen testdata/golden/level-*.json.
// Die Reihenfolge der RNG-Aufrufe ist Teil des Vertrags und darf sich nicht ändern.
package level

import (
	"fmt"
	"math"
	"sort"

	"k3c/engine/rng"
)

// Chunk ist ein Abschnitt fester Breite.
type Chunk struct {
	Index      int     `json:"index"`
	Kind       string  `json:"kind"`
	StartUnits float64 `json:"startUnits"`
}

// Entity ist ein Objekt im Level (Burg, Portal, Baum, Truhe, Skill-Punkt …).
type Entity struct {
	Kind string  `json:"kind"`
	X    float64 `json:"x"`
}

// Layout ist ein fertiges Level, gleiche JSON-Form wie `LevelLayout` in TS.
type Layout struct {
	Seed            string   `json:"seed"`
	BiomeID         string   `json:"biomeId"`
	WidthUnits      float64  `json:"widthUnits"`
	ChunkWidthUnits float64  `json:"chunkWidthUnits"`
	HubCenterUnits  float64  `json:"hubCenterUnits"`
	Chunks          []Chunk  `json:"chunks"`
	Entities        []Entity `json:"entities"`
}

// eventKinds in fester Reihenfolge wie `EVENT_KINDS` in TS.
var eventKinds = []string{"chest", "recruitCamp"}

func isEvent(kind string) bool { return kind == "chest" || kind == "recruitCamp" }

// gen hält den Zustand während der Erzeugung; "" in kinds heißt: Chunk noch frei.
type gen struct {
	b     Biome
	r     *rng.Rng
	kinds []string
	cw    float64
	hub   float64
}

func (g *gen) center(i int) float64 { return float64(float64(i)*g.cw) + g.cw/2 }

func (g *gen) free() []int {
	var out []int
	for i, k := range g.kinds {
		if k == "" {
			out = append(out, i)
		}
	}
	return out
}

// Generate baut das Level für Biom und Seed.
func Generate(b Biome, seed string) (Layout, error) {
	g := &gen{b: b, r: rng.New(b.ID + ":" + seed), cw: b.ChunkWidthUnits}
	g.frame()
	if err := g.placePortals(); err != nil {
		return Layout{}, err
	}
	if err := g.placeEvents(seed); err != nil {
		return Layout{}, err
	}
	w := weights(b.ChunkWeights)
	for _, i := range g.free() {
		g.kinds[i] = g.r.Weighted(w)
	}
	chunks := make([]Chunk, len(g.kinds))
	for i, k := range g.kinds {
		chunks[i] = Chunk{Index: i, Kind: k, StartUnits: float64(i) * g.cw}
	}
	entities := g.entities(chunks)
	sort.SliceStable(entities, func(i, j int) bool { return entities[i].X < entities[j].X })
	return Layout{
		Seed: seed, BiomeID: b.ID, WidthUnits: float64(len(chunks)) * g.cw, ChunkWidthUnits: g.cw,
		HubCenterUnits: g.hub, Chunks: chunks, Entities: entities,
	}, nil
}

// frame legt Chunk-Anzahl, Hub in der Mitte, Ausgang und Rand fest.
func (g *gen) frame() {
	// Gerade Anzahl Chunks, damit der Hub exakt in der Mitte liegt. math.Round = Math.round für positive Werte.
	hubChunks := max(2, int(math.Round(g.b.HubWidthUnits/g.cw/2))*2)
	count := int(math.Round(float64(g.r.Int(g.b.LengthUnits.Min, g.b.LengthUnits.Max)) / g.cw))
	if count%2 != 0 {
		count++
	}
	g.kinds = make([]string, count)
	hubStart := count/2 - hubChunks/2
	for i := range hubChunks {
		g.kinds[hubStart+i] = "hub"
	}
	exit, edge := 0, count-1
	if g.b.ExitSide == "right" {
		exit, edge = count-1, 0
	}
	g.kinds[exit] = "exit"
	g.kinds[edge] = "edge"
	g.hub = float64(count/2) * g.cw
}

// placePortals verteilt die Portale abwechselnd links/rechts mit Mindestabstand zum Hub, Startseite zufällig.
func (g *gen) placePortals() error {
	left := g.r.Next() < 0.5
	for n := range g.b.Portals.Count {
		options := g.eligible(left)
		if len(options) == 0 {
			options = g.eligible(!left)
		}
		if len(options) == 0 {
			return fmt.Errorf("kein Platz für Portal %d/%d in %s", n+1, g.b.Portals.Count, g.b.ID)
		}
		g.kinds[rng.Pick(g.r, options)] = "portal"
		left = !left
	}
	return nil
}

func (g *gen) eligible(left bool) []int {
	var out []int
	for i, k := range g.kinds {
		dx := g.center(i) - g.hub
		onSide := dx > 0
		if left {
			onSide = dx < 0
		}
		if k == "" && onSide && math.Abs(dx) >= g.b.Portals.MinDistanceFromHubUnits {
			out = append(out, i)
		}
	}
	return out
}

// placeEvents belegt Truhen- und Camp-Chunks; Camps bevorzugt nah am Hub.
func (g *gen) placeEvents(seed string) error {
	for _, kind := range eventKinds {
		want := g.b.EventChunks[kind]
		slots := rng.Shuffle(g.r, g.free())
		count := g.r.Int(want.Min, want.Max)
		if len(slots) < count {
			return fmt.Errorf("level zu kurz für %dx %s (Seed %s)", count, kind, seed)
		}
		if kind == "recruitCamp" {
			// Stabil wie Array.prototype.sort: gleich weit entfernte Slots behalten ihre Reihenfolge.
			sort.SliceStable(slots, func(i, j int) bool {
				return math.Abs(g.center(slots[i])-g.hub) < math.Abs(g.center(slots[j])-g.hub)
			})
		}
		for _, s := range slots[:count] {
			g.kinds[s] = kind
		}
	}
	return nil
}

// entities setzt Burg, Portale, Ausgang, Ereignisse, Ressourcen und Skill-Punkte.
func (g *gen) entities(chunks []Chunk) []Entity {
	out := []Entity{{Kind: "castle", X: g.hub}}
	for _, c := range chunks {
		switch {
		case c.Kind == "hub" || c.Kind == "edge":
			continue
		case c.Kind == "exit" || c.Kind == "portal" || isEvent(c.Kind):
			out = append(out, Entity{Kind: c.Kind, X: c.StartUnits + g.cw/2})
			continue
		}
		resources, _ := g.b.ResourcesPerChunk.Get(c.Kind)
		for _, res := range resources {
			for range g.r.Int(res.Value[0], res.Value[1]) {
				out = append(out, Entity{Kind: res.Key, X: g.inside(c)})
			}
		}
	}
	// Skill-Punkte: versteckt in beliebigen Chunks außerhalb von Hub und Rand.
	var hideouts []Chunk
	for _, c := range chunks {
		if c.Kind != "hub" && c.Kind != "edge" {
			hideouts = append(hideouts, c)
		}
	}
	for range g.r.Int(g.b.SkillPoints.Min, g.b.SkillPoints.Max) {
		c := rng.Pick(g.r, hideouts)
		out = append(out, Entity{Kind: "skillPoint", X: g.inside(c)})
	}
	return out
}

// inside ist eine zufällige Position mit 2 Units Abstand zu den Chunk-Grenzen.
// float64(…) rundet das Produkt vor der Addition, sonst darf Go zu FMA fusionieren (B-071).
func (g *gen) inside(c Chunk) float64 {
	return c.StartUnits + 2 + float64(g.r.Next()*(g.cw-4))
}

func weights(o Ordered[float64]) []rng.Weight {
	out := make([]rng.Weight, len(o))
	for i, e := range o {
		out[i] = rng.Weight{Key: e.Key, W: e.Value}
	}
	return out
}
