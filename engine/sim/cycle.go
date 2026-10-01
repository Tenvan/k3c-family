package sim

import (
	"math"

	"k3c/engine/level"
)

// Tag/Nacht-Zyklus (Port von src/world/sim/cycle.ts): Tag → Dämmerung → Nacht → nächster Tag.
// Unter Tage läuft der globale Zyklus der Oberwelt weiter, Wellen kommen dort über den Aggressionspool.

// CycleInfo ist der Stand des Zyklus. Day 1 = erster Tag, die Nacht gehört zum Tag davor.
type CycleInfo struct {
	Phase       string  `json:"phase"` // day, dusk, night
	Day         int     `json:"day"`
	Progress    float64 `json:"progress"` // 0..1 innerhalb der Phase
	SecondsLeft float64 `json:"secondsLeft"`
}

func cycleAt(c level.Cycle, seconds float64) CycleInfo {
	day := c.DayMinutes * 60
	dusk := c.TwilightMinutes * 60
	night := c.NightMinutes * 60
	length := day + dusk + night
	n := math.Floor(seconds / length)
	t := seconds - float64(n*length)
	d := int(n) + 1
	if t < day {
		return CycleInfo{"day", d, t / day, day - t}
	}
	if t < day+dusk {
		return CycleInfo{"dusk", d, (t - day) / dusk, day + dusk - t}
	}
	return CycleInfo{"night", d, (t - day - dusk) / night, length - t}
}

func stepCycle(w *World, dt float64) {
	before := w.Cycle
	w.Cycle = cycleAt(globalDayNight, float64(w.Time*w.CycleSpeed))
	changed := before.Phase != w.Cycle.Phase
	if changed && w.Cycle.Phase == "dusk" {
		w.Events = append(w.Events, Event{"type": "dusk"})
	}
	if changed && w.Cycle.Phase == "night" {
		w.Events = append(w.Events, Event{"type": "night", "day": w.Cycle.Day})
		// SP06: bei dayNight startWave
	}
	if changed && w.Cycle.Phase == "day" {
		w.Events = append(w.Events, Event{"type": "dawn", "day": w.Cycle.Day})
		// SP06: bei dayNight sendEnemiesHome; SP05.3: payDawnIncome
	}
	if w.Aggression != nil && w.Biome.Cycle.Type == "aggressionPool" {
		a := math.Min(100, *w.Aggression+float64(w.Biome.Cycle.PercentPerMinute/60*dt*w.CycleSpeed))
		if a >= 100 {
			a = 0
			// SP06: startWave
		}
		*w.Aggression = a
	}
}
