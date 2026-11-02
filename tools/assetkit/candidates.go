package assets

import (
	"cmp"
	"fmt"
	"net/url"
	"os"
	"path"
	"slices"
	"strings"
)

// Kandidaten eines Asset-Slots (B-677, WZ33): Zuordnungen, Vorrat, Netz (Vorschläge, Recherche) und Katalog in einer
// Liste, ohne HTTP und Oberfläche. Felder und Werte wie api/candidateTypes.ts des Studio-Frontends.

// Stufe eines Kandidaten auf dem Weg ins Spiel.
const (
	StageNet   = "netz"
	StageStock = "vorrat"
	StageGame  = "spiel"
)

// Lizenz-Marke eines Kandidaten.
const (
	MarkFree    = "frei"
	MarkSA      = "sa"
	MarkCheck   = "pruefen"
	MarkBlocked = "gesperrt"
)

// Herkunft eines Kandidaten.
const (
	OriginCatalog    = "katalog"
	OriginStock      = "vorrat"
	OriginSuggestion = "vorschlag"
	OriginResearch   = "recherche"
)

// Präfixe der Kandidaten-IDs.
const (
	idChoice     = "wahl:"
	idCatalog    = "cat:"
	idSuggestion = "sug:"
	idHit        = "hit:"
)

// defaultCandidateLimit gilt für die Katalog-Treffer, wenn limit ≤ 0 ist.
const defaultCandidateLimit = 50

// Candidate ist eine Zeile der Kandidatenliste (JSON wie api/candidateTypes.ts).
type Candidate struct {
	ID             string   `json:"id"`
	Stage          string   `json:"stufe"`
	Pref           int      `json:"praef"` // 0 = nicht zugeordnet, n = Präferenz n des Slots
	Title          string   `json:"titel"`
	File           string   `json:"datei"`
	Source         string   `json:"quelle"`
	Artist         string   `json:"urheber,omitempty"`
	License        string   `json:"lizenz"`
	Mark           string   `json:"marke"`
	Reason         string   `json:"grund,omitempty"` // warum „Zuordnen“ gesperrt ist
	Size           int64    `json:"groesse"`
	Preview        string   `json:"vorschau"`
	Link           string   `json:"link,omitempty"`
	Origins        []string `json:"herkunft"`
	Assignable     bool     `json:"zuordenbar"`
	ConfirmLicense bool     `json:"lizenzBestaetigen,omitempty"` // Treffer mit Lizenz „prüfen“: 🧑 kann bestätigen
}

// CandidateOptions steuern die Liste: Katalog-Suche (leer = Beschreibung des Slots), Netz-Zustand und die
// Adress-Präfixe für Vorschauen (Dateien unter public/ und im Vorrat).
type CandidateOptions struct {
	Query     string
	Limit     int
	Offline   bool
	PublicURL string
	VorratURL string
}

const offlineReason = "offline"

type candidateBuilder struct {
	root    string
	slot    Slot
	opt     CandidateOptions
	catalog []Asset
	chosen  map[string]bool
	taken   map[string]bool       // Katalog-Dateien, die schon als Vorrat-Kandidat erscheinen
	byURL   map[string]*Candidate // Datei-URL → Kandidat; nil = URL gehört einem verworfenen oder verwendeten Eintrag
}

