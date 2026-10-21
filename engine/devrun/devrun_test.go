package devrun

import (
	"context"
	"log/slog"
	"net"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"
)

// appSource ist das Testprogramm: Es schreibt Start, Stopp (os.Interrupt) und Ende nach <state>/log.txt und beendet
// sich mit dem Code aus <state>/exit, sobald die Datei auftaucht. EXIT_AT_START=<code> beendet es gleich nach dem Start,
// IGNORE_INT=1 lässt es os.Interrupt übergehen.
const appSource = `package main

import (
	"fmt"
	"os"
	"os/signal"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

const version = "%s"

func main() {
	dir := os.Args[1]
	note := func(s string) {
		f, _ := os.OpenFile(filepath.Join(dir, "log.txt"), os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
		fmt.Fprintln(f, version+" "+s)
		_ = f.Close()
	}
	note("start " + os.Getenv("K3C_DEV_CHILD") + os.Getenv("EXTRA"))
	if c := os.Getenv("EXIT_AT_START"); c != "" {
		code, _ := strconv.Atoi(c)
		os.Exit(code)
	}
	sig := make(chan os.Signal, 1)
	signal.Notify(sig, os.Interrupt)
	for {
		select {
		case <-sig:
			if os.Getenv("IGNORE_INT") != "" {
				note("ignoriert")
				continue
			}
			note("stop")
			return
		case <-time.After(20 * time.Millisecond):
		}
		if b, err := os.ReadFile(filepath.Join(dir, "exit")); err == nil {
			_ = os.Remove(filepath.Join(dir, "exit"))
			code, _ := strconv.Atoi(strings.TrimSpace(string(b)))
			note("exit " + strconv.Itoa(code))
			os.Exit(code)
		}
	}
}
`

// recorder sammelt die Log-Meldungen von Run.
type recorder struct {
	mu   sync.Mutex
	msgs []string
}

func (r *recorder) Enabled(context.Context, slog.Level) bool { return true }
func (r *recorder) WithAttrs([]slog.Attr) slog.Handler       { return r }
func (r *recorder) WithGroup(string) slog.Handler            { return r }
func (r *recorder) Handle(_ context.Context, rec slog.Record) error {
	r.mu.Lock()
	defer r.mu.Unlock()
	r.msgs = append(r.msgs, rec.Message)
	return nil
}

func (r *recorder) all() []string {
	r.mu.Lock()
	defer r.mu.Unlock()
	return append([]string(nil), r.msgs...)
}

func (r *recorder) count(prefix string) int {
	r.mu.Lock()
	defer r.mu.Unlock()
	n := 0
	for _, m := range r.msgs {
		if strings.HasPrefix(m, prefix) {
			n++
		}
	}
	return n
}

// app ist ein Dev-Lauf des Testprogramms.
type app struct {
	t     *testing.T
	src   string // Ordner des Programms (Dir)
	state string // Ordner für log.txt und exit
	rec   *recorder
	done  chan error
	stop  context.CancelFunc
	ended bool  // Run ist zurückgekehrt
	err   error // Ergebnis von Run
}

// newAppDirs legt das Testprogramm (Version "a") und den Zustandsordner an; überspringt mit -short.
func newAppDirs(t *testing.T) (src, state string) {
	t.Helper()
	if testing.Short() {
		t.Skip("baut ein Testprogramm mit go build")
	}
	t.Parallel()
	root := t.TempDir()
	src, state = filepath.Join(root, "app"), filepath.Join(root, "state")
	write(t, filepath.Join(src, "go.mod"), "module devrunapp\n\ngo 1.22\n")
	write(t, filepath.Join(src, "main.go"), strings.Replace(appSource, "%s", "a", 1))
	write(t, filepath.Join(state, ".keep"), "")
	return src, state
}

