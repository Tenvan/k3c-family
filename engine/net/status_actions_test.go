package net

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"k3c/engine/room"
	"k3c/engine/store"
)

// actionServer ist wie diagServer, mit eigenem Spielstand-Speicher und einem Log-Puffer.
func actionServer(t *testing.T) (*httptest.Server, memSaves, *lockedBuffer) {
	t.Helper()
	saves := memSaves{}
	m := room.NewManager(saves)
	codes := []string{"FAMILIE", "ZWEITER"}
	m.NewCode = func() string { c := codes[0]; codes = codes[1:]; return c }
	logBuf := &lockedBuffer{} // der Server loggt aus eigenen Goroutinen (Trennen, Speichern)
	root := t.TempDir()
	srv := httptest.NewServer(NewHandler(Config{Dist: root, StatusToken: diagToken, Version: "test", StartedAt: time.Now(),
		Saves: &store.Saves{Dir: filepath.Join(root, "saves")}, Reports: &store.Reports{Dir: filepath.Join(root, "reports")},
		Rooms: m, Log: slog.New(slog.NewTextHandler(logBuf, nil))}))
	t.Cleanup(srv.Close)
	return srv, saves, logBuf
}

func post(t *testing.T, srv *httptest.Server, path, auth string) (int, map[string]any) {
	t.Helper()
	req, _ := http.NewRequest("POST", srv.URL+path, nil)
	if auth != "" {
		req.Header.Set("Authorization", auth)
	}
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	var out map[string]any
	_ = json.NewDecoder(res.Body).Decode(&out)
	return res.StatusCode, out
}

func asDiag(t *testing.T, srv *httptest.Server, path string) (int, map[string]any) {
	t.Helper()
	return post(t, srv, path, "Bearer "+diagToken)
}

const (
	deviceA = "aaaaaa11-0000-4c1e-9a33-7e2f0d6c8a15"
	deviceB = "aaaaaa22-0000-4c1e-9a33-7e2f0d6c8a15"
)

// waitDevice wartet, bis das Gerät im Raum-Status den Zustand connected hat.
func waitDevice(t *testing.T, srv *httptest.Server, kennung string, connected bool) {
	t.Helper()
	for range 100 {
		_, sum := diagGet(t, srv, "/api/status?room=FAMILIE")
		for _, d := range sum["devices"].([]any) {
			if m := d.(map[string]any); m["id"] == kennung && m["connected"] == connected {
				return
			}
		}
		time.Sleep(20 * time.Millisecond)
	}
	t.Fatalf("Gerät %s wird nicht connected=%v", kennung, connected)
}

// B-088/AC-03: disconnect trennt die Verbindung, die Monarchen warten.
func TestDisconnectTrenntGeraet(t *testing.T) {
	srv, _, logBuf := actionServer(t)
	a := hello(t, srv, deviceA)
	create(a, "familie", 0)
	a.entered()
	waitDevice(t, srv, "aaaaaa", true)

	code, body := asDiag(t, srv, "/api/status/disconnect?room=FAMILIE&device=aaaaaa")
	if code != 200 || body["ok"] != true || body["device"] != "aaaaaa" {
		t.Fatalf("disconnect: %d %v", code, body)
	}
	waitDevice(t, srv, "aaaaaa", false)
	_, sum := diagGet(t, srv, "/api/status?room=FAMILIE")
	if sum["devices"].([]any)[0].(map[string]any)["slots"] == nil {
		t.Errorf("Gerät behält seine Plätze bis zur Frist: %v", sum)
	}
	log := logBuf.String()
	if !strings.Contains(log, "ns=diag") || !strings.Contains(log, "gerät getrennt") || strings.Contains(log, deviceA) || strings.Contains(log, diagToken) {
		t.Errorf("Log: %q", log)
	}
}

// B-088/AC-03: save sichert den Spielstand des Raums.
func TestSaveSichertSpielstand(t *testing.T) {
	srv, saves, logBuf := actionServer(t)
	a := hello(t, srv, deviceA)
	create(a, "familie", 0)
	a.entered()
	if _, ok := saves.peek("familie"); ok {
		t.Fatal("vor dem Aufruf darf noch nichts gesichert sein")
	}
	code, body := asDiag(t, srv, "/api/status/save?room=FAMILIE")
	if code != 200 || body["ok"] != true || body["save"] != "familie" {
		t.Fatalf("save: %d %v", code, body)
	}
	var save map[string]any
	raw, _ := saves.peek("familie")
	if err := json.Unmarshal(raw, &save); err != nil || save["campaignId"] == nil {
		t.Errorf("Spielstand: %v %s", err, raw)
	}
	if log := logBuf.String(); !strings.Contains(log, "ns=diag") || !strings.Contains(log, "spielstand gesichert") {
		t.Errorf("Log: %q", log)
	}
}

// B-088/AC-03: unbekannter Raum oder Gerät 404, gleiche Kennung bei zwei Geräten 409.
func TestAktionenFehlerfaelle(t *testing.T) {
	srv, _, _ := actionServer(t)
	a := hello(t, srv, deviceA)
	create(a, "familie", 0)
	a.entered()
	b := hello(t, srv, deviceB)
	joinRoom(b, "FAMILIE", 1)
	b.entered()
	waitDevice(t, srv, "aaaaaa1", true)

	for path, want := range map[string]int{
		"/api/status/save?room=NIX":                         404,
		"/api/status/disconnect?room=NIX&device=aaaaaa":     404,
		"/api/status/disconnect?room=FAMILIE&device=zzzzzz": 404,
		"/api/status/disconnect?room=FAMILIE":               404,
		"/api/status/disconnect?room=FAMILIE&device=aaaaaa": 409,
	} {
		if code, _ := asDiag(t, srv, path); code != want {
			t.Errorf("%s: %d, erwartet %d", path, code, want)
		}
	}
	if code, _ := asDiag(t, srv, "/api/status/disconnect?room=FAMILIE&device=aaaaaa2"); code != 200 {
		t.Errorf("eindeutige Kennung: %d", code)
	}
}

// B-088/AC-04: Token wie bei /api/status; GET ist 405.
func TestAktionenSindGeschuetzt(t *testing.T) {
	srv, _, _ := actionServer(t)
	for _, path := range []string{"/api/status/disconnect?room=FAMILIE&device=x", "/api/status/save?room=FAMILIE"} {
		for _, auth := range []string{"", "Bearer falsch", diagToken} {
			if code, _ := post(t, srv, path, auth); code != 401 {
				t.Errorf("%s mit %q: %d", path, auth, code)
			}
		}
		if code, _ := get(t, srv.URL+path, "Bearer "+diagToken); code != 405 {
			t.Errorf("GET %s: %d", path, code)
		}
	}
	off := httptest.NewServer(NewHandler(Config{}))
	defer off.Close()
	if code, _ := post(t, off, "/api/status/save?room=FAMILIE", "Bearer x"); code != 404 {
		t.Errorf("Diagnose aus: %d", code)
	}
}
