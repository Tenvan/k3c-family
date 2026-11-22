package playtest

import (
	"fmt"
	"path/filepath"
	"regexp"
	"strings"
)

// Tickets aus angehakten Befunden (B-1224). Die Anlage läuft über die Planungslogik der Workbench (Planner, im
// Betrieb planning.Create/Sections): Pflichtfelder Domäne, Typ, Prio, Projekt und Sprint. Nur angehakte Befunde ohne
// Ticket kommen dran; ein Befund mit duplicate_of legt nichts an, das vorhandene Ticket bekommt einen Hinweis. Scheitert
// ein Befund, steht der Grund an ihm und die anderen laufen weiter.

// Planner ist der Zugang zur Planungslogik (Tests setzen Attrappen ein).
type Planner struct {
	Create   func(slug, title string, fields map[string]string) (notes string, err error)
	Get      func(id string) (string, error)
	Sections func(id string, bodies map[string]string) error
}

// TicketDefaults gelten für Befunde ohne eigene Angabe (Auswahl der Seite bzw. Aufruf des Tools).
type TicketDefaults struct {
	Domain, Type, Prio, Project, Sprint string
}

// TicketResult fasst einen Lauf zusammen.
type TicketResult struct {
	Created []string // "f1 → B-1300"
	Noted   []string // "f2 → Hinweis in B-1100"
	Failed  []string // "f3: Grund"
	Skipped int      // nicht angehakt oder schon mit Ticket
	Similar []string // Hinweise der Planung auf ähnliche Titel
}

var (
	newTicketRe = regexp.MustCompile(`(?m)^(B-\d+) angelegt`)
	slugBad     = regexp.MustCompile(`[^a-z0-9]+`)
	umlauts     = strings.NewReplacer("ä", "ae", "ö", "oe", "ü", "ue", "ß", "ss")
)

// slugOf macht aus dem Titel einen Kurznamen (a-z, 0-9, -; höchstens 40 Zeichen).
func slugOf(title string) string {
	s := slugBad.ReplaceAllString(umlauts.Replace(strings.ToLower(title)), "-")
	if len(s) > 40 {
		s = s[:40]
	}
	if s = strings.Trim(s, "-"); s == "" {
		return "befund"
	}
	return s
}