// startApp startet einen Dev-Lauf des Testprogramms; env ergänzt die Umgebung des Kindes.
func startApp(t *testing.T, restartCode int, env ...string) *app {
	t.Helper()
	src, state := newAppDirs(t)
	a := &app{t: t, src: src, state: state, rec: &recorder{}, done: make(chan error, 1)}
	ctx, cancel := context.WithCancel(context.Background())
	a.stop = cancel
	t.Cleanup(func() { _ = a.cancel() })
	cfg := Config{Name: "test", Dir: a.src, Package: ".", Child: filepath.Join("bin", "app-dev.exe"), Args: []string{a.state},
		Env: append([]string{"EXTRA=x"}, env...), Watch: []string{"."}, RestartCode: restartCode, Grace: 5 * time.Second, Log: slog.New(a.rec)}
	go func() { a.done <- Run(ctx, cfg) }()
	return a
}

// version schreibt das Programm mit neuer Version (andere Länge: die Größe ändert sich sicher mit).
func (a *app) version(v string) {
	write(a.t, filepath.Join(a.src, "main.go"), strings.Replace(appSource, "%s", v, 1))
}

// cancel beendet den Dev-Lauf und liefert das Ergebnis von Run (mehrfach aufrufbar).
func (a *app) cancel() error {
	a.stop()
	if a.ended {
		return a.err
	}
	select {
	case a.err = <-a.done:
		a.ended = true
	case <-time.After(20 * time.Second):
		a.t.Fatalf("Run endet nicht; Programm-Log %q, Run-Log %q", a.lines(), a.rec.all())
	}
	return a.err
}

func (a *app) lines() []string {
	b, _ := os.ReadFile(filepath.Join(a.state, "log.txt"))
	return strings.Split(strings.TrimSpace(string(b)), "\n")
}

func (a *app) has(line string, n int) bool { return a.count(line) >= n }

func (a *app) count(line string) int {
	c := 0
	for _, l := range a.lines() {
		if l == line {
			c++
		}
	}
	return c
}

// waitFor wartet, bis cond gilt.
func (a *app) waitFor(what string, cond func() bool) {
	a.t.Helper()
	deadline := time.Now().Add(60 * time.Second)
	for !cond() {
		if time.Now().After(deadline) {
			a.t.Fatalf("%s: kam nicht; Programm-Log %q, Run-Log %q", what, a.lines(), a.rec.all())
		}
		time.Sleep(50 * time.Millisecond)
	}
}

func (a *app) waitLine(line string, n int) {
	a.t.Helper()
	a.waitFor(line, func() bool { return a.has(line, n) })
}

// Bauen und starten, bei Änderung neu bauen und sauber tauschen, ein Build-Fehler lässt das Kind stehen, ctx beendet sauber.
func TestRunBautNeuUndBehaeltKindBeiBuildFehler(t *testing.T) {
	a := startApp(t, 0)
	a.waitLine("a start 1x", 1)

	a.version("bb")
	a.waitLine("bb start 1x", 1)
	if !a.has("a stop", 1) {
		t.Fatalf("altes Kind nicht sauber beendet (os.Interrupt): %q", a.lines())
	}

	write(t, filepath.Join(a.src, "main.go"), "package main\n\nfunc main() { kaputt }\n")
	a.waitFor("❌-Log", func() bool { return a.rec.count("❌") > 0 })
	time.Sleep(300 * time.Millisecond)
	if a.has("bb stop", 1) {
		t.Fatalf("Build-Fehler hat das laufende Kind beendet: %q", a.lines())
	}

	a.version("ccc")
	a.waitLine("ccc start 1x", 1)
	if err := a.cancel(); err != nil {
		t.Fatalf("Run = %v", err)
	}
	if !a.has("ccc stop", 1) {
		t.Fatalf("Kind beim Ende von ctx nicht sauber beendet: %q", a.lines())
	}
}

