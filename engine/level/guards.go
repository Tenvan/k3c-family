package level

import (
	"fmt"
	"math"
	"slices"
)

// Gegnerlager (docs/rules/geplante-chunks.md § 9, B-517): feste Wachen eines geplanten Chunks. Dieses Paket prüft Art und
// Lage und gibt die Wachen je Stufe weiter (Layout.Guards); als Gegner entstehen sie in engine/sim. Werte aus
// data/guards.json.

// PlannedEnemy ist eine Wache eines Gegnerlagers; At zählt in Units ab Chunk-Anfang.
type PlannedEnemy struct {
	Kind string  `json:"kind"`
	At   float64 `json:"at"`
}

// Guard ist eine Wache im Level: Gegnerart und X-Position in Units (Chunk-Anfang + At). Sie ist kein Level-Objekt
// (Layout.Entities), sondern ein Gegner der Sim.
type Guard struct {
	Kind string
	X    float64
}

// GuardRules sind die Werte aus data/guards.json.
type GuardRules struct {
	AggroRadiusUnits float64 `json:"aggroRadiusUnits"`
	MaxPerChunk      int     `json:"maxPerChunk"`
	MinGapUnits      float64 `json:"minGapUnits"`
}

var guardRules = loadGuardRules()

// DefaultGuardRules liefert die eingebetteten Werte (data/guards.json).
func DefaultGuardRules() GuardRules { return guardRules }

func loadGuardRules() GuardRules {
	var r GuardRules
	readData("guards.json", &r)
	if err := checkGuardRules(r); err != nil {
		panic(fmt.Errorf("guards.json: %w", err))
	}
	return r
}

// checkGuardRules prüft die Werte: Radius, Anzahl und Abstand sind positiv.
func checkGuardRules(r GuardRules) error {
	switch {
	case !(r.AggroRadiusUnits > 0):
		return fmt.Errorf("aggroRadiusUnits %v ist nicht positiv", r.AggroRadiusUnits)
	case r.MaxPerChunk < 1:
		return fmt.Errorf("maxPerChunk %d ist kleiner als 1", r.MaxPerChunk)
	case !(r.MinGapUnits > 0):
		return fmt.Errorf("minGapUnits %v ist nicht positiv", r.MinGapUnits)
	}
	return nil
}

// checkGuards prüft das Gegnerlager eines Chunks: Anzahl, Art im Biom (Portal-Pool), Lage ≥ 2 Units von der Chunk-Grenze und
// außerhalb des äußersten Tors, Abstand der Wachen untereinander (§ 9). Fehler nennen die Wache.
func checkGuards(c PlannedChunk, b Biome, g geometry, lines WallLines) error {
	if len(c.Enemies) > guardRules.MaxPerChunk {
		return fmt.Errorf("höchstens %d Wachen je Chunk, geplant sind %d", guardRules.MaxPerChunk, len(c.Enemies))
	}
	outer := lines.outermostGate()
	for i, e := range c.Enemies {
		switch {
		case !slices.Contains(b.Enemies.Portal, e.Kind):
			return fmt.Errorf("die Wache %q ist im Biom %s nicht erlaubt (erlaubt: %v)", e.Kind, b.ID, b.Enemies.Portal)
		case e.At < edgeGapUnits || e.At > g.cw-edgeGapUnits:
			return fmt.Errorf("die Wache %s bei %v: mindestens %d Units Abstand zur Chunk-Grenze (%d…%v)", e.Kind, e.At, edgeGapUnits, edgeGapUnits, g.cw-edgeGapUnits)
		case g.dist(c.Slot, e.At) <= outer:
			return fmt.Errorf("die Wache %s bei %v liegt %v Units vom Hub, innerhalb des äußersten Tors (%v)", e.Kind, e.At, g.dist(c.Slot, e.At), outer)
		}
		for _, o := range c.Enemies[:i] {
			if math.Abs(e.At-o.At) < guardRules.MinGapUnits {
				return fmt.Errorf("die Wache %s bei %v, weniger als %v Units von %s bei %v", e.Kind, e.At, guardRules.MinGapUnits, o.Kind, o.At)
			}
		}
	}
	return nil
}

// guardsOf sind die Wachen der geplanten Chunks mit ihrer X-Position; ohne Würfe, die Ströme bleiben unberührt.
func (g *gen) guardsOf(chunks []Chunk) []Guard {
	var out []Guard
	for _, c := range chunks {
		if pc, ok := g.planned[c.Index]; ok {
			for _, e := range pc.Enemies {
				out = append(out, Guard{Kind: e.Kind, X: c.StartUnits + e.At})
			}
		}
	}
	return out
}
