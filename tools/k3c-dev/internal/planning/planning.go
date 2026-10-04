// Package planning liest und schreibt die Planung des Repos: aktive und geplante Sprints mit ihren Sessions
// (docs/sprints/), Tickets (docs/backlog/) und die Dokumente Plan, Fragenkatalog und Glossar. Die Dateien bleiben die
// Quelle; Format und Vorlagen stehen in docs/arbeitsweise.md, die Schreibwege in store.go (B-210).
package planning

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
)

// Session ist eine Zeile der Session-Tabelle eines Sprints. Ein Entwurf hat nur Stichpunkte: Status "entwurf".
type Session struct {
	Nr     string `json:"nr"`
	Typ    string `json:"typ"`
	Agent  string `json:"agent"`
	Status string `json:"status"`
	Titel  string `json:"titel"`
	File   string `json:"-"`              // Spalte „Datei“ der Tabelle, relativ zum Sprint-Ordner
	Text   string `json:"text,omitempty"` // Inhalt der Session-Datei (Markdown) für das Detail-Panel
}

// Sprint ist die Kopfzeile und Session-Tabelle einer Sprint-README.
type Sprint struct {
	ID       string    `json:"id"`
	Title    string    `json:"title"`
	Domain   string    `json:"domain"`
	Status   string    `json:"status"` // aktiv | geplant | erledigt
	Reife    string    `json:"reife"`
	Spec     string    `json:"spec"`
	Tickets  []string  `json:"tickets"`
	Sessions []Session `json:"sessions"`
	Worktree string    `json:"worktree,omitempty"` // Branch eines Worktrees, der gerade an diesem Sprint arbeitet
}

// Ticket ist der Kopf einer Backlog-Datei.
type Ticket struct {
	Nr     string `json:"nr"`
	Title  string `json:"title"`
	Domain string `json:"domain"`
	Typ    string `json:"typ"`
	Prio   string `json:"prio"`
	Status string `json:"status"`
	Sprint string `json:"sprint"`
	Spec   string `json:"spec"`
}

// Data ist der Stand für die Oberfläche; die Listen sind nie nil.
type Data struct {
	Sprints []Sprint `json:"sprints"`
	Tickets []Ticket `json:"tickets"`
	Done    int      `json:"done"` // Zahl der erledigten Sprints
}

// Docs sind die lesbaren Dokumente; der Name kommt von der Oberfläche, der Pfad nie. DocOrder ist ihre Reihenfolge im
// Umschalter; das Glossar erscheint nur, wenn es die Datei gibt (B-211).
var (
	Docs     = map[string]string{"plan": "plan-weiterentwicklung.md", "fragen": "fragenkatalog.md", "glossar": "glossar.md"}
	DocOrder = []string{"plan", "fragen", "glossar"}
)

var (
	field   = regexp.MustCompile(`^- \*\*([^:*]+):\*\*\s*(.*)$`)
	session = regexp.MustCompile(`^\|\s*([A-Za-z0-9]+\.\d+)\s*\|\s*` + "`?" + `([^|` + "`" + `]*?)` + "`?" + `\s*\|\s*([^|]*?)\s*\|\s*([^|]*?)\s*\|\s*([^|]*?)\s*\|`)
	bullet  = regexp.MustCompile(`^- ([A-Za-z0-9]+\.\d+)\s+(.*)$`)
	ticket  = regexp.MustCompile(`^B-\d+-.*\.md$`)
)

// Load liest die Planung unter root/docs. Eine unlesbare Datei überspringt nur ihren Eintrag; fehlt docs/sprints oder
// docs/backlog ganz, kommt ein Fehler (falsche Wurzel).
func Load(root string) (Data, error) {
	d := Data{Sprints: []Sprint{}, Tickets: []Ticket{}}
	for _, status := range []string{"aktiv", "geplant", "erledigt"} {
		dir := filepath.Join(root, "docs", "sprints", status)
		entries, err := os.ReadDir(dir)
		if err != nil && status == "erledigt" {
			continue // ohne erledigte Sprints geht es auch
		}
		if err != nil {
			return d, fmt.Errorf("sprints nicht lesbar: %w", err)
		}
		for _, e := range entries {
			if !e.IsDir() {
				continue
			}
			if status == "erledigt" {
				d.Done++
			}
			if sp, ok := loadSprint(filepath.Join(dir, e.Name()), status); ok {
				d.Sprints = append(d.Sprints, sp)
			}
		}
	}
	markWorktrees(d.Sprints, Worktrees(root))
	files, err := os.ReadDir(filepath.Join(root, "docs", "backlog"))
	if err != nil {
		return d, fmt.Errorf("backlog nicht lesbar: %w", err)
	}
	for _, f := range files {
		if f.IsDir() || !ticket.MatchString(f.Name()) {
			continue
		}
		text, err := os.ReadFile(filepath.Join(root, "docs", "backlog", f.Name()))
		if err != nil {
			continue
		}
		d.Tickets = append(d.Tickets, ParseTicket(string(text)))
	}
	sort.Slice(d.Tickets, func(i, j int) bool { return d.Tickets[i].Nr < d.Tickets[j].Nr })
	return d, nil
}

