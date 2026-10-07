package mcpsrv

import (
	"context"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
)

// sim_test online headless (TR1.2): Bot-Geräte gegen den echten Handler im Testprozess (nur Loopback).

func gameServer(t *testing.T, token string) *httptest.Server {
	t.Helper()
	dir := t.TempDir()
	saves := &store.Saves{Dir: filepath.Join(dir, "saves")}
	m := room.NewManager(saves)
	ctx, cancel := context.WithCancel(context.Background())
	go m.Run(ctx)
	srv := httptest.NewServer(k3cnet.NewHandler(k3cnet.Config{Rooms: m, StatusToken: token, StartedAt: time.Now(),
		Saves: saves, Reports: &store.Reports{Dir: filepath.Join(dir, "reports")}}))
	t.Cleanup(func() { srv.Close(); cancel(); m.Close() })
	return srv
}

func TestSimTestOnlineHeadless(t *testing.T) {
	srv := gameServer(t, "geheim-1234")
	t.Setenv("K3C_SERVER_URL", srv.URL)
	t.Setenv("K3C_STATUS_TOKEN", "geheim-1234")
	old := onlineSample
	onlineSample = 300 * time.Millisecond
	t.Cleanup(func() { onlineSample = old })
	s := New(Config{Version: "test", Root: t.TempDir()})
	id := start(t, s, simTestIn{Mode: "online", Rooms: 2, Players: 2, Duration: "3s", Focus: []string{"perf", "stability", "balance"}})
	out := waitDone(t, s, id)
	t.Log("\n" + out)
	for _, want := range []string{"✅ " + id, "online·headless·perf,stability,balance", "2 Räume × 2 Bots (saver)",
		"Tick p99 max", "Monarchen bewegt 4/4", "Trennungen 0, Fehler 0, neue Server-Fehler 0", "Ereignisse:"} {
		if !strings.Contains(out, want) {
			t.Errorf("status ohne %q:\n%s", want, out)
		}
	}
	if strings.Contains(out, "stability)") || strings.Contains(out, ", stability") {
		t.Errorf("Stabilität verfehlt:\n%s", out)
	}
	if n := len(strings.Split(out, "\n")); n > maxStatusLines {
		t.Errorf("%d Zeilen", n)
	}
}

func TestSimTestOnlineOhneServer(t *testing.T) {
	t.Setenv("K3C_SERVER_URL", "http://127.0.0.1:1")
	s := New(Config{Version: "test", Root: t.TempDir()})
	_, err := s.simTest(context.Background(), simTestIn{Action: "start", Mode: "online"})
	if err == nil || !strings.Contains(err.Error(), "svc_start") {
		t.Errorf("Fehler %v, erwartet Hinweis auf svc_start", err)
	}
}
