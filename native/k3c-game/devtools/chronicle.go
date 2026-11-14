package devtools

// Chronik des laufenden Spiels (B-889) als view.Listener: knappe Logs nur bei Wechseln und als Zusammenfassung, nie je
// Takt. 🚦 Tageszeit, Welle, Bau fertig, Burgfall, Stufenwechsel je Spieler; 🏹 Kampf je Nacht und Stufe; 🪙 Gold je
// Spieler und Tag; 🐢 Takt über Budget (data/native.json, höchstens alle slowLogEverySeconds). Am Ende schreibt
// report.go die Sitzungszusammenfassung. Die Zahlen kommen aus Welt und Ereignissen des Frames; nicht im Aufnahme-Modus.

import (
	"encoding/json"
	"log/slog"
	"time"

	"k3c/data"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// nativeValues sind die Werte aus data/native.json.
type nativeValues struct {
	StepBudgetMs        float64 `json:"stepBudgetMs"`
	SlowLogEverySeconds float64 `json:"slowLogEverySeconds"`
}

// loadNative liest data/native.json; fehlt ein Wert: 4 ms bzw. 5 s.
func loadNative() nativeValues {
	v := nativeValues{StepBudgetMs: 4, SlowLogEverySeconds: 5}
	if raw, err := data.Files.ReadFile("native.json"); err == nil {
		if err := json.Unmarshal(raw, &v); err != nil {
			slog.Warn("💥 data/native.json unlesbar, Standardwerte", "err", err)
		}
	}
	if v.StepBudgetMs <= 0 {
		v.StepBudgetMs = 4
	}
	return v
}

// nightStats ist der Kampf einer Nacht in einer Stufe.
type nightStats struct {
	day                int
	spawned, killed    int
	losses, troops     int // eigene Verluste (Monarch am Boden, Bau zerstört, Truppen weniger); Truppen bei Nachtbeginn
	dmgEnemy, dmgOwn   float64
}

// chronicle ist der Zustand der Chronik.
type chronicle struct {
	log  *slog.Logger
	now  func() time.Time
	dir  string
	vals nativeValues

	active                 bool
	start                  time.Time
	seed                   string
	startTick, tick        uint64
	firstDay, day          int
	phase, wave            map[int]any // je Stufe: letzte Tageszeit bzw. Welle
	stage                  [view.MaxPlayers]int
	gold, income, spending [view.MaxPlayers]int
	seen                   [view.MaxPlayers]bool
	nights                 map[int]*nightStats
	slowN                  int
	slowAt                 uint64
	written                bool
}

// newChronicle legt die Chronik an; Berichte gehen nach dir.
func newChronicle(log *slog.Logger, now func() time.Time, dir string) *chronicle {
	return &chronicle{log: log, now: now, dir: dir, vals: loadNative()}
}

// NewChronicle liefert die Chronik als Listener (Logs über slog.Default, Berichte nach dir). Ohne Grafik-Bibliothek
// (Tests, --assets) schreibt sie keinen Bericht.
func NewChronicle(lib *art.Library, dir string) view.Listener {
	if lib == nil {
		dir = ""
	}
	return newChronicle(slog.Default(), time.Now, dir)
}

// Listen schreibt die Chronik mit dem Frame f fort.
func (c *chronicle) Listen(f *view.Frame) {
	if f.Recording {
		return
	}
	if len(f.Stages) == 0 {
		if c.active {
			c.finish(outcomeName(f.Outcome))
			c.active = false
		}
		return
	}
	if !c.active {
		c.begin(f)
	}
	c.startTick = min(c.startTick, f.Tick) // F6 lädt einen älteren Stand
	c.tick = f.Tick
	for i := range f.Stages {
		c.stageChanges(f, &f.Stages[i])
	}
	c.seats(f)
	c.budget(f)
	if f.Outcome != view.OutcomeNone && !c.written {
		c.finish(outcomeName(f.Outcome))
	}
}

// begin startet eine Sitzung mit dem ersten Frame eines Spiels.
func (c *chronicle) begin(f *view.Frame) {
	w := f.Stages[0].World
	*c = chronicle{log: c.log, now: c.now, dir: c.dir, vals: c.vals, active: true, start: c.now(), startTick: f.Tick,
		phase: map[int]any{}, wave: map[int]any{}, nights: map[int]*nightStats{}}
	if w != nil {
		c.seed, c.firstDay, c.day = w.Seed, w.Cycle.Day, w.Cycle.Day
	}
	for p := range c.stage {
		c.stage[p] = -1
	}
}

// changed merkt v unter key in m und meldet einen Wechsel (nicht beim ersten Wert).
func changed(m map[int]any, key int, v any) bool {
	old, ok := m[key]
	m[key] = v
	return ok && old != v
}

// outcomeName ist die Ende-Ursache.
func outcomeName(o view.Outcome) string {
	switch o {
	case view.OutcomeWon:
		return "gewonnen"
	case view.OutcomeLost:
		return "verloren"
	}
	return "verlassen"
}
