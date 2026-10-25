package room

import (
	"slices"

	"k3c/engine/sim"
)

// DevAction ist eine Dev-Aktion eines Geräts (Nachricht dev, B-178): gold und material brauchen slot und amount,
// material zusätzlich resource, timescale braucht factor, pause braucht paused (B-231); wave und castleFall (Stufenverlust
// in der Stufe des Monarchen, WZ14.5) brauchen slot, phase braucht
// phase (day, dusk, night) (B-232); grade braucht grade (Name aus data/difficulty.json, gilt ab der nächsten Welle, B-080).
// maxStock (+100 je Aufruf), fillStock und nextPhase brauchen nichts; cheapBuild und timescaleAuto brauchen on, buildAll
// braucht level (1 bis 5). Kampf (B-603, dev_combat.go): boss braucht slot und boss (Id aus data/bosses.json), event
// braucht event (fullMoon, bloodMoon, merchantRaid), bossPhase und night brauchen nichts.
type DevAction struct {
	Action   string
	Slot     *int
	Amount   int
	Resource string
	Factor   int
	Paused   *bool
	Phase    string
	Grade    string
	On       *bool
	Level    int
	Boss     string
	Event    string
}

// devMaxAmount ist die Obergrenze von amount (gold, material).
const devMaxAmount = 1000

// MaxTimescale ist der größte Faktor des Zeitraffers; er schützt das Tick-Budget (B-178).
const MaxTimescale = 8

// devTimescales sind die erlaubten Faktoren von timescale.
var devTimescales = []int{1, 2, 4, MaxTimescale}

// Dev führt eine Dev-Aktion eines Geräts aus; slot ist ein Slot des Geräts. Ohne Dev-Mode (Manager.Dev) ist sie
// verboten, noch vor jeder Feldprüfung, damit ein Server ohne Dev-Mode nichts über Felder verrät; die Ablehnung steht
// als Warnung im Log, jede gelungene Aktion als Info.
func (r *Room) Dev(id string, peer Peer, a DevAction) error {
	if !r.devAllowed(short(id), a) {
		return ErrForbidden
	}
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if d == nil || d.observer { // Beobachter ändern nichts, auch nicht raumweit (Phase, Grad, Zeitraffer; Review TR4.2)
		return ErrBadRequest
	}
	return r.devApply(short(id), a, func(slot int) (int, bool) {
		idx, ok := d.slots[slot]
		return idx, ok
	})
}

// DevPage führt eine Dev-Aktion der Seite /dm aus (B-232): ohne Gerät, slot ist der Index des Monarchen.
func (r *Room) DevPage(a DevAction) error {
	if !r.devAllowed("dm", a) {
		return ErrForbidden
	}
	r.mu.Lock()
	defer r.mu.Unlock()
	return r.devApply("dm", a, func(slot int) (int, bool) { return slot, true })
}

// devAllowed prüft den Dev-Mode und loggt eine Ablehnung.
func (r *Room) devAllowed(who string, a DevAction) bool {
	if !r.m.Dev {
		r.log().Warn("🚫 Dev-Aktion abgelehnt", "device", who, "aktion", a.Action)
	}
	return r.m.Dev
}

// devApply wendet die Aktion an (unter Raum-Sperre); player bildet slot auf den Spielerindex der Insel ab.
func (r *Room) devApply(who string, a DevAction, player func(int) (int, bool)) error {
	attrs := []any{"device", who, "aktion", a.Action}
	var ok bool
	switch a.Action {
	case "gold", "material":
		var more []any
		more, ok = r.devStock(a, player)
		attrs = append(attrs, more...)
	case "wave", "castleFall":
		var idx int
		if idx, ok = devIndex(a, player); ok {
			if a.Action == "wave" {
				ok = sim.DevStartWave(r.isl, idx)
			} else {
				ok = sim.DevCastleFall(r.isl, idx)
			}
			attrs = append(attrs, "slot", *a.Slot)
		}
	case "phase":
		ok = sim.DevSetPhase(r.isl, a.Phase)
		attrs = append(attrs, "phase", a.Phase)
	case "grade":
		ok = sim.SetGrade(r.isl, a.Grade, r.m.Dev) == nil
		attrs = append(attrs, "grad", a.Grade)
	case "timescale":
		ok = slices.Contains(devTimescales, a.Factor)
		if ok {
			r.timescale = a.Factor
		}
		attrs = append(attrs, "faktor", a.Factor)
	case "pause":
		ok = a.Paused != nil
		if ok {
			r.paused = *a.Paused
			attrs = append(attrs, "angehalten", r.paused)
		}
	default:
		var more []any
		more, ok = r.devExtra(who, a, player)
		attrs = append(attrs, more...)
	}
	if !ok {
		return ErrBadRequest
	}
	r.log().Info("🐛 Dev-Aktion", attrs...)
	return nil
}

