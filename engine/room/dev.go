package room

import (
	"slices"

	"k3c/engine/sim"
)

// DevAction ist eine Dev-Aktion eines Geräts (Nachricht dev, B-178): gold und material brauchen slot und amount,
// material zusätzlich resource, timescale braucht factor, pause braucht paused (B-231).
type DevAction struct {
	Action   string
	Slot     *int
	Amount   int
	Resource string
	Factor   int
	Paused   *bool
}

// devMaxAmount ist die Obergrenze von amount (gold, material).
const devMaxAmount = 1000

// MaxTimescale ist der größte Faktor des Zeitraffers; er schützt das Tick-Budget (B-178).
const MaxTimescale = 8

// devTimescales sind die erlaubten Faktoren von timescale.
var devTimescales = []int{1, 2, 4, MaxTimescale}

// Dev führt eine Dev-Aktion aus. Ohne Dev-Mode (Manager.Dev) ist sie verboten, noch vor jeder Feldprüfung, damit ein
// Server ohne Dev-Mode nichts über Felder verrät; die Ablehnung steht als Warnung im Log, jede gelungene Aktion als Info.
func (r *Room) Dev(id string, peer Peer, a DevAction) error {
	if !r.m.Dev {
		r.log().Warn("🚫 Dev-Aktion abgelehnt", "device", short(id), "aktion", a.Action)
		return ErrForbidden
	}
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if d == nil {
		return ErrBadRequest
	}
	attrs := []any{"device", short(id), "aktion", a.Action}
	switch a.Action {
	case "gold":
		idx, ok := devPlayer(d, a)
		if !ok || !sim.DevDropGold(r.isl, idx, a.Amount) {
			return ErrBadRequest
		}
		attrs = append(attrs, "slot", *a.Slot, "amount", a.Amount)
	case "material":
		idx, ok := devPlayer(d, a)
		if !ok {
			return ErrBadRequest
		}
		taken, ok := sim.DevAddStock(r.isl, idx, a.Resource, a.Amount)
		if !ok {
			return ErrBadRequest
		}
		attrs = append(attrs, "slot", *a.Slot, "amount", a.Amount, "resource", a.Resource, "genommen", taken)
	case "timescale":
		if !slices.Contains(devTimescales, a.Factor) {
			return ErrBadRequest
		}
		r.timescale = a.Factor
		attrs = append(attrs, "faktor", a.Factor)
	case "pause":
		if a.Paused == nil {
			return ErrBadRequest
		}
		r.paused = *a.Paused
		attrs = append(attrs, "angehalten", r.paused)
	default:
		return ErrBadRequest
	}
	r.log().Info("🐛 Dev-Aktion", attrs...)
	return nil
}

// devPlayer prüft slot und amount und liefert den Spielerindex der Insel zum Slot des Geräts.
func devPlayer(d *device, a DevAction) (int, bool) {
	if a.Slot == nil || a.Amount < 1 || a.Amount > devMaxAmount {
		return 0, false
	}
	idx, ok := d.slots[*a.Slot]
	return idx, ok
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
