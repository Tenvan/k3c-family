package backend

import (
	"encoding/json"
	"fmt"
	"slices"

	assets "k3c/tools/assetkit"
)

// Aktionen der Seite Assets samt Quellen (B-679): dieselben Aufrufe wie die Knöpfe der Seite; Stand und Bytes werden
// bei der Prüfung gemerkt, Downloads gelten mit „Ausführen“ als bestätigt (die Karte nennt die Bytes, B-601).

func assetActions() []actionSpec {
	return []actionSpec{
		{"zuordnung_setzen", `slot, wahl: [{"datei": "audio/x.ogg", "sheet"?, "sprite"?}] (Reihenfolge = Präferenz, leer = Zuordnung leeren)`,
			"Zuordnung eines Slots setzen", prepareChoices},
		{"kandidat_zuordnen", "slot, id (aus „Kandidaten“)", "Kandidaten zuordnen, lädt bei Bedarf", prepareCandidate},
		{"vorschlag_uebernehmen", "slot, id (Vorschlagsliste, im Vorrat geladen)", "Vorschlag übernehmen", prepareAdopt},
		{"vorschlag_verwerfen", "slot, id", "Vorschlag verwerfen", prepareDrop},
		{"recherche", "slot, suche", "Recherche für einen Slot starten", prepareResearch},
		{"lizenz_bestaetigen", "slot, treffer (id eines Recherche-Treffers)", "Lizenz eines Treffers bestätigen", prepareLicense},
		{"laden", "ids: [Einträge der Vorschlagsliste]", "Einträge in den Vorrat laden", prepareDownload},
		{"vorrat_uebernehmen", "id", "Eintrag aus dem Vorrat nach public/ übernehmen", prepareVorrat},
		{"aufraeumen", "(keine)", "Unverwendete Dateien aus dem Vorrat löschen", prepareCleanup},
		{"netz", "modus: auto | online | offline", "Netzmodus der Seite Quellen setzen", prepareNetwork},
	}
}

func slotOf(a *App, id string) (*assets.Slot, error) {
	if err := need("slot", id); err != nil {
		return nil, err
	}
	reg, err := assets.Load(a.root)
	if err != nil {
		return nil, err
	}
	s := reg.Find(id)
	if s == nil {
		return nil, fmt.Errorf("unbekannter Asset-Slot %q", id)
	}
	return s, nil
}

type slotID struct {
	Slot string `json:"slot"`
	ID   string `json:"id"`
}

func prepareChoices(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		Slot string          `json:"slot"`
		Wahl []assets.Choice `json:"wahl"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Zuordnung " + f.Slot, details: []string{}}
	stand, err := assets.Stamp(a.root)
	if err != nil {
		return p, err
	}
	if _, err := slotOf(a, f.Slot); err != nil {
		return p, err
	}
	for i, c := range f.Wahl {
		if err := need("datei", c.File); err != nil {
			return p, err
		}
		p.details = append(p.details, fmt.Sprintf("Präferenz %d: %s", i+1, c.File))
	}
	if len(f.Wahl) == 0 {
		p.details = append(p.details, "Zuordnung leeren")
	}
	p.run = func() (string, error) {
		saved, err := a.AssetsSet(f.Slot, f.Wahl, stand)
		if err != nil {
			return "", err
		}
		a.publish(evAssetsChanged)
		return fmt.Sprintf("%s: Status %s", f.Slot, saved.Slot.Status), nil
	}
	return p, nil
}

func prepareCandidate(a *App, raw json.RawMessage) (actionPlan, error) {
	var f slotID
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Kandidat für " + f.Slot}
	if err := need("id", f.ID); err != nil {
		return p, err
	}
	view, err := a.AssetCandidates(f.Slot, "", 0)
	if err != nil {
		return p, err
	}
	i := slices.IndexFunc(view.Candidates, func(c assets.Candidate) bool { return c.ID == f.ID })
	if i < 0 {
		return p, fmt.Errorf("kandidat %q ist nicht in der Liste von %s", f.ID, f.Slot)
	}
	c := view.Candidates[i]
	p.details = []string{fmt.Sprintf("%s · %s · %s · %s", c.Title, c.File, c.Source, c.License), fmt.Sprintf("%d Bytes", c.Size)}
	if !c.Assignable {
		return p, fmt.Errorf("nicht zuordenbar: %s", c.Reason)
	}
	p.run = func() (string, error) {
		saved, err := a.AssetCandidateAssign(f.Slot, f.ID, view.Stand, c.Size)
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("%s ← %s (Status %s)", f.Slot, c.File, saved.Slot.Status), nil
	}
	return p, nil
}

func prepareAdopt(a *App, raw json.RawMessage) (actionPlan, error) {
	var f slotID
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Vorschlag " + f.ID + " für " + f.Slot}
	stand, err := assets.Stamp(a.root) // vor der Prüfung: ändert sich das Register danach, lehnt Übernehmen ab
	if err != nil {
		return p, err
	}
	cards, err := a.suggestionCards(func(s assets.Suggestion) bool { return s.ID == f.ID && s.Slot == f.Slot }, nil)
	if err != nil {
		return p, err
	}
	if len(cards) == 0 {
		return p, fmt.Errorf("vorschlag %q für %s unbekannt, verworfen oder schon übernommen", f.ID, f.Slot)
	}
	c := cards[0]
	p.details = []string{fmt.Sprintf("%s · %s · %s · %s", c.Title, c.File, c.Source, c.License)}
	if block := adoptBlockOf(c); block != "" {
		return p, fmt.Errorf("%s", block)
	}
	p.run = func() (string, error) {
		if _, err := a.AssetSuggestionAdopt(f.Slot, f.ID, stand); err != nil {
			return "", err
		}
		return f.Slot + " ← " + c.File, nil
	}
	return p, nil
}

// adoptBlockOf ist der Grund, warum ein Vorschlag nicht übernehmbar ist (wie chat/SuggestionCard.tsx › adoptBlock).
func adoptBlockOf(c AssetSuggestion) string {
	switch {
	case !c.LicenseOK:
		return c.Reason
	case c.Status != assets.VorratLoaded:
		return "noch nicht im Vorrat: erst laden (Aktion laden)"
	}
	return ""
}

func prepareDrop(a *App, raw json.RawMessage) (actionPlan, error) {
	var f slotID
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Vorschlag " + f.ID + " verwerfen"}
	if err := need("id", f.ID); err != nil {
		return p, err
	}
	if _, err := slotOf(a, f.Slot); err != nil {
		return p, err
	}
	p.run = func() (string, error) { return "verworfen", a.AssetSuggestionDrop(f.Slot, f.ID) }
	return p, nil
}

func prepareResearch(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		Slot  string `json:"slot"`
		Query string `json:"suche"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Recherche für " + f.Slot, details: []string{"Suche: " + orNone(f.Query)}}
	if _, err := slotOf(a, f.Slot); err != nil {
		return p, err
	}
	p.run = func() (string, error) {
		res, err := a.AssetsResearch(f.Slot, f.Query)
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("%d Treffer", len(res.Hits)), nil
	}
	return p, nil
}

