package net

import (
	"context"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/coder/websocket"
	"k3c/engine/room"
	"k3c/engine/store"
)

// originServer ist ein Server mit Räumen und einem Spielstand "offen".
func originServer(t *testing.T) (*httptest.Server, *store.Saves, *room.Manager) {
	t.Helper()
	saves := &store.Saves{Dir: filepath.Join(t.TempDir(), "saves")}
	reports := &store.Reports{Dir: filepath.Join(t.TempDir(), "reports")}
	m := room.NewManager(saves)
	t.Cleanup(m.Close) // speichert offene Räume vor dem Aufräumen von TempDir
	srv := httptest.NewServer(NewHandler(Config{Saves: saves, Reports: reports, Rooms: m}))
	t.Cleanup(srv.Close)
	return srv, saves, m
}

func withOrigin(t *testing.T, method, url, origin string) int {
	t.Helper()
	req, err := http.NewRequest(method, url, strings.NewReader("{}"))
	if err != nil {
		t.Fatal(err)
	}
	if origin != "" {
		req.Header.Set("Origin", origin)
	}
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	_ = res.Body.Close()
	return res.StatusCode
}

// BD10.1, B-613/AC-01: Jede schreibende Methode unter /api/ mit fremdem Origin (auch "null") ist 403, der Handler läuft
// nicht: Raum und Spielstand bleiben.
func TestFremdeHerkunftAbgelehnt(t *testing.T) {
	srv, saves, m := originServer(t)
	x := hello(t, srv, "xbox")
	create(x, "offen", 0)
	x.entered()
	if _, err := saves.Store("rest", []byte(`{"version":8,"campaignId":"rest","world":"standard"}`)); err != nil {
		t.Fatal(err)
	}
	cases := []struct{ method, path string }{
		{http.MethodPost, "/api/room/close?slot=offen"},
		{http.MethodDelete, "/api/save?slot=rest"},
		{http.MethodPost, "/api/save?slot=rest"},
		{http.MethodPost, "/api/save/restore?slot=rest"},
		{http.MethodPost, "/api/report"},
		{http.MethodPost, "/api/clientlog"},
		{http.MethodPost, "/api/status/disconnect"},
		{http.MethodPost, "/api/status/save"},
		{http.MethodPost, "/api/dev"},
		{http.MethodPost, "/api/dev/fullsave"},
		{http.MethodPut, "/api/save?slot=rest"},
		{http.MethodPatch, "/api/level"},
	}
	for _, origin := range []string{"https://fremd.example", "null", "http://127.0.0.1:1"} {
		for _, c := range cases {
			if got := withOrigin(t, c.method, srv.URL+c.path, origin); got != http.StatusForbidden {
				t.Errorf("%s %s Origin %s: %d, erwartet 403", c.method, c.path, origin, got)
			}
		}
	}
	if len(m.Rooms()) != 1 {
		t.Fatal("Raum trotz Ablehnung geschlossen")
	}
	if _, err := saves.Load("rest"); err != nil {
		t.Fatalf("Spielstand trotz Ablehnung weg: %v", err)
	}
}

// BD10.1, B-613/AC-02: Die eigene Seite (Origin = Host) und Anfragen ohne Origin dürfen schreiben; Lesen prüft keinen Origin.
func TestEigeneHerkunftErlaubt(t *testing.T) {
	srv, saves, _ := originServer(t)
	x := hello(t, srv, "xbox")
	create(x, "offen", 0)
	x.entered()
	if got := withOrigin(t, http.MethodPost, srv.URL+"/api/room/close?slot=offen", srv.URL); got != http.StatusOK {
		t.Fatalf("Raum schließen von der eigenen Seite: %d", got)
	}
	if got := withOrigin(t, http.MethodGet, srv.URL+"/api/saves", "https://fremd.example"); got != http.StatusOK {
		t.Fatalf("GET mit fremdem Origin: %d, Lesen bleibt erlaubt", got)
	}
	if got := withOrigin(t, http.MethodDelete, srv.URL+"/api/save?slot=offen", ""); got != http.StatusOK {
		t.Fatalf("Löschen ohne Origin (curl): %d", got)
	}
	if _, err := saves.Load("offen"); err == nil {
		t.Fatal("Spielstand nach Löschen noch da")
	}
}

// BD10.1, B-613/AC-02: Zugriff im Heimnetz über IP oder Namen, direkt (8080) oder über den Vite-Proxy (5183, Host bleibt
// erhalten), passt, wenn Origin und Host denselben Host samt Port nennen; ein anderer Port ist fremd.
func TestHerkunftImHeimnetz(t *testing.T) {
	cases := []struct {
		host, origin string
		ok           bool
	}{
		{"192.168.1.20:8080", "http://192.168.1.20:8080", true},
		{"192.168.1.20:5183", "http://192.168.1.20:5183", true},
		{"PI.local:8080", "http://pi.local:8080", true},
		{"[fe80::1]:8080", "http://[fe80::1]:8080", true},
		{"pi.local", "https://pi.local", true},
		{"192.168.1.20:8080", "", true},
		{"192.168.1.20:8080", "http://192.168.1.20:5183", false},
		{"192.168.1.20:8080", "http://192.168.1.21:8080", false},
		{"192.168.1.20:8080", "http://192.168.1.20.fremd.example:8080", false},
		{"192.168.1.20:8080", "null", false},
	}
	for _, c := range cases {
		r := httptest.NewRequest(http.MethodPost, "/api/room/close", nil)
		r.Host = c.host
		if c.origin != "" {
			r.Header.Set("Origin", c.origin)
		}
		if got := sameOrigin(r); got != c.ok {
			t.Errorf("Host %s Origin %q: %v, erwartet %v", c.host, c.origin, got, c.ok)
		}
	}
}

// BD10.2, B-613/AC-01: Der WebSocket-Handshake prüft die Herkunft selbst (coder/websocket Accept): fremder Origin wird
// abgelehnt, die eigene Seite und Clients ohne Origin kommen durch.
func TestWebSocketHerkunft(t *testing.T) {
	srv, _, _ := originServer(t)
	url := "ws" + strings.TrimPrefix(srv.URL, "http") + "/ws"
	for origin, ok := range map[string]bool{"https://fremd.example": false, "null": false, srv.URL: true, "": true} {
		ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
		opts := &websocket.DialOptions{HTTPHeader: http.Header{}}
		if origin != "" {
			opts.HTTPHeader.Set("Origin", origin)
		}
		ws, _, err := websocket.Dial(ctx, url, opts)
		if err == nil {
			_ = ws.CloseNow()
		}
		cancel()
		if (err == nil) != ok {
			t.Errorf("WebSocket Origin %q: Fehler %v, erlaubt erwartet %v", origin, err, ok)
		}
	}
}
