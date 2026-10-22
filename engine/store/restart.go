package store

import (
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"time"
)

// Gemerkte Räume für einen geplanten Neustart (WZ25.1, B-508): eine Datei im Spielstand-Ordner, die kein Spielstand ist
// (Endung .state, nicht .json, deshalb nicht in der Spielstandliste). Der Server schreibt sie nur vor dem Neustart wegen
// geänderter Dateien und liest und löscht sie beim Start; nie in einem beobachteten Ordner, ein Absturz hinterlässt sie nicht.

const restartRoomsFile = "restart-rooms.state"

// RememberedRoomsMaxAge ist das Höchstalter der Merkdatei: Herunterfahren und Neustart dauern Sekunden.
const RememberedRoomsMaxAge = 2 * time.Minute

// RememberRooms schreibt die gemerkten Räume (atomar).
func (s *Saves) RememberRooms(data []byte) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	return writeAtomic(filepath.Join(s.Dir, restartRoomsFile), data)
}

// TakeRememberedRooms liest die gemerkten Räume und löscht die Datei; ohne Datei ErrNotFound.
func (s *Saves) TakeRememberedRooms() ([]byte, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	path := filepath.Join(s.Dir, restartRoomsFile)
	data, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return nil, ErrNotFound
	}
	if err != nil {
		return nil, err
	}
	// Eine alte Datei stammt von einem Neustart, der nicht stattfand (Supervisor gestoppt, Build gescheitert): verwerfen,
	// ein späterer normaler Start öffnet keine alten Räume.
	info, statErr := os.Stat(path)
	stale := statErr == nil && s.now().Sub(info.ModTime()) > RememberedRoomsMaxAge
	if err := os.Remove(path); err != nil {
		return nil, err
	}
	if stale {
		return nil, ErrNotFound
	}
	return data, nil
}
