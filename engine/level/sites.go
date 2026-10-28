package level

import (
	"math"
	"slices"

	"k3c/engine/rng"
)

// WallSite ist ein verteilter Mauer- oder Turm-Platz einer Seite (B-652): absolute Position in Units, Zone n von innen
// (ab 1). Ersetzt die festen Mauerlinien; bis BS12.2 gibt der Generator beides aus.
type WallSite struct {
	Side int     // -1 links, +1 rechts
	Kind string  // "wall" oder "tower"
	X    float64 // absolute Lage in Units
	Zone int     // Zone n von innen, ab 1
}

// SitePlacement sind die Felder aus data/hub.json › sitePlacement (B-652).
type SitePlacement struct {
	ZoneStartUnits    float64 `json:"zoneStartUnits"`    // Abstand der ersten Zone zur Hub-Mitte
	Zones             int     `json:"zones"`             // Zonen je Seite (Zone n ab Hub-Stufe n sichtbar)
	PerZoneMin        int     `json:"perZoneMin"`        // Mauern und Türme je Zone, mindestens
	PerZoneMax        int     `json:"perZoneMax"`        // und höchstens
	TowerNearShare    float64 `json:"towerNearShare"`    // Anteil der Türme neben einer Mauer
	TowerNearMinUnits float64 `json:"towerNearMinUnits"` // Abstand eines nahen Turms zur Mauer, von
	TowerNearMaxUnits float64 `json:"towerNearMaxUnits"` // bis
	ClearRadiusUnits  float64 `json:"clearRadiusUnits"`  // Freiraum von Bäumen und Felsen (Sim, BS12.2)
	EdgeMarginUnits   float64 `json:"edgeMarginUnits"`   // Abstand zum Level-Rand und zum nächsten Portal
}

// sitePlacement ist der eingebettete Stand aus data/hub.json, nur lesen.
var sitePlacement = loadSitePlacement()

// DefaultSitePlacement liefert die eingebetteten Werte (data/hub.json › sitePlacement).
func DefaultSitePlacement() SitePlacement { return sitePlacement }

func loadSitePlacement() SitePlacement {
	var hub struct {
		SitePlacement SitePlacement `json:"sitePlacement"`
	}
	readData("hub.json", &hub)
	return hub.SitePlacement
}

// placeSites verteilt je Seite und Zone 1 bis PerZoneMax Mauern und Türme aus dem eigenen Strom `<biom>:<seed>:wallsites`
// (die übrigen Ströme bleiben unberührt). Mindestabstand zweier Zahlziele ist payGapUnits; nahe Türme stehen
// TowerNearMin…Max neben einer Mauer der Zone. Reihenfolge der Würfe: links, dann rechts, je Zone von innen.
func (g *gen) placeSites(entities []Entity, chunks []Chunk) []WallSite {
	p := sitePlacement
	r := rng.New(g.b.ID + ":" + g.seed + ":wallsites")
	taken := g.blockedXs(entities)
	var out []WallSite
	for _, side := range []int{-1, 1} {
		limit := g.sideLimit(side, entities, p.EdgeMarginUnits)
		for zone := 1; zone <= p.Zones; zone++ {
			lo := p.ZoneStartUnits + float64(zone-1)*g.cw
			hi := math.Min(lo+g.cw, limit)
			if hi-lo < 2*payGapUnits {
				continue
			}
			zoneSites := g.placeZone(r, p, side, zone, lo, hi, &taken, chunks)
			out = append(out, zoneSites...)
		}
	}
	return out
}

// zonePlacer legt die Plätze einer Zone [lo, hi) (Abstand zur Hub-Mitte) einer Seite an und trägt sie in taken ein.
type zonePlacer struct {
	g      *gen
	r      *rng.Rng
	p      SitePlacement
	side   int
	zone   int
	lo, hi float64
	taken  *[]float64
	chunks []Chunk
	walls  []float64
	out    []WallSite
}

func (g *gen) placeZone(r *rng.Rng, p SitePlacement, side, zone int, lo, hi float64, taken *[]float64, chunks []Chunk) []WallSite {
	z := &zonePlacer{g: g, r: r, p: p, side: side, zone: zone, lo: lo, hi: hi, taken: taken, chunks: chunks}
	for range r.Int(p.PerZoneMin, p.PerZoneMax) {
		if d, ok := z.pick(lo, hi); ok {
			z.add("wall", d)
			z.walls = append(z.walls, d)
		}
	}
	for range r.Int(p.PerZoneMin, p.PerZoneMax) {
		d, ok := z.nearWall()
		if !ok {
			d, ok = z.pick(lo, hi)
		}
		if ok {
			z.add("tower", d)
		}
	}
	return z.out
}

