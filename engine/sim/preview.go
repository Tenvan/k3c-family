package sim

import (
	"fmt"
	"math"
)

// Hilfen für die Vorschau des Welt-Betrachters (WZ7.8, B-460). Beide arbeiten nur auf der übergebenen Welt, lesen die
// geladenen Daten und setzen keinen globalen Zustand.

// PreviewCycle setzt die Tageszeit der Welt direkt: Sim-Phase (day, dusk, night) und Fortschritt in [0, 1) innerhalb
// der Phase. Der Tag bleibt, w.Time wird so abgeleitet, dass cycleAt dieselbe Phase liefert; es entstehen keine
// Ereignisse. Eine andere Phase (auch dawn, nur ein Ereignis beim Tagesbeginn) und ein Fortschritt außerhalb von
// [0, 1) (1 wäre der Beginn der nächsten Phase) ergeben einen Fehler, ebenso CycleSpeed 0 (der Zyklus steht); die Welt
// bleibt dann unverändert. w.Time darf auch rückwärts springen: gedacht für eine frisch erzeugte Vorschau-Welt, nicht
// für eine laufende Simulation (Fristen wie CraftDone liegen sonst in der Zukunft).
func PreviewCycle(w *World, phase string, progress float64) error {
	day := float64(globalDayNight.DayMinutes * 60)
	dusk := float64(globalDayNight.TwilightMinutes * 60)
	night := float64(globalDayNight.NightMinutes * 60)
	var start, length float64
	switch phase {
	case "day":
		start, length = 0, day
	case "dusk":
		start, length = day, dusk
	case "night":
		start, length = day+dusk, night
	default:
		return fmt.Errorf("vorschau: unbekannte Phase %q (day, dusk, night)", phase)
	}
	if !(progress >= 0 && progress < 1) {
		return fmt.Errorf("vorschau: Fortschritt %v außerhalb von [0, 1)", progress)
	}
	speed := w.CycleSpeed
	if speed <= 0 {
		return fmt.Errorf("vorschau: CycleSpeed %v, der Zyklus steht", speed)
	}
	cycleLen := day + dusk + night
	today := math.Floor(float64(w.Time*speed) / cycleLen)
	seconds := today*cycleLen + start + progress*length
	w.Time = seconds / speed
	for range 8 { // (x/speed)*speed ist nicht immer genau x: bei Phasenbeginn nicht in der vorigen Phase landen
		if cycleAt(globalDayNight, float64(w.Time*speed)).Phase == phase {
			break
		}
		w.Time = math.Nextafter(w.Time, math.Inf(1))
	}
	w.Cycle = CycleInfo{Phase: phase, Day: int(today) + 1, Progress: progress, SecondsLeft: (1 - progress) * length}
	return nil
}

// PreviewHubLevel baut die Welt auf Hub-Stufe level aus (1 bis Anzahl hub.json › levels). Plätze und Hub-Platz folgen
// den Regeln der Simulation wie bei DevBuildAll (devBuildSite: Hub-Stufe des Platzes, Mauerlinie, Tore bleiben).
// Die Material-Stufe von Mauer und Turm läuft danach je Art reihum in der Reihenfolge der Plätze von 1 bis
// min(level, Stufenzahl der Art), damit jede erlaubte Stufe im Bild steht und nextLevel (Stufe m erst ab Hub-Stufe m)
// nie verletzt wird. Bei einem Fehler bleibt die Welt unverändert. Sie baut nur auf, nichts ab: gedacht für eine
// frisch erzeugte Welt (CreateWorld), nicht für eine, die schon höher ausgebaut ist.
func PreviewHubLevel(w *World, level int) error {
	if n := len(hubOf(w).Levels); level < 1 || level > n {
		return fmt.Errorf("vorschau: Hub-Stufe %d außerhalb von 1 bis %d", level, n)
	}
	w.HubLevel = level
	if h := w.hubSite; h != nil {
		h.Upgrade, h.UpgradePaid, h.BuildProgress, h.WorkerID = "", 0, 1, nil
	}
	for _, s := range w.Sites {
		devBuildSite(w, s, level)
	}
	seen := map[string]int{}
	for _, s := range w.Sites {
		levels := buildings[s.Kind].Levels
		if s.State != "built" || len(levels) == 0 {
			continue
		}
		s.Level = seen[s.Kind]%min(level, len(levels)) + 1
		seen[s.Kind]++
		s.HP, s.MaxHP = levels[s.Level-1].HP, levels[s.Level-1].HP
	}
	return nil
}
