package planning

import (
	"fmt"
	"regexp"
	"slices"
	"strings"
)

// Nächste Session (B-518): plan_next wählt aus den lokalen Dateien wie docs/arbeitsweise.md › Autonomer Ablauf,
// Schritt 1: aktive Projekte nach Rang (ABN zuletzt) → ihre aktiven Sprints in der Reihenfolge der Projekt-Tabelle →
// erste Session mit Status offen, Agent autonom, erledigten Abhängigkeiten und nicht gesperrter Domäne. Der Stand
// der Sprint-Branches kommt aus den lokalen Refs origin/sprint/<präfix> (next_origin.go, B-519).

// NextFilter ist die Eingabe von plan_next; leer = alle Domänen.
type NextFilter struct {
	Domain string `json:"domain,omitempty" jsonschema:"nur Sessions dieser Domäne: REG, SIM, SRV, CLI, PLAT, INF, DEV, PAP"`
}

var (
	reTicketRef   = regexp.MustCompile(`\bB-\d+\b`)
	reCriterion   = regexp.MustCompile(`^- \*\*(AC-\d+)\*\*`)
	reCriterionID = regexp.MustCompile(`\bAC-\d+\b`)
)

// maxReasons begrenzt die Gründe je Projekt, wenn keine Session frei ist.
const maxReasons = 3

// candidate ist eine offene Session eines aktiven Sprints; why ist leer, wenn sie frei ist.
type candidate struct {
	project Project
	sprint  Sprint
	session Session
	scope   scoped
	why     string
}

// projectNext ist ein aktives Projekt mit seinen aktiven Sprints und deren offenen Sessions in Reihenfolge.
type projectNext struct {
	project Project
	active  []string
	cands   []candidate
}

// nextState hält, was die Prüfung je Session braucht.
type nextState struct {
	root    string
	status  map[string]string // Session-ID → Status aus der Session-Tabelle, alle Sprints
	tickets map[string]string // Ticket-Nr → Status (Backlog ohne Archiv)
	running []scoped            // Sessions in Arbeit aktiver Sprints mit Dateimenge und Sperren (next_lanes.go)
	readmes map[string]string   // Sprint-ID → README von origin/sprint/<präfix> (B-519), sonst lokal
	specs   map[string]PlanSpec // Sprint-ID → Planungsangaben der README (Sperre, Bereich)
}

// Next liefert die nächste Session als verdichteten Text: Kopf, Session-Datei, ihre Kriterien aus der Sprint-README
// und parallel mögliche Sessions (andere Domäne oder andere Lane, höchstens drei); ohne freie Session den Grund je Projekt.
func Next(root string, f NextFilter) (string, error) {
	want := strings.ToUpper(strings.TrimSpace(f.Domain))
	if want != "" && !slices.Contains(domains, want) {
		return "", fmt.Errorf("unbekannte Domäne %q (%s)", f.Domain, strings.Join(domains, ", "))
	}
	d, err := Load(root)
	if err != nil {
		return "", err
	}
	notes, readmes := originOverlay(root, &d)
	st := newNextState(root, d, readmes)
	plan := st.plan(d)
	var cands []candidate
	for _, pn := range plan {
		cands = append(cands, pn.cands...)
	}
	for _, c := range cands {
		if c.why == "" && (want == "" || c.session.Domain == want) {
			out, err := st.pickText(c, parallel(cands, c, st.agents()))
			return withNotes(out, notes), err
		}
	}
	return withNotes(noneText(plan, want), notes), nil
}

// agents zählt die Sessions in Arbeit, die einen Agenten belegen; eine Mensch-Session (Abnahme am Gerät) sperrt mit
// Dateien und Sperren, belegt aber keinen der vier Plätze (B-839).
func (st nextState) agents() int {
	n := 0
	for _, r := range st.running {
		if r.session.Agent == "autonom" {
			n++
		}
	}
	return n
}

// withNotes hängt die Herkunft des Stands je aktivem Sprint (origin oder lokal) an die Antwort.
func withNotes(text string, notes []string) string {
	if len(notes) == 0 {
		return text
	}
	return text + "\n\n── Stand der Sprints ──\n" + strings.Join(notes, "\n")
}

