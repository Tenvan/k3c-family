package botdev

import (
	"context"
	"net/http/httptest"
	"path/filepath"
	"testing"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
	"k3c/tools/k3c-dev/internal/balance"
)

// startServer startet den echten Handler mit Räumen im Testprozess (nur Loopback).
func startServer(t *testing.T, token string) *httptest.Server {
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

func TestBotGeraeteSteuernMonarchen(t *testing.T) {
	srv := startServer(t, "geheim")
	bot, _ := balance.BotByName("saver")
	ctx := context.Background()
	a, err := Dial(ctx, srv.URL, "bot-a", bot)
	if err != nil {
		t.Fatal(err)
	}
	code, err := a.Create(ctx, "test-botdev")
	if err != nil {
		t.Fatal(err)
	}
	b, err := Dial(ctx, srv.URL, "bot-b", bot)
	if err != nil {
		t.Fatal(err)
	}
	if err := b.Join(ctx, code); err != nil {
		t.Fatal(err)
	}
	time.Sleep(3 * time.Second)
	a.Stop()
	b.Stop()
	for _, d := range []*Device{a, b} {
		st := d.Stats()
		if st.States == 0 || st.Inputs == 0 || !st.Moved || st.Disconnected || st.Errors != 0 {
			t.Errorf("%s: %+v", d.name, st)
		}
	}
}

func TestDeltaListen(t *testing.T) {
	state := map[string]any{"players": []any{map[string]any{"id": 1.0, "x": 1.0}, map[string]any{"id": 2.0, "x": 2.0}}, "drops": []any{}, "day": 1.0}
	applyDelta(state, map[string]any{
		"players": map[string]any{"set": []any{map[string]any{"id": 2.0, "x": 5.0}, map[string]any{"id": 3.0, "x": 3.0}}, "del": []any{1.0}},
		"day":     2.0, "unset": []any{"drops"},
	})
	ps := state["players"].([]any)
	if len(ps) != 2 || ps[0].(map[string]any)["x"] != 5.0 || ps[1].(map[string]any)["id"] != 3.0 || state["day"] != 2.0 {
		t.Errorf("Zustand: %v", state)
	}
	if _, ok := state["drops"]; ok {
		t.Error("unset nicht angewendet")
	}
}
