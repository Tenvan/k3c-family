// Package level ist der prozedurale Level-Generator (Port von src/world/levelGenerator.ts).
//
// Aufbau: [Rand/Ausgang] ... [Portal] ... [Chunks] [HUB] [Chunks] ... [Portal] ... [Ausgang/Rand].
// Biome mit `lair` bekommen zusätzlich den Bau des Endbosses vor dem Ausgang (lairX, ohne Würfel).
// Gleicher Seed => gleiches Level wie in TypeScript; geprüft gegen testdata/golden/level-*.json.
// Die Reihenfolge der RNG-Aufrufe ist Teil des Vertrags und darf sich nicht ändern.
// Die Mauerlinien (lines.go) streuen aus einem eigenen Strom `<biom>:<seed>:sites` und berühren ihn nicht.
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
	// Lines sind die Mauerlinien je Seite (links, dann rechts, je von innen); ohne JSON-Ausgabe bis B-208.
	Lines []WallLine `json:"-"`
	// Ruins sind die Ruinen der geplanten Chunks (§ 8, B-468); Plätze der Sim, nicht Teil der level-Nachricht.
	Ruins []Ruin `json:"-"`
}

// eventKinds in fester Reihenfolge wie `EVENT_KINDS` in TS.
var eventKinds = []string{"chest", "recruitCamp"}

func isEvent(kind string) bool { return kind == "chest" || kind == "recruitCamp" }

