package gameexe

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func write(t *testing.T, path string, mt time.Time) {
	t.Helper()
	if err := os.MkdirAll(filepath.Dir(path), 0o750); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(path, []byte("x"), 0o600); err != nil {
		t.Fatal(err)
	}
	if err := os.Chtimes(path, mt, mt); err != nil {
		t.Fatal(err)
	}
}

func TestSuchlisteReihenfolge(t *testing.T) {
	root := t.TempDir()
	now := time.Now()
	if _, _, err := Locate(root); err == nil || !strings.Contains(err.Error(), BuildCommand) {
		t.Errorf("ohne EXE: %v, will Hinweis auf %q", err, BuildCommand)
	}
	pkg := filepath.Join(root, "dist", "k3c-game", "k3c-game.exe")
	write(t, pkg, now)
	exe, dir, err := Locate(root)
	if err != nil || exe != pkg || dir != filepath.Dir(pkg) {
		t.Errorf("Paket: %q %q %v", exe, dir, err)
	}
	build := filepath.Join(root, "rust", "target", "release", "k3c-game.exe")
	write(t, build, now)
	exe, dir, err = Locate(root)
	if err != nil || exe != build || dir != filepath.Join(root, ".") {
		t.Errorf("Build hat Vorrang: %q %q %v", exe, dir, err)
	}
}

func TestFrische(t *testing.T) {
	root := t.TempDir()
	built := time.Now().Add(-time.Hour)
	exe := filepath.Join(root, "rust", "target", "release", "k3c-game.exe")
	write(t, exe, built)
	write(t, filepath.Join(root, "rust", "crates", "a", "src", "lib.rs"), built.Add(-time.Minute))
	write(t, filepath.Join(root, "rust", "crates", "a", "Cargo.toml"), built.Add(-time.Minute))
	write(t, filepath.Join(root, "rust", "Cargo.lock"), built.Add(-time.Minute))
	write(t, filepath.Join(root, "rust", "crates", "a", "README.md"), built.Add(time.Hour)) // zählt nicht
	if _, _, err := Find(root); err != nil {
		t.Fatalf("frische EXE: %v", err)
	}
	for _, f := range []string{"rust/crates/a/src/lib.rs", "rust/crates/a/Cargo.toml", "rust/Cargo.lock"} {
		p := filepath.Join(root, filepath.FromSlash(f))
		write(t, p, built.Add(time.Minute))
		_, _, err := Find(root)
		if err == nil || !strings.Contains(err.Error(), f) || !strings.Contains(err.Error(), BuildCommand) {
			t.Errorf("%s jünger: Fehler %v, will Datei und Bau-Befehl", f, err)
		}
		write(t, p, built.Add(-time.Minute))
	}
}

func TestFindNimmtFrischenKandidaten(t *testing.T) {
	root := t.TempDir()
	now := time.Now()
	write(t, filepath.Join(root, "rust", "target", "release", "k3c-game.exe"), now.Add(-time.Hour))
	pkg := filepath.Join(root, "dist", "k3c-game", "k3c-game.exe")
	write(t, pkg, now)
	write(t, filepath.Join(root, "rust", "crates", "a", "lib.rs"), now.Add(-time.Minute))
	exe, _, err := Find(root)
	if err != nil || exe != pkg {
		t.Errorf("Find = %q, %v, will das frische Paket", exe, err)
	}
}