// loadSprint liest die README eines Sprint-Ordners und die Session-Dateien aus der Spalte „Datei“ (nur Dateinamen im
// Ordner, kein Pfad).
func loadSprint(dir, status string) (Sprint, bool) {
	text, err := os.ReadFile(filepath.Join(dir, "README.md"))
	if err != nil {
		return Sprint{}, false
	}
	sp := ParseSprint(string(text), filepath.Base(dir), status)
	for i, x := range sp.Sessions {
		if x.File != "" && !strings.ContainsAny(x.File, `/\`) {
			if b, err := os.ReadFile(filepath.Join(dir, x.File)); err == nil {
				sp.Sessions[i].Text = string(b)
			}
		}
	}
	return sp, true
}

// Doc liefert eines der Dokumente aus Docs.
func Doc(root, name string) (string, error) {
	file, ok := Docs[name]
	if !ok {
		return "", fmt.Errorf("unbekanntes Dokument %q", name)
	}
	b, err := os.ReadFile(filepath.Join(root, "docs", file))
	return string(b), err
}

// Available nennt die Dokumente aus DocOrder, deren Datei existiert.
func Available(root string) []string {
	out := []string{}
	for _, name := range DocOrder {
		if _, err := os.Stat(filepath.Join(root, "docs", Docs[name])); err == nil {
			out = append(out, name)
		}
	}
	return out
}

// header zerlegt `# SP11 · SRV · Titel` oder `# B-090 · Titel` am Mittelpunkt.
func header(lines []string) []string {
	for _, l := range lines {
		if rest, ok := strings.CutPrefix(l, "# "); ok {
			return strings.Split(rest, " · ")
		}
	}
	return nil
}

func fields(lines []string) map[string]string {
	m := map[string]string{}
	for _, l := range lines {
		if f := field.FindStringSubmatch(l); f != nil {
			m[f[1]] = f[2]
		}
	}
	return m
}

// ParseSprint liest eine Sprint-README; dir (Ordnername) ersetzt eine fehlende Kopfzeile.
func ParseSprint(text, dir, status string) Sprint {
	lines := strings.Split(strings.ReplaceAll(text, "\r\n", "\n"), "\n")
	f := fields(lines)
	s := Sprint{ID: strings.SplitN(dir, "-", 2)[0], Title: dir, Domain: f["Domäne"], Status: status, Reife: f["Reife"],
		Spec: f["Spec"], Tickets: []string{}, Sessions: []Session{}}
	if h := header(lines); len(h) >= 3 {
		s.ID, s.Domain, s.Title = h[0], h[1], strings.Join(h[2:], " · ")
	}
	for _, t := range strings.Split(f["Tickets"], ",") {
		if t = strings.TrimSpace(t); t != "" {
			s.Tickets = append(s.Tickets, t)
		}
	}
	inSessions := false
	for _, l := range lines {
		if strings.HasPrefix(l, "## ") {
			inSessions = l == "## Sessions"
			continue
		}
		if !inSessions {
			continue
		}
		if m := session.FindStringSubmatch(l); m != nil {
			s.Sessions = append(s.Sessions, Session{Nr: m[1], File: m[2], Typ: m[3], Agent: m[4], Status: m[5]})
		} else if m := bullet.FindStringSubmatch(l); m != nil {
			s.Sessions = append(s.Sessions, Session{Nr: m[1], Status: "entwurf", Titel: m[2]})
		}
	}
	return s
}

// ParseTicket liest den Kopf einer Backlog-Datei.
func ParseTicket(text string) Ticket {
	lines := strings.Split(strings.ReplaceAll(text, "\r\n", "\n"), "\n")
	f := fields(lines)
	t := Ticket{Domain: f["Domäne"], Typ: f["Typ"], Prio: f["Prio"], Status: f["Status"], Sprint: f["Sprint"], Spec: f["Spec"]}
	if h := header(lines); len(h) >= 2 {
		t.Nr, t.Title = h[0], strings.Join(h[1:], " · ")
	}
	return t
}
