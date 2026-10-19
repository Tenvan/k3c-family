package sim

// Zustands-Spiegel (K4.1a, B-383): reine Anzeige-Felder für das Protokoll, ohne rng und nicht im Spielstand.
// Sie werden nach jedem Tick neu gesetzt; fehlt der Zustand, fehlt das Feld im JSON.

// WarnZone: nächster Flächenschlag eines Bosses (Ort und Radius in Units, In = Sekunden bis dahin).
type WarnZone struct {
	X  float64 `json:"x"`
	R  float64 `json:"r"`
	In float64 `json:"in"`
}

// ActiveEvent: ein laufendes Nacht-Event (ID wie in data/events.json) mit Restzeit der Nacht in Sekunden.
type ActiveEvent struct {
	ID          string  `json:"id"`
	SecondsLeft float64 `json:"secondsLeft"`
}

// SwitchState: Wechselpunkt der Insel offen, Fortschritt 0..1, Wechsel bereit (island_switch.go).
type SwitchState struct {
	Open     bool    `json:"open"`
	Progress float64 `json:"progress"`
	Ready    bool    `json:"ready"`
}

// mirrorState setzt Boss-Phase, Warnkreis und Nacht-Events der Stufe (Ende von StepIsland, island.go).
func mirrorState(w *World) {
	for _, e := range w.Enemies {
		if e.Boss {
			mirrorBoss(w, e)
		}
	}
	w.NightEvents = nil
	for i := range nightEvents {
		if ev := &nightEvents[i]; eventActive(w, ev) {
			w.NightEvents = append(w.NightEvents, ActiveEvent{ev.ID, w.Cycle.SecondsLeft})
		}
	}
}

// mirrorBoss: Phase (ab 1, nur der laufende Endboss) und Warnkreis (nur bei der Fähigkeit `aoe`).
func mirrorBoss(w *World, e *Enemy) {
	e.Phase, e.Warn = 0, nil
	if isl := w.island; isl != nil && isl.endboss != nil && isl.endboss.boss == e {
		e.Phase = isl.endboss.phase + 1
	}
	if b := bossByID(e.Kind); b != nil {
		if a := bossAbilityOf(w, b); a.Type == "aoe" {
			e.Warn = &WarnZone{X: unitX(e.X), R: a.Radius, In: e.AoeIn}
		}
	}
}

// mirrorSwitch setzt den Wechselzustand der Insel in allen Stufen (nach stepIslandSwitch).
func mirrorSwitch(isl *Island) {
	var s *SwitchState
	if isl.GateOpen {
		s = &SwitchState{Open: true, Progress: min(isl.switchProgress, 1), Ready: isl.SwitchReady}
	}
	for _, w := range isl.Stages {
		w.IslandSwitch = s
	}
}
