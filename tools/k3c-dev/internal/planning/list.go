package planning

import (
	"fmt"
	"os"
	"sort"
	"strings"
)

// Filter für List; leere Felder filtern nicht.
type Filter struct {
	Kind    string `json:"kind,omitempty" jsonschema:"ticket, sprint oder leer für beides"`
	Status  string `json:"status,omitempty" jsonschema:"Status, z. B. aktiv, geplant, offen, eingeplant, erledigt"`
	Domain  string `json:"domain,omitempty" jsonschema:"Domäne: REG, SIM, SRV, CLI, PLAT, INF"`
	Sprint  string `json:"sprint,omitempty" jsonschema:"Sprint-ID: nur dieser Sprint und seine Tickets"`
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
	if f.Kind != "ticket" {
		for _, s := range d.Sprints {
			if match(f.Status, s.Status) && match(f.Domain, s.Domain) && match(f.Sprint, s.ID) {
				out = append(out, sprintLines(s)...)
			}
		}
	}
	if f.Kind != "sprint" {
		for _, t := range d.Tickets {
			if match(f.Status, t.Status) && match(f.Domain, t.Domain) && match(f.Sprint, t.Sprint) {
				out = append(out, fmt.Sprintf("%s %s %s %s %s %s · %s", t.Nr, t.Domain, t.Typ, t.Prio, t.Status, t.Sprint, t.Title))
			}
		}
	}
	if len(out) == 0 {
		return "keine Treffer", nil
	}
	return strings.Join(out, "\n"), nil
}

func match(want, have string) bool { return want == "" || strings.EqualFold(want, have) }

func sprintLines(s Sprint) []string {
	done := 0
	for _, x := range s.Sessions {
		if x.Status == "fertig" {
			done++
		}
	}
	out := []string{fmt.Sprintf("%s %s %s Reife %s Spec %s · %s · Sessions %d/%d", s.ID, s.Domain, s.Status, s.Reife, s.Spec,
		s.Title, done, len(s.Sessions))}
	if s.Status == "erledigt" {
		return out
	}
	for _, x := range s.Sessions {
		out = append(out, strings.TrimRight(fmt.Sprintf("  %s %s %s %s %s", x.Nr, x.Typ, x.Agent, x.Status, x.Titel), " "))
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
