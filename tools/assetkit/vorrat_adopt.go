package assets

import (
	"errors"
	"fmt"
	"log/slog"
	"os"
	"path"
	"strings"
	"sync"
	"time"
)

// adoptMu reiht Übernahmen hintereinander (B-678): der zweite Aufruf für denselben Eintrag sieht dann die Datei des ersten
// und legt keine zweite Kopie an.
var adoptMu sync.Mutex

// groupPrefix kennzeichnet Gruppen der Vorschlagsliste ohne eigenen Slot (Skill-Effekt-Grafiken je Linie).
const groupPrefix = "skill-effekt:"

// ErrVorratPackage: Pakete (zip, 7z) übernimmt das Studio nicht selbst, die passende Datei wählt 🧑 von Hand.
var ErrVorratPackage = errors.New("Paket: Datei von Hand auswählen") //nolint:staticcheck // Meldung erscheint so in der Oberfläche

// VorratAdopt übernimmt einen geladenen Eintrag: Kopie an seinen Platz unter public/ (wie ImportFile), Credit-Zeile
// (daraus der Katalog-Eintrag) und Zuordnung als letzte Präferenz des Slots; Gruppen wie `skill-effekt:*` bekommen
// keine Zuordnung. Ein schon übernommener Eintrag liefert seine Datei, ohne etwas zu schreiben.
func VorratAdopt(root, id string, now time.Time) (string, error) {
	return vorratAdopt(root, id, "", now)
}

// VorratAdoptIf ist VorratAdopt mit der Schreibregel von SetChoicesIf (Chat-Vorschläge, B-600): nur auf dem Stand
// stand des Registers, sonst ErrStale vor jedem Schreibzugriff; ein schon übernommener Eintrag wird dem Slot
// zugeordnet, falls er dort fehlt. Liefert die Datei und den neuen Stand.
func VorratAdoptIf(root, id, stand string, now time.Time) (string, string, error) {
	if stand == "" {
		return "", "", ErrStale
	}
	cur, err := Stamp(root)
	if err != nil {
		return "", "", err
	}
	if cur != stand {
		return "", "", ErrStale
	}
	rel, err := vorratAdopt(root, id, stand, now)
	if err != nil {
		return "", "", err
	}
	next, err := Stamp(root)
	return rel, next, err
}

// vorratAdopt übernimmt; stand ≠ "" verlangt diesen Stand des Registers: geprüft unter mu vor dem ersten Schreiben, mu
// bleibt bis nach dem Anhängen gehalten, damit bei ErrStale weder Datei noch Credit-Zeile übrig bleiben.
func vorratAdopt(root, id, stand string, now time.Time) (string, error) {
	adoptMu.Lock()
	defer adoptMu.Unlock()
	s, e, err := vorratAdoptTarget(root, id)
	if err != nil {
		return "", err
	}
	attach := appendChoice
	if stand != "" {
		mu.Lock()
		defer mu.Unlock()
		if err := checkStand(root, stand); err != nil {
			return "", err
		}
		attach = appendChoiceLocked
	}
	if e.Public != "" && fileExists(publicPath(root, e.Public)) {
		if stand == "" {
			return e.Public, nil
		}
		return e.Public, attach(root, s.Slot, e.Public, stand)
	}
	data, ext, err := readAdoptFile(root, s, e)
	if err != nil {
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
	if err := attach(root, s.Slot, rel, stand); err != nil {
		return "", err
	}
	markAdopted(root, id, rel)
	return rel, nil
}

// readAdoptFile liest die Vorrat-Datei des Eintrags und prüft vor dem Schreiben Endung, Slot und Signatur.
func readAdoptFile(root string, s Suggestion, e VorratEntry) ([]byte, string, error) {
	ext := strings.ToLower(path.Ext(e.File))
	if ext == ".zip" || ext == ".7z" {
		return nil, "", ErrVorratPackage
	}
	if !contains(importExt[s.Kind], ext) {
		return nil, "", fmt.Errorf("%s: %s passt nicht zur Art %s", s.ID, ext, s.Kind)
	}
	if err := checkAdoptSlot(root, s); err != nil {
		return nil, "", err
	}
	data, err := os.ReadFile(vorratPath(root, e.File))
	if err != nil {
		return nil, "", err
	}
	return data, ext, checkSignature(data, ext)
}

// markAdopted vermerkt die übernommene Datei am Index-Eintrag. Die Zuordnung steht dann schon: scheitert nur der Index,
// gilt die Übernahme trotzdem (eine erneute Übernahme legt dann eine zweite Kopie an, statt die erste zu erkennen).
func markAdopted(root, id, rel string) {
	err := changeVorrat(root, func(v *VorratIndex) error {
		if x := v.find(id); x != nil {
			x.Public = rel
		}
		return nil
	})
	if err != nil {
		slog.Warn("❌ Vorrat-Index nicht aktualisiert", "id", id, "datei", rel, "fehler", err)
	}
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
		if !SuggestionLicenseAllowed(s.License) {
			return Suggestion{}, VorratEntry{}, fmt.Errorf("%s: Lizenz %s nicht erlaubt (Q70)", id, s.License)
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

// appendChoice hängt rel als letzte Präferenz an den Slot, wenn er sie noch nicht hat; Gruppen bekommen keine
// Zuordnung. stand ≠ "" verlangt diesen Stand des Registers (sonst ErrStale, nichts geschrieben).
func appendChoice(root, slotID, rel, stand string) error {
	mu.Lock()
	defer mu.Unlock()
	return appendChoiceLocked(root, slotID, rel, stand)
}

// checkStand liefert ErrStale, wenn das Register nicht mehr den Stand stand hat; der Aufrufer hält mu.
func checkStand(root, stand string) error {
	cur, err := Stamp(root)
	if err != nil {
		return err
	}
	if cur != stand {
		return ErrStale
	}
	return nil
}

// appendChoiceLocked ist appendChoice, der Aufrufer hält mu.
func appendChoiceLocked(root, slotID, rel, stand string) error {
	if strings.HasPrefix(slotID, groupPrefix) {
		return nil
	}
	if stand != "" {
		if err := checkStand(root, stand); err != nil {
			return err
		}
	}
	reg, err := Load(root)
	if err != nil {
		return err
	}
	slot := reg.Find(slotID)
	if slot == nil {
		return fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	for _, c := range slot.Choices {
		if c.File == rel {
			return nil
		}
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