// gen hält den Zustand während der Erzeugung; "" in kinds heißt: Chunk noch frei.
type gen struct {
	b     Biome
	r     *rng.Rng
	seed  string
	kinds []string
	cw    float64
	hub   float64
	count int // eingestellte Chunk-Anzahl (Options.ChunkCount); 0 = gewürfelte Länge aus lengthUnits
	// Geplante Chunks (planned_gen.go): hubStart und hubChunks legen die Plätze fest, planned ordnet Chunk-Index
	// und Plan zu, fixed zählt feste Portale, Truhen, Camps und Adern, sideFixed feste Portale je Seite.
	lines WallLines
	hubStart, hubChunks int
	planned             map[int]PlannedChunk
	fixed               map[string]int
	sideFixed           [2]int
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

// Generate baut das Level für Biom und Seed ohne Planung und mit den eingebetteten Mauerlinien.
func Generate(b Biome, seed string) (Layout, error) { return GenerateWith(b, seed, Options{}) }

// GenerateWith baut das Level mit Planung und Mauerlinien aus o; der Nullwert von Options ergibt dasselbe Level wie
// Generate. Eine ungültige Planung wird abgelehnt (ValidatePlan).
func GenerateWith(b Biome, seed string, o Options) (Layout, error) {
	lines := o.lines()
	if err := checkCount(b, o.ChunkCount); err != nil {
		return Layout{}, err
	}
	if err := ValidatePlanFor(b, o.Planned, lines, o.ChunkCount); err != nil {
		return Layout{}, fmt.Errorf("geplante Chunks: %w", err)
	}
	g := &gen{b: b, r: rng.New(b.ID + ":" + seed), seed: seed, lines: lines, cw: b.ChunkWidthUnits, count: o.ChunkCount}
	g.frame()
	g.reserve(o.Planned)
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
	wallLines := lines.generate(b.ID, seed, g.hub)
	g.lavaOffLines(wallLines)
	chunks := make([]Chunk, len(g.kinds))
	for i, k := range g.kinds {
		chunks[i] = Chunk{Index: i, Kind: k, StartUnits: float64(i) * g.cw}
	}
	entities := g.entities(chunks)
	if b.Lair {
		entities = append(entities, Entity{Kind: "lair", X: g.lairX()})
	}
	sort.SliceStable(entities, func(i, j int) bool { return entities[i].X < entities[j].X })
	return Layout{
		Seed: seed, BiomeID: b.ID, WidthUnits: float64(len(chunks)) * g.cw, ChunkWidthUnits: g.cw,
		HubCenterUnits: g.hub, Chunks: chunks, Entities: entities, Lines: wallLines,
		Ruins: g.ruinsOf(chunks),
	}, nil
}

// hubChunksOf ist die gerade Zahl der Hub-Chunks eines Bioms. math.Round = Math.round für positive Werte.
func hubChunksOf(b Biome) int { return max(2, int(math.Round(b.HubWidthUnits/b.ChunkWidthUnits/2))*2) }

// checkCount prüft eine eingestellte Chunk-Anzahl (0 = Länge aus lengthUnits): gerade, damit der Hub mittig liegt,
// und Platz für Hub, Ausgang, Rand und je Seite mindestens einen Chunk. Die erlaubten Größen prüft engine/sim.
func checkCount(b Biome, n int) error {
	if least := hubChunksOf(b) + 4; n != 0 && (n%2 != 0 || n < least) {
		return fmt.Errorf("eingestellte Chunk-Anzahl %d in %s: gerade und mindestens %d", n, b.ID, least)
	}
	return nil
}

// frame legt Chunk-Anzahl, Hub in der Mitte, Ausgang und Rand fest. Mit eingestellter Anzahl (g.count) entfällt der
// Wurf der Länge; ohne bleibt die Reihenfolge der Würfe wie bisher.
func (g *gen) frame() {
	// Gerade Anzahl Chunks, damit der Hub exakt in der Mitte liegt.
	hubChunks := hubChunksOf(g.b)
	count := g.count
	if count == 0 {
		count = int(math.Round(float64(g.r.Int(g.b.LengthUnits.Min, g.b.LengthUnits.Max)) / g.cw))
		if count%2 != 0 {
			count++
		}
	}
	g.kinds = make([]string, count)
	hubStart := count/2 - hubChunks/2
	g.hubStart, g.hubChunks = hubStart, hubChunks
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
	if g.sideFixed[0] != g.sideFixed[1] {
		left = g.sideFixed[0] < g.sideFixed[1] // beginnt auf der Seite mit weniger festen Portalen
	}
	for n := range g.b.Portals.Count - g.fixed["portal"] {
		options := g.eligible(left)
		if len(options) == 0 {
			options = g.eligible(!left)
		}
		if len(options) == 0 {
			return fmt.Errorf("kein Platz für Portal %d/%d in %s", n+1+g.fixed["portal"], g.b.Portals.Count, g.b.ID)
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

// lairX: Der Bau des Endbosses (docs/rules/bosse.md § 1: „am Ende vor dem Eingang“, K2.1c) liegt an der inneren Kante
// des Ausgangs-Chunks. Er belegt kein eigenes Chunk und würfelt nicht: Portale, Ereignisse, Ressourcen und die Dichte
// bleiben, wie sie ohne Bau wären.
func (g *gen) lairX() float64 {
	if g.b.ExitSide == "right" {
		return float64(len(g.kinds)-1) * g.cw
	}
	return g.cw
}

// placeEvents belegt Truhen- und Camp-Chunks; Camps bevorzugt nah am Hub, aber zuerst Chunks, deren Mitte bei jeder
// Streuung auf Abstand zu allen Linien-Plätzen liegt (B-261). Die Würfe bleiben dieselben, nur die Sortierung ändert sich.
func (g *gen) placeEvents(seed string) error {
	for _, kind := range eventKinds {
		want := g.b.EventChunks[kind]
		slots := rng.Shuffle(g.r, g.free())
		count := max(0, g.r.Int(want.Min, want.Max)-g.fixed[kind]) // feste Truhen und Camps zählen mit
		if len(slots) < count {
			return fmt.Errorf("level zu kurz für %dx %s (Seed %s)", count, kind, seed)
		}
		if kind == "recruitCamp" {
			// Stabil: gleich gute Slots behalten ihre Reihenfolge aus dem Mischen.
			sort.SliceStable(slots, func(i, j int) bool {
				di, dj := g.center(slots[i])-g.hub, g.center(slots[j])-g.hub
				if ci, cj := g.lines.ClearOf(di), g.lines.ClearOf(dj); ci != cj {
					return ci
				}
				return math.Abs(di) < math.Abs(dj)
			})
		}
		for _, s := range slots[:count] {
			g.kinds[s] = kind
		}
	}
	g.swapCampsWithChests()
	return nil
}

// swapCampsWithChests: Fehlt einem Camp ein freier Chunk auf Abstand zu den Linien (kurzes Level), tauscht es den
// Chunk mit der hub-nächsten Truhe auf Abstand; Truhen sind keine Zahlziele (B-261). Kein Wurf, nur ein Tausch.
func (g *gen) swapCampsWithChests() {
	dist := func(i int) float64 { return math.Abs(g.center(i) - g.hub) }
	for i, k := range g.kinds {
		if _, fixed := g.planned[i]; k != "recruitCamp" || fixed || g.lines.ClearOf(g.center(i)-g.hub) {
			continue
		}
		best := -1
		for j, kj := range g.kinds {
			if _, fixed := g.planned[j]; kj == "chest" && !fixed && g.lines.ClearOf(g.center(j)-g.hub) && (best < 0 || dist(j) < dist(best)) {
				best = j
			}
		}
		if best >= 0 {
			g.kinds[i], g.kinds[best] = "chest", "recruitCamp"
		}
	}
}

// entities setzt Burg, Portale, Ausgang, Ereignisse, Ressourcen und Skill-Punkte.
func (g *gen) entities(chunks []Chunk) []Entity {
	out := []Entity{{Kind: "castle", X: g.hub}}
	for _, c := range chunks {
		if pc, ok := g.planned[c.Index]; ok {
			out = append(out, g.plannedEntities(c, pc)...)
			continue
		}
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
				out = append(out, Entity{Kind: res.Key, X: g.inside(g.r, c)})
			}
		}
	}
	// Skill-Punkte: versteckt in beliebigen Chunks außerhalb von Hub und Rand.
	var hideouts []Chunk
	for _, c := range chunks {
		if _, fixed := g.planned[c.Index]; c.Kind != "hub" && c.Kind != "edge" && !fixed {
			hideouts = append(hideouts, c)
		}
	}
	for range g.r.Int(g.b.SkillPoints.Min, g.b.SkillPoints.Max) {
		c := rng.Pick(g.r, hideouts)
		out = append(out, Entity{Kind: "skillPoint", X: g.inside(g.r, c)})
	}
	return append(out, g.veins(chunks)...)
}

// veins setzt die Adern (B-114) nach allen übrigen Würfen, damit bestehende Seeds gleich bleiben: je Ader ein eigener
// Ressourcen-Chunk (nicht Hub, Rand, Ausgang, Portal oder Ereignis).
func (g *gen) veins(chunks []Chunk) []Entity {
	var spots []Chunk
	for _, c := range chunks {
		if _, fixed := g.planned[c.Index]; c.Kind != "hub" && c.Kind != "edge" && c.Kind != "exit" && c.Kind != "portal" && !isEvent(c.Kind) && !fixed {
			spots = append(spots, c)
		}
	}
	var out []Entity
	for range min(max(0, g.b.Veins.Count-g.fixed[g.b.Veins.Kind]), len(spots)) { // feste Adern zählen mit
		i := g.r.Int(0, len(spots)-1)
		out = append(out, Entity{Kind: g.b.Veins.Kind, X: g.inside(g.r, spots[i])})
		spots = append(spots[:i], spots[i+1:]...)
	}
	return out
}

// inside ist eine zufällige Position aus dem Strom r mit 2 Units Abstand zu den Chunk-Grenzen.
// float64(…) rundet das Produkt vor der Addition, sonst darf Go zu FMA fusionieren (B-071).
func (g *gen) inside(r *rng.Rng, c Chunk) float64 {
	return c.StartUnits + 2 + float64(r.Next()*(g.cw-4))
}

func weights(o Ordered[float64]) []rng.Weight {
	out := make([]rng.Weight, len(o))
	for i, e := range o {
		out[i] = rng.Weight{Key: e.Key, W: e.Value}
	}
	return out
}

// lavaOffLines: Ein Lava-Chunk, dessen Streifen (Chunk-Mitte ± widthUnits/2, dazu payGapUnits Abstand) Mauer, Turm
// oder Tor einer Linie trifft, wird ohne Würfel zur ersten anderen Art aus chunkWeights (B-291). Geplante Chunks bleiben.
func (g *gen) lavaOffLines(lines []WallLine) {
	if g.b.Lava.WidthUnits <= 0 {
		return
	}
	reach := g.b.Lava.WidthUnits/2 + payGapUnits
	other := ""
	for _, e := range g.b.ChunkWeights {
		if e.Key != "lava" && other == "" {
			other = e.Key
		}
	}
	for i, k := range g.kinds {
		if _, fixed := g.planned[i]; k != "lava" || fixed || other == "" {
			continue
		}
		for _, l := range lines {
			if math.Abs(l.Wall-g.center(i)) < reach || math.Abs(l.Tower-g.center(i)) < reach || math.Abs(l.Gate-g.center(i)) < reach {
				g.kinds[i] = other
				break
			}
		}
	}
}
