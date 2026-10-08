package planning

import (
	"fmt"
	"os"
	"path"
	"regexp"
	"slices"
	"sort"
	"strconv"
	"strings"
)

// Projekte (B-357): docs/projekte/<KÜRZEL>-slug.md nach vorlagen/projekt.md, Übersicht docs/projekte/README.md.
// Nur aktive Projekte haben einen Rang, lückenlos ab 1; ABN (Abnahmen am Gerät) läuft ohne Rang neben der Rangfolge.

// Project ist der Kopf einer Projekt-Datei und die Sprints ihrer Tabelle in Reihenfolge.
type Project struct {
	ID      string   `json:"id"`
	Title   string   `json:"title"`
	Status  string   `json:"status"` // aktiv | ruht | erledigt
	Rang    string   `json:"rang"`   // Zahl ab 1 oder –
	Sprints []string `json:"sprints"`
	File    string   `json:"file"` // Dateiname in docs/projekte
}

// unranked ist das Projekt der Abnahmen am Gerät: aktiv, aber ohne Rang (arbeitsweise.md › Projekte und Rang).
const unranked = "ABN"

var reProjectID = regexp.MustCompile(`^[A-Z]{3}$`)

// ParseProject liest eine Projekt-Datei; file ist ihr Dateiname.
func ParseProject(text, file string) Project {
	lines := splitLines(strings.ReplaceAll(text, "\r\n", "\n"))
	f := fields(lines[:headEnd(lines)])
	p := Project{ID: strings.SplitN(file, "-", 2)[0], Title: file, Status: f["Status"], Rang: f["Rang"], Sprints: projectSprints(lines), File: file}
	if h := header(lines); len(h) >= 2 {
		p.ID, p.Title = h[0], strings.Join(h[1:], " · ")
	}
	return p
}

// projectSprints sind die Sprint-IDs der Tabelle unter `## Sprints`.
func projectSprints(lines []string) []string {
	out := []string{}
	if t, ok := findTable(lines, heading("## Sprints")); ok {
		for _, l := range lines[t.first:t.end] {
			if c := cells(l); c[0] != "" {
				out = append(out, c[0])
			}
		}
	}
	return out
}

// loadProjects liest alle Projekte unter root/docs/projekte; ohne Ordner eine leere Liste.
func loadProjects(root string) []Project {
	out := []Project{}
	entries, _ := os.ReadDir(docsPath(root, "projekte"))
	for _, e := range entries {
		if isProjectFile(e.Name()) && !e.IsDir() {
			if text, _, err := readText(root, "projekte/"+e.Name()); err == nil {
				out = append(out, ParseProject(text, e.Name()))
			}
		}
	}
	sortProjects(out)
	return out
}

func isProjectFile(name string) bool {
	return len(name) > 4 && reProjectID.MatchString(name[:3]) && name[3] == '-' && strings.HasSuffix(name, ".md")
}

// sortProjects: aktive nach Rang (ABN danach), dann ruhende, dann erledigte; sonst nach Kürzel.
func sortProjects(ps []Project) {
	order := map[string]int{"aktiv": 0, "ruht": 1, "erledigt": 2}
	key := func(p Project) int {
		if n, err := strconv.Atoi(p.Rang); err == nil && p.Status == "aktiv" {
			return n
		}
		return 1 << 20
	}
	sort.SliceStable(ps, func(i, j int) bool {
		a, b := ps[i], ps[j]
		if order[a.Status] != order[b.Status] {
			return order[a.Status] < order[b.Status]
		}
		if key(a) != key(b) {
			return key(a) < key(b)
		}
		return a.ID < b.ID
	})
}

// projectsIn liest die Projekte mit den schon geplanten Änderungen des changeSets.
func projectsIn(c *changeSet) ([]Project, error) {
	names := map[string]bool{}
	entries, _ := os.ReadDir(docsPath(c.root, "projekte"))
	for _, e := range entries {
		names[e.Name()] = !e.IsDir()
	}
	for rel := range c.writes {
		if dir, name := path.Split(rel); dir == "projekte/" {
			names[name] = true
		}
	}
	for _, rel := range c.remove {
		delete(names, path.Base(rel))
	}
	out := []Project{}
	for name, file := range names {
		if !file || !isProjectFile(name) {
			continue
		}
		text, err := c.read("projekte/" + name)
		if err != nil {
			return nil, err
		}
		out = append(out, ParseProject(text, name))
	}
	sortProjects(out)
	return out, nil
}

func findProject(ps []Project, id string) (Project, bool) {
	i := slices.IndexFunc(ps, func(p Project) bool { return p.ID == id })
	if i < 0 {
		return Project{}, false
	}
	return ps[i], true
}

// createProject legt docs/projekte/<ID>-<slug>.md an; ein aktives Projekt bekommt Rang = letzter + 1.
func createProject(c *changeSet, in NewDoc, tpl string) error {
	if !reProjectID.MatchString(in.ID) {
		return fmt.Errorf("Projekt-Kürzel %q: drei Großbuchstaben, z. B. GRA", in.ID)
	}
	ps, err := projectsIn(c)
	if err != nil {
		return err
	}
	if _, ok := findProject(ps, in.ID); ok {
		return fmt.Errorf("das Projekt %s gibt es schon", in.ID)
	}
	if _, ok := in.Fields["Rang"]; ok {
		return fmt.Errorf("den Rang setzt plan_set nach dem Anlegen; neu: letzter + 1")
	}
	text, err := fromTemplate(c, "projekt", tpl, in.ID+" · "+in.Title, map[string]string{"Status": "aktiv",
		"Rang": "–", "Ziel-Tickets": "–"}, in.Fields)
	if err != nil {
		return err
	}
	lines := splitLines(text)
	if t, ok := findTable(lines, heading("## Sprints")); ok { // Beispielzeile der Vorlage raus
		lines = append(lines[:t.first], lines[t.end:]...)
	}
	rel := "projekte/" + in.ID + "-" + in.Slug + ".md"
	c.write(rel, joinLines(lines))
	c.crlf[rel] = c.crlf["vorlagen/projekt.md"]
	c.notes = append(c.notes, in.ID+" angelegt: docs/"+rel+" (Übersicht ergänzt)")
	return rerank(c, in.ID, 0)
}

