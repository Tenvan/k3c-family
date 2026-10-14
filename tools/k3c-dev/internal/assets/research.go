package assets

import (
	"bytes"
	"context"
	"crypto/sha1"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// ResearchPath hält die Recherche-Treffer je Asset-Slot (B-298), getrennt vom Slot-Register.
const ResearchPath = "data/assets-recherche.json"

// Lizenz-Status eines Recherche-Treffers nach Q70.
const (
	LicenseKnown    = "erkannt"        // übernehmbar (NC nur als markierter Lückenfüller)
	LicenseCheck    = "pruefen"        // unklar: erst nach Bestätigung durch 🧑
	LicenseExcluded = "ausgeschlossen" // ND oder Quelle ohne Weitergabe der Rohdateien
)

// Hit ist ein Recherche-Treffer: Vorschlag mit Link, Quelle und Lizenz; Download nur bei direkter Datei.
type Hit struct {
	ID            string `json:"id"`
	Title         string `json:"titel"`
	Link          string `json:"link"`
	Source        string `json:"quelle"`
	Artist        string `json:"urheber,omitempty"`
	License       string `json:"lizenz,omitempty"`
	Preview       string `json:"vorschau,omitempty"`
	Download      string `json:"download,omitempty"`
	LicenseStatus string `json:"lizenzStatus"`
	Flag          string `json:"kennzeichen,omitempty"`
	Created       string `json:"erstellt"`
}

// Research ist der Inhalt von data/assets-recherche.json: Treffer je Slot-ID.
type Research struct {
	Comment string           `json:"_comment"`
	Hits    map[string][]Hit `json:"treffer"`
}

const researchComment = "Recherche-Treffer je Asset-Slot (B-298, M10.4): Vorschläge mit Link, Quelle, Urheber, Lizenz und Vorschau. " +
	"Schreibt k3c-dev (Recherche, MCP assets_hit_add); ein Treffer bleibt, bis 🧑 ihn verwirft. lizenzStatus: erkannt, pruefen, ausgeschlossen (Q70)."

// LoadResearch liest die Treffer; fehlt die Datei, ist sie leer. Ungültiges JSON ist ein Fehler (nie überschreiben).
func LoadResearch(root string) (*Research, error) {
	b, err := os.ReadFile(filepath.Join(root, ResearchPath))
	if errors.Is(err, fs.ErrNotExist) {
		return &Research{Comment: researchComment, Hits: map[string][]Hit{}}, nil
	}
	if err != nil {
		return nil, fmt.Errorf("%s: %w", ResearchPath, err)
	}
	r := Research{}
	if err := json.Unmarshal(b, &r); err != nil {
		return nil, fmt.Errorf("%s ungültig, nichts geschrieben: %w", ResearchPath, err)
	}
	if r.Hits == nil {
		r.Hits = map[string][]Hit{}
	}
	return &r, nil
}

func (r *Research) save(root string) error {
	var buf bytes.Buffer
	enc := json.NewEncoder(&buf)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	if err := enc.Encode(r); err != nil {
		return err
	}
	return writeAtomic(filepath.Join(root, ResearchPath), buf.Bytes())
}

// HitsOf liefert die gespeicherten Treffer eines Slots.
func HitsOf(root, slotID string) ([]Hit, error) {
	r, err := LoadResearch(root)
	if err != nil {
		return nil, err
	}
	return r.Hits[slotID], nil
}

// AddHits legt Treffer zu einem vorhandenen Asset-Slot ab; gleicher Link wird nicht doppelt angelegt.
// Die Lizenz wird nach Q70 eingeordnet, sofern der Treffer keinen Status mitbringt. Antwort: Zahl neuer Treffer.
func AddHits(root, slotID string, hits []Hit, now time.Time) (int, error) {
	mu.Lock()
	defer mu.Unlock()
	reg, err := Load(root)
	if err != nil {
		return 0, err
	}
	if reg.Find(slotID) == nil {
		return 0, fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	r, err := LoadResearch(root)
	if err != nil {
		return 0, err
	}
	added := 0
	for _, h := range hits {
		if h.Link == "" || h.Title == "" {
			return 0, errors.New("treffer braucht titel und link")
		}
		if hasLink(r.Hits[slotID], h.Link) {
			continue
		}
		h.ID = hitID(h.Link)
		h.Created = now.Format("2006-01-02")
		// Immer nach Q70 einordnen; ein vorsichtigeres „pruefen“ des Aufrufers bleibt stehen.
		st, flag := ClassifyLicense(h.License, h.Link)
		if h.LicenseStatus != LicenseCheck || st != LicenseKnown {
			h.LicenseStatus = st
		}
		h.Flag = flag
		r.Hits[slotID] = append(r.Hits[slotID], h)
		added++
	}
	if added == 0 {
		return 0, nil
	}
	return added, r.save(root)
}

// DropHit verwirft einen Treffer (🧑 entscheidet); ein unbekannter Treffer ist ein Fehler.
func DropHit(root, slotID, id string) error {
	mu.Lock()
	defer mu.Unlock()
	r, err := LoadResearch(root)
	if err != nil {
		return err
	}
	list := r.Hits[slotID]
	for i, h := range list {
		if h.ID == id {
			r.Hits[slotID] = append(list[:i:i], list[i+1:]...)
			if len(r.Hits[slotID]) == 0 {
				delete(r.Hits, slotID)
			}
			return r.save(root)
		}
	}
	return fmt.Errorf("treffer %q zu %s nicht gefunden", id, slotID)
}

func hasLink(list []Hit, link string) bool {
	for _, h := range list {
		if h.Link == link {
			return true
		}
	}
	return false
}

// hitID ist stabil je Link (kurz, für Oberfläche und MCP).
func hitID(link string) string {
	sum := sha1.Sum([]byte(link))
	return hex.EncodeToString(sum[:4])
}

// excludedHosts geben die Rohdateien nicht weiter (Q70): nie übernehmbar.
var excludedHosts = []string{"pixabay.com", "mixkit.co", "craftpix.net", "zapsplat.com", "sonniss.com"}

// knownLicenses sind nach Q70 übernehmbar (NC nur als markierter Lückenfüller).
var knownLicenses = []string{"CC0", "PUBLIC DOMAIN", "CC BY", "CC-BY", "OGA-BY", "OFL"}

// ClassifyLicense ordnet eine Lizenz nach Q70 ein: Status und Kennzeichen (NC, SA, ND).
func ClassifyLicense(license, link string) (string, string) {
	if u, err := url.Parse(link); err == nil {
		host := strings.TrimPrefix(strings.ToLower(u.Hostname()), "www.")
		for _, x := range excludedHosts {
			if host == x || strings.HasSuffix(host, "."+x) {
				return LicenseExcluded, ""
			}
		}
	}
	flag := LicenseFlag(license)
	if flag == "ND" {
		return LicenseExcluded, flag
	}
	up := strings.ToUpper(license)
	for _, k := range knownLicenses {
		if strings.Contains(up, k) {
			return LicenseKnown, flag
		}
	}
	return LicenseCheck, flag
}

// ResearchResult ist das Ergebnis einer Recherche zu einem Slot: alle gespeicherten Treffer, neu angelegte, Hinweise der Quellen.
type ResearchResult struct {
	Hits  []Hit    `json:"treffer"`
	Added int      `json:"neu"`
	Notes []string `json:"hinweise"`
}

// RunResearch sucht für einen Asset-Slot (Suchbegriff leer = Beschreibung des Slots) und legt die Treffer ab.
func RunResearch(ctx context.Context, root, slotID, query string, s Searcher, now time.Time) (ResearchResult, error) {
	reg, err := Load(root)
	if err != nil {
		return ResearchResult{}, err
	}
	slot := reg.Find(slotID)
	if slot == nil {
		return ResearchResult{}, fmt.Errorf("unbekannter Asset-Slot %q", slotID)
	}
	if strings.TrimSpace(query) == "" {
		query = searchTerm(*slot)
	}
	found, notes := s.Search(ctx, query, slot.Kind)
	added, err := AddHits(root, slotID, found, now)
	if err != nil {
		return ResearchResult{}, err
	}
	hits, err := HitsOf(root, slotID)
	if notes == nil {
		notes = []string{}
	}
	return ResearchResult{Hits: hits, Added: added, Notes: notes}, err
}

// searchTerm ist die Vorgabe der Suche: Beschreibung ohne Abschnitt („Gebäude: farm“ → „farm“).
func searchTerm(s Slot) string {
	d := s.Description
	if i := strings.LastIndex(d, ":"); i >= 0 {
		d = d[i+1:]
	}
	return strings.TrimSpace(d)
}
