package mcpsrv

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/coder/websocket"

	k3cnet "k3c/engine/net"
	"k3c/tools/k3c-dev/internal/botfeed"
)

// sim_test online mit Clients (TR1.3): ein Fake-Client in Go statt Browser. Er tritt dem Raum aus ?room= mit
// ?players= Slots bei, liest den Bot-Feed aus ?botfeed= und schreibt Diagnose-Zeilen ins Client-Log.

// fakeClients zählt die Kommandos je Raum und Slot.
type fakeClients struct {
	game string // ws://… des Spielservers
	log  string // logs/k3c-client.jsonl
	mu   sync.Mutex
	got  map[string]map[int]int
}

func (f *fakeClients) launch(_ context.Context, _, _, raw string) (func(), error) {
	u, err := url.Parse(raw)
	if err != nil {
		return nil, err
	}
	q := u.Query()
	room, feed := q.Get("room"), q.Get("botfeed")
	players, _ := strconv.Atoi(q.Get("players"))
	if !strings.HasPrefix(feed, "ws://127.0.0.1:") || players < 1 || len(room) != 4 {
		return nil, fmt.Errorf("URL %s unvollständig", raw)
	}
	ctx, cancel := context.WithCancel(context.Background())
	g, err := f.join(ctx, room, players)
	if err != nil {
		cancel()
		return nil, err
	}
	fc, _, err := websocket.Dial(ctx, feed, nil)
	if err != nil {
		cancel()
		return nil, err
	}
	f.mu.Lock()
	f.got[room] = map[int]int{}
	f.mu.Unlock()
	go f.read(ctx, fc, room)
	f.diag(raw, 58, 40)
	f.diag(raw, 61, 25)
	return func() { cancel(); _ = fc.CloseNow(); _ = g.CloseNow() }, nil
}

// join meldet das Gerät am Spielserver an und tritt mit players Slots bei; danach liest es nur noch mit.
func (f *fakeClients) join(ctx context.Context, room string, players int) (*websocket.Conn, error) {
	c, _, err := websocket.Dial(ctx, f.game, nil)
	if err != nil {
		return nil, err
	}
	c.SetReadLimit(8 << 20)
	slots := make([]int, players)
	for i := range slots {
		slots[i] = i
	}
	for _, m := range []any{map[string]any{"t": "hello", "v": k3cnet.ProtocolVersion, "device": "fake-" + room},
		map[string]any{"t": "join", "room": room, "slots": slots}} {
		data, _ := json.Marshal(m)
		if err := c.Write(ctx, websocket.MessageText, data); err != nil {
			return nil, err
		}
	}
	go func() {
		for {
			if _, _, err := c.Read(ctx); err != nil {
				return
			}
		}
	}()
	return c, nil
}

func (f *fakeClients) read(ctx context.Context, c *websocket.Conn, room string) {
	for {
		_, data, err := c.Read(ctx)
		if err != nil {
			return
		}
		var m botfeed.Cmd
		if json.Unmarshal(data, &m) == nil {
			f.mu.Lock()
			f.got[room][m.Slot]++
			f.mu.Unlock()
		}
	}
}

// diag schreibt eine Diagnose-Zeile wie der Spielserver aus /api/clientlog.
func (f *fakeClients) diag(page string, fps, lat int) {
	f.mu.Lock()
	defer f.mu.Unlock()
	line, _ := json.Marshal(map[string]any{"time": time.Now().Format(time.RFC3339Nano), "level": "INFO", "msg": "📈 Diagnose",
		"ns": "client", "url": page, "ctx": fmt.Sprintf(`{"fps":%d,"latencyMs":%d,"bufferMs":80}`, fps, lat)})
	file, err := os.OpenFile(f.log, os.O_CREATE|os.O_APPEND|os.O_WRONLY, 0o644)
	if err == nil {
		_, _ = file.Write(append(line, '\n'))
		_ = file.Close()
	}
}

