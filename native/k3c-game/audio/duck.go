package audio

// Ducking wie src/audio/ducking.ts (SO4.3, Beschluss Q16): Eine Warnung senkt die Musik kurz ab und stellt sie danach
// wieder her; überlappende Warnungen verlängern die Absenkung. Zeit in Sekunden aus dem Takt (Listen-Zähler / tps).
// Die Form kommt aus data/sounds.json › mischung (B-935); defaultDuck gilt ohne Eintrag.

// duckShape ist die Form einer Absenkung: Pegel der Musik während der Warnung, Absenken, Halten nach der letzten
// Warnung und Zurückkehren auf vollen Pegel in Sekunden.
type duckShape struct {
	level, attack, hold, release float64
}

// defaultDuck ist die Form ohne Mischpegel in sounds.json (Stand vor B-935).
var defaultDuck = duckShape{level: 0.35, attack: 0.15, hold: 1.2, release: 0.8}

// duckWindow ist eine Absenkung: Rampe ab start hinab, gesenkt bis end, danach Rampe hinauf. Null-Wert = keine.
type duckWindow struct {
	start, end float64
	shape      duckShape
	set        bool
}

// duckGain ist der Pegelfaktor der Musik zum Zeitpunkt t (ducking.ts › duckGain).
func duckGain(t float64, w duckWindow) float64 {
	s := w.shape
	switch {
	case !w.set || t <= w.start:
		return 1
	case t < w.start+s.attack:
		return 1 - (1-s.level)*(t-w.start)/s.attack
	case t <= w.end:
		return s.level
	case t < w.end+s.release:
		return s.level + (1-s.level)*(t-w.end)/s.release
	default:
		return 1
	}
}

// extendDuck meldet eine neue Warnung zum Zeitpunkt t mit Form s (ducking.ts › extendDuck): eine laufende Absenkung
// wird verlängert statt gestapelt; in der Rampe hinauf beginnt die neue Rampe hinab beim aktuellen Pegel, ohne Sprung.
func extendDuck(w duckWindow, t float64, s duckShape) duckWindow {
	g := duckGain(t, w)
	end := t + s.hold
	if w.set && t <= w.end {
		end = max(end, w.end)
	}
	if w.set && t > w.start && t <= w.end {
		return duckWindow{start: w.start, end: end, shape: w.shape, set: true}
	}
	return duckWindow{start: t - s.attack*(1-g)/(1-s.level), end: end, shape: s, set: true}
}