// free: d (Abstand zur Hub-Mitte) liegt in der Zone, nicht in Lava und hält payGapUnits zu allen belegten Orten.
func (z *zonePlacer) free(d float64) bool {
	if d < z.lo || d >= z.hi {
		return false
	}
	x := z.g.hub + float64(z.side)*d
	if z.g.lavaAt(z.chunks, x) {
		return false
	}
	return !slices.ContainsFunc(*z.taken, func(t float64) bool { return math.Abs(t-x) < payGapUnits })
}

// pick würfelt einen freien Abstand in [from, to]; findet das nichts, nimmt es lückenlos den ersten freien der Zone.
func (z *zonePlacer) pick(from, to float64) (float64, bool) {
	for range 40 {
		if d := math.Round(from + z.r.Next()*(to-from)); z.free(d) {
			return d, true
		}
	}
	for d := math.Ceil(z.lo); d < z.hi; d++ {
		if z.free(d) {
			return d, true
		}
	}
	return 0, false
}

// nearWall würfelt, ob ein Turm neben einer Mauer der Zone steht (towerNearShare), und liefert dessen Abstand.
func (z *zonePlacer) nearWall() (float64, bool) {
	if len(z.walls) == 0 || z.r.Next() >= z.p.TowerNearShare {
		return 0, false
	}
	w := z.walls[z.r.Int(0, len(z.walls)-1)]
	sign := 1.0
	if z.r.Next() < 0.5 {
		sign = -1
	}
	off := math.Round(z.p.TowerNearMinUnits + z.r.Next()*(z.p.TowerNearMaxUnits-z.p.TowerNearMinUnits))
	for _, d := range []float64{w + sign*off, w - sign*off} {
		if z.free(d) {
			return d, true
		}
	}
	return 0, false
}

func (z *zonePlacer) add(kind string, d float64) {
	x := z.g.hub + float64(z.side)*d
	*z.taken = append(*z.taken, x)
	z.out = append(z.out, WallSite{Side: z.side, Kind: kind, X: x, Zone: z.zone})
}

// blockedXs sind die Orte, von denen jedes Zahlziel payGapUnits Abstand halten muss: Hub-Plätze, Burg und alle
// Ereignisse, Portale, Ausgang und Bau.
func (g *gen) blockedXs(entities []Entity) []float64 {
	var out []float64
	for _, h := range hubSites {
		out = append(out, g.hub+h.offset)
	}
	for _, e := range entities {
		switch e.Kind {
		case "castle", "portal", "exit", "chest", "recruitCamp", "lair":
			out = append(out, e.X)
		}
	}
	return out
}

// sideLimit ist die größte Entfernung zur Hub-Mitte auf einer Seite: kurz vor dem nächsten Portal bzw. Level-Rand.
func (g *gen) sideLimit(side int, entities []Entity, margin float64) float64 {
	limit := float64(len(g.kinds))*g.cw/2 - margin
	for _, e := range entities {
		if e.Kind != "portal" && e.Kind != "exit" {
			continue
		}
		if d := (e.X - g.hub) * float64(side); d > 0 && d-margin < limit {
			limit = d - margin
		}
	}
	return limit
}

// lavaAt: x liegt in einem Lava-Chunk.
func (g *gen) lavaAt(chunks []Chunk, x float64) bool {
	i := int(x / g.cw)
	return i >= 0 && i < len(chunks) && chunks[i].Kind == "lava"
}

// freeGround liefert den Baugrund (B-652, 🧑 2026-10-28, Variante A): Je Seite liegen die erste Mauer und der erste Turm der
// Zone 1 auf freiem Grund; die Sim legt im Freiraum dieser Plätze keine Bäume und Felsen an (World, FreeGround). Ohne das ist im
// Wald vor der ersten Nacht kein Platz bezahlbar. Die Level-Entitäten bleiben unverändert (Vorschau, Umwandlung in feste Objekte).
func freeGround(sites []WallSite) []float64 {
	var free []float64
	for _, side := range []int{-1, 1} {
		for _, kind := range []string{"wall", "tower"} {
			if i := slices.IndexFunc(sites, func(s WallSite) bool { return s.Side == side && s.Zone == 1 && s.Kind == kind }); i >= 0 {
				free = append(free, sites[i].X)
			}
		}
	}
	return free
}

// OnFreeGround: x liegt im Freiraum eines Baugrund-Platzes (Layout.FreeGround).
func (l Layout) OnFreeGround(x float64) bool {
	return slices.ContainsFunc(l.FreeGround, func(g float64) bool { return math.Abs(x-g) < sitePlacement.ClearRadiusUnits })
}
