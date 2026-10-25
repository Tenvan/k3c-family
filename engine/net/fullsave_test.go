package net

import (
	"bytes"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"k3c/engine/room"
)

// SV1.3 / AC-05: POST /api/dev/fullsave legt im Dev-Mode den Spielstand an, ohne Dev-Mode 403.
func TestDevVollAusgebaut(t *testing.T) {
	srv, m := devServer(t)
	post := func(body string) int {
		t.Helper()
		res, err := http.Post(srv.URL+"/api/dev/fullsave", "application/json", strings.NewReader(body))
		if err != nil {
			t.Fatal(err)
		}
		_ = res.Body.Close()
		return res.StatusCode
	}
	if code := post(`{"seed":"abc"}`); code != 200 {
		t.Fatalf("mit Dev-Mode: %d", code)
	}
	if _, err := m.Store.Load("voll-abc"); err != nil {
		t.Fatalf("Spielstand fehlt: %v", err)
	}
	if code := post(`{"seed":"../x"}`); code != 400 {
		t.Fatalf("ungültiger Seed: %d", code)
	}
	m.Dev = false
	if code := post(`{"seed":"xyz"}`); code != 403 {
		t.Fatalf("ohne Dev-Mode: %d", code)
	}
	if _, err := m.Store.Load("voll-xyz"); err == nil {
		t.Fatal("ohne Dev-Mode trotzdem gespeichert")
	}
}

// B-424/AC-02: Ist der Stand in einem Raum mit verbundenem Gerät offen, antwortet die Dev-API 409.
func TestDevVollAusgebautRaumBelegt(t *testing.T) {
	srv, _ := devServer(t)
	post := func() int {
		t.Helper()
		res, err := http.Post(srv.URL+"/api/dev/fullsave", "application/json", strings.NewReader(`{"seed":"abc"}`))
		if err != nil {
			t.Fatal(err)
		}
		_ = res.Body.Close()
		return res.StatusCode
	}
	if code := post(); code != 200 {
		t.Fatalf("erster Aufruf: %d", code)
	}
	x := hello(t, srv, "xbox")
	x.send(map[string]any{"t": "create", "save": "voll-abc", "fresh": false, "depth": 0, "slots": []int{0}})
	x.entered()
	if code := post(); code != 409 {
		t.Fatalf("Raum mit verbundenem Gerät: %d, erwartet 409", code)
	}
}

// B-426/AC-03: Ohne Content-Type application/json lehnt /api/dev/fullsave mit 415 ab, schreibt 🚫 ins Log und legt nichts an.
func TestDevVollAusgebautVerlangtJSON(t *testing.T) {
	m := room.NewManager(memSaves{})
	m.Dev = true
	var logs bytes.Buffer
	srv := httptest.NewServer(NewHandler(Config{Rooms: m, Log: slog.New(slog.NewTextHandler(&logs, nil))}))
	t.Cleanup(srv.Close)
	for _, ct := range []string{"", "text/plain", "application/x-www-form-urlencoded", "multipart/form-data; boundary=x"} {
		req, err := http.NewRequest(http.MethodPost, srv.URL+"/api/dev/fullsave", strings.NewReader(`{"seed":"abc"}`))
		if err != nil {
			t.Fatal(err)
		}
		if ct != "" {
			req.Header.Set("Content-Type", ct)
		}
		res, err := http.DefaultClient.Do(req)
		if err != nil {
			t.Fatal(err)
		}
		_ = res.Body.Close()
		if res.StatusCode != http.StatusUnsupportedMediaType {
			t.Errorf("Content-Type %q: %d, erwartet 415", ct, res.StatusCode)
		}
	}
	if _, err := m.Store.Load("voll-abc"); err == nil {
		t.Fatal("ohne JSON trotzdem gespeichert")
	}
	if !strings.Contains(logs.String(), "🚫 Spielstand voll ausgebaut ohne JSON abgelehnt") {
		t.Fatalf("Log ohne 🚫: %s", logs.String())
	}
	res, err := http.Post(srv.URL+"/api/dev/fullsave", "application/json; charset=utf-8", strings.NewReader(`{"seed":"abc"}`))
	if err != nil {
		t.Fatal(err)
	}
	_ = res.Body.Close()
	if res.StatusCode != http.StatusOK {
		t.Fatalf("mit charset: %d", res.StatusCode)
	}
}
