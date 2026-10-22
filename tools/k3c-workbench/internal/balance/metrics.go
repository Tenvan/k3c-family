package balance

import (
	"slices"

	"k3c/engine/sim"
)

// Kennzahlen je Lauf (B-099 › Anforderungen), alle aus Zustand und Ereignissen der Simulation. Ticks zählen ab 1
// (Tick n = nach dem n-ten Schritt). Zeiger sind `null`, solange es den Zeitpunkt im Lauf nicht gab.

// Metrics sind die Kennzahlen eines Laufs; feste Feldreihenfolge, damit das JSON byte-gleich bleibt.
type Metrics struct {
	// GoldThresholdTick: erster Tick, an dem das Gold aller Monarchen zusammen mindestens die Schwelle erreicht.
	GoldThresholdTick *int `json:"goldThresholdTick"`
	FirstWallTick     *int `json:"firstWallTick"` // erstes Ereignis built mit kind wall
	// FirstBowTick: erstes Ereignis armed (ein Bauer holt einen Bogen aus der Werkstatt); Start-Bogenschützen zählen nicht.
	FirstBowTick   *int          `json:"firstBowTick"`
	CastleFallTick *int          `json:"castleFallTick"` // erstes castleFallen
	ReachedDepth   int           `json:"reachedDepth"`   // tiefste Stufe, auf der am Ende ein Monarch steht
	Days           []DayMetrics  `json:"days"`
	Waves          []WaveMetrics `json:"waves"`
}

// DayMetrics: Stand zu Dämmerungsbeginn (Ereignis dusk) und Gold-Fluss eines Tags. Der Fluss ist die Änderung des
// Golds aller Monarchen je Tick: Zunahme = Einnahmen, Abnahme = Ausgaben (auch gestohlenes und fallen gelassenes Gold).
type DayMetrics struct {
	Day         int        `json:"day"`
	DuskTick    *int       `json:"duskTick"`
	GoldAtDusk  *int       `json:"goldAtDusk"`
	StockAtDusk *sim.Stock `json:"stockAtDusk"`
	Income      int        `json:"income"`
	Expense     int        `json:"expense"`
}

// WaveMetrics: eine Welle vom Ereignis wave bis zum nächsten dawn (oder zur nächsten Welle). Verluste sind Truppen
// (ohne Landstreicher), die zu Wellenbeginn lebten und am Ende fehlen; überlebt = kein castleFallen in der Welle.
type WaveMetrics struct {
	Wave               int  `json:"wave"`
	StartTick          int  `json:"startTick"`
	EndTick            *int `json:"endTick"` // null: Lauf endete in der Welle
	Enemies            int  `json:"enemies"`
	TroopsAtStart      int  `json:"troopsAtStart"`
	TroopsLost         int  `json:"troopsLost"`
	BuildingsDestroyed int  `json:"buildingsDestroyed"`
	PlayersDown        int  `json:"playersDown"`
	Survived           bool `json:"survived"`
}

// collector sammelt die Kennzahlen Tick für Tick.
type collector struct {
	isl       *sim.Island
	threshold int
	gold      int // Gold aller Monarchen nach dem letzten Tick
	m         Metrics
	wave      *WaveMetrics
	waveIDs   []int // Truppen-IDs zu Beginn der laufenden Welle
}

func newCollector(isl *sim.Island, threshold int) *collector {
	c := &collector{isl: isl, threshold: threshold, m: Metrics{Days: []DayMetrics{}, Waves: []WaveMetrics{}}}
	c.gold = c.totalGold()
	return c
}

func (c *collector) totalGold() int {
	sum := 0
	for _, p := range c.isl.Players() {
		sum += p.Gold
	}
	return sum
}

// observe liest den Zustand nach Tick `tick`.
func (c *collector) observe(tick int) {
	day := c.day(c.isl.Stages[0].Cycle.Day)
	gold := c.totalGold()
	if d := gold - c.gold; d > 0 {
		day.Income += d
	} else {
		day.Expense -= d
	}
	c.gold = gold
	if c.threshold > 0 && gold >= c.threshold && c.m.GoldThresholdTick == nil {
		c.m.GoldThresholdTick = ptr(tick)
	}
	for _, w := range c.isl.Stages {
		for _, ev := range w.Events {
			c.event(w, ev, tick)
		}
	}
}

func (c *collector) event(w *sim.World, ev sim.Event, tick int) {
	switch ev["type"] {
	case "built":
		if ev["kind"] == "wall" {
			first(&c.m.FirstWallTick, tick)
		}
	case "armed":
		first(&c.m.FirstBowTick, tick)
	case "castleFallen":
		first(&c.m.CastleFallTick, tick)
		if c.wave != nil {
			c.wave.Survived = false
		}
	case "dusk":
		d := c.day(w.Cycle.Day)
		stock := *w.Stock
		d.DuskTick, d.GoldAtDusk, d.StockAtDusk = ptr(tick), ptr(c.gold), &stock
	case "wave":
		c.closeWave(w, tick)
		c.openWave(w, ev, tick)
	case "dawn":
		c.closeWave(w, tick)
	case "destroyed":
		c.countInWave(func(wm *WaveMetrics) { wm.BuildingsDestroyed++ })
	case "playerDown":
		c.countInWave(func(wm *WaveMetrics) { wm.PlayersDown++ })
	}
}

func (c *collector) openWave(w *sim.World, ev sim.Event, tick int) {
	n, _ := ev["wave"].(int)
	count, _ := ev["count"].(int)
	c.wave = &WaveMetrics{Wave: n, StartTick: tick, Enemies: count, Survived: true}
	c.waveIDs = fighterIDs(w)
	c.wave.TroopsAtStart = len(c.waveIDs)
}

func (c *collector) closeWave(w *sim.World, tick int) {
	if c.wave == nil {
		return
	}
	c.wave.EndTick = ptr(tick)
	c.finishWave(w)
}

// finishWave zählt die Verluste und legt die Welle ab.
func (c *collector) finishWave(w *sim.World) {
	alive := fighterIDs(w)
	for _, id := range c.waveIDs {
		if _, ok := slices.BinarySearch(alive, id); !ok {
			c.wave.TroopsLost++
		}
	}
	c.m.Waves = append(c.m.Waves, *c.wave)
	c.wave, c.waveIDs = nil, nil
}

func (c *collector) countInWave(f func(*WaveMetrics)) {
	if c.wave != nil {
		f(c.wave)
	}
}

// day liefert die Zeile des Tags; fehlende Tage davor werden angelegt.
func (c *collector) day(n int) *DayMetrics {
	for len(c.m.Days) < n {
		c.m.Days = append(c.m.Days, DayMetrics{Day: len(c.m.Days) + 1})
	}
	return &c.m.Days[n-1]
}

// result schließt eine offene Welle (ohne EndTick) und setzt die erreichte Tiefe.
func (c *collector) result() *Metrics {
	if c.wave != nil {
		c.finishWave(c.isl.Stages[0])
	}
	for _, w := range c.isl.Stages {
		if len(w.Players) > 0 {
			c.m.ReachedDepth = max(c.m.ReachedDepth, w.Biome.Depth)
		}
	}
	return &c.m
}

// fighterIDs: sortierte IDs aller Truppen außer Landstreichern.
func fighterIDs(w *sim.World) []int {
	ids := []int{}
	for _, t := range w.Troops {
		if t.Kind != "vagrant" {
			ids = append(ids, t.ID)
		}
	}
	slices.Sort(ids)
	return ids
}

func first(p **int, tick int) {
	if *p == nil {
		*p = ptr(tick)
	}
}

func ptr(v int) *int { return &v }
