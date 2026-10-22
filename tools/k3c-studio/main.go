// Command k3c-studio ist das Asset-Studio von K3C als eigenständiges Fenster (B-459, B-498): ein Wails-Fenster (wie die
// Workbench k3c-workbench) mit gemerkter Größe und Position, ohne den Spielserver. Das Backend (backend/) und die
// Oberfläche (frontend/dist) bedient die EXE selbst auf Loopback; das Fenster zeigt diese Adresse. Schließen beendet alles.
package main

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"flag"
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
	"k3c/tools/k3c-studio/backend"
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
	// stop beendet den Kontext der Engine, studio wartet danach auf das Speichern ihrer Räume
	stop   context.CancelFunc
	studio *backend.App
}

func main() {
	headless := flag.Bool("headless", false, "ohne Fenster: nur Backend und Engine auf Loopback (K3C_STUDIO_PORT), bis Strg+C")
	dev := flag.Bool("dev", false, "Entwicklung: baut das Studio bei jeder Go-Änderung neu und startet es als -headless (engine/devrun)")
	flag.Parse()
	if err := run(*headless, *dev); err != nil {
		fmt.Fprintln(os.Stderr, "k3c-studio:", err)
		if !*headless && !*dev {
			showError("K3C Asset-Studio", err.Error()) // das Fenster hat unter Windows keine Konsole
		}
		os.Exit(1)
	}
}

func run(headless, dev bool) error {
	root, err := findRoot()
	if err != nil {
		return err
	}
	if dev {
		return runDev(root)
	}
	port := os.Getenv("K3C_STUDIO_PORT")
	if port == "" {
		port = defaultPort
	}
	if n, err := strconv.Atoi(port); err != nil || n < 1 || n > 65535 {
		return fmt.Errorf("K3C_STUDIO_PORT %q ist kein Port", port)
	}
	if headless {
		return runHeadless(root, port) // die Seiten liefert Vite, frontend/dist ist nicht nötig
	}
	if err := checkBuilt(root); err != nil {
		return err
	}
	ln, err := listen(port)
	if err != nil {
		return err
	}
	a := &app{root: root, ln: ln, url: windowURL(ln.Addr().String()), ready: make(chan struct{}),
		log: slog.New(conlog.New(os.Stderr, slog.LevelInfo, conlog.Color()))}
	if _, got, _ := net.SplitHostPort(ln.Addr().String()); got != port {
		a.log.Warn("🚫 Port belegt, anderer Port gewählt (die Fenster-Einstellungen gelten je Adresse)", "ns", "main", "wunsch", port, "port", got)
	}
	return wails.Run(a.options())
}

// windowURL ist die Adresse, die das Fenster zeigt: die gebaute Oberfläche der EXE. Entwickelt wird ohne Fenster
// (`task k3c-studio:dev`: -dev und Vite im Browser); `task k3c-studio:window` zeigt unter `wails dev` dieselbe Adresse.
func windowURL(addr string) string {
	return "http://" + addr + studioPage
}

// checkBuilt verlangt die gebaute Oberfläche; sie kommt aus `task k3c-studio:frontend`.
func checkBuilt(root string) error {
	if _, err := os.Stat(filepath.Join(root, filepath.FromSlash(uiDir), "index.html")); err != nil {
		return fmt.Errorf("%s/index.html fehlt in %s – erst bauen: task k3c-studio:frontend", uiDir, root)
	}
	return nil
}

// options beschreibt das Fenster wie in k3c-workbench: gemerkte Größe, verdeckter Start, bis die gemerkte Position gesetzt ist.
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
	studioCtx, stop := context.WithCancel(ctx)
	handler, studio := newHandler(studioCtx, a.root, a.log)
	a.stop, a.studio = stop, studio
	a.srv = &http.Server{Handler: handler, ReadHeaderTimeout: 10 * time.Second}
	go func() {
		if err := a.srv.Serve(a.ln); err != nil && !errors.Is(err, http.ErrServerClosed) {
			a.log.Error("💥 Asset-Studio-Server beendet", "ns", "main", "err", err)
		}
	}()
	close(a.ready)
	a.log.Info("🚀 Asset-Studio bereit", "ns", "main", "root", a.root, "addr", a.ln.Addr().String())
}

// stopServer beendet den Server des Studios beim Schließen des Fensters und wartet kurz, bis die Engine ihre Räume
// gespeichert hat.
func (a *app) stopServer() {
	<-a.ready
	_ = a.srv.Close()
	a.stop()
	if !a.studio.EngineWait(engineSaveWait) {
		a.log.Warn("⏳ Räume des Studios nicht rechtzeitig gespeichert", "ns", "room", "frist", engineSaveWait.String())
	}
}

// engineSaveWait ist die Frist für das Speichern der Räume beim Schließen.
const engineSaveWait = 5 * time.Second

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
