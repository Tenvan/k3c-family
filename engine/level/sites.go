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

// placeZone legt Mauern und Türme einer Zone [lo, hi) (Abstand zur Hub-Mitte) an und trägt sie in taken ein.
func (g *gen) placeZone(r *rng.Rng, p SitePlacement, side, zone int, lo, hi float64, taken *[]float64, chunks []Chunk) []WallSite {
	s := float64(side)
	free := func(d float64) bool {
		x := g.hub + s*d
		if g.lavaAt(chunks, x) {
			return false
		}
		return !slices.ContainsFunc(*taken, func(t float64) bool { return math.Abs(t-x) < payGapUnits })
	}
	pick := func(from, to float64) (float64, bool) {
		for range 40 {
			d := math.Round(from + r.Next()*(to-from))
			if d >= lo && d < hi && free(d) {
				return d, true
			}
		}
		for d := math.Ceil(lo); d < hi; d++ { // lückenlos zurückfallen, damit jede Zone mindestens je einen Platz bekommt
			if free(d) {
				return d, true
			}
		}
		return 0, false
	}
	var out []WallSite
	add := func(kind string, d float64) {
		x := g.hub + s*d
		*taken = append(*taken, x)
		out = append(out, WallSite{Side: side, Kind: kind, X: x, Zone: zone})
	}
	var walls []float64
	for range r.Int(p.PerZoneMin, p.PerZoneMax) {
		if d, ok := pick(lo, hi); ok {
			add("wall", d)
			walls = append(walls, d)
		}
	}
	for range r.Int(p.PerZoneMin, p.PerZoneMax) {
		near := len(walls) > 0 && r.Next() < p.TowerNearShare
		var d float64
		var ok bool
		if near {
			w := walls[r.Int(0, len(walls)-1)]
			sign := 1.0
			if r.Next() < 0.5 {
				sign = -1
			}
			off := math.Round(p.TowerNearMinUnits + r.Next()*(p.TowerNearMaxUnits-p.TowerNearMinUnits))
			if d = w + sign*off; d >= lo && d < hi && free(d) {
				ok = true
			} else if d = w - sign*off; d >= lo && d < hi && free(d) {
				ok = true
			}
		}
		if !ok {
			d, ok = pick(lo, hi)
		}
		if ok {
			add("tower", d)
		}
	}
	return out
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
