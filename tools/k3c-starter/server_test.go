package main

import (
	"net"
	"net/http"
	"os"
	"os/signal"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
	"time"
)

// TestMain macht aus dem Testprogramm einen falschen k3c-server: Mit K3C_FAKE_SERVER=1 antwortet es auf Loopback
// unter /api/health, beendet sich bei Strg+C sauber (wie der echte Server) und sofort mit K3C_FAKE_EXIT=1. Der Test startet es als „Server-EXE“, so läuft nichts
// außerhalb von 127.0.0.1 und keine echte Server-EXE.
func TestMain(m *testing.M) {
	if os.Getenv("K3C_FAKE_SERVER") == "1" {
		if os.Getenv("K3C_FAKE_EXIT") == "1" {
			println("falscher Server: Abbruch")
			os.Exit(3)
		}
		mux := http.NewServeMux()
		mux.HandleFunc("/api/health", func(w http.ResponseWriter, _ *http.Request) { _, _ = w.Write([]byte("ok")) })
		stop := make(chan os.Signal, 1)
		signal.Notify(stop, os.Interrupt)
		go func() { _ = http.ListenAndServe("127.0.0.1:"+os.Getenv("K3C_HTTP_PORT"), mux) }()
		println("falscher Server läuft")
		<-stop
		println("falscher Server: sauber beendet")
		os.Exit(0)
	}
	os.Exit(m.Run())
}

// freePort liefert einen freien Loopback-Port.
func freePort(t *testing.T) string {
	t.Helper()
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = l.Close() }()
	_, port, _ := net.SplitHostPort(l.Addr().String())
	return port
}

func fakeSupervisor(t *testing.T, port string) *supervisor {
	t.Helper()
	exe, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	t.Setenv("K3C_FAKE_SERVER", "1")
	t.Setenv("K3C_HTTP_PORT", port)
	s := newSupervisor(layout{root: t.TempDir(), serverExe: exe, dist: t.TempDir()})
	t.Cleanup(s.Stop)
	return s
}

func waitState(t *testing.T, s *supervisor, ok func(Status) bool) Status {
	t.Helper()
	deadline := time.Now().Add(20 * time.Second)
	for time.Now().Before(deadline) {
		if st := s.State(); ok(st) {
			return st
		}
		time.Sleep(50 * time.Millisecond)
	}
	t.Fatalf("Zustand nicht erreicht: %+v", s.State())
	return Status{}
}

// B-041/AC-01: Der Starter startet den Server, zeigt „läuft“ mit Adresse und beendet ihn wieder.
func TestStartetUndBeendetServer(t *testing.T) {
	port := freePort(t)
	s := fakeSupervisor(t, port)
	if st := s.State(); st.Running || len(st.URLs) != 0 {
		t.Fatalf("vor dem Start: %+v", st)
	}
	s.Start()
	st := waitState(t, s, func(st Status) bool { return st.Running })
	if st.Port != port || len(st.URLs) == 0 || st.URLs[0] != "http://localhost:"+port+"/" {
		t.Fatalf("Adressen: %+v", st)
	}
	for _, u := range st.URLs {
		if !strings.HasSuffix(u, ":"+port+"/") {
			t.Errorf("Adresse ohne Port: %s", u)
		}
	}
	if res, err := http.Get("http://127.0.0.1:" + port + "/api/health"); err != nil || res.StatusCode != 200 {
		t.Fatalf("Server antwortet nicht: %v", err)
	}
	waitState(t, s, func(st Status) bool { return len(st.Log) > 0 }) // Ausgabe des Servers kommt im Fenster an
	s.Stop()
	assertCleanStop(t, s, port)
}

// assertCleanStop: Der Server hat das Signal erhalten und sich selbst beendet (kein harter Abbruch), der Port ist frei.
func assertCleanStop(t *testing.T, s *supervisor, port string) {
	t.Helper()
	if log := strings.Join(s.State().Log, "|"); !strings.Contains(log, "sauber beendet") || strings.Contains(log, "Hinweis") {
		t.Fatalf("Stop war nicht sauber (Signal kam nicht an): %s", log)
	}
	if st := s.State(); st.Running || st.Starting || st.Message != "Server ist gestoppt." {
		t.Fatalf("nach Stop: %+v", st)
	}
	if _, err := http.Get("http://127.0.0.1:" + port + "/api/health"); err == nil {
		t.Fatal("Server antwortet nach Stop noch")
	}
}

