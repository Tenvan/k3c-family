package assets

import (
	"bytes"
	"encoding/json"
	"fmt"
	"path/filepath"
	"sync"
)

// suggestionsMu reiht Schreiber der Vorschlagsliste in diesem Prozess hintereinander (Verwerfen aus dem Chat, B-600).
var suggestionsMu sync.Mutex

// DropSuggestion setzt den Status des Eintrags id auf „verworfen“ und schreibt die Liste atomar (B-600, WZ29.7);
// slot ≠ "" muss der Slot des Eintrags sein. Ein schon verworfener Eintrag schreibt nichts.
func DropSuggestion(root, slot, id string) error {
	suggestionsMu.Lock()
	defer suggestionsMu.Unlock()
	list, err := LoadSuggestions(root)
	if err != nil {
		return err
	}
	for i := range list.List {
		s := &list.List[i]
		if s.ID != id {
			continue
		}
		if slot != "" && s.Slot != slot {
			return fmt.Errorf("eintrag %q gehört zu %s, nicht zu %s", id, s.Slot, slot)
		}
		if s.Status == VorratDropped {
			return nil
		}
		s.Status = VorratDropped
		return list.save(root)
	}
	return fmt.Errorf("eintrag %q nicht in der Vorschlagsliste", id)
}

// save schreibt die Liste wie das Register: zwei Leerzeichen Einrückung, Umlaute und & im Klartext, Zeilenende \n.
func (s *Suggestions) save(root string) error {
	var buf bytes.Buffer
	enc := json.NewEncoder(&buf)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	if err := enc.Encode(s); err != nil {
		return err
	}
	return writeAtomic(filepath.Join(root, SuggestionsPath), buf.Bytes())
}
