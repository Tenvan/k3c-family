// Package watch beobachtet Ordner, die das Asset-Studio schreibt (data/**, Grafiken, Sound; B-507), durch Abfragen
// in festem Takt: Größe und Änderungszeit jeder Datei. Mehrere Änderungen kurz hintereinander werden entprellt und
// ergeben einen Vorgang. Uhr und Dateisystem sind eingespeist (Poll bekommt die Zeit, Root ein fs.FS); das Paket
// liest nur und schreibt nie.
package watch

import (
	"context"
	"io/fs"
	"maps"
	"slices"
	"strings"
	"time"
)

// Root ist ein beobachteter Ordner; Name steht vor den Pfaden der Meldung („data/economy.json“).
type Root struct {
	Name string
	FS   fs.FS
}

// stamp ist der Stand einer Datei.
type stamp struct {
	size, mod int64
}

// Watcher vergleicht bei jedem Poll den Stand aller Dateien mit dem vorigen. Nicht nebenläufig benutzen.
type Watcher struct {
	roots   []Root
	quiet   time.Duration
	seen    map[string]stamp
	changed map[string]bool
	last    time.Time // Zeit der letzten erkannten Änderung, solange changed nicht leer ist
}

// New merkt sich den aktuellen Stand; quiet ist die Entprellfrist nach der letzten Änderung. Dateien mit einer
// Änderungszeit ab since (Laden der Daten, vor New) zählen schon als geändert, damit keine Änderung zwischen Laden und
// Grundstand verloren geht; die Frist läuft dann ab since. Null: keine. Zeiten nach jetzt (falsche Uhr, kopierte
// Dateien) zählen nicht, sonst startete jeder Prozess sofort wieder neu.
func New(roots []Root, quiet time.Duration, since time.Time) *Watcher {
	w := &Watcher{roots: roots, quiet: quiet, changed: map[string]bool{}}
	w.seen = w.scan()
	now := time.Now().UnixNano()
	for p, s := range w.seen {
		if !since.IsZero() && s.mod >= since.UnixNano() && s.mod <= now { // Zeit in der Zukunft (falsche Uhr): keine Änderung
			w.mark(p, since)
		}
	}
	return w
}

// Poll liest den Stand zur Zeit now. Liefert die geänderten Pfade (neu, geändert, gelöscht; sortiert), sobald seit der
// letzten Änderung quiet vergangen ist, sonst nil. Jede Änderung startet die Frist neu.
func (w *Watcher) Poll(now time.Time) []string {
	cur := w.scan()
	for p, s := range cur {
		if old, ok := w.seen[p]; !ok || old != s {
			w.mark(p, now)
		}
	}
	for p := range w.seen {
		if _, ok := cur[p]; !ok {
			w.mark(p, now)
		}
	}
	w.seen = cur
	if len(w.changed) == 0 || now.Sub(w.last) < w.quiet {
		return nil
	}
	out := slices.Sorted(maps.Keys(w.changed))
	clear(w.changed)
	return out
}

func (w *Watcher) mark(path string, now time.Time) {
	w.changed[path] = true
	w.last = now
}

// isTemp: Temp-Dateien des Studios beim atomaren Schreiben (.import-*, .assets-*, .tmp-*) sind keine Änderung; erst das
// Umbenennen zählt (B-511).
func isTemp(name string) bool {
	return strings.HasPrefix(name, ".import-") || strings.HasPrefix(name, ".assets-") || strings.HasPrefix(name, ".tmp-")
}

// scan liest Größe und Änderungszeit aller Dateien; fehlende oder unlesbare Ordner und Dateien fallen weg.
func (w *Watcher) scan() map[string]stamp {
	out := map[string]stamp{}
	for _, r := range w.roots {
		_ = fs.WalkDir(r.FS, ".", func(p string, d fs.DirEntry, err error) error {
			if err != nil || d.IsDir() || isTemp(d.Name()) {
				return nil
			}
			if info, err := d.Info(); err == nil {
				out[r.Name+"/"+p] = stamp{info.Size(), info.ModTime().UnixNano()}
			}
			return nil
		})
	}
	return out
}

// Run ruft Poll bei jedem Takt aus tick und fire mit den Pfaden eines Vorgangs, bis ctx endet. fire läuft in dieser
// Goroutine; solange es läuft, wird nicht abgefragt.
func Run(ctx context.Context, w *Watcher, tick <-chan time.Time, fire func([]string)) {
	for {
		select {
		case <-ctx.Done():
			return
		case now := <-tick:
			if paths := w.Poll(now); paths != nil {
				fire(paths)
			}
		}
	}
}
