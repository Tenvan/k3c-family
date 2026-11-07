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
	Dir  string // echter Ordner für Datei-Ereignisse (notify.go); leer: nur Abfragen
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
	spare   map[string]stamp // zweiter Stand, wird je Poll wiederverwendet statt neu angelegt
	changed map[string]bool
	last    time.Time // Zeit der letzten erkannten Änderung, solange changed nicht leer ist
}

// New merkt sich den aktuellen Stand; quiet ist die Entprellfrist nach der letzten Änderung. Dateien mit einer
// Änderungszeit ab since (Laden der Daten, vor New) zählen schon als geändert, damit keine Änderung zwischen Laden und
// Grundstand verloren geht; die Frist läuft dann ab since. Null: keine. Zeiten nach jetzt (falsche Uhr, kopierte
// Dateien) zählen nicht, sonst startete jeder Prozess sofort wieder neu.
func New(roots []Root, quiet time.Duration, since time.Time) *Watcher {
	w := &Watcher{roots: roots, quiet: quiet, changed: map[string]bool{}, seen: map[string]stamp{}, spare: map[string]stamp{}}
	w.scan(w.seen)
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
	cur := w.spare
	clear(cur)
	w.scan(cur)
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
	w.seen, w.spare = cur, w.seen
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

// isTemp: Temp-Dateien beim atomaren Schreiben (.import-*, .assets-*, .tmp-*) sind keine Änderung; erst das Umbenennen
// zählt (B-511). Papierschnitt-Specs (`*.paper.json`) auch nicht: neu startet erst das gerenderte PNG (US1, B-701 D-11).
func isTemp(name string) bool {
	return strings.HasPrefix(name, ".import-") || strings.HasPrefix(name, ".assets-") || strings.HasPrefix(name, ".tmp-") ||
		strings.HasSuffix(name, ".paper.json")
}

// scan liest Größe und Änderungszeit aller Dateien; fehlende oder unlesbare Ordner und Dateien fallen weg.
func (w *Watcher) scan(out map[string]stamp) {
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
}

// Pending: Es gibt erkannte Änderungen, die auf das Ende der Entprellfrist warten; dann muss weiter abgefragt werden.
func (w *Watcher) Pending() bool { return len(w.changed) > 0 }

// fullScanEvery ist der Abstand des Sicherheits-Scans, wenn Datei-Ereignisse die Abfrage steuern (verpasste Ereignisse,
// Netzlaufwerke).
const fullScanEvery = 30 * time.Second

// Run ruft Poll bei jedem Takt aus tick und fire mit den Pfaden eines Vorgangs, bis ctx endet. fire läuft in dieser
// Goroutine; solange es läuft, wird nicht abgefragt. wake (nil erlaubt) meldet Datei-Ereignisse: Mit wake wird nur
// abgefragt, wenn ein Ereignis kam, eine Änderung noch auf ihre Frist wartet oder fullScanEvery vergangen ist; im
// Leerlauf kostet der Takt dann nichts. Ohne wake wird bei jedem Takt abgefragt.
func Run(ctx context.Context, w *Watcher, tick <-chan time.Time, wake <-chan struct{}, fire func([]string)) {
	dirty := true
	var scanned time.Time
	for {
		select {
		case <-ctx.Done():
			return
		case <-wake:
			dirty = true
		case now := <-tick:
			if wake != nil && !dirty && !w.Pending() && now.Sub(scanned) < fullScanEvery {
				continue
			}
			dirty, scanned = false, now
			if paths := w.Poll(now); paths != nil {
				fire(paths)
			}
		}
	}
}
