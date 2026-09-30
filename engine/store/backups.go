package store

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"
)

// BackupKeep ist die Zahl der Sicherungen je Spielstand (B-028, Entscheidung 🧑: 5).
const BackupKeep = 5

// stampLayout sortiert als Text wie als Zeit; Nanosekunden, damit schnelle Speichervorgänge nicht kollidieren.
const stampLayout = "20060102T150405.000000000Z"

// Backup ist eine Sicherung eines Spielstands.
type Backup struct {
	Name    string    `json:"name"`
	SavedAt time.Time `json:"savedAt"`
	Size    int64     `json:"size"`
}

func (s *Saves) backupDir(slot string) string {
	return filepath.Join(s.Dir, "backups", slot)
}

// rotate legt den bisherigen Stand eines Slots als Sicherung ab und behält die BackupKeep neuesten.
func (s *Saves) rotate(slot string, prev []byte) error {
	dir := s.backupDir(slot)
	name := s.now().UTC().Format(stampLayout) + ".json"
	for i := 1; exists(filepath.Join(dir, name)); i++ {
		name = fmt.Sprintf("%s-%d.json", s.now().UTC().Format(stampLayout), i)
	}
	if err := writeAtomic(filepath.Join(dir, name), prev); err != nil {
		return err
	}
	list, err := s.backups(slot)
	if err != nil {
		return nil // Aufräumen ist Kür: der neue Stand wird trotzdem geschrieben
	}
	for _, b := range list[min(len(list), BackupKeep):] {
		_ = os.Remove(filepath.Join(dir, b.Name)) // beim nächsten Speichern erneut versucht
	}
	return nil
}

func exists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

// Backups listet die Sicherungen eines Slots, neueste zuerst.
func (s *Saves) Backups(slot string) ([]Backup, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.backups(slot)
}

func (s *Saves) backups(slot string) ([]Backup, error) {
	if !slotName.MatchString(slot) {
		return nil, ErrSlot
	}
	entries, err := os.ReadDir(s.backupDir(slot))
	if errors.Is(err, fs.ErrNotExist) {
		return []Backup{}, nil
	}
	if err != nil {
		return nil, err
	}
	out := []Backup{}
	for _, e := range entries {
		info, err := e.Info()
		if err != nil || e.IsDir() || !strings.HasSuffix(e.Name(), ".json") || strings.HasPrefix(e.Name(), ".") {
			continue
		}
		at, _ := time.Parse(stampLayout, strings.SplitN(strings.TrimSuffix(e.Name(), ".json"), "-", 2)[0])
		out = append(out, Backup{Name: e.Name(), SavedAt: at, Size: info.Size()})
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Name > out[j].Name })
	return out, nil
}

// Restore macht eine Sicherung zum aktuellen Stand; der bisherige wird dabei selbst gesichert. Der Name wird nur
// angenommen, wenn er in der Liste des Slots steht (nie als Pfad).
func (s *Saves) Restore(slot, name string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	list, err := s.backups(slot)
	if err != nil {
		return err
	}
	for _, b := range list {
		if b.Name == name {
			data, err := os.ReadFile(filepath.Join(s.backupDir(slot), b.Name))
			if err != nil {
				return err
			}
			_, err = s.store(slot, data)
			return err
		}
	}
	return ErrNotFound
}
