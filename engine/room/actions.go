package room

import (
	"math"
	"slices"
	"time"

	"k3c/engine/sim"
)

// Aktionen eines Geräts im Raum und der Takt. Jede öffentliche Methode sperrt den Raum; ändern sich Plätze, erfährt
// der Manager es danach (Raumliste an Geräte ohne Raum).

// own ist das Gerät id, wenn peer seine aktuelle Verbindung ist und der Raum offen; sonst nil. Eine ersetzte
// Verbindung darf nichts mehr am neuen Gerät ändern.
func (r *Room) own(id string, peer Peer) *device {
	d := r.devices[id]
	if r.closed || d == nil || !d.connected || d.peer != peer {
		return nil
	}
	return d
}

// AddSlot nimmt einen lokalen Spieler dazu (docs/protocol.md › Lokale Spieler hinzufügen/entfernen).
func (r *Room) AddSlot(id string, peer Peer, slot int) (err error) {
	defer func() { r.m.notify(err == nil) }() // läuft nach dem Entsperren
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	switch {
	case slot >= MaxSlots:
		return ErrTooManySlots
	case d == nil || slot < 0:
		return ErrBadRequest
	}
	if _, used := d.slots[slot]; used {
		return ErrBadRequest
	}
	if r.count(Free) == 0 && len(r.monarchs) >= MaxMonarchs {
		return ErrRoomFull
	}
	r.take(d, id, slot, r.assign(id, slot, r.deviceStage(d)))
	r.syncFree()
	r.broadcastSeats()
	return nil
}

// RemoveSlot lässt einen lokalen Spieler gehen; sein Monarch ist sofort frei. left: Das war der letzte Slot, das
// Gerät hat den Raum verlassen.
func (r *Room) RemoveSlot(id string, peer Peer, slot int) (left bool, err error) {
	defer func() { r.m.notify(err == nil) }()
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if d == nil {
		return false, ErrBadRequest
	}
	idx, ok := d.slots[slot]
	if !ok {
		return false, ErrBadRequest
	}
	r.release(idx)
	delete(d.slots, slot)
	if len(d.slots) == 0 {
		r.leave(id)
		return true, nil
	}
	r.syncFree()
	r.broadcastSeats()
	return false, nil
}

// Input setzt die Eingaben mehrerer Slots, alles oder nichts; der Raum rechnet mit der zuletzt empfangenen.
// moveX wird auf −1…1 begrenzt, NaN ist ungültig.
func (r *Room) Input(id string, peer Peer, in map[int]sim.PlayerCommand) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if d == nil || len(in) == 0 {
		return ErrBadRequest
	}
	for slot, cmd := range in {
		if _, ok := d.slots[slot]; !ok || math.IsNaN(cmd.MoveX) {
			return ErrBadRequest
		}
	}
	for slot, cmd := range in {
		cmd.MoveX = math.Max(-1, math.Min(1, cmd.MoveX))
		r.monarchs[d.slots[slot]].input = cmd
	}
	return nil
}

// Leave: Das Gerät verlässt den Raum bewusst, alle seine Monarchen sind sofort frei.
func (r *Room) Leave(id string, peer Peer) {
	defer r.m.notify(true)
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.own(id, peer) != nil {
		r.leave(id)
	}
}

func (r *Room) leave(id string) {
	d := r.devices[id]
	if d == nil {
		return
	}
	for _, idx := range d.slots {
		r.release(idx)
	}
	delete(r.devices, id)
	r.log().Info("👋 Gerät hat den Raum verlassen", "device", short(id), "geraete", r.connected())
	r.afterDisconnect()
}

// Drop: Die Verbindung peer ist abgebrochen. Seine Monarchen warten (stehen still), nach WaitFor sind sie frei.
// Eine schon ersetzte Verbindung oder ein geschlossener Raum wird ignoriert.
func (r *Room) Drop(id string, peer Peer) {
	ok := false
	defer func() { r.m.notify(ok) }()
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.own(id, peer)
	if ok = d != nil; !ok {
		return
	}
	now := r.m.now()
	for _, idx := range d.slots {
		mo := r.monarchs[idx]
		mo.state, mo.since, mo.input = Waiting, now, sim.PlayerCommand{}
	}
	d.connected = false
	r.met.drop(id)
	r.log().Info("⏳ Gerät getrennt, Monarchen warten", "device", short(id), "frist", WaitFor.String(), "geraete", r.connected())
	r.afterDisconnect()
}

