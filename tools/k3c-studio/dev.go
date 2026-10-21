package main

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net"
	"os"
	"os/signal"
	"path/filepath"
	"runtime"
	"syscall"
	"time"

	"k3c/engine/conlog"
	"k3c/engine/devrun"
)

// devGrace ist die Frist, in der das Kind nach dem Stopp-Signal seine Räume speichern darf (engineSaveWait plus Luft).
const devGrace = 10 * time.Second

// listenWait ist die Zeit, die ein Kind von devrun auf den Port wartet: das alte Kind gibt ihn gerade frei.
const listenWait = 5 * time.Second

// runHeadless ist das Studio ohne Fenster (`-headless`): Backend und eingebettete Engine auf Loopback, bis zum Signal.
// Der Port ist fest (K3C_STUDIO_PORT, Standard 8091), denn Vite leitet /api genau dorthin (K3C_STUDIO_API).
func runHeadless(root, port string) error {
	log := slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color()))
	ln, err := devrun.Listen("tcp", net.JoinHostPort("127.0.0.1", port), listenWait)
	if err != nil {
		return fmt.Errorf("der Port %s ist belegt (K3C_STUDIO_PORT ändert ihn, auch für den Vite-Proxy): %w", port, err)
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	a := &app{root: root, ln: ln, url: windowURL(ln.Addr().String()), ready: make(chan struct{}), log: log}
	a.startup(ctx)
	<-ctx.Done()
	log.Info("🛑 Asset-Studio beendet", "ns", "main")
	a.stopServer()
	return nil
}

// runDev ist der Entwicklungsmodus (`task k3c-studio:dev:server`, aus der Repo-Wurzel): devrun baut das Studio bei jeder
// Go-Änderung neu und führt es als Kind mit -headless aus; die Oberfläche liefert Vite (Port 5192, leitet /api weiter).
// data/ wird nicht beobachtet: das Studio schreibt dort selbst.
func runDev(root string) error {
	dir := filepath.Join(root, "tools", "k3c-studio")
	if _, err := os.Stat(filepath.Join(dir, "go.mod")); err != nil {
		return errors.New("-dev gehört in die Repo-Wurzel (tools/k3c-studio/go.mod nicht gefunden)")
	}
	// je Backend-Port eine eigene Kind-EXE: zwei Dev-Läufe (Wurzel, Worktree, Test) ersetzen sich nicht gegenseitig die EXE
	child := filepath.Join(dir, "build", "bin", "k3c-studio-devchild-"+envOr("K3C_STUDIO_PORT", "8192"))
	if runtime.GOOS == "windows" {
		child += ".exe"
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	return devrun.Run(ctx, devrun.Config{
		Name:    "studio",
		Dir:     dir,
		Package: ".",
		Child:   child,
		Args:    []string{"-headless"},
		Watch:   []string{".", "../assetkit", "../ruletable", "../../engine", "../../go.mod", "../../go.sum"},
		Grace:   devGrace,
		Log:     slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color())),
	})
}

// envOr liefert die Umgebungsvariable name oder dflt.
func envOr(name, dflt string) string {
	if v := os.Getenv(name); v != "" {
		return v
	}
	return dflt
}
