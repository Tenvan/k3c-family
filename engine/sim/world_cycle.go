package sim

import (
	"math"
	"slices"

	"k3c/engine/level"
)

// Tag/Nacht-Zyklus (Q65, docs/rules/wirtschaft.md § 3): Tag → Dämmerung → Nacht → Morgengrauen → nächster Tag.
// Unter Tage läuft der globale Zyklus der Oberwelt weiter, Wellen kommen dort über den Aggressionspool.

// CycleInfo ist der Stand des Zyklus. Day 1 = erster Tag; die Nacht gehört zum Tag davor, das Morgengrauen schon zum
// neuen Tag (mit ihm fällt das Ereignis dawn, „Tag n – die Sonne geht auf“).
type CycleInfo struct {
	Phase       string  `json:"phase"` // day, dusk, night, dawn
	Day         int     `json:"day"`
	Progress    float64 `json:"progress"` // 0..1 innerhalb der Phase
	SecondsLeft float64 `json:"secondsLeft"`
}

// cyclePhase ist eine Phase des Zyklus mit Beginn und Länge in Zyklus-Sekunden.
type cyclePhase struct {
	name          string
	start, length float64
}

// cyclePhases liefert die vier Phasen in ihrer Reihenfolge und die Länge eines ganzen Zyklus.
func cyclePhases(c level.Cycle) ([4]cyclePhase, float64) {
	day, dusk := c.DayMinutes*60, c.TwilightMinutes*60
	night, dawn := c.NightMinutes*60, c.DawnMinutes*60
	return [4]cyclePhase{
		{"day", 0, day}, {"dusk", day, dusk}, {"night", day + dusk, night}, {"dawn", day + dusk + night, dawn},
	}, day + dusk + night + dawn
}

func cycleAt(c level.Cycle, seconds float64) CycleInfo {
	phases, length := cyclePhases(c)
	n := math.Floor(seconds / length)
	t := seconds - n*length
	for _, p := range phases[:3] {
		if t < p.start+p.length {
			return CycleInfo{p.name, int(n) + 1, (t - p.start) / p.length, p.start + p.length - t}
		}
	}
	dawn := phases[3]
	return CycleInfo{dawn.name, int(n) + 2, (t - dawn.start) / dawn.length, length - t}
}

func stepCycle(w *World, dt float64) {
	before := w.Cycle
	w.Cycle = cycleAt(globalDayNight, float64(w.Time*w.CycleSpeed))
	changed := before.Phase != w.Cycle.Phase
	dayNight := w.Biome.Cycle.Type == "dayNight"
	if changed && w.Cycle.Phase == "dusk" {
		w.Events = append(w.Events, Event{"type": "dusk"})
	}
	if changed && w.Cycle.Phase == "night" {
		w.Events = append(w.Events, Event{"type": "night", "day": w.Cycle.Day})
		nightEventsStart(w) // Vollmond, Blutmond (events_moon.go)
		if dayNight {
			startWave(w)
		}
	}
	if changed && w.Cycle.Phase == "dawn" { // der Übergang Morgengrauen → Tag löst nichts aus (Q65)
		w.Events = append(w.Events, Event{"type": "dawn", "day": w.Cycle.Day})
		if dayNight {
			sendEnemiesHome(w)
		}
		payDawnIncome(w)
		tavernDawn(w)
		merchantDawn(w)
		nightEventsEnd(w)
	}
	if w.Aggression != nil && w.Biome.Cycle.Type == "aggressionPool" {
		addPool(w, float64(w.Biome.Cycle.PercentPerMinute/60*dt*w.CycleSpeed))
		if *w.Aggression >= 100 {
			*w.Aggression = 0
			startWave(w)
		}
	}
}

// minWaveGapSeconds ist der Mindestabstand zweier Wellen unter Tage (4 min Spielzeit, zielkorridore.md § 1, B-646).
const minWaveGapSeconds = 240

// poolFrozen: Der Aggressionspool wächst nicht, solange Gegner einer Welle leben oder noch kommen und innerhalb des
// Mindestabstands nach der letzten Welle. Ohne das löst jede getötete Welle über +5 % je Kill sofort die nächste aus
// (Wellen-Lawine, B-646).
func poolFrozen(w *World) bool {
	if w.Wave > 0 && w.Time-w.lastWaveAt < minWaveGapSeconds {
		return true
	}
	if len(w.SpawnQueue) > 0 {
		return true
	}
	return slices.ContainsFunc(w.Enemies, func(e *Enemy) bool { return e.HP > 0 && !e.Guard })
}

// addPool erhöht den Aggressionspool um amount (höchstens auf 100), sofern es ihn gibt und er nicht eingefroren ist.
func addPool(w *World, amount float64) {
	if w.Aggression == nil || w.Biome.Cycle.Type != "aggressionPool" || poolFrozen(w) {
		return
	}
	*w.Aggression = math.Min(100, *w.Aggression+amount)
}