// devExtra führt die Aktionen für Lager, Baukosten, Ausbau, Tageszeit und Kampf aus und liefert die Log-Felder.
func (r *Room) devExtra(who string, a DevAction, player func(int) (int, bool)) (attrs []any, ok bool) {
	switch a.Action {
	case "maxStock":
		ok = sim.DevRaiseMaxStock(r.isl)
		attrs = []any{"bonus", r.isl.Dev.StockBonus}
	case "fillStock":
		sim.DevFillStock(r.isl)
		ok = true
	case "cheapBuild":
		if ok = a.On != nil; ok {
			sim.DevSetCheapBuild(r.isl, *a.On)
			attrs = []any{"an", *a.On}
		}
	case "buildAll":
		ok = sim.DevBuildAll(r.isl, a.Level)
		attrs = []any{"stufe", a.Level}
	case "nextPhase":
		attrs, ok = []any{"phase", sim.DevNextPhase(r.isl)}, true
	case "timescaleAuto":
		if ok = a.On != nil; ok {
			r.autoReset, r.resetPhase = *a.On, r.isl.Stages[r.start].Cycle.Phase
			attrs = []any{"an", *a.On}
		}
	default:
		attrs, ok = r.devCombat(who, a, player)
	}
	return attrs, ok
}

// devStock führt gold und material aus und liefert die Log-Felder.
func (r *Room) devStock(a DevAction, player func(int) (int, bool)) ([]any, bool) {
	idx, ok := devIndex(a, player)
	if !ok || a.Amount < 1 || a.Amount > devMaxAmount {
		return nil, false
	}
	attrs := []any{"slot", *a.Slot, "amount", a.Amount}
	if a.Action == "gold" {
		return attrs, sim.DevDropGold(r.isl, idx, a.Amount)
	}
	taken, ok := sim.DevAddStock(r.isl, idx, a.Resource, a.Amount)
	return append(attrs, "resource", a.Resource, "genommen", taken), ok
}

// devIndex liefert den Spielerindex zum slot der Aktion.
func devIndex(a DevAction, player func(int) (int, bool)) (int, bool) {
	if a.Slot == nil {
		return 0, false
	}
	return player(*a.Slot)
}

// resetScaleOnPhase beendet den Zeitraffer beim ersten Tageszeitwechsel nach timescaleAuto (unter Raum-Sperre).
func (r *Room) resetScaleOnPhase() {
	if !r.autoReset || r.isl.Stages[r.start].Cycle.Phase == r.resetPhase {
		return
	}
	r.log().Info("⏩ Zeitraffer endet mit der Tageszeit", "faktor", r.timescale, "phase", r.isl.Stages[r.start].Cycle.Phase)
	r.timescale, r.autoReset = 1, false
}

// steps sind die Simulationsschritte eines Ticks: 0 in der Dev-Pause und in der Pause des Couch-Raums (pause.go),
// sonst scale() (unter Raum-Sperre).
func (r *Room) steps() int {
	if r.paused || r.couchPaused() {
		return 0
	}
	return r.scale()
}

// scale ist der Faktor des Zeitraffers, mindestens 1 (unter Raum-Sperre).
func (r *Room) scale() int {
	return max(r.timescale, 1)
}
