// Command k3c-studio ist das Asset-Studio von K3C als eigenständiges Fenster (B-459, B-498): ein Wails-Fenster (wie die
// Workbench k3c-dev) mit gemerkter Größe und Position, ohne den Spielserver. Das Backend (backend/) und die
// Oberfläche (dist/studio) bedient die EXE selbst auf Loopback; das Fenster zeigt diese Adresse. Schließen beendet alles.
package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"log/slog"
	"net"
	"net/http"
	"os"
	"path/filepath"
	"strconv"
	"time"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
	"github.com/wailsapp/wails/v2/pkg/runtime"

	"k3c/engine/conlog"
	assets "k3c/tools/assetkit"
)

// instancePrefix plus Hash der Wurzel verbindet einen zweiten Start für denselben Checkout mit dem laufenden Fenster.
const instancePrefix = "k3c-studio-8f2c4a61-"

// defaultPort ist der feste Port des Studios, damit die Einstellungen des Fensters (localStorage je Adresse) bleiben;
// K3C_STUDIO_PORT ändert ihn. Ist er belegt, nimmt die EXE einen freien Port.
const defaultPort = "8091"

// app hält Wurzel, Adresse und Fenster-Kontext.
type app struct {
	root  string
	url   string
	log   *slog.Logger
	ctx   context.Context
	ready chan struct{} // schließt startup: Wails ruft OnStartup und OnSecondInstanceLaunch in eigenen Goroutinen
	srv   *http.Server
	ln    net.Listener
}

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-studio:", err)
		showError("K3C Asset-Studio", err.Error()) // die EXE hat unter Windows keine Konsole
		os.Exit(1)
	}
}

func run() error {
	root, err := findRoot()
	if err != nil {
		return err
	}
	if err := checkBuilt(root); err != nil {
		return err
	}
	port := os.Getenv("K3C_STUDIO_PORT")
	if port == "" {
		port = defaultPort
	}
	if n, err := strconv.Atoi(port); err != nil || n < 1 || n > 65535 {
		return fmt.Errorf("K3C_STUDIO_PORT %q ist kein Port", port)
	}
	ln, err := listen(port)
	if err != nil {
		return err
	}
	a := &app{root: root, ln: ln, url: "http://" + ln.Addr().String() + studioPage, ready: make(chan struct{}),
		log: slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color()))}
	if _, got, _ := net.SplitHostPort(ln.Addr().String()); got != port {
		a.log.Warn("🚫 Port belegt, anderer Port gewählt (die Fenster-Einstellungen gelten je Adresse)", "ns", "main", "wunsch", port, "port", got)
	}
	return wails.Run(a.options())
}

// checkBuilt verlangt die gebaute Oberfläche; sie kommt aus `task studio:frontend`.
func checkBuilt(root string) error {
	if _, err := os.Stat(filepath.Join(root, "dist", "studio", "index.html")); err != nil {
		return fmt.Errorf("dist/studio/index.html fehlt in %s – erst bauen: task studio:frontend", root)
	}
	return nil
}

// options beschreibt das Fenster wie in k3c-dev: gemerkte Größe, verdeckter Start, bis die gemerkte Position gesetzt ist.
func (a *app) options() *options.App {
	win, placed := loadWindow(configPath("k3c-studio.json"))
	return &options.App{
		Title: "Asset-Studio", Width: win.Width, Height: win.Height, MinWidth: minWidth, MinHeight: minHeight,
		StartHidden:      true,
		BackgroundColour: &options.RGBA{R: 0x0b, G: 0x10, B: 0x26, A: 0xff},
		AssetServer:      &assetserver.Options{Handler: a.redirect()},
		OnStartup:        a.startup,
		OnDomReady: func(ctx context.Context) {
			if placed && onScreen(win) {
				placeWindow(ctx, win.X, win.Y)
			}
			runtime.WindowShow(ctx)
		},
		OnBeforeClose: a.beforeClose,
		OnShutdown:    func(context.Context) { a.stopServer() },
		SingleInstanceLock: &options.SingleInstanceLock{UniqueId: instanceID(a.root),
			OnSecondInstanceLaunch: func(options.SecondInstanceData) { a.secondInstance() }},
	}
}

// redirect ist die Startseite des Wails-Fensters: sie führt auf die Adresse des Studios (Seiten und Ereignisströme
// laufen über den eigenen Server, nicht über den Asset-Server von Wails).
func (a *app) redirect() http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		_, _ = fmt.Fprintf(w, `<!doctype html><meta charset="utf-8"><body style="background:#0b1026"><script>location.replace(%q)</script>`, a.url)
	})
}

// startup startet den Server des Studios; er endet mit dem Fenster.
func (a *app) startup(ctx context.Context) {
	a.ctx = ctx
	a.srv = &http.Server{Handler: newHandler(ctx, a.root), ReadHeaderTimeout: 10 * time.Second}
	go func() {
		if err := a.srv.Serve(a.ln); err != nil && !errors.Is(err, http.ErrServerClosed) {
			a.log.Error("💥 Asset-Studio-Server beendet", "ns", "main", "err", err)
		}
	}()
	close(a.ready)
	a.log.Info("🚀 Asset-Studio bereit", "ns", "main", "root", a.root, "addr", a.ln.Addr().String())
}

// stopServer beendet den Server des Studios beim Schließen des Fensters.
func (a *app) stopServer() {
	<-a.ready
	_ = a.srv.Close()
}

// beforeClose merkt Größe und Position des Fensters; das Schließen läuft immer weiter.
func (a *app) beforeClose(ctx context.Context) bool {
	if runtime.WindowIsMinimised(ctx) {
		return false // minimiert liefert Windows Platzhalter-Maße; die zuletzt gemerkten bleiben
	}
	x, y := runtime.WindowGetPosition(ctx)
	w, h := runtime.WindowGetSize(ctx)
	if err := saveWindow(configPath("k3c-studio.json"), windowState{X: x, Y: y, Width: w, Height: h}); err != nil {
		a.log.Warn("❌ Fenster nicht gemerkt", "ns", "main", "err", err)
	}
	return false
}

// secondInstance holt das laufende Fenster nach vorn, wenn das Studio für denselben Checkout ein zweites Mal startet.
func (a *app) secondInstance() {
	<-a.ready
	runtime.WindowUnminimise(a.ctx)
	runtime.WindowShow(a.ctx)
	runtime.WindowSetAlwaysOnTop(a.ctx, true) // Windows holt ein Fenster sonst nicht aus dem Hintergrund
	runtime.WindowSetAlwaysOnTop(a.ctx, false)
}

// instanceID ist die Kennung des Single-Instance-Locks je Checkout.
func instanceID(root string) string {
	sum := sha256.Sum256([]byte(filepath.Clean(root)))
	return instancePrefix + hex.EncodeToString(sum[:8])
}

// findRoot sucht die Repo-Wurzel ab dem Arbeitsverzeichnis (Start aus einem Worktree bedient diesen), ersatzweise ab
// dem Ordner der EXE (Doppelklick auf bin/k3c-studio.exe).
func findRoot() (string, error) {
	if wd, err := os.Getwd(); err == nil {
		if root, err := assets.FindRoot(wd); err == nil {
			return root, nil
		}
	}
	exe, err := os.Executable()
	if err != nil {
		return "", err
	}
	return assets.FindRoot(filepath.Dir(exe))
}
