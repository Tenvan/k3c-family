package net

import (
	"context"
	"encoding/json"
	"net/http"
	"strconv"
	"sync"
	"sync/atomic"
	"time"

	"github.com/coder/websocket"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
)

// WebSocket /ws nach Protokoll v4. Jede Verbindung hat eine Lese-Schleife (diese Goroutine) und eine
// Schreib-Goroutine mit gepuffertem Kanal. Ist der Puffer voll, wird die Verbindung geschlossen, das zählt als Abbruch.

const (
	sendBuffer   = 64
	maxMsgBytes  = 16 << 10
	maxDeviceLen = 64
	helloTimeout = 10 * time.Second
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
		c.s.log.Error("💥 Nachricht nicht kodierbar", "err", err)
		return
	}
	select {
	case c.send <- out{data: b}:
	default:
		c.s.log.Warn("🐢 Sendepuffer voll, Verbindung zu", "device", c.device)
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
func (c *conn) State(tick int, w *sim.World, timescale int, paused bool) {
	s := stateOf(w, timescale, paused)
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

// current ist der Raum des Geräts; ein inzwischen geschlossener Raum zählt als keiner.
func (c *conn) current() *room.Room {
	c.mu.Lock()
	r := c.room
	c.mu.Unlock()
	if r != nil && r.Closed() {
		c.leaveRoom()
		return nil
	}
	return r
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
	if s.cfg.Conns != nil {
		s.cfg.Conns.Add(1)
		defer s.cfg.Conns.Done()
	}
	ws, err := websocket.Accept(w, r, nil)
	if err != nil {
		return
	}
	ws.SetReadLimit(maxMsgBytes)
	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()
	device, ok := handshake(ctx, ws)
	if !ok {
		s.log.Warn("🚫 WebSocket: Handschlag abgelehnt", "ns", "ws", "remote", r.RemoteAddr, "erwartet", ProtocolVersion, "userAgent", r.UserAgent())
		b, _ := json.Marshal(errMsg(codeVersion))
		_ = ws.Write(ctx, websocket.MessageText, b)
		_ = ws.Close(websocket.StatusPolicyViolation, codeVersion)
		return
	}
	c := &conn{s: s, ws: ws, device: device, send: make(chan out, sendBuffer), cancel: cancel}
	go c.writer(ctx)
	c.enqueue(welcome())
	s.track(c, true)
	began := time.Now()
	s.log.Info("🔌 WebSocket: Gerät verbunden", "ns", "ws", "device", short(device), "remote", r.RemoteAddr, "verbindungen", s.openConns())
	var readErr error
	defer func() {
		s.track(c, false)
		room := ""
		if rm := c.current(); rm != nil {
			room = rm.Code
			rm.Drop(c.device, c)
		}
		s.log.Info("👋 WebSocket: Gerät getrennt", "ns", "ws", "device", short(device), "raum", room, "dauerS", int(time.Since(began).Seconds()),
			"grund", closeReason(readErr), "verbindungen", s.openConns())
		_ = ws.CloseNow()
	}()
	c.enqueue(roomsMsg{"rooms", s.cfg.Rooms.Rooms()})
	for {
		_, data, err := ws.Read(ctx)
		if err != nil {
			readErr = err
			return
		}
		c.handle(data)
	}
}

// handshake: Die erste Nachricht muss ein gültiges hello mit v 2 und einer Geräte-ID sein.
func handshake(ctx context.Context, ws *websocket.Conn) (string, bool) {
	ctx, cancel := context.WithTimeout(ctx, helloTimeout)
	defer cancel()
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

// short kürzt eine Geräte-ID für das Log.
func short(id string) string {
	if len(id) > 8 {
		return id[:8]
	}
	return id
}

// closeReason nennt, warum die Lese-Schleife endete: Statuscode der Gegenstelle, sonst der Fehlertext.
func closeReason(err error) string {
	if err == nil {
		return "geschlossen"
	}
	if code := websocket.CloseStatus(err); code != -1 {
		return "close " + strconv.Itoa(int(code))
	}
	return err.Error()
}

// openConns zählt die offenen Verbindungen.
func (s *server) openConns() int {
	s.mu.Lock()
	defer s.mu.Unlock()
	return len(s.conns)
}