// Closed: Ist der Raum geschlossen (aufgeräumt, abgestürzt, Server beendet)?
func (r *Room) Closed() bool {
	r.mu.Lock()
	defer r.mu.Unlock()
	return r.closed
}

// afterDisconnect: Plätze melden und speichern, bei jedem Verlassen und jedem Abbruch eines Geräts (B-147, S2.2); ist
// kein Gerät mehr verbunden, ist der Raum zusätzlich pausiert und die Frist EmptyFor läuft.
// Ein geschlossener Raum speichert nicht (nach einem Absturz ist sein Zustand nicht sicher).
func (r *Room) afterDisconnect() {
	if r.closed {
		return
	}
	r.syncFree()
	r.broadcastSeats()
	if r.connected() > 0 {
		r.save()
		return
	}
	r.log().Info("💾 Raum leer und pausiert, speichert")
	if r.timescale > 1 {
		r.log().Info("⏩ Zeitraffer beendet", "faktor", r.timescale)
	}
	r.timescale, r.paused = 1, false
	r.save()
	r.emptySince = r.m.now()
}

// Tick rechnet einen Schritt mit 1/TickHz Sekunden, im Zeitraffer scale() solche Schritte. Ein Raum ohne verbundenes Gerät ist pausiert und tickt nicht.
// Wechselt ein Gerät die Stufe, speichert der Raum und schickt das neue Level vor dem Zustand.
// false: Der Raum ist geschlossen.
func (r *Room) Tick() bool {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.closed {
		return false
	}
	if r.connected() == 0 {
		return true
	}
	commands := make([]sim.PlayerCommand, len(r.monarchs))
	for i, mo := range r.monarchs {
		if mo.state == Taken {
			commands[i] = mo.input
		}
	}
	r.syncFree()
	if r.beforeStep != nil {
		r.beforeStep()
	}
	before := r.depths()
	for range r.steps() { // Zeitraffer: scale() Schritte mit denselben Eingaben, ein Tick; Dev-Pause: keiner
		sim.StepIsland(r.isl, commands, 1.0/TickHz)
		r.met.observe(r.isl)
	}
	r.tick++
	travelled := false
	for _, d := range r.devices {
		if d.connected && r.pushState(d) {
			travelled = true
		}
	}
	if after := r.depths(); !slices.Equal(before, after) { // irgendein Monarch hat die Stufe gewechselt
		travelled = true
		r.log().Info("🪜 Stufenwechsel", "vorher", before, "nachher", after, "tick", r.tick)
		r.broadcastSeats()
	}
	if travelled {
		r.save()
	}
	return true
}

// sweep prüft die Fristen nach Wanduhr, auch im pausierten Raum. remove: Raum ist seit EmptyFor leer.
func (r *Room) sweep(now time.Time) (changed, remove bool) {
	for _, mo := range r.monarchs {
		if mo.state == Waiting && now.Sub(mo.since) >= WaitFor {
			r.release(r.index(mo))
			changed = true
		}
	}
	for id, d := range r.devices {
		if !d.connected && r.allFree(d) {
			delete(r.devices, id)
		}
	}
	if changed {
		r.log().Info("⏳ Wartende Monarchen nach Frist frei")
		r.syncFree()
		r.broadcastSeats()
	}
	return changed, r.connected() == 0 && !r.emptySince.IsZero() && now.Sub(r.emptySince) >= EmptyFor
}

func (r *Room) index(mo *monarch) int {
	for i, m := range r.monarchs {
		if m == mo {
			return i
		}
	}
	return -1
}

func (r *Room) allFree(d *device) bool {
	for _, idx := range d.slots {
		if r.monarchs[idx].state != Free {
			return false
		}
	}
	return true
}

// Has: Ist das Gerät (verbunden oder wartend) im Raum?
func (r *Room) Has(id string) bool {
	r.mu.Lock()
	defer r.mu.Unlock()
	_, ok := r.devices[id]
	return ok
}

// Info ist ein Eintrag der Raumliste (docs/protocol.md › Nachrichten, rooms).
type Info struct {
	Code    string `json:"code"`
	Name    string `json:"name"`
	Depth   int    `json:"depth"`
	Grade   string `json:"grade"`
	Taken   int    `json:"taken"` // besetzt + wartend
	Free    int    `json:"free"`  // 4 − taken
	Running bool   `json:"running"`
}

func (r *Room) info() Info {
	taken := r.count(Taken) + r.count(Waiting)
	return Info{r.Code, r.Name, r.isl.Stages[0].Biome.Depth, r.isl.Options.Grade, taken, MaxMonarchs - taken, r.connected() > 0}
}
