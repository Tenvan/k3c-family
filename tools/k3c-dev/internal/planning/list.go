package planning

import (
	"fmt"
	"os"
	"regexp"
	"slices"
	"sort"
	"strings"
)

// Filter für List; leere Felder filtern nicht.
type Filter struct {
	Kind    string `json:"kind,omitempty" jsonschema:"ticket, sprint, projekt oder leer für Sprints und Tickets"`
	Status  string `json:"status,omitempty" jsonschema:"Status, z. B. aktiv, geplant, offen, eingeplant, erledigt"`
	Domain  string `json:"domain,omitempty" jsonschema:"Domäne: REG, SIM, SRV, CLI, PLAT, INF"`
	Sprint  string `json:"sprint,omitempty" jsonschema:"Sprint-ID: nur dieser Sprint und seine Tickets"`
	Projekt string `json:"projekt,omitempty" jsonschema:"Projekt-Kürzel: nur dieses Projekt, seine Sprints und Tickets"`
	Archive bool   `json:"archive,omitempty" jsonschema:"auch erledigte und verworfene Tickets aus backlog/archiv"`
}

// List liefert Sprints (mit Sessions) und Tickets als knappe Zeilen.
func List(root string, f Filter) (string, error) {
	d, err := Load(root)
	if err != nil {
		return "", err
	}
	if f.Archive {
		d.Tickets = append(d.Tickets, archived(root)...)
	}
	var out []string
	switch f.Kind {
	case "projekt":
		out = projectLines(d, f)
	case "sprint":
		out = sprintList(d, f)
	case "ticket":
		out = ticketList(d, f)
	default:
		out = append(sprintList(d, f), ticketList(d, f)...)
	}
	if len(out) == 0 {
		return "keine Treffer", nil
	}
	return strings.Join(out, "\n"), nil
}

func match(want, have string) bool { return want == "" || strings.EqualFold(want, have) }

func sprintList(d Data, f Filter) []string {
	var out []string
	sortByPrio(d.Sprints)
	for _, s := range d.Sprints {
		if match(f.Status, s.Status) && match(f.Domain, s.Domain) && match(f.Sprint, s.ID) && match(f.Projekt, s.Project) {
			out = append(out, sprintLines(s)...)
		}
	}
	return out
}

func ticketList(d Data, f Filter) []string {
	var out []string
	for _, t := range d.Tickets {
		if match(f.Status, t.Status) && match(f.Domain, t.Domain) && match(f.Sprint, t.Sprint) && match(f.Projekt, t.Project) {
			out = append(out, fmt.Sprintf("%s %s %s %s %s %s %s · %s", t.Nr, t.Domain, t.Typ, t.Prio, t.Env, t.Status, t.Sprint, t.Title))
		}
	}
	return out
}

// projectLines: je Projekt `Rang Kürzel Status Sprints erledigt/gesamt · Titel` und die Sprints in Reihenfolge.
func projectLines(d Data, f Filter) []string {
	status := map[string]string{}
	for _, s := range d.Sprints {
		status[s.ID] = s.Status
	}
	var out []string
	for _, p := range d.Projects {
		if !match(f.Status, p.Status) || !match(f.Projekt, p.ID) {
			continue
		}
		done := 0
		var list []string
		for _, id := range p.Sprints {
			if status[id] == "erledigt" {
				done++
			}
			list = append(list, id+" "+firstOf(status[id], "fehlt"))
		}
		out = append(out, fmt.Sprintf("%s %s %s Sprints %d/%d · %s", p.Rang, p.ID, p.Status, done, len(p.Sprints), p.Title))
		if len(list) > 0 {
			out = append(out, "  "+strings.Join(list, " · "))
		}
	}
	return out
}

func sprintLines(s Sprint) []string {
	done := 0
	for _, x := range s.Sessions {
		if x.Status == "fertig" {
			done++
		}
	}
	out := []string{fmt.Sprintf("%s %s %s Prio %s Reife %s Spec %s · %s · Sessions %d/%d", s.ID, s.Domain, s.Status, s.Prio, s.Reife, s.Spec,
		s.Title, done, len(s.Sessions))}
	if s.Status == "erledigt" {
		return out
	}
	for _, x := range s.Sessions {
		out = append(out, strings.TrimRight(fmt.Sprintf("  %s %s %s %s %s %s", x.Nr, x.Typ, x.Agent, x.Env, x.Status, x.Titel), " "))
	}
	return out
}

func archived(root string) []Ticket {
	entries, _ := os.ReadDir(docsPath(root, "backlog/archiv"))
	var out []Ticket
	for _, e := range entries {
		if ticket.MatchString(e.Name()) {
			if text, _, err := readText(root, "backlog/archiv/"+e.Name()); err == nil {
				out = append(out, ParseTicket(text))
			}
		}
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Nr < out[j].Nr })
	return out
}

// SprintPrio ist die höchste Prio der genannten Tickets (offen oder archiviert); ohne bekannte Prio `?`.
func SprintPrio(root, tickets string) string {
	d, _ := Load(root)
	prio := map[string]string{}
	for _, t := range append(d.Tickets, archived(root)...) {
		prio[t.Nr] = t.Prio
	}
	best := "?"
	for _, id := range reTicketRef.FindAllString(tickets, -1) { // auch „B-011 teils“
		if p := prio[id]; prioRank(p) < prioRank(best) {
			best = p
		}
	}
	return best
}

var reTicketRef = regexp.MustCompile(`B-\d{3}`)

// sortByPrio ordnet je Status (aktiv, geplant, erledigt) nach Prio, sonst bleibt die Fahrplan-Reihenfolge.
func sortByPrio(sprints []Sprint) {
	order := map[string]int{"aktiv": 0, "geplant": 1, "erledigt": 2}
	sort.SliceStable(sprints, func(i, j int) bool {
		a, b := sprints[i], sprints[j]
		return order[a.Status] < order[b.Status] || order[a.Status] == order[b.Status] && prioRank(a.Prio) < prioRank(b.Prio)
	})
}

func prioRank(p string) int {
	if i := slices.Index(prios, p); i >= 0 {
		return i
	}
	return len(prios)
}