// B-041/AC-02: Port belegt → Meldung im Fenster statt Absturz, kein Prozess.
func TestPortBelegtMeldet(t *testing.T) {
	l, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = l.Close() }()
	_, port, _ := net.SplitHostPort(l.Addr().String())
	s := fakeSupervisor(t, port)
	s.Start()
	st := s.State()
	if st.Running || st.Starting || !strings.Contains(st.Message, "belegt") || !strings.Contains(st.Message, port) {
		t.Fatalf("Zustand: %+v", st)
	}
}

// Beendet sich der Server von selbst, nennt das Fenster es samt Meldungen.
func TestServerBeendetSichUnerwartet(t *testing.T) {
	port := freePort(t)
	s := fakeSupervisor(t, port)
	t.Setenv("K3C_FAKE_EXIT", "1")
	s.Start()
	st := waitState(t, s, func(st Status) bool { return strings.Contains(st.Message, "unerwartet") })
	if st.Running || !strings.Contains(st.Message, "exit status 3") {
		t.Fatalf("Zustand: %+v", st)
	}
	waitState(t, s, func(st Status) bool { return strings.Contains(strings.Join(st.Log, "\n"), "Abbruch") })
}

func TestStartOhneServerExeMeldet(t *testing.T) {
	s := newSupervisor(layout{root: t.TempDir(), serverExe: filepath.Join(t.TempDir(), "gibt-es-nicht"), dist: t.TempDir()})
	s.port = freePort(t)
	s.Start()
	if st := s.State(); st.Running || !strings.Contains(st.Message, "startet nicht") {
		t.Fatalf("Zustand: %+v", st)
	}
}

func TestFindLayout(t *testing.T) {
	name := "k3c-server"
	if runtime.GOOS == "windows" {
		name += ".exe"
	}
	root := filepath.Clean(t.TempDir()) // unter task steht TMP mit Schrägstrichen: Join bereinigt sie
	mustWrite := func(path string) {
		t.Helper()
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(path, []byte("x"), 0o644); err != nil {
			t.Fatal(err)
		}
	}
	mustWrite(filepath.Join(root, "dist", "index.html"))
	if _, err := findLayout(root, filepath.Join(root, "bin")); err == nil || !strings.Contains(err.Error(), name) {
		t.Fatalf("ohne Server-EXE: %v", err)
	}
	mustWrite(filepath.Join(root, "bin", name))
	// Doppelklick auf bin/k3c-starter.exe: Wurzel ist der Ordner über bin/, die Server-EXE liegt daneben
	lay, err := findLayout(t.TempDir(), filepath.Join(root, "bin"))
	if err != nil || lay.root != root || lay.dist != filepath.Join(root, "dist") || filepath.Base(lay.serverExe) != name {
		t.Fatalf("Layout: %+v, %v", lay, err)
	}
	// Start aus der Wurzel
	if lay, err = findLayout(root, t.TempDir()); err != nil || lay.root != root {
		t.Fatalf("Layout aus der Wurzel: %+v, %v", lay, err)
	}
	if _, err := findLayout(t.TempDir(), t.TempDir()); err == nil || !strings.Contains(err.Error(), "dist/index.html") {
		t.Fatalf("ohne dist: %v", err)
	}
}

// Ohne gefundenes Layout öffnet das Fenster trotzdem und nennt den Grund.
func TestAppOhneLayoutNenntGrund(t *testing.T) {
	a := newApp(layout{}, os.ErrNotExist)
	a.Start()
	a.Stop()
	if st := a.State(); st.Running || !strings.Contains(st.Message, "does not exist") && !strings.Contains(st.Message, "not exist") {
		t.Fatalf("Zustand: %+v", st)
	}
}

func TestLineWriterZerlegtZeilen(t *testing.T) {
	var got []string
	w := &lineWriter{sink: func(l string) { got = append(got, l) }}
	_, _ = w.Write([]byte("eins\r\nzw"))
	_, _ = w.Write([]byte("ei\n\ndrei\n"))
	if strings.Join(got, "|") != "eins|zwei|drei" {
		t.Fatalf("Zeilen: %q", got)
	}
}
