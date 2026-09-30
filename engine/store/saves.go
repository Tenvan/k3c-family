package store

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
	"time"
)

// MaxSaveBytes ist die Obergrenze eines Spielstands (wie server/saves.mjs).
const MaxSaveBytes = 1 << 20

// ErrSlot heißt: der Name des Spielstands ist ungültig.
var ErrSlot = errors.New("ungültiger Slot")

var (
	slotName = regexp.MustCompile(`^[a-z0-9-]{1,32}$`)
	unsafe   = regexp.MustCompile(`[^A-Za-z0-9-]`)
)

// Saves sind die Spielstände unter Dir, je Slot eine Datei <slot>.json.
type Saves struct {
	Dir string
	Now func() time.Time // Test-Naht; nil = time.Now
}

func (s *Saves) now() time.Time {
	if s.Now == nil {
		return time.Now()
	}
	return s.Now()
}

func (s *Saves) path(slot string) (string, error) {
	if !slotName.MatchString(slot) {
		return "", ErrSlot
	}
	return filepath.Join(s.Dir, slot+".json"), nil
}

// Load liest einen Spielstand; ohne Datei ErrNotFound.
func (s *Saves) Load(slot string) ([]byte, error) {
	path, err := s.path(slot)
	if err != nil {
		return nil, err
	}
	data, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return nil, ErrNotFound
	}
	return data, err
}

// header sind die Felder, die jeder Spielstand haben muss.
type header struct {
	campaignID string
	savedAt    string
}

// parseSave prüft: ein JSON-Objekt mit campaignId (Text) und version (Zahl).
func parseSave(data []byte) (header, error) {
	var m map[string]any
	if json.Unmarshal(data, &m) != nil || m == nil {
		return header{}, ErrInvalid
	}
	id, ok := m["campaignId"].(string)
	if _, isNum := m["version"].(float64); !ok || !isNum {
		return header{}, ErrInvalid
	}
	h := header{campaignID: id}
	switch v := m["savedAt"].(type) {
	case string:
		h.savedAt = v
	case float64:
		h.savedAt = strconv.FormatFloat(v, 'f', -1, 64)
	}
	return h, nil
}

// Store schreibt einen Spielstand. Gehört der bisherige Stand des Slots zu einem anderen Spiel (andere campaignId),
// wird er vorher als <slot>-<savedAt>.json gesichert (wie server/saves.mjs); backup ist dann dessen Dateiname, sonst
// leer. Sonst wandert der bisherige Stand in die rotierenden Sicherungen (B-028, BackupKeep je Slot).
func (s *Saves) Store(slot string, data []byte) (backup string, err error) {
	path, err := s.path(slot)
	if err != nil {
		return "", err
	}
	if len(data) > MaxSaveBytes {
		return "", ErrTooLarge
	}
	next, err := parseSave(data)
	if err != nil {
		return "", err
	}
	if prev, err := os.ReadFile(path); err == nil {
		old, perr := parseSave(prev)
		if perr == nil && old.campaignID != next.campaignID {
			backup = fmt.Sprintf("%s-%s.json", slot, s.stamp(old.savedAt))
			if err := os.Rename(path, filepath.Join(s.Dir, backup)); err != nil {
				return "", err
			}
		} else if err := s.rotate(slot, prev); err != nil {
			return "", err
		}
	}
	return backup, writeAtomic(path, data)
}

// Count ist die Zahl der Spielstände (ohne Sicherungen).
func (s *Saves) Count() int {
	files, _ := filepath.Glob(filepath.Join(s.Dir, "*.json"))
	n := 0
	for _, f := range files {
		if slotName.MatchString(strings.TrimSuffix(filepath.Base(f), ".json")) {
			n++
		}
	}
	return n
}

// stamp macht aus savedAt einen sicheren Dateinamen-Teil (nie ein Pfad); ohne savedAt die aktuelle Zeit in ms.
func (s *Saves) stamp(savedAt string) string {
	clean := unsafe.ReplaceAllString(strings.NewReplacer(":", "-", ".", "-").Replace(savedAt), "")
	if clean == "" || len(clean) > 40 {
		return strconv.FormatInt(s.now().UnixMilli(), 10)
	}
	return clean
}
