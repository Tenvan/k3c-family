package assets

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"slices"
	"strings"
	"time"
)

// AssignOptions sind die Naht von AssignCandidate zum Netz: der Client der Downloads (Host- und Redirect-Prüfung
// liegt in den Funktionen darunter), die Offline-Probe (nur gefragt, wenn ein Download nötig ist), der Download-Platz
// (Reserve belegt ihn nur für die Dauer eines Downloads, false = es läuft schon einer; Release gibt ihn frei) und die Uhr.
type AssignOptions struct {
	Client  *http.Client
	Offline func() bool
	Reserve func() bool
	Release func()
	Now     time.Time
}

// reserve belegt den Download-Platz; ohne Reserve in den Optionen ist er immer frei.
func (o AssignOptions) reserve() (func(), error) {
	if o.Reserve == nil {
		return func() {}, nil
	}
	if !o.Reserve() {
		return nil, errors.New("es läuft schon ein Download")
	}
	return o.Release, nil
}

// AssignCandidate ordnet den Kandidaten id dem Slot als letzte Präferenz zu und lädt ihn dafür bei Bedarf (Netz → Vorrat →
// Spiel in einem Aufruf). Der Kandidat wird aus Katalog, Vorschlagsliste und Treffern des Slots neu aufgelöst; Lizenz und
// Größe kommen nie aus der Anfrage. Der Stand des Registers wird vor jedem Schreiben und jedem Download geprüft (ErrStale);
// confirmedBytes muss bei Vorschlägen deren Größe sein. Scheitert ein Schritt, ist nichts zugeordnet (ein schon geladener
// Vorrat-Eintrag bleibt im Vorrat). Liefert den Slot und den neuen Stand.
func AssignCandidate(ctx context.Context, root, slotID, id, stand string, confirmedBytes int64, o AssignOptions) (*Slot, string, error) {
	if stand == "" {
		return nil, "", ErrStale
	}
	if cur, err := Stamp(root); err != nil {
		return nil, "", err
	} else if cur != stand {
		return nil, "", ErrStale
	}
	reg, err := Load(root)
	if err != nil {
		return nil, "", err
	}
	slot := reg.Find(slotID)
	if slot == nil {
		return nil, "", fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	if key, ok := strings.CutPrefix(id, idCatalog); ok {
		return assignCatalog(root, *slot, key, stand)
	}
	switch key, ok := strings.CutPrefix(id, idSuggestion); {
	case strings.HasPrefix(id, idChoice):
		return nil, "", errors.New("schon zugeordnet")
	case ok:
		err = assignSuggestion(ctx, root, *slot, key, stand, confirmedBytes, o)
	case strings.HasPrefix(id, idHit):
		err = assignHit(ctx, root, *slot, strings.TrimPrefix(id, idHit), stand, o)
	default:
		return nil, "", fmt.Errorf("kandidat %q unbekannt", id)
	}
	if err != nil {
		return nil, "", err
	}
	return reloadSlot(root, slotID)
}

// assignCatalog hängt eine Datei des Katalogs an; ND und NC bleiben gesperrt (Q70).
func assignCatalog(root string, slot Slot, file, stand string) (*Slot, string, error) {
	list, err := Catalog(root)
	if err != nil {
		return nil, "", err
	}
	i := slices.IndexFunc(list, func(a Asset) bool { return a.File == file })
	switch {
	case i < 0:
		return nil, "", fmt.Errorf("datei %q nicht im Katalog", file)
	case list[i].Kind != slot.Kind:
		return nil, "", fmt.Errorf("%s ist %s, der Slot %s", file, list[i].Kind, slot.Kind)
	case licenseBlocked(list[i].License, list[i].Flag):
		return nil, "", fmt.Errorf("%s: Lizenz %s nicht erlaubt (Q70)", file, list[i].License)
	case slices.ContainsFunc(slot.Choices, func(c Choice) bool { return c.File == file }):
		return nil, "", errors.New("schon zugeordnet")
	}
	return SetChoicesIf(root, slot.ID, append(slices.Clone(slot.Choices), Choice{File: file}), stand)
}

// assignSuggestion übernimmt einen Vorschlag aus dem Vorrat; ist er noch offen, lädt es ihn vorher synchron (nach
// Offline-, Lizenz- und Größenprüfung) in den Vorrat.
func assignSuggestion(ctx context.Context, root string, slot Slot, sid, stand string, confirmed int64, o AssignOptions) error {
	list, err := LoadSuggestions(root)
	if err != nil {
		return err
	}
	i := slices.IndexFunc(list.List, func(s Suggestion) bool { return s.ID == sid })
	if i < 0 {
		return fmt.Errorf("vorschlag %q unbekannt", sid)
	}
	s := list.List[i]
	if s.Slot != slot.ID {
		return fmt.Errorf("vorschlag %q gehört zu %s, nicht zu %s", sid, s.Slot, slot.ID)
	}
	states, err := VorratStates(root, []Suggestion{s})
	if err != nil {
		return err
	}
	switch st := states[s.ID]; st.Status {
	case VorratLoaded:
	case VorratOpen:
		if err := loadSuggestion(ctx, root, s, confirmed, o); err != nil {
			return err
		}
	default:
		return fmt.Errorf("%s ist %s: %s", sid, st.Status, st.Reason)
	}
	_, _, err = VorratAdoptIf(root, sid, stand, o.Now)
	return err
}

// loadSuggestion prüft und lädt einen offenen Vorschlag in den Vorrat.
func loadSuggestion(ctx context.Context, root string, s Suggestion, confirmed int64, o AssignOptions) error {
	if o.Offline != nil && o.Offline() {
		return errors.New("offline: Downloads brauchen Internet")
	}
	if _, _, err := CheckVorratDownload(s); err != nil {
		return err
	}
	if isPackage(s.FileName) {
		return ErrVorratPackage
	}
	if confirmed != s.Size {
		return fmt.Errorf("bestätigt %d Bytes, der Vorschlag hat %d: bitte neu bestätigen", confirmed, s.Size)
	}
	release, err := o.reserve()
	if err != nil {
		return err
	}
	defer release()
	if _, err := VorratDownload(ctx, root, s, o.Client, o.Now, nil); err != nil {
		return fmt.Errorf("download %s: %w", s.ID, err)
	}
	return nil
}

// assignHit lädt einen Recherche-Treffer mit direktem Download und erkannter Lizenz; downloadHit legt Datei, Credit und
// Zuordnung (letzte Präferenz, nur auf dem Stand stand) an. Die Größe ist bei Treffern unbekannt, confirmedBytes zählt
// hier nicht. Wird der Stand erst nach dem Download veraltet, ist die Datei importiert (Katalog), aber nicht zugeordnet.
func assignHit(ctx context.Context, root string, slot Slot, hid, stand string, o AssignOptions) error {
	_, h, err := importTarget(root, slot.ID, hid)
	if err != nil {
		return err
	}
	switch {
	case h.File != "":
		return fmt.Errorf("treffer %s ist schon importiert", hid)
	case licenseBlocked(h.License, h.Flag):
		return fmt.Errorf("treffer %s: %s", hid, blockedReason(h.License, h.Flag))
	case h.LicenseStatus == LicenseCheck:
		return fmt.Errorf("treffer %s: Lizenz prüfen und erst bestätigen", hid)
	case !hitDownloadable(h):
		return fmt.Errorf("treffer %s: kein direkter Download, bitte manuell laden und importieren", hid)
	case o.Offline != nil && o.Offline():
		return errors.New("offline: Downloads brauchen Internet")
	}
	release, err := o.reserve()
	if err != nil {
		return err
	}
	defer release()
	_, err = downloadHit(ctx, root, slot.ID, hid, stand, o.Client, o.Now)
	return err
}

func reloadSlot(root, id string) (*Slot, string, error) {
	reg, err := Load(root)
	if err != nil {
		return nil, "", err
	}
	slot := reg.Find(id)
	if slot == nil {
		return nil, "", fmt.Errorf("unbekannter Asset-Slot %q", id)
	}
	next, err := Stamp(root)
	return slot, next, err
}