// RestartCode startet ohne Neubau neu, ein anderer Code wartet auf die nächste Änderung.
func TestRunStartetBeiRestartCodeNeu(t *testing.T) {
	a := startApp(t, 75)
	a.waitLine("a start 1x", 1)

	write(t, filepath.Join(a.state, "exit"), "75")
	a.waitLine("a start 1x", 2)
	if n := a.rec.count("🔨"); n != 1 {
		t.Fatalf("Neustart mit RestartCode hat neu gebaut: %d Builds", n)
	}

	write(t, filepath.Join(a.state, "exit"), "3")
	a.waitLine("a exit 3", 1)
	time.Sleep(restartPause + 500*time.Millisecond)
	if a.has("a start 1x", 3) {
		t.Fatalf("Kind nach Exit-Code 3 neu gestartet: %q", a.lines())
	}

	a.version("bb")
	a.waitLine("bb start 1x", 1)
}

func TestListenWartetAufFreienPort(t *testing.T) {
	busy, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatal(err)
	}
	addr := busy.Addr().String()
	if _, err := Listen("tcp", addr, 200*time.Millisecond); err == nil {
		t.Fatal("belegte Adresse: Fehler erwartet")
	}
	go func() {
		time.Sleep(300 * time.Millisecond)
		_ = busy.Close()
	}()
	ln, err := Listen("tcp", addr, 5*time.Second)
	if err != nil {
		t.Fatalf("Listen nach Freigabe = %v", err)
	}
	_ = ln.Close()
}

func TestSnapshotNimmtNurGoQuellen(t *testing.T) {
	dir := t.TempDir()
	for _, f := range []string{"a.go", "a_test.go", "go.mod", "x.html", "note.md", ".git/b.go", "frontend/c.go",
		"node_modules/d.go", "bin/e.go", "build/f.go", "sub/g.go"} {
		write(t, filepath.Join(dir, f), "x")
	}
	got := snapshot([]string{dir, filepath.Join(dir, "note.md"), filepath.Join(dir, "fehlt")}, []string{".go", ".html"})
	want := []string{"a.go", "go.mod", "x.html", "note.md", "sub/g.go"}
	if len(got) != len(want) {
		t.Fatalf("snapshot = %v, will %v", got, want)
	}
	for _, w := range want {
		if _, ok := got[filepath.ToSlash(filepath.Join(dir, w))]; !ok {
			t.Errorf("%s fehlt in %v", w, got)
		}
	}
}

func TestTempPath(t *testing.T) {
	if got := tempPath(filepath.Join("bin", "x-dev.exe")); got != filepath.Join("bin", "x-dev.new.exe") {
		t.Errorf("tempPath = %q", got)
	}
}

func write(t *testing.T, path, text string) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		t.Fatal(err)
	}
	// atomar: das Testprogramm liest die Datei jederzeit und darf sie nie leer sehen
	tmp := path + ".tmp"
	if err := os.WriteFile(tmp, []byte(text), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.Rename(tmp, path); err != nil {
		t.Fatal(err)
	}
}

func waitUntil(t *testing.T, what string, cond func() bool) {
	t.Helper()
	deadline := time.Now().Add(60 * time.Second)
	for !cond() {
		if time.Now().After(deadline) {
			t.Fatalf("%s: kam nicht", what)
		}
		time.Sleep(50 * time.Millisecond)
	}
}

func TestFreshNurWennDieEXENeuerIst(t *testing.T) {
	dir := t.TempDir()
	src, exe := filepath.Join(dir, "a.go"), filepath.Join(dir, "app.exe")
	write(t, src, "package main")
	if fresh(exe, snapshot([]string{dir}, []string{".go"})) {
		t.Error("ohne EXE ist nichts aktuell")
	}
	write(t, exe, "x")
	past := time.Now().Add(-time.Hour)
	if err := os.Chtimes(src, past, past); err != nil {
		t.Fatal(err)
	}
	if !fresh(exe, snapshot([]string{dir}, []string{".go"})) {
		t.Error("EXE neuer als die Quelle muss als aktuell gelten")
	}
	future := time.Now().Add(time.Hour)
	if err := os.Chtimes(src, future, future); err != nil {
		t.Fatal(err)
	}
	if fresh(exe, snapshot([]string{dir}, []string{".go"})) {
		t.Error("eine neuere Quelle verlangt einen Build")
	}
	if fresh(exe, nil) {
		t.Error("ohne beobachtete Dateien wird gebaut")
	}
}
