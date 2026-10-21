package level

import (
	"fmt"
	"math"
	"slices"
)

// Ruinen (docs/rules/geplante-chunks.md § 8, B-468): ein gebautes, stark beschädigtes Nutzgebäude an fester Lage eines
// geplanten Chunks. Dieses Paket prüft Art und Lage und gibt die Ruinen je Stufe weiter (Layout.Ruins); als Platz in
// der Sim entstehen sie in engine/sim. Werte aus data/ruins.json.

// ruinGapUnits ist der Mindestabstand einer Ruine zum Farm-Weltplatz, zu Adern, Portalen und festen Objekten. Zu den
// Bauplätzen der Mauerlinien gilt keiner: Ruinen dürfen überall liegen, die Lage wählt die Planung (§ 8).
const ruinGapUnits = 4

// hubOnlyKinds gehören dem Hub (Burg, Linien, Stufenwechsel) und sind nie Ruinen.
var hubOnlyKinds = []string{"castle", "wall", "tower", "gate", "stairsUp", "stairsDown"}

// PlannedBuilding ist die Ruine eines geplanten Chunks; At zählt in Units ab Chunk-Anfang.
type PlannedBuilding struct {
	Kind string  `json:"kind"`
	At   float64 `json:"at"`
}

// Ruin ist eine Ruine im Level: Art und X-Position in Units (Chunk-Anfang + At). Sie ist kein Level-Objekt
// (Layout.Entities), sondern ein Platz der Sim.
type Ruin struct {
	Kind string
	X    float64
}

// RuinRules sind die Werte aus data/ruins.json.
type RuinRules struct {
	HPShare float64  `json:"hpShare"`
	Kinds   []string `json:"kinds"`
}

// buildingInfo ist der Teil eines Eintrags aus data/buildings.json, den die Ruinen brauchen.
type buildingInfo struct {
	HP         float64 `json:"hp"`
	Plantation *struct {
		StripUnits float64 `json:"stripUnits"`
	} `json:"plantation"`
}

var ruinRules, ruinBuildings = loadRuinRules()

// DefaultRuinRules liefert eine Kopie der eingebetteten Ruinen-Werte (data/ruins.json).
func DefaultRuinRules() RuinRules {
	r := ruinRules
	r.Kinds = slices.Clone(r.Kinds)
	return r
}

func loadRuinRules() (RuinRules, map[string]buildingInfo) {
	var r RuinRules
	var buildings map[string]buildingInfo
	readData("ruins.json", &r)
	readData("buildings.json", &buildings)
	if err := checkRuinRules(r, buildings); err != nil {
		panic(fmt.Errorf("ruins.json: %w", err))
	}
	return r, buildings
}

// checkRuinRules prüft die Werte gegen die Gebäude: Anteil in (0, 1], jede Art bekannt und kein Hub-Gebäude.
func checkRuinRules(r RuinRules, buildings map[string]buildingInfo) error {
	if !(r.HPShare > 0 && r.HPShare <= 1) {
		return fmt.Errorf("hpShare %v liegt nicht in (0, 1]", r.HPShare)
	}
	if len(r.Kinds) == 0 {
		return fmt.Errorf("keine Art erlaubt")
	}
	for _, k := range r.Kinds {
		if slices.Contains(hubOnlyKinds, k) {
			return fmt.Errorf("%q gehört dem Hub und ist keine Ruine", k)
		}
		if _, ok := buildings[k]; !ok {
			return fmt.Errorf("unbekannte Gebäudeart %q", k)
		}
	}
	return nil
}

// RuinHP ist die Start-HP einer Ruine der Art kind: HPShare × HP der Art, aufgerundet. Nur für erlaubte Arten
// (die Prüfung der Planung lässt keine anderen durch); eine unbekannte Art ergibt 0.
func RuinHP(kind string) float64 {
	return math.Ceil(ruinRules.HPShare * ruinBuildings[kind].HP)
}

// checkBuildings prüft die Ruine eines Chunks (höchstens eine) gegen die Lage-Regeln aus § 8. near sind die festen
// Objekte des Chunks. Das Ergebnis geht in die Prüfung von kind@x ein.
func checkBuildings(c PlannedChunk, g geometry, near []placed) ([]placed, error) {
	if len(c.Buildings) == 0 {
		return nil, nil
	}
	if len(c.Buildings) > 1 {
		return nil, fmt.Errorf("höchstens eine Ruine je Chunk, geplant sind %d", len(c.Buildings))
	}
	b := c.Buildings[0]
	if !slices.Contains(ruinRules.Kinds, b.Kind) {
		return nil, fmt.Errorf("%q ist als Ruine nicht erlaubt (erlaubt: %v)", b.Kind, ruinRules.Kinds)
	}
	p := placed{kind: b.Kind, slot: c.Slot, at: b.At, dist: g.dist(c.Slot, b.At), fixed: true}
	if err := checkRuinPlace(p, g, near); err != nil {
		return nil, err
	}
	return []placed{p}, nil
}

func checkRuinPlace(p placed, g geometry, near []placed) error {
	edge := float64(edgeGapUnits)
	if pl := ruinBuildings[p.kind].Plantation; pl != nil {
		edge = math.Max(edge, pl.StripUnits)
	}
	if p.at < edge || p.at > g.cw-edge {
		return fmt.Errorf("die Ruine %s bei %v: mindestens %v Units Abstand zur Chunk-Grenze (%v…%v)", p.kind, p.at, edge, edge, g.cw-edge)
	}
	for _, site := range farmOffsets {
		if math.Abs(p.dist-site) < ruinGapUnits {
			return fmt.Errorf("die Ruine %s %v Units vom Hub, weniger als %d Units vom Farm-Weltplatz (%v)", p.kind, p.dist, ruinGapUnits, site)
		}
	}
	for _, o := range near {
		if math.Abs(p.at-o.at) < ruinGapUnits {
			return fmt.Errorf("die Ruine %s bei %v, weniger als %d Units von %s bei %v", p.kind, p.at, ruinGapUnits, o.kind, o.at)
		}
	}
	return nil
}

// checkRuinsAcross prüft den Abstand jeder Ruine zu festen Objekten anderer Chunks: Portale und Camps dürfen bis an
// die Chunk-Grenze liegen, Ressourcen, Truhen und Adern halten dort schon den Rand-Abstand.
func checkRuinsAcross(ruins, objects []placed) error {
	for _, r := range ruins {
		for _, o := range objects {
			if o.slot != r.slot && math.Abs(r.key().x-o.key().x) < ruinGapUnits {
				return fmt.Errorf("auf Platz %+d: Ruine %s bei %v, weniger als %d Units von %s auf Platz %+d", r.slot, r.kind, r.at, ruinGapUnits, o.kind, o.slot)
			}
		}
	}
	return nil
}

// ruinsOf sind die Ruinen der geplanten Chunks mit ihrer X-Position; ohne Würfe, die Ströme bleiben unberührt.
func (g *gen) ruinsOf(chunks []Chunk) []Ruin {
	var out []Ruin
	for _, c := range chunks {
		if pc, ok := g.planned[c.Index]; ok {
			for _, b := range pc.Buildings {
				out = append(out, Ruin{Kind: b.Kind, X: c.StartUnits + b.At})
			}
		}
	}
	return out
}
