package balance

import (
	"slices"

	"k3c/engine/sim"
)

// Kennzahlen eines Insel-Laufs mit World (BS11, B-642): je Tiefe der Insel Ankunft und Burgfall, besiegte Bosse mit
// Tag, Endboss und Sieg. Ohne World (eine Stufe) bleiben alle Felder leer und stehen nicht im JSON.

// StageMetrics: eine Stufe der Insel (Reihenfolge = Stufenindex). Arrival* ist der erste Tick, an dem ein Monarch auf
// der Stufe stand (Stufe 0: Tick 0); null: nie erreicht.
type StageMetrics struct {
	Depth          int  `json:"depth"`
	ArrivalDay     *int `json:"arrivalDay"`
	ArrivalTick    *int `json:"arrivalTick"`
	CastleFallTick *int `json:"castleFallTick"`
}

// BossMetrics: ein besiegter Boss (Ereignis bossDefeated) mit Tag und Tick.
type BossMetrics struct {
	Boss string `json:"boss"`
	Day  int    `json:"day"`
	Tick int    `json:"tick"`
}

// IslandMetrics fasst die Felder zusammen, die nur ein Insel-Lauf füllt.
type IslandMetrics struct {
	Stages      []StageMetrics `json:"stages,omitempty"`
	Bosses      []BossMetrics  `json:"bosses,omitempty"`
	EndbossDay  *int           `json:"endbossDay,omitempty"`  // Tag des Siegs über den Endboss der Insel
	EndbossTick *int           `json:"endbossTick,omitempty"` // Tick dazu
	VictoryTick *int           `json:"victoryTick,omitempty"` // Ereignis victory (nur auf der letzten Insel der Welt)
}

func (c *collector) multi() bool { return len(c.isl.Stages) > 1 }

// initStages legt die Zeilen der Stufen an (nur Insel-Lauf).
func (c *collector) initStages() {
	if !c.multi() {
		return
	}
	for _, w := range c.isl.Stages {
		c.m.Stages = append(c.m.Stages, StageMetrics{Depth: w.Biome.Depth})
	}
	c.observeIsland(0)
}

// observeIsland merkt Ankünfte und den Endboss-Sieg nach Tick `tick`.
func (c *collector) observeIsland(tick int) {
	if !c.multi() {
		return
	}
	for i, w := range c.isl.Stages {
		if s := &c.m.Stages[i]; s.ArrivalTick == nil && len(w.Players) > 0 {
			s.ArrivalTick, s.ArrivalDay = ptr(tick), ptr(w.Cycle.Day)
		}
	}
	if c.isl.EndbossDefeated && c.m.EndbossTick == nil {
		c.m.EndbossTick, c.m.EndbossDay = ptr(tick), ptr(c.isl.Stages[0].Cycle.Day)
	}
}

// islandEvent verbucht Ereignisse der Stufe w, die nur der Insel-Lauf misst.
func (c *collector) islandEvent(w *sim.World, ev sim.Event, tick int) {
	if !c.multi() {
		return
	}
	switch ev["type"] {
	case "bossDefeated":
		id, _ := ev["boss"].(string)
		c.m.Bosses = append(c.m.Bosses, BossMetrics{Boss: id, Day: w.Cycle.Day, Tick: tick})
	case "castleFallen":
		if i := slices.Index(c.isl.Stages, w); i >= 0 {
			first(&c.m.Stages[i].CastleFallTick, tick)
		}
	case "victory":
		first(&c.m.VictoryTick, tick)
	}
}