func pick(vals ...string) string {
	for _, v := range vals {
		if strings.TrimSpace(v) != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}

// titleOf: Titel des Befunds, sonst die ersten Worte des Zitats.
func titleOf(f Finding) string {
	if t := strings.TrimSpace(f.Title); t != "" {
		return t
	}
	q := strings.Join(strings.Fields(f.Quote), " ")
	if r := []rune(q); len(r) > 70 {
		q = string(r[:70]) + " …"
	}
	return "Spieltest: " + q
}

// ticketText sind die Abschnitte des neuen Tickets: Zitat, Zeitpunkt, Bild und Spielkontext.
func (s *Store) ticketText(sessionID string, f Finding) map[string]string {
	var b strings.Builder
	fmt.Fprintf(&b, "Aus dem Spieltest %s, Befund %s (Zeitpunkt %d ms Unix).\n\n> %s\n", sessionID, f.ID, f.Ts,
		strings.ReplaceAll(strings.TrimSpace(f.Quote), "\n", "\n> "))
	if f.Context != "" {
		fmt.Fprintf(&b, "\n- **Spielkontext:** %s\n", f.Context)
	}
	if f.Frame != "" {
		fmt.Fprintf(&b, "- **Bild:** `%s` (lokal, nicht eingecheckt)\n", filepath.ToSlash(filepath.Join("playtests", sessionID, f.Frame)))
	}
	return map[string]string{"Ausgangslage": b.String()}
}

// CreateTickets legt Tickets für alle angehakten Befunde ohne Ticket an und trägt die IDs in findings.json ein.
func (s *Store) CreateTickets(id string, def TicketDefaults, p Planner) (TicketResult, error) {
	var res TicketResult
	dir, err := s.sessionDir(id)
	if err != nil {
		return res, err
	}
	findingsMu.Lock()
	defer findingsMu.Unlock()
	list, err := readFindings(dir)
	if err != nil {
		return res, err
	}
	for i := range list {
		f := &list[i]
		if !f.Selected || f.Ticket != "" {
			res.Skipped++
			continue
		}
		f.Error = ""
		var msg string
		var err error
		if f.DuplicateOf != "" {
			msg, err = s.noteDuplicate(id, *f, p)
			if err == nil {
				res.Noted = append(res.Noted, f.ID+" → "+msg)
			}
		} else {
			var notes []string
			msg, notes, err = s.createOne(id, *f, def, p)
			f.Ticket = msg
			if err == nil || msg != "" {
				res.Created = append(res.Created, f.ID+" → "+msg)
				res.Similar = append(res.Similar, notes...)
			}
		}
		if err != nil {
			f.Error = err.Error()
			res.Failed = append(res.Failed, f.ID+": "+err.Error())
		}
		if werr := writeFindings(dir, list); werr != nil {
			return res, werr
		}
	}
	return res, nil
}

// createOne legt das Ticket an; die ID wird auch geliefert, wenn nur das Füllen der Abschnitte scheiterte.
func (s *Store) createOne(session string, f Finding, def TicketDefaults, p Planner) (string, []string, error) {
	fields := map[string]string{"Domäne": pick(f.Domain, def.Domain), "Typ": pick(f.Type, def.Type), "Prio": pick(f.Prio, def.Prio, "mittel"),
		"Projekt": pick(f.Project, def.Project), "Sprint": pick(f.Sprint, def.Sprint)}
	var missing []string
	for _, k := range []string{"Domäne", "Typ", "Projekt", "Sprint"} {
		if fields[k] == "" {
			missing = append(missing, k)
		}
	}
	if len(missing) > 0 {
		return "", nil, fmt.Errorf("%s fehlt (am Befund oder in der Auswahl setzen)", strings.Join(missing, ", "))
	}
	out, err := p.Create(slugOf(titleOf(f)), titleOf(f), fields)
	if err != nil {
		return "", nil, err
	}
	m := newTicketRe.FindStringSubmatch(out)
	if m == nil {
		return "", nil, fmt.Errorf("Ticket-ID nicht erkannt: %s", out)
	}
	var similar []string
	for _, l := range strings.Split(out, "\n") {
		if strings.Contains(strings.ToLower(l), "ähnlich") {
			similar = append(similar, l)
		}
	}
	if err := p.Sections(m[1], s.ticketText(session, f)); err != nil {
		return m[1], similar, fmt.Errorf("%s angelegt, Abschnitt nicht gefüllt: %w", m[1], err)
	}
	return m[1], similar, nil
}

// noteDuplicate trägt einen Hinweis in Notizen des vorhandenen Tickets ein (einmal je Befund).
func (s *Store) noteDuplicate(session string, f Finding, p Planner) (string, error) {
	md, err := p.Get(f.DuplicateOf)
	if err != nil {
		return "", fmt.Errorf("duplicate_of %s: %w", f.DuplicateOf, err)
	}
	tag := "Spieltest " + session + ", Befund " + f.ID
	body := sectionBody(md, "Notizen")
	if strings.Contains(body, tag) {
		return "Hinweis schon in " + f.DuplicateOf, nil
	}
	quote := strings.Join(strings.Fields(f.Quote), " ")
	line := "- " + tag + ": „" + quote + "“"
	if f.Context != "" {
		line += " (" + f.Context + ")"
	}
	if t := strings.TrimSpace(body); t == "" || t == "–" || t == "-" {
		body = line
	} else {
		body = strings.TrimRight(body, "\n ") + "\n" + line
	}
	if err := p.Sections(f.DuplicateOf, map[string]string{"Notizen": body}); err != nil {
		return "", fmt.Errorf("Hinweis in %s: %w", f.DuplicateOf, err)
	}
	return "Hinweis in " + f.DuplicateOf, nil
}

// sectionBody ist der Text unter `## name` bis zur nächsten `## `-Überschrift (leer, wenn es sie nicht gibt).
func sectionBody(md, name string) string {
	var out []string
	in := false
	for _, l := range strings.Split(strings.ReplaceAll(md, "\r\n", "\n"), "\n") {
		if strings.HasPrefix(l, "## ") {
			if in {
				break
			}
			in = strings.TrimSpace(l[3:]) == name
			continue
		}
		if in {
			out = append(out, l)
		}
	}
	return strings.TrimSpace(strings.Join(out, "\n"))
}
