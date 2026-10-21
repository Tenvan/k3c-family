package main

import (
	"io"
	"log/slog"
	"slices"
	"strings"
	"testing"
)

// Der Dev-Lauf baut den Server aus der Repo-Wurzel, startet das Kind ohne eigenen Supervisor mit dem Daten-Ordner und
// startet es bei restartCode (geänderte data/ oder public/) ohne Neubau neu.
func TestDevConfig(t *testing.T) {
	c := devConfig("/repo", "/repo/data", slog.New(slog.NewTextHandler(io.Discard, nil)))
	if c.Dir != "/repo" || c.Package != "./cmd/k3c-server" || strings.TrimSuffix(c.Child, ".exe") != "bin/k3c-server" || c.RestartCode != restartCode || c.Grace != devGrace {
		t.Fatalf("devConfig = %+v", c)
	}
	if !slices.Contains(c.Env, childEnv+"=1") || !slices.Contains(c.Env, "K3C_DATA_DIR=/repo/data") {
		t.Errorf("Env = %q, will K3C_CHILD=1 und den Daten-Ordner", c.Env)
	}
	if !slices.Equal(c.Watch, []string{"cmd/k3c-server", "engine", "data"}) {
		t.Errorf("Watch = %q", c.Watch)
	}
	if e := devConfig("/repo", "", nil).Env; !slices.Contains(e, "K3C_DATA_DIR=") {
		t.Errorf("ohne Daten-Ordner: Env = %q, will eingebettet (leeres K3C_DATA_DIR)", e)
	}
}

func TestSamePath(t *testing.T) {
	if !samePath("bin/k3c-server", "bin/./k3c-server") || samePath("bin/k3c-server", "bin/k3c-server-devhost") {
		t.Error("samePath vergleicht falsch")
	}
}
