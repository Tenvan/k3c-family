package store

import (
	"bytes"
	"errors"
	"log/slog"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"syscall"
	"testing"
)

// B-187: Ein kurz verweigertes Ersetzen wird wiederholt (nur unter Windows), andere Fehler nicht.
func TestRenameWiederholtVerweigertesErsetzen(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("Wiederholung gilt nur unter Windows")
	}
	calls := 0
	defer func(f func(string, string) error) { renameFile = f }(renameFile)
	renameFile = func(a, b string) error {
		calls++
		if calls < 3 {
			return &os.LinkError{Op: "rename", Old: a, New: b, Err: errAccessDenied}
		}
		return nil
	}
	if err := Rename("a", "b"); err != nil || calls != 3 {
		t.Fatalf("err=%v, Versuche=%d, erwartet gelungen im dritten", err, calls)
	}
}

func TestRenameGibtDauerhaftenFehlerNachDenVersuchenZurueck(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("Wiederholung gilt nur unter Windows")
	}
	calls := 0
	defer func(f func(string, string) error) { renameFile = f }(renameFile)
	renameFile = func(a, b string) error { calls++; return &os.LinkError{Op: "rename", Err: errSharingViolation} }
	if err := Rename("a", "b"); !errors.Is(err, errSharingViolation) || calls != renameTries {
		t.Fatalf("err=%v, Versuche=%d, erwartet %d", err, calls, renameTries)
	}
}

func TestRenameAndereFehlerSofort(t *testing.T) {
	calls := 0
	defer func(f func(string, string) error) { renameFile = f }(renameFile)
	renameFile = func(a, b string) error { calls++; return syscall.ENOENT }
	if err := Rename("a", "b"); err == nil || calls != 1 {
		t.Fatalf("err=%v, Versuche=%d, erwartet ein Versuch", err, calls)
	}
}

func TestRenameErsetztVorhandeneDatei(t *testing.T) {
	dir := t.TempDir()
	a, b := filepath.Join(dir, "a"), filepath.Join(dir, "b")
	_ = os.WriteFile(a, []byte("neu"), 0o644)
	_ = os.WriteFile(b, []byte("alt"), 0o644)
	if err := Rename(a, b); err != nil {
		t.Fatal(err)
	}
	if got, _ := os.ReadFile(b); string(got) != "neu" {
		t.Errorf("Inhalt %q", got)
	}
}

// ST1.2 (B-187/AC-01): Eine gesperrte Zieldatei lässt writeAtomic wiederholen (⏳ im Log); bleibt sie gesperrt, meldet
// writeAtomic den Fehler (❌), der alte Stand bleibt Byte für Byte und keine temporäre Datei liegt herum.
func TestWriteAtomicGesperrteZieldatei(t *testing.T) {
	if runtime.GOOS != "windows" {
		t.Skip("Wiederholung gilt nur unter Windows")
	}
	dir := t.TempDir()
	path := filepath.Join(dir, "autosave.json")
	_ = os.WriteFile(path, []byte("alt"), 0o644)
	buf := &bytes.Buffer{}
	defer func(l *slog.Logger) { slog.SetDefault(l) }(slog.Default())
	slog.SetDefault(slog.New(slog.NewTextHandler(buf, nil)))
	locked := 0 // so viele Versuche ist die Datei noch gesperrt
	defer func(f func(string, string) error) { renameFile = f }(renameFile)
	renameFile = func(a, b string) error {
		if locked > 0 {
			locked--
			return &os.LinkError{Op: "rename", Old: a, New: b, Err: errSharingViolation}
		}
		return os.Rename(a, b)
	}

	locked = renameTries
	if err := writeAtomic(path, []byte("neu")); !errors.Is(err, errSharingViolation) {
		t.Fatalf("dauerhaft gesperrt: err=%v", err)
	}
	if got, _ := os.ReadFile(path); string(got) != "alt" {
		t.Fatalf("alter Stand verändert: %q", got)
	}
	if tmp, _ := filepath.Glob(filepath.Join(dir, ".tmp-*")); len(tmp) != 0 {
		t.Fatalf("temporäre Dateien liegen herum: %v", tmp)
	}
	if !strings.Contains(buf.String(), "⏳") || !strings.Contains(buf.String(), "❌") {
		t.Fatalf("Log ohne ⏳ und ❌:\n%s", buf.String())
	}

	locked = 2
	if err := writeAtomic(path, []byte("neu")); err != nil {
		t.Fatalf("kurz gesperrt: %v", err)
	}
	if got, _ := os.ReadFile(path); string(got) != "neu" {
		t.Fatalf("neuer Stand fehlt: %q", got)
	}
}
