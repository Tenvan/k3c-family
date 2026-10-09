package planning

import (
	"fmt"
	"os"
	"path"
	"strconv"
	"strings"
	"time"
)

// NewDoc beschreibt ein neues Ticket, einen Sprint, eine Session (B-210) oder ein Projekt (B-357). ID bei Sprint (M9),
// Session (M9.1) und Projekt (GRA); ein Ticket bekommt die nächste freie Nummer.
type NewDoc struct {
	Kind   string            `json:"kind"`
	ID     string            `json:"id,omitempty"`
	Slug   string            `json:"slug"`
	Title  string            `json:"title"`
	Fields map[string]string `json:"fields,omitempty"`
}

// Create legt das Dokument als Kopie der Vorlage an und trägt es in Index, Fahrplan bzw. Session-Tabelle ein.
func Create(root string, in NewDoc) (string, error) {
	writeMu.Lock()
	defer writeMu.Unlock()
	if !reSlug.MatchString(in.Slug) {
		return "", fmt.Errorf("ungültiger Kurzname %q: nur a-z, 0-9 und `-`", in.Slug)
	}
	if err := plainValue("Titel", in.Title); err != nil {
		return "", err
	}
	mk, ok := map[string]func(*changeSet, NewDoc, string) error{
		"ticket": createTicket, "sprint": createSprint, "session": createSession, "projekt": createProject}[in.Kind]
	if !ok {
		return "", fmt.Errorf("unbekannte Art %q: ticket, sprint, session oder projekt", in.Kind)
	}
	c := newChangeSet(root)
	tpl, err := c.read("vorlagen/" + in.Kind + ".md")
	if err != nil {
		return "", err
	}
	if err := mk(c, in, tpl); err != nil {
		return "", err
	}
	return strings.Join(c.notes, "\n"), c.apply()
}

// fromTemplate ersetzt Überschrift und Felder (Vorgaben, dann Eingabe) einer Vorlage.
func fromTemplate(c *changeSet, kind, tpl, header string, defaults, values map[string]string) (string, error) {
	lines := splitLines(tpl)
	lines[findRow(lines, "# ")] = "# " + header
	merged := map[string]string{}
	for k, v := range defaults {
		merged[k] = v
	}
	for k, v := range values {
		merged[k] = v
	}
	return setFields(c.root, kind, joinLines(lines), merged)
}

func createTicket(c *changeSet, in NewDoc, tpl string) error {
	nr, err := nextTicket(c.root)
	if err != nil {
		return err
	}
	text, err := fromTemplate(c, "ticket", tpl, nr+" · "+in.Title, map[string]string{"Status": "offen", "Umgebung": "?", "Sprint": "–", "Projekt": "–",
		"Erstellt": time.Now().Format("2006-01-02"), "Spec": "Entwurf", "Revision": "1", "Freigabe": "–"}, in.Fields)
	if err != nil {
		return err
	}
	t := ParseTicket(text)
	if t.Status != "offen" && t.Status != "eingeplant" {
		return fmt.Errorf("ein neues Ticket ist offen oder eingeplant")
	}
	if err := checkProjectRef(c, t.Project); err != nil {
		return err
	}
	rel := "backlog/" + nr + "-" + in.Slug + ".md"
	c.write(rel, text)
	c.crlf[rel] = c.crlf["vorlagen/ticket.md"]
	c.notes = append(c.notes, nr+" angelegt: docs/"+rel+" (Index ergänzt)")
	return syncIndex(c, t, rel)
}

// nextTicket ist die höchste Nummer in backlog/ und backlog/archiv/ plus eins.
func nextTicket(root string) (string, error) {
	maxNr := 0
	for _, dir := range []string{"backlog", "backlog/archiv"} {
		entries, err := os.ReadDir(docsPath(root, dir))
		if err != nil && dir == "backlog" {
			return "", err
		}
		for _, e := range entries {
			if ticket.MatchString(e.Name()) {
				n, _ := strconv.Atoi(e.Name()[2:5])
				maxNr = max(maxNr, n)
			}
		}
	}
	return fmt.Sprintf("B-%03d", maxNr+1), nil
}

