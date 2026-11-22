package playtest

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"sync"
)

// Befunde (B-1224): Claude liest Transkript, Marker und Bilder per MCP und schlägt Befunde vor (findings.json); die
// Workbench speichert sie und zeigt sie mit Haken. Den Haken (selected), die Ticket-ID und den Fehler setzt nie der
// Vorschlag von Claude, sondern nur die Oberfläche (UpdateFinding) bzw. die Ticket-Anlage (CreateTickets).

// Finding ist ein Befund.
type Finding struct {
	ID          string `json:"id"`
	Title       string `json:"title,omitempty"`   // Titel des Tickets als Aussage
	Quote       string `json:"quote"`             // Zitat aus dem Transkript (oder Beobachtung im Bild)
	Ts          int64  `json:"ts"`                // Zeitpunkt, Unix-ms
	Frame       string `json:"frame,omitempty"`   // Bild relativ zur Session, z. B. frames/marker-2.jpg
	Context     string `json:"context,omitempty"` // Spielkontext (Welt, Insel, Stufe, Tageszeit, Spieler)
	Domain      string `json:"domain,omitempty"`
	Type        string `json:"type,omitempty"`
	Prio        string `json:"prio,omitempty"`
	Project     string `json:"project,omitempty"`
	Sprint      string `json:"sprint,omitempty"`
	DuplicateOf string `json:"duplicate_of,omitempty"` // offenes Ticket B-123, sonst leer
	Selected    bool   `json:"selected"`               // Haken von Ralf
	Ticket      string `json:"ticket,omitempty"`       // ID nach der Anlage
	Error       string `json:"error,omitempty"`        // Grund, wenn die Anlage scheiterte
}

// FindingIn ist ein Vorschlag von Claude (ohne Haken, Ticket und Fehler).
type FindingIn struct {
	ID          string `json:"id" jsonschema:"Kennung des Befunds, a-z A-Z 0-9 - _ (höchstens 32), z. B. f1"`
	Title       string `json:"title,omitempty" jsonschema:"Titel des Tickets als Aussage"`
	Quote       string `json:"quote" jsonschema:"Zitat aus dem Transkript"`
	Ts          int64  `json:"ts" jsonschema:"Zeitpunkt in Unix-ms (aus playtest_get)"`
	Frame       string `json:"frame,omitempty" jsonschema:"Bild relativ zur Session, z. B. frames/marker-2.jpg"`
	Context     string `json:"context,omitempty" jsonschema:"Spielkontext: Welt, Insel, Stufe, Tageszeit, Spieler"`
	Domain      string `json:"domain,omitempty" jsonschema:"Vorschlag Domäne des Tickets (REG, SIM, SRV, CLI, PLAT, INF, DEV, PAP)"`
	Type        string `json:"type,omitempty" jsonschema:"Vorschlag Typ: Idee, Problem, Schuld oder Frage"`
	Prio        string `json:"prio,omitempty" jsonschema:"Vorschlag Prio: hoch, mittel, niedrig oder ?"`
	Project     string `json:"project,omitempty" jsonschema:"Vorschlag Projekt-Kürzel"`
	Sprint      string `json:"sprint,omitempty" jsonschema:"Vorschlag Sprint-Kürzel"`
	DuplicateOf string `json:"duplicate_of,omitempty" jsonschema:"offenes Ticket B-123, das denselben Befund beschreibt (geprüft mit plan_list), sonst leer"`
}

// Fehler der Befunde; die Oberfläche zeigt den Text.
var (
	ErrBadFinding = errors.New("ungültiger Befund")
	findingIDRe   = regexp.MustCompile(`^[A-Za-z0-9_-]{1,32}$`)
	ticketIDRe    = regexp.MustCompile(`^B-\d+$`)
	findingsMu    sync.Mutex
)

var (
	allowedType = map[string]bool{"": true, "Idee": true, "Problem": true, "Schuld": true, "Frage": true}
	allowedPrio = map[string]bool{"": true, "hoch": true, "mittel": true, "niedrig": true, "?": true}
)

func (f FindingIn) validate() error {
	switch {
	case !findingIDRe.MatchString(f.ID):
		return fmt.Errorf("%w: id %q (a-z, A-Z, 0-9, -, _; höchstens 32 Zeichen)", ErrBadFinding, f.ID)
	case strings.TrimSpace(f.Quote) == "":
		return fmt.Errorf("%w %s: quote fehlt", ErrBadFinding, f.ID)
	case !allowedType[f.Type]:
		return fmt.Errorf("%w %s: type %q (Idee, Problem, Schuld, Frage)", ErrBadFinding, f.ID, f.Type)
	case !allowedPrio[f.Prio]:
		return fmt.Errorf("%w %s: prio %q (hoch, mittel, niedrig, ?)", ErrBadFinding, f.ID, f.Prio)
	case f.DuplicateOf != "" && !ticketIDRe.MatchString(f.DuplicateOf):
		return fmt.Errorf("%w %s: duplicate_of %q ist keine Ticket-ID (B-123)", ErrBadFinding, f.ID, f.DuplicateOf)
	case strings.ContainsAny(f.Frame, `\:`) || strings.Contains(f.Frame, ".."):
		return fmt.Errorf("%w %s: frame muss ein Pfad in der Session sein", ErrBadFinding, f.ID)
	}
	return nil
}

