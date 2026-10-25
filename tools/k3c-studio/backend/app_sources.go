package backend

import (
	"errors"
	"fmt"
	"path"
	"slices"
	"time"

	assets "k3c/tools/assetkit"
)

// Seite „Quellen“ (B-601, WZ29.6): Vorschlagsliste data/assets-vorschlaege.json, Download nach Bestätigung in den Vorrat
// .work/vorrat/, Übernahme nach public/ mit Credit und Katalog-Eintrag, Aufräumen. Felder wie api/sourceTypes.ts.

// SourceProposal ist ein Eintrag der Liste mit Status aus Liste und Vorrat.
type SourceProposal struct {
	assets.Suggestion
	LicenseOK bool   `json:"lizenzErlaubt"`
	Reason    string `json:"sperrgrund,omitempty"`
	Error     string `json:"fehler,omitempty"`
	Checksum  string `json:"pruefsumme,omitempty"`
	Preview   string `json:"vorschau,omitempty"`
	Used      bool   `json:"verwendet"`
}

// SourceInfo ist die Übersicht je Quelle mit Lizenzregeln und Zählern.
type SourceInfo struct {
	Name     string `json:"name"`
	Rules    string `json:"lizenzregeln"`
	Proposed int    `json:"vorgeschlagen"`
	Loaded   int    `json:"geladen"`
	Used     int    `json:"verwendet"`
}

// SourcesView ist die Antwort von Sources; offline = kein Internet, die Liste bleibt sichtbar.
type SourcesView struct {
	Offline   bool             `json:"offline"`
	Sources   []SourceInfo     `json:"quellen"`
	Proposals []SourceProposal `json:"vorschlaege"`
}

// SourceProgress ist der Fortschritt des Downloads eines Eintrags.
type SourceProgress struct {
	ID     string `json:"id"`
	Status string `json:"status"` // wartet, laedt, fertig, fehler
	Loaded int64  `json:"geladen"`
	Total  int64  `json:"gesamt"`
	Error  string `json:"fehler,omitempty"`
}

// SourceCleanupFile ist eine Datei, die das Aufräumen entfernt (oder entfernt hat).
type SourceCleanupFile struct {
	ID   string `json:"id"`
	Name string `json:"dateiname"`
	Size int64  `json:"groesse"`
}

// SourceCleanupView ist die Vorschau oder das Ergebnis des Aufräumens.
type SourceCleanupView struct {
	Files []SourceCleanupFile `json:"dateien"`
	Size  int64               `json:"groesse"`
}

// sourceRules sind die Lizenzregeln je Quelle in einem Satz (Q70).
var sourceRules = map[string]string{
	"OpenGameArt": "Lizenz je Datei auf der Seite prüfen: CC0, CC-BY, OGA-BY, CC-BY-SA übernehmbar, NC und ND nie.",
	"Kenney":      "CC0, Namensnennung freiwillig.",
	"itch.io":     "Lizenz je Pack auf der Seite: nur CC0, CC-BY, OGA-BY, CC-BY-SA.",
	"freesound":   "CC0 oder CC-BY je Ton; Sampling+ und NC nie.",
}

// Sources liefert Quellen-Übersicht und Liste mit Status (Binding); offline prüft eine kurze Erreichbarkeitsprobe.
func (a *App) Sources() (SourcesView, error) {
	list, err := assets.LoadSuggestions(a.root)
	if err != nil {
		return SourcesView{}, err
	}
	states, err := assets.VorratStates(a.root, list.List)
	if err != nil {
		return SourcesView{}, err
	}
	st := a.sourceState()
	errs := st.errors()
	view := SourcesView{Offline: st.offline(a.context()), Proposals: []SourceProposal{}}
	for _, s := range list.List {
		view.Proposals = append(view.Proposals, proposal(s, states[s.ID], errs[s.ID]))
	}
	view.Sources = sourceInfos(view.Proposals)
	return view, nil
}

func proposal(s assets.Suggestion, st assets.VorratState, fault string) SourceProposal {
	s.Status = st.Status
	p := SourceProposal{Suggestion: s, LicenseOK: assets.SuggestionLicenseAllowed(s.License), Reason: st.Reason,
		Error: fault, Used: st.Used}
	if st.Entry != nil {
		p.Checksum, p.Preview = st.Entry.SHA256, vorratURL+st.Entry.File
	}
	return p
}

