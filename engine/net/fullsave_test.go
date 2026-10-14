package net

import (
	"net/http"
	"strings"
	"testing"
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
