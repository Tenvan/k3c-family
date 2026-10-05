package net

import (
	"context"
	"encoding/json"
	"fmt"
	"maps"
	"net/http"
	"strconv"
	"sync"
	"sync/atomic"
	"time"

	"github.com/coder/websocket"

	"k3c/engine/level"
	"k3c/engine/room"
)

// WebSocket /ws nach Protokoll v4. Jede Verbindung hat eine Lese-Schleife (diese Goroutine) und eine
// Schreib-Goroutine mit Warteschlange. Ein Zustand ersetzt einen noch wartenden derselben Stufe am Ende der Warteschlange
// (nur der neueste zählt, B-276; je Stufe ein Strom, B-176); Delta und JSON des Zustands baut erst die Schreib-Goroutine. Ist die Warteschlange voll, wird die
// Verbindung geschlossen, das zählt als Abbruch.

const (
	sendBuffer   = 64
	maxMsgBytes  = 16 << 10
	maxDeviceLen = 64
	helloTimeout = 10 * time.Second
	maxEvents    = 256 // Ereignisse verworfener Zustände, die mit dem neuesten nachgeliefert werden
)

// out ist eine Nachricht in der Warteschlange: fertige Bytes (data), ein Zustand (state) oder mit close das Schließen.
type out struct {
	data   []byte
	level  bool           // Level: der nächste Zustand dieser Stufe ist ein snap
	stage  int            // Stufe von Level und Zustand
	state  map[string]any // aus stateOf, unveränderlich und mit den anderen Geräten der Stufe geteilt
	tick   int
	ack    int64 // höchstes verrechnetes seq beim Aufbau des Zustands (unter der Raum-Sperre)
	events []any // nicht nil: Ereignisse verworfener Zustände samt denen von state
	close  websocket.StatusCode
	reason string
}

// conn ist ein Gerät; es setzt room.Peer um. Die Peer-Methoden laufen unter der Sperre des Raums und schreiben nur
// in die Warteschlange.
type conn struct {
	s      *server
	ws     *websocket.Conn
	device string
	cancel context.CancelFunc
	seq    atomic.Int64           // höchstes verrechnetes seq dieser Verbindung (ack)
	prev   map[int]map[string]any // je Stufe der zuletzt gesendete Zustand; fehlt = nächster ist ein snap (nur Schreib-Goroutine)

	qmu    sync.Mutex
	queue  []out
	warned bool          // verworfene Ereignisse schon gemeldet (einmal je Verbindung)
	drops  int           // ersetzte (verworfene) Zustände seit dem letzten Punkt der Messreihe (ping.go)
	wake   chan struct{} // Länge 1: weckt die Schreib-Goroutine

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
	c.push(out{data: b})
}

// push stellt o hinten an; ein Zustand ersetzt einen wartenden derselben Stufe (waiting) und übernimmt dessen Ereignisse.
func (c *conn) push(o out) {
	c.qmu.Lock()
	n, lost, i := len(c.queue), 0, c.waiting(o)
	switch {
	case i >= 0:
		o.events, lost = mergeEvents(c.queue[i], o.state)
		c.queue[i] = o
		c.drops++
		if lost > 0 && c.warned {
			lost = 0
		}
		c.warned = c.warned || lost > 0
	case n >= sendBuffer:
		c.qmu.Unlock()
		c.s.log.Warn("🐢 Warteschlange voll, Verbindung zu", "ns", "ws", "device", short(c.device))
		c.cancel()
		return
	default:
		c.queue = append(c.queue, o)
	}
	c.qmu.Unlock()
	if lost > 0 {
		c.s.log.Warn("🐢 Ereignisse verworfen", "ns", "ws", "device", short(c.device), "anzahl", lost)
	}
	select {
	case c.wake <- struct{}{}:
	default:
	}
}

// waiting ist der Platz des wartenden Zustands derselben Stufe, den der Zustand o ersetzt: gesucht nur in den Zuständen
// am Ende der Warteschlange, damit er kein Level und keine andere Nachricht überholt; -1: keiner.
func (c *conn) waiting(o out) int {
	for i := len(c.queue) - 1; o.state != nil && i >= 0 && c.queue[i].state != nil; i-- {
		if c.queue[i].stage == o.stage {
			return i
		}
	}
	return -1
}

// pop nimmt die vorderste Nachricht; false: Warteschlange leer.
func (c *conn) pop() (out, bool) {
	c.qmu.Lock()
	defer c.qmu.Unlock()
	if len(c.queue) == 0 {
		return out{}, false
	}
	o := c.queue[0]
	c.queue[0] = out{}
	c.queue = c.queue[1:]
	return o, true
}