// sourceInfos zählt je Quelle; die bekannten Quellen stehen vorn, unbekannte folgen in der Reihenfolge der Liste.
func sourceInfos(list []SourceProposal) []SourceInfo {
	names := slices.Clone(assets.SuggestionSources)
	for _, p := range list {
		if !slices.Contains(names, p.Source) {
			names = append(names, p.Source)
		}
	}
	out := make([]SourceInfo, len(names))
	for i, n := range names {
		out[i] = SourceInfo{Name: n, Rules: sourceRules[n]}
		for _, p := range list {
			if p.Source != n {
				continue
			}
			out[i].Proposed++
			if p.Status == assets.VorratLoaded {
				out[i].Loaded++
			}
			if p.Used {
				out[i].Used++
			}
		}
	}
	return out
}

// SourceDownload startet den Download der Einträge im Hintergrund. confirmedBytes ist die Gesamtgröße, die Ralf im
// Dialog bestätigt hat; weicht sie von der Summe der Einträge ab, lädt nichts (Bestätigung auch im Backend, B-601).
// Scheitert offline, bei gesperrten oder unbekannten Einträgen und während ein Download läuft.
func (a *App) SourceDownload(ids []string, confirmedBytes int64) error {
	list, err := assets.LoadSuggestions(a.root)
	if err != nil {
		return err
	}
	var todo []assets.Suggestion
	for _, id := range ids {
		i := slices.IndexFunc(list.List, func(s assets.Suggestion) bool { return s.ID == id })
		if i < 0 {
			return errors.New("unbekannter Eintrag " + id)
		}
		if _, _, err := assets.CheckVorratDownload(list.List[i]); err != nil {
			return err
		}
		todo = append(todo, list.List[i])
	}
	if len(todo) == 0 {
		return errors.New("nichts ausgewählt")
	}
	var sum int64
	for _, s := range todo {
		sum += s.Size
	}
	if sum != confirmedBytes {
		return fmt.Errorf("bestätigt %d Bytes, die Auswahl hat %d: bitte neu bestätigen", confirmedBytes, sum)
	}
	st := a.sourceState()
	if st.offline(a.context()) {
		return errors.New("offline: Downloads brauchen Internet")
	}
	if !st.begin(todo) {
		return errors.New("es läuft schon ein Download")
	}
	go st.run(a.context(), a.root, todo)
	return nil
}

// SourceProgress liefert den Fortschritt der laufenden und eben beendeten Downloads.
func (a *App) SourceProgress() []SourceProgress { return a.sourceState().snapshot() }

// SourceAdopt kopiert einen geladenen Eintrag an seinen Platz unter public/ (Credit, Katalog, Zuordnung).
func (a *App) SourceAdopt(id string) error {
	if _, err := assets.VorratAdopt(a.root, id, time.Now()); err != nil {
		return err
	}
	a.publish(evAssetsChanged)
	return nil
}

// SourceCleanupPreview nennt die geladenen, unverwendeten Dateien, die das Aufräumen entfernen würde.
func (a *App) SourceCleanupPreview() (SourceCleanupView, error) {
	list, err := assets.VorratCleanupPreview(a.root)
	return cleanupView(list), err
}

// SourceCleanup entfernt sie aus dem Vorrat (nach Bestätigung); die Einträge sind danach wieder offen.
func (a *App) SourceCleanup() (SourceCleanupView, error) {
	list, err := assets.VorratCleanup(a.root)
	return cleanupView(list), err
}

func cleanupView(list []assets.VorratEntry) SourceCleanupView {
	view := SourceCleanupView{Files: []SourceCleanupFile{}}
	for _, e := range list {
		view.Files = append(view.Files, SourceCleanupFile{ID: e.ID, Name: path.Base(e.File), Size: e.Size})
		view.Size += e.Size
	}
	return view
}

// publish meldet ein Ereignis, wenn ein Event-Bus da ist (Tests legen App teils ohne an).
func (a *App) publish(name string) {
	if a.bus != nil {
		a.bus.publish(name)
	}
}
