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
	FirstBowTick   *int `json:"firstBowTick"`
	CastleFallTick *int `json:"castleFallTick"` // erstes castleFallen
	// FirstWorkshopTick: erstes Ereignis built mit kind workshop (WZ28.4).
	FirstWorkshopTick *int          `json:"firstWorkshopTick"`
	ReachedDepth      int           `json:"reachedDepth"` // tiefste Stufe, auf der am Ende ein Monarch steht
	Days              []DayMetrics  `json:"days"`
	Waves             []WaveMetrics `json:"waves"`
}

// DayMetrics: Stand zu Dämmerungsbeginn (Ereignis dusk) und Gold-Fluss eines Tags. Der Fluss ist die Änderung des
// Golds aller Monarchen je Tick: Zunahme = Einnahmen, Abnahme = Ausgaben (auch gestohlenes und fallen gelassenes Gold).
// Ein Tag reicht vom Morgen bis zum nächsten Morgen, Nacht n gehört also zu Tag n (WZ28.4):
//   - NightIncome/NightExpense: der Teil von Income/Expense in der Phase night (Tag-Anteil = Differenz)
//   - StockIn/StockOut: Zu- und Abgang des Insel-Vorrats je Rohstoff (Änderung je Tick)
//   - CastleFallen: In diesem Tag (samt Nacht) fiel die Burg
//   - PeasantTicks/PeasantIdleTicks: Bauern-Ticks und davon ohne Auftrag und ohne Arbeitsplatz (Wartezeit)
type DayMetrics struct {
	Day              int        `json:"day"`
	DuskTick         *int       `json:"duskTick"`
	GoldAtDusk       *int       `json:"goldAtDusk"`
	StockAtDusk      *sim.Stock `json:"stockAtDusk"`
	Income           int        `json:"income"`
	Expense          int        `json:"expense"`
	NightIncome      int        `json:"nightIncome"`
	NightExpense     int        `json:"nightExpense"`
	StockIn          sim.Stock  `json:"stockIn"`
	StockOut         sim.Stock  `json:"stockOut"`
	CastleFallen     bool       `json:"castleFallen"`
	PeasantTicks     int        `json:"peasantTicks"`
	PeasantIdleTicks int        `json:"peasantIdleTicks"`
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
	gold      int       // Gold aller Monarchen nach dem letzten Tick
	stock     sim.Stock // Insel-Vorrat nach dem letzten Tick
	m         Metrics
	wave      *WaveMetrics
	waveIDs   []int // Truppen-IDs zu Beginn der laufenden Welle
}

func newCollector(isl *sim.Island, threshold int) *collector {
	c := &collector{isl: isl, threshold: threshold, m: Metrics{Days: []DayMetrics{}, Waves: []WaveMetrics{}}}
	c.gold, c.stock = c.totalGold(), *isl.Stock
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
	night := c.isl.Stages[0].Cycle.Phase == "night"
	gold := c.totalGold()
	if d := gold - c.gold; d > 0 {
		day.Income += d
		day.NightIncome += d * b2i(night)
	} else {
		day.Expense -= d
		day.NightExpense -= d * b2i(night)
	}
	c.gold = gold
	c.observeStock(day)
	c.observePeasants(day)
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
		switch ev["kind"] {
		case "wall":
			first(&c.m.FirstWallTick, tick)
		case "workshop":
			first(&c.m.FirstWorkshopTick, tick)
		}
	case "armed":
		first(&c.m.FirstBowTick, tick)
	case "castleFallen":
		first(&c.m.CastleFallTick, tick)
		c.day(w.Cycle.Day).CastleFallen = true
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

// observeStock verbucht die Änderung des Insel-Vorrats je Rohstoff als Zu- oder Abgang des Tags.
func (c *collector) observeStock(day *DayMetrics) {
	now := *c.isl.Stock
	flow := func(in, out *int, before, after int) {
		if d := after - before; d > 0 {
			*in += d
		} else {
			*out -= d
		}
	}
	flow(&day.StockIn.Wood, &day.StockOut.Wood, c.stock.Wood, now.Wood)
	flow(&day.StockIn.Stone, &day.StockOut.Stone, c.stock.Stone, now.Stone)
	flow(&day.StockIn.Copper, &day.StockOut.Copper, c.stock.Copper, now.Copper)
	flow(&day.StockIn.Iron, &day.StockOut.Iron, c.stock.Iron, now.Iron)
	flow(&day.StockIn.Crystal, &day.StockOut.Crystal, c.stock.Crystal, now.Crystal)
	c.stock = now
}

// observePeasants zählt Bauern-Ticks; wartend ist ein Bauer ohne Auftrag und ohne Arbeitsplatz (Handwerker an der
// Werkstatt arbeiten ohne Auftrag).
func (c *collector) observePeasants(day *DayMetrics) {
	for _, w := range c.isl.Stages {
		for _, t := range w.Troops {
			if t.Kind != "peasant" {
				continue
			}
			day.PeasantTicks++
			day.PeasantIdleTicks += b2i(t.Job == nil && t.WorkSite == 0)
		}
	}
}

func b2i(b bool) int {
	if b {
		return 1
	}
	return 0
}

func first(p **int, tick int) {
	if *p == nil {
		*p = ptr(tick)
	}
}

func ptr(v int) *int { return &v }
