// Package botdev ist ein Bot-Gerät für Testläufe (B-348, TR1.2): eine WebSocket-Verbindung zum Spielserver mit einem
// Slot, dessen Monarchen ein Bot des Balancing-Testers steuert. Der Bot entscheidet auf dem Zustand, den der Server
// schickt, und antwortet mit `input` wie ein Client.
package botdev

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"sync"
	"time"

	"github.com/coder/websocket"

	k3cnet "k3c/engine/net"
	"k3c/engine/sim"
	"k3c/tools/k3c-dev/internal/balance"
)

const setupTimeout = 10 * time.Second

// Stats sind die Zähler eines Geräts.
type Stats struct {
	States, Inputs, Errors int
	Disconnected           bool           // Verbindung ohne Stop verloren
	Moved                  bool           // der Monarch hat seine Position geändert
	Events                 map[string]int // Ereignisse der eigenen Stufe nach Typ
}

// Device ist ein Bot-Gerät; Felder hinter mu ändert nur play.
type Device struct {
	name    string
	ws      *websocket.Conn
	bot     balance.Bot
	monarch int
	seq     int64
	done    chan struct{}

	mu      sync.Mutex
	stage   int
	states  map[int]map[string]any
	firstX  *float64
	stopped bool
	stats   Stats
}

type inMsg struct {
	T       string          `json:"t"`
	Room    string          `json:"room"`
	Stage   int             `json:"stage"`
	S       json.RawMessage `json:"s"`
	Code    string          `json:"code"`
	Message string          `json:"message"`
	You     []struct {
		Slot, Monarch, Stage int
	} `json:"you"`
}

// Dial verbindet zu base (http://host:port) und schickt den Handschlag.
func Dial(ctx context.Context, base, name string, bot balance.Bot) (*Device, error) {
	ctx, cancel := context.WithTimeout(ctx, setupTimeout)
	defer cancel()
	url := strings.Replace(strings.Replace(base, "https://", "wss://", 1), "http://", "ws://", 1) + "/ws"
	ws, _, err := websocket.Dial(ctx, url, nil)
	if err != nil {
		return nil, fmt.Errorf("WebSocket nicht erreichbar unter %s", url)
	}
	ws.SetReadLimit(8 << 20) // snap mit Level ist groß
	d := &Device{name: name, ws: ws, bot: bot, done: make(chan struct{}), states: map[int]map[string]any{},
		stats: Stats{Events: map[string]int{}}}
	if err := d.send(ctx, map[string]any{"t": "hello", "v": k3cnet.ProtocolVersion, "device": name}); err != nil {
		return nil, err
	}
	if _, err := d.await(ctx, "welcome"); err != nil {
		_ = ws.CloseNow()
		return nil, err
	}
	return d, nil
}

// Create legt einen Raum mit neuem Spielstand an (Name mit Präfix test-, der Server räumt ihn auf) und liefert den Code.
func (d *Device) Create(ctx context.Context, save string) (string, error) {
	return d.enter(ctx, map[string]any{"t": "create", "save": save, "fresh": true, "slots": []int{0}})
}

// Join tritt dem Raum per Code bei.
func (d *Device) Join(ctx context.Context, code string) error {
	_, err := d.enter(ctx, map[string]any{"t": "join", "room": code, "slots": []int{0}})
	return err
}

func (d *Device) enter(ctx context.Context, msg map[string]any) (string, error) {
	ctx, cancel := context.WithTimeout(ctx, setupTimeout)
	defer cancel()
	if err := d.send(ctx, msg); err != nil {
		return "", err
	}
	m, err := d.await(ctx, "joined")
	if err != nil {
		return "", err
	}
	if len(m.You) > 0 {
		d.monarch, d.stage = m.You[0].Monarch, m.You[0].Stage
	}
	go d.play()
	return m.Room, nil
}

