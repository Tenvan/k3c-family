package net

import (
	"context"
	"encoding/json"
	"net/http"
	"sync"
	"sync/atomic"

	"github.com/coder/websocket"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
)

// WebSocket /ws nach Protokoll v2. Jede Verbindung hat eine Lese-Schleife (diese Goroutine) und eine
// Schreib-Goroutine mit gepuffertem Kanal. Ist der Puffer voll, wird die Verbindung geschlossen, das zählt als Abbruch.

const (
	sendBuffer   = 64
	maxMsgBytes  = 16 << 10
	maxDeviceLen = 64
)

// out ist eine Nachricht im Sendekanal; mit close statt data schließt die Schreib-Goroutine die Verbindung.
type out struct {
	data   []byte
	close  websocket.StatusCode
	reason string
}

// conn ist ein Gerät; es setzt room.Peer um. Die Peer-Methoden laufen unter der Sperre des Raums und schreiben nur
// in den Sendekanal.
type conn struct {
	s      *server
	ws     *websocket.Conn
	device string
	send   chan out
	cancel context.CancelFunc
	seq    atomic.Int64 // höchstes verrechnetes seq dieser Verbindung (ack)
	prev   map[string]any // zuletzt gesendeter Zustand; nil = nächster Zustand ist ein snap (nur unter Raum-Sperre)

	mu     sync.Mutex
	room   *room.Room
	inRoom bool // ab Joined, bis Verlassen oder room_closed; steuert die Raumliste
}

func (c *conn) enqueue(v any) {
	b, err := json.Marshal(v)
	if err != nil {
		c.s.log.Error("Nachricht nicht kodierbar", "err", err)
		return
	}
	select {
	case c.send <- out{data: b}:
	default:
		c.s.log.Warn("Sendepuffer voll, Verbindung zu", "device", c.device)
		c.cancel()
	}
}

func (c *conn) Joined(code, name string, you []room.Seat) {
	c.mu.Lock()
	c.inRoom = true
	c.mu.Unlock()
	c.enqueue(joinedMsg{"joined", code, name, you})
}

// Level: nach dem Level kommt immer ein voller Zustand (Beitreten, Wiederverbinden, Stufenwechsel).
func (c *conn) Level(depth int, layout level.Layout) {
	c.prev = nil
	c.enqueue(levelMsg{"level", depth, layout})
}

// State schickt snap nach Level, sonst delta zum zuletzt gesendeten Zustand.
func (c *conn) State(tick int, w *sim.World) {
	s := stateOf(w)
	msg := stateMsg{"snap", tick, c.seq.Load(), s}
	if c.prev != nil {
		msg.T, msg.S = "delta", deltaOf(c.prev, s)
	}
	c.prev = s
	c.enqueue(msg)
}

func (c *conn) Seats(you []room.Seat, monarchs []string) {
	c.enqueue(seatsMsg{"seats", you, monarchs})
}

// Replaced: Fehler schicken und schließen, das Gerät verbindet sich nicht automatisch neu.
func (c *conn) Replaced() {
	c.leaveRoom()
	c.enqueue(errMsg(codeReplaced))
	c.closeAfterSend(websocket.StatusPolicyViolation, codeReplaced)
}

// closeAfterSend schließt die Verbindung, sobald alles davor gesendet ist.
func (c *conn) closeAfterSend(code websocket.StatusCode, reason string) {
	select {
	case c.send <- out{close: code, reason: reason}:
	default:
		c.cancel() // Puffer voll: sofort schließen
	}
}

// Closed: Raum abgestürzt oder Server fährt herunter; das Gerät ist danach in keinem Raum. Beim Herunterfahren
// (final) schließt der Server die Verbindung nach dem Fehler.
func (c *conn) Closed(final bool) {
	c.leaveRoom()
	c.enqueue(errMsg(codeRoomClosed))
	if final {
		c.closeAfterSend(websocket.StatusGoingAway, codeRoomClosed)
	}
}

func (c *conn) leaveRoom() {
	c.mu.Lock()
	c.room, c.inRoom = nil, false
	c.mu.Unlock()
}

func (c *conn) current() *room.Room {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.room
}

func (c *conn) writer(ctx context.Context) {
	for {
		select {
		case <-ctx.Done():
			return
		case o := <-c.send:
			if o.data == nil {
				_ = c.ws.Close(o.close, o.reason)
				c.cancel()
				return
			}
			if err := c.ws.Write(ctx, websocket.MessageText, o.data); err != nil {
				c.cancel()
				return
			}
		}
	}
}

// websocket ist GET /ws.
func (s *server) websocket(w http.ResponseWriter, r *http.Request) {
	ws, err := websocket.Accept(w, r, nil)
	if err != nil {
		return
	}
	ws.SetReadLimit(maxMsgBytes)
	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()
	device, ok := handshake(ctx, ws)
	if !ok {
		b, _ := json.Marshal(errMsg(codeVersion))
		_ = ws.Write(ctx, websocket.MessageText, b)
		_ = ws.Close(websocket.StatusPolicyViolation, codeVersion)
		return
	}
	c := &conn{s: s, ws: ws, device: device, send: make(chan out, sendBuffer), cancel: cancel}
	go c.writer(ctx)
	c.enqueue(welcome())
	s.track(c, true)
	defer func() {
		s.track(c, false)
		if r := c.current(); r != nil {
			r.Drop(c.device, c)
		}
		_ = ws.CloseNow()
	}()
	c.enqueue(roomsMsg{"rooms", s.cfg.Rooms.Rooms()})
	for {
		_, data, err := ws.Read(ctx)
		if err != nil {
			return
		}
		c.handle(data)
	}
}

// handshake: Die erste Nachricht muss ein gültiges hello mit v 2 und einer Geräte-ID sein.
func handshake(ctx context.Context, ws *websocket.Conn) (string, bool) {
	_, data, err := ws.Read(ctx)
	var m inMsg
	if err != nil || json.Unmarshal(data, &m) != nil {
		return "", false
	}
	ok := m.T == "hello" && m.V == ProtocolVersion && m.Device != "" && len(m.Device) <= maxDeviceLen
	return m.Device, ok
}

// track hält die Verbindungen für die Raumliste.
func (s *server) track(c *conn, add bool) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if add {
		s.conns[c] = true
	} else {
		delete(s.conns, c)
	}
}

// broadcastRooms schickt die Raumliste an alle Geräte ohne Raum (room.Manager.Changed).
func (s *server) broadcastRooms() {
	msg := roomsMsg{"rooms", s.cfg.Rooms.Rooms()}
	s.mu.Lock()
	defer s.mu.Unlock()
	for c := range s.conns {
		c.mu.Lock()
		in := c.inRoom
		c.mu.Unlock()
		if !in {
			c.enqueue(msg)
		}
	}
}
