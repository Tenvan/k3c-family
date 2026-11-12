package botdev

import (
	"context"
	"encoding/json"
	"net/http/httptest"
	"path/filepath"
	"testing"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
	"k3c/tools/k3c-workbench/internal/simexe"
)

// Die Tests laufen gegen den echten Handler im Testprozess (nur Tests importieren engine/net, room, store); die Bots
// ersetzt ein fester Entscheider, k3c-sim bot prüft cmd/k3c-sim (bot_test.go).

// stepRight geht nach rechts und zahlt; zählt die Anfragen.
type stepRight struct{ calls int }

func (s *stepRight) Decide(_ json.RawMessage, players []int) ([]simexe.Command, error) {
	s.calls++
	out := make([]simexe.Command, len(players))
	for i, p := range players {
		out[i] = simexe.Command{Player: p, MoveX: 1, Pay: true}
	}
	return out, nil
}

func TestProtokollVersionWieServer(t *testing.T) {
	if ProtocolVersion != k3cnet.ProtocolVersion {
		t.Errorf("botdev.ProtocolVersion %d, engine/net %d", ProtocolVersion, k3cnet.ProtocolVersion)
	}
}

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

// skipOnline: Online-Tests starten Räume im echten Handler; sie entfallen mit dem Online-Spiel (Entscheidung 009,
// B-823) und laufen bis dahin nur ohne -short (B-619, B-704).
func skipOnline(t *testing.T) {
	t.Helper()
	if testing.Short() {
		t.Skip("Online-Test mit Spielserver, läuft ohne -short (B-619, B-704)")
	}
}

// waitUntil wartet, bis ok gilt: Fortschritt kommt im Takt des Servers, unter Last langsamer als die Wanduhr (B-704).
func waitUntil(t *testing.T, what string, ok func() bool) {
	t.Helper()
	deadline := time.Now().Add(30 * time.Second)
	for !ok() {
		if time.Now().After(deadline) {
			t.Fatalf("nicht erreicht: %s", what)
		}
		time.Sleep(20 * time.Millisecond)
	}
}

func TestBotGeraeteSteuernMonarchen(t *testing.T) {
	skipOnline(t)
	srv := startServer(t, "geheim")
	bot := &stepRight{}
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
	for _, d := range []*Device{a, b} {
		waitUntil(t, d.name+" bewegt seinen Monarchen", func() bool {
			st := d.Stats()
			return st.States > 0 && st.Inputs > 0 && st.Moved
		})
	}
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

// TR4.2, B-352: Der Beobachter legt den Raum ohne Platz an, liest den Zustand mit dem Monarchen des Spielers und
// schickt keine Eingabe.
func TestBeobachterOhnePlatz(t *testing.T) {
	skipOnline(t)
	srv := startServer(t, "geheim")
	bot := &stepRight{}
	ctx := context.Background()
	o, err := Dial(ctx, srv.URL, "beobachter", bot)
	if err != nil {
		t.Fatal(err)
	}
	code, err := o.CreateObserved(ctx, "test-beobachter")
	if err != nil {
		t.Fatal(err)
	}
	p, err := Dial(ctx, srv.URL, "spieler", bot)
	if err != nil {
		t.Fatal(err)
	}
	if err := p.Join(ctx, code); err != nil {
		t.Fatal(err)
	}
	waitUntil(t, "Beobachter sieht den Spieler", func() bool {
		w, _ := o.World()
		return o.Stats().States > 0 && w != nil && len(w.Players) == 1
	})
	w, own := o.World()
	p.Stop()
	o.Stop()
	if st := o.Stats(); st.States == 0 || st.Inputs != 0 || st.Errors != 0 || st.Disconnected {
		t.Errorf("Beobachter: %+v", st)
	}
	if w == nil || own != -1 || len(w.Players) != 1 || !json.Valid(w.State) {
		t.Fatalf("Zustand des Beobachters: eigener Monarch %d, Welt %v", own, w)
	}
}
