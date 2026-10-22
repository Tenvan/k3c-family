// Package botfeed ist der Bot-Feed der Workbench (B-348, TR1.3): ein WebSocket je Client eines Testlaufs unter
// /bot/<lauf>/<client>, über den die Workbench die Kommandos ihrer Bots an die lokalen Spieler des Clients schickt
// (B-349 › Notizen). Der Feed nimmt nur Verbindungen von Loopback an; was der Client schickt, wird verworfen.
package botfeed

import (
	"context"
	"encoding/json"
	"net"
	"net/http"
	"strings"
	"sync"
	"time"

	"github.com/coder/websocket"

	"k3c/engine/sim"
)

// Prefix ist der Pfad der Route; ein Feed liegt unter Prefix + "<lauf>/<client>".
const Prefix = "/bot/"

const writeTimeout = 2 * time.Second

// Cmd ist eine Feed-Nachricht: das Kommando eines Slots, Felder wie sim.PlayerCommand.
type Cmd struct {
	Slot   int     `json:"slot"`
	MoveX  float64 `json:"moveX"`
	Sprint bool    `json:"sprint"`
	Pay    bool    `json:"pay"`
	Attack bool    `json:"attack"`
	Skill  int     `json:"skill"`
}

// CmdOf baut die Nachricht eines Slots aus einem Bot-Kommando.
func CmdOf(slot int, c sim.PlayerCommand) Cmd {
	return Cmd{Slot: slot, MoveX: c.MoveX, Sprint: c.Sprint, Pay: c.Pay, Attack: c.Attack, Skill: c.Skill}
}

// Feed sind die Verbindungen eines Clients (nach einem Neuladen kurz zwei).
type Feed struct {
	mu    sync.Mutex
	conns map[*websocket.Conn]bool
	ever  bool // mindestens einmal verbunden
	sent  int
}

// Connected sagt, ob gerade ein Client verbunden ist und ob je einer da war.
func (f *Feed) Connected() (now, ever bool) {
	f.mu.Lock()
	defer f.mu.Unlock()
	return len(f.conns) > 0, f.ever
}

// Sent ist die Zahl der zugestellten Nachrichten.
func (f *Feed) Sent() int {
	f.mu.Lock()
	defer f.mu.Unlock()
	return f.sent
}

// Send schickt je Slot eine Nachricht an alle Verbindungen; eine Verbindung, die nicht annimmt, wird geschlossen.
func (f *Feed) Send(ctx context.Context, cmds []Cmd) {
	f.mu.Lock()
	conns := make([]*websocket.Conn, 0, len(f.conns))
	for c := range f.conns {
		conns = append(conns, c)
	}
	f.mu.Unlock()
	for _, c := range conns {
		for _, cmd := range cmds {
			data, _ := json.Marshal(cmd)
			wctx, cancel := context.WithTimeout(ctx, writeTimeout)
			err := c.Write(wctx, websocket.MessageText, data)
			cancel()
			if err != nil {
				_ = c.CloseNow()
				break
			}
			f.mu.Lock()
			f.sent++
			f.mu.Unlock()
		}
	}
}

func (f *Feed) add(c *websocket.Conn) {
	f.mu.Lock()
	defer f.mu.Unlock()
	f.conns[c], f.ever = true, true
}

func (f *Feed) remove(c *websocket.Conn) {
	f.mu.Lock()
	defer f.mu.Unlock()
	delete(f.conns, c)
}

// closeAll schließt alle Verbindungen; Close wartet auf den Client, daher außerhalb des Locks.
func (f *Feed) closeAll() {
	f.mu.Lock()
	conns := make([]*websocket.Conn, 0, len(f.conns))
	for c := range f.conns {
		conns = append(conns, c)
	}
	f.mu.Unlock()
	for _, c := range conns {
		_ = c.Close(websocket.StatusGoingAway, "Testlauf beendet")
	}
}

// Hub hält die Feeds aller laufenden Testläufe.
type Hub struct {
	mu    sync.Mutex
	feeds map[string]*Feed // "<lauf>/<client>"
}

// NewHub baut einen leeren Hub.
func NewHub() *Hub { return &Hub{feeds: map[string]*Feed{}} }

// Open legt den Feed unter name ("<lauf>/<client>") an.
func (h *Hub) Open(name string) *Feed {
	f := &Feed{conns: map[*websocket.Conn]bool{}}
	h.mu.Lock()
	defer h.mu.Unlock()
	h.feeds[name] = f
	return f
}

// Close schließt den Feed und seine Verbindungen.
func (h *Hub) Close(name string) {
	h.mu.Lock()
	f := h.feeds[name]
	delete(h.feeds, name)
	h.mu.Unlock()
	if f != nil {
		f.closeAll()
	}
}

func (h *Hub) feed(name string) *Feed {
	h.mu.Lock()
	defer h.mu.Unlock()
	return h.feeds[name]
}

// ServeHTTP nimmt einen Client an: nur von Loopback, nur für einen offenen Feed. Die Seite des Clients liegt auf
// einem anderen Port (Vite), daher sind Origins von localhost und 127.0.0.1 erlaubt.
func (h *Hub) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if !Loopback(r.RemoteAddr) {
		http.Error(w, "Bot-Feed nur über Loopback", http.StatusForbidden)
		return
	}
	f := h.feed(strings.TrimPrefix(r.URL.Path, Prefix))
	if f == nil {
		http.Error(w, "kein laufender Testlauf unter diesem Feed", http.StatusNotFound)
		return
	}
	c, err := websocket.Accept(w, r, &websocket.AcceptOptions{OriginPatterns: []string{"localhost:*", "127.0.0.1:*"}})
	if err != nil {
		return
	}
	f.add(c)
	defer f.remove(c)
	for { // Eingehendes verwerfen, bis die Verbindung endet
		if _, _, err := c.Read(context.Background()); err != nil {
			return
		}
	}
}

// Loopback sagt, ob eine Gegenstelle (host:port) auf diesem Rechner liegt.
func Loopback(remote string) bool {
	host, _, err := net.SplitHostPort(remote)
	if err != nil {
		return false
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}
