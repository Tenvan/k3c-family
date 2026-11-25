package world

import (
	"errors"
	"log/slog"
	"os"
	"path/filepath"
	"runtime"
	"syscall"
	"time"
)

// Windows: ERROR_ACCESS_DENIED und ERROR_SHARING_VIOLATION, wenn ein Fremdprozess (Virenscanner, Indexer) die Datei kurz hält.
const (
	errAccessDenied     syscall.Errno = 5
	errSharingViolation syscall.Errno = 32
)

// Wiederholungen beim Ersetzen einer kurz gesperrten Datei (Kopie aus engine/store, B-187): bis zu renameTries Versuche.
const (
	renameTries = 6
	renamePause = 20 * time.Millisecond
)

// renameFile ist os.Rename; die Naht für den Test.
var renameFile = os.Rename

// renameReplace benennt oldPath nach newPath um und ersetzt eine vorhandene Datei. Unter Windows wird ein kurz
// verweigertes Ersetzen begrenzt wiederholt (⏳ im Log, nach dem letzten Versuch ❌); andere Fehler melden sofort.
func renameReplace(oldPath, newPath string) error {
	err := renameFile(oldPath, newPath)
	i := 1
	for ; i < renameTries && retryable(err); i++ {
		slog.Warn("⏳ Datei gesperrt, neuer Versuch", "ns", "world", "datei", filepath.Base(newPath), "versuch", i+1, "err", err)
		time.Sleep(renamePause)
		err = renameFile(oldPath, newPath)
	}
	if i > 1 && err != nil {
		slog.Error("❌ Datei bleibt gesperrt, nicht ersetzt", "ns", "world", "datei", filepath.Base(newPath), "versuche", i, "err", err)
	}
	return err
}

func retryable(err error) bool {
	var errno syscall.Errno
	return runtime.GOOS == "windows" && errors.As(err, &errno) && (errno == errAccessDenied || errno == errSharingViolation)
}