func (d *Device) await(ctx context.Context, want string) (inMsg, error) {
	for {
		_, data, err := d.ws.Read(ctx)
		if err != nil {
			return inMsg{}, fmt.Errorf("%s: Verbindung beim Warten auf %s verloren", d.name, want)
		}
		var m inMsg
		if json.Unmarshal(data, &m) != nil {
			continue
		}
		switch m.T {
		case want:
			return m, nil
		case "error":
			return m, fmt.Errorf("%s: Server lehnt ab: %s (%s)", d.name, m.Code, m.Message)
		}
	}
}

func (d *Device) send(ctx context.Context, msg any) error {
	data, _ := json.Marshal(msg)
	if err := d.ws.Write(ctx, websocket.MessageText, data); err != nil {
		return fmt.Errorf("%s: Senden gescheitert", d.name)
	}
	return nil
}

// play liest bis zum Ende der Verbindung und antwortet auf jeden Zustand der eigenen Stufe mit einer Eingabe.
func (d *Device) play() {
	defer close(d.done)
	ctx := context.Background()
	for {
		_, data, err := d.ws.Read(ctx)
		if err != nil {
			d.mu.Lock()
			d.stats.Disconnected = !d.stopped
			d.mu.Unlock()
			return
		}
		var m inMsg
		if json.Unmarshal(data, &m) != nil {
			continue
		}
		cmd, ok := d.handle(m)
		if !ok {
			continue
		}
		d.seq++
		pad := map[string]any{"slot": 0, "moveX": cmd.MoveX, "sprint": cmd.Sprint, "pay": cmd.Pay, "attack": cmd.Attack, "skill": cmd.Skill}
		if d.send(ctx, map[string]any{"t": "input", "seq": d.seq, "p": []any{pad}}) != nil {
			return
		}
		d.mu.Lock()
		d.stats.Inputs++
		d.mu.Unlock()
	}
}

// handle verarbeitet eine Nachricht; ok heißt: eine Eingabe ist fällig.
func (d *Device) handle(m inMsg) (sim.PlayerCommand, bool) {
	d.mu.Lock()
	defer d.mu.Unlock()
	switch m.T {
	case "error":
		d.stats.Errors++
	case "seats":
		if len(m.You) > 0 {
			d.stage = m.You[0].Stage
		}
	case "snap", "delta":
		return d.onState(m)
	}
	return sim.PlayerCommand{}, false
}

func (d *Device) onState(m inMsg) (sim.PlayerCommand, bool) {
	var s map[string]any
	if json.Unmarshal(m.S, &s) != nil {
		return sim.PlayerCommand{}, false
	}
	if m.T == "snap" {
		d.states[m.Stage] = s
	} else if st := d.states[m.Stage]; st != nil {
		applyDelta(st, s)
	}
	if m.Stage != d.stage || d.states[m.Stage] == nil {
		return sim.PlayerCommand{}, false
	}
	d.stats.States++
	for _, ev := range asList(s["events"]) {
		if t, ok := ev.(map[string]any)["type"].(string); ok {
			d.stats.Events[t]++
		}
	}
	w, err := worldOf(d.states[m.Stage])
	if err != nil {
		d.stats.Errors++
		return sim.PlayerCommand{}, false
	}
	for _, p := range w.Players {
		if p.Index == d.monarch {
			d.track(p.X)
			return d.bot(w, p), true
		}
	}
	return sim.PlayerCommand{}, false
}

func (d *Device) track(x float64) {
	if d.firstX == nil {
		d.firstX = &x
	}
	d.stats.Moved = d.stats.Moved || x != *d.firstX
}

// Stats liefert eine Kopie der Zähler.
func (d *Device) Stats() Stats {
	d.mu.Lock()
	defer d.mu.Unlock()
	s := d.stats
	s.Events = map[string]int{}
	for k, v := range d.stats.Events {
		s.Events[k] = v
	}
	return s
}

// Stop verlässt den Raum, schließt die Verbindung und wartet auf play.
func (d *Device) Stop() {
	d.mu.Lock()
	d.stopped = true
	d.mu.Unlock()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	_ = d.send(ctx, map[string]any{"t": "leave"})
	_ = d.ws.Close(websocket.StatusNormalClosure, "Testlauf beendet")
	select {
	case <-d.done:
	case <-time.After(3 * time.Second):
	}
}