// mergeEvents sind die Ereignisse des verworfenen Zustands old und des neuen cur, höchstens maxEvents (die neuesten),
// und die Zahl der dabei verworfenen; nil, wenn old keine hatte.
func mergeEvents(old out, cur map[string]any) ([]any, int) {
	prev := old.events
	if prev == nil {
		prev, _ = old.state["events"].([]any)
	}
	if len(prev) == 0 {
		return nil, 0
	}
	now, _ := cur["events"].([]any)
	all := append(append(make([]any, 0, len(prev)+len(now)), prev...), now...)
	lost := max(0, len(all)-maxEvents)
	return all[lost:], lost
}

func (c *conn) Joined(code, name string, you []room.Seat) {
	c.mu.Lock()
	c.inRoom = true
	c.mu.Unlock()
	c.enqueue(joinedMsg{"joined", code, name, you})
}

// Level: nach dem Level kommt immer ein voller Zustand der Stufe (Beitreten, Wiederverbinden, neue Stufe).
func (c *conn) Level(stage, depth int, layout level.Layout) {
	b, err := json.Marshal(levelMsg{"level", stage, depth, layout})
	if err != nil {
		c.s.log.Error("💥 Nachricht nicht kodierbar", "err", err)
		return
	}
	c.push(out{data: b, level: true, stage: stage})
}

// State stellt den Zustand (aus stateOf über room.Manager.Snapshot) in die Warteschlange; snap oder delta entscheidet
// die Schreib-Goroutine (stateData). ack wird hier gelesen, damit es nur seqs bestätigt, die im Zustand stecken.
func (c *conn) State(tick, stage int, state any) {
	if s, ok := state.(map[string]any); ok {
		c.push(out{state: s, tick: tick, stage: stage, ack: c.seq.Load()})
	}
}

// stateData kodiert einen Zustand: snap nach Level, sonst delta zum zuletzt gesendeten derselben Stufe; nil: nicht kodierbar.
func (c *conn) stateData(o out) []byte {
	msg := stateMsg{"snap", o.stage, o.tick, o.ack, o.state}
	if prev := c.prev[o.stage]; prev != nil {
		msg.T, msg.S = "delta", deltaOf(prev, o.state)
	} else if len(o.events) > 0 {
		msg.S = maps.Clone(o.state) // geteilt: nicht ändern
	}
	if len(o.events) > 0 {
		msg.S["events"] = o.events
	}
	b, err := json.Marshal(msg)
	if err != nil {
		c.s.log.Error("💥 Zustand nicht kodierbar", "ns", "ws", "err", err)
		return nil
	}
	if c.prev == nil {
		c.prev = map[int]map[string]any{}
	}
	c.prev[o.stage] = o.state
	return b
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
	c.push(out{close: code, reason: reason}) // Warteschlange voll: sofort zu
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

// writer sendet die Warteschlange. Ein Panic schließt nur diese Verbindung, das Gerät verbindet sich neu.
func (c *conn) writer(ctx context.Context) {
	defer func() {
		if p := recover(); p != nil {
			c.s.log.Error("💥 Schreib-Goroutine abgestürzt, Verbindung zu", "ns", "ws", "device", short(c.device), "err", fmt.Sprint(p))
			c.cancel()
		}
	}()
	for {
		select {
		case <-ctx.Done():
			return
		case <-c.wake:
		}
		for o, ok := c.pop(); ok; o, ok = c.pop() {
			if !c.write(ctx, o) {
				c.cancel()
				return
			}
		}
	}
}

// write sendet eine Nachricht; false: Die Verbindung ist zu.
func (c *conn) write(ctx context.Context, o out) bool {
	if o.close != 0 {
		_ = c.ws.Close(o.close, o.reason)
		return false
	}
	data := c.encode(o)
	if data == nil {
		return true
	}
	return c.ws.Write(ctx, websocket.MessageText, data) == nil
}

// encode sind die Bytes einer Nachricht (ein Zustand als snap oder delta); ein Level setzt prev seiner Stufe zurück.
// nil: nichts senden.
func (c *conn) encode(o out) []byte {
	data := o.data
	if o.state != nil {
		data = c.stateData(o)
	}
	if o.level {
		delete(c.prev, o.stage)
	}
	return data
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
	c := &conn{s: s, ws: ws, device: device, wake: make(chan struct{}, 1), cancel: cancel}
	go c.writer(ctx)
	go c.pinger(ctx, pingEvery)
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
