package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"k3c/data"
	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/engine/store"
	"k3c/engine/watch"
)

// Datei-Watch (B-507, AC-04): Der Server beobachtet immer data/** und public/ (Grafiken, Sound). Nach einer entprellten
// Änderung prüft ein Kindprozess (-check-data) den neuen Stand mit demselben Ladecode wie beim Start; ist er gültig,
// bekommen alle Geräte den Hinweis „Server startet neu“, nach restartDelay endet der Prozess mit restartCode und der
// Supervisor (supervise.go) startet ihn neu. Ein ungültiger Stand lässt den Server mit dem alten Stand weiterlaufen.

const (
	restartCode  = 75               // Exit-Code: Kind will neu gestartet werden (EX_TEMPFAIL)
	restartDelay = 3 * time.Second  // Hinweis sichtbar, bevor die Räume schließen
	watchQuiet   = 2 * time.Second  // Entprellfrist nach der letzten Änderung (Studio schreibt mehrere Dateien)
	watchEvery   = time.Second      // Abfragetakt
	checkLimit   = 30 * time.Second // Zeitlimit der Prüfung im Kindprozess
)

// errRestart beendet run für einen Neustart (main: Exit restartCode).
var errRestart = errors.New("neustart wegen geänderter Dateien")

// restarter ist der Ablauf nach einer Änderung; alle Teile sind eingespeist (Test).
type restarter struct {
	log      *slog.Logger
	check    func() error        // prüft den neuen Stand
	announce func()              // Hinweis an alle Geräte
	wait     func(time.Duration) // Frist zwischen Hinweis und Neustart
	restart  func()              // beendet den Server mit errRestart
}

func (r restarter) changed(paths []string) {
	r.log.Info("🔁 Dateien geändert, prüfe neuen Stand", "ns", "watch", "anzahl", len(paths), "dateien", firstN(paths, 10))
	if err := r.check(); err != nil {
		r.log.Error("❌ neuer Stand ungültig, Server läuft mit altem Stand weiter", "ns", "watch", "err", err)
		return
	}
	r.log.Info("⏳ Server startet neu", "ns", "watch", "fristS", restartDelay.Seconds())
	r.announce()
	r.wait(restartDelay)
	r.restart()
}

func firstN(s []string, n int) []string { return s[:min(n, len(s))] }

// watchRoots sind data/ und public/, ohne fehlende Ordner und ohne solche, die einen Schreib-Ordner des Servers
// (Spielstände, Berichte, Logs) enthalten oder in einem liegen: Der Server schreibt nie in beobachtete Ordner.
func watchRoots(cfg config, writes []string, log *slog.Logger) []watch.Root {
	var out []watch.Root
	for _, r := range []struct{ name, dir string }{{"data", cfg.data}, {"public", cfg.public}} {
		if info, err := os.Stat(r.dir); err != nil || !info.IsDir() {
			continue
		}
		if w := overlap(r.dir, writes); w != "" {
			log.Warn("🚫 Ordner nicht beobachtet: Server schreibt dort", "ns", "watch", "ordner", r.dir, "schreibt", w)
			continue
		}
		out = append(out, watch.Root{Name: r.name, FS: os.DirFS(r.dir)})
	}
	return out
}

// overlap ist der erste Schreib-Ordner, der dir enthält oder in dir liegt; leer = keiner.
func overlap(dir string, writes []string) string {
	for _, w := range writes {
		if w != "" && (inside(dir, w) || inside(w, dir)) {
			return w
		}
	}
	return ""
}

// inside: Liegt path in base (oder ist base)?
func inside(base, path string) bool {
	a, err1 := filepath.Abs(base)
	b, err2 := filepath.Abs(path)
	if err1 != nil || err2 != nil {
		return false
	}
	rel, err := filepath.Rel(a, b)
	return err == nil && rel != ".." && !strings.HasPrefix(rel, ".."+string(filepath.Separator)) && !filepath.IsAbs(rel)
}

