package room

import (
	"time"

	"k3c/engine/sim"
)

// Pause eines Geräts (docs/rules/bedienung.md § 1, docs/protocol.md › v8, B-214): Nachrichten `pause` und `resume`
// setzen device.paused. Ist das pausierte Gerät das einzige verbundene (Couch-Raum), rechnet der Tick keinen Schritt.
// Sonst läuft der Raum weiter, die Monarchen des Geräts stehen still und sind protectTicks lang geschützt
// (sim.Player.Protected), danach verwundbar. Die Frist zählt Ticks (deterministisch), nicht die Wanduhr.

// protectTicks ist die Höchstdauer des Schutzes online: WaitFor (60 s) in Ticks.
const protectTicks = int(WaitFor/time.Second) * TickHz

// Pause hält das Gerät an (on) oder setzt es fort. Ein zweites `pause` verlängert den Schutz nicht, ein `resume` ohne
// Pause ändert nichts. Beim Anhalten und Fortsetzen verfallen die letzten Eingaben seiner Monarchen (auch die während
// der Pause empfangenen), damit niemand mit einer alten Eingabe losläuft.
func (r *Room) Pause(id string, peer Peer, on bool) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if d == nil {
		return ErrBadRequest
	}
	if d.paused == on {
		return nil
	}
	d.paused, d.protect = on, 0
	if on {
		d.protect = protectTicks
	}
	for _, idx := range d.slots {
		r.monarchs[idx].input = sim.PlayerCommand{}
	}
	r.log().Info("🚦 Gerät pausiert", "device", short(id), "pausiert", on, "couch", r.couchPaused())
	return nil
}

// couchPaused: Das einzige verbundene Gerät ist pausiert, der Raum steht (unter Raum-Sperre).
func (r *Room) couchPaused() bool {
	n, paused := 0, false
	for _, d := range r.devices {
		if d.connected {
			n++
			paused = d.paused
		}
	}
	return n == 1 && paused
}

// devicePaused: Das Gerät id ist verbunden und pausiert; seine Eingaben zählen nicht (unter Raum-Sperre).
func (r *Room) devicePaused(id string) bool {
	d := r.devices[id]
	return d != nil && d.connected && d.paused
}

// pauseTick setzt vor den Schritten eines Ticks den Schutz je Monarch und zählt die Frist der pausierten Geräte
// herunter; im Couch-Raum und in der Dev-Pause steht sie (unter Raum-Sperre).
func (r *Room) pauseTick() {
	couch := r.couchPaused()
	players := r.isl.Players()
	for i, mo := range r.monarchs {
		d := r.devices[mo.device]
		players[i].Protected = !couch && mo.state == Taken && r.devicePaused(mo.device) && d.protect > 0
	}
	if couch || r.paused {
		return
	}
	for id, d := range r.devices {
		if !d.connected || !d.paused || d.protect == 0 {
			continue
		}
		if d.protect--; d.protect == 0 {
			r.log().Info("⏳ Schutz der Pause endet", "device", short(id), "frist", WaitFor.String())
		}
	}
}
