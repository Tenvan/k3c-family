package main

import (
	"io"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"net/url"
	"path/filepath"
	"strings"
	"testing"
)

func TestKonfiguration(t *testing.T) {
	if c := loadConfig(); c.httpPort != "8080" || c.httpsPort != "8443" || c.dist != "dist" || c.saves != "saves" {
		t.Errorf("Vorgaben: %+v", c)
	}
	t.Setenv("K3C_HTTP_PORT", "9090")
	t.Setenv("K3C_SAVES_DIR", "/data/saves")
	if c := loadConfig(); c.httpPort != "9090" || c.saves != "/data/saves" {
		t.Errorf("Umgebung: %+v", c)
	}
}

func TestDevModeAusUmgebung(t *testing.T) {
	for v, want := range map[string]bool{"": true, "1": true, "0": false} {
		t.Setenv("K3C_DEV", v)
		if got := loadConfig().dev; got != want {
			t.Errorf("K3C_DEV=%q: dev=%v, erwartet %v", v, got, want)
		}
	}
}

func TestOhneBuildKeinStart(t *testing.T) {
	cfg := loadConfig()
	cfg.dist = filepath.Join(t.TempDir(), "dist")
	err := run(cfg, slog.New(slog.NewTextHandler(io.Discard, nil)))
	if err == nil || !strings.Contains(err.Error(), "task build") {
		t.Errorf("run ohne dist: %v", err)
	}
}

func TestHealthSchalter(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/health" {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()
	u, _ := url.Parse(srv.URL)
	if code := checkHealth(u.Port()); code != 0 {
		t.Errorf("gesund: %d", code)
	}
	srv.Close()
	if code := checkHealth(u.Port()); code != 1 {
		t.Errorf("aus: %d", code)
	}
}
