package view

import (
	"math"
	"slices"
)

// Grenzen der Stresstest-Werte (Testing, B-896); menu blättert innerhalb, Normalized begrenzt gespeicherte Werte.
const (
	StressMaxDepth   = 3   // tiefste Stufe
	StressMaxSeed    = 999 // wie die Lobby
	StressMaxBuild   = 5   // Ausbaustufen der Dev-Aktion buildAll
	StressMaxTroops  = 200 // Obergrenze der Dev-Aktion spawn
	StressMaxFactor  = 50  // linear: + bis 50 je Welle; exponentiell: × bis 50
	StressMaxWaves   = 99
	StressMaxMinutes = 120
	StressEnemy      = "goblin" // Standard-Sorte der zusätzlichen Gegner
	StressTroop      = "archer" // Standard-Sorte der Truppen vorab
)

// DefaultStress sind die Werte ohne Vorgabe: ein Spieler, Oberfläche, 10 Gegner +5 je Welle, Abbruch nach 5 Wellen.
func DefaultStress() StressTest {
	return StressTest{Players: 1, Enemies: 10, Growth: GrowthLinear, Factor: 5, Timescale: 1, Waves: 5,
		Enemy: StressEnemy, Troop: StressTroop}
}

// Normalized begrenzt alle Werte auf ihren Bereich; unbekannter Anstieg wird linear, unbekannter Zeitraffer 1,
// leere Sorten die Standard-Sorten.
func (t StressTest) Normalized() StressTest {
	t.Players = clampInt(t.Players, 1, MaxPlayers)
	t.Depth = clampInt(t.Depth, 0, StressMaxDepth)
	t.Seed = clampInt(t.Seed, 0, StressMaxSeed)
	t.Enemies = clampInt(t.Enemies, 0, StressMaxEnemies)
	if t.Growth != GrowthExp {
		t.Growth = GrowthLinear
	}
	if math.IsNaN(t.Factor) {
		t.Factor = 0
	}
	t.Factor = math.Max(0, math.Min(StressMaxFactor, t.Factor))
	t.Build = clampInt(t.Build, 0, StressMaxBuild)
	t.Troops = clampInt(t.Troops, 0, StressMaxTroops)
	if !slices.Contains(Timescales[:], t.Timescale) {
		t.Timescale = 1
	}
	t.Waves = clampInt(t.Waves, 0, StressMaxWaves)
	t.Minutes = clampInt(t.Minutes, 0, StressMaxMinutes)
	if t.Enemy == "" {
		t.Enemy = StressEnemy
	}
	if t.Troop == "" {
		t.Troop = StressTroop
	}
	return t
}

func clampInt(v, lo, hi int) int { return max(lo, min(hi, v)) }
