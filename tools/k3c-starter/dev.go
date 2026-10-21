package main

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"os/signal"
	"path/filepath"
	"runtime"
	"syscall"
	"time"

	"k3c/engine/devrun"
)

// devGrace ist die Frist, in der der Kindprozess nach dem Stopp-Signal den Server (bis 10 s + 5 s) beendet haben darf.
const devGrace = 20 * time.Second

// runDev ist der Entwicklungsmodus (`task k3c-starter:dev:server`, aus der Repo-Wurzel): devrun baut den Starter bei jeder
// Go-Änderung neu und führt ihn als Kindprozess mit -headless aus; die Oberfläche liefert Vite (Port 5194, leitet /api weiter).
func runDev() error {
	root, err := os.Getwd()
	if err != nil {
		return err
	}
	dir := filepath.Join(root, "tools", "k3c-starter")
	if _, err := os.Stat(filepath.Join(dir, "go.mod")); err != nil {
		return fmt.Errorf("-dev gehört in die Repo-Wurzel (tools/k3c-starter/go.mod nicht gefunden in %s)", root)
	}
	port := os.Getenv("K3C_STARTER_API_PORT")
	if port == "" {
		port = "8194"
	}
	// je API-Port eine eigene Kind-EXE: parallele Dev-Läufe ersetzen sich nicht gegenseitig die EXE
	child := filepath.Join(dir, "build", "bin", "k3c-starter-devchild-"+port)
	if runtime.GOOS == "windows" {
		child += ".exe"
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	return devrun.Run(ctx, devrun.Config{
		Name:    "starter",
		Dir:     dir,
		Package: ".",
		Child:   child,
		Args:    []string{"-headless"},
		Env:     []string{"K3C_ROOT=" + root},
		Watch:   []string{".", "../../engine/devrun"}, // der Starter bindet engine/devrun über replace ein
		Grace:   devGrace,
		Log:     slog.Default(),
	})
}
