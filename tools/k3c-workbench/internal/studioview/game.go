package studioview

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sync"
	"time"

	"k3c/engine/room"
	"k3c/engine/store"
)

// „Im Spiel ansehen“ (Entscheidung 005 › 5, B-847): legt den Testspielstand voll-<seed> (voll ausgebaut, Standard-Welt)
// außerhalb des Repos an und startet das native Spiel mit festem Pfad (keine Firewall-Abfrage, kein go run), das ihn
// lädt. Der Browser-Client (dist/index.html, game.html) wird nicht mehr gebraucht (Entscheidung 009).

// fullSeed: `voll-<seed>` muss ins Namensformat eines Spielstands passen (wie fullSeed in engine/room).
var fullSeed = regexp.MustCompile(`^[a-z0-9-]{1,27}$`)

// nativeBuildTask baut die native EXE; die Meldung bei fehlender EXE nennt ihn (B-847/AC-02).
const nativeBuildTask = "task native:game:build"

// nativeExe sind die Orte der nativen EXE, zuerst der Build im Checkout, dann das Spiel-Paket; dir ist der
// Arbeitsordner, unter dem das Spiel public/ findet. Pfade relativ zur Wurzel des Checkouts.
var nativeExe = []struct{ exe, dir string }{
	{"native/k3c-game/bin/k3c-game.exe", "."},
	{"dist/k3c-game/k3c-game.exe", "dist/k3c-game"},
}

// Game ist „Im Spiel ansehen“ eines Checkouts.
type Game struct {
	ctx   context.Context
	root  string
	saves string
	log   *slog.Logger
	// launch startet das Spiel und kehrt sofort zurück (Tests ersetzen es); das Spiel überlebt die Workbench.
	launch func(cmd *exec.Cmd) error

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
	return &Game{ctx: ctx, root: root, saves: savesDir, log: log, launch: startDetached}
}

// DefaultSavesDir ist `<Cache des Nutzers>/k3c/studioview/<Hash des Checkouts>/saves`, nie im Repo.
func DefaultSavesDir(root string) string {
	base, err := os.UserCacheDir()
	if err != nil {
		base = os.TempDir()
	}
	sum := sha256.Sum256([]byte(filepath.Clean(root)))
	return filepath.Join(base, "k3c", "studioview", hex.EncodeToString(sum[:6]), "saves")
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

// exe ist der feste Pfad der nativen EXE und ihr Arbeitsordner; ohne EXE ein Hinweis auf den Build-Task.
func (g *Game) exe() (exe, dir string, err error) {
	for _, n := range nativeExe {
		exe = filepath.Join(g.root, filepath.FromSlash(n.exe))
		if st, err := os.Stat(exe); err == nil && !st.IsDir() {
			return exe, filepath.Join(g.root, filepath.FromSlash(n.dir)), nil
		}
	}
	return "", "", hint("Natives Spiel fehlt (" + nativeExe[0].exe + ") – erst bauen: " + nativeBuildTask)
}

// fullSave legt voll-<seed> über die Engine an (wie der Spielserver im Dev-Modus) und liefert den Namen.
func (g *Game) fullSave(seed string) (string, error) {
	if err := os.MkdirAll(g.saves, 0o750); err != nil {
		return "", err
	}
	rooms := room.NewManager(&store.Saves{Dir: g.saves})
	rooms.Log = g.log
	rooms.Dev = true
	rooms.Worlds = os.DirFS(filepath.Join(g.root, "data", "worlds"))
	defer rooms.Close()
	save, err := rooms.FullSave(seed)
	if err != nil {
		return "", fullSaveHint(err)
	}
	return save, nil
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

// fullSaveHint macht aus einem Fehler der Engine beim Anlegen von „voll ausgebaut“ einen Hinweis.
func fullSaveHint(err error) error {
	switch {
	case errors.Is(err, room.ErrSaveInUse):
		return hint("Spielstand ist gerade in einem Raum mit verbundenem Gerät offen")
	case errors.Is(err, room.ErrBadRequest):
		return hint("Seed ungültig")
	}
	return hint(fmt.Sprintf("Spielstand nicht angelegt: %v", err))
}
