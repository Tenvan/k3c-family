package main

import (
	"context"
	"log/slog"
	stdnet "net"
	"os"
	"os/signal"
	"path/filepath"
	"runtime"
	"strings"
	"syscall"
	"time"

	"k3c/engine/devrun"
)

// Dev-Lauf (-dev, engine/devrun): Der Prozess (bin/k3c-server-devhost, lauscht selbst nicht) baut den Server bei jeder
// Änderung an Go-Code in cmd/k3c-server, engine/ oder data/ neu und tauscht ihn aus. Das Kind ist die feste EXE
// bin/k3c-server, damit die Firewall-Freigabe bleibt (B-085). Es läuft mit K3C_CHILD=1 ohne eigenen Supervisor; data/
// und public/ beobachtet es weiter selbst und endet dafür mit restartCode, dann startet devrun es ohne Neubau neu.

// portWait: So lange wartet ein Dev-Kind auf den Port, den sein Vorgänger gerade freigibt.
const portWait = 5 * time.Second

// devGrace: Frist für das saubere Ende des Kindes (rooms.Close, waitConns 2 s, Shutdown bis 5 s).
const devGrace = 10 * time.Second

// devChild ist die Kind-EXE des Dev-Laufs, relativ zur Repo-Wurzel: dieselbe wie bei task build und task start.
func devChild() string {
	if runtime.GOOS == "windows" {
		return "bin/k3c-server.exe"
	}
	return "bin/k3c-server"
}

// devConfig ist der Dev-Lauf aus der Repo-Wurzel root; dataDir ist der absolute Daten-Ordner ("" = eingebettet).
func devConfig(root, dataDir string, log *slog.Logger) devrun.Config {
	return devrun.Config{Name: "server", Dir: root, Package: "./cmd/k3c-server", Child: devChild(),
		Env: childEnviron(nil, dataDir, dataDir == ""), Watch: []string{"cmd/k3c-server", "engine", "data"},
		RestartCode: restartCode, Grace: devGrace, Log: log}
}

// runDev ist der Schalter -dev; Strg+C beendet das Kind sauber (Räume gespeichert).
func runDev() int {
	log := consoleLogger()
	root, err := os.Getwd()
	if err != nil {
		log.Error("💥 Arbeitsordner unbekannt", "ns", "dev", "err", err)
		return 1
	}
	if self, err := os.Executable(); err == nil && samePath(self, filepath.Join(root, devChild())) {
		log.Error("🚫 -dev nicht aus "+devChild()+" starten: Der Dev-Lauf ersetzt diese EXE (task k3c-server:dev nutzt bin/k3c-server-devhost)", "ns", "dev")
		return 1
	}
	ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
	defer stop()
	if err := devrun.Run(ctx, devConfig(root, dataDir(), log)); err != nil {
		log.Error("💥 Dev-Lauf fehlgeschlagen", "ns", "dev", "err", err)
		return 1
	}
	return 0
}

// samePath vergleicht zwei Pfade (Windows ohne Groß- und Kleinschreibung).
func samePath(a, b string) bool {
	a, b = filepath.Clean(a), filepath.Clean(b)
	return a == b || (runtime.GOOS == "windows" && strings.EqualFold(a, b))
}

// listenTCP bindet addr; als Dev-Kind wartet es bis portWait, falls das alte Kind den Port noch hält.
func listenTCP(addr string) (stdnet.Listener, error) {
	if devrun.IsChild() {
		return devrun.Listen("tcp", addr, portWait)
	}
	return stdnet.Listen("tcp", addr)
}
