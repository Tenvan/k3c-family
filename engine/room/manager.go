package room

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"log/slog"
	"math/rand/v2"
	"regexp"
	"slices"
	"strings"
	"sync"
	"time"

	"k3c/engine/sim"
	"k3c/engine/store"
)

// Store sind die Spielstände (engine/store.Saves). Load liefert store.ErrNotFound, wenn es keinen gibt.
type Store interface {
	Load(name string) ([]byte, error)
	Store(name string, data []byte) (backup string, err error)
	// Delete entfernt einen Spielstand samt Sicherungen; ein fehlender ist kein Fehler.
	Delete(name string) error
}

// TestPrefix reserviert Spielstandnamen für Testläufe (Testseite, B-086): Räume mit diesem Präfix löschen ihren Spielstand,
// wenn sie nach der Frist EmptyFor aufgeräumt werden, der Server beim Start auch alte Dateien (store.Saves.Purge).
const TestPrefix = "test-"

var saveName = regexp.MustCompile(`^[a-z0-9-]{1,32}$`)

// codeLetters: Großbuchstaben ohne I und O (docs/protocol.md › Begriffe).
const codeLetters = "ABCDEFGHJKLMNPQRSTUVWXYZ"

// Manager hält alle Räume eines Servers. Sperr-Reihenfolge: erst Manager, dann Raum, nie umgekehrt.
type Manager struct {
	Store   Store
	Now     func() time.Time // Test-Naht; nil = time.Now
	NewCode func() string    // Test-Naht; nil = zufällig
	Log     *slog.Logger
	// Changed meldet, dass sich Raumliste oder Plätze geändert haben (für `rooms` an Geräte ohne Raum).
	// Aufruf ohne gehaltene Sperre.
	Changed func()

	mu       sync.Mutex
	rooms    map[string]*Room
	ctx      context.Context // gesetzt von Run; neue Räume bekommen dann ihre Goroutine
	failures []Failure       // die letzten abgestürzten Räume
	shut     bool            // nach Close: keine neuen Räume
}

// NewManager legt einen Manager ohne Räume an.
func NewManager(s Store) *Manager { return &Manager{Store: s, rooms: map[string]*Room{}} }

func (m *Manager) now() time.Time {
	if m.Now == nil {
		return time.Now()
	}
	return m.Now()
}

func (m *Manager) log() *slog.Logger {
	if m.Log == nil {
		return slog.New(slog.NewTextHandler(io.Discard, nil))
	}
	return m.Log
}

func (m *Manager) notify(changed bool) {
	if changed && m.Changed != nil {
		m.Changed()
	}
}

func (m *Manager) code() string {
	for {
		c := ""
		if m.NewCode != nil {
			c = m.NewCode()
		} else {
			var b strings.Builder
			for range 4 {
				b.WriteByte(codeLetters[rand.IntN(len(codeLetters))])
			}
			c = b.String()
		}
		if _, used := m.rooms[c]; !used {
			return c
		}
	}
}

// Create erstellt einen Raum mit neuem (fresh) oder gespeichertem Spielstand und nimmt das Gerät auf. Prüf-Reihenfolge
// aus docs/protocol.md › Beitreten; ist der gespeicherte Stand schon offen, tritt das Gerät diesem Raum bei.
func (m *Manager) Create(id string, peer Peer, name string, fresh bool, depth int, slots []int) (*Room, error) {
	if err := ValidSlots(slots); err != nil {
		return nil, err
	}
	if !saveName.MatchString(name) {
		return nil, ErrBadRequest
	}
	var r *Room
	var err error
	defer func() { m.notify(err == nil) }()
	m.mu.Lock()
	defer m.mu.Unlock()
	if m.shut {
		return nil, ErrClosed
	}
	r, err = m.create(id, peer, name, fresh, depth, slots)
	return r, err
}

func (m *Manager) create(id string, peer Peer, name string, fresh bool, depth int, slots []int) (*Room, error) {
	for _, r := range m.rooms {
		if r.Name != name {
			continue
		}
		if fresh {
			return nil, ErrSaveExists
		}
		return r, r.lockedJoin(id, peer, slots)
	}
	camp, err := m.open(name, fresh, depth)
	if err != nil {
		return nil, err
	}
	if len(m.rooms) >= MaxRooms {
		return nil, ErrTooManyRooms
	}
	r := &Room{Code: m.code(), Name: name, m: m, camp: camp, devices: map[string]*device{}}
	m.rooms[r.Code] = r
	if m.ctx != nil {
		go r.run(m.ctx)
	}
	return r, r.lockedJoin(id, peer, slots)
}

