package planning

import (
	"os"
	"strconv"
	"strings"
)

// Knoten der Wellenberechnung (B-783): Sessions in Arbeit (belegen Plätze), offene autonome Sessions aktiver Sprints,
// freigegebene Tickets ohne Sprint und freigegebene geplante Sprints, die 🧑 nicht brauchen.

// node ist ein Kandidat für eine Welle.
type node struct {
	id, kind, title, domain, project string
	sprint                            string // Sprint der Session
	rank, prio, order                 int
	area, locks, deps                 []string
	running                           bool
	why                               string // leer = in einer Welle; sonst der Grund
	fault                             bool   // why ist ein Fehler der Planungsangaben (unbekannte ID)
	level, wave                       int
	note                              string // warum nicht in einer früheren Welle
}

// waveData hält die geladene Planung und die Knoten.
type waveData struct {
	root    string
	d       Data
	nodes   []*node
	byID    map[string]*node
	sprints map[string]Sprint
	status  map[string]string // Session-ID → Status
	tickets map[string]Ticket
	specs   map[string]PlanSpec // Ticket-Nr oder Sprint-ID → Planungsangaben
	ranks   map[string]int      // Projekt → Platz (aktive nach Rang)
	active  map[string]bool     // Projekt aktiv
}

// loadWaveData lädt die Planung mit den Planungsangaben aller nicht archivierten Tickets und offenen Sprints.
func loadWaveData(root string) (*waveData, error) {
	d, err := Load(root)
	if err != nil {
		return nil, err
	}
	w := &waveData{root: root, d: d, byID: map[string]*node{}, sprints: map[string]Sprint{}, status: map[string]string{},
		tickets: map[string]Ticket{}, specs: ticketSpecs(root), ranks: map[string]int{}, active: map[string]bool{}}
	for i, p := range d.Projects {
		w.active[p.ID] = p.Status == "aktiv"
		if _, err := strconv.Atoi(p.Rang); err == nil && p.Status == "aktiv" {
			w.ranks[p.ID] = i
		}
	}
	for _, t := range d.Tickets {
		w.tickets[t.Nr] = t
	}
	for _, sp := range d.Sprints {
		w.sprints[sp.ID] = sp
		for _, x := range sp.Sessions {
			w.status[x.Nr] = x.Status
		}
		if sp.Status != "erledigt" {
			if r, ok := findSprint(root, sp.ID); ok {
				if text, _, err := readText(root, r.rel); err == nil {
					w.specs[sp.ID] = parsePlanSpec(text)
				}
			}
		}
	}
	w.build()
	return w, nil
}

// ticketSpecs liest die Planungsangaben aller Tickets in docs/backlog (ohne Archiv).
func ticketSpecs(root string) map[string]PlanSpec {
	out := map[string]PlanSpec{}
	entries, _ := os.ReadDir(docsPath(root, "backlog"))
	for _, e := range entries {
		if e.IsDir() || !ticket.MatchString(e.Name()) {
			continue
		}
		if text, _, err := readText(root, "backlog/"+e.Name()); err == nil {
			out[ParseTicket(text).Nr] = parsePlanSpec(text)
		}
	}
	return out
}

func (w *waveData) rankOf(project string) int {
	if r, ok := w.ranks[project]; ok {
		return r
	}
	return noRank
}

func (w *waveData) add(n *node) {
	n.order = len(w.nodes)
	w.nodes = append(w.nodes, n)
	w.byID[n.id] = n
}

// build legt die Knoten an: zuerst Sessions aktiver Sprints, dann Tickets, dann geplante Sprints.
func (w *waveData) build() {
	for _, sp := range w.d.Sprints {
		if sp.Status == "aktiv" && (sp.Project == "" || w.active[sp.Project] || !w.known(sp.Project)) {
			w.addSessions(sp)
		}
	}
	for _, t := range w.d.Tickets {
		sp := w.specs[t.Nr]
		if t.Status == "offen" && unplanned(firstOf(t.Sprint, "–")) && t.Spec == "freigegeben" && autonomous(sp) {
			w.add(&node{id: t.Nr, kind: "Ticket", title: t.Title, domain: t.Domain, project: t.Project,
				rank: w.rankOf(t.Project), prio: prioOrder(t.Prio), area: sp.Area, locks: sp.Locks, deps: sp.Deps,
				why: missingArea(sp)})
		}
	}
	for _, s := range w.d.Sprints {
		sp := w.specs[s.ID]
		if s.Status == "geplant" && s.Spec == "freigegeben" && autonomous(sp) {
			w.add(&node{id: s.ID, kind: "Sprint", title: s.Title, domain: s.Domain, project: s.Project,
				rank: w.rankOf(s.Project), prio: 1, area: sp.Area, locks: sp.Locks, deps: sp.Deps, why: missingArea(sp)})
		}
	}
}

