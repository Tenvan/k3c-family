package audio

// Ducking wie src/audio/ducking.ts (SO4.3, Beschluss Q16): Eine Warnung senkt die Musik kurz ab und stellt sie danach
// wieder her; überlappende Warnungen verlängern die Absenkung. Zeit in Sekunden aus dem Takt (Listen-Zähler / tps).
const (
	duckLevel   = 0.35 // Pegel der Musik während der Warnung
	duckAttack  = 0.15 // Absenken in Sekunden
	duckHold    = 1.2  // gesenkt halten nach der letzten Warnung
	duckRelease = 0.8  // zurück auf vollen Pegel
)

// duckWindow ist eine Absenkung: Rampe ab start hinab, gesenkt bis end, danach Rampe hinauf. Null-Wert = keine.
type duckWindow struct {
	start, end float64
	set        bool
}

// duckGain ist der Pegelfaktor der Musik zum Zeitpunkt t (ducking.ts › duckGain).
func duckGain(t float64, w duckWindow) float64 {
	switch {
	case !w.set || t <= w.start:
		return 1
	case t < w.start+duckAttack:
		return 1 - (1-duckLevel)*(t-w.start)/duckAttack
	case t <= w.end:
		return duckLevel
	case t < w.end+duckRelease:
		return duckLevel + (1-duckLevel)*(t-w.end)/duckRelease
	default:
		return 1
	}
}

// extendDuck meldet eine neue Warnung zum Zeitpunkt t (ducking.ts › extendDuck): eine laufende Absenkung wird
// verlängert statt gestapelt; in der Rampe hinauf beginnt die neue Rampe hinab beim aktuellen Pegel, ohne Sprung.
func extendDuck(w duckWindow, t float64) duckWindow {
	g := duckGain(t, w)
	end := t + duckHold
	if w.set && t <= w.end {
		end = max(end, w.end)
	}
	if w.set && t > w.start && t <= w.end {
		return duckWindow{start: w.start, end: end, set: true}
	}
	return duckWindow{start: t - duckAttack*(1-g)/(1-duckLevel), end: end, set: true}
}
