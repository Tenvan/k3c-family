package store

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// MaxReportBytes ist die Obergrenze eines Testberichts (wie server/reports.mjs).
const MaxReportBytes = 256 << 10

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