func newNextState(root string, d Data, readmes map[string]string) nextState {
	st := nextState{root: root, status: map[string]string{}, tickets: map[string]string{}, readmes: readmes,
		specs: map[string]PlanSpec{}}
	for _, s := range d.Sprints {
		if s.Status == "aktiv" {
			st.specs[s.ID] = parsePlanSpec(st.readme(s.ID))
		}
		for _, x := range s.Sessions {
			st.status[x.Nr] = x.Status
			if s.Status == "aktiv" && x.Status == "in Arbeit" { // auch ohne Domäne: Dateien und Sperren zählen (B-839)
				st.running = append(st.running, st.scope(s, x))
			}
		}
	}
	for _, t := range d.Tickets {
		st.tickets[t.Nr] = t.Status
	}
	return st
}

// plan liefert je aktivem Projekt (Reihenfolge aus loadProjects) die aktiven Sprints seiner Tabelle und deren offene
// Sessions in Tabellen-Reihenfolge.
func (st nextState) plan(d Data) []projectNext {
	active := map[string]Sprint{}
	for _, s := range d.Sprints {
		if s.Status == "aktiv" {
			active[s.ID] = s
		}
	}
	var out []projectNext
	for _, p := range d.Projects {
		if p.Status != "aktiv" {
			continue
		}
		pn := projectNext{project: p}
		for _, id := range p.Sprints {
			if sp, ok := active[id]; ok {
				pn.active = append(pn.active, id)
				pn.cands = append(pn.cands, st.open(p, sp)...)
			}
		}
		out = append(out, pn)
	}
	return out
}

// open sind die Sessions mit Status offen eines Sprints, jede mit dem Grund, warum sie nicht dran ist.
func (st nextState) open(p Project, sp Sprint) []candidate {
	var out []candidate
	for _, x := range sp.Sessions {
		if x.Status == "offen" {
			c := candidate{project: p, sprint: sp, session: x, scope: st.scope(sp, x)}
			c.why = st.blocked(c.scope)
			out = append(out, c)
		}
	}
	return out
}

// scope: Dateimenge aus „Erlaubte Dateien“ (sonst Bereich der Sprint-README) und Sperren des Sprints.
func (st nextState) scope(sp Sprint, x Session) scoped {
	spec := st.specs[sp.ID]
	area := allowedFiles(x.Text)
	if len(area) == 0 {
		area = spec.Area
	}
	return scoped{session: x, sprint: sp.ID, area: area, locks: spec.Locks}
}

// readme ist die Sprint-README von origin (B-519), sonst die lokale; leer, wenn keine lesbar ist.
func (st nextState) readme(id string) string {
	if text, ok := st.readmes[id]; ok {
		return text
	}
	if r, ok := findSprint(st.root, id); ok {
		if text, _, err := readText(st.root, r.rel); err == nil {
			return text
		}
	}
	return ""
}

// blocked nennt, warum eine offene Session nicht dran ist; leer = frei.
func (st nextState) blocked(s scoped) string {
	x := s.session
	if x.Agent != "autonom" {
		return "Agent " + firstOf(x.Agent, "?")
	}
	if !isSessionFile(x.File) {
		return "ohne Datei"
	}
	if open := st.openDeps(x); len(open) > 0 {
		return "wartet auf " + strings.Join(open, ", ")
	}
	return busy(st.running, s)
}

// openDeps sind die unerledigten Einträge aus „Abhängig von“: Sessions nicht fertig oder verworfen, Tickets weder
// erledigt noch verworfen noch im Archiv. Unbekannte IDs gelten als offen (Status „fehlt“).
func (st nextState) openDeps(x Session) []string {
	var open []string
	for _, id := range x.Deps {
		if s := firstOf(st.status[id], "fehlt"); s != "fertig" && s != "verworfen" {
			open = append(open, id+" ("+s+")")
		}
	}
	for _, nr := range reTicketRef.FindAllString(sessionHead(x)["Abhängig von"], -1) {
		if s := st.ticketStatus(nr); s != "erledigt" && s != "verworfen" {
			open = append(open, nr+" ("+s+")")
		}
	}
	return open
}

// ticketStatus: Status aus dem Backlog; im Archiv liegen nur erledigte und verworfene Tickets.
func (st nextState) ticketStatus(nr string) string {
	if s, ok := st.tickets[nr]; ok {
		return s
	}
	if findEntry(docsPath(st.root, "backlog/archiv"), nr+"-", false) != "" {
		return "erledigt"
	}
	return "fehlt"
}