// startWatch beobachtet die Ordner im Takt watchEvery, bis ctx endet.
func startWatch(ctx context.Context, roots []watch.Root, r restarter) {
	names := make([]string, len(roots))
	for i, root := range roots {
		names[i] = root.Name
	}
	r.log.Info("🚀 Datei-Watch aktiv", "ns", "watch", "ordner", names, "entprellenS", watchQuiet.Seconds())
	w := watch.New(roots, watchQuiet, data.LoadedAt) // was nach dem Laden der Daten geändert wurde, zählt als Änderung
	ticker := time.NewTicker(watchEvery)
	go func() {
		defer ticker.Stop()
		watch.Run(ctx, w, ticker.C, r.changed)
	}()
}

// checkChild prüft den Stand in cfg.data mit einem Kindprozess `k3c-server -check-data`: Er lädt beim Start alle Daten
// wie der Server (ein Fehler endet mit Panic) und prüft die Welten (checkData).
func checkChild(cfg config) func() error {
	return func() error {
		exe, err := os.Executable()
		if err != nil {
			return err
		}
		ctx, cancel := context.WithTimeout(context.Background(), checkLimit)
		defer cancel()
		cmd := exec.CommandContext(ctx, exe, "-check-data")
		cmd.Env = append(os.Environ(), data.DirEnv+"="+absDir(cfg.data))
		out, err := cmd.CombinedOutput()
		if err != nil {
			return fmt.Errorf("%w: %s", err, firstLine(out))
		}
		return nil
	}
}

// firstLine ist die erste nicht leere Zeile der Ausgabe (bei einem Panic der Grund), höchstens 300 Zeichen.
func firstLine(out []byte) string {
	for line := range strings.Lines(string(out)) {
		if line = strings.TrimSpace(line); line != "" {
			return line[:min(len(line), 300)]
		}
	}
	return ""
}

// checkData prüft im Dev-Mode jede Welt-Datei in cfg.worlds (ohne Dev-Mode liest der Server keine); die übrigen
// Daten hat der Prozess beim Start schon geladen. Exit-Code 0 = gültig, 1 = Fehler (Gründe nach w).
func checkData(cfg config, w io.Writer) int {
	entries, err := os.ReadDir(cfg.worlds)
	if !cfg.dev || err != nil {
		return 0
	}
	code, fsys := 0, os.DirFS(cfg.worlds)
	for _, e := range entries {
		id, ok := strings.CutSuffix(e.Name(), ".json")
		if e.IsDir() || !ok {
			continue
		}
		if _, err := sim.LoadWorld(fsys, id); err != nil {
			_, _ = fmt.Fprintln(w, err)
			code = 1
		}
	}
	return code
}

// roomMemory ist die Ablage der gemerkten Räume (store.Saves); die Naht für den Test.
type roomMemory interface {
	RememberRooms(data []byte) error
	TakeRememberedRooms() ([]byte, error)
}

// rememberRooms merkt sich die offenen Räume vor dem Herunterfahren für einen geplanten Neustart (B-508). Bei einem
// Absturz oder einem Signal gibt es keine gemerkten Räume: nur der geplante Neustart öffnet sie wieder.
func rememberRooms(rooms *room.Manager, mem roomMemory, log *slog.Logger) {
	refs := rooms.OpenRefs()
	if len(refs) == 0 {
		return
	}
	raw, err := json.Marshal(refs)
	if err == nil {
		err = mem.RememberRooms(raw)
	}
	if err != nil {
		log.Error("❌ offene Räume nicht gemerkt, Geräte landen nach dem Neustart in der Raumliste", "ns", "watch", "err", err)
		return
	}
	log.Info("💾 offene Räume für den Neustart gemerkt", "ns", "watch", "raeume", len(refs))
}

// reopenRooms öffnet die gemerkten Räume und löscht die Merkdatei; ohne Datei geschieht nichts.
func reopenRooms(rooms *room.Manager, mem roomMemory, log *slog.Logger) {
	raw, err := mem.TakeRememberedRooms()
	if errors.Is(err, store.ErrNotFound) {
		return
	}
	var refs []room.RoomRef
	if err == nil {
		err = json.Unmarshal(raw, &refs)
	}
	if err != nil {
		log.Warn("🚫 gemerkte Räume nicht lesbar, keine Räume geöffnet", "ns", "watch", "err", err)
		return
	}
	n := rooms.Reopen(refs)
	log.Info("🏰 Räume nach dem Neustart geöffnet", "ns", "watch", "gemerkt", len(refs), "geoeffnet", n)
}
