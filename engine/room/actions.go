package room

import (
	"math"
	"time"

	"k3c/engine/sim"
)

// Aktionen eines Geräts im Raum und der Takt. Jede öffentliche Methode sperrt den Raum; ändern sich Plätze, erfährt
// der Manager es danach (Raumliste an Geräte ohne Raum).

// AddSlot nimmt einen lokalen Spieler dazu (docs/protocol.md › Lokale Spieler hinzufügen/entfernen).
func (r *Room) AddSlot(id string, slot int) error {
	r.mu.Lock()
	err := r.addSlot(id, slot)
	r.mu.Unlock()
	r.m.notify(err == nil)
	return err
}

func (r *Room) addSlot(id string, slot int) error {
	d := r.devices[id]
	switch {
	case slot >= MaxSlots:
		return ErrTooManySlots
	case d == nil || !d.connected || slot < 0:
		return ErrBadRequest
	}
	if _, used := d.slots[slot]; used {
		return ErrBadRequest
	}
	if r.count(Free) == 0 && len(r.monarchs) >= MaxMonarchs {
		return ErrRoomFull
	}
	r.take(d, id, slot, r.assign(id, slot))
	r.syncFree()
	r.broadcastSeats()
	return nil
}

// RemoveSlot lässt einen lokalen Spieler gehen; sein Monarch ist sofort frei. Ohne Slot verlässt das Gerät den Raum.
func (r *Room) RemoveSlot(id string, slot int) error {
	r.mu.Lock()
	err := r.removeSlot(id, slot)
	r.mu.Unlock()
	r.m.notify(err == nil)
	return err
}

func (r *Room) removeSlot(id string, slot int) error {
	d := r.devices[id]
	if d == nil || !d.connected {
		return ErrBadRequest
	}
	idx, ok := d.slots[slot]
	if !ok {
		return ErrBadRequest
	}
	r.release(idx)
	delete(d.slots, slot)
	if len(d.slots) == 0 {
		r.leave(id)
		return nil
	}
	r.syncFree()
	r.broadcastSeats()
	return nil
}

// Input setzt die Eingabe eines Slots; der Raum rechnet mit der zuletzt empfangenen. moveX wird auf −1…1 begrenzt.
func (r *Room) Input(id string, slot int, cmd sim.PlayerCommand) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	d := r.devices[id]
	if d == nil || !d.connected {
		return ErrBadRequest
	}
	idx, ok := d.slots[slot]
	if !ok || math.IsNaN(cmd.MoveX) {
		return ErrBadRequest
	}
	cmd.MoveX = math.Max(-1, math.Min(1, cmd.MoveX))
	r.monarchs[idx].input = cmd
	return nil
}

// Leave: Das Gerät verlässt den Raum bewusst, alle seine Monarchen sind sofort frei.
func (r *Room) Leave(id string) {
	r.mu.Lock()
	r.leave(id)
	r.mu.Unlock()
	r.m.notify(true)
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
	r.afterDisconnect()
}

// Drop: Die Verbindung peer ist abgebrochen. Seine Monarchen warten (stehen still), nach WaitFor sind sie frei.
// Eine schon ersetzte Verbindung wird ignoriert.
func (r *Room) Drop(id string, peer Peer) {
	r.mu.Lock()
	d := r.devices[id]
	ok := d != nil && d.connected && d.peer == peer
	if ok {
		now := r.m.now()
		for _, idx := range d.slots {
			mo := r.monarchs[idx]
			mo.state, mo.since, mo.input = Waiting, now, sim.PlayerCommand{}
		}
		d.connected = false
		r.afterDisconnect()
	}
	r.mu.Unlock()
	r.m.notify(ok)
}

// afterDisconnect: Plätze melden; ist kein Gerät mehr verbunden, ist der Raum pausiert und speichert sofort.
func (r *Room) afterDisconnect() {
	r.syncFree()
	r.broadcastSeats()
	if r.connected() == 0 {
		r.save()
		r.emptySince = r.m.now()
	}
}

// Tick rechnet einen Schritt mit 1/TickHz Sekunden. Ein Raum ohne verbundenes Gerät ist pausiert und tickt nicht.
// Nach einem Stufenwechsel speichert er und schickt das neue Level vor dem Zustand.
func (r *Room) Tick() {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.closed || r.connected() == 0 {
		return
	}
	commands := make([]sim.PlayerCommand, len(r.monarchs))
	for i, mo := range r.monarchs {
		if mo.state == Taken {
			commands[i] = mo.input
		}
	}
	w := r.syncFree()
	sim.Step(w, commands, 1.0/TickHz)
	r.tick++
	travelled := w.Travel != nil && w.Travel.Progress >= 1
	if travelled {
		w = r.camp.Travel(w.Travel.ToDepth)
		r.save()
	}
	for _, d := range r.devices {
		if !d.connected {
			continue
		}
		if travelled {
			d.peer.Level(r.camp.Depth, w.Level)
		}
		d.peer.State(r.tick, w)
	}
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

// Info ist ein Eintrag der Raumliste (docs/protocol.md › Nachrichten, rooms).
type Info struct {
	Code    string `json:"code"`
	Name    string `json:"name"`
	Depth   int    `json:"depth"`
	Taken   int    `json:"taken"` // besetzt + wartend
	Free    int    `json:"free"`  // 4 − taken
	Running bool   `json:"running"`
}

func (r *Room) info() Info {
	taken := r.count(Taken) + r.count(Waiting)
	return Info{r.Code, r.Name, r.camp.Depth, taken, MaxMonarchs - taken, r.connected() > 0}
}
