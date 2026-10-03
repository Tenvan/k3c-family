package net

import (
	"sync"

	"k3c/engine/store"
)

// Der Fake für room.Store (Load, Store, Delete); der Raum speichert aus mehreren Goroutinen (Tick, Drop), deshalb mit Sperre.
// Direkte Zugriffe der Tests (`m.Store.(memSaves)["alt"]`) laufen nur im Aufbau, vor dem ersten Gerät.
var memSavesMu sync.Mutex

func (s memSaves) Load(name string) ([]byte, error) {
	memSavesMu.Lock()
	defer memSavesMu.Unlock()
	if d, ok := s[name]; ok {
		return d, nil
	}
	return nil, store.ErrNotFound
}

func (s memSaves) Store(name string, data []byte) (string, error) {
	memSavesMu.Lock()
	defer memSavesMu.Unlock()
	s[name] = data
	return "", nil
}

func (s memSaves) Delete(name string) error {
	memSavesMu.Lock()
	defer memSavesMu.Unlock()
	delete(s, name)
	return nil
}
