package planning

import (
	"context"
	"hash/fnv"
	"io/fs"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

// Stamp ist ein Fingerabdruck aller Planungsdateien: Pfad, Größe und Änderungszeit der Sprint-Ordner, Tickets und
// Dokumente. Ändert sich irgendetwas daran, ändert sich der Wert; Dateien außerhalb der Planung zählen nicht.
// Gelesen werden nur Metadaten, nie Inhalte.
func Stamp(root string) string {
	h := fnv.New64a()
	add := func(path string, info fs.FileInfo) {
		rel, _ := filepath.Rel(root, path)
		_, _ = h.Write([]byte(filepath.ToSlash(rel) + "|" + strconv.FormatInt(info.Size(), 10) + "|" +
			strconv.FormatInt(info.ModTime().UnixNano(), 10) + "\n"))
	}
	// Verzeichnisse zählen nur über ihren Namen: ihre Änderungszeit ist unter Windows unzuverlässig (sie zieht verspätet nach).
	name := func(path string) {
		rel, _ := filepath.Rel(root, path)
		_, _ = h.Write([]byte(filepath.ToSlash(rel) + "/\n"))
	}
	docs := filepath.Join(root, "docs")
	for _, dir := range []string{"sprints/aktiv", "sprints/geplant", "sprints/erledigt", "backlog"} {
		_ = filepath.WalkDir(filepath.Join(docs, filepath.FromSlash(dir)), func(path string, d fs.DirEntry, err error) error {
			if err != nil {
				return nil
			}
			sub := d.IsDir() && path != filepath.Join(docs, filepath.FromSlash(dir))
			if sub && dir == "backlog" {
				return filepath.SkipDir // archiv/ ändert die Anzeige nicht
			}
			if sub && strings.HasSuffix(dir, "erledigt") {
				name(path) // nur die Zahl der erledigten Sprints zählt, nicht ihr Inhalt
				return filepath.SkipDir
			}
			if info, err := d.Info(); err == nil && strings.HasSuffix(path, ".md") {
				add(path, info)
			}
			return nil
		})
	}
	for _, file := range Docs {
		if info, err := os.Stat(filepath.Join(docs, file)); err == nil {
			add(filepath.Join(docs, file), info)
		}
	}
	return strconv.FormatUint(h.Sum64(), 16)
}

// Watch prüft im Takt every, ob sich Stamp ändert, und ruft dann onChange. Der Aufruf direkt nach dem Start gilt als
// Ausgangsstand und löst nichts aus. Endet ctx, endet die Goroutine.
//
// ponytail: Polling statt Dateisystem-Ereignissen; ein Stat über ein paar hundert Dateien alle 2 s kostet nichts, und
// fsnotify wäre eine neue Abhängigkeit. Bei sehr großem Backlog den Takt erhöhen.
func Watch(ctx context.Context, root string, every time.Duration, onChange func()) {
	last := Stamp(root)
	go func() {
		t := time.NewTicker(every)
		defer t.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-t.C:
				if cur := Stamp(root); cur != last {
					last = cur
					onChange()
				}
			}
		}
	}()
}
