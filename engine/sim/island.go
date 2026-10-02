package sim

import "fmt"

// Insel (Entscheidung 003, B-100): Ein Level mit mehreren Stufen. Alle Stufen laufen im selben Takt weiter, auch ohne
// Spieler; jeder Spieler ist an genau eine Stufe gebunden. Das Baumaterial gehört der Insel (ein Vorrat für alle Stufen),
// Gold gehört dem Spieler. Die Insel steht neben der Campaign (campaign.go), die der Raum bis zur Umstellung (B-133)
// weiter benutzt.

// Island ist eine Insel mit ihren Stufen.
type Island struct {
	ID         string // gleiche ID = gleiches Spiel (Spielstand)
	Seed       string
	CycleSpeed float64
	// Stages: Stufe i ist die Welt der i-ten Tiefe aus CreateIsland.
	Stages []*World
	// Stock ist der Vorrat der Insel; alle Stufen zeigen auf denselben Wert.
	Stock      *Stock
	nextPlayer int
	// travel: Reisefortschritt je Spielerindex (island_travel.go); nur über die Stufen- und Spielerlisten iterieren.
	travel map[int]*islandTravel
}

// CreateIsland baut die Stufen für die angegebenen Tiefen (Reihenfolge = Stufenindex). CycleSpeed 0 bedeutet 1.
func CreateIsland(seed string, depths []int, cycleSpeed float64) (*Island, error) {
	return createIsland(seed, depths, cycleSpeed, 0)
}

// createIsland wie CreateIsland, mit Startzeit (Laden eines Spielstands).
func createIsland(seed string, depths []int, cycleSpeed, startTime float64) (*Island, error) {
	if len(depths) == 0 {
		return nil, fmt.Errorf("insel: keine Stufen")
	}
	isl := &Island{ID: seed, Seed: seed, CycleSpeed: cycleSpeed, Stock: &Stock{}, travel: map[int]*islandTravel{}}
	for _, d := range depths {
		if !hasDepth(d) {
			return nil, fmt.Errorf("insel: unbekannte Tiefe %d", d)
		}
		w, err := CreateWorld(biomeForDepth(d), seed, Options{CycleSpeed: cycleSpeed, Time: startTime})
		if err != nil {
			return nil, fmt.Errorf("insel: Stufe mit Tiefe %d: %w", d, err)
		}
		w.Stock = isl.Stock
		w.noTravel = true
		isl.Stages = append(isl.Stages, w)
	}
	return isl, nil
}

// AddIslandPlayer stellt einen neuen Monarchen an die Burg der Stufe `stage`. Der Index zählt inselweit hoch und
// bleibt auch nach einem Stufenwechsel derselbe (Eingaben werden über ihn zugeordnet).
func AddIslandPlayer(isl *Island, stage int) *Player {
	p := addPlayerAt(isl.Stages[stage], isl.nextPlayer)
	isl.nextPlayer++
	return p
}

// StepIsland rechnet einen Tick in allen Stufen, von Stufe 0 aufwärts. commands wird nach dem Spielerindex der Insel
// gelesen (`commands[p.Index]`); jede Stufe bekommt dasselbe Feld und nimmt sich ihre Spieler heraus.
func StepIsland(isl *Island, commands []PlayerCommand, dt float64) {
	for _, w := range isl.Stages {
		Step(w, commands, dt)
	}
	stepIslandTravel(isl, dt)
}