// rerank vergibt die Ränge der aktiven Projekte lückenlos neu; id rückt auf Platz pos (1 … n), pos 0 lässt ein
// schon aktives Projekt an seinem Platz und hängt ein neu aktives hinten an. Danach folgt die Übersicht.
func rerank(c *changeSet, id string, pos int) error {
	ps, err := projectsIn(c)
	if err != nil {
		return err
	}
	ranked, err := rankOrder(ps, id, pos)
	if err != nil {
		return err
	}
	want := map[string]string{}
	for i, p := range ranked {
		want[p.ID] = strconv.Itoa(i + 1)
	}
	for _, p := range ps {
		rang := firstOf(want[p.ID], "–")
		if p.Rang == rang {
			continue
		}
		text, err := c.read("projekte/" + p.File)
		if err != nil {
			return err
		}
		c.write("projekte/"+p.File, setField(text, "Rang", rang))
	}
	return syncOverview(c)
}

// rankOrder ist die neue Reihenfolge der aktiven Projekte mit Rang (ohne ABN); ps ist nach Rang sortiert, neu aktive
// (Rang –) stehen hinten.
func rankOrder(ps []Project, id string, pos int) ([]Project, error) {
	var ranked []Project
	for _, p := range ps {
		if p.Status == "aktiv" && p.ID != unranked && p.ID != id {
			ranked = append(ranked, p)
		}
	}
	p, ok := findProject(ps, id)
	if !ok || p.Status != "aktiv" || p.ID == unranked {
		return ranked, nil
	}
	at := len(ranked)
	if old, err := strconv.Atoi(p.Rang); err == nil && old-1 < at {
		at = old - 1
	}
	if pos > len(ranked)+1 {
		return nil, fmt.Errorf("der Rang %d liegt außerhalb 1 … %d", pos, len(ranked)+1)
	}
	if pos > 0 {
		at = pos - 1
	}
	return slices.Insert(ranked, at, p), nil
}

// setProject ändert ein Projekt: Status und Ziel-Tickets wie die Vorlage, Rang über rerank, Sprints als neue
// Reihenfolge der Sprint-Tabelle.
func setProject(c *changeSet, r ref, values map[string]string) error {
	text, err := c.read(r.rel)
	if err != nil {
		return err
	}
	pos, err := rankValue(r.id, values)
	if err != nil {
		return err
	}
	rest := map[string]string{}
	for k, v := range values {
		if k != "Rang" && k != "Sprints" {
			rest[k] = v
		}
	}
	if text, err = setFields(c.root, "projekt", text, rest); err != nil {
		return err
	}
	if order, ok := values["Sprints"]; ok {
		if text, err = reorderSprints(text, r.id, order); err != nil {
			return err
		}
	}
	if pos > 0 && headFields(text)["Status"] != "aktiv" {
		return fmt.Errorf("nur aktive Projekte haben einen Rang")
	}
	c.write(r.rel, text)
	c.notes = append(c.notes, r.id+" geändert: docs/"+r.rel+" (Ränge und Übersicht nachgezogen)")
	return rerank(c, r.id, pos)
}

// rankValue prüft den gewünschten Rang: eine Zahl ab 1, nie für ABN; 0 = nicht gesetzt.
func rankValue(id string, values map[string]string) (int, error) {
	v, ok := values["Rang"]
	if !ok {
		return 0, nil
	}
	if id == unranked {
		return 0, fmt.Errorf("%s läuft ohne Rang neben der Rangfolge", unranked)
	}
	n, err := strconv.Atoi(v)
	if err != nil || n < 1 {
		return 0, fmt.Errorf("der Rang %q ist keine Zahl ab 1", v)
	}
	return n, nil
}

// deleteProject löscht ein Projekt ohne Sprints und ohne Sprints oder Tickets, die es nennen.
func deleteProject(c *changeSet, r ref) error {
	text, err := c.read(r.rel)
	if err != nil {
		return err
	}
	if p := ParseProject(text, path.Base(r.rel)); len(p.Sprints) > 0 {
		return fmt.Errorf("%s hat Sprints (%s); erst zuordnen", r.id, strings.Join(p.Sprints, ", "))
	}
	d, err := Load(c.root)
	if err != nil {
		return err
	}
	for _, s := range d.Sprints {
		if s.Project == r.id {
			return fmt.Errorf("Sprint %s nennt %s", s.ID, r.id)
		}
	}
	for _, t := range append(d.Tickets, archived(c.root)...) {
		if t.Project == r.id {
			return fmt.Errorf("Ticket %s nennt %s", t.Nr, r.id)
		}
	}
	c.remove = append(c.remove, r.rel)
	return rerank(c, "", 0)
}

// checkProjectRef: Wert des Felds `Projekt` ist `–` oder ein bestehendes Kürzel.
func checkProjectRef(c *changeSet, v string) error {
	if v == "–" {
		return nil
	}
	ps, err := projectsIn(c)
	if err != nil {
		return err
	}
	if _, ok := findProject(ps, v); !ok {
		return fmt.Errorf("unbekanntes Projekt %q", v)
	}
	return nil
}
