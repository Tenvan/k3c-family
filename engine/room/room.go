// Package room ist das Raummodell aus docs/protocol.md (Protokoll v2): Räume, Geräte mit lokalen Spielern (Slots),
// Monarchen (taken, waiting, free), Fristen nach Wanduhr und Speichern. Es kennt kein Netz; engine/net setzt Peer
// je Verbindung um und übersetzt die Fehler in die Codes des Protokolls.
package room

import (
	"encoding/json"
	"errors"
	"slices"
	"strings"
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
	ErrClosed       = errors.New("room_closed") // Server fährt herunter
	ErrForbidden    = errors.New("forbidden")   // dev ohne Dev-Mode (B-178)
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
	Depth   int `json:"depth"` // Tiefe der Stufe, in der der Monarch steht
}

// Options sind die Raum-Optionen aus `create` (leer = Standard). Der Manager prüft sie beim Anlegen eines neuen Stands und setzt sie auf die Insel.
type Options struct{ Grade, Goal, Defeat string }

// Peer ist die Verbindung eines Geräts. Der Raum ruft die Methoden unter seiner Sperre auf: Sie dürfen nicht blockieren
// und nicht in den Raum oder Manager zurückrufen. State muss w sofort lesen, w gehört danach wieder dem Raum.
type Peer interface {
	Joined(room, name string, you []Seat)
	Level(depth int, layout level.Layout)
	State(tick int, w *sim.World)
	Seats(you []Seat, monarchs []string)
	Replaced()         // dieselbe Geräte-ID ist über eine neue Verbindung beigetreten
	Closed(final bool) // Raum geschlossen; final: Server fährt herunter, die Verbindung endet danach
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
	stage     int // Stufe, deren Level das Gerät zuletzt bekam (-1: noch keine)
}

// Room ist ein laufendes Spiel: eine Insel, ein Spielstand, ein Code. Monarch i ist der Insel-Spieler mit Index i.
type Room struct {
	Code, Name string

	mu         sync.Mutex
	m          *Manager
	isl        *sim.Island
	Opts       Options // angeforderte Raum-Optionen (gelten nur für neue Stände)
	start      int // Startstufe: dort treten neue Geräte ohne Spieler ein
	monarchs   []*monarch
	devices    map[string]*device
	tick       int
	emptySince time.Time
	closed     bool
	durations  []time.Duration // letzte Tick-Dauern (run.go)
	slowLogged time.Time       // letzte Meldung eines langsamen Ticks (logging.go)
	slowCount  int             // langsame Ticks seit dieser Meldung
	beforeStep func()          // Test-Naht: läuft im Tick vor StepIsland
}

// ValidSlots: Slot 4 oder höher → too_many_slots, sonst leer, negativ oder doppelt → bad_request.
func ValidSlots(slots []int) error {
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
	if err := ValidSlots(slots); err != nil {
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
	d := &device{peer: peer, slots: keep, connected: true, stage: -1}
	r.devices[id] = d
	for _, slot := range slots {
		idx, ok := keep[slot]
		if !ok {
			idx = r.assign(id, slot, r.deviceStage(d))
		}
		r.take(d, id, slot, idx)
	}
	r.emptySince = time.Time{}
	r.syncFree()
	r.log().Info("Gerät im Raum", "device", short(id), "slots", slots, "wiederverbunden", old != nil, "geraete", r.connected())
	peer.Joined(r.Code, r.Name, r.seats(d))
	r.pushState(d)
	r.broadcastSeats()
	return nil
}

// assign: freien Monarchen wählen, den (Gerät, Slot) zuletzt hatte, sonst den kleinsten freien, sonst einen neuen
// Insel-Spieler in der Stufe stage (die Stufe des Geräts).
func (r *Room) assign(id string, slot, stage int) int {
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
	sim.AddIslandPlayer(r.isl, stage)
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

func (r *Room) seats(d *device) []Seat {
	out := []Seat{}
	for slot, idx := range d.slots {
		out = append(out, Seat{slot, idx, r.isl.Stages[r.isl.StageOf(idx)].Biome.Depth})
	}
	slices.SortFunc(out, func(a, b Seat) int { return a.Slot - b.Slot })
	return out
}

// syncFree setzt sim.Player.Free (B-059): Nur besetzte Monarchen entscheiden über den Stufenwechsel.
func (r *Room) syncFree() {
	players := r.isl.Players()
	for i, mo := range r.monarchs {
		players[i].Free = mo.state != Taken
	}
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
			d.peer.Seats(r.seats(d), states)
		}
	}
}

// save schreibt den Spielstand (docs/protocol.md › Spielstand). Fehler landen im Log, der Raum läuft weiter.
// dropTestSave löscht den Spielstand eines Testraums (TestPrefix), nachdem der leere Raum aufgeräumt wurde.
func (r *Room) dropTestSave() {
	if !strings.HasPrefix(r.Name, TestPrefix) {
		return
	}
	if err := r.m.Store.Delete(r.Name); err != nil {
		r.m.log().Error("Test-Spielstand nicht gelöscht", "room", r.Code, "save", r.Name, "err", err)
	}
}

func (r *Room) save() {
	backup, err := r.store()
	if err != nil {
		r.log().Error("Spielstand nicht gespeichert", "err", err)
		return
	}
	r.log().Debug("Spielstand gespeichert", "tick", r.tick, "sicherung", backup)
}

// store schreibt den Spielstand und liefert den Namen der Sicherung des vorigen Stands (leer, wenn es keine gab).
func (r *Room) store() (backup string, err error) {
	data, err := json.Marshal(r.isl.ToSave(r.m.now().UTC().Format("2006-01-02T15:04:05.000Z07:00")))
	if err != nil {
		return "", err
	}
	return r.m.Store.Store(r.Name, data)
}

// SaveNow sichert den Spielstand sofort (Diagnose, B-088) und meldet den Fehler, statt ihn nur zu loggen.
func (r *Room) SaveNow() (backup string, err error) {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.closed {
		return "", ErrClosed
	}
	return r.store()
}
