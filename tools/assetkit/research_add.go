package assets

import (
	"context"
	"net/url"
	"path"
	"slices"
	"strings"
)

// Recherche-Treffer als Vorschläge übernehmen (B-673): Treffer mit direkter Datei-URL, erlaubter Quelle und Lizenz
// erkannt oder bestätigt werden zu Kandidaten; alle anderen stehen mit Grund in der Liste der übersprungenen. Die Treffer
// bleiben in der Recherche stehen (der Vorschlag verweist per Datei-URL auf dieselbe Datei).

// ResearchCandidates wandelt die Treffer in Kandidaten. source ≠ "" nimmt nur Treffer dieser Quelle. Slots in
// alphabetischer Reihenfolge, damit Plan und Übernahme dieselbe Reihenfolge haben.
func ResearchCandidates(root, source string) ([]Suggestion, []SkippedSuggestion, error) {
	r, err := LoadResearch(root)
	if err != nil {
		return nil, nil, err
	}
	reg, err := Load(root)
	if err != nil {
		return nil, nil, err
	}
	targets := SuggestionTargets(reg)
	slots := make([]string, 0, len(r.Hits))
	for s := range r.Hits {
		slots = append(slots, s)
	}
	slices.Sort(slots)
	cands, skipped := []Suggestion{}, []SkippedSuggestion{}
	for _, slot := range slots {
		for _, h := range r.Hits[slot] {
			if source != "" && !strings.EqualFold(h.Source, source) {
				continue
			}
			c, why := hitCandidate(slot, h, reg, targets)
			if why != "" {
				skipped = append(skipped, SkippedSuggestion{Title: h.Title, FileURL: h.Download, Reason: why})
				continue
			}
			cands = append(cands, c)
		}
	}
	return cands, skipped, nil
}

func hitCandidate(slot string, h Hit, reg *Register, targets map[string]string) (Suggestion, string) {
	switch {
	case !slices.ContainsFunc(SuggestionSources, func(n string) bool { return strings.EqualFold(n, h.Source) }):
		return Suggestion{}, "Quelle nicht erlaubt"
	case strings.TrimSpace(h.Download) == "":
		return Suggestion{}, "keine direkte Datei-URL"
	case h.LicenseStatus != LicenseKnown && h.LicenseStatus != LicenseConfirmed:
		return Suggestion{}, "Lizenz nicht geklärt (" + h.LicenseStatus + ")"
	}
	license := normalizeLicense(h.License)
	if !SuggestionLicenseAllowed(license) {
		return Suggestion{}, "Lizenz „" + h.License + "“ nicht erlaubt (Q70)"
	}
	kind, ok := targets[slot]
	if s := reg.Find(slot); !ok && s != nil {
		kind, ok = s.Kind, true
	}
	if !ok {
		return Suggestion{}, "Slot nicht im Register"
	}
	name := ""
	if u, err := url.Parse(h.Download); err == nil {
		name = path.Base(u.Path)
	}
	return Suggestion{Slot: slot, Kind: kind, Title: h.Title, Source: h.Source, Page: h.Link, FileURL: h.Download,
		Artist: h.Artist, License: license, FileName: name, Priority: 2}, ""
}

// normalizeLicense schreibt „CC BY 4.0“ als „CC-BY 4.0“, wie die Liste und SuggestionLicenseAllowed es erwarten.
func normalizeLicense(l string) string {
	l = strings.TrimSpace(l)
	if strings.HasPrefix(strings.ToUpper(l), "CC BY") {
		return "CC-BY" + l[len("CC BY"):]
	}
	return l
}

// PlanResearchImport nennt, was die Übernahme anhängen würde (Größe per size), und was sie überspringt; schreibt nichts.
func PlanResearchImport(ctx context.Context, root, source string, size SizeFunc) (SuggestionAdd, error) {
	cands, skipped, err := ResearchCandidates(root, source)
	if err != nil {
		return SuggestionAdd{}, err
	}
	res, err := PlanSuggestions(ctx, root, cands, size)
	res.Skipped = append(skipped, res.Skipped...)
	return res, err
}

// ImportResearch hängt die Treffer an die Vorschlagsliste. only ≠ nil nimmt nur Treffer mit diesen Datei-URLs (die
// im Dialog bestätigten); die übrigen werden nicht aufgeführt.
func ImportResearch(ctx context.Context, root, source string, only []string, size SizeFunc) (SuggestionAdd, error) {
	cands, skipped, err := ResearchCandidates(root, source)
	if err != nil {
		return SuggestionAdd{}, err
	}
	if only != nil {
		cands = slices.DeleteFunc(cands, func(c Suggestion) bool { return !slices.Contains(only, c.FileURL) })
	}
	res, err := AddSuggestions(ctx, root, cands, size)
	res.Skipped = append(skipped, res.Skipped...)
	return res, err
}
