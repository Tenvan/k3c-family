package assets

import (
	"cmp"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"net/http"
	"net/url"
	"os"
	"path"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"time"
)

// headTimeout ist die Frist je HEAD-Anfrage (Größe einer Datei).
const headTimeout = 10 * time.Second

// Neue Vorschläge anhängen (B-673): eine Funktion für Studio (Chat, Recherche-Übernahme) und Workbench. Jeder Eintrag
// durchläuft checkSuggestion; ungültige und doppelte (nach Datei-URL) werden mit Grund übersprungen, die übrigen in einem
// Schritt atomar geschrieben. Geladen wird nichts.

// ErrSuggestionsChanged: Die Vorschlagsliste hat sich während der Prüfung geändert, es wurde nichts geschrieben.
var ErrSuggestionsChanged = errors.New("die Vorschlagsliste wurde währenddessen geändert, nichts geschrieben: bitte neu laden")

// SizeFunc liefert die Größe der Datei unter rawURL in Bytes (im Betrieb per HEAD, im Test ein Ersatz).
type SizeFunc func(ctx context.Context, rawURL string) (int64, error)

// SkippedSuggestion ist ein Kandidat, der nicht angehängt wird, mit Grund.
type SkippedSuggestion struct {
	Title   string `json:"titel"`
	FileURL string `json:"dateiUrl"`
	Reason  string `json:"grund"`
}

// SuggestionAdd ist das Ergebnis (oder der Plan) eines Anhängens.
type SuggestionAdd struct {
	Added   []Suggestion        `json:"uebernommen"`
	Skipped []SkippedSuggestion `json:"uebersprungen"`
	Bytes   int64               `json:"bytes"`
}

// HeadSize fragt die Größe per HEAD (Content-Length); Ziel muss eine erlaubte Quelle sein, Weiterleitungen werden geprüft.
func HeadSize(client *http.Client) SizeFunc {
	c := *client
	c.CheckRedirect = VorratRedirect
	return func(ctx context.Context, rawURL string) (int64, error) {
		u, err := url.Parse(rawURL)
		if err != nil {
			return 0, err
		}
		if err := VorratURLAllowed(u); err != nil {
			return 0, err
		}
		ctx, cancel := context.WithTimeout(ctx, headTimeout) // ein hängender Host blockiert nicht den ganzen Lauf
		defer cancel()
		req, err := http.NewRequestWithContext(ctx, http.MethodHead, u.String(), nil)
		if err != nil {
			return 0, err
		}
		req.Header.Set("User-Agent", userAgent)
		res, err := c.Do(req)
		if err != nil {
			return 0, err
		}
		_ = res.Body.Close()
		if res.StatusCode != http.StatusOK {
			return 0, fmt.Errorf("HEAD: HTTP %d", res.StatusCode)
		}
		if res.ContentLength <= 0 {
			return 0, errors.New("HEAD: keine Größe angegeben")
		}
		return res.ContentLength, nil
	}
}

// loadSuggestionsStamped liest die Liste samt Prüfsumme ihrer Bytes (leer, wenn die Datei fehlt).
func loadSuggestionsStamped(root string) (*Suggestions, string, error) {
	b, err := os.ReadFile(filepath.Join(root, SuggestionsPath))
	if errors.Is(err, fs.ErrNotExist) {
		return &Suggestions{}, "", nil
	}
	if err != nil {
		return nil, "", err
	}
	var list Suggestions
	if err := json.Unmarshal(b, &list); err != nil {
		return nil, "", fmt.Errorf("%s: %w", SuggestionsPath, err)
	}
	sum := sha256.Sum256(b)
	return &list, hex.EncodeToString(sum[:]), nil
}

// PlanSuggestions prüft die Kandidaten gegen Register und Liste und nennt, was angehängt würde; schreibt nichts.
func PlanSuggestions(ctx context.Context, root string, in []Suggestion, size SizeFunc) (SuggestionAdd, error) {
	res, _, _, err := planSuggestions(ctx, root, in, size)
	return res, err
}

