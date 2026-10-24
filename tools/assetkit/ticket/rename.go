package ticket

import (
	"errors"
	"io/fs"
	"os"
	"time"
)

// renameTries und renamePause: Unter Windows hält ein Virenscanner oder der Suchindex eine frisch geschriebene Datei
// kurz offen; os.Rename meldet dann „Access is denied“. Ein paar kurze Wiederholungen genügen.
const (
	renameTries = 5
	renamePause = 40 * time.Millisecond
)

// Rename ersetzt to durch from wie os.Rename, wiederholt aber bei einer Zugriffssperre; scheitert es, wird from
// gelöscht, damit keine Temp-Datei liegen bleibt.
func Rename(from, to string) error {
	var err error
	for i := range renameTries {
		if err = os.Rename(from, to); err == nil || !errors.Is(err, fs.ErrPermission) {
			break
		}
		time.Sleep(renamePause * time.Duration(i+1))
	}
	if err != nil {
		_ = os.Remove(from)
	}
	return err
}
