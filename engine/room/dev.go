package room

import (
	"slices"

	"k3c/engine/sim"
)

// DevAction ist eine Dev-Aktion eines Geräts (Nachricht dev, B-178): gold und material brauchen slot und amount,
// material zusätzlich resource, timescale braucht factor, pause braucht paused (B-231); wave braucht slot, phase braucht
// phase (day, dusk, night) (B-232).
type DevAction struct {
	Action   string
	Slot     *int
	Amount   int
	Resource string
	Factor   int
	Paused   *bool
	Phase    string
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
	if d == nil {
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
	case "wave":
		var idx int
		if idx, ok = devIndex(a, player); ok {
			ok = sim.DevStartWave(r.isl, idx)
			attrs = append(attrs, "slot", *a.Slot)
		}
	case "phase":
		ok = sim.DevSetPhase(r.isl, a.Phase)
		attrs = append(attrs, "phase", a.Phase)
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
		ok = false
	}
	if !ok {
		return ErrBadRequest
	}
	r.log().Info("🐛 Dev-Aktion", attrs...)
	return nil
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

// steps sind die Simulationsschritte eines Ticks: 0 in der Dev-Pause, sonst scale() (unter Raum-Sperre).
func (r *Room) steps() int {
	if r.paused {
		return 0
	}
	return r.scale()
}

// scale ist der Faktor des Zeitraffers, mindestens 1 (unter Raum-Sperre).
func (r *Room) scale() int {
	return max(r.timescale, 1)
}