// clientsServer: Spielserver, Fake-Vite und eine gestartete Workbench mit Fake-Clients.
func clientsServer(t *testing.T) (*Server, *fakeClients) {
	t.Helper()
	srv := gameServer(t, "geheim-1234")
	t.Setenv("K3C_SERVER_URL", srv.URL)
	t.Setenv("K3C_STATUS_TOKEN", "geheim-1234")
	vite := httptest.NewServer(http.NotFoundHandler())
	t.Cleanup(vite.Close)
	t.Setenv("K3C_VITE_PORT", vite.URL[strings.LastIndex(vite.URL, ":")+1:])
	root := t.TempDir()
	if err := os.MkdirAll(filepath.Join(root, "logs"), 0o755); err != nil {
		t.Fatal(err)
	}
	f := &fakeClients{game: "ws" + strings.TrimPrefix(srv.URL, "http") + "/ws",
		log: filepath.Join(root, "logs", "k3c-client.jsonl"), got: map[string]map[int]int{}}
	s := New(Config{Version: "test", Root: root})
	s.findBrowser = func() (string, error) { return "edge.exe", nil }
	s.launch = f.launch
	if err := s.Start(); err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { _ = s.Stop() })
	return s, f
}

func TestSimTestClientsFakeClient(t *testing.T) {
	s, f := clientsServer(t)
	oldSample, oldTick := onlineSample, clientTick
	onlineSample, clientTick = 300*time.Millisecond, 50*time.Millisecond
	t.Cleanup(func() { onlineSample, clientTick = oldSample, oldTick })
	id := start(t, s, simTestIn{Mode: "online", Clients: 2, Players: 2, Duration: "3s", Focus: []string{"perf", "stability"}})
	out := waitDone(t, s, id)
	t.Log("\n" + out)
	for _, want := range []string{"✅ " + id, "online·2 Clients·perf,stability", "2 Clients × 2 Spieler (saver), gestartet",
		"Feed: verbunden 2/2, nie verbunden 0", "Client-FPS min 58, Mittel 60; Latenz max 40 ms, Puffer max 80 ms (4 Diagnose-Zeilen)",
		"Client-Fehler 0", "Trennungen 0, Fehler 0"} {
		if !strings.Contains(out, want) {
			t.Errorf("status ohne %q", want)
		}
	}
	if strings.Contains(out, "stability)") || len(strings.Split(out, "\n")) > maxStatusLines {
		t.Errorf("Stabilität verfehlt oder zu lang:\n%s", out)
	}
	f.mu.Lock()
	defer f.mu.Unlock()
	if len(f.got) != 2 {
		t.Fatalf("Räume %v, erwartet 2", f.got)
	}
	for room, slots := range f.got {
		if slots[0] == 0 || slots[1] == 0 || len(slots) != 2 {
			t.Errorf("Raum %s: Kommandos je Slot %v, erwartet Slot 0 und 1", room, slots)
		}
	}
}

func TestSimTestClientsAblehnung(t *testing.T) {
	s, _ := clientsServer(t)
	startErr := func(in simTestIn) string {
		in.Action, in.Mode = "start", "online"
		_, err := s.simTest(context.Background(), in)
		if err == nil {
			t.Fatalf("%+v nicht abgelehnt", in)
		}
		if strings.Contains(err.Error(), "\n") {
			t.Errorf("Grund über mehrere Zeilen: %q", err)
		}
		return err.Error()
	}
	cases := []struct {
		in   simTestIn
		want string
	}{
		{simTestIn{Clients: 1, Players: 4}, "players 1 bis 3 mit Clients"},
		{simTestIn{Clients: 2, Rooms: 2}, "rooms ergibt sich aus clients"},
		{simTestIn{Clients: 2, Attach: "ABCD"}, "attach nur mit mode online und clients 1"},
		{simTestIn{Clients: 1, Attach: "AB1"}, "attach: Raumcode aus 4 Buchstaben"},
	}
	for _, c := range cases {
		if got := startErr(c.in); !strings.Contains(got, c.want) {
			t.Errorf("%+v: %q, erwartet %q", c.in, got, c.want)
		}
	}
	s.findBrowser = func() (string, error) { return "", fmt.Errorf("kein Browser gefunden") }
	if got := startErr(simTestIn{Clients: 1}); !strings.Contains(got, "kein Browser") || !strings.Contains(got, "attach") {
		t.Errorf("ohne Browser: %q", got)
	}
	s.findBrowser = func() (string, error) { return "edge.exe", nil }
	t.Setenv("K3C_VITE_PORT", "1")
	if got := startErr(simTestIn{Clients: 1}); !strings.Contains(got, "Vite") || !strings.Contains(got, "svc_start") {
		t.Errorf("ohne Vite: %q", got)
	}
}
