// Package devrun ist der Dev-Lauf der Apps („Backend startet sich selbst neu“): Eine EXE mit -dev baut eine Kopie von
// sich als Kind, startet sie und baut sie bei jeder Änderung an ihren Go-Quellen neu. Das ersetzt wails dev und den
// externen Watcher tools/watch (k3c-watch).
package devrun

import (
	"context"
	"errors"
	"log/slog"
	"net"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// childEnv markiert ein von Run gestartetes Kind.
const childEnv = "K3C_DEV_CHILD"

var (
	pollEvery    = 300 * time.Millisecond // Abfragetakt der Änderungszeiten
	quiet        = 300 * time.Millisecond // Entprellfrist: so lange darf sich nichts mehr ändern (Speichern in Schritten, Formatierer)
	swapWait     = 3 * time.Second        // Windows gibt die EXE eines beendeten Prozesses manchmal erst verzögert frei
	restartPause = time.Second            // Mindestlaufzeit vor einem Neustart mit RestartCode (Bremse gegen eine Schleife)
	loopWindow   = time.Minute            // Schleifenbremse wie cmd/k3c-server/supervise.go: mehr als loopMax Neustarts
	loopMax      = 3                      // mit RestartCode in diesem Fenster warten auf die nächste Änderung
)

// isChild ist K3C_DEV_CHILD beim Start des Prozesses; der Wert verschwindet danach aus der Umgebung, damit Programme,
// die das Kind startet, nicht als devrun-Kind gelten.
var isChild = takeChildEnv()

// takeChildEnv liest K3C_DEV_CHILD und entfernt es aus der eigenen Umgebung.
func takeChildEnv() bool {
	v := os.Getenv(childEnv) == "1"
	_ = os.Unsetenv(childEnv)
	return v
}

// Config beschreibt einen Dev-Lauf.
type Config struct {
	Name    string   // für Logs, z. B. "studio"
	Dir     string   // Arbeitsordner für go build (Modul-Wurzel der App); dessen go.mod und go.sum werden immer beobachtet
	Package string   // Paket für go build, z. B. "." oder "./cmd/k3c-server"
	Child   string   // fester Pfad der Kind-EXE, z. B. bin/k3c-studio-dev.exe (absolut oder relativ zu Dir)
	Args    []string // Argumente des Kindes; das Kind läuft im Arbeitsverzeichnis des Aufrufers
	Env     []string // zusätzliche Umgebung des Kindes (K3C_DEV_CHILD=1 setzt Run selbst)
	// Watch sind Ordner (rekursiv, relativ zu Dir oder absolut), deren Go-Quellen beobachtet werden (*.go ohne _test.go,
	// go.mod, go.sum); frontend/, node_modules/, bin/, build/ und versteckte Ordner werden übersprungen. Eine einzeln
	// genannte Datei zählt immer.
	Watch       []string
	Extra       []string      // zusätzliche Dateiendungen, z. B. ".html" (optional)
	RestartCode int           // Exit-Code des Kindes, der einen Neustart ohne Neubau auslöst (k3c-server: 75); 0 = keiner
	Grace       time.Duration // Zeit für sauberes Beenden, danach hart (Standard 5 s)
	Log         *slog.Logger  // nil = slog.Default()
}

// Run baut das Kind, startet es und baut es bei jeder Änderung neu: erst bauen (an temporären Pfad neben Child), bei
// Erfolg Kind sauber beenden, warten, austauschen (Rename mit Wiederholung bei Sperre wie B-187), neu starten. Ein
// Build-Fehler lässt das laufende Kind stehen (❌-Log). Endet ctx, wird das Kind sauber beendet. Endet das Kind von
// selbst mit RestartCode, startet Run es neu; mit anderem Code wartet Run auf die nächste Änderung (kein Neustart-Loop).
func Run(ctx context.Context, c Config) error {
	r, err := newRunner(c)
	if err != nil {
		return err
	}
	return r.run(ctx)
}

// IsChild meldet, ob der Prozess ein von Run gestartetes Kind ist (K3C_DEV_CHILD=1 beim Start; gemerkt beim Laden des
// Pakets, danach aus der Umgebung entfernt).
func IsChild() bool { return isChild }

// Listen bindet wie net.Listen, wiederholt aber bis wait, solange die Adresse belegt ist (altes Kind gibt den Port
// gerade frei).
func Listen(network, addr string, wait time.Duration) (net.Listener, error) {
	deadline := time.Now().Add(wait)
	for {
		ln, err := net.Listen(network, addr)
		if err == nil || !addrInUse(err) || time.Now().After(deadline) {
			return ln, err
		}
		time.Sleep(100 * time.Millisecond)
	}
}

// runner ist ein laufender Dev-Lauf.
type runner struct {
	c     Config
	log   *slog.Logger
	dir   string   // Dir, absolut
	child string   // Child, absolut
	watch []string // beobachtete Pfade, absolut
	exts  []string // beobachtete Endungen (klein)
	cur   *child   // laufendes Kind, nil = keins
	// restarts sind die Neustarts mit RestartCode seit dem letzten Build (Schleifenbremse).
	restarts []time.Time
}

func newRunner(c Config) (*runner, error) {
	if c.Package == "" || c.Child == "" {
		return nil, errors.New("devrun: Package und Child sind Pflicht")
	}
	dir, err := filepath.Abs(c.Dir)
	if err != nil {
		return nil, err
	}
	if c.Grace <= 0 {
		c.Grace = 5 * time.Second
	}
	log := c.Log
	if log == nil {
		log = slog.Default()
	}
	abs := func(p string) string {
		if filepath.IsAbs(p) {
			return filepath.Clean(p)
		}
		return filepath.Join(dir, p)
	}
	r := &runner{c: c, log: log.With("ns", "dev", "app", c.Name), dir: dir, child: abs(c.Child),
		watch: []string{abs("go.mod"), abs("go.sum")}, exts: []string{".go"}}
	for _, w := range c.Watch {
		r.watch = append(r.watch, abs(w))
	}
	for _, e := range c.Extra {
		r.exts = append(r.exts, strings.ToLower(e))
	}
	return r, nil
}

// run baut und startet einmal, beobachtet dann die Quellen und baut bei jeder Änderung neu, bis ctx endet.
func (r *runner) run(ctx context.Context) error {
	defer r.shutdown()
	r.log.Info("🚀 Dev-Lauf startet", "paket", r.c.Package, "kind", r.child)
	base := snapshot(r.watch, r.exts)
	r.rebuild(ctx)
	tick := time.NewTicker(pollEvery)
	defer tick.Stop()
	for {
		select {
		case <-ctx.Done():
			r.log.Info("🛑 Dev-Lauf endet")
			return nil
		case <-r.exited():
			r.onExit(ctx)
			continue
		case <-tick.C:
		}
		now := snapshot(r.watch, r.exts)
		file := changed(base, now)
		if file == "" {
			continue
		}
		r.log.Info("⏳ Änderung erkannt", "datei", file)
		base = r.settle(ctx, now)
		if ctx.Err() == nil {
			r.rebuild(ctx)
		}
	}
}

// settle wartet, bis quiet lang nichts mehr geändert wurde, und liefert den Stand danach.
func (r *runner) settle(ctx context.Context, from map[string]fileSig) map[string]fileSig {
	cur := from
	for {
		select {
		case <-ctx.Done():
			return cur
		case <-time.After(quiet):
		}
		next := snapshot(r.watch, r.exts)
		if changed(cur, next) == "" {
			return next
		}
		cur = next
	}
}

// exited liefert den Kanal, der beim Ende des Kindes schließt; ohne Kind blockiert er (nil-Kanal).
func (r *runner) exited() <-chan struct{} {
	if r.cur == nil {
		return nil
	}
	return r.cur.done
}

// onExit reagiert auf ein Kind, das sich selbst beendet hat: RestartCode startet es neu (frühestens restartPause nach
// seinem Start, höchstens loopMax-mal je loopWindow), jeder andere Code wartet auf die nächste Änderung.
func (r *runner) onExit(ctx context.Context) {
	ch := r.cur
	r.cur = nil
	if r.c.RestartCode == 0 || ch.code != r.c.RestartCode {
		r.log.Warn("💥 Kind beendet, warte auf die nächste Änderung", "code", ch.code)
		return
	}
	now := time.Now()
	r.restarts = append(r.restarts, now)
	for len(r.restarts) > 0 && now.Sub(r.restarts[0]) > loopWindow {
		r.restarts = r.restarts[1:]
	}
	if len(r.restarts) > loopMax {
		r.log.Error("🚫 zu viele Neustarts in kurzer Zeit, warte auf die nächste Änderung", "anzahl", len(r.restarts), "fensterS", loopWindow.Seconds())
		r.restarts = nil
		return
	}
	r.log.Info("🔁 Kind will neu starten", "code", ch.code)
	select {
	case <-ctx.Done():
		return
	case <-time.After(restartPause - time.Since(ch.began)):
	}
	r.start()
}

// rebuild baut an einen temporären Pfad und tauscht erst nach einem erfolgreichen Build: Ein Fehler lässt das laufende
// Kind stehen.
func (r *runner) rebuild(ctx context.Context) {
	tmp := tempPath(r.child)
	r.log.Info("🔨 Build startet", "paket", r.c.Package)
	began := time.Now()
	if err := r.build(ctx, tmp); err != nil {
		_ = os.Remove(tmp)
		if ctx.Err() == nil { // bei Strg+C ist der abgebrochene Build kein Fehler
			r.log.Error("❌ Build fehlgeschlagen, das laufende Kind bleibt", "err", err)
		}
		return
	}
	r.log.Info("✅ Build fertig", "dauerMs", time.Since(began).Milliseconds())
	r.restarts = nil
	if r.cur != nil {
		r.log.Info("🔁 Kind startet neu")
		r.stop(nil)
	}
	if err := swap(tmp, r.child); err != nil {
		r.log.Error("❌ neue EXE nicht eingesetzt, starte die alte", "err", err)
		_ = os.Remove(tmp)
	}
	r.start()
}

// tempPath ist der Build-Pfad neben der Kind-EXE: bin/x-dev.exe → bin/x-dev.new.exe.
func tempPath(exe string) string {
	ext := filepath.Ext(exe)
	return strings.TrimSuffix(exe, ext) + ".new" + ext
}

// swap ersetzt die Kind-EXE durch den neuen Build. Unter Windows bleibt die EXE eines gerade beendeten Prozesses kurz
// gesperrt (wie B-187): wiederholen bis swapWait.
func swap(from, to string) error {
	deadline := time.Now().Add(swapWait)
	for {
		err := os.Rename(from, to)
		if err == nil || time.Now().After(deadline) {
			return err
		}
		time.Sleep(50 * time.Millisecond)
	}
}
