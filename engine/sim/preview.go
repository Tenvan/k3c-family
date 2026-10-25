package sim

import (
	"fmt"
	"math"
	"slices"
)

// Hilfen für die Vorschau des Welt-Betrachters (WZ7.8, B-460). Beide arbeiten nur auf der übergebenen Welt, lesen die
// geladenen Daten und setzen keinen globalen Zustand.

// PreviewCycle setzt die Tageszeit der Welt direkt: Sim-Phase (day, dusk, night, dawn) und Fortschritt in [0, 1)
// innerhalb der Phase. Der Zyklus bleibt, w.Time wird so abgeleitet, dass cycleAt dieselbe Phase liefert; es entstehen
// keine Ereignisse. Eine andere Phase und ein Fortschritt außerhalb von [0, 1) (1 wäre der Beginn der nächsten Phase)
// ergeben einen Fehler, ebenso CycleSpeed 0 (der Zyklus steht); die Welt bleibt dann unverändert. w.Time darf auch
// rückwärts springen: gedacht für eine frisch erzeugte Vorschau-Welt, nicht für eine laufende Simulation (Fristen wie
// CraftDone liegen sonst in der Zukunft).
func PreviewCycle(w *World, phase string, progress float64) error {
	phases, cycleLen := cyclePhases(globalDayNight)
	i := slices.IndexFunc(phases[:], func(p cyclePhase) bool { return p.name == phase })
	if i < 0 {
		return fmt.Errorf("vorschau: unbekannte Phase %q (day, dusk, night, dawn)", phase)
	}
	start, length := phases[i].start, phases[i].length
	if !(progress >= 0 && progress < 1) {
		return fmt.Errorf("vorschau: Fortschritt %v außerhalb von [0, 1)", progress)
	}
	speed := w.CycleSpeed
	if speed <= 0 {
		return fmt.Errorf("vorschau: CycleSpeed %v, der Zyklus steht", speed)
	}
	today := math.Floor(float64(w.Time*speed) / cycleLen)
	seconds := today*cycleLen + start + progress*length
	w.Time = seconds / speed
	for range 8 { // (x/speed)*speed ist nicht immer genau x: bei Phasenbeginn nicht in der vorigen Phase landen
		if cycleAt(globalDayNight, float64(w.Time*speed)).Phase == phase {
			break
		}
		w.Time = math.Nextafter(w.Time, math.Inf(1))
	}
	day := int(today) + 1
	if phase == "dawn" { // das Morgengrauen gehört schon zum nächsten Tag (cycleAt)
		day++
	}
	w.Cycle = CycleInfo{Phase: phase, Day: day, Progress: progress, SecondsLeft: (1 - progress) * length}
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
	w.HubLevel, w.HubStep = level, 1
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

// SiteLook ist der Zustand eines Platzes in der Vorschau (Asset-Studio): Baustufe, Zustand der Trefferpunkte oder noch
// nicht gebaut. Nullwerte lassen den Platz, wie PreviewHubLevel ihn hinterlassen hat.
type SiteLook struct {
	Level     int  `json:"level"`     // Baustufe 1 bis Stufenzahl der Art; 0 = unverändert
	HPPercent int  `json:"hpPercent"` // Trefferpunkte in Prozent der Höchstwerte (1 bis 100); 0 = unverändert
	Unbuilt   bool `json:"unbuilt"`   // leerer Bauplatz (nicht bezahlt), Level und HPPercent entfallen
}

// PreviewSetSite setzt einen Platz der Vorschau-Welt auf look. Eine Baustufe oder Trefferpunkte an einem nicht
// gebauten Platz werden ignoriert; eine Stufe außerhalb der Stufen der Art ist ein Fehler.
func PreviewSetSite(s *Site, look SiteLook) error {
	if look.Unbuilt {
		*s = Site{ID: s.ID, Kind: s.Kind, X: s.X, State: "unpaid", MaxHP: buildings[s.Kind].HP, Ruin: s.Ruin} // eine Ruine behält ihre Kennung wie bei destroySite
		return nil
	}
	if s.State != "built" {
		return nil
	}
	levels := buildings[s.Kind].Levels
	if look.Level != 0 {
		if look.Level < 1 || look.Level > len(levels) {
			return fmt.Errorf("vorschau: Baustufe %d außerhalb von 1 bis %d (%s)", look.Level, len(levels), s.Kind)
		}
		s.Level = look.Level
		s.HP, s.MaxHP = levels[look.Level-1].HP, levels[look.Level-1].HP
	}
	if look.HPPercent != 0 {
		if look.HPPercent < 1 || look.HPPercent > 100 {
			return fmt.Errorf("vorschau: Trefferpunkte %d %% außerhalb von 1 bis 100 (%s)", look.HPPercent, s.Kind)
		}
		s.HP = s.MaxHP * float64(look.HPPercent) / 100
	}
	return nil
}
