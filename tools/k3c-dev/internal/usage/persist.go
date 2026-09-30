package usage

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
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
	path    string
	onErr   func(error)
	timer   *time.Timer
	writeMu sync.Mutex // eine Datei, ein Schreiber
	warned  bool       // Schreibfehler nur einmal je Programmlauf melden
}

// Open legt einen Tracker an, der Gesamtzeit und Zeitreihe in path speichert und beim Start lädt. Eine kaputte oder
// fremde Datei wird zu path.bak, der Tracker beginnt leer; onErr (optional) erfährt davon.
func Open(path string, now func() time.Time, onErr func(error)) *Tracker {
	if onErr == nil {
		onErr = func(error) {}
	}
	t := New(now)
	t.persist = &persist{path: path, onErr: onErr}
	data, err := os.ReadFile(path)
	if errors.Is(err, os.ErrNotExist) {
		return t
	}
	var f fileData
	if err == nil {
		err = json.Unmarshal(data, &f)
	}
	if err == nil && (f.Version != fileVersion || f.AllTime == nil || f.AllTime.Tools == nil) {
		err = fmt.Errorf("version %d statt %d oder ohne Gesamtzeit", f.Version, fileVersion)
	}
	if err != nil {
		bak := path + ".bak"
		if rerr := os.Rename(path, bak); rerr != nil {
			onErr(fmt.Errorf("statistik %s unlesbar (%v) und nicht nach .bak verschiebbar: %w", path, err, rerr))
		} else {
			onErr(fmt.Errorf("statistik %s unlesbar (%v), neu begonnen; alte Datei: %s", path, err, bak))
		}
		return t
	}
	t.allTime, t.minutes = f.AllTime, f.Minutes
	t.prune()
	return t
}

// scheduleSave plant ein Schreiben in saveDelay, falls noch keins aussteht. Aufruf unter t.mu.
func (t *Tracker) scheduleSave() {
	if t.persist == nil || t.persist.timer != nil {
		return
	}
	t.persist.timer = time.AfterFunc(saveDelay, func() { _ = t.Flush() })
}

// Flush schreibt sofort (beim Beenden und aus dem Timer). Ohne Datei tut es nichts.
func (t *Tracker) Flush() error {
	if t.persist == nil {
		return nil
	}
	t.mu.Lock()
	if t.persist.timer != nil {
		t.persist.timer.Stop()
		t.persist.timer = nil
	}
	data, err := json.Marshal(fileData{Version: fileVersion, AllTime: t.allTime, Minutes: t.minutes})
	t.mu.Unlock()
	if err == nil {
		err = t.persist.write(data)
	}
	if err != nil {
		t.persist.warnOnce(err)
	}
	return err
}

// write schreibt atomar: erst die temporäre Datei, dann umbenennen, damit ein Absturz nie eine halbe Datei hinterlässt.
func (p *persist) write(data []byte) error {
	p.writeMu.Lock()
	defer p.writeMu.Unlock()
	if err := os.MkdirAll(filepath.Dir(p.path), 0o755); err != nil {
		return err
	}
	tmp := p.path + ".tmp"
	if err := os.WriteFile(tmp, data, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, p.path)
}

func (p *persist) warnOnce(err error) {
	p.writeMu.Lock()
	defer p.writeMu.Unlock()
	if !p.warned {
		p.warned = true
		p.onErr(fmt.Errorf("statistik nicht gespeichert, läuft im Speicher weiter: %w", err))
	}
}