// AddSuggestions hängt die gültigen Kandidaten als „offen“ an die Liste und schreibt sie atomar; die übrigen stehen mit
// Grund in Skipped. Fehlt Size, fragt size die Datei (HEAD). Ändert sich die Datei währenddessen: ErrSuggestionsChanged.
// Grenze: suggestionsMu sperrt nur im eigenen Prozess; schreibt ein anderer Prozess (Workbench) genau zwischen Vergleich
// und writeAtomic, gewinnt der spätere Schreiber. Bewusst ohne Lock-Datei, weil das Fenster sehr klein ist.
func AddSuggestions(ctx context.Context, root string, in []Suggestion, size SizeFunc) (SuggestionAdd, error) {
	res, list, stamp, err := planSuggestions(ctx, root, in, size)
	if err != nil || len(res.Added) == 0 {
		return res, err
	}
	suggestionsMu.Lock()
	defer suggestionsMu.Unlock()
	_, now, err := loadSuggestionsStamped(root)
	if err != nil {
		return SuggestionAdd{}, err
	}
	if now != stamp {
		return SuggestionAdd{}, ErrSuggestionsChanged
	}
	list.List = append(list.List, res.Added...)
	return res, list.save(root)
}

func planSuggestions(ctx context.Context, root string, in []Suggestion, size SizeFunc) (SuggestionAdd, *Suggestions, string, error) {
	res := SuggestionAdd{Added: []Suggestion{}, Skipped: []SkippedSuggestion{}}
	list, stamp, err := loadSuggestionsStamped(root)
	if err != nil {
		return res, nil, "", err
	}
	reg, err := Load(root)
	if err != nil {
		return res, nil, "", err
	}
	targets := SuggestionTargets(reg)
	urls := map[string]bool{}
	next := 0
	for _, s := range list.List {
		urls[s.FileURL] = true
		if n, err := strconv.Atoi(strings.TrimPrefix(s.ID, "v")); err == nil {
			next = max(next, n)
		}
	}
	for _, c := range in {
		s, err := prepareSuggestion(ctx, c, reg, targets, urls, size)
		if err != nil {
			res.Skipped = append(res.Skipped, SkippedSuggestion{Title: c.Title, FileURL: c.FileURL, Reason: err.Error()})
			continue
		}
		next++
		s.ID = fmt.Sprintf("v%03d", next)
		urls[s.FileURL] = true
		res.Added = append(res.Added, s)
		res.Bytes += s.Size
	}
	return res, list, stamp, nil
}

// prepareSuggestion ergänzt Vorgaben (Status offen, Priorität 2, Dateiname, Größe) und prüft den Kandidaten.
func prepareSuggestion(ctx context.Context, s Suggestion, reg *Register, targets map[string]string, urls map[string]bool, size SizeFunc) (Suggestion, error) {
	s.FileURL = strings.TrimSpace(s.FileURL)
	if urls[s.FileURL] {
		return s, errors.New("Datei-URL schon in der Liste") //nolint:staticcheck // Grund für die Oberfläche
	}
	if i := slices.IndexFunc(SuggestionSources, func(n string) bool { return strings.EqualFold(n, s.Source) }); i >= 0 {
		s.Source = SuggestionSources[i]
	} else {
		return s, errors.New("Quelle nicht erlaubt") //nolint:staticcheck // Grund für die Oberfläche
	}
	kind, ok := targets[s.Slot]
	if !ok {
		if slot := reg.Find(s.Slot); slot != nil {
			kind, ok = slot.Kind, true
		}
	}
	if !ok {
		return s, fmt.Errorf("slot %q nicht im Register", s.Slot)
	}
	if s.Kind != kind {
		return s, fmt.Errorf("art %q passt nicht zum Slot (%s)", s.Kind, kind)
	}
	s.Status, s.Priority = "offen", cmp.Or(s.Priority, 2)
	if s.FileName == "" {
		if u, err := url.Parse(s.FileURL); err == nil {
			s.FileName = path.Base(u.Path)
		}
	}
	if s.Size <= 0 && size != nil && checkURLs(s) == nil {
		n, err := size(ctx, s.FileURL)
		if err != nil {
			return s, fmt.Errorf("Größe nicht ermittelbar: %w", err) //nolint:staticcheck // Grund für die Oberfläche
		}
		s.Size = n
	}
	s.ID = "neu"
	if err := checkSuggestion(s); err != nil {
		return s, err
	}
	u, _ := url.Parse(s.FileURL)
	return s, VorratURLAllowed(u)
}

func checkURLs(s Suggestion) error {
	for _, raw := range []string{s.Page, s.FileURL} {
		if u, err := url.Parse(raw); err != nil || u.Scheme != "https" || u.Host == "" {
			return fmt.Errorf("adresse %q ist keine https-URL", raw)
		}
	}
	return nil
}