type findingsFile struct {
	Findings []Finding `json:"findings"`
}

// sessionDir prüft die Kennung und liefert den Ordner einer vorhandenen Session.
func (s *Store) sessionDir(id string) (string, error) {
	ses, err := s.Get(id)
	if err != nil {
		return "", err
	}
	return filepath.Join(s.Root, ses.ID), nil
}

// Findings liest findings.json; fehlt die Datei, ist die Liste leer.
func (s *Store) Findings(id string) ([]Finding, error) {
	dir, err := s.sessionDir(id)
	if err != nil {
		return nil, err
	}
	return readFindings(dir)
}

func readFindings(dir string) ([]Finding, error) {
	raw, err := os.ReadFile(filepath.Join(dir, fileFindings))
	if errors.Is(err, os.ErrNotExist) {
		return []Finding{}, nil
	}
	if err != nil {
		return nil, err
	}
	var f findingsFile
	if err := json.Unmarshal(raw, &f); err != nil {
		return nil, fmt.Errorf("findings.json nicht lesbar: %w", err)
	}
	if f.Findings == nil {
		f.Findings = []Finding{}
	}
	return f.Findings, nil
}

func writeFindings(dir string, list []Finding) error {
	raw, err := json.MarshalIndent(findingsFile{Findings: list}, "", "  ")
	if err != nil {
		return err
	}
	tmp := filepath.Join(dir, fileFindings+".tmp")
	if err := os.WriteFile(tmp, raw, 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, filepath.Join(dir, fileFindings))
}

// SetFindings schreibt oder aktualisiert Befunde nach id. Haken, Ticket und Fehler eines vorhandenen Befunds bleiben
// erhalten; ein neuer Befund beginnt ohne Haken. Alle Vorschläge werden erst geprüft, bei einem Fehler wird nichts
// geschrieben. Ergebnis: neue und aktualisierte Befunde.
func (s *Store) SetFindings(id string, in []FindingIn) (added, updated int, err error) {
	dir, err := s.sessionDir(id)
	if err != nil {
		return 0, 0, err
	}
	if len(in) == 0 {
		return 0, 0, fmt.Errorf("%w: keine Befunde angegeben", ErrBadFinding)
	}
	seen := map[string]bool{}
	for _, f := range in {
		if err := f.validate(); err != nil {
			return 0, 0, err
		}
		if seen[f.ID] {
			return 0, 0, fmt.Errorf("%w: id %s doppelt", ErrBadFinding, f.ID)
		}
		seen[f.ID] = true
	}
	findingsMu.Lock()
	defer findingsMu.Unlock()
	list, err := readFindings(dir)
	if err != nil {
		return 0, 0, err
	}
	for _, f := range in {
		n := Finding{ID: f.ID, Title: f.Title, Quote: f.Quote, Ts: f.Ts, Frame: f.Frame, Context: f.Context, Domain: f.Domain,
			Type: f.Type, Prio: f.Prio, Project: f.Project, Sprint: f.Sprint, DuplicateOf: f.DuplicateOf}
		at := -1
		for i := range list {
			if list[i].ID == f.ID {
				at = i
			}
		}
		if at < 0 {
			list = append(list, n)
			added++
			continue
		}
		n.Selected, n.Ticket, n.Error = list[at].Selected, list[at].Ticket, list[at].Error
		list[at] = n
		updated++
	}
	return added, updated, writeFindings(dir, list)
}

// FindingPatch ändert, was Ralf auf der Seite ändert: Haken und Vorschlag für Domäne, Typ, Prio, Projekt, Sprint.
type FindingPatch struct {
	Selected *bool   `json:"selected,omitempty"`
	Domain   *string `json:"domain,omitempty"`
	Type     *string `json:"type,omitempty"`
	Prio     *string `json:"prio,omitempty"`
	Project  *string `json:"project,omitempty"`
	Sprint   *string `json:"sprint,omitempty"`
}

// UpdateFinding wendet die Änderung von der Seite auf einen Befund an; ein Befund mit Ticket bleibt unverändert.
func (s *Store) UpdateFinding(id, fid string, p FindingPatch) (Finding, error) {
	dir, err := s.sessionDir(id)
	if err != nil {
		return Finding{}, err
	}
	findingsMu.Lock()
	defer findingsMu.Unlock()
	list, err := readFindings(dir)
	if err != nil {
		return Finding{}, err
	}
	for i := range list {
		f := &list[i]
		if f.ID != fid {
			continue
		}
		if f.Ticket != "" {
			return *f, fmt.Errorf("%w %s: Ticket %s ist schon angelegt", ErrBadFinding, fid, f.Ticket)
		}
		if p.Selected != nil {
			f.Selected = *p.Selected
		}
		for dst, src := range map[*string]*string{&f.Domain: p.Domain, &f.Type: p.Type, &f.Prio: p.Prio, &f.Project: p.Project, &f.Sprint: p.Sprint} {
			if src != nil {
				*dst = strings.TrimSpace(*src)
			}
		}
		if !allowedType[f.Type] || !allowedPrio[f.Prio] {
			return *f, fmt.Errorf("%w %s: Typ oder Prio ungültig", ErrBadFinding, fid)
		}
		return *f, writeFindings(dir, list)
	}
	return Finding{}, fmt.Errorf("%w: Befund %s gibt es nicht", ErrNotFound, fid)
}
