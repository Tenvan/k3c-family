package gameexe

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// BuildCommand baut die Spiel-EXE; die Fehler bei fehlender oder veralteter EXE nennen ihn.
const BuildCommand = "task build"

// candidates sind die Orte der EXE, zuerst der Build im Checkout, dann das Spiel-Paket; dir ist der Arbeitsordner, unter
// dem das Spiel public/ findet. Pfade relativ zur Wurzel des Checkouts.
var candidates = []struct{ exe, dir string }{
	{"rust/target/release/k3c-game.exe", "."},
	{"dist/k3c-game/k3c-game.exe", "dist/k3c-game"},
}

// Locate liefert die erste vorhandene EXE im Checkout root und ihren Arbeitsordner, ohne die Frische zu prüfen
// („Im Spiel ansehen“ nimmt sie so). Ohne EXE ein Fehler mit dem Bau-Befehl.
func Locate(root string) (exe, dir string, err error) {
	for _, c := range candidates {
		exe = filepath.Join(root, filepath.FromSlash(c.exe))
		if st, err := os.Stat(exe); err == nil && !st.IsDir() {
			return exe, filepath.Join(root, filepath.FromSlash(c.dir)), nil
		}
	}
	return "", "", fmt.Errorf("natives Spiel fehlt (%s) – erst bauen: %s", candidates[0].exe, BuildCommand)
}

// Find liefert die erste frische EXE (siehe Stale). Ist keine frisch, kommt ein Fehler mit dem Bau-Befehl und es wird nicht
// gerechnet; fehlt jede EXE, ebenso.
func Find(root string) (exe, dir string, err error) {
	var staleErr error
	for _, c := range candidates {
		e := filepath.Join(root, filepath.FromSlash(c.exe))
		if st, err := os.Stat(e); err != nil || st.IsDir() {
			continue
		}
		src, stale := Stale(root, e)
		if !stale {
			return e, filepath.Join(root, filepath.FromSlash(c.dir)), nil
		}
		if staleErr == nil {
			staleErr = fmt.Errorf("natives Spiel ist veraltet (%s ist jünger als %s) – erst bauen: %s",
				filepath.ToSlash(src), filepath.ToSlash(c.exe), BuildCommand)
		}
	}
	if staleErr != nil {
		return "", "", staleErr
	}
	_, _, err = Locate(root)
	return "", "", err
}

// Stale meldet, ob eine Datei unter rust/crates (*.rs, Cargo.toml) oder rust/Cargo.lock jünger ist als die EXE (mtime,
// erfasst auch nicht committete Änderungen, kein git-Aufruf), und nennt die jüngste Datei relativ zum Checkout.
func Stale(root, exe string) (newest string, stale bool) {
	st, err := os.Stat(exe)
	if err != nil {
		return "", false
	}
	built := st.ModTime()
	var latest time.Time
	consider := func(path string, mt time.Time) {
		if mt.After(built) && mt.After(latest) {
			latest, newest = mt, path
		}
	}
	lock := filepath.Join(root, "rust", "Cargo.lock")
	if lst, err := os.Stat(lock); err == nil {
		consider(lock, lst.ModTime())
	}
	_ = filepath.WalkDir(filepath.Join(root, "rust", "crates"), func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		if d.IsDir() {
			if d.Name() == "target" {
				return filepath.SkipDir
			}
			return nil
		}
		if name := d.Name(); strings.HasSuffix(name, ".rs") || name == "Cargo.toml" {
			if info, err := d.Info(); err == nil {
				consider(path, info.ModTime())
			}
		}
		return nil
	})
	if newest == "" {
		return "", false
	}
	if rel, err := filepath.Rel(root, newest); err == nil {
		newest = rel
	}
	return newest, true
}
