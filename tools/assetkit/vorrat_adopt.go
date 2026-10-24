package assets

import (
	"errors"
	"fmt"
	"os"
	"path"
	"strings"
	"time"
)

// groupPrefix kennzeichnet Gruppen der Vorschlagsliste ohne eigenen Slot (Skill-Effekt-Grafiken je Linie).
const groupPrefix = "skill-effekt:"

// ErrVorratPackage: Pakete (zip, 7z) übernimmt das Studio nicht selbst, die passende Datei wählt 🧑 von Hand.
var ErrVorratPackage = errors.New("Paket: Datei von Hand auswählen") //nolint:staticcheck // Meldung erscheint so in der Oberfläche

// VorratAdopt übernimmt einen geladenen Eintrag: Kopie an seinen Platz unter public/ (wie ImportFile), Credit-Zeile
// (daraus der Katalog-Eintrag) und Zuordnung als letzte Präferenz des Slots; Gruppen wie `skill-effekt:*` bekommen
// keine Zuordnung. Ein schon übernommener Eintrag liefert seine Datei, ohne etwas zu schreiben.
func VorratAdopt(root, id string, now time.Time) (string, error) {
	s, e, err := vorratAdoptTarget(root, id)
	if err != nil {
		return "", err
	}
	if e.Public != "" && fileExists(publicPath(root, e.Public)) {
		return e.Public, nil
	}
	ext := strings.ToLower(path.Ext(e.File))
	if ext == ".zip" || ext == ".7z" {
		return "", ErrVorratPackage
	}
	if !contains(importExt[s.Kind], ext) {
		return "", fmt.Errorf("%s: %s passt nicht zur Art %s", id, ext, s.Kind)
	}
	if err := checkAdoptSlot(root, s); err != nil {
		return "", err
	}
	data, err := os.ReadFile(vorratPath(root, e.File))
	if err != nil {
		return "", err
	}
	if err := checkSignature(data, ext); err != nil {
		return "", err
	}
	rel, err := storeImport(root, s.Kind, s.Title, ext, data)
	if err != nil {
		return "", err
	}
	hit := Hit{Title: s.Title, Artist: s.Artist, License: s.License, Link: s.Page, Source: s.Source, LicenseStatus: LicenseKnown}
	if err := addCredit(root, s.Kind, rel, hit, now); err != nil {
		return "", err
	}
	if err := appendChoice(root, s.Slot, rel); err != nil {
		return "", err
	}
	return rel, changeVorrat(root, func(v *VorratIndex) error {
		if x := v.find(id); x != nil {
			x.Public = rel
		}
		return nil
	})
}

// vorratAdoptTarget sucht den Eintrag in Liste und Index; er muss geladen sein.
func vorratAdoptTarget(root, id string) (Suggestion, VorratEntry, error) {
	list, err := LoadSuggestions(root)
	if err != nil {
		return Suggestion{}, VorratEntry{}, err
	}
	states, err := VorratStates(root, list.List)
	if err != nil {
		return Suggestion{}, VorratEntry{}, err
	}
	for _, s := range list.List {
		if s.ID != id {
			continue
		}
		st := states[id]
		if st.Status != VorratLoaded {
			return Suggestion{}, VorratEntry{}, fmt.Errorf("%s ist %s, nicht geladen", id, st.Status)
		}
		return s, *st.Entry, nil
	}
	return Suggestion{}, VorratEntry{}, fmt.Errorf("eintrag %q nicht in der Vorschlagsliste", id)
}

// checkAdoptSlot prüft vor dem Kopieren, dass der Slot existiert und zur Art passt (Gruppen ausgenommen).
func checkAdoptSlot(root string, s Suggestion) error {
	if strings.HasPrefix(s.Slot, groupPrefix) {
		return nil
	}
	reg, err := Load(root)
	if err != nil {
		return err
	}
	slot := reg.Find(s.Slot)
	if slot == nil {
		return fmt.Errorf("unbekannter Asset-Slot %q", s.Slot)
	}
	if slot.Kind != s.Kind {
		return fmt.Errorf("%s ist %s, der Eintrag %s", s.Slot, slot.Kind, s.Kind)
	}
	return nil
}

// appendChoice hängt rel als letzte Präferenz an den Slot; Gruppen bekommen keine Zuordnung.
func appendChoice(root, slotID, rel string) error {
	if strings.HasPrefix(slotID, groupPrefix) {
		return nil
	}
	mu.Lock()
	defer mu.Unlock()
	reg, err := Load(root)
	if err != nil {
		return err
	}
	slot := reg.Find(slotID)
	if slot == nil {
		return fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	_, err = setChoices(root, slotID, append(slot.Choices, Choice{File: rel}))
	return err
}

func publicPath(root, rel string) string {
	full, err := publicFile(root, rel)
	if err != nil {
		return ""
	}
	return full
}

// VorratCleanupPreview nennt die geladenen Dateien, die keinem Slot zugeordnet sind (das Aufräumen entfernt sie).
func VorratCleanupPreview(root string) ([]VorratEntry, error) {
	vorratMu.Lock()
	defer vorratMu.Unlock()
	v, err := LoadVorrat(root)
	if err != nil {
		return nil, err
	}
	return unusedVorrat(root, v)
}

// VorratCleanup entfernt die geladenen, unverwendeten Dateien samt Index-Eintrag; die Einträge sind danach wieder offen.
func VorratCleanup(root string) ([]VorratEntry, error) {
	var removed []VorratEntry
	err := changeVorrat(root, func(v *VorratIndex) error {
		list, err := unusedVorrat(root, v)
		if err != nil {
			return err
		}
		gone := map[string]bool{}
		for _, e := range list {
			if err := os.Remove(vorratPath(root, e.File)); err != nil && !os.IsNotExist(err) {
				return err
			}
			gone[e.ID] = true
			removed = append(removed, e)
		}
		kept := v.Entries[:0]
		for _, e := range v.Entries {
			if !gone[e.ID] {
				kept = append(kept, e)
			}
		}
		v.Entries = kept
		return nil
	})
	return removed, err
}

// unusedVorrat sind die Einträge mit Datei im Vorrat, deren übernommene Datei (falls vorhanden) keinem Slot zugeordnet ist.
func unusedVorrat(root string, v *VorratIndex) ([]VorratEntry, error) {
	used, err := usedFiles(root)
	if err != nil {
		return nil, err
	}
	out := []VorratEntry{}
	for _, e := range v.Entries {
		if fileExists(vorratPath(root, e.File)) && (e.Public == "" || !used[e.Public]) {
			out = append(out, e)
		}
	}
	return out, nil
}
