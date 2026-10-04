package main

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"sync"
	"time"

	"github.com/coder/websocket"

	k3cnet "k3c/engine/net"
	"k3c/engine/rng"
)

// setupTimeout begrenzt Verbinden, Handschlag und Beitreten je Bot.
const setupTimeout = 10 * time.Second

// Nachrichten nach docs/protocol.md (hello, create, join, input, leave); die Typen in engine/net sind nicht exportiert.
type (
	outMsg struct {
		T      string `json:"t"`
		V      int    `json:"v,omitempty"`
		Device string `json:"device,omitempty"`
		Save   string `json:"save,omitempty"`
		Fresh  *bool  `json:"fresh,omitempty"`
		Room   string `json:"room,omitempty"`
		Slots  []int  `json:"slots,omitempty"`
		Seq    int64  `json:"seq,omitempty"`
		P      []pad  `json:"p,omitempty"`
	}
	pad struct {
		Slot   int     `json:"slot"`
		MoveX  float64 `json:"moveX"`
		Sprint bool    `json:"sprint"`
		Pay    bool    `json:"pay"`
	}
	inMsg struct {
		T       string `json:"t"`
		V       int    `json:"v"`
		Room    string `json:"room"`
		Tick    int    `json:"tick"`
		Code    string `json:"code"`
		Message string `json:"message"`
	}
)

// bot ist ein Gerät mit einem Spieler (Slot 0).
type bot struct {
	device string
	ws     *websocket.Conn
	sent   func(string, []byte)
	script script
	seq    int64
	run    *roomRun // nur beim ersten Bot eines Raums: sammelt die Tick-Reihe
	start  time.Time
	done   sync.WaitGroup
}

// startRooms verbindet alle Bots: der erste je Raum erstellt ihn, die anderen treten per Code bei. Bei einem Fehler
// liefert es die bis dahin verbundenen Bots, damit der Aufrufer sie trennt.
func startRooms(ctx context.Context, c config) ([]*roomRun, []*bot, error) {
	var runs []*roomRun
	var bots []*bot
	for i := range c.rooms {
		r := &roomRun{Name: fmt.Sprintf("%s%d", roomPrefix(c), i)}
		runs = append(runs, r)
		for j := range c.players {
			b, err := connect(ctx, c, i, j)
			if err != nil {
				return runs, bots, err
			}
			bots = append(bots, b)
			if err := b.enter(ctx, r, j == 0); err != nil {
				return runs, bots, failf("%s, Bot %d: %v", r.Name, j, err)
			}
			if j == 0 {
				b.run, b.start = r, time.Now()
			}
			b.done.Add(1)
			go b.play()
		}
	}
	return runs, bots, nil
}

// connect öffnet /ws und schickt den Handschlag mit der Protokollversion aus engine/net.
func connect(ctx context.Context, c config, room, n int) (*bot, error) {
	ctx, cancel := context.WithTimeout(ctx, setupTimeout)
	defer cancel()
	ws, _, err := websocket.Dial(ctx, wsURL(c.url), nil)
	if err != nil {
		return nil, failf("WebSocket nicht erreichbar unter %s", wsURL(c.url))
	}
	ws.SetReadLimit(8 << 20) // snap mit Level ist groß
	b := &bot{device: fmt.Sprintf("load-%s-%d-%d", c.tag, room, n), ws: ws, sent: c.sent,
		script: script{r: rng.New(fmt.Sprintf("%s/%d/%d", c.seed, room, n))}}
	err = b.send(ctx, outMsg{T: "hello", V: k3cnet.ProtocolVersion, Device: b.device})
	if err == nil {
		_, err = b.await(ctx, "welcome")
	}
	if err != nil {
		_ = ws.CloseNow()
		return nil, err
	}
	return b, nil
}

// enter erstellt den Raum (erster Bot, neuer Spielstand) oder tritt ihm per Code bei und wartet auf joined.
func (b *bot) enter(ctx context.Context, r *roomRun, first bool) error {
	ctx, cancel := context.WithTimeout(ctx, setupTimeout)
	defer cancel()
	fresh := true
	m := outMsg{T: "join", Room: r.Code, Slots: []int{0}}
	if first {
		m = outMsg{T: "create", Save: r.Name, Fresh: &fresh, Slots: []int{0}}
	}
	if err := b.send(ctx, m); err != nil {
		return err
	}
	joined, err := b.await(ctx, "joined")
	if err != nil {
		return err
	}
	r.Code = joined.Room
	return nil
}

// await liest bis zur Nachricht vom Typ want; eine Fehler-Nachricht des Servers bricht ab.
func (b *bot) await(ctx context.Context, want string) (inMsg, error) {
	for {
		_, data, err := b.ws.Read(ctx)
		if err != nil {
			return inMsg{}, failf("Verbindung beim Warten auf %s verloren", want)
		}
		var m inMsg
		if err := json.Unmarshal(data, &m); err != nil {
			return m, failf("Nachricht nicht lesbar beim Warten auf %s", want)
		}
		switch m.T {
		case want:
			return m, nil
		case "error":
			return m, failf("Server lehnt ab: %s (%s)", m.Code, m.Message)
		}
	}
}

func (b *bot) send(ctx context.Context, m outMsg) error {
	data, _ := json.Marshal(m)
	if b.sent != nil {
		b.sent(b.device, data)
	}
	if err := b.ws.Write(ctx, websocket.MessageText, data); err != nil {
		return failf("Senden an den Server gescheitert (%s)", m.T)
	}
	return nil
}

// play antwortet auf jeden Zustand (snap, delta) mit der nächsten Eingabe: Takt des Servers, nicht der Wanduhr. Endet,
// wenn die Verbindung zu ist.
func (b *bot) play() {
	defer b.done.Done()
	ctx := context.Background()
	for {
		_, data, err := b.ws.Read(ctx)
		if err != nil {
			return
		}
		var m inMsg
		if json.Unmarshal(data, &m) != nil {
			continue
		}
		switch m.T {
		case "snap", "delta":
			if b.run != nil {
				b.run.Ticks = append(b.run.Ticks, tickPoint{time.Since(b.start), m.Tick})
			}
			b.seq++
			if b.send(ctx, outMsg{T: "input", Seq: b.seq, P: []pad{b.script.next()}}) != nil {
				return
			}
		case "error":
			if b.run != nil {
				b.run.Errors++
			}
		}
	}
}

// stop verlässt den Raum (Monarch sofort frei), schließt die Verbindung und wartet auf play.
func (b *bot) stop() {
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()
	_ = b.send(ctx, outMsg{T: "leave"})
	_ = b.ws.Close(websocket.StatusNormalClosure, "Lauf beendet")
	b.done.Wait()
}

// script erzeugt die Eingaben eines Bots aus seinem Seed: Abschnitte von 10 bis 60 Ticks, darin stehen und Münzen geben
// (2 von 10), laufen in zufälliger Richtung (6 von 10) oder sprinten (2 von 10).
type script struct {
	r    *rng.Rng
	left int
	cur  pad
}

func (s *script) next() pad {
	if s.left == 0 {
		s.left = s.r.Int(10, 60)
		kind, dir := s.r.Int(0, 9), float64(s.r.Int(0, 1)*2-1)
		s.cur = pad{MoveX: dir, Sprint: kind >= 8}
		if kind < 2 {
			s.cur = pad{Pay: true}
		}
	}
	s.left--
	return s.cur
}

// wsURL macht aus http(s)://host den Weg ws(s)://host/ws.
func wsURL(base string) string {
	return strings.Replace(strings.Replace(base, "https://", "wss://", 1), "http://", "ws://", 1) + "/ws"
}