func createSprint(c *changeSet, in NewDoc, tpl string) error {
	if !reSprintID.MatchString(in.ID) {
		return fmt.Errorf("Sprint-ID %q: Großbuchstaben und Zahl, z. B. M9", in.ID)
	}
	if _, ok := findSprint(c.root, in.ID); ok {
		return fmt.Errorf("Sprint %s gibt es schon", in.ID)
	}
	dom := in.Fields["Domäne"]
	text, err := fromTemplate(c, "sprint", tpl, in.ID+" · "+dom+" · "+in.Title, map[string]string{"Status": "geplant",
		"Projekt": "–", "Reife": "Entwurf", "Tickets": "–", "Spec": "Entwurf", "Revision": "1", "Freigabe": "–"}, in.Fields)
	if err != nil {
		return err
	}
	if headFields(text)["Status"] != "geplant" {
		return fmt.Errorf("ein neuer Sprint ist geplant")
	}
	lines := splitLines(text)
	if t, ok := findTable(lines, heading("## Sessions")); ok { // Beispielzeilen der Vorlage raus
		lines = append(lines[:t.first], lines[t.end:]...)
	}
	text = joinLines(lines)
	dir := "sprints/geplant/" + in.ID + "-" + in.Slug
	c.write(dir+"/README.md", text)
	c.crlf[dir+"/README.md"] = c.crlf["vorlagen/sprint.md"]
	c.notes = append(c.notes, in.ID+" angelegt: docs/"+dir+"/ (Fahrplan ergänzt)")
	sp := ParseSprint(text, path.Base(dir), "geplant")
	if err := checkProjectRef(c, sp.Project); err != nil {
		return err
	}
	if err := syncProjectSprint(c, sp); err != nil {
		return err
	}
	return syncRoadmap(c, sp, headFields(text), "", dir)
}

func createSession(c *changeSet, in NewDoc, tpl string) error {
	m := reSession.FindStringSubmatch(in.ID)
	if m == nil {
		return fmt.Errorf("Session-ID %q: Sprint-ID, Punkt, Nummer, z. B. M9.1", in.ID)
	}
	sp, ok := findSprint(c.root, m[1])
	if !ok || sp.state == "erledigt" {
		return fmt.Errorf("Sprint %s fehlt oder ist erledigt", m[1])
	}
	if findEntry(docsPath(c.root, sp.dir), in.ID+"-", false) != "" {
		return fmt.Errorf("Session %s gibt es schon", in.ID)
	}
	text, err := fromTemplate(c, "session", tpl, in.ID+" · "+in.Title, map[string]string{"Status": "offen",
		"Typ": "Umsetzung", "Agent": "autonom", "Umgebung": "?", "Branch": strings.ToLower(m[1]) + "/" + m[2] + "-" + in.Slug,
		"Abhängig von": "–", "Tickets": "–", "Kriterien": "–"}, in.Fields)
	if err != nil {
		return err
	}
	file := in.ID + "-" + in.Slug + ".md"
	rel := sp.dir + "/" + file
	c.write(rel, text)
	c.crlf[rel] = c.crlf["vorlagen/session.md"]
	c.notes = append(c.notes, in.ID+" angelegt: docs/"+rel+" (Session-Tabelle ergänzt)")
	f := headFields(text)
	if err := syncSessionRow(c, sp.dir+"/README.md", Session{Nr: in.ID, File: file, Typ: f["Typ"], Agent: f["Agent"], Status: f["Status"]}); err != nil {
		return err
	}
	return syncSprintDomain(c, sp.dir, sp.state)
}

// Delete löscht einen Sprint-Entwurf in geplant/, eine Session darin oder ein Projekt ohne Sprints und Tickets;
// Tickets werden verworfen, nie gelöscht.
func Delete(root, id string) (string, error) {
	writeMu.Lock()
	defer writeMu.Unlock()
	r, err := resolve(root, id)
	if err != nil {
		return "", err
	}
	if r.kind == "ticket" {
		return "", fmt.Errorf("ein Ticket wird nicht gelöscht: plan_set mit Status verworfen")
	}
	c := newChangeSet(root)
	if r.kind == "projekt" {
		if err := deleteProject(c, r); err != nil {
			return "", err
		}
		return r.id + " gelöscht: docs/" + r.rel, c.apply()
	}
	readme, err := c.read(r.dir + "/README.md")
	if err != nil {
		return "", err
	}
	if r.state != "geplant" || headFields(readme)["Spec"] != "Entwurf" {
		return "", fmt.Errorf("löschen nur in geplanten Sprints mit Spec Entwurf; sonst Status ändern")
	}
	if r.kind == "sprint" {
		text, err := c.read("sprints/README.md")
		if err != nil {
			return "", err
		}
		lines, _, _ := takeRoadmapRow(splitLines(text), r.id, r.dir)
		c.write("sprints/README.md", joinLines(lines))
		c.remove = append(c.remove, r.dir)
		if err := syncProjectSprint(c, Sprint{ID: r.id, Project: "–"}); err != nil {
			return "", err
		}
	} else {
		c.write(r.dir+"/README.md", joinLines(removeRow(splitLines(readme), "| "+r.id+" |")))
		c.remove = append(c.remove, r.rel)
		if err := syncSprintDomain(c, r.dir, r.state); err != nil {
			return "", err
		}
	}
	return r.id + " gelöscht: docs/" + r.rel, c.apply()
}

// Get liefert ein Dokument als Markdown mit Pfad in der ersten Zeile.
func Get(root, id string) (string, error) {
	r, err := resolve(root, id)
	if err != nil {
		return "", err
	}
	text, _, err := readText(root, r.rel)
	return "docs/" + r.rel + "\n\n" + text, err
}