func prepareLicense(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		Slot string `json:"slot"`
		Hit  string `json:"treffer"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Lizenz bestätigen: " + f.Hit}
	if _, err := slotOf(a, f.Slot); err != nil {
		return p, err
	}
	hits, err := a.AssetsHits(f.Slot)
	if err != nil {
		return p, err
	}
	i := slices.IndexFunc(hits, func(h assets.Hit) bool { return h.ID == f.Hit })
	if i < 0 {
		return p, fmt.Errorf("treffer %q gehört nicht zu %s", f.Hit, f.Slot)
	}
	h := hits[i]
	p.details = []string{fmt.Sprintf("%s · %s · Lizenz %s (%s)", h.Title, h.Source, orNone(h.License), h.LicenseStatus), h.Link}
	p.run = func() (string, error) { return "Lizenz bestätigt", a.AssetsLicenseOK(f.Slot, f.Hit) }
	return p, nil
}

func prepareDownload(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		IDs []string `json:"ids"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: fmt.Sprintf("%d Einträge laden", len(f.IDs)), details: []string{}}
	if len(f.IDs) == 0 {
		return p, fmt.Errorf("feld %q ist leer", "ids")
	}
	list, err := assets.LoadSuggestions(a.root)
	if err != nil {
		return p, err
	}
	var sum int64
	for _, id := range f.IDs {
		i := slices.IndexFunc(list.List, func(s assets.Suggestion) bool { return s.ID == id })
		if i < 0 {
			return p, fmt.Errorf("unbekannter Eintrag %s", id)
		}
		s := list.List[i]
		if _, _, err := assets.CheckVorratDownload(s); err != nil {
			return p, err
		}
		sum += s.Size
		p.details = append(p.details, fmt.Sprintf("%s · %s · %s · %d Bytes", s.ID, s.Title, s.License, s.Size))
	}
	p.details = append(p.details, fmt.Sprintf("zusammen %d Bytes (Ausführen bestätigt den Download)", sum))
	p.run = func() (string, error) {
		return "Download läuft (Fortschritt auf der Seite Assets › Quellen)", a.SourceDownload(f.IDs, sum)
	}
	return p, nil
}

func prepareVorrat(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		ID string `json:"id"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Aus dem Vorrat übernehmen: " + f.ID}
	if err := need("id", f.ID); err != nil {
		return p, err
	}
	list, err := assets.LoadSuggestions(a.root)
	if err != nil {
		return p, err
	}
	if !slices.ContainsFunc(list.List, func(s assets.Suggestion) bool { return s.ID == f.ID }) {
		return p, fmt.Errorf("unbekannter Eintrag %s", f.ID)
	}
	p.run = func() (string, error) { return "übernommen", a.SourceAdopt(f.ID) }
	return p, nil
}

func prepareCleanup(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct{}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Vorrat aufräumen"}
	prev, err := a.SourceCleanupPreview()
	if err != nil {
		return p, err
	}
	p.details = []string{fmt.Sprintf("%d Dateien, %d Bytes", len(prev.Files), prev.Size)}
	if len(prev.Files) == 0 {
		return p, fmt.Errorf("nichts aufzuräumen")
	}
	p.run = func() (string, error) {
		done, err := a.SourceCleanup()
		return fmt.Sprintf("%d Dateien gelöscht", len(done.Files)), err
	}
	return p, nil
}

func prepareNetwork(a *App, raw json.RawMessage) (actionPlan, error) {
	var f struct {
		Mode string `json:"modus"`
	}
	if err := fields(raw, &f); err != nil {
		return actionPlan{}, err
	}
	p := actionPlan{title: "Netzmodus " + f.Mode}
	if f.Mode != NetworkAuto && f.Mode != NetworkOnline && f.Mode != NetworkOffline {
		return p, fmt.Errorf("modus %q unbekannt (auto, online, offline)", f.Mode)
	}
	p.run = func() (string, error) {
		v, err := a.SourceSetNetwork(f.Mode)
		return fmt.Sprintf("Modus %s, offline=%v", v.Mode, v.Offline), err
	}
	return p, nil
}
