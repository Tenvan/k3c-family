package backend

import (
	"errors"

	assets "k3c/tools/assetkit"
)

// Vorschläge füllen (B-673): Übernahme aus den Recherche-Treffern der Workbench und das Schreibwerkzeug für den Chat.
// Beide laufen über assets.AddSuggestions (Prüfung je Eintrag, Duplikate nach Datei-URL, atomar); die Größe kommt per
// HEAD, deshalb sind sie offline gesperrt. Geladen wird nichts.

func (a *App) needOnline() error {
	if a.sourceState().offline(a.context()) {
		return errors.New("offline: Größe per HEAD nicht möglich, bitte online erneut versuchen")
	}
	return nil
}

// SourceResearchPreview nennt, welche Recherche-Treffer (source ≠ "" nur dieser Quelle) übernommen würden und welche
// mit Grund übersprungen werden; schreibt nichts.
func (a *App) SourceResearchPreview(source string) (assets.SuggestionAdd, error) {
	if err := a.needOnline(); err != nil {
		return assets.SuggestionAdd{}, err
	}
	return assets.PlanResearchImport(a.context(), a.root, source, assets.HeadSize(a.sourceState().client))
}

// SourceResearchImport übernimmt die Treffer, deren Datei-URL in urls steht (die im Dialog bestätigten), als Vorschläge
// „offen“ in data/assets-vorschlaege.json.
func (a *App) SourceResearchImport(source string, urls []string) (assets.SuggestionAdd, error) {
	if err := a.needOnline(); err != nil {
		return assets.SuggestionAdd{}, err
	}
	if len(urls) == 0 {
		return assets.SuggestionAdd{}, errors.New("nichts bestätigt")
	}
	res, err := assets.ImportResearch(a.context(), a.root, source, urls, assets.HeadSize(a.sourceState().client))
	if err == nil && len(res.Added) > 0 {
		a.publish(evAssetsChanged)
	}
	return res, err
}

// SourceSuggestAdd ist das Schreibwerkzeug des Chats: hängt Kandidaten als „offen“ an die Vorschlagsliste. ID, Status und
// Größe vergibt das Backend (Größe per HEAD); ungültige Einträge (Lizenz, Quelle, Slot, Art, Endung, doppelte Datei-URL)
// stehen mit Grund im Ergebnis, nichts wird geladen.
func (a *App) SourceSuggestAdd(entries []assets.Suggestion) (assets.SuggestionAdd, error) {
	if err := a.needOnline(); err != nil {
		return assets.SuggestionAdd{}, err
	}
	in := make([]assets.Suggestion, len(entries))
	for i, e := range entries {
		e.ID, e.Status, e.Size = "", "", 0
		if e.Priority < 1 || e.Priority > 3 {
			e.Priority = 2
		}
		in[i] = e
	}
	res, err := assets.AddSuggestions(a.context(), a.root, in, assets.HeadSize(a.sourceState().client))
	if err == nil && len(res.Added) > 0 {
		a.publish(evAssetsChanged)
	}
	return res, err
}
