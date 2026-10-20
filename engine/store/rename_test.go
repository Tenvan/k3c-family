package store

import (
	"errors"
	"os"
	"path/filepath"
	"runtime"
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
