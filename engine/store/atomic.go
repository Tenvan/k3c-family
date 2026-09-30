// Package store hält Spielstände und Testberichte als Dateien (Entscheidung 001, engine/store). Geschrieben wird immer
// atomar: erst eine temporäre Datei im selben Ordner, dann umbenennen. Ein unterbrochenes Schreiben lässt den alten
// Stand heil.
package store

import (
	"errors"
	"os"
	"path/filepath"
)

// Fehler, die die HTTP-Schicht in Statuscodes übersetzt.
var (
	ErrInvalid  = errors.New("ungültiger Inhalt")
	ErrNotFound = errors.New("nicht gefunden")
	ErrTooLarge = errors.New("zu groß")
)

// writeAtomic schreibt data nach path über eine temporäre Datei im selben Ordner.
func writeAtomic(path string, data []byte) error {
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	tmp, err := os.CreateTemp(dir, ".tmp-*")
	if err != nil {
		return err
	}
	name := tmp.Name()
	_, err = tmp.Write(data)
	if cerr := tmp.Close(); err == nil {
		err = cerr
	}
	if err == nil {
		err = os.Rename(name, path) // ersetzt unter Windows und Linux eine vorhandene Datei
	}
	if err != nil {
		_ = os.Remove(name)
	}
	return err
}