// Candidates liefert die Kandidaten des Slots: Zuordnungen (nach Präferenz), Vorrat, Netz (Vorschläge nach Priorität,
// dann Recherche-Treffer) und Katalog-Treffer zur Suche. Gleiche Datei-URL erscheint einmal mit allen Herkünften.
func Candidates(root, slotID string, o CandidateOptions) ([]Candidate, error) {
	reg, err := Load(root)
	if err != nil {
		return nil, err
	}
	slot := reg.Find(slotID)
	if slot == nil {
		return nil, fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	cat, err := Catalog(root)
	if err != nil {
		return nil, err
	}
	sugs, err := LoadSuggestions(root)
	if err != nil {
		return nil, err
	}
	states, err := VorratStates(root, sugs.List)
	if err != nil {
		return nil, err
	}
	hits, err := HitsOf(root, slotID)
	if err != nil {
		return nil, err
	}
	b := &candidateBuilder{root: root, slot: *slot, opt: o, catalog: cat, chosen: map[string]bool{},
		taken: map[string]bool{}, byURL: map[string]*Candidate{}}
	out := b.assigned()
	stock, net := b.suggestions(sugs.List, states)
	net = append(net, b.hits(hits)...)
	out = append(out, deref(stock)...)
	out = append(out, deref(net)...)
	return append(out, b.catalogHits()...), nil
}

func deref(list []*Candidate) []Candidate {
	out := make([]Candidate, len(list))
	for i, c := range list {
		out[i] = *c
	}
	return out
}

// assigned sind die Zuordnungen des Slots in der Reihenfolge der Präferenz; eine Datei mit mehreren Ausschnitten
// erscheint einmal (die ID ist die Datei).
func (b *candidateBuilder) assigned() []Candidate {
	byFile := map[string]Asset{}
	for _, a := range b.catalog {
		byFile[a.File] = a
	}
	out := []Candidate{}
	for i, ch := range b.slot.Choices {
		if b.chosen[ch.File] {
			continue
		}
		b.chosen[ch.File] = true
		a := byFile[ch.File]
		c := b.gameCandidate(idChoice, ch.File, a)
		c.Pref, c.Assignable, c.Reason = i+1, false, "schon zugeordnet"
		if !fileExists(publicPath(b.root, ch.File)) {
			c.Reason = "Datei fehlt unter public/"
		}
		out = append(out, c)
	}
	return out
}

// gameCandidate ist die Zeile einer Datei unter public/ (Zuordnung oder Katalog); a ist ihr Katalog-Eintrag, falls vorhanden.
func (b *candidateBuilder) gameCandidate(prefix, file string, a Asset) Candidate {
	c := Candidate{ID: prefix + file, Stage: StageGame, Title: cmp.Or(a.Title, path.Base(file)), File: file, Source: a.Source,
		Artist: a.Artist, License: a.License, Mark: licenseMark(a.License), Preview: b.opt.PublicURL + file,
		Origins: []string{OriginCatalog}, Assignable: true}
	if strings.HasPrefix(a.Source, "https://") {
		c.Link = a.Source
	}
	if st, err := os.Stat(publicPath(b.root, file)); err == nil {
		c.Size = st.Size()
	}
	return c
}

// suggestions teilt die Vorschläge des Slots in Vorrat (geladen, nicht verwendet) und Netz (offen); verworfene,
// gesperrte und verwendete Einträge fehlen, ihre Datei-URL bleibt für die Dublettenprüfung der Treffer vermerkt.
func (b *candidateBuilder) suggestions(list []Suggestion, states map[string]VorratState) (stock, net []*Candidate) {
	var mine []Suggestion
	for _, s := range list {
		if s.Slot == b.slot.ID && s.Kind == b.slot.Kind {
			mine = append(mine, s)
		}
	}
	slices.SortStableFunc(mine, func(x, y Suggestion) int { return cmp.Compare(x.Priority, y.Priority) })
	for _, s := range mine {
		st := states[s.ID]
		var c *Candidate
		switch {
		case st.Status == VorratLoaded && !st.Used:
			c = b.stockCandidate(s, *st.Entry)
			stock = append(stock, c)
		case st.Status == VorratOpen:
			c = b.netSuggestion(s)
			net = append(net, c)
		}
		if c != nil || b.byURL[s.FileURL] == nil { // ein verworfener Eintrag mit gleicher URL verdrängt keinen Kandidaten
			b.byURL[s.FileURL] = c
		}
	}
	return stock, net
}

func (b *candidateBuilder) stockCandidate(s Suggestion, e VorratEntry) *Candidate {
	c := &Candidate{ID: idSuggestion + s.ID, Stage: StageStock, Title: s.Title, File: cmp.Or(e.Public, s.FileName),
		Source: s.Source, Artist: s.Artist, License: s.License, Mark: licenseMark(s.License), Size: e.Size,
		Preview: b.opt.VorratURL + e.File, Link: s.Page, Origins: []string{OriginStock}, Assignable: true}
	if isPackage(s.FileName) {
		c.Assignable, c.Reason = false, ErrVorratPackage.Error()
	}
	if e.Public != "" && fileExists(publicPath(b.root, e.Public)) {
		b.taken[e.Public] = true // dieselbe Datei steht im Katalog: eine Zeile mit beiden Herkünften
		c.Origins = append(c.Origins, OriginCatalog)
	}
	return c
}

func (b *candidateBuilder) netSuggestion(s Suggestion) *Candidate {
	c := &Candidate{ID: idSuggestion + s.ID, Stage: StageNet, Title: s.Title, File: s.FileName, Source: s.Source,
		Artist: s.Artist, License: s.License, Mark: licenseMark(s.License), Size: s.Size, Link: s.Page,
		Origins: []string{OriginSuggestion}}
	switch _, _, err := CheckVorratDownload(s); {
	case err != nil:
		c.Reason = err.Error()
	case isPackage(s.FileName):
		c.Reason = ErrVorratPackage.Error()
	case b.opt.Offline:
		c.Reason = offlineReason
	}
	c.Assignable = c.Reason == ""
	return c
}

func isPackage(name string) bool {
	ext := strings.ToLower(path.Ext(name))
	return ext == ".zip" || ext == ".7z"
}

// hits sind die Recherche-Treffer ohne Import und ohne Ausschluss (Q70). Ein Treffer mit der Datei-URL eines Vorschlags
// ergänzt dessen Herkunft statt eine zweite Zeile zu bilden.
func (b *candidateBuilder) hits(list []Hit) []*Candidate {
	var out []*Candidate
	for _, h := range list {
		if h.File != "" || h.LicenseStatus == LicenseExcluded {
			continue
		}
		if c, seen := b.byURL[h.Download]; seen && h.Download != "" {
			if c != nil {
				c.Origins = append(c.Origins, OriginResearch)
			}
			continue
		}
		out = append(out, b.hitCandidate(h))
	}
	return out
}

func (b *candidateBuilder) hitCandidate(h Hit) *Candidate {
	c := &Candidate{ID: idHit + h.ID, Stage: StageNet, Title: h.Title, File: hitFileName(h), Source: h.Source,
		Artist: h.Artist, License: h.License, Mark: hitMark(h), Link: h.Link, Origins: []string{OriginResearch}}
	switch {
	case licenseBlocked(h.License, h.Flag):
		c.Reason = blockedReason(h.License, h.Flag)
	case h.LicenseStatus == LicenseCheck:
		c.Reason, c.ConfirmLicense = "Lizenz prüfen", true
	case !hitDownloadable(h):
		c.Reason = "kein direkter Download, Seite öffnen"
	case b.opt.Offline:
		c.Reason = offlineReason
	}
	c.Assignable = c.Reason == ""
	return c
}

// licenseBlocked: ND, NC und Sampling+ sind nie übernehmbar (Q70), auch nicht nach „Lizenz bestätigen“. flag ist das
// Kennzeichen des Treffers oder Katalog-Eintrags; zusätzlich zählt der Lizenztext.
func licenseBlocked(license, flag string) bool {
	for _, f := range []string{flag, LicenseFlag(license)} {
		if f == "ND" || f == "NC" {
			return true
		}
	}
	return strings.Contains(strings.ToUpper(license), "SAMPLING")
}

func blockedReason(license, flag string) string {
	return "Lizenz " + cmp.Or(license, flag) + " nicht erlaubt (Q70)"
}

// hitMark: unbestätigte unklare Lizenz = prüfen, von 🧑 bestätigte gilt als frei, sofern kein SA-Kennzeichen bleibt;
// gesperrte Lizenzen bleiben gesperrt.
func hitMark(h Hit) string {
	m := licenseMark(h.License)
	switch {
	case licenseBlocked(h.License, h.Flag):
		return MarkBlocked
	case h.LicenseStatus == LicenseCheck:
		return MarkCheck
	case h.LicenseStatus == LicenseConfirmed && m == MarkCheck:
		return MarkFree
	}
	return m
}

// hitDownloadable: direkter Download über https von einem Host aus DownloadHosts (wie DownloadHit).
func hitDownloadable(h Hit) bool {
	u, err := url.Parse(h.Download)
	return h.Download != "" && err == nil && u.Scheme == "https" && downloadAllowed(u)
}

func hitFileName(h Hit) string {
	if u, err := url.Parse(h.Download); err == nil && path.Base(u.Path) != "." && path.Base(u.Path) != "/" {
		return path.Base(u.Path)
	}
	return h.Title
}

// catalogHits sind die Katalog-Treffer zur Suche für die Art des Slots, ohne Zuordnungen des Slots und ohne Dateien, die
// schon als Vorrat-Kandidat erscheinen.
func (b *candidateBuilder) catalogHits() []Candidate {
	query := strings.TrimSpace(b.opt.Query)
	if query == "" {
		query = cmp.Or(searchTerm(b.slot), b.slot.ID)
	}
	limit := b.opt.Limit
	if limit <= 0 {
		limit = defaultCandidateLimit
	}
	out := []Candidate{}
	for _, a := range Search(b.catalog, query, b.slot.Kind, 0) {
		if len(out) == limit {
			break
		}
		if b.chosen[a.File] || b.taken[a.File] {
			continue
		}
		c := b.gameCandidate(idCatalog, a.File, a)
		if licenseBlocked(a.License, a.Flag) {
			c.Mark, c.Assignable, c.Reason = MarkBlocked, false, blockedReason(a.License, a.Flag)
		}
		out = append(out, c)
	}
	return out
}

// licenseMark ordnet einen Lizenztext ein: CC0, CC-BY, OGA-BY u. a. = frei, SA = sa, NC, ND, Sampling+ = gesperrt,
// leer oder unklar = pruefen.
func licenseMark(license string) string {
	up := strings.ToUpper(license)
	switch {
	case licenseBlocked(license, ""):
		return MarkBlocked
	case LicenseFlag(license) == "SA":
		return MarkSA
	case license == selfMade, license == ownBuilding:
		return MarkFree
	}
	for _, k := range knownLicenses {
		if strings.Contains(up, k) {
			return MarkFree
		}
	}
	return MarkCheck
}
