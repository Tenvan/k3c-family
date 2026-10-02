package services

import (
	"bytes"
	"context"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"testing"
	"time"

	"k3c/tools/k3c-dev/internal/applog"
	"k3c/tools/k3c-dev/internal/console"
	"k3c/tools/k3c-dev/internal/logs"
)

func TestOutLevel(t *testing.T) {
	for _, c := range []struct{ in, want string }{
		{"Build ERROR in x", "ERROR"}, {"Fehler beim Laden", "ERROR"}, {"fatal: nope", "ERROR"},
		{"Warnung: warn", "WARN"}, {"ready in 300 ms", "INFO"}, {"", "INFO"},
	} {
		if got := outLevel(c.in); got != c.want {
			t.Errorf("outLevel(%q) = %s, want %s", c.in, got, c.want)
		}
	}
}

func readOut(t *testing.T, root, name string) []logs.Entry {
	t.Helper()
	res, err := logs.Scan(filepath.Join(root, "logs", name+".jsonl"), logs.Query{})
	if err != nil {
		t.Fatal(err)
	}
	return res.Entries
}

// Die Ausgabe eines Dienstes landet als JSON-Zeile in logs/<name klein>.jsonl, ohne ANSI, mit Strom und Level.
func TestDienstAusgabeInDatei(t *testing.T) {
	root := t.TempDir()
	var buf lockedBuf
	f := &fake{}
	f.healthy.Store(true)
	c := New([]Service{svc("Vite", false)}, Options{Root: root, Console: console.New(0, nil), Log: slog.New(slog.NewTextHandler(&buf, nil)),
		Start: f.start, Check: f.check, Listen: f.listen, Sample: f.sample, KillPID: f.killPID, StartPoll: 2 * time.Millisecond})
	if _, err := c.Start(context.Background(), "Vite"); err != nil {
		t.Fatal(err)
	}
	if _, err := c.Stop(context.Background(), "Vite", false); err != nil {
		t.Fatal(err)
	}
	waitFor(t, "Prozessende geloggt", func() bool { return strings.Contains(buf.String(), "prozess beendet") })
	es := readOut(t, root, "vite")
	if len(es) != 1 || es[0].NS != "out" || es[0].Level != "INFO" || es[0].Msg != "hallo von Vite" || es[0].Data["stream"] != "stdout" {
		t.Errorf("Datei: %+v", es)
	}
	for _, want := range []string{"dienst Vite: start", "command=x", "port=1", "exit=0", "durch=k3c-dev"} {
		if !strings.Contains(buf.String(), want) {
			t.Errorf("Log ohne %q:\n%s", want, buf.String())
		}
	}
}

// Hat der Dienst ein eigenes Log, schreibt k3c-dev keine Ausgabe-Datei.
func TestDienstMitEigenemLogOhneAusgabeDatei(t *testing.T) {
	root := t.TempDir()
	f := &fake{}
	f.healthy.Store(true)
	s := svc("Heimnetz", false)
	s.Log = "k3c-server"
	c := New([]Service{s}, Options{Root: root, Start: f.start, Check: f.check, Listen: f.listen, Sample: f.sample,
		KillPID: f.killPID, StartPoll: 2 * time.Millisecond})
	if _, err := c.Start(context.Background(), "Heimnetz"); err != nil {
		t.Fatal(err)
	}
	_, _ = c.Stop(context.Background(), "Heimnetz", false)
	if _, err := os.Stat(filepath.Join(root, "logs")); err == nil {
		entries, _ := os.ReadDir(filepath.Join(root, "logs"))
		t.Errorf("unerwartete Dateien: %v", entries)
	}
}

func TestOutSinkAnsiRotationUndFehler(t *testing.T) {
	root := t.TempDir()
	dir := filepath.Join(root, "logs")
	_ = os.MkdirAll(dir, 0o755)
	_ = os.WriteFile(filepath.Join(dir, "x.jsonl"), make([]byte, applog.MaxFileSize+1), 0o644)
	s := openOutSink(root, "X", applog.Discard(), time.Now)
	s.write("stderr", "\x1b[31mWARN\x1b[0m rot")
	s.close()
	es := readOut(t, root, "x")
	if len(es) != 1 || es[0].Msg != "WARN rot" || es[0].Level != "WARN" || es[0].Data["stream"] != "stderr" {
		t.Errorf("Eintrag: %+v", es)
	}
	if st, err := os.Stat(filepath.Join(dir, "x.jsonl.1")); err != nil || st.Size() <= applog.MaxFileSize {
		t.Errorf("rotierte Datei: %v, %v", st, err)
	}
	// Ordner nicht anlegbar: nur Warn, kein Absturz.
	file := filepath.Join(root, "datei")
	_ = os.WriteFile(file, nil, 0o644)
	var buf lockedBuf
	bad := openOutSink(file, "Y", slog.New(slog.NewTextHandler(&buf, nil)), time.Now)
	bad.write("stdout", "egal")
	bad.close()
	if !strings.Contains(buf.String(), "Ausgabe-Log nicht geöffnet") {
		t.Errorf("keine Warnung: %q", buf.String())
	}
}

// Wechsel der Gesundheitsprüfung im Lauf stehen mit Grund im Log.
func TestGesundheitswechselGeloggt(t *testing.T) {
	var buf lockedBuf
	f := &fake{}
	f.healthy.Store(true)
	c := New([]Service{svc("Vite", false)}, Options{Console: console.New(0, nil), Log: slog.New(slog.NewTextHandler(&buf, nil)),
		Start: f.start, Check: f.check, Listen: f.listen, Sample: f.sample, KillPID: f.killPID,
		StartPoll: 2 * time.Millisecond, WatchEvery: 5 * time.Millisecond, FailLimit: 100000})
	if _, err := c.Start(context.Background(), "Vite"); err != nil {
		t.Fatal(err)
	}
	f.healthy.Store(false)
	waitFor(t, "ungesund", func() bool { return strings.Contains(buf.String(), "ungesund") && strings.Contains(buf.String(), "antwortet nicht") })
	f.healthy.Store(true)
	waitFor(t, "wieder gesund", func() bool { return strings.Contains(buf.String(), "wieder gesund") })
	_, _ = c.Stop(context.Background(), "Vite", false)
}

// lockedBuf ist ein Puffer, den Log und Test gleichzeitig benutzen dürfen.
type lockedBuf struct {
	mu sync.Mutex
	b  bytes.Buffer
}

func (l *lockedBuf) Write(p []byte) (int, error) { l.mu.Lock(); defer l.mu.Unlock(); return l.b.Write(p) }
func (l *lockedBuf) String() string              { l.mu.Lock(); defer l.mu.Unlock(); return l.b.String() }
