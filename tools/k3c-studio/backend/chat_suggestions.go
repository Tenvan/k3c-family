package backend

import (
	"cmp"
	"errors"
	"fmt"
	"os/exec"
	"slices"
	"time"

	assets "k3c/tools/assetkit"
)

// Vorschlagskarten des Chats auf Sound- und Grafik-Seite (B-600, WZ29.7; Karten aus WZ30.5): Einträge der
// Vorschlagsliste mit Status aus dem Vorrat, vom Chat genannte zuerst. Geschrieben wird nur nach „Übernehmen“ und nur
// mit erlaubter Lizenz (Q70) aus dem Vorrat; Downloads gibt es nur auf der Seite Quellen nach Bestätigung (B-601).
// Felder wie api/suggestionTypes.ts.

// AssetSuggestion ist eine Karte.
type AssetSuggestion struct {
	ID        string `json:"id"`
	Slot      string `json:"slot"`
	Kind      string `json:"art"`
	Bus       string `json:"bus,omitempty"`
	Title     string `json:"titel"`
	File      string `json:"datei"`
	Source    string `json:"quelle"`
	Artist    string `json:"urheber,omitempty"`
	License   string `json:"lizenz"`
	LicenseOK bool   `json:"lizenzErlaubt"`
	Reason    string `json:"lizenzGrund,omitempty"`
	Size      int64  `json:"groesse"`
	Preview   string `json:"vorschau"`
	Status    string `json:"status"` // offen, geladen, gesperrt (aus dem Vorrat)
}

// AssetSuggestionsView ist die Antwort von AssetSuggestions; offline = Chat oder Internet nicht erreichbar, die Liste
// ist dann leer. Stand ist der Stand des Registers für AssetSuggestionAdopt (Schreibregel wie AssetsSet).
type AssetSuggestionsView struct {
	Offline bool              `json:"offline"`
	Stand   string            `json:"stand"`
	List    []AssetSuggestion `json:"vorschlaege"`
}

// AssetSuggestions liefert die Karten zu slot: alle offenen Einträge des Slots, vom Chat genannte zuerst; slot = ""
// nur die vom Chat genannten. Verworfene und schon zugeordnete fehlen.
func (a *App) AssetSuggestions(slot string) (AssetSuggestionsView, error) {
	view := AssetSuggestionsView{List: []AssetSuggestion{}}
	if a.assignOffline() {
		view.Offline = true
		return view, nil
	}
	stand, err := assets.Stamp(a.root)
	if err != nil {
		return view, err
	}
	rank := a.assignState().rank()
	list, err := a.suggestionCards(func(s assets.Suggestion) bool {
		_, picked := rank[s.ID]
		return (slot == "" && picked) || (slot != "" && s.Slot == slot)
	}, rank)
	if err != nil {
		return view, err
	}
	view.Stand, view.List = stand, list
	return view, nil
}

// assignOffline: ohne Chat-CLI oder ohne Internet (Probe der Seite Quellen) ist der Bereich offline.
func (a *App) assignOffline() bool {
	if _, err := exec.LookPath(a.chats().cli.program); err != nil {
		return true
	}
	return a.sourceState().offline(a.context())
}

// suggestionCards baut die Karten der Einträge, die keep annimmt, ohne verworfene und schon zugeordnete; rank ≠ nil
// sortiert vom Chat genannte nach vorn, sonst nach Priorität.
func (a *App) suggestionCards(keep func(assets.Suggestion) bool, rank map[string]int) ([]AssetSuggestion, error) {
	list, err := assets.LoadSuggestions(a.root)
	if err != nil {
		return nil, err
	}
	states, err := assets.VorratStates(a.root, list.List)
	if err != nil {
		return nil, err
	}
	reg, err := assets.Load(a.root)
	if err != nil {
		return nil, err
	}
	var picked []assets.Suggestion
	for _, s := range list.List {
		if st := states[s.ID]; keep(s) && st.Status != assets.VorratDropped && !st.Used {
			picked = append(picked, s)
		}
	}
	slices.SortStableFunc(picked, func(x, y assets.Suggestion) int {
		return cmp.Or(cmp.Compare(order(rank, x.ID), order(rank, y.ID)), cmp.Compare(x.Priority, y.Priority))
	})
	out := []AssetSuggestion{}
	for _, s := range picked {
		out = append(out, suggestionCard(s, states[s.ID], reg))
	}
	return out, nil
}

// order ist die Position in rank; nicht Genannte stehen dahinter.
func order(rank map[string]int, id string) int {
	if i, ok := rank[id]; ok {
		return i
	}
	return len(rank)
}

func suggestionCard(s assets.Suggestion, st assets.VorratState, reg *assets.Register) AssetSuggestion {
	c := AssetSuggestion{ID: s.ID, Slot: s.Slot, Kind: s.Kind, Title: s.Title, File: s.FileName, Source: s.Source,
		Artist: s.Artist, License: s.License, Size: s.Size, Status: st.Status,
		LicenseOK: assets.SuggestionLicenseAllowed(s.License) && st.Status != assets.VorratBlocked}
	if !c.LicenseOK {
		c.Reason = cmp.Or(st.Reason, fmt.Sprintf("Lizenz %s nicht erlaubt (Q70)", s.License))
	}
	if st.Entry != nil {
		c.Preview = vorratURL + st.Entry.File
		c.File = cmp.Or(st.Entry.Public, c.File)
	}
	if slot := reg.Find(s.Slot); slot != nil && s.Kind == assets.KindSound {
		c.Bus = slot.Bus
	}
	return c
}

// AssetSuggestionAdopt übernimmt den Vorschlag id für slot nach „Übernehmen“: Kopie aus dem Vorrat nach public/,
// Credit und Zuordnung, nur auf dem Stand stand des Registers (sonst Fehler wie AssetsSet). Lehnt ab bei Lizenz
// außerhalb Q70 und bei Einträgen, die noch nicht im Vorrat liegen. Liefert den neuen Stand.
func (a *App) AssetSuggestionAdopt(slot, id, stand string) (string, error) {
	cards, err := a.suggestionCards(func(s assets.Suggestion) bool { return s.ID == id }, nil)
	if err != nil {
		return "", err
	}
	if len(cards) == 0 {
		return "", fmt.Errorf("vorschlag %q unbekannt, verworfen oder schon übernommen", id)
	}
	c := cards[0]
	switch {
	case slot != "" && c.Slot != slot:
		return "", fmt.Errorf("vorschlag %q gehört zu %s, nicht zu %s", id, c.Slot, slot)
	case !c.LicenseOK:
		return "", errors.New("Lizenz nicht erlaubt: " + c.Reason) //nolint:staticcheck // Meldung der Oberfläche
	case c.Status != assets.VorratLoaded:
		return "", fmt.Errorf("%s liegt nicht im Vorrat: erst auf der Seite Quellen laden (Download nur nach Bestätigung)", id)
	}
	_, next, err := assets.VorratAdoptIf(a.root, id, stand, time.Now())
	if err != nil {
		return "", err
	}
	a.publish(evAssetsChanged)
	return next, nil
}

// AssetSuggestionDrop verwirft den Vorschlag id (Status „verworfen“ in der Vorschlagsliste, atomar geschrieben).
func (a *App) AssetSuggestionDrop(slot, id string) error {
	return assets.DropSuggestion(a.root, slot, id)
}
