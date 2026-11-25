package world

import (
	"errors"
	"os"
	"path/filepath"
	"runtime"
	"syscall"
	"testing"
)

// renameReplace ersetzt eine vorhandene Datei.
func TestRenameReplaceErsetzt(t *testing.T) {
	dir := t.TempDir()
	a, b := filepath.Join(dir, "a"), filepath.Join(dir, "b")
	for p, c := range map[string]string{a: "neu", b: "alt"} {
		if err := os.WriteFile(p, []byte(c), 0o600); err != nil {
			t.Fatal(err)
		}
	}
	if err := renameReplace(a, b); err != nil {
		t.Fatal(err)
	}
	if got, _ := os.ReadFile(b); string(got) != "neu" {
		t.Errorf("Inhalt %q", got)
	}
}

// Unter Windows wird eine kurz gesperrte Datei wiederholt, andere Fehler nicht.
func TestRenameReplaceWiederholt(t *testing.T) {
	orig := renameFile
	t.Cleanup(func() { renameFile = orig })
	n := 0
	renameFile = func(_, _ string) error {
		n++
		if n < 3 {
			return syscall.Errno(errSharingViolation)
		}
		return nil
	}
	err := renameReplace("a", "b")
	if runtime.GOOS == "windows" {
		if err != nil || n != 3 {
			t.Errorf("windows: err %v, %d Versuche", err, n)
		}
	} else if err == nil || n != 1 {
		t.Errorf("sonst: err %v, %d Versuche", err, n)
	}
	other := errors.New("kaputt")
	n = 0
	renameFile = func(_, _ string) error { n++; return other }
	if err := renameReplace("a", "b"); !errors.Is(err, other) || n != 1 {
		t.Errorf("anderer Fehler: %v, %d Versuche", err, n)
	}
}
