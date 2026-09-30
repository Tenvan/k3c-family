package main

import (
	"io"
	"log/slog"
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

func TestOhneBuildKeinStart(t *testing.T) {
	cfg := loadConfig()
	cfg.dist = filepath.Join(t.TempDir(), "dist")
	err := run(cfg, slog.New(slog.NewTextHandler(io.Discard, nil)))
	if err == nil || !strings.Contains(err.Error(), "npm run build") {
		t.Errorf("run ohne dist: %v", err)
	}
}
