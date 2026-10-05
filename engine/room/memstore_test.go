package room

import (
	"sync"

	"k3c/engine/store"
)

// memStore ist ein Spielstand-Speicher im Speicher. Die Methoden laufen auch in der Schreib-Goroutine des Raums
// (saver.go), deshalb die Sperre; Tests lesen die Felder erst nach waitSaved.
type memStore struct {
	mu      sync.Mutex
	data    map[string][]byte
	saves   int
	deleted []string
}

func (s *memStore) Load(name string) ([]byte, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	if d, ok := s.data[name]; ok {
		return d, nil
	}
	return nil, store.ErrNotFound
}

func (s *memStore) Store(name string, data []byte) (string, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.data[name] = data
	s.saves++
	return "", nil
}

func (s *memStore) Delete(name string) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	delete(s.data, name)
	s.deleted = append(s.deleted, name)
	return nil
}
