package store

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"log/slog"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

// MaxReportBytes ist die Obergrenze eines Testberichts (wie server/reports.mjs).
const MaxReportBytes = 256 << 10

// MaxReports ist die Zahl der Berichte, die liegen bleiben (B-142): 100 × MaxReportBytes = höchstens 25 MB auf der
// SD-Karte des Pi, und 100 Testläufe reichen für jeden Vergleich. Ältere Berichte löscht Store vor dem Schreiben.
const MaxReports = 100

const reportGlob = "gamepad-*.json"

// Reports sind die Testberichte der Gamepad-Testseite unter Dir.
type Reports struct {
	Dir string
	Now func() time.Time // Test-Naht; nil = time.Now
}

// Store ergänzt receivedAt und remoteAddress und schreibt den Bericht als gamepad-<zeit>.json. Die Antwort ist der
// Pfad der Datei. Ein Bericht muss ein JSON-Objekt sein.
func (r *Reports) Store(data []byte, remote string) (string, error) {
	if len(data) > MaxReportBytes {
		return "", ErrTooLarge
	}
	var report map[string]any
	if json.Unmarshal(data, &report) != nil || report == nil {
		return "", ErrInvalid
	}
	now := time.Now
	if r.Now != nil {
		now = r.Now
	}
	received := now().UTC().Format("2006-01-02T15:04:05.000Z")
	report["receivedAt"] = received
	report["remoteAddress"] = remote
	out, err := json.MarshalIndent(report, "", "  ")
	if err != nil {
		return "", err
	}
	if err := os.MkdirAll(r.Dir, 0o755); err != nil {
		return "", err
	}
	if n, err := r.prune(MaxReports - 1); err != nil {
		return "", err
	} else if n > 0 {
		slog.Info("🧹 alte Berichte gelöscht", "ns", "report", "anzahl", n)
	}
	base := "gamepad-" + strings.NewReplacer(":", "-", ".", "-").Replace(received)
	return writeNew(r.Dir, base, out)
}

// writeNew legt <base>.json an, ohne eine vorhandene Datei zu überschreiben (zwei Berichte in derselben Millisekunde
// bekommen -1, -2 …).
func writeNew(dir, base string, data []byte) (string, error) {
	for i := 0; i < 100; i++ {
		name := base
		if i > 0 {
			name = fmt.Sprintf("%s-%d", base, i)
		}
		path := filepath.Join(dir, name+".json")
		f, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0o644)
		if errors.Is(err, fs.ErrExist) {
			continue
		}
		if err != nil {
			return "", err
		}
		_, err = f.Write(data)
		if cerr := f.Close(); err == nil {
			err = cerr
		}
		return path, err
	}
	return "", fmt.Errorf("%s: zu viele Berichte in derselben Millisekunde", base)
}

// prune löscht die ältesten Berichte in Dir, bis höchstens keep übrig sind, und meldet, wie viele es waren. Die Ordnung
// folgt dem Zeitstempel im Dateinamen, nicht der Dateizeit; andere Dateien und Ordner (etwa saves/) bleiben unberührt.
func (r *Reports) prune(keep int) (int, error) {
	files, err := filepath.Glob(filepath.Join(r.Dir, reportGlob))
	if err != nil || len(files) <= keep {
		return 0, err
	}
	sort.Strings(files)
	old := files[:len(files)-keep]
	for _, f := range old {
		if err := os.Remove(f); err != nil && !errors.Is(err, fs.ErrNotExist) {
			return 0, err
		}
	}
	return len(old), nil
}

// Count ist die Zahl der gespeicherten Berichte.
func (r *Reports) Count() int {
	files, _ := filepath.Glob(filepath.Join(r.Dir, reportGlob))
	return len(files)
}

// StoreSession schreibt einen Spielmetrik-Report (B-150, docs/protocol.md › Spielmetrik-Report) als
// session-<zeit>.json, ohne eine vorhandene Datei zu überschreiben. Die Antwort ist der Pfad der Datei. Ein Report
// muss ein JSON-Objekt sein.
func (r *Reports) StoreSession(data []byte) (string, error) {
	if len(data) > MaxReportBytes {
		return "", ErrTooLarge
	}
	var report map[string]any
	if json.Unmarshal(data, &report) != nil || report == nil {
		return "", ErrInvalid
	}
	now := time.Now
	if r.Now != nil {
		now = r.Now
	}
	if err := os.MkdirAll(r.Dir, 0o755); err != nil {
		return "", err
	}
	stamp := strings.NewReplacer(":", "-", ".", "-").Replace(now().UTC().Format("2006-01-02T15:04:05.000Z"))
	return writeNew(r.Dir, "session-"+stamp, data)
}
