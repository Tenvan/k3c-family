package usage

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"sync"
	"time"
)

const (
	fileVersion = 1
	// saveDelay bündelt Schreibvorgänge: ein fleißiger Agent ruft im Sekundentakt, die Datei muss das nicht mitmachen.
	saveDelay = 2 * time.Second
)

// fileData ist der Inhalt der Datei; die Sitzung wird nie gespeichert.
type fileData struct {
	Version int      `json:"version"`
	AllTime *scope   `json:"allTime"`
	Minutes []minute `json:"minutes"`
}

// persist hält Pfad, Timer und den Fehler-Rückruf.
type persist struct {
	path     string
	onErr    func(error)
	timer    *time.Timer // unter Tracker.mu
	writeMu  sync.Mutex  // Schnappschuss und Schreiben am Stück, damit nie ein älterer Stand zuletzt schreibt
	warnOnce sync.Once   // Schreibfehler nur einmal je Programmlauf melden
}

// Open legt einen Tracker an, der Gesamtzeit und Zeitreihe in path speichert und beim Start lädt. Eine kaputte oder
// fremde Datei wird zur Sicherung (.bak, eine vorhandene bleibt erhalten), der Tracker beginnt leer. Lässt sich die
// Datei nicht lesen (z. B. gesperrt), bleibt sie unangetastet und dieser Lauf speichert nicht, damit der erste
// Flush die gültige Statistik nicht überschreibt. onErr (optional) erfährt jeweils davon.
func Open(path string, now func() time.Time, onErr func(error)) *Tracker {
	if onErr == nil {
		onErr = func(error) {}
	}
	t := New(now)
	data, err := os.ReadFile(path)
	switch {
	case errors.Is(err, os.ErrNotExist):
	case err != nil:
		onErr(fmt.Errorf("statistik %s nicht lesbar (%v); dieser Lauf zählt nur im Speicher", path, err))
		return t
	default:
		f, err := decode(data)
		if err != nil {
			moveAside(path, err, now, onErr)
		} else {
			t.allTime, t.minutes = f.AllTime, f.Minutes
			t.prune()
		}
	}
	t.persist = &persist{path: path, onErr: onErr}
	return t
}

// decode liest die Datei und macht sie robust gegen fehlende Felder (von Hand bearbeitet oder anderes Werkzeug),
// damit Record später nie in eine nil-Map schreibt.
func decode(data []byte) (fileData, error) {
	var f fileData
	if err := json.Unmarshal(data, &f); err != nil {
		return f, err
	}
	if f.Version != fileVersion || f.AllTime == nil {
		return f, fmt.Errorf("version %d statt %d oder ohne Gesamtzeit", f.Version, fileVersion)
	}
	if f.AllTime.Tools == nil {
		f.AllTime.Tools = map[string]*toolAgg{}
	}
	for name, ta := range f.AllTime.Tools {
		if ta == nil {
			delete(f.AllTime.Tools, name)
			continue
		}
		ta.repair()
	}
	for i := range f.Minutes {
		f.Minutes[i].repair()
	}
	sort.Slice(f.Minutes, func(i, j int) bool { return f.Minutes[i].T < f.Minutes[j].T })
	return f, nil
}

func (ta *toolAgg) repair() {
	if ta.Args == nil {
		ta.Args = map[string]int{}
	}
	if ta.ErrorMsgs == nil {
		ta.ErrorMsgs = map[string]int{}
	}
	if ta.ArgDur == nil {
		ta.ArgDur = map[string]*durAgg{}
	}
	for k, d := range ta.ArgDur {
		if d == nil {
			delete(ta.ArgDur, k)
		}
	}
}

func (m *minute) repair() {
	if m.Calls == nil {
		m.Calls = map[string]int{}
	}
	if m.Ms == nil {
		m.Ms = map[string]float64{}
	}
	if m.MaxMs == nil {
		m.MaxMs = map[string]float64{}
	}
	if m.Outliers == nil {
		m.Outliers = map[string]int{}
	}
}

// moveAside verschiebt eine unbrauchbare Datei nach path.bak; gibt es die schon, bekommt die neue Sicherung einen
// Zeitstempel, damit keine frühere Sicherung verloren geht.
func moveAside(path string, cause error, now func() time.Time, onErr func(error)) {
	bak := path + ".bak"
	if _, err := os.Stat(bak); err == nil {
		bak = path + "." + now().Format("20060102-150405") + ".bak"
	}
	if err := os.Rename(path, bak); err != nil {
		onErr(fmt.Errorf("statistik %s unbrauchbar (%v) und nicht verschiebbar: %w", path, cause, err))
		return
	}
	onErr(fmt.Errorf("statistik %s unbrauchbar (%v), neu begonnen; alte Datei: %s", path, cause, bak))
}

// scheduleSave plant ein Schreiben in saveDelay, falls noch keins aussteht. Aufruf unter t.mu.
func (t *Tracker) scheduleSave() {
	if t.persist == nil || t.persist.timer != nil {
		return
	}
	t.persist.timer = time.AfterFunc(saveDelay, func() { _ = t.Flush() })
}

// Flush schreibt sofort (beim Beenden und aus dem Timer). Ohne Datei tut es nichts. Reihenfolge der Sperren:
// writeMu, dann t.mu; Record nimmt nur t.mu.
func (t *Tracker) Flush() error {
	p := t.persist
	if p == nil {
		return nil
	}
	p.writeMu.Lock()
	t.mu.Lock()
	if p.timer != nil {
		p.timer.Stop()
		p.timer = nil
	}
	data, err := json.Marshal(fileData{Version: fileVersion, AllTime: t.allTime, Minutes: t.minutes})
	t.mu.Unlock()
	if err == nil {
		err = p.write(data)
	}
	p.writeMu.Unlock()
	if err != nil {
		p.warnOnce.Do(func() { p.onErr(fmt.Errorf("statistik nicht gespeichert, läuft im Speicher weiter: %w", err)) })
	}
	return err
}

// write schreibt atomar: erst die temporäre Datei, dann umbenennen, damit ein Absturz nie eine halbe Datei
// hinterlässt. Aufruf unter writeMu.
func (p *persist) write(data []byte) error {
	if err := os.MkdirAll(filepath.Dir(p.path), 0o755); err != nil {
		return err
	}
	tmp := p.path + ".tmp"
	if err := os.WriteFile(tmp, data, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, p.path)
}
