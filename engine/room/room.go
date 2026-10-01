// Package room ist das Raummodell aus docs/protocol.md (Protokoll v2): Räume, Geräte mit lokalen Spielern (Slots),
// Monarchen (taken, waiting, free), Fristen nach Wanduhr und Speichern. Es kennt kein Netz; engine/net setzt Peer
// je Verbindung um und übersetzt die Fehler in die Codes des Protokolls.
package room

import (
	"encoding/json"
	"errors"
	"slices"
	"sync"
	"time"

	"k3c/engine/level"
	"k3c/engine/sim"
)

// Grenzen und Fristen aus docs/protocol.md.
const (
	MaxMonarchs = 4
	MaxSlots    = 4
	MaxRooms    = 4
	TickHz      = 30
	WaitFor     = 60 * time.Second
	EmptyFor    = 10 * time.Minute
)

// Fehler; der Text ist der Code aus docs/protocol.md › Fehler-Codes.
var (
	ErrRoomFull     = errors.New("room_full")
	ErrTooManySlots = errors.New("too_many_slots")
	ErrTooManyRooms = errors.New("too_many_rooms")
	ErrRoomNotFound = errors.New("room_not_found")
	ErrSaveExists   = errors.New("save_exists")
	ErrSaveNotFound = errors.New("save_not_found")
	ErrBadRequest   = errors.New("bad_request")
)

// Zustände eines Monarchen.
const (
	Taken   = "taken"
	Waiting = "waiting"
	Free    = "free"
)

// Seat ordnet einem Slot des Geräts einen Monarchen zu.
type Seat struct {
	Slot    int `json:"slot"`
	Monarch int `json:"monarch"`
}

// Peer ist die Verbindung eines Geräts. Der Raum ruft die Methoden unter seiner Sperre auf: Sie dürfen nicht blockieren
// und nicht in den Raum oder Manager zurückrufen. State muss w sofort lesen, w gehört danach wieder dem Raum.
type Peer interface {
	Joined(room, name string, you []Seat)
	Level(depth int, layout level.Layout)
	State(tick int, w *sim.World)
	Seats(you []Seat, monarchs []string)
	Replaced() // dieselbe Geräte-ID ist über eine neue Verbindung beigetreten
	Closed()   // Raum geschlossen (Absturz oder Server fährt herunter)
}

type monarch struct {
	state  string
	device string // besetzendes bzw. zuletzt besetzendes Gerät
	slot   int
	since  time.Time // Beginn von waiting
	input  sim.PlayerCommand
}

type device struct {
	peer      Peer
	slots     map[int]int // Slot → Monarch
	connected bool
}

// Room ist ein laufendes Spiel: eine Kampagne, ein Spielstand, ein Code. Monarch i ist CurrentWorld().Players[i].
type Room struct {
	Code, Name string

	mu         sync.Mutex
	m          *Manager
	camp       *sim.Campaign
	monarchs   []*monarch
	devices    map[string]*device
	tick       int
	emptySince time.Time
	closed     bool
}

// validSlots: Slot 4 oder höher → too_many_slots, sonst leer, negativ oder doppelt → bad_request.
func validSlots(slots []int) error {
	for _, s := range slots {
		if s >= MaxSlots {
			return ErrTooManySlots
		}
	}
	for i, s := range slots {
		if s < 0 || slices.Contains(slots[:i], s) {
			return ErrBadRequest
		}
	}
	if len(slots) == 0 {
		return ErrBadRequest
	}
	return nil
}

// join nimmt ein Gerät auf oder verbindet es wieder (docs/protocol.md › Beitreten, Wiederverbinden). Alles oder nichts.
func (r *Room) join(id string, peer Peer, slots []int) error {
	if err := validSlots(slots); err != nil {
		return err
	}
	keep := map[int]int{}
	old := r.devices[id]
	if old != nil {
		for slot, idx := range old.slots {
			if slices.Contains(slots, slot) {
				keep[slot] = idx
			}
		}
	}
	released := 0
	if old != nil {
		released = len(old.slots) - len(keep)
	}
	if len(slots)-len(keep) > r.count(Free)+released+MaxMonarchs-len(r.monarchs) {
		return ErrRoomFull
	}
	if old != nil {
		if old.connected && old.peer != peer {
			old.peer.Replaced()
		}
		for slot, idx := range old.slots {
			if _, ok := keep[slot]; !ok {
				r.release(idx)
			}
		}
	}
	d := &device{peer: peer, slots: keep, connected: true}
	r.devices[id] = d
	for _, slot := range slots {
		idx, ok := keep[slot]
		if !ok {
			idx = r.assign(id, slot)
		}
		r.take(d, id, slot, idx)
	}
	r.emptySince = time.Time{}
	w := r.syncFree()
	peer.Joined(r.Code, r.Name, d.seats())
	peer.Level(r.camp.Depth, w.Level)
	peer.State(r.tick, w)
	r.broadcastSeats()
	return nil
}

// assign: freien Monarchen wählen, den (Gerät, Slot) zuletzt hatte, sonst den kleinsten freien, sonst einen neuen.
func (r *Room) assign(id string, slot int) int {
	best := -1
	for i, mo := range r.monarchs {
		if mo.state != Free {
			continue
		}
		if mo.device == id && mo.slot == slot {
			return i
		}
		if best < 0 {
			best = i
		}
	}
	if best >= 0 {
		return best
	}
	r.camp.JoinPlayer()
	r.monarchs = append(r.monarchs, &monarch{})
	return len(r.monarchs) - 1
}

func (r *Room) take(d *device, id string, slot, idx int) {
	d.slots[slot] = idx
	*r.monarchs[idx] = monarch{state: Taken, device: id, slot: slot}
}

func (r *Room) release(idx int) {
	mo := r.monarchs[idx]
	mo.state, mo.input = Free, sim.PlayerCommand{}
}

func (r *Room) count(state string) int {
	n := 0
	for _, mo := range r.monarchs {
		if mo.state == state {
			n++
		}
	}
	return n
}

func (r *Room) connected() int {
	n := 0
	for _, d := range r.devices {
		if d.connected {
			n++
		}
	}
	return n
}

func (d *device) seats() []Seat {
	out := []Seat{}
	for slot, idx := range d.slots {
		out = append(out, Seat{slot, idx})
	}
	slices.SortFunc(out, func(a, b Seat) int { return a.Slot - b.Slot })
	return out
}

// syncFree setzt sim.Player.Free (B-059): Nur besetzte Monarchen entscheiden über den Stufenwechsel.
func (r *Room) syncFree() *sim.World {
	w := r.camp.CurrentWorld()
	for i, mo := range r.monarchs {
		w.Players[i].Free = mo.state != Taken
	}
	return w
}

func (r *Room) states() []string {
	out := make([]string, len(r.monarchs))
	for i, mo := range r.monarchs {
		out[i] = mo.state
	}
	return out
}

func (r *Room) broadcastSeats() {
	states := r.states()
	for _, d := range r.devices {
		if d.connected {
			d.peer.Seats(d.seats(), states)
		}
	}
}

// save schreibt den Spielstand (docs/protocol.md › Spielstand). Fehler landen im Log, der Raum läuft weiter.
func (r *Room) save() {
	data, err := json.Marshal(r.camp.ToSave(r.m.now().UTC().Format("2006-01-02T15:04:05.000Z07:00")))
	if err == nil {
		_, err = r.m.Store.Store(r.Name, data)
	}
	if err != nil {
		r.m.log().Error("Spielstand nicht gespeichert", "room", r.Code, "save", r.Name, "err", err)
	}
}
