package watch

import (
	"context"
	"io/fs"
	"os"
	"path/filepath"

	"github.com/fsnotify/fsnotify"
)

// Notify meldet Datei-Ereignisse der Roots mit Dir über das Betriebssystem (ReadDirectoryChangesW, inotify). Der Kanal
// bekommt ein Signal (gepuffert, nie blockierend) bei jeder Änderung außer an Temp-Dateien und Papierschnitt-Specs;
// neue Unterordner werden nachträglich beobachtet. Fehler beim Einrichten: nil und der Fehler, der Aufrufer fragt dann
// wie bisher im Takt ab. Das Signal ersetzt nicht den Vergleich in Poll, es sagt nur, wann er sich lohnt.
func Notify(ctx context.Context, roots []Root) (<-chan struct{}, error) {
	nw, err := fsnotify.NewWatcher()
	if err != nil {
		return nil, err
	}
	n := 0
	for _, r := range roots {
		if r.Dir != "" {
			n += addTree(nw, r.Dir)
		}
	}
	if n == 0 {
		_ = nw.Close()
		return nil, os.ErrNotExist
	}
	out := make(chan struct{}, 1)
	go func() {
		defer func() { _ = nw.Close() }()
		for {
			select {
			case <-ctx.Done():
				return
			case err, ok := <-nw.Errors:
				if !ok {
					return
				}
				_ = err
				signal(out) // Überlauf o. Ä.: sicherheitshalber abfragen
			case ev, ok := <-nw.Events:
				if !ok {
					return
				}
				if ev.Has(fsnotify.Create) {
					if st, err := os.Stat(ev.Name); err == nil && st.IsDir() {
						addTree(nw, ev.Name)
						signal(out)
						continue
					}
				}
				if !isTemp(filepath.Base(ev.Name)) {
					signal(out)
				}
			}
		}
	}()
	return out, nil
}

func signal(c chan<- struct{}) {
	select {
	case c <- struct{}{}:
	default:
	}
}

// addTree beobachtet dir und alle Unterordner; liefert die Zahl der Ordner.
func addTree(nw *fsnotify.Watcher, dir string) int {
	n := 0
	_ = filepath.WalkDir(dir, func(p string, d fs.DirEntry, err error) error {
		if err == nil && d.IsDir() && nw.Add(p) == nil {
			n++
		}
		return nil
	})
	return n
}
