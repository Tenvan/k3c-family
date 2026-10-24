package assets

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"net/url"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// SuggestionsPath hält die Download-Vorschlagsliste für Ton und Grafik (B-601, WZ29.5); das Studio lädt daraus in den
// Vorrat `.work/vorrat/` (WZ29.6).
const SuggestionsPath = "data/assets-vorschlaege.json"

// MinSuggestions ist die Mindestzahl Kandidaten je Lücke und je Skill-Linie (B-601/AC-01).
const MinSuggestions = 3

// MaxSuggestionBytes begrenzt einen Download (Zip-Packs eingeschlossen); größere Einträge lehnt die Prüfung ab.
const MaxSuggestionBytes = 50 << 20

// SuggestionSources sind die Quellen der Liste, zugleich die Schlüssel der Quellen-Übersicht im Studio.
var SuggestionSources = []string{"OpenGameArt", "Kenney", "itch.io", "freesound"}

// SuggestionStatus sind die Status eines Eintrags; „gesperrt“ setzt das Backend, wenn die Quelle eine andere Lizenz nennt.
var SuggestionStatus = []string{"offen", "geladen", "verworfen", "gesperrt"}

// Suggestion ist ein Eintrag der Vorschlagsliste (Felder wie `SourceProposal` im Studio-Frontend, ohne Laufzeitfelder).
type Suggestion struct {
	ID       string `json:"id"`
	Slot     string `json:"slot"` // Slot-ID oder Gruppe (`skill-effekt:<linie>`)
	Kind     string `json:"art"`
	Title    string `json:"titel"`
	Source   string `json:"quelle"`
	Page     string `json:"seite"`
	FileURL  string `json:"dateiUrl"`
	Artist   string `json:"urheber,omitempty"`
	License  string `json:"lizenz"`
	FileName string `json:"dateiname"`
	Size     int64  `json:"groesse"`
	Priority int    `json:"prioritaet"`
	Note     string `json:"hinweis,omitempty"`
	Status   string `json:"status"`
}

// Suggestions ist der Inhalt von data/assets-vorschlaege.json.
type Suggestions struct {
	Comment string       `json:"_comment"`
	List    []Suggestion `json:"vorschlaege"`
}

// LoadSuggestions liest die Liste; fehlt die Datei, ist sie leer. Ungültiges JSON ist ein Fehler.
func LoadSuggestions(root string) (*Suggestions, error) {
	b, err := os.ReadFile(filepath.Join(root, SuggestionsPath))
	if errors.Is(err, fs.ErrNotExist) {
		return &Suggestions{}, nil
	}
	if err != nil {
		return nil, err
	}
	var s Suggestions
	if err := json.Unmarshal(b, &s); err != nil {
		return nil, fmt.Errorf("%s: %w", SuggestionsPath, err)
	}
	return &s, nil
}

// SuggestionLicenseAllowed: Lizenz nach Q70 übernehmbar (CC0, CC-BY, OGA-BY, CC-BY-SA); NC und ND nie, Sampling+ nie.
func SuggestionLicenseAllowed(license string) bool {
	l := strings.ToUpper(license)
	if strings.Contains(l, "SAMPLING") {
		return false
	}
	if f := LicenseFlag(license); f == "ND" || f == "NC" {
		return false
	}
	return strings.HasPrefix(l, "CC0") || strings.HasPrefix(l, "CC-BY") || strings.HasPrefix(l, "OGA-BY")
}

// SuggestionTargets sind die Ziele, die je MinSuggestions Kandidaten brauchen: jede Lücke des Registers (ihre Art) und
// je Skill-Linie (`sfx:skill:<linie>`) ein Ton für den Slot und eine Grafik für die Gruppe `skill-effekt:<linie>`.
// Schlüssel ist das Ziel, Wert die Art.
func SuggestionTargets(reg *Register) map[string]string {
	out := map[string]string{}
	for _, s := range reg.Slots {
		if len(s.Choices) == 0 {
			out[s.ID] = s.Kind
		}
		if line, ok := strings.CutPrefix(s.ID, "sfx:skill:"); ok {
			out[s.ID] = KindSound
			out["skill-effekt:"+line] = KindGraphic
		}
	}
	return out
}

// CheckSuggestions prüft Pflichtfelder, Lizenz, Größe und eindeutige IDs und Datei-URLs je Eintrag sowie die
// Mindestzahl je Ziel (B-601/AC-01); leer = in Ordnung.
func CheckSuggestions(list []Suggestion, targets map[string]string) []error {
	var errs []error
	ids, urls := map[string]bool{}, map[string]bool{}
	count := map[string]int{}
	for _, s := range list {
		if err := checkSuggestion(s); err != nil {
			errs = append(errs, fmt.Errorf("%s: %w", s.ID, err))
		}
		if ids[s.ID] || urls[s.FileURL] {
			errs = append(errs, fmt.Errorf("%s: ID oder Datei-URL doppelt", s.ID))
		}
		ids[s.ID], urls[s.FileURL] = true, true
		if kind, ok := targets[s.Slot]; ok && kind == s.Kind && s.Status != "verworfen" {
			count[s.Slot]++
		}
	}
	keys := make([]string, 0, len(targets))
	for k := range targets {
		keys = append(keys, k)
	}
	slices.Sort(keys)
	for _, k := range keys {
		if count[k] < MinSuggestions {
			errs = append(errs, fmt.Errorf("%s (%s): %d Kandidaten, mindestens %d", k, targets[k], count[k], MinSuggestions))
		}
	}
	return errs
}

func checkSuggestion(s Suggestion) error {
	if s.ID == "" || s.Slot == "" || s.Title == "" || s.FileName == "" {
		return errors.New("id, slot, titel und dateiname sind Pflicht")
	}
	if err := checkSuggestionValues(s); err != nil {
		return err
	}
	for _, u := range []string{s.Page, s.FileURL} {
		if p, err := url.Parse(u); err != nil || p.Scheme != "https" || p.Host == "" {
			return fmt.Errorf("adresse %q ist keine https-URL", u)
		}
	}
	return nil
}

// checkSuggestionValues prüft Art, Quelle, Lizenz, Größe, Priorität und Status gegen ihre Wertebereiche.
func checkSuggestionValues(s Suggestion) error {
	switch {
	case s.Kind != KindGraphic && s.Kind != KindSound:
		return fmt.Errorf("art %q unbekannt", s.Kind)
	case !slices.Contains(SuggestionSources, s.Source):
		return fmt.Errorf("quelle %q unbekannt", s.Source)
	case !SuggestionLicenseAllowed(s.License):
		return fmt.Errorf("lizenz %q nicht erlaubt (Q70)", s.License)
	case s.Size <= 0 || s.Size > MaxSuggestionBytes:
		return fmt.Errorf("groesse %d außerhalb 1 … %d", s.Size, MaxSuggestionBytes)
	case s.Priority < 1 || s.Priority > 3:
		return fmt.Errorf("prioritaet %d außerhalb 1 … 3", s.Priority)
	case !slices.Contains(SuggestionStatus, s.Status):
		return fmt.Errorf("status %q unbekannt", s.Status)
	}
	return nil
}
