package studioview

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sync"
	"time"

	"k3c/tools/k3c-studio/internal/gameexe"
)

// „Im Spiel ansehen“ (Entscheidung 005 › 5, B-847): legt den Testspielstand voll-<seed> (voll ausgebaut, Standard-Welt)
// außerhalb des Repos an und startet das Rust-Spiel mit festem Pfad (keine Firewall-Abfrage, kein cargo run), das ihn
// lädt. Der Browser-Client (dist/index.html, game.html) wird nicht mehr gebraucht (Entscheidung 009).

// fullSeed: `voll-<seed>` muss ins Namensformat eines Spielstands passen (wie fullSeed in engine/room).
var fullSeed = regexp.MustCompile(`^[a-z0-9-]{1,27}$`)

// Game ist „Im Spiel ansehen“ eines Checkouts.
type Game struct {
	ctx   context.Context
	root  string
	saves string
	log   *slog.Logger
	// launch startet das Spiel und kehrt sofort zurück (Tests ersetzen es); das Spiel überlebt das Studio.
	launch func(cmd *exec.Cmd) error
	// call führt `k3c-game sim <Sub>` aus (Tests ersetzen es; sonst gameexe.Call).
	call func(ctx context.Context, o gameexe.Options) (gameexe.Event, error)

	mu     sync.Mutex
	closed bool // nach Close startet nichts mehr

	open sync.Mutex // ein Open zugleich: zwei Aufrufe schrieben sonst denselben Spielstand gleichzeitig
}

// NewGame bereitet „Im Spiel ansehen“ für root vor; savesDir ist der Ordner der Testspielstände (leer = Cache des
// Nutzers, DefaultSavesDir). Endet ctx, startet Open nichts mehr. log darf nil sein.
func NewGame(ctx context.Context, root, savesDir string, log *slog.Logger) *Game {
	if log == nil {
		log = slog.New(slog.NewTextHandler(io.Discard, nil))
	}
	if savesDir == "" {
		savesDir = DefaultSavesDir(root)
	}
	return &Game{ctx: ctx, root: root, saves: savesDir, log: log, launch: startDetached, call: gameexe.Call}
}

// DefaultSavesDir ist `<Checkout>/.work/studioview/saves` (ignoriert, Regel „Schreiben nur im Repo“).
func DefaultSavesDir(root string) string {
	return filepath.Join(root, ".work", "studioview", "saves")
}

// Open legt den Testspielstand `voll-<seed>` an und startet das native Spiel, das ihn lädt; die Antwort sagt, was
// gestartet ist. Ein zweiter Aufruf startet ein weiteres Fenster.
func (g *Game) Open(seed string) (string, error) {
	if !fullSeed.MatchString(seed) {
		return "", hint("Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)")
	}
	g.open.Lock()
	defer g.open.Unlock()
	g.mu.Lock()
	closed := g.closed || g.ctx.Err() != nil
	g.mu.Unlock()
	if closed {
		return "", hint("„Im Spiel ansehen“ ist beendet")
	}
	exe, dir, err := g.exe()
	if err != nil {
		return "", err
	}
	save, err := g.fullSave(seed)
	if err != nil {
		return "", err
	}
	cmd := exec.Command(exe, "--saves", g.saves, "--load", save, "--players", "1")
	cmd.Dir = dir
	if err := g.launch(cmd); err != nil {
		return "", hint(fmt.Sprintf("Natives Spiel nicht gestartet: %v", err))
	}
	g.log.Info("🚀 Im Spiel ansehen: natives Spiel gestartet", "ns", "room", "exe", cmd.Path, "save", save)
	return "Natives Spiel gestartet mit Spielstand " + save, nil
}

// exe ist der feste Pfad der nativen EXE und ihr Arbeitsordner (Suchliste in gameexe); ohne EXE ein Hinweis auf den
// Build-Task. Die Frische prüft „Im Spiel ansehen“ nicht: ein älterer Build zeigt den Spielstand trotzdem.
func (g *Game) exe() (exe, dir string, err error) {
	exe, dir, err = gameexe.Locate(g.root)
	if err != nil {
		return "", "", hint(err.Error())
	}
	return exe, dir, nil
}

// fullSave legt voll-<seed> über `k3c-game sim save` an (Port von room.Manager.FullSave, RU40.2) und liefert den Namen.
func (g *Game) fullSave(seed string) (string, error) {
	if err := os.MkdirAll(g.saves, 0o750); err != nil {
		return "", err
	}
	input, err := json.Marshal(map[string]string{"seed": seed, "saves": g.saves})
	if err != nil {
		return "", err
	}
	ev, err := g.call(g.ctx, gameexe.Options{Root: g.root, Sub: "save", Input: input, Log: g.log})
	if err != nil {
		return "", fullSaveHint(err)
	}
	if ev.Text == "" {
		return "", hint("Spielstand nicht angelegt: Spiel-EXE meldet keinen Namen")
	}
	return ev.Text, nil
}

// startDetached startet das Spiel und holt den Prozess im Hintergrund ab, ohne auf ihn zu warten.
func startDetached(cmd *exec.Cmd) error {
	if err := cmd.Start(); err != nil {
		return err
	}
	go func() { _ = cmd.Wait() }()
	return nil
}

// Close: Danach startet Open nichts mehr; ein laufendes Spiel bleibt offen. Ein zweites Close tut nichts.
func (g *Game) Close(_ time.Duration) {
	g.mu.Lock()
	g.closed = true
	g.mu.Unlock()
}

// fullSaveHint macht aus einem Fehler von `sim save` einen Hinweis (Exit 2 = Seed ungültig, sonst die Meldung der EXE).
func fullSaveHint(err error) error {
	if err.Error() == "Exit 2" {
		return hint("Seed ungültig")
	}
	return hint(fmt.Sprintf("Spielstand nicht angelegt: %v", err))
}