// open lädt den Spielstand oder legt einen neuen an. Ein geladener Stand hat keine Monarchen, Gold gilt pro Index.
func (m *Manager) open(name string, fresh bool, depth int) (*sim.Campaign, error) {
	data, err := m.Store.Load(name)
	switch {
	case fresh && err == nil:
		return nil, ErrSaveExists
	case !fresh && errors.Is(err, store.ErrNotFound):
		return nil, ErrSaveNotFound
	case err != nil && !errors.Is(err, store.ErrNotFound):
		return nil, err
	case fresh:
		// Neuer Stand wie createCampaign(seed, {depth}) in TS: leerer Spielstand dieser Tiefe, Seed = Name.
		// ponytail: Seed ist der Name; ein eigener Seed käme mit einem Feld in `create` (Protokoll-Änderung).
		data, _ = json.Marshal(sim.SaveGame{
			Version: sim.SaveVersion, CampaignID: name + "-" + m.now().UTC().Format("20060102T150405.000"), Seed: name,
			Depth: depth, UnlockedDepth: depth, Players: []sim.PlayerSave{}, Hubs: []sim.HubSave{},
		})
	}
	s, err := sim.ParseSave(data)
	if err != nil {
		if fresh {
			return nil, ErrBadRequest // unbekannte Startstufe
		}
		return nil, err
	}
	return sim.FromSave(s, 1), nil
}

func (r *Room) lockedJoin(id string, peer Peer, slots []int) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.closed {
		return ErrRoomNotFound
	}
	return r.join(id, peer, slots)
}

// Join tritt einem Raum per Code bei oder verbindet wieder.
func (m *Manager) Join(id string, peer Peer, code string, slots []int) (*Room, error) {
	if err := ValidSlots(slots); err != nil {
		return nil, err
	}
	err := ErrRoomNotFound
	defer func() { m.notify(err == nil) }()
	m.mu.Lock()
	defer m.mu.Unlock()
	r := m.rooms[code]
	if m.shut {
		err = ErrClosed
	} else if r != nil {
		err = r.lockedJoin(id, peer, slots)
	}
	return r, err
}

// Room ist der Raum mit diesem Code oder nil.
func (m *Manager) Room(code string) *Room {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.rooms[code]
}

// Rooms ist die Raumliste, nach Code sortiert.
func (m *Manager) Rooms() []Info {
	m.mu.Lock()
	defer m.mu.Unlock()
	out := []Info{}
	for _, r := range m.rooms {
		r.mu.Lock()
		out = append(out, r.info())
		r.mu.Unlock()
	}
	slices.SortFunc(out, func(a, b Info) int { return strings.Compare(a.Code, b.Code) })
	return out
}

// Sweep prüft die Fristen aller Räume nach Wanduhr: wartende Monarchen nach WaitFor frei, leere Räume nach EmptyFor
// speichern und aufräumen (Code wird frei).
func (m *Manager) Sweep() {
	now := m.now()
	changed := false
	defer func() { m.notify(changed) }()
	m.mu.Lock()
	defer m.mu.Unlock()
	for code, r := range m.rooms {
		c, remove := r.lockedSweep(now)
		if remove {
			delete(m.rooms, code)
		}
		changed = changed || c || remove
	}
}

func (r *Room) lockedSweep(now time.Time) (changed, remove bool) {
	r.mu.Lock()
	defer r.mu.Unlock()
	changed, remove = r.sweep(now)
	if remove {
		r.save()
		r.closed = true
		r.dropTestSave()
	}
	return changed, remove
}

// Close speichert alle Räume, meldet ihren Geräten `room_closed` und räumt sie auf (Server fährt herunter).
// Danach nimmt der Manager keine Räume mehr an (room_closed).
func (m *Manager) Close() {
	defer m.notify(true)
	m.mu.Lock()
	defer m.mu.Unlock()
	m.shut = true
	for code, r := range m.rooms {
		r.closeFinal()
		delete(m.rooms, code)
	}
}

func (r *Room) closeFinal() {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.save()
	r.closed = true
	for _, d := range r.devices {
		if d.connected {
			d.peer.Closed(true)
		}
	}
}
