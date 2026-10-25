// Package store hält Spielstände und Testberichte als Dateien (Entscheidung 001, engine/store). Geschrieben wird immer
// atomar: erst eine temporäre Datei im selben Ordner, dann umbenennen. Ein unterbrochenes Schreiben lässt den alten
// Stand heil.
package store

import (
	"errors"
	"log/slog"
	"os"
	"path/filepath"
	"runtime"
	"syscall"
	"time"
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
		err = Rename(name, path) // ersetzt unter Windows und Linux eine vorhandene Datei
	}
	if err != nil {
		_ = os.Remove(name)
	}
	return err
}

// Windows: ERROR_ACCESS_DENIED und ERROR_SHARING_VIOLATION, wenn ein Fremdprozess (Virenscanner, Indexer) die Datei kurz hält.
const (
	errAccessDenied     syscall.Errno = 5
	errSharingViolation syscall.Errno = 32
)

// Wiederholungen beim Ersetzen einer kurz gesperrten Datei (B-187): bis zu renameTries Versuche im Abstand renamePause.
const (
	renameTries = 6
	renamePause = 20 * time.Millisecond
)

// renameFile ist os.Rename; die Naht für den Test.
var renameFile = os.Rename

// Rename benennt oldPath nach newPath um und ersetzt eine vorhandene Datei. Unter Windows wird ein kurz verweigertes
// Ersetzen begrenzt wiederholt (⏳ im Log, nach dem letzten Versuch ❌); andere Fehler und andere Systeme melden den
// Fehler sofort. Scheitert es, bleibt newPath unverändert.
func Rename(oldPath, newPath string) error {
	err := renameFile(oldPath, newPath)
	i := 1
	for ; i < renameTries && retryable(err); i++ {
		slog.Warn("⏳ Datei gesperrt, neuer Versuch", "ns", "store", "datei", filepath.Base(newPath), "versuch", i+1, "err", err)
		time.Sleep(renamePause)
		err = renameFile(oldPath, newPath)
	}
	if i > 1 && err != nil {
		slog.Error("❌ Datei bleibt gesperrt, nicht ersetzt", "ns", "store", "datei", filepath.Base(newPath), "versuche", i, "err", err)
	}
	return err
}

func retryable(err error) bool {
	var errno syscall.Errno
	return runtime.GOOS == "windows" && errors.As(err, &errno) && (errno == errAccessDenied || errno == errSharingViolation)
}
