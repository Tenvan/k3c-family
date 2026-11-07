package studioview

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"net"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"regexp"
	"sync"
	"time"

	k3cnet "k3c/engine/net"
	"k3c/engine/room"
	"k3c/engine/store"
)

// „Im Spiel ansehen“ (Entscheidung 005 › 5, Sprint-README WB1 › D-04; übernommen aus tools/k3c-studio/backend, TS1.2):
// die Engine läuft als Bibliothek in der Workbench, mit eigenem Loopback-Listener auf einem freien Port (kein fester
// Port, keine Firewall-Abfrage) und Testspielständen außerhalb des Repos. Gestartet wird erst beim ersten Aufruf.

// fullSeed: `voll-<seed>` muss ins Namensformat eines Spielstands passen (wie fullSeed in engine/room).
var fullSeed = regexp.MustCompile(`^[a-z0-9-]{1,27}$`)

// Game ist „Im Spiel ansehen“ eines Checkouts.
type Game struct {
	ctx    context.Context
	cancel context.CancelFunc // beendet den Raum-Takt (Close)
	root   string
	saves  string
	log    *slog.Logger

	mu     sync.Mutex
	rooms  *room.Manager
	srv    *http.Server
	base   string        // http://127.0.0.1:<port>
	done   chan struct{} // schließt, wenn die Räume gespeichert und geschlossen sind
	closed bool          // nach Close startet nichts mehr
}

// NewGame bereitet „Im Spiel ansehen“ für root vor; savesDir ist der Ordner der Testspielstände (leer = Cache des
// Nutzers, DefaultSavesDir). Endet ctx, speichert die Engine ihre Räume. log darf nil sein.
func NewGame(ctx context.Context, root, savesDir string, log *slog.Logger) *Game {
	if log == nil {
		log = slog.New(slog.NewTextHandler(io.Discard, nil))
	}
	if savesDir == "" {
		savesDir = DefaultSavesDir(root)
	}
	ctx, cancel := context.WithCancel(ctx)
	return &Game{ctx: ctx, cancel: cancel, root: root, saves: savesDir, log: log}
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

// Open legt den Testspielstand `voll-<seed>` an (voll ausgebaut, Standard-Welt) und liefert die Adresse, unter der die
// Landingpage `game.html` mit ihm startet.
func (g *Game) Open(seed string) (string, error) {
	if !fullSeed.MatchString(seed) {
		return "", hint("Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)")
	}
	if _, err := os.Stat(filepath.Join(g.root, "dist", "index.html")); err != nil {
		return "", hint("Spiel-Client fehlt (dist/index.html) – erst bauen: task build")
	}
	rooms, base, err := g.start()
	if err != nil {
		return "", err
	}
	save, err := rooms.FullSave(seed)
	if err != nil {
		return "", fullSaveHint(err)
	}
	q := url.Values{"autostart": {"1"}, "save": {save}}
	return base + "/index.html#" + url.QueryEscape("game.html?"+q.Encode()), nil
}

// start startet Engine und Listener beim ersten Aufruf und liefert beide; nach Close lehnt es ab.
func (g *Game) start() (*room.Manager, string, error) {
	g.mu.Lock()
	defer g.mu.Unlock()
	switch {
	case g.closed:
		return nil, "", hint("„Im Spiel ansehen“ ist beendet")
	case g.srv != nil:
		return g.rooms, g.base, nil
	}
	if err := os.MkdirAll(g.saves, 0o750); err != nil {
		return nil, "", err
	}
	saves := &store.Saves{Dir: g.saves}
	rooms := room.NewManager(saves)
	rooms.Log = g.log
	rooms.Dev = true
	rooms.Worlds = os.DirFS(filepath.Join(g.root, "data", "worlds"))
	api := k3cnet.NewHandler(k3cnet.Config{
		Dist: filepath.Join(g.root, "dist"), Saves: saves, Reports: &store.Reports{Dir: filepath.Join(g.saves, "reports")},
		Log: g.log, Version: "workbench", StartedAt: time.Now(), Rooms: rooms, Conns: &sync.WaitGroup{},
	})
	ln, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		return nil, "", err
	}
	g.srv = &http.Server{Handler: handler(g.root, api), ReadHeaderTimeout: 10 * time.Second}
	g.rooms, g.base, g.done = rooms, "http://"+ln.Addr().String(), make(chan struct{})
	go func() { _ = g.srv.Serve(ln) }()
	go func() {
		rooms.Run(g.ctx)
		rooms.Close()
		close(g.done)
	}()
	g.log.Info("🚀 Im Spiel ansehen bereit", "ns", "room", "adresse", g.base, "spielstaende", g.saves)
	return g.rooms, g.base, nil
}

// shutdownWait begrenzt das Abwarten offener Anfragen, damit die Frist von Close fürs Speichern der Räume bleibt.
const shutdownWait = time.Second

// Close beendet Raum-Takt und Listener und wartet höchstens d, bis die Engine ihre Räume gespeichert hat. Danach
// startet Open nichts mehr; ein zweites Close tut nichts.
func (g *Game) Close(d time.Duration) {
	g.mu.Lock()
	srv, done, closed := g.srv, g.done, g.closed
	g.closed = true
	g.mu.Unlock()
	g.cancel()
	if srv == nil || closed {
		return
	}
	stop, cancel := context.WithTimeout(context.Background(), min(d, shutdownWait))
	_ = srv.Shutdown(stop)
	cancel()
	select {
	case <-done:
	case <-time.After(d):
		g.log.Warn("⏳ Räume von „Im Spiel ansehen“ nicht rechtzeitig gespeichert", "ns", "room")
	}
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