// sessionHead sind die Kopf-Felder einer Session-Datei.
func sessionHead(x Session) map[string]string {
	return headFields(strings.ReplaceAll(x.Text, "\r\n", "\n"))
}

// pickText: Kopf, Session-Datei, Kriterien aus der Sprint-README und die parallel möglichen Sessions.
func (st nextState) pickText(c candidate, more []candidate) (string, error) {
	r, ok := findSprint(st.root, c.sprint.ID)
	if !ok {
		return "", fmt.Errorf("sprint %s nicht gefunden", c.sprint.ID)
	}
	readme, ok := st.readmes[c.sprint.ID]
	if !ok {
		var err error
		if readme, _, err = readText(st.root, r.rel); err != nil {
			return "", err
		}
	}
	x, head := c.session, sessionHead(c.session)
	crit := firstOf(head["Kriterien"], "–")
	out := []string{
		"Nächste Session: " + x.Nr + " · " + x.Titel,
		"Pfad: docs/" + r.dir + "/" + x.File,
		fmt.Sprintf("Sprint: %s · %s (Projekt %s, Rang %s)", c.sprint.ID, c.sprint.Title, c.project.ID, c.project.Rang),
		domainLine(x),
		"Branch: sprint/" + sprintPrefix(c.sprint), // wie originOverlay (B-839)
		"Kriterien: " + crit,
		"Hinweis: Stand je Sprint (origin oder lokal) siehe unten; Branches, die lokal noch nicht bekannt sind, prüft der " +
			"Agent mit git fetch --prune.",
		"",
		"── Session-Datei ──",
		strings.TrimRight(strings.ReplaceAll(x.Text, "\r\n", "\n"), "\n"),
		"",
		"── Kriterien aus der Sprint-README ──",
	}
	if lines := criteria(readme, crit); len(lines) > 0 {
		out = append(out, lines...)
	} else {
		out = append(out, "keine")
	}
	out = append(out, "", "── Parallel möglich ──")
	return strings.Join(append(out, parallelLines(more)...), "\n"), nil
}

// criteria liefert aus `## Akzeptanzkriterien` die Einträge, deren ID want nennt; „alle“ liefert alle. Eingerückte
// Folgezeilen gehören zum Eintrag davor.
func criteria(readme, want string) []string {
	ids := reCriterionID.FindAllString(want, -1)
	all := strings.Contains(strings.ToLower(want), "alle")
	lines := splitLines(readme)
	start := slices.Index(lines, "## Akzeptanzkriterien")
	if start < 0 {
		return nil
	}
	var out []string
	keep := false
	for _, l := range lines[start+1:] {
		if strings.HasPrefix(l, "## ") {
			break
		}
		if m := reCriterion.FindStringSubmatch(l); m != nil {
			keep = all || slices.Contains(ids, m[1])
		} else if !strings.HasPrefix(l, " ") {
			keep = false
		}
		if keep {
			out = append(out, l)
		}
	}
	return out
}

// noneText: keine Session frei, je aktivem Projekt kurz der Grund.
func noneText(plan []projectNext, want string) string {
	title := "Keine Session frei"
	if want != "" {
		title += " in Domäne " + want
	}
	out := []string{title + ". Gründe je Projekt:"}
	if len(plan) == 0 {
		out = append(out, "- kein aktives Projekt")
	}
	for _, pn := range plan {
		out = append(out, "- "+pn.project.ID+": "+pn.reason())
	}
	return strings.Join(out, "\n")
}

// reason fasst zusammen, warum im Projekt keine Session dran ist; eine freie Session fiel nur dem Domänen-Filter zum Opfer.
func (pn projectNext) reason() string {
	if len(pn.active) == 0 {
		return "kein aktiver Sprint"
	}
	if len(pn.cands) == 0 {
		return "keine offene Session (" + strings.Join(pn.active, ", ") + ")"
	}
	var why []string
	for _, c := range pn.cands {
		why = append(why, c.session.Nr+" "+firstOf(c.why, "andere Domäne "+c.session.Domain))
	}
	if n := len(why) - maxReasons; n > 0 {
		why = append(why[:maxReasons], fmt.Sprintf("… %d weitere", n))
	}
	return strings.Join(why, "; ")
}
