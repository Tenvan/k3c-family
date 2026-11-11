package local

import (
	"fmt"
	"slices"
	"sync"
	"time"

	"k3c/engine/sim"
	"k3c/engine/store"
	"k3c/native/k3c-game/view"
)

// watchedStore ist room.Store über store.Saves und merkt sich das Ergebnis jeder Speicherung: Der Raum schreibt im
// Hintergrund und meldet Fehler nur im Log, die Session macht daraus Cues.
type watchedStore struct {
	saves   *store.Saves
	mu      sync.Mutex // Store läuft auch in der Schreib-Goroutine des Raums
	results []error    // nil = gespeichert
}

func (w *watchedStore) Load(name string) ([]byte, error) { return w.saves.Load(name) }
func (w *watchedStore) Delete(name string) error         { return w.saves.Delete(name) }

func (w *watchedStore) Store(name string, data []byte) (string, error) {
	backup, err := w.saves.Store(name, data)
	w.mu.Lock()
	w.results = append(w.results, err)
	w.mu.Unlock()
	return backup, err
}

// take liefert die Ergebnisse seit dem letzten Aufruf.
func (w *watchedStore) take() []error {
	w.mu.Lock()
	defer w.mu.Unlock()
	out := w.results
	w.results = nil
	return out
}

// List liefert die Spielstände, neueste zuerst (gleiche Zeit: nach Name). Kaputte und unbekannte Versionen stehen
// mit Loadable false und Grund in der Liste.
func (s *Session) List() ([]view.SaveInfo, error) {
	raw := s.saves.List()
	out := make([]view.SaveInfo, 0, len(raw))
	for _, e := range raw {
		out = append(out, s.info(e))
	}
	slices.SortStableFunc(out, func(a, b view.SaveInfo) int { return b.SavedAt.Compare(a.SavedAt) })
	return out, nil
}

// info prüft einen Eintrag wie beim Laden (sim.ParseIslandSave), ohne die Insel aufzubauen.
func (s *Session) info(e store.SaveInfo) view.SaveInfo {
	v := view.SaveInfo{Name: e.Name, Day: e.Day, Depths: e.Depths}
	v.SavedAt, _ = time.Parse(time.RFC3339, e.SavedAt)
	if e.Error != "" {
		v.Problem = "Spielstand unlesbar"
		return v
	}
	data, err := s.saves.Load(e.Name)
	if err == nil {
		_, err = sim.ParseIslandSave(data)
	}
	if err != nil {
		v.Problem = "Spielstand nicht ladbar: " + err.Error()
		return v
	}
	v.Loadable = true
	return v
}

// Delete nimmt einen Spielstand aus der Liste wie der Server (store.Saves.Discard: in die Sicherungen, B-506); nicht
// den des laufenden Spiels.
func (s *Session) Delete(name string) error {
	if s.r != nil && name == s.name {
		return userErr("Der Spielstand des laufenden Spiels lässt sich nicht löschen", nil)
	}
	discard := func() error { return s.saves.Discard(name) }
	var err error
	if s.m != nil {
		err = s.m.DeleteSave(name, discard)
	} else {
		err = discard()
	}
	if err != nil {
		return describe(err, name)
	}
	s.log.Info("🧹 Spielstand gelöscht", "save", name)
	return nil
}

// freeName ist der erste freie Name spiel-1, spiel-2 … im Ordner.
func (s *Session) freeName() string {
	used := map[string]bool{}
	for _, e := range s.saves.List() {
		used[e.Name] = true
	}
	for i := 1; ; i++ {
		if n := fmt.Sprintf("spiel-%d", i); !used[n] {
			return n
		}
	}
}