func (w *waveData) known(project string) bool { _, ok := w.active[project]; return ok }

// addSessions: Sessions in Arbeit (alle Agenten) und offene autonome Sessions mit Datei.
func (w *waveData) addSessions(sp Sprint) {
	spec := w.specs[sp.ID]
	for _, x := range sp.Sessions {
		running := x.Status == "in Arbeit"
		if !running && (x.Status != "offen" || x.Agent != "autonom" || !isSessionFile(x.File)) {
			continue
		}
		area := allowedFiles(x.Text)
		why := ""
		if len(area) == 0 {
			area = spec.Area
			why = missingArea(spec)
		}
		head := sessionHead(x)["Abhängig von"]
		deps := append(sessionRef.FindAllString(head, -1), reTicketRef.FindAllString(head, -1)...)
		w.add(&node{id: x.Nr, kind: "Session", title: x.Titel, domain: x.Domain, project: sp.Project, sprint: sp.ID,
			rank: w.rankOf(sp.Project), prio: 1, area: area, locks: spec.Locks, deps: append(deps, spec.Deps...),
			running: running, why: whenNotRunning(running, why)})
	}
}

func whenNotRunning(running bool, why string) string {
	if running {
		return ""
	}
	return why
}

// needsHuman: `Braucht 🧑` nennt etwas (nicht leer, nicht `–`, auch nicht `– (Notiz)`).
func needsHuman(needs string) bool {
	return needs != "" && !strings.HasPrefix(needs, "–") && needs != "-"
}

// autonomous: die Zeile `Braucht 🧑` steht da und ist `–`.
func autonomous(sp PlanSpec) bool { return sp.Needs != "" && !needsHuman(sp.Needs) }

// missingArea ist der Grund „Bereich fehlt“ für Angaben ohne Bereich.
func missingArea(sp PlanSpec) string {
	if len(sp.Area) == 0 {
		return "Bereich fehlt"
	}
	return ""
}

// prioOrder: hoch vor mittel vor niedrig.
func prioOrder(p string) int {
	switch strings.ToLower(p) {
	case "hoch":
		return 0
	case "mittel":
		return 1
	case "niedrig":
		return 2
	}
	return 3
}

// depTarget löst eine ID aus „Abhängig von“ auf: done = erledigt; nodes = Knoten, nach denen der Abnehmer kommt;
// wait = Grund, warum er in keiner Welle steht; unknown = die ID gibt es nicht.
func (w *waveData) depTarget(id string) (done bool, nodes []*node, wait string, unknown bool) {
	if n, ok := w.byID[id]; ok {
		return false, []*node{n}, "", false
	}
	switch {
	case reSession.MatchString(id):
		s, ok := w.status[id]
		if !ok {
			return false, nil, "", true
		}
		return s == "fertig" || s == "verworfen", nil, id + " (" + s + ")", false
	case strings.HasPrefix(id, "B-"):
		return w.ticketDep(id)
	}
	sp, ok := w.sprints[id]
	if !ok {
		return false, nil, "", true
	}
	return w.sprintDep(sp)
}

func (w *waveData) ticketDep(id string) (bool, []*node, string, bool) {
	t, ok := w.tickets[id]
	if !ok {
		if findEntry(docsPath(w.root, "backlog/archiv"), id+"-", false) != "" {
			return true, nil, "", false
		}
		return false, nil, "", true
	}
	if t.Status == "erledigt" || t.Status == "verworfen" {
		return true, nil, "", false
	}
	if sp, ok := w.sprints[t.Sprint]; ok {
		done, nodes, wait, _ := w.sprintDep(sp)
		return done, nodes, strings.Replace(wait, sp.ID+" ", id+" → "+sp.ID+" ", 1), false
	}
	return false, nil, id + " (" + t.Status + ")", false
}

// sprintDep: Ein Sprint ist erledigt, wenn er im Ordner erledigt liegt; sonst kommt der Abnehmer nach allen offenen
// Sessions, die selbst Knoten sind. Eine offene Session, die kein Knoten ist (Mensch, ohne Datei), hält ihn auf.
func (w *waveData) sprintDep(sp Sprint) (bool, []*node, string, bool) {
	if sp.Status == "erledigt" {
		return true, nil, "", false
	}
	var nodes []*node
	for _, x := range sp.Sessions {
		if x.Status == "fertig" || x.Status == "verworfen" {
			continue
		}
		n, ok := w.byID[x.Nr]
		if !ok {
			return false, nil, sp.ID + " (" + x.Nr + " " + x.Status + ", Agent " + firstOf(x.Agent, "?") + ")", false
		}
		nodes = append(nodes, n)
	}
	if len(nodes) == 0 && sp.Status == "geplant" {
		return false, nil, sp.ID + " (geplant)", false
	}
	return false, nodes, "", false
}
